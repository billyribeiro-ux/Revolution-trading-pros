//! Read-only provider checks. Credential shape and HTTP 200 alone are never proof.
use crate::config::Config;
use reqwest::{Client, Url};
use serde::Serialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    net::IpAddr,
    time::{Duration, Instant},
};

#[derive(Debug, Serialize)]
pub(super) struct Probe {
    pub success: bool,
    pub status: &'static str,
    pub message: String,
    pub latency_ms: Option<i64>,
    pub status_code: Option<i32>,
}
impl Probe {
    fn unavailable(message: impl Into<String>) -> Self {
        Self {
            success: false,
            status: "unavailable",
            message: message.into(),
            latency_ms: None,
            status_code: None,
        }
    }
}
fn credential<'a>(credentials: &'a HashMap<String, String>, key: &str) -> Result<&'a str, Probe> {
    credentials
        .get(key)
        .map(String::as_str)
        .filter(|value| {
            !value.trim().is_empty()
                && !value.contains(['\r', '\n', '\0'])
                && !value.contains("********")
        })
        .ok_or_else(|| Probe::unavailable(format!("Missing or invalid {key}")))
}

/// An operator-only test override must be development + literal loopback. Never
/// accept a caller-supplied replacement for a fixed provider origin.
pub(super) fn adapter_base(environment: &str, value: Option<&str>) -> Result<Option<Url>, String> {
    let Some(value) = value.filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    let url = Url::parse(value).map_err(|_| "Invalid CONNECTION_TEST_BASE_URL")?;
    let host = url.host_str().unwrap_or("").trim_matches(['[', ']']);
    if environment != "development"
        || url.scheme() != "http"
        || !host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(
            "CONNECTION_TEST_BASE_URL requires development and a literal loopback HTTP URL".into(),
        );
    }
    Ok(Some(url))
}

async fn client_for(url: &Url, environment: &str) -> Result<Client, Probe> {
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(Probe::unavailable(
            "Provider URLs cannot contain credentials, queries or fragments",
        ));
    }
    let host = url
        .host_str()
        .ok_or_else(|| Probe::unavailable("Missing provider host"))?
        .trim_matches(['[', ']']);
    let local =
        environment == "development" && host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback());
    if url.scheme() != "https" && !(local && url.scheme() == "http") {
        return Err(Probe::unavailable(
            "Provider requires HTTPS (loopback HTTP is development-only)",
        ));
    }
    let addresses: Vec<_> = tokio::time::timeout(
        Duration::from_secs(3),
        tokio::net::lookup_host((host, url.port_or_known_default().unwrap_or(443))),
    )
    .await
    .map_err(|_| Probe::unavailable("Provider DNS lookup timed out"))?
    .map_err(|_| Probe::unavailable("Provider DNS lookup failed"))?
    .collect();
    if addresses.is_empty()
        || (!local
            && addresses
                .iter()
                .any(|address| crate::services::cms_webhooks::is_forbidden_egress_ip(address.ip())))
    {
        return Err(Probe::unavailable(
            "Provider host resolves to a prohibited network",
        ));
    }
    Client::builder()
        .timeout(Duration::from_secs(8))
        .connect_timeout(Duration::from_secs(3))
        .redirect(reqwest::redirect::Policy::none())
        .no_proxy()
        .resolve_to_addrs(host, &addresses)
        .build()
        .map_err(|_| Probe::unavailable("Provider client is unavailable"))
}

async fn bounded_json(mut response: reqwest::Response) -> Result<Value, Probe> {
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| Probe::unavailable("Provider response was interrupted"))?
    {
        if bytes.len() + chunk.len() > 64 * 1024 {
            return Err(Probe::unavailable("Provider response exceeded 64 KiB"));
        }
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes)
        .map_err(|_| Probe::unavailable("Provider returned an invalid JSON response"))
}

pub(super) async fn check(
    config: &Config,
    key: &str,
    credentials: &HashMap<String, String>,
    environment: &str,
) -> Probe {
    let start = Instant::now();
    let result = check_inner(config, key, credentials, environment).await;
    match result {
        Ok(mut probe) => {
            probe.latency_ms = Some(start.elapsed().as_millis().try_into().unwrap_or(i64::MAX));
            probe
        }
        Err(probe) => probe,
    }
}

async fn check_inner(
    config: &Config,
    key: &str,
    credentials: &HashMap<String, String>,
    environment: &str,
) -> Result<Probe, Probe> {
    let mut path = match key {
        "stripe" => "/v1/balance".to_owned(),
        "sendgrid" => "/v3/scopes".to_owned(),
        "openai" => "/v1/models".to_owned(),
        "meilisearch" => "/version".to_owned(),
        "bunny_cdn" => {
            let zone = credential(credentials, "storage_zone")?;
            if !zone
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
            {
                return Err(Probe::unavailable("Invalid storage zone name"));
            }
            format!("/{zone}/")
        }
        "google_analytics" => {
            let property = credential(credentials, "property_id")?;
            let stream = credential(credentials, "stream_id")?;
            if ![property, stream]
                .iter()
                .all(|part| part.bytes().all(|byte| byte.is_ascii_digit()))
            {
                return Err(Probe::unavailable(
                    "Property and stream IDs must be numeric",
                ));
            }
            format!("/v1beta/properties/{property}/dataStreams/{stream}")
        }
        "cloudflare_r2" => return check_r2(config, credentials).await,
        _ => {
            return Err(Probe::unavailable(
                "No provider check is defined for this service",
            ));
        }
    };
    let origin = match key {
        "stripe" => "https://api.stripe.com",
        "sendgrid" => "https://api.sendgrid.com",
        "openai" => "https://api.openai.com",
        "bunny_cdn" => "https://storage.bunnycdn.com",
        "google_analytics" => "https://analyticsadmin.googleapis.com",
        "meilisearch" => credential(credentials, "host")?,
        _ => unreachable!(),
    };
    let adapter = adapter_base(
        &config.environment,
        std::env::var("CONNECTION_TEST_BASE_URL").ok().as_deref(),
    )
    .map_err(Probe::unavailable)?;
    let mut url = if let Some(base) = adapter {
        path = format!("/{key}{path}");
        base
    } else {
        Url::parse(origin).map_err(|_| Probe::unavailable("Invalid provider origin"))?
    };
    // Retain a configured self-hosted reverse-proxy prefix, but no user-controlled query.
    url.set_path(&format!("{}{path}", url.path().trim_end_matches('/')));
    let client = client_for(&url, &config.environment).await?;
    let mut request = client.get(url).header("Accept", "application/json");
    request = match key {
        "stripe" => request.bearer_auth(credential(credentials, "secret_key")?),
        "bunny_cdn" => request.header("AccessKey", credential(credentials, "api_key")?),
        "google_analytics" => request.bearer_auth(credential(credentials, "access_token")?),
        _ => request.bearer_auth(credential(credentials, "api_key")?),
    };
    if key == "openai"
        && let Some(organization) = credentials
            .get("organization_id")
            .filter(|value| !value.is_empty())
    {
        request = request.header("OpenAI-Organization", organization);
    }
    let response = request
        .send()
        .await
        .map_err(|_| Probe::unavailable("Provider could not be reached or timed out"))?;
    let status_code = Some(i32::from(response.status().as_u16()));
    if !response.status().is_success() {
        return Ok(Probe {
            success: false,
            status: if response.status().is_client_error() && response.status().as_u16() != 429 {
                "rejected"
            } else {
                "unavailable"
            },
            message: format!("Provider returned HTTP {}", response.status().as_u16()),
            latency_ms: None,
            status_code,
        });
    }
    let payload = bounded_json(response).await?;
    let valid = match key {
        "stripe" => {
            payload["object"] == "balance"
                && payload["available"].is_array()
                && payload["livemode"].as_bool() == Some(environment == "production")
        }
        "sendgrid" => payload["scopes"]
            .as_array()
            .is_some_and(|scopes| scopes.iter().any(|scope| scope == "mail.send")),
        "openai" => payload["object"] == "list" && payload["data"].is_array(),
        "meilisearch" => payload["pkgVersion"]
            .as_str()
            .is_some_and(|version| !version.is_empty()),
        "bunny_cdn" => payload.is_array(),
        "google_analytics" => {
            payload["webStreamData"]["measurementId"].as_str()
                == Some(credential(credentials, "measurement_id")?)
        }
        _ => false,
    };
    Ok(Probe {
        success: valid,
        status: if valid { "verified" } else { "rejected" },
        message: if valid {
            "Read-only provider check passed".into()
        } else {
            "Provider response did not verify the required resource, permission or environment"
                .into()
        },
        latency_ms: None,
        status_code,
    })
}

async fn check_r2(config: &Config, credentials: &HashMap<String, String>) -> Result<Probe, Probe> {
    use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
    let account = credential(credentials, "account_id")?;
    if account.len() != 32 || !account.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(Probe::unavailable("Invalid R2 account ID"));
    }
    let adapter = adapter_base(
        &config.environment,
        std::env::var("CONNECTION_TEST_BASE_URL").ok().as_deref(),
    )
    .map_err(Probe::unavailable)?;
    let endpoint = adapter
        .map(|url| url.to_string())
        .unwrap_or_else(|| format!("https://{account}.r2.cloudflarestorage.com"));
    let sdk = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("auto"))
        .credentials_provider(Credentials::new(
            credential(credentials, "access_key_id")?,
            credential(credentials, "secret_access_key")?,
            None,
            None,
            "connection-check",
        ))
        .endpoint_url(endpoint)
        .force_path_style(true)
        .retry_config(aws_sdk_s3::config::retry::RetryConfig::disabled())
        .build();
    let result = tokio::time::timeout(
        Duration::from_secs(8),
        aws_sdk_s3::Client::from_conf(sdk)
            .head_bucket()
            .bucket(credential(credentials, "bucket_name")?)
            .send(),
    )
    .await;
    Ok(match result {
        Ok(Ok(_)) => Probe {
            success: true,
            status: "verified",
            message:
                "Authenticated bucket check passed (read/write object permissions are separate)"
                    .into(),
            latency_ms: None,
            status_code: Some(200),
        },
        Ok(Err(_)) => Probe::unavailable(
            "Bucket check failed; verify bucket identity, credentials and permissions",
        ),
        Err(_) => Probe::unavailable("Bucket check timed out"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adapter_cannot_redirect_production_or_access_remote_internal_hosts() {
        for value in ["http://127.0.0.1:19021/", "http://[::1]:19021/"] {
            assert!(adapter_base("development", Some(value)).is_ok());
            assert!(adapter_base("production", Some(value)).is_err());
        }
        for value in [
            "http://localhost:19021/",
            "https://example.test/",
            "http://169.254.169.254/",
            "http://127.0.0.1/?token=secret",
            "http://x@127.0.0.1/",
        ] {
            assert!(adapter_base("development", Some(value)).is_err(), "{value}");
        }
    }
}

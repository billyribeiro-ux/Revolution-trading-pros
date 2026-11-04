//! Durable, at-least-once CRM notifications. Receivers deduplicate X-Webhook-Delivery-Id.
use anyhow::{Result, anyhow, bail};
use async_trait::async_trait;
use serde_json::{Value, json};
use sqlx::PgPool;
use std::{net::SocketAddr, time::Duration};
use uuid::Uuid;

pub const EVENTS: &[(&str, &str)] = &[
    ("contact_created", "Contact Created"),
    ("contact_updated", "Contact Updated"),
    ("contact_deleted", "Contact Deleted"),
    ("tag_applied", "Tag Applied"),
    ("tag_removed", "Tag Removed"),
    ("list_applied", "List Applied"),
    ("list_removed", "List Removed"),
    ("email_opened", "Email Opened"),
    ("email_clicked", "Email Clicked"),
    ("contact_unsubscribed", "Contact Unsubscribed"),
    ("deal_created", "Deal Created"),
    ("deal_won", "Deal Won"),
    ("deal_lost", "Deal Lost"),
];

/// Syntactic checks at authoring; fresh DNS checks and pinning at every delivery.
pub fn target(raw: &str) -> Result<reqwest::Url> {
    let url = reqwest::Url::parse(raw)?;
    if raw.chars().any(char::is_control)
        || raw.len() > 500
        || url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        bail!("Webhook URL must be HTTPS, at most 500 bytes, without credentials or fragments");
    }
    let host = url
        .host_str()
        .ok_or_else(|| anyhow!("Webhook URL requires a host"))?
        .trim_matches(['[', ']']);
    if host.eq_ignore_ascii_case("localhost")
        || host.ends_with(".localhost")
        || host
            .parse()
            .is_ok_and(super::cms_webhooks::is_forbidden_egress_ip)
    {
        bail!("Webhook URL cannot target internal addresses");
    }
    Ok(url)
}

pub fn validate_headers(headers: &Value) -> Result<()> {
    let headers = headers
        .as_object()
        .ok_or_else(|| anyhow!("Headers must be an object"))?;
    if headers.len() > 20 {
        bail!("At most 20 custom headers are supported");
    }
    for (name, value) in headers {
        let value = value
            .as_str()
            .ok_or_else(|| anyhow!("Header values must be strings"))?;
        if name.len() > 128
            || value.len() > 2048
            || name.parse::<reqwest::header::HeaderName>().is_err()
            || value.parse::<reqwest::header::HeaderValue>().is_err()
            || matches!(
                name.to_ascii_lowercase().as_str(),
                "host"
                    | "content-type"
                    | "content-length"
                    | "transfer-encoding"
                    | "connection"
                    | "proxy-authorization"
                    | "proxy-connection"
                    | "upgrade"
                    | "te"
                    | "trailer"
            )
            || name.to_ascii_lowercase().starts_with("x-webhook-")
        {
            bail!("Invalid or reserved webhook header: {name}");
        }
    }
    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct Attempt {
    pub id: Uuid,
    pub webhook_id: i64,
    pub payload: Value,
    pub event: String,
    pub attempts: i32,
    pub lease_token: Uuid,
    pub url: String,
    pub secret: Option<String>,
    pub headers: Value,
}
#[derive(Debug)]
pub struct Outcome {
    pub code: Option<i32>,
    pub body: String,
    pub error: Option<String>,
    pub permanent: bool,
}
#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, attempt: &Attempt) -> Outcome;
}
pub struct HttpTransport;

/// Build the exact signed request body once. Tests verify the receiver's bytes.
pub fn signed_request(
    client: &reqwest::Client,
    url: reqwest::Url,
    attempt: &Attempt,
) -> Result<reqwest::RequestBuilder> {
    validate_headers(&attempt.headers)?;
    let body = serde_json::to_string(&attempt.payload)?;
    let mut request = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("X-Webhook-Event", &attempt.event)
        .header("X-Webhook-Delivery-Id", attempt.id.to_string());
    if let Some(secret) = &attempt.secret {
        request = request.header(
            "X-Webhook-Signature",
            format!(
                "sha256={}",
                super::cms_webhooks::generate_signature(&body, secret)
            ),
        );
    }
    if let Some(headers) = attempt.headers.as_object() {
        for (name, value) in headers {
            request = request.header(
                name,
                value.as_str().ok_or_else(|| anyhow!("Invalid header"))?,
            );
        }
    }
    Ok(request.body(body))
}

pub async fn response_outcome(response: Result<reqwest::Response, reqwest::Error>) -> Outcome {
    match response {
        Ok(mut response) => {
            let code = i32::from(response.status().as_u16());
            let mut bytes = Vec::new();
            // Bound retained response data; the request's total timeout includes body reads.
            while bytes.len() < 4096 {
                match response.chunk().await {
                    Ok(Some(chunk)) => {
                        bytes.extend_from_slice(&chunk[..chunk.len().min(4096 - bytes.len())])
                    }
                    Ok(None) => break,
                    Err(_) => break, // HTTP status remains the receiver's acknowledgement.
                }
            }
            Outcome {
                code: Some(code),
                body: String::from_utf8_lossy(&bytes).into_owned(),
                error: (!(200..300).contains(&code))
                    .then(|| format!("Receiver returned HTTP {code}")),
                permanent: (300..500).contains(&code) && code != 408 && code != 429,
            }
        }
        Err(error) => Outcome {
            code: None,
            body: String::new(),
            error: Some(
                if error.is_timeout() {
                    "Receiver request timed out"
                } else {
                    "Receiver connection failed"
                }
                .into(),
            ),
            permanent: false,
        },
    }
}
#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, attempt: &Attempt) -> Outcome {
        let result: Result<reqwest::RequestBuilder> = async {
            let url = target(&attempt.url)?;
            let host = url
                .host_str()
                .ok_or_else(|| anyhow!("Missing host"))?
                .trim_matches(['[', ']']);
            let addresses: Vec<SocketAddr> = tokio::time::timeout(
                Duration::from_secs(3),
                tokio::net::lookup_host((host, url.port_or_known_default().unwrap_or(443))),
            )
            .await??
            .collect();
            if addresses.is_empty()
                || addresses
                    .iter()
                    .any(|address| super::cms_webhooks::is_forbidden_egress_ip(address.ip()))
            {
                bail!("Receiver DNS resolved to a forbidden or empty address set");
            }
            // Disable environment proxies and redirects, and pin the checked DNS answer.
            let client = reqwest::Client::builder()
                .no_proxy()
                .redirect(reqwest::redirect::Policy::none())
                .resolve_to_addrs(host, &addresses)
                .connect_timeout(Duration::from_secs(3))
                .timeout(Duration::from_secs(8))
                .build()?;
            signed_request(&client, url, attempt)
        }
        .await;
        match result {
            Ok(request) => response_outcome(request.send().await).await,
            Err(error) => Outcome {
                code: None,
                body: String::new(),
                error: Some(error.to_string()),
                permanent: target(&attempt.url).is_err() || error.to_string().contains("forbidden"),
            },
        }
    }
}

pub async fn claim(pool: &PgPool, only: Option<Uuid>) -> Result<Option<Attempt>> {
    sqlx::query("UPDATE crm_webhook_deliveries SET status='failed',error_message='Final delivery lease expired; receiver outcome may be unknown',lease_token=NULL,lease_expires_at=NULL WHERE status='processing' AND lease_expires_at<NOW() AND attempts>=3").execute(pool).await?;
    sqlx::query("UPDATE crm_webhook_deliveries d SET status='cancelled',error_message='Webhook disabled',lease_token=NULL,lease_expires_at=NULL FROM crm_webhooks w WHERE d.webhook_id=w.id AND w.is_active=false AND d.status='queued'").execute(pool).await?;
    let item=sqlx::query_as("WITH next AS (SELECT d.id,w.url,w.secret,w.headers FROM crm_webhook_deliveries d JOIN crm_webhooks w ON w.id=d.webhook_id WHERE ($1::uuid IS NULL OR d.id=$1) AND w.is_active=true AND d.attempts<3 AND ((d.status='queued' AND d.due_at<=NOW()) OR (d.status='processing' AND d.lease_expires_at<NOW())) ORDER BY d.triggered_at,d.id LIMIT 1 FOR UPDATE OF d SKIP LOCKED) UPDATE crm_webhook_deliveries d SET status='processing',attempts=attempts+1,lease_token=gen_random_uuid(),lease_expires_at=NOW()+INTERVAL '30 seconds' FROM next WHERE d.id=next.id RETURNING d.id,d.webhook_id,d.payload,d.event,d.attempts,d.lease_token,next.url,next.secret,next.headers")
        .bind(only).fetch_optional(pool).await?;
    Ok(item)
}

pub async fn process_one(
    pool: &PgPool,
    transport: &impl Transport,
    only: Option<Uuid>,
) -> Result<bool> {
    let Some(attempt) = claim(pool, only).await? else {
        return Ok(false);
    };
    let outcome = transport.send(&attempt).await;
    let success =
        outcome.code.is_some_and(|code| (200..300).contains(&code)) && outcome.error.is_none();
    let status = if success {
        "delivered"
    } else if outcome.permanent || attempt.attempts >= 3 {
        "failed"
    } else {
        "queued"
    };
    let mut tx = pool.begin().await?;
    let saved=sqlx::query("UPDATE crm_webhook_deliveries SET status=$3,response_code=$4,response_body=$5,error_message=$6,due_at=NOW()+make_interval(secs=>$7),delivered_at=CASE WHEN $3='delivered' THEN NOW() ELSE NULL END,lease_token=NULL,lease_expires_at=NULL WHERE id=$1 AND lease_token=$2 AND status='processing'")
        .bind(attempt.id).bind(attempt.lease_token).bind(status).bind(outcome.code).bind(&outcome.body).bind(&outcome.error).bind(30*attempt.attempts).execute(&mut *tx).await?;
    if saved.rows_affected() == 1 {
        sqlx::query("UPDATE crm_webhooks SET trigger_count=COALESCE(trigger_count,0)+1,failure_count=COALESCE(failure_count,0)+$2,last_triggered_at=NOW() WHERE id=$1").bind(attempt.webhook_id).bind(i32::from(!success)).execute(&mut *tx).await?;
        super::crm_logs::record(&mut tx,if success{"info"}else{"warning"},"webhook","CRM webhook delivery attempt",json!({"webhook_id":attempt.webhook_id.to_string(),"delivery_id":attempt.id,"status":status,"attempt":attempt.attempts,"response_code":outcome.code}),None).await?;
    }
    tx.commit().await?;
    Ok(true)
}

pub async fn run(pool: PgPool) {
    loop {
        for _ in 0..10 {
            match process_one(&pool, &HttpTransport, None).await {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => {
                    tracing::error!(%error,"CRM webhook worker failed");
                    break;
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

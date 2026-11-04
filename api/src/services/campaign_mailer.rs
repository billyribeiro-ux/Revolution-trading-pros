//! Bounded Postmark submission for CRM broadcasts. Acceptance is not inbox delivery.
use anyhow::{Result, anyhow, bail};
use reqwest::{Client, Url};
use serde::Serialize;
use serde_json::Value;
use std::{collections::BTreeMap, net::IpAddr, time::Duration};

#[derive(Clone)]
pub struct Identity {
    pub name: String,
    pub email: String,
}
impl Identity {
    pub(crate) fn formatted(&self) -> Result<String> {
        crate::utils::errors::validate_email(&self.email)
            .map_err(|_| anyhow!("Configure a valid sender or reply-to email"))?;
        if self.name.contains(['\r', '\n', '\0']) || self.email.contains(['\r', '\n', '\0']) {
            bail!("Email identities cannot contain line breaks or null characters");
        }
        let formatted = if self.name.is_empty() {
            self.email.clone()
        } else {
            format!(
                "\"{}\" <{}>",
                self.name.replace('\\', "\\\\").replace('"', "\\\""),
                self.email
            )
        };
        if formatted.encode_utf16().count() > 255 {
            bail!("Email identity exceeds the provider's 255-character limit");
        }
        Ok(formatted)
    }
}
pub struct Message {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub plain_text: bool,
    pub text_body: Option<String>,
    pub from: Option<Identity>,
    pub reply_to: Option<Identity>,
    pub unsubscribe_url: String,
    pub metadata: BTreeMap<String, String>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Submission {
    Accepted {
        message_id: String,
    },
    Rejected {
        reason: String,
        retryable: bool,
    },
    /// A crash, timeout or ambiguous response must never be retried as a known failure.
    Uncertain {
        reason: String,
    },
}
#[derive(Clone)]
pub struct CampaignMailer {
    client: Client,
    endpoint: Url,
    token: String,
    default_sender: Identity,
    stream: String,
}
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Header {
    name: String,
    value: String,
}
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Payload {
    from: String,
    to: String,
    subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    html_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
    headers: Vec<Header>,
    track_opens: bool,
    track_links: &'static str,
    message_stream: String,
    metadata: BTreeMap<String, String>,
}
pub(crate) fn endpoint(environment: &str, override_url: Option<&str>) -> Result<Url> {
    let Some(value) = override_url.filter(|value| !value.trim().is_empty()) else {
        return Ok(Url::parse("https://api.postmarkapp.com/email")?);
    };
    if environment != "development" {
        bail!("POSTMARK_TEST_ENDPOINT is allowed only in development");
    }
    let parsed = Url::parse(value)?;
    let loopback = parsed
        .host_str()
        .and_then(|host| {
            host.trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<IpAddr>()
                .ok()
        })
        .is_some_and(|ip| ip.is_loopback());
    if !matches!(parsed.scheme(), "http" | "https")
        || !loopback
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        bail!(
            "POSTMARK_TEST_ENDPOINT must be an HTTP(S) URL on a literal loopback address without credentials, query or fragment"
        );
    }
    Ok(parsed)
}
impl CampaignMailer {
    pub fn stream(&self) -> &str {
        &self.stream
    }
    pub fn sender(&self) -> &Identity {
        &self.default_sender
    }
    /// Operators must explicitly select the provider's broadcast stream; never
    /// silently send CRM broadcasts through a transactional default stream.
    pub fn from_env(environment: &str) -> Result<Self> {
        fn value(keys: &[&str]) -> String {
            keys.iter()
                .find_map(|key| {
                    std::env::var(key)
                        .ok()
                        .filter(|value| !value.trim().is_empty())
                        .map(|value| value.trim().to_string())
                })
                .unwrap_or_default()
        }
        let token = value(&["POSTMARK_TOKEN", "POSTMARK_API_KEY"]);
        let email = value(&["POSTMARK_FROM_EMAIL", "EMAIL_FROM_ADDRESS", "FROM_EMAIL"]);
        let stream = value(&["POSTMARK_BROADCAST_STREAM"]);
        let name = value(&["EMAIL_FROM_NAME", "FROM_NAME"]);
        let local = value(&["POSTMARK_TEST_ENDPOINT"]);
        Self::new(
            environment,
            &token,
            Identity { name, email },
            &stream,
            Some(&local),
        )
    }
    /// Explicit configuration is also the controlled-adapter seam. The same
    /// loopback-only development restriction applies to every caller.
    pub fn new(
        environment: &str,
        token: &str,
        default_sender: Identity,
        stream: &str,
        test_endpoint: Option<&str>,
    ) -> Result<Self> {
        if token.is_empty()
            || token.len() > 1024
            || !token.bytes().all(|byte| byte.is_ascii_graphic())
        {
            bail!("Configure POSTMARK_TOKEN or POSTMARK_API_KEY");
        }
        default_sender.formatted()?;
        if stream.is_empty()
            || stream.len() > 100
            || !stream.bytes().all(|byte| byte.is_ascii_graphic())
        {
            bail!("Configure POSTMARK_BROADCAST_STREAM with an authorized broadcast stream ID");
        }
        let endpoint = endpoint(environment, test_endpoint)?;
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5))
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Ok(Self {
            client,
            endpoint,
            token: token.to_string(),
            default_sender,
            stream: stream.to_string(),
        })
    }
    fn payload(&self, message: Message) -> Result<Payload> {
        crate::utils::errors::validate_email(&message.to)
            .map_err(|_| anyhow!("Recipient email is invalid"))?;
        if message.to.contains(['\r', '\n', '\0'])
            || message.subject.contains(['\r', '\n', '\0'])
            || message.subject.is_empty()
            || message.subject.chars().count() > 500
        {
            bail!("Email requires a valid recipient and a single-line subject of 1–500 characters");
        }
        if message.body.trim().is_empty()
            || message.body.len() > 4_000_000
            || message.body.contains('\0')
        {
            bail!("Email body is empty or exceeds the supported limit");
        }
        let unsubscribe = Url::parse(&message.unsubscribe_url)?;
        if !matches!(unsubscribe.scheme(), "http" | "https")
            || unsubscribe.host_str().is_none()
            || !unsubscribe.username().is_empty()
            || unsubscribe.password().is_some()
            || message.unsubscribe_url.contains(['\r', '\n', '\0'])
        {
            bail!("Email requires a valid HTTP(S) unsubscribe link");
        }
        let from = message
            .from
            .as_ref()
            .unwrap_or(&self.default_sender)
            .formatted()?;
        let reply_to = message
            .reply_to
            .map(|identity| identity.formatted())
            .transpose()?;
        Ok(Payload {
            from,
            to: message.to,
            subject: message.subject,
            html_body: if message.plain_text {
                None
            } else {
                Some(message.body.clone())
            },
            text_body: if message.plain_text {
                Some(message.body)
            } else {
                message.text_body
            },
            reply_to,
            headers: vec![
                Header {
                    name: "List-Unsubscribe".into(),
                    value: format!("<{}>", message.unsubscribe_url),
                },
                Header {
                    name: "List-Unsubscribe-Post".into(),
                    value: "List-Unsubscribe=One-Click".into(),
                },
            ],
            track_opens: true,
            track_links: "HtmlAndText",
            message_stream: self.stream.clone(),
            metadata: message.metadata,
        })
    }
    pub async fn submit(&self, message: Message) -> Submission {
        let payload = match self.payload(message) {
            Ok(payload) => payload,
            Err(error) => {
                return Submission::Rejected {
                    reason: error.to_string(),
                    retryable: false,
                };
            }
        };
        submit_request(&self.client, &self.endpoint, &self.token, &payload).await
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(crate) struct UncertainSubmission(pub String);
#[derive(Debug, thiserror::Error)]
#[error("{reason}")]
pub(crate) struct RejectedSubmission {
    pub reason: String,
    pub retryable: bool,
}
impl Submission {
    pub(crate) fn into_result(self) -> Result<String> {
        match self {
            Self::Accepted { message_id } => Ok(message_id),
            Self::Rejected { reason, retryable } => {
                Err(RejectedSubmission { reason, retryable }.into())
            }
            Self::Uncertain { reason } => Err(UncertainSubmission(reason).into()),
        }
    }
}

pub(crate) async fn submit_request<T: Serialize + ?Sized>(
    client: &Client,
    endpoint: &Url,
    token: &str,
    payload: &T,
) -> Submission {
    let mut response=match client.post(endpoint.clone()).header("X-Postmark-Server-Token",token).header("Accept","application/json").json(payload).send().await {
   Ok(response)=>response,
   Err(error) if error.is_builder()=>return Submission::Rejected{reason:"Email provider request configuration is invalid".into(),retryable:false},
   Err(error) if error.is_connect()=>return Submission::Rejected{reason:"Unable to connect to email provider".into(),retryable:true},
   Err(_)=>return Submission::Uncertain{reason:"Email submission ended without an acceptance or rejection; reconcile before retrying".into()}
  };
    let status = response.status();
    if status.as_u16() == 429 {
        return Submission::Rejected {
            reason: "Email provider rate limit reached".into(),
            retryable: true,
        };
    }
    if status.as_u16() == 408 || status.is_server_error() {
        return Submission::Uncertain {
            reason: format!(
                "Email provider returned HTTP {}; acceptance is unknown",
                status.as_u16()
            ),
        };
    }
    if !status.is_success() {
        return Submission::Rejected {
            reason: format!(
                "Email provider rejected submission (HTTP {})",
                status.as_u16()
            ),
            retryable: false,
        };
    }
    let mut bytes = Vec::new();
    loop {
        match response.chunk().await {
            Ok(Some(chunk)) if bytes.len().saturating_add(chunk.len()) <= 65536 => {
                bytes.extend_from_slice(&chunk)
            }
            Ok(None) => break,
            _ => {
                return Submission::Uncertain {
                    reason: "Email provider acceptance response was unreadable or exceeded 64 KiB"
                        .into(),
                };
            }
        }
    }
    let response: Value = match serde_json::from_slice(&bytes) {
        Ok(value) => value,
        Err(_) => {
            return Submission::Uncertain {
                reason: "Email provider returned an invalid acceptance response".into(),
            };
        }
    };
    match response["ErrorCode"].as_i64() {
        Some(0) => match response["MessageID"]
            .as_str()
            .filter(|id| !id.trim().is_empty() && id.len() <= 255 && !id.contains('\0'))
        {
            Some(id) => Submission::Accepted {
                message_id: id.to_string(),
            },
            None => Submission::Uncertain {
                reason: "Email provider did not return an accepted message ID".into(),
            },
        },
        Some(code) => Submission::Rejected {
            reason: format!("Email provider rejected submission (error code {code})"),
            retryable: false,
        },
        None => Submission::Uncertain {
            reason: "Email provider did not acknowledge acceptance or rejection".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
    use serde_json::json;
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };
    fn sender() -> Identity {
        Identity {
            name: "Fixture Sender, Jr.".into(),
            email: "sender@example.test".into(),
        }
    }
    fn message() -> Message {
        Message {
            to: "recipient@example.test".into(),
            subject: "Fixture subject".into(),
            body: "<p>Fixture body</p>".into(),
            plain_text: false,
            text_body: None,
            from: None,
            reply_to: Some(Identity {
                name: "Replies".into(),
                email: "reply@example.test".into(),
            }),
            unsubscribe_url: "http://127.0.0.1:5274/api/crm-mail/unsubscribe/fixture-token".into(),
            metadata: BTreeMap::from([("delivery_id".into(), "fixture-delivery".into())]),
        }
    }
    #[test]
    fn adapter_configuration_is_explicit_and_loopback_only() {
        assert!(CampaignMailer::new("production", "", sender(), "broadcast", None).is_err());
        assert!(CampaignMailer::new("production", "fixture-token", sender(), "", None).is_err());
        for (environment, url) in [
            ("production", "http://127.0.0.1:1/email"),
            ("development", "http://example.test/email"),
            ("development", "http://192.168.1.1/email"),
            ("development", "http://user:password@127.0.0.1/email"),
            ("development", "http://127.0.0.1/email?token=x"),
        ] {
            assert!(endpoint(environment, Some(url)).is_err(), "{url}");
        }
        assert!(endpoint("development", Some("http://127.0.0.1:19020/email")).is_ok());
        assert!(endpoint("development", Some("http://[::1]:19020/email")).is_ok());
    }
    #[tokio::test]
    async fn controlled_provider_requires_acknowledgment_and_preserves_broadcast_payload() {
        #[derive(Clone)]
        struct Adapter {
            calls: Arc<AtomicUsize>,
            payloads: Arc<Mutex<Vec<Value>>>,
        }
        async fn provider(
            State(state): State<Adapter>,
            headers: axum::http::HeaderMap,
            Json(payload): Json<Value>,
        ) -> (StatusCode, String) {
            assert_eq!(
                headers["x-postmark-server-token"],
                "controlled-adapter-token"
            );
            state.payloads.lock().unwrap().push(payload);
            match state.calls.fetch_add(1, Ordering::SeqCst) {
                0 => (
                    StatusCode::OK,
                    json!({"ErrorCode":10,"MessageID":"not-accepted"}).to_string(),
                ),
                1 => (StatusCode::OK, json!({"ErrorCode":0}).to_string()),
                2 => (StatusCode::TOO_MANY_REQUESTS, "".into()),
                3 => (StatusCode::INTERNAL_SERVER_ERROR, "".into()),
                4 => (StatusCode::OK, "invalid-json".into()),
                _ => (
                    StatusCode::OK,
                    json!({"ErrorCode":0,"MessageID":"controlled-message-id"}).to_string(),
                ),
            }
        }
        let adapter = Adapter {
            calls: Arc::new(AtomicUsize::new(0)),
            payloads: Arc::new(Mutex::new(Vec::new())),
        };
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/email", listener.local_addr().unwrap());
        let app = Router::new()
            .route("/email", post(provider))
            .with_state(adapter.clone());
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let mailer = CampaignMailer::new(
            "development",
            "controlled-adapter-token",
            sender(),
            "fixture-broadcast",
            Some(&url),
        )
        .unwrap();
        assert!(matches!(
            mailer.submit(message()).await,
            Submission::Rejected {
                retryable: false,
                ..
            }
        ));
        assert!(matches!(
            mailer.submit(message()).await,
            Submission::Uncertain { .. }
        ));
        assert!(matches!(
            mailer.submit(message()).await,
            Submission::Rejected {
                retryable: true,
                ..
            }
        ));
        assert!(matches!(
            mailer.submit(message()).await,
            Submission::Uncertain { .. }
        ));
        assert!(matches!(
            mailer.submit(message()).await,
            Submission::Uncertain { .. }
        ));
        assert_eq!(
            mailer.submit(message()).await,
            Submission::Accepted {
                message_id: "controlled-message-id".into()
            }
        );
        let mut plain = message();
        plain.plain_text = true;
        plain.body = "Literal <plain> text".into();
        assert!(matches!(
            mailer.submit(plain).await,
            Submission::Accepted { .. }
        ));
        let mut invalid = message();
        invalid.subject = "Invalid\r\nsubject".into();
        assert!(matches!(
            mailer.submit(invalid).await,
            Submission::Rejected {
                retryable: false,
                ..
            }
        ));
        assert_eq!(adapter.calls.load(Ordering::SeqCst), 7);
        let payloads = adapter.payloads.lock().unwrap();
        let html = &payloads[5];
        assert_eq!(
            html["From"],
            "\"Fixture Sender, Jr.\" <sender@example.test>"
        );
        assert_eq!(html["ReplyTo"], "\"Replies\" <reply@example.test>");
        assert_eq!(html["MessageStream"], "fixture-broadcast");
        assert_eq!(html["HtmlBody"], "<p>Fixture body</p>");
        assert_eq!(html["Metadata"]["delivery_id"], "fixture-delivery");
        assert_eq!(html["Headers"][1]["Name"], "List-Unsubscribe-Post");
        assert_eq!(html["TrackOpens"], true);
        assert!(payloads[6].get("HtmlBody").is_none());
        assert_eq!(payloads[6]["TextBody"], "Literal <plain> text");
        server.abort();
    }
}

//! Email service - Postmark Integration
//! ICT 11+ Principal Engineer Grade
//!
//! Provides email sending functionality via Postmark API:
//! - Email verification emails
//! - Welcome emails
//! - Password reset emails
//! - Newsletter and transactional emails

use super::campaign_mailer::{
    Identity, RejectedSubmission, Submission, UncertainSubmission, submit_request,
};
use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use sqlx::PgPool;

#[cfg(test)]
mod provider_tests {
    use super::*;
    use axum::{Json, Router, extract::State, routing::post};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn capability_values_are_redacted_only_in_logs() {
        let original = serde_json::json!({"name":"Fixture recipient","verification_link":"https://example.test/verify?token=fixture-private", "nested":{"token":"fixture-private","expires":60},"app_url":"https://example.test"});
        let logged = redacted_model(&original);
        assert_eq!(logged["verification_link"], "[redacted]");
        assert_eq!(logged["nested"]["token"], "[redacted]");
        assert_eq!(logged["nested"]["expires"], 60);
        assert_eq!(logged["name"], original["name"]);
        assert_eq!(logged["app_url"], original["app_url"]);
        assert!(
            original["verification_link"]
                .as_str()
                .unwrap()
                .contains("fixture-private")
        );
        assert!(!logged.to_string().contains("fixture-private"));
    }

    #[tokio::test]
    async fn missing_configuration_is_not_reported_as_success() {
        let sender = EmailService::new("", "sender@example.test", "http://127.0.0.1");
        assert!(!sender.is_enabled());
        assert!(!sender.is_token_configured());
        assert!(
            sender
                .send("recipient@example.test", "Fixture", "<p>Fixture</p>", None)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn legacy_raw_send_requires_acknowledgment_and_preserves_transactional_fields() {
        async fn provider(
            State(calls): State<Arc<AtomicUsize>>,
            Json(payload): Json<serde_json::Value>,
        ) -> Json<serde_json::Value> {
            assert_eq!(
                payload["From"],
                "\"Revolution Trading Pros\" <sender@example.test>"
            );
            assert_eq!(payload["To"], "recipient@example.test");
            assert_eq!(payload["TextBody"], "Fixture text");
            assert_eq!(payload["MessageStream"], "outbound");
            Json(match calls.fetch_add(1, Ordering::SeqCst) {
                0 => serde_json::json!({"ErrorCode":10,"MessageID":"not-accepted"}),
                1 => serde_json::json!({"ErrorCode":0}),
                _ => {
                    serde_json::json!({"ErrorCode":0,"MessageID":"controlled-transactional-message"})
                }
            })
        }
        let calls = Arc::new(AtomicUsize::new(0));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/email", listener.local_addr().unwrap());
        let app = Router::new()
            .route("/email", post(provider))
            .with_state(calls.clone());
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let mut sender = EmailService::new(
            "controlled-adapter-token",
            "sender@example.test",
            "http://127.0.0.1",
        );
        sender.endpoint = Ok(reqwest::Url::parse(&endpoint).unwrap());
        assert!(
            sender
                .send(
                    "recipient@example.test",
                    "Fixture",
                    "<p>Fixture</p>",
                    Some("Fixture text")
                )
                .await
                .is_err()
        );
        let uncertain = sender
            .send(
                "recipient@example.test",
                "Fixture",
                "<p>Fixture</p>",
                Some("Fixture text"),
            )
            .await
            .unwrap_err();
        assert!(uncertain.downcast_ref::<UncertainSubmission>().is_some());
        sender
            .send(
                "recipient@example.test",
                "Fixture",
                "<p>Fixture</p>",
                Some("Fixture text"),
            )
            .await
            .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 3);
        server.abort();
    }
}

/// Build the shared Postmark HTTP client with a request timeout.
///
/// 30s request timeout plus a connect timeout, via the shared builder, which
/// fails loudly instead of falling back to an untimed client.
fn build_http_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(5))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("static bounded email client must build")
}

/// Capability-bearing confirmation/reset values are delivered but never copied to logs.
fn redacted_model(model: &serde_json::Value) -> serde_json::Value {
    match model {
        serde_json::Value::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .map(|(key, value)| {
                    let normalized = key.to_ascii_lowercase();
                    let secret = normalized.contains("token")
                        || normalized.contains("password")
                        || normalized.contains("secret")
                        || normalized.ends_with("_link")
                        || normalized.ends_with("_url") && normalized != "app_url";
                    (
                        key.clone(),
                        if secret {
                            serde_json::json!("[redacted]")
                        } else {
                            redacted_model(value)
                        },
                    )
                })
                .collect(),
        ),
        serde_json::Value::Array(items) => {
            serde_json::Value::Array(items.iter().map(redacted_model).collect())
        }
        value => value.clone(),
    }
}

/// Transactional email submission with persisted, truthful provider states.
/// Missing configuration is unavailable; only acknowledged acceptance is sent.
#[derive(Clone)]
pub struct EmailService {
    client: Client,
    endpoint: std::result::Result<reqwest::Url, String>,
    /// Absent credentials cause explicit unavailable errors without HTTP requests.
    token: Option<String>,
    from_email: String,
    from_name: String,
    app_url: String,
    smtp: Option<super::smtp::SmtpSettings>,
    runtime: Option<RuntimeConfiguration>,
}

#[derive(Clone)]
struct RuntimeConfiguration {
    pool: PgPool,
    encryption_key: String,
    environment: String,
}

#[derive(Serialize)]
struct PostmarkEmail {
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "To")]
    to: String,
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "HtmlBody")]
    html_body: String,
    #[serde(rename = "TextBody")]
    text_body: Option<String>,
    #[serde(rename = "MessageStream")]
    message_stream: String,
}

/// Postmark `email/withTemplate` request body.
#[derive(Serialize)]
struct PostmarkTemplateEmail<'a> {
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "To")]
    to: &'a str,
    #[serde(rename = "TemplateAlias")]
    template_alias: &'a str,
    #[serde(rename = "TemplateModel")]
    template_model: &'a serde_json::Value,
    #[serde(rename = "MessageStream")]
    message_stream: &'static str,
}

impl EmailService {
    /// Construct from an explicit token string. Caller decides whether
    /// to pass `Some` or `None`. Batch 6 prefers `from_env()` which
    /// applies the empty-string-is-None rule automatically.
    pub fn new(token: &str, from_email: &str, app_url: &str) -> Self {
        let token = if token.is_empty() {
            None
        } else {
            Some(token.to_string())
        };
        Self {
            client: build_http_client(),
            endpoint: super::campaign_mailer::endpoint(
                &std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into()),
                std::env::var("POSTMARK_TEST_ENDPOINT").ok().as_deref(),
            )
            .map_err(|error| error.to_string()),
            token,
            from_email: from_email.to_string(),
            from_name: "Revolution Trading Pros".to_string(),
            app_url: app_url.trim_end_matches('/').to_string(),
            smtp: None,
            runtime: None,
        }
    }

    /// Bind application instances to durable configuration. Explicit standalone
    /// test clients remain independent unless the caller deliberately binds them.
    pub fn with_runtime(mut self, pool: PgPool, config: &crate::config::Config) -> Self {
        self.runtime = Some(RuntimeConfiguration {
            pool,
            encryption_key: config.credentials_encryption_key.clone(),
            environment: config.environment.clone(),
        });
        self
    }
    pub async fn resolved(&self) -> Result<Self> {
        let mut service = self.clone();
        service.runtime = None;
        if let Some(runtime) = &self.runtime {
            let stored = super::smtp::load_from(
                &runtime.pool,
                &runtime.encryption_key,
                &runtime.environment,
            )
            .await
            .map_err(|_| anyhow::anyhow!("Email configuration storage is unavailable"))?;
            if let Some(stored) = stored {
                if stored.value["status"] != "verified"
                    || stored.value["last_verified_at"]
                        .as_str()
                        .is_none_or(|value| chrono::DateTime::parse_from_rfc3339(value).is_err())
                {
                    anyhow::bail!("Saved SMTP configuration requires a successful connection check")
                }
                service.from_email = stored.settings.from_address.clone();
                service.from_name = stored.settings.from_name.clone();
                service.smtp = Some(stored.settings);
            }
        }
        Ok(service)
    }
    pub fn provider(&self) -> &'static str {
        if self.smtp.is_some() {
            "smtp"
        } else {
            "postmark"
        }
    }
    pub(crate) fn smtp_settings(&self) -> Option<&super::smtp::SmtpSettings> {
        self.smtp.as_ref()
    }
    pub fn from_name(&self) -> &str {
        &self.from_name
    }

    /// Read explicit credentials and sender aliases; use the validated local
    /// adapter only in development. No token or sender is fabricated.
    pub fn from_env(app_url: &str) -> Self {
        fn configured(keys: &[&str]) -> String {
            keys.iter()
                .find_map(|key| {
                    std::env::var(key)
                        .ok()
                        .filter(|v| !v.trim().is_empty())
                        .map(|v| v.trim().to_string())
                })
                .unwrap_or_default()
        }
        let token = configured(&["POSTMARK_TOKEN", "POSTMARK_API_KEY"]);
        let from = configured(&["POSTMARK_FROM_EMAIL", "EMAIL_FROM_ADDRESS", "FROM_EMAIL"]);
        let mut service = Self::new(&token, &from, app_url);
        let name = configured(&["EMAIL_FROM_NAME", "FROM_NAME"]);
        if !name.is_empty() {
            service.from_name = name;
        }
        service
    }
    fn configuration(&self) -> Result<(&str, &reqwest::Url)> {
        let token = self
            .token
            .as_deref()
            .filter(|token| {
                !token.is_empty()
                    && token.len() <= 1024
                    && token.bytes().all(|byte| byte.is_ascii_graphic())
            })
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Email provider unavailable: configure POSTMARK_TOKEN or POSTMARK_API_KEY"
                )
            })?;
        Identity {
            name: self.from_name.clone(),
            email: self.from_email.clone(),
        }
        .formatted()?;
        let endpoint = self
            .endpoint
            .as_ref()
            .map_err(|error| anyhow::anyhow!("Email provider configuration is invalid: {error}"))?;
        Ok((token, endpoint))
    }
    pub fn is_token_configured(&self) -> bool {
        self.token.is_some()
    }
    pub fn configuration_error(&self) -> Option<String> {
        if self.smtp.is_some() {
            return None;
        }
        self.configuration().err().map(|error| error.to_string())
    }

    /// True when explicit credentials, sender and endpoint are usable. Used by the admin
    /// `/api/admin/email/status` diagnostic.
    pub fn is_enabled(&self) -> bool {
        self.smtp.is_some() || self.configuration().is_ok()
    }

    /// Configured from-email. Read by `/api/admin/email/status`.
    pub fn from_email(&self) -> &str {
        &self.from_email
    }

    /// Persist a submission record, then store unavailable, rejected, unknown
    /// or acknowledged acceptance. Never silently discard receipt-write errors.
    pub async fn send_transactional(
        &self,
        pool: &PgPool,
        to: &str,
        template_alias: &str,
        model: serde_json::Value,
    ) -> Result<()> {
        crate::utils::errors::validate_email(to)
            .map_err(|_| anyhow::anyhow!("Invalid recipient email"))?;
        if template_alias.is_empty()
            || template_alias.chars().count() > 100
            || template_alias.contains(['\r', '\n', '\0'])
        {
            anyhow::bail!("Invalid email template alias");
        }
        let resolved = self.resolved().await;
        let provider = resolved
            .as_ref()
            .map(Self::provider)
            .unwrap_or("configuration");
        let logged_model = redacted_model(&model);
        let log_id:i64=sqlx::query_scalar("INSERT INTO email_logs(email,to_email,subject,template_type,template_alias,model,status,provider,queued_at,created_at) VALUES($1,$1,$2,LEFT($3,50),$3,$4,'queued',$5,NOW(),NOW()) RETURNING id")
            .bind(to).bind(format!("[template:{template_alias}]")).bind(template_alias).bind(logged_model).bind(provider).fetch_one(pool).await?;
        let service = match resolved {
            Ok(service) => service,
            Err(error) => {
                Self::unavailable(pool, log_id, &error).await?;
                return Err(error);
            }
        };
        let outcome = if let Some(smtp) = &service.smtp {
            let brand = if service.from_name.is_empty() {
                &service.from_email
            } else {
                &service.from_name
            };
            let rendered = match super::transactional_templates::load(
                pool,
                template_alias,
                model,
                &service.app_url,
                brand,
            )
            .await
            {
                Ok(rendered) => rendered,
                Err(error) => {
                    let database = error
                        .chain()
                        .any(|cause| cause.downcast_ref::<sqlx::Error>().is_some());
                    if database {
                        let error = anyhow::anyhow!("Email template storage is unavailable");
                        Self::unavailable(pool, log_id, &error).await?;
                        return Err(error);
                    }
                    return Self::record_outcome(
                        pool,
                        log_id,
                        Submission::Rejected {
                            reason: error.to_string(),
                            retryable: false,
                        },
                    )
                    .await
                    .map(|_| ());
                }
            };
            let from = Identity {
                name: service.from_name.clone(),
                email: service.from_email.clone(),
            };
            super::smtp_message::submit(
                smtp,
                super::smtp_message::Message {
                    to,
                    subject: &rendered.subject,
                    html: Some(&rendered.html),
                    text: rendered.text.as_deref(),
                    from: &from,
                    reply_to: None,
                    unsubscribe: None,
                },
            )
            .await
        } else {
            let (token, endpoint) = match service.configuration() {
                Ok(configuration) => configuration,
                Err(error) => {
                    Self::unavailable(pool, log_id, &error).await?;
                    return Err(error);
                }
            };
            let mut endpoint = endpoint.clone();
            endpoint.set_path(&format!(
                "{}/withTemplate",
                endpoint.path().trim_end_matches('/')
            ));
            let from = Identity {
                name: service.from_name.clone(),
                email: service.from_email.clone(),
            }
            .formatted()?;
            let body = PostmarkTemplateEmail {
                from,
                to,
                template_alias,
                template_model: &model,
                message_stream: "outbound",
            };
            submit_request(&service.client, &endpoint, token, &body).await
        };
        Self::record_outcome(pool, log_id, outcome)
            .await
            .map(|_| ())
    }
    async fn unavailable(pool: &PgPool, id: i64, error: &anyhow::Error) -> Result<()> {
        sqlx::query("UPDATE email_logs SET status='unavailable',sent_at=NULL,error=$2,error_message=$2 WHERE id=$1")
            .bind(id).bind(error.to_string()).execute(pool).await?;
        Ok(())
    }
    async fn record_outcome(pool: &PgPool, id: i64, outcome: Submission) -> Result<String> {
        match outcome {
            Submission::Accepted { message_id } => {
                if sqlx::query("UPDATE email_logs SET status='sent',sent_at=NOW(),provider_message_id=$2,error=NULL,error_message=NULL WHERE id=$1")
                    .bind(id).bind(&message_id).execute(pool).await.is_err() {
                    return Err(UncertainSubmission(format!("Provider accepted message {message_id}, but its local receipt could not be persisted; reconcile before retrying")).into());
                }
                Ok(message_id)
            }
            Submission::Rejected { reason, retryable } => {
                sqlx::query("UPDATE email_logs SET status='failed',sent_at=NULL,error=$2,error_message=$2 WHERE id=$1").bind(id).bind(&reason).execute(pool).await?;
                Err(RejectedSubmission { reason, retryable }.into())
            }
            Submission::Uncertain { reason } => {
                sqlx::query("UPDATE email_logs SET status='delivery_unknown',sent_at=NULL,error=$2,error_message=$2 WHERE id=$1").bind(id).bind(&reason).execute(pool).await?;
                Err(UncertainSubmission(reason).into())
            }
        }
    }
    /// Record runtime-bound raw submissions and require the provider's real acknowledgment.
    pub async fn send_receipt(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: Option<&str>,
    ) -> Result<String> {
        crate::utils::errors::validate_email(to)
            .map_err(|_| anyhow::anyhow!("Invalid recipient email"))?;
        if subject.trim().is_empty()
            || subject.chars().count() > 500
            || subject.contains(['\r', '\n', '\0'])
            || html.contains('\0')
            || html.len() > 4_000_000
            || text.is_some_and(|text| text.len() > 4_000_000 || text.contains('\0'))
        {
            anyhow::bail!("Invalid email subject or body");
        }
        let resolved = self.resolved().await;
        let provider = resolved
            .as_ref()
            .map(Self::provider)
            .unwrap_or("configuration");
        let log = if let Some(runtime) = &self.runtime {
            Some(sqlx::query_scalar::<_,i64>("INSERT INTO email_logs(email,to_email,subject,template_type,status,provider,queued_at,created_at) VALUES($1,$1,$2,'raw','queued',$3,NOW(),NOW()) RETURNING id")
                .bind(to).bind(subject).bind(provider).fetch_one(&runtime.pool).await?)
        } else {
            None
        };
        let service = match resolved {
            Ok(service) => service,
            Err(error) => {
                if let (Some(runtime), Some(id)) = (&self.runtime, log) {
                    Self::unavailable(&runtime.pool, id, &error).await?;
                }
                return Err(error);
            }
        };
        let outcome = if let Some(smtp) = &service.smtp {
            let from = Identity {
                name: service.from_name.clone(),
                email: service.from_email.clone(),
            };
            super::smtp_message::submit(
                smtp,
                super::smtp_message::Message {
                    to,
                    subject,
                    html: Some(html),
                    text,
                    from: &from,
                    reply_to: None,
                    unsubscribe: None,
                },
            )
            .await
        } else {
            let (token, endpoint) = match service.configuration() {
                Ok(configuration) => configuration,
                Err(error) => {
                    if let (Some(runtime), Some(id)) = (&self.runtime, log) {
                        Self::unavailable(&runtime.pool, id, &error).await?;
                    }
                    return Err(error);
                }
            };
            let from = Identity {
                name: service.from_name.clone(),
                email: service.from_email.clone(),
            }
            .formatted()?;
            let email = PostmarkEmail {
                from,
                to: to.into(),
                subject: subject.into(),
                html_body: html.into(),
                text_body: text.map(str::to_owned),
                message_stream: "outbound".into(),
            };
            submit_request(&service.client, endpoint, token, &email).await
        };
        if let (Some(runtime), Some(id)) = (&self.runtime, log) {
            Self::record_outcome(&runtime.pool, id, outcome).await
        } else {
            outcome.into_result()
        }
    }
    pub async fn send(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text: Option<&str>,
    ) -> Result<()> {
        self.send_receipt(to, subject, html, text).await.map(|_| ())
    }

    /// Send email verification email
    pub async fn send_verification_email(&self, to: &str, name: &str, token: &str) -> Result<()> {
        let verification_url = format!("{}/verify-email?token={}", self.app_url, token);

        let subject = "Verify Your Email - Revolution Trading Pros";

        let html = Self::email_template(
            "Verify Your Email",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Thank you for registering with Revolution Trading Pros! To complete your registration 
                    and access all features, please verify your email address by clicking the button below:
                </p>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}" 
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669); 
                              color: white; text-decoration: none; padding: 16px 32px; 
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Verify Email Address
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    Or copy and paste this link into your browser:
                </p>
                <p style="font-size: 14px; color: #10b981; word-break: break-all;">
                    {}
                </p>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    This link will expire in <strong>24 hours</strong>.
                </p>
                <p style="font-size: 14px; color: #6b7280; margin-top: 16px;">
                    If you didn't create an account with Revolution Trading Pros, you can safely ignore this email.
                </p>
                "#,
                html_escape(name),
                verification_url,
                verification_url
            ),
        );

        let text = format!(
            "Hi {name},\n\n\
            Thank you for registering with Revolution Trading Pros!\n\n\
            Please verify your email by visiting this link:\n{verification_url}\n\n\
            This link will expire in 24 hours.\n\n\
            If you didn't create an account, you can safely ignore this email.\n\n\
            - The Revolution Trading Pros Team"
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send welcome email (after verification)
    pub async fn send_welcome_email(&self, to: &str, name: &str) -> Result<()> {
        let subject = "Welcome to Revolution Trading Pros! 🎉";
        let dashboard_url = format!("{}/dashboard", self.app_url);

        let html = Self::email_template(
            "Welcome to the Revolution!",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your email has been verified and your account is now fully activated! 
                    Welcome to the Revolution Trading Pros community.
                </p>
                <div style="background: linear-gradient(135deg, #f0fdf4, #dcfce7); 
                            border-radius: 16px; padding: 24px; margin: 24px 0;">
                    <h3 style="color: #166534; margin: 0 0 16px 0; font-size: 18px;">
                        🚀 Get Started
                    </h3>
                    <ul style="color: #15803d; margin: 0; padding-left: 20px; line-height: 2;">
                        <li>Access live trading rooms</li>
                        <li>Explore professional trading courses</li>
                        <li>Set up real-time trading alerts</li>
                        <li>Join our Discord community</li>
                    </ul>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}" 
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669); 
                              color: white; text-decoration: none; padding: 16px 32px; 
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Go to Dashboard
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    If you have any questions, our support team is here to help.
                </p>
                "#,
                html_escape(name),
                dashboard_url
            ),
        );

        let text = format!(
            "Hi {name},\n\n\
            Your email has been verified and your account is now fully activated!\n\n\
            Welcome to the Revolution Trading Pros community.\n\n\
            Get started:\n\
            - Access live trading rooms\n\
            - Explore professional trading courses\n\
            - Set up real-time trading alerts\n\
            - Join our Discord community\n\n\
            Visit your dashboard: {dashboard_url}\n\n\
            - The Revolution Trading Pros Team"
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send password reset email
    pub async fn send_password_reset(&self, to: &str, name: &str, token: &str) -> Result<()> {
        let reset_url = format!(
            "{}/reset-password?token={}&email={}",
            self.app_url,
            token,
            urlencoding::encode(to)
        );

        let subject = "Reset Your Password - Revolution Trading Pros";

        let html = Self::email_template(
            "Reset Your Password",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    We received a request to reset your password for your Revolution Trading Pros account. 
                    Click the button below to create a new password:
                </p>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}" 
                       style="display: inline-block; background: linear-gradient(135deg, #f59e0b, #d97706); 
                              color: white; text-decoration: none; padding: 16px 32px; 
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(245, 158, 11, 0.4);">
                        Reset Password
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    Or copy and paste this link into your browser:
                </p>
                <p style="font-size: 14px; color: #f59e0b; word-break: break-all;">
                    {}
                </p>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    This link will expire in <strong>1 hour</strong>.
                </p>
                <div style="background: #fef3c7; border-left: 4px solid #f59e0b; 
                            padding: 16px; margin-top: 24px; border-radius: 0 8px 8px 0;">
                    <p style="color: #92400e; margin: 0; font-size: 14px;">
                        <strong>⚠️ Security Notice:</strong> If you didn't request this password reset, 
                        please ignore this email. Your password will remain unchanged.
                    </p>
                </div>
                "#,
                html_escape(name),
                reset_url,
                reset_url
            ),
        );

        let text = format!(
            "Hi {name},\n\n\
            We received a request to reset your password.\n\n\
            Click this link to reset your password:\n{reset_url}\n\n\
            This link will expire in 1 hour.\n\n\
            If you didn't request this, please ignore this email.\n\n\
            - The Revolution Trading Pros Team"
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send payment failed email
    /// ICT 7 Fix: Added missing payment failed notification
    pub async fn send_payment_failed_email(
        &self,
        to: &str,
        name: &str,
        subscription_name: &str,
        retry_url: &str,
    ) -> Result<()> {
        let subject = "Payment Failed - Action Required";
        let billing_url = format!("{}/dashboard/account/billing", self.app_url);

        let html = Self::email_template(
            "Payment Failed",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px; color: #ef4444;">&#9888;</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    We were unable to process the payment for your <strong>{}</strong> subscription.
                </p>
                <div style="background: #fef2f2; border: 1px solid #fecaca; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #991b1b; margin: 0; font-size: 14px;">
                        <strong>What this means:</strong> Your subscription access may be limited
                        until we can successfully process your payment.
                    </p>
                </div>
                <div style="background: #f3f4f6; border-radius: 12px; padding: 20px; margin: 24px 0;">
                    <p style="color: #374151; margin: 0 0 12px 0; font-size: 14px;">
                        <strong>Common reasons for payment failure:</strong>
                    </p>
                    <ul style="color: #6b7280; margin: 0; padding-left: 20px; font-size: 14px;">
                        <li>Insufficient funds</li>
                        <li>Card expired or cancelled</li>
                        <li>Incorrect billing information</li>
                        <li>Card issuer declined the transaction</li>
                    </ul>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #ef4444, #dc2626);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(239, 68, 68, 0.4);">
                        Update Payment Method
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px; text-align: center;">
                    Need help? Contact our support team at support@revolutiontradingpros.com
                </p>
                "#,
                html_escape(name),
                html_escape(subscription_name),
                billing_url
            ),
        );

        let text = format!(
            "Hi {name},\n\n\
            We were unable to process the payment for your {subscription_name} subscription.\n\n\
            What this means: Your subscription access may be limited until we can successfully process your payment.\n\n\
            Common reasons for payment failure:\n\
            - Insufficient funds\n\
            - Card expired or cancelled\n\
            - Incorrect billing information\n\
            - Card issuer declined the transaction\n\n\
            Please update your payment method: {billing_url}\n\n\
            Need help? Contact support@revolutiontradingpros.com\n\n\
            - The Revolution Trading Pros Team"
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SUBSCRIPTION EMAIL NOTIFICATIONS - ICT 7+ Enterprise
    // ═══════════════════════════════════════════════════════════════════════════

    /// Send subscription confirmation email
    #[allow(clippy::too_many_arguments)]
    pub async fn send_subscription_confirmation(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        price: f64,
        billing_cycle: &str,
        has_trial: bool,
        trial_days: i32,
    ) -> Result<()> {
        let subject = format!("Subscription Confirmed - {plan_name}");
        let dashboard_url = format!("{}/my/subscriptions", self.app_url);

        let trial_notice = if has_trial {
            format!(
                r#"<div style="background: #dbeafe; border-left: 4px solid #3b82f6;
                            padding: 16px; margin: 24px 0; border-radius: 0 8px 8px 0;">
                    <p style="color: #1e40af; margin: 0; font-size: 14px;">
                        <strong>Trial Period:</strong> You have a {trial_days}-day free trial.
                        Your card won't be charged until the trial ends.
                    </p>
                </div>"#
            )
        } else {
            String::new()
        };

        let html = Self::email_template(
            "Subscription Confirmed!",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">🎉</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Welcome! Your subscription to <strong>{}</strong> has been confirmed.
                </p>
                {}
                <div style="background: #f0fdf4; border: 1px solid #86efac; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <h3 style="color: #166534; margin: 0 0 12px 0; font-size: 16px;">Subscription Details</h3>
                    <table style="width: 100%; font-size: 14px; color: #374151;">
                        <tr>
                            <td style="padding: 8px 0;">Plan:</td>
                            <td style="padding: 8px 0; text-align: right; font-weight: 600;">{}</td>
                        </tr>
                        <tr>
                            <td style="padding: 8px 0;">Price:</td>
                            <td style="padding: 8px 0; text-align: right; font-weight: 600;">${:.2}/{}</td>
                        </tr>
                    </table>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Manage Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                trial_notice,
                html_escape(plan_name),
                price,
                billing_cycle,
                dashboard_url
            ),
        );

        let text = format!(
            "Hi {},\n\nYour subscription to {} has been confirmed!\n\nPlan: {}\nPrice: ${:.2}/{}\n\n{}\nManage: {}\n\n- The Revolution Trading Pros Team",
            name,
            plan_name,
            plan_name,
            price,
            billing_cycle,
            if has_trial {
                format!("You have a {trial_days}-day free trial.")
            } else {
                String::new()
            },
            dashboard_url
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    /// Send subscription cancellation email
    pub async fn send_subscription_cancelled(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        end_date: &str,
        cancel_immediately: bool,
    ) -> Result<()> {
        let subject = "Subscription Cancelled - Revolution Trading Pros";
        let resubscribe_url = format!("{}/pricing", self.app_url);

        let message = if cancel_immediately {
            "Your subscription has been cancelled immediately."
        } else {
            "Your subscription will remain active until the end of your billing period."
        };

        let html = Self::email_template(
            "Subscription Cancelled",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your <strong>{}</strong> subscription has been cancelled.
                </p>
                <div style="background: #fef3c7; border-left: 4px solid #f59e0b;
                            padding: 16px; margin: 24px 0; border-radius: 0 8px 8px 0;">
                    <p style="color: #92400e; margin: 0; font-size: 14px;">
                        {} {}
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Resubscribe
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                message,
                if !cancel_immediately {
                    format!("<br><strong>Access until:</strong> {end_date}")
                } else {
                    String::new()
                },
                resubscribe_url
            ),
        );

        let text = format!(
            "Hi {name},\n\nYour {plan_name} subscription has been cancelled.\n\n{message}\n\n- The Revolution Trading Pros Team"
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send payment failed with grace period email.
    /// `amount_cents` is integer cents (per architecture standard). Formatted to dollars for display only.
    pub async fn send_payment_failed_with_grace(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        amount_cents: i64,
        grace_period_end: &str,
        retry_count: i32,
    ) -> Result<()> {
        let amount = amount_cents as f64 / 100.0; // display only

        let subject = "Action Required: Payment Failed";
        let update_url = format!("{}/my/subscriptions", self.app_url);

        let urgency = if retry_count >= 3 {
            "FINAL NOTICE: Your subscription will be cancelled if payment is not received."
        } else {
            "Please update your payment method to avoid losing access."
        };

        let html = Self::email_template(
            "Payment Failed",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">⚠️</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    We were unable to process your payment of <strong>${:.2}</strong> for <strong>{}</strong>.
                </p>
                <div style="background: #fef2f2; border: 1px solid #fecaca; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #991b1b; margin: 0; font-size: 14px;">
                        <strong>{}</strong><br><br>
                        Grace period ends: <strong>{}</strong>
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #ef4444, #dc2626);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Update Payment Method
                    </a>
                </div>
                "#,
                html_escape(name),
                amount,
                html_escape(plan_name),
                urgency,
                grace_period_end,
                update_url
            ),
        );

        let text = format!(
            "Hi {name},\n\nPayment of ${amount:.2} for {plan_name} failed.\n\n{urgency}\nGrace period ends: {grace_period_end}\n\nUpdate payment: {update_url}\n\n- The Revolution Trading Pros Team"
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send renewal reminder email.
    /// `amount_cents` is integer cents (per architecture standard). Formatted to dollars for display.
    pub async fn send_renewal_reminder(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        amount_cents: i64,
        renewal_date: &str,
    ) -> Result<()> {
        let amount = amount_cents as f64 / 100.0; // display only
        let subject = format!("Subscription Renewal Reminder - {plan_name}");
        let manage_url = format!("{}/my/subscriptions", self.app_url);

        let html = Self::email_template(
            "Renewal Reminder",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your <strong>{}</strong> subscription will renew on <strong>{}</strong> for <strong>${:.2}</strong>.
                </p>
                <div style="background: #f0f9ff; border: 1px solid #bae6fd; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #0369a1; margin: 0; font-size: 14px;">
                        No action is required. Your subscription will automatically renew.
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Manage Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                renewal_date,
                amount,
                manage_url
            ),
        );

        let text = format!(
            "Hi {name},\n\nYour {plan_name} subscription will renew on {renewal_date} for ${amount:.2}.\n\nManage: {manage_url}\n\n- The Revolution Trading Pros Team"
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    /// Send trial ending soon notification
    pub async fn send_trial_ending_soon(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        trial_end_date: &str,
        price_cents: i64,
        billing_cycle: &str,
    ) -> Result<()> {
        let price = price_cents as f64 / 100.0; // display only
        let subject = "Your Trial Ends Soon - Revolution Trading Pros";
        let manage_url = format!("{}/my/subscriptions", self.app_url);

        let html = Self::email_template(
            "Trial Ending Soon",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your free trial for <strong>{}</strong> ends on <strong>{}</strong>.
                </p>
                <div style="background: #fef3c7; border-left: 4px solid #f59e0b;
                            padding: 16px; margin: 24px 0; border-radius: 0 8px 8px 0;">
                    <p style="color: #92400e; margin: 0; font-size: 14px;">
                        After your trial ends, you'll be charged <strong>${:.2}/{}</strong>.
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Manage Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                trial_end_date,
                price,
                billing_cycle,
                manage_url
            ),
        );

        let text = format!(
            "Hi {name},\n\nYour trial for {plan_name} ends on {trial_end_date}. You'll be charged ${price:.2}/{billing_cycle}.\n\nManage: {manage_url}\n\n- The Revolution Trading Pros Team"
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send subscription upgraded email
    pub async fn send_subscription_upgraded(
        &self,
        to: &str,
        name: &str,
        old_plan: &str,
        new_plan: &str,
        new_price: f64,
        billing_cycle: &str,
    ) -> Result<()> {
        let subject = format!("Subscription Upgraded to {new_plan}");
        let manage_url = format!("{}/my/subscriptions", self.app_url);

        let html = Self::email_template(
            "Subscription Upgraded!",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">🚀</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your subscription has been upgraded from <strong>{}</strong> to <strong>{}</strong>.
                </p>
                <div style="background: #f0fdf4; border: 1px solid #86efac; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #166534; margin: 0; font-size: 14px;">
                        New price: <strong>${:.2}/{}</strong>
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        View Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(old_plan),
                html_escape(new_plan),
                new_price,
                billing_cycle,
                manage_url
            ),
        );

        let text = format!(
            "Hi {name},\n\nUpgraded from {old_plan} to {new_plan}. New price: ${new_price:.2}/{billing_cycle}\n\nView: {manage_url}\n\n- The Revolution Trading Pros Team"
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // NEWSLETTER EMAIL FUNCTIONS - ICT 7+ Double Opt-In & GDPR Compliance
    // ═══════════════════════════════════════════════════════════════════════════

    /// Send newsletter confirmation email (double opt-in)
    pub async fn send_newsletter_confirmation(
        &self,
        to: &str,
        name: &str,
        confirm_token: &str,
    ) -> Result<()> {
        let confirm_url = format!(
            "{}/newsletter/confirm?token={}",
            self.app_url, confirm_token
        );

        let subject = "Confirm Your Newsletter Subscription";

        let html = Self::email_template(
            "Confirm Your Subscription",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi{},
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Thank you for subscribing to the Revolution Trading Pros newsletter!
                    Please confirm your subscription by clicking the button below:
                </p>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Confirm Subscription
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    Or copy and paste this link into your browser:
                </p>
                <p style="font-size: 14px; color: #10b981; word-break: break-all;">
                    {}
                </p>
                <div style="background: #f3f4f6; border-radius: 12px; padding: 20px; margin-top: 24px;">
                    <p style="color: #6b7280; margin: 0; font-size: 14px;">
                        <strong>Why confirm?</strong> We use double opt-in to ensure you actually
                        requested this newsletter and to comply with privacy regulations (GDPR).
                    </p>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    If you didn't subscribe to our newsletter, you can safely ignore this email.
                </p>
                "#,
                if name.is_empty() {
                    String::new()
                } else {
                    format!(" <strong>{}</strong>", html_escape(name))
                },
                confirm_url,
                confirm_url
            ),
        );

        let text = format!(
            "Hi{},\n\n\
            Thank you for subscribing to the Revolution Trading Pros newsletter!\n\n\
            Please confirm your subscription by visiting this link:\n{}\n\n\
            If you didn't subscribe, you can safely ignore this email.\n\n\
            - The Revolution Trading Pros Team",
            if name.is_empty() {
                String::new()
            } else {
                format!(" {name}")
            },
            confirm_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send newsletter with unsubscribe link (bulk email)
    pub async fn send_newsletter(
        &self,
        to: &str,
        name: &str,
        subject: &str,
        html_content: &str,
        unsubscribe_token: &str,
    ) -> Result<()> {
        let unsubscribe_url = format!(
            "{}/newsletter/unsubscribe?token={}",
            self.app_url, unsubscribe_token
        );

        // Add unsubscribe footer to content
        let html_with_footer = format!(
            r#"{}
            <div style="border-top: 1px solid #e5e7eb; margin-top: 32px; padding-top: 24px; text-align: center;">
                <p style="font-size: 12px; color: #9ca3af; margin: 0 0 8px 0;">
                    You're receiving this email because you subscribed to Revolution Trading Pros newsletter.
                </p>
                <p style="font-size: 12px; color: #9ca3af; margin: 0;">
                    <a href="{}" style="color: #6b7280; text-decoration: underline;">Unsubscribe</a>
                    &nbsp;|&nbsp;
                    <a href="{}/privacy" style="color: #6b7280; text-decoration: underline;">Privacy Policy</a>
                </p>
            </div>"#,
            html_content, unsubscribe_url, self.app_url
        );

        let full_html = Self::email_template(
            subject,
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi{},
                </p>
                {}
                "#,
                if name.is_empty() {
                    String::new()
                } else {
                    format!(" <strong>{}</strong>", html_escape(name))
                },
                html_with_footer
            ),
        );

        let text = format!(
            "Hi{},\n\n{}\n\n---\nUnsubscribe: {}\n\n- The Revolution Trading Pros Team",
            if name.is_empty() {
                String::new()
            } else {
                format!(" {name}")
            },
            "View this email in your browser for the full content.",
            unsubscribe_url
        );

        self.send(to, subject, &full_html, Some(&text)).await
    }

    /// Send order confirmation email
    pub async fn send_order_confirmation(&self, to: &str, order_number: &str) -> Result<()> {
        let subject = format!("Order Confirmed - #{order_number}");
        let orders_url = format!("{}/dashboard/account/orders", self.app_url);

        let html = Self::email_template(
            "Order Confirmed!",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">✅</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px; text-align: center;">
                    Thank you for your purchase! Your order <strong>#{order_number}</strong> has been confirmed.
                </p>
                <div style="background: #f0fdf4; border: 1px solid #86efac; border-radius: 12px;
                            padding: 20px; margin: 24px 0; text-align: center;">
                    <p style="color: #166534; margin: 0; font-size: 14px;">
                        Your order is now being processed. You will receive access to your purchases shortly.
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{orders_url}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        View Your Orders
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px; text-align: center;">
                    If you have any questions about your order, please contact our support team.
                </p>
                "#
            ),
        );

        let text = format!(
            "Order Confirmed!\n\n\
            Thank you for your purchase! Your order #{order_number} has been confirmed.\n\n\
            Your order is now being processed. You will receive access to your purchases shortly.\n\n\
            View your orders: {orders_url}\n\n\
            - The Revolution Trading Pros Team"
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    /// Generate a beautiful HTML email template wrapper
    fn email_template(title: &str, content: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
</head>
<body style="margin: 0; padding: 0; background-color: #f3f4f6; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;">
    <div style="max-width: 600px; margin: 0 auto; padding: 40px 20px;">
        <!-- Header -->
        <div style="text-align: center; margin-bottom: 32px;">
            <div style="display: inline-block; background: linear-gradient(135deg, #064e3b, #065f46); 
                        padding: 16px 32px; border-radius: 16px;">
                <h1 style="margin: 0; color: #6ee7b7; font-size: 24px; font-weight: 800; letter-spacing: -0.5px;">
                    Revolution Trading Pros
                </h1>
            </div>
        </div>
        
        <!-- Main Card -->
        <div style="background: white; border-radius: 24px; padding: 40px; 
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05), 0 10px 15px rgba(0, 0, 0, 0.1);">
            <h2 style="color: #111827; font-size: 28px; margin: 0 0 24px 0; font-weight: 700;">
                {title}
            </h2>
            {content}
        </div>
        
        <!-- Footer -->
        <div style="text-align: center; margin-top: 32px; padding: 0 20px;">
            <p style="color: #9ca3af; font-size: 14px; margin: 0 0 8px 0;">
                © 2025 Revolution Trading Pros. All rights reserved.
            </p>
            <p style="color: #9ca3af; font-size: 12px; margin: 0;">
                You're receiving this email because you have an account with Revolution Trading Pros.
            </p>
        </div>
    </div>
</body>
</html>"#
        )
    }
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("John & Jane"), "John &amp; Jane");
        assert_eq!(html_escape("Hello \"World\""), "Hello &quot;World&quot;");
    }

    #[test]
    fn test_email_template_generation() {
        let template = EmailService::email_template("Test Title", "<p>Test content</p>");
        assert!(template.contains("Test Title"));
        assert!(template.contains("<p>Test content</p>"));
        assert!(template.contains("Revolution Trading Pros"));
    }
}

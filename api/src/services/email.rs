//! Email service - Postmark

use anyhow::Result;
use reqwest::Client;
use serde::Serialize;

#[derive(Clone)]
#[allow(dead_code)]
pub struct EmailService {
    client: Client,
    token: String,
    from_email: String,
}

#[derive(Serialize)]
#[allow(dead_code)]
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

#[allow(dead_code)]
impl EmailService {
    pub fn new(token: &str, from_email: &str) -> Self {
        Self {
            client: Client::new(),
            token: token.to_string(),
            from_email: from_email.to_string(),
        }
    }

    pub async fn send(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<()> {
        let email = PostmarkEmail {
            from: self.from_email.clone(),
            to: to.to_string(),
            subject: subject.to_string(),
            html_body: html_body.to_string(),
            text_body: text_body.map(|s| s.to_string()),
            message_stream: "outbound".to_string(),
        };

        self.client
            .post("https://api.postmarkapp.com/email")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Postmark-Server-Token", &self.token)
            .json(&email)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn send_welcome(&self, to: &str, name: &str) -> Result<()> {
        let subject = "Welcome to Revolution Trading Pros!";
        let html = format!(
            r#"
            <h1>Welcome, {}!</h1>
            <p>Thank you for joining Revolution Trading Pros.</p>
            <p>Get started by exploring our courses and resources.</p>
            <p>Best regards,<br>The Revolution Trading Pros Team</p>
            "#,
            name
        );
        self.send(to, subject, &html, None).await
    }

    pub async fn send_password_reset(&self, to: &str, reset_url: &str) -> Result<()> {
        let subject = "Reset Your Password";
        let html = format!(
            r#"
            <h1>Password Reset Request</h1>
            <p>Click the link below to reset your password:</p>
            <p><a href="{}">Reset Password</a></p>
            <p>This link expires in 1 hour.</p>
            <p>If you didn't request this, please ignore this email.</p>
            "#,
            reset_url
        );
        self.send(to, subject, &html, None).await
    }
}

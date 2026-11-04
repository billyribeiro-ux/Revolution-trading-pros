//! SMTP messages preserve UTF-8 content using RFC2045/2047 Base64 MIME.
use super::{
    campaign_mailer::{Identity, Submission},
    smtp::SmtpSettings,
};
use anyhow::{Result, anyhow, bail};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use lettre::{AsyncTransport, address::Envelope};

pub struct Message<'a> {
    pub to: &'a str,
    pub subject: &'a str,
    pub html: Option<&'a str>,
    pub text: Option<&'a str>,
    pub from: &'a Identity,
    pub reply_to: Option<&'a Identity>,
    pub unsubscribe: Option<&'a str>,
}
pub struct Prepared {
    pub envelope: Envelope,
    pub bytes: Vec<u8>,
    pub message_id: String,
}
fn words(value: &str) -> String {
    let mut encoded = Vec::new();
    let mut chunk = String::new();
    for character in value.chars() {
        if chunk.len() + character.len_utf8() > 39 {
            encoded.push(format!("=?UTF-8?B?{}?=", STANDARD.encode(chunk.as_bytes())));
            chunk.clear();
        }
        chunk.push(character);
    }
    if !chunk.is_empty() {
        encoded.push(format!("=?UTF-8?B?{}?=", STANDARD.encode(chunk.as_bytes())));
    }
    encoded.join("\r\n ")
}
fn identity(value: &Identity) -> Result<String> {
    value.formatted()?;
    Ok(if value.name.is_empty() {
        format!("<{}>", value.email)
    } else {
        format!("{}\r\n <{}>", words(&value.name), value.email)
    })
}
fn part(kind: &str, body: &str) -> String {
    // Canonical text line endings are encoded as content, not transport syntax.
    let canonical = body
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', "\r\n");
    let encoded = STANDARD.encode(canonical.as_bytes());
    let mut content = format!(
        "Content-Type: text/{kind}; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\n"
    );
    for line in encoded.as_bytes().chunks(76) {
        // Base64 is ASCII by definition.
        content.push_str(std::str::from_utf8(line).expect("Base64 is UTF-8"));
        content.push_str("\r\n");
    }
    content
}
pub fn prepare(message: Message<'_>) -> Result<Prepared> {
    crate::utils::errors::validate_email(message.to)
        .map_err(|_| anyhow!("Invalid SMTP recipient"))?;
    if message.to.contains(['\r', '\n', '\0'])
        || message.subject.trim().is_empty()
        || message.subject.chars().count() > 500
        || message.subject.contains(['\r', '\n', '\0'])
    {
        bail!("SMTP email requires a valid recipient and a single-line subject of 1–500 characters")
    }
    let html = message.html.filter(|body| !body.is_empty());
    let text = message.text.filter(|body| !body.is_empty());
    if html.is_none() && text.is_none()
        || [html, text]
            .iter()
            .flatten()
            .any(|body| body.len() > 4_000_000 || body.contains('\0'))
    {
        bail!("SMTP email body is empty or exceeds the supported limit")
    }
    let from = identity(message.from)?;
    let envelope = Envelope::new(Some(message.from.email.parse()?), vec![message.to.parse()?])?;
    let domain = message
        .from
        .email
        .rsplit_once('@')
        .ok_or_else(|| anyhow!("Invalid SMTP sender"))?
        .1;
    let message_id = format!("{}@{}", uuid::Uuid::new_v4(), domain);
    let mut content = format!(
        "From: {from}\r\nTo: <{}>\r\nSubject: {}\r\nDate: {}\r\nMessage-ID: <{message_id}>\r\nMIME-Version: 1.0\r\n",
        message.to,
        words(message.subject),
        chrono::Utc::now().to_rfc2822()
    );
    if let Some(reply) = message.reply_to {
        content.push_str(&format!("Reply-To: {}\r\n", identity(reply)?));
    }
    if let Some(unsubscribe) = message.unsubscribe {
        let url = reqwest::Url::parse(unsubscribe)?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || unsubscribe.len() > 900
            || unsubscribe.contains(['\r', '\n', '\0', '<', '>'])
        {
            bail!("Invalid SMTP unsubscribe URL")
        }
        content.push_str(&format!(
            "List-Unsubscribe: <{url}>\r\nList-Unsubscribe-Post: List-Unsubscribe=One-Click\r\n"
        ));
    }
    match (text, html) {
        (Some(text), Some(html)) => {
            let boundary = format!("admin-{}", uuid::Uuid::new_v4());
            content.push_str(&format!("Content-Type: multipart/alternative; boundary=\"{boundary}\"\r\n\r\n--{boundary}\r\n{}\r\n--{boundary}\r\n{}\r\n--{boundary}--\r\n",part("plain",text),part("html",html)));
        }
        (Some(body), None) => content.push_str(&part("plain", body)),
        (None, Some(body)) => content.push_str(&part("html", body)),
        (None, None) => unreachable!("validated nonempty content"),
    }
    if content.split("\r\n").any(|line| line.len() > 998) {
        bail!("SMTP header exceeds the supported line length")
    }
    Ok(Prepared {
        envelope,
        bytes: content.into_bytes(),
        message_id,
    })
}
pub async fn submit(settings: &SmtpSettings, message: Message<'_>) -> Submission {
    let prepared = match prepare(message) {
        Ok(prepared) => prepared,
        Err(error) => {
            return Submission::Rejected {
                reason: error.to_string(),
                retryable: false,
            };
        }
    };
    let transport = match settings.transport().await {
        Ok(transport) => transport,
        Err(_) => {
            return Submission::Rejected {
                reason: "SMTP transport is unavailable before submission".into(),
                retryable: true,
            };
        }
    };
    match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        transport.send_raw(&prepared.envelope, &prepared.bytes),
    )
    .await
    {
        // Lettre returns only after the DATA acknowledgment. This ID identifies
        // the actual accepted message; it is not a claim of inbox delivery.
        Ok(Ok(_)) => Submission::Accepted {
            message_id: prepared.message_id,
        },
        Ok(Err(error)) if error.is_permanent() || error.is_transient() => Submission::Rejected {
            reason: "SMTP relay explicitly rejected the submission".into(),
            retryable: error.is_transient(),
        },
        _ => Submission::Uncertain {
            reason: "SMTP acknowledgment was not received; reconcile before retrying".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    fn sender() -> Identity {
        Identity {
            name: "Équipe de démonstration 東京".into(),
            email: "sender@example.test".into(),
        }
    }
    fn message(from: &Identity) -> Message<'_> {
        Message {
            to: "recipient@example.test",
            subject: "Résumé 東京 🧪 — fixture",
            html: Some("<p>Fixture &amp; é</p>"),
            text: Some("Fixture é\nSecond line"),
            from,
            reply_to: Some(from),
            unsubscribe: Some("https://example.test/unsubscribe?token=fixture"),
        }
    }
    #[test]
    fn unicode_headers_keep_complete_characters_and_reject_injection_before_transport() {
        let input = "Résumé 東京 🧪 ".repeat(25);
        let encoded = words(&input);
        let decoded = encoded
            .split("\r\n ")
            .map(|word| {
                assert!(word.len() <= 75);
                let bytes = STANDARD
                    .decode(
                        word.strip_prefix("=?UTF-8?B?")
                            .unwrap()
                            .strip_suffix("?=")
                            .unwrap(),
                    )
                    .unwrap();
                String::from_utf8(bytes)
                    .expect("Every encoded word contains complete UTF-8 characters")
            })
            .collect::<String>();
        assert_eq!(decoded, input);
        let from = sender();
        let prepared = prepare(message(&from)).unwrap();
        let wire = std::str::from_utf8(&prepared.bytes).unwrap();
        assert!(wire.contains("Content-Type: multipart/alternative"));
        assert!(wire.contains("List-Unsubscribe-Post: List-Unsubscribe=One-Click\r\n"));
        assert!(wire.contains("Content-Type: text/plain; charset=utf-8"));
        assert!(wire.contains("Content-Type: text/html; charset=utf-8"));
        assert!(
            wire.split("\r\n")
                .filter(|line| line.contains("=?UTF-8?B?"))
                .all(|line| line.len() <= 76)
        );
        let mut invalid = message(&from);
        invalid.subject = "Fixture\r\nBcc: outsider@example.test";
        assert!(prepare(invalid).is_err());
        let mut invalid = message(&from);
        invalid.unsubscribe = Some("https://example.test/\r\nBcc: outsider@example.test");
        assert!(prepare(invalid).is_err());
    }
    #[tokio::test]
    async fn actual_data_acknowledgment_distinguishes_accepted_rejected_and_unknown() {
        for outcome in ["accepted", "rejected", "unknown"] {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let port = listener.local_addr().unwrap().port();
            let (sent, received) = tokio::sync::oneshot::channel();
            let server = tokio::spawn(async move {
                let (socket, _) = listener.accept().await.unwrap();
                let (read, mut write) = socket.into_split();
                let mut read = BufReader::new(read);
                write
                    .write_all(b"220 fixture.example.test ESMTP\r\n")
                    .await
                    .unwrap();
                let mut content = Vec::new();
                let mut data = false;
                loop {
                    let mut line = String::new();
                    let bytes = tokio::time::timeout(
                        std::time::Duration::from_secs(3),
                        read.read_line(&mut line),
                    )
                    .await
                    .unwrap()
                    .unwrap();
                    if bytes == 0 {
                        break;
                    }
                    if data {
                        if line == ".\r\n" {
                            sent.send(content).unwrap();
                            match outcome {
                                "accepted" => write
                                    .write_all(b"250 2.0.0 queued fixture\r\n")
                                    .await
                                    .unwrap(),
                                "rejected" => write
                                    .write_all(b"550 5.7.0 fixture rejection\r\n")
                                    .await
                                    .unwrap(),
                                _ => write.shutdown().await.unwrap(),
                            }
                            break;
                        }
                        content.extend_from_slice(line.as_bytes());
                        continue;
                    }
                    let reply = if line.starts_with("EHLO ") {
                        "250-fixture.example.test\r\n250 SMTPUTF8\r\n"
                    } else if line.starts_with("MAIL FROM:") || line.starts_with("RCPT TO:") {
                        "250 2.0.0 OK\r\n"
                    } else if line.trim() == "DATA" {
                        data = true;
                        "354 Continue\r\n"
                    } else {
                        panic!("Unexpected SMTP command")
                    };
                    write.write_all(reply.as_bytes()).await.unwrap();
                }
            });
            let mut settings = SmtpSettings {
                provider: "smtp".into(),
                host: "127.0.0.1".into(),
                port,
                username: String::new(),
                password: None,
                encryption: "null".into(),
                from_address: "sender@example.test".into(),
                from_name: "Fixture sender".into(),
            };
            settings.validate("development").unwrap();
            let from = sender();
            let submitted = submit(&settings, message(&from)).await;
            let raw = received.await.unwrap();
            let raw = String::from_utf8(raw).unwrap();
            match (outcome, submitted) {
                ("accepted", Submission::Accepted { message_id }) => {
                    assert!(raw.contains(&format!("Message-ID: <{message_id}>\r\n")))
                }
                (
                    "rejected",
                    Submission::Rejected {
                        retryable: false, ..
                    },
                ) => {}
                ("unknown", Submission::Uncertain { .. }) => {}
                (_, other) => panic!("Unexpected submission classification: {other:?}"),
            }
            assert!(raw.contains("List-Unsubscribe:"));
            server.await.unwrap();
        }
    }
}

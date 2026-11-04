//! Encrypted runtime-scoped SMTP configuration and authenticated, bounded transport.
use crate::AppState;
use anyhow::{Result, anyhow, bail};
use lettre::{
    AsyncSmtpTransport, Tokio1Executor,
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{collections::HashMap, net::IpAddr, time::Duration};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SmtpSettings {
    pub provider: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub encryption: String,
    pub from_address: String,
    pub from_name: String,
}
impl SmtpSettings {
    pub fn validate(&mut self, environment: &str) -> Result<()> {
        self.host = self.host.trim().to_ascii_lowercase();
        self.from_address = self.from_address.trim().to_string();
        self.from_name = self.from_name.trim().to_string();
        if self.provider != "smtp"
            || self.port == 0
            || self.host.is_empty()
            || self.host.len() > 253
            || self
                .host
                .contains(['/', '\\', ':', '@', '?', '#', '\r', '\n', '\0', ' '])
                && self.host.parse::<IpAddr>().is_err()
        {
            bail!("Configure a valid SMTP hostname and port");
        }
        if self.host.parse::<IpAddr>().is_err()
            && !self.host.split('.').all(|label| {
                !label.is_empty()
                    && label.len() <= 63
                    && !label.starts_with('-')
                    && !label.ends_with('-')
                    && label
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            })
        {
            bail!("Invalid SMTP hostname");
        }
        if !["tls", "ssl", "null"].contains(&self.encryption.as_str()) {
            bail!("SMTP encryption must be TLS, SSL, or None");
        }
        // The existing None control is usable for a development-only local relay.
        // Enabled remote/production SMTP always validates TLS and the server identity.
        if self.encryption == "null"
            && !(environment == "development"
                && self.host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback()))
        {
            bail!("Unencrypted SMTP is allowed only for a literal loopback relay in development");
        }
        if self.username.len() > 1024
            || self.password.as_ref().is_some_and(|p| {
                p.len() > 4096
                    || p.contains(['\r', '\n', '\0'])
                    || p.contains("••••••••")
                    || p.contains("********")
            })
            || self.username.contains(['\r', '\n', '\0'])
        {
            bail!("Invalid SMTP credentials");
        }
        if !self.username.is_empty() && self.password.as_deref().is_none_or(str::is_empty) {
            bail!("A password is required for authenticated SMTP");
        }
        if self.username.is_empty()
            && self
                .password
                .as_deref()
                .is_some_and(|password| !password.is_empty())
        {
            bail!("A username is required when supplying an SMTP password");
        }
        super::campaign_mailer::Identity {
            name: self.from_name.clone(),
            email: self.from_address.clone(),
        }
        .formatted()?;
        Ok(())
    }
    pub fn public_view(&self) -> Value {
        json!({"provider":"smtp","host":self.host,"port":self.port,"username":self.username,
            "has_password":self.password.as_ref().is_some_and(|p|!p.is_empty()),"encryption":self.encryption,
            "from_address":self.from_address,"from_name":self.from_name})
    }
    fn encrypted(&self, key: &str) -> Result<String> {
        let map = HashMap::from([
            ("provider".into(), self.provider.clone()),
            ("host".into(), self.host.clone()),
            ("port".into(), self.port.to_string()),
            ("username".into(), self.username.clone()),
            ("password".into(), self.password.clone().unwrap_or_default()),
            ("encryption".into(), self.encryption.clone()),
            ("from_address".into(), self.from_address.clone()),
            ("from_name".into(), self.from_name.clone()),
        ]);
        let encrypted = crate::utils::crypto::encrypt_map(key, &map);
        if !encrypted.starts_with("v1:") {
            bail!("SMTP credentials could not be encrypted")
        }
        Ok(encrypted)
    }
    fn decrypted(blob: &str, state: &AppState) -> Result<Self> {
        if !blob.starts_with("v1:") {
            bail!("Stored SMTP configuration is invalid")
        }
        let mut map =
            crate::utils::crypto::decrypt_map(&state.config.credentials_encryption_key, blob);
        let mut take = |key: &str| {
            map.remove(key)
                .ok_or_else(|| anyhow!("Stored SMTP configuration cannot be decrypted"))
        };
        let mut settings = Self {
            provider: take("provider")?,
            host: take("host")?,
            port: take("port")?.parse()?,
            username: take("username")?,
            password: Some(take("password")?),
            encryption: take("encryption")?,
            from_address: take("from_address")?,
            from_name: take("from_name")?,
        };
        settings.validate(&state.config.environment)?;
        Ok(settings)
    }
    pub async fn transport(&self) -> Result<AsyncSmtpTransport<Tokio1Executor>> {
        // Resolve once and connect to the selected address; the original hostname
        // remains the TLS identity. No second DNS lookup can change that destination.
        let address = tokio::time::timeout(
            Duration::from_secs(3),
            tokio::net::lookup_host((self.host.as_str(), self.port)),
        )
        .await
        .map_err(|_| anyhow!("SMTP hostname lookup timed out"))?
        .map_err(|_| anyhow!("SMTP hostname could not be resolved"))?
        .next()
        .ok_or_else(|| anyhow!("SMTP hostname has no addresses"))?;
        let tls = match self.encryption.as_str() {
            "tls" => Tls::Required(TlsParameters::new(self.host.clone())?),
            "ssl" => Tls::Wrapper(TlsParameters::new(self.host.clone())?),
            "null" => Tls::None,
            _ => bail!("Invalid SMTP TLS mode"),
        };
        let mut builder =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(address.ip().to_string())
                .port(self.port)
                .tls(tls)
                .timeout(Some(Duration::from_secs(10)));
        if !self.username.is_empty() {
            builder = builder.credentials(Credentials::new(
                self.username.clone(),
                self.password.clone().unwrap_or_default(),
            ));
        }
        Ok(builder.build())
    }
    pub async fn verify(&self) -> Result<()> {
        let transport = self.transport().await?;
        match tokio::time::timeout(Duration::from_secs(15), transport.test_connection()).await {
            Ok(Ok(true)) => Ok(()),
            Ok(Err(error)) if error.is_client() => {
                bail!("SMTP connection or authentication was rejected")
            }
            _ => bail!(
                "SMTP connection check failed; confirm hostname, port, TLS and relay credentials"
            ),
        }
    }
}

pub struct StoredSmtp {
    pub settings: SmtpSettings,
    pub blob: String,
    pub value: Value,
}
pub fn settings_key(state: &AppState) -> String {
    format!(
        "email.smtp.{}",
        super::credential_resolver::runtime_environment(&state.config)
    )
}
pub async fn load(state: &AppState) -> Result<Option<StoredSmtp>> {
    let value: Option<Value> = sqlx::query_scalar("SELECT value FROM settings WHERE key=$1")
        .bind(settings_key(state))
        .fetch_optional(&state.db.pool)
        .await?;
    value
        .map(|value| {
            let blob = value["encrypted"]
                .as_str()
                .ok_or_else(|| anyhow!("Stored SMTP configuration is invalid"))?
                .to_string();
            Ok(StoredSmtp {
                settings: SmtpSettings::decrypted(&blob, state)?,
                blob,
                value,
            })
        })
        .transpose()
}
pub fn merge_password(input: &mut SmtpSettings, stored: Option<&StoredSmtp>) -> Result<()> {
    if input.username.is_empty() && input.password.as_deref().is_none_or(str::is_empty) {
        input.password = None;
        return Ok(());
    }
    if input.password.as_deref().is_none_or(str::is_empty)
        && let Some(stored) = stored
    {
        if (input
            .host
            .trim()
            .eq_ignore_ascii_case(&stored.settings.host)
            && input.username == stored.settings.username)
            || stored
                .settings
                .password
                .as_deref()
                .is_none_or(str::is_empty)
        {
            input.password = stored.settings.password.clone();
        } else {
            bail!("Enter the password again when changing the SMTP host or username")
        }
    }
    Ok(())
}
pub async fn save(state: &AppState, mut input: SmtpSettings, actor: i64) -> Result<Value> {
    let mut tx = state.db.pool.begin().await?;
    // A per-scope lock also serializes creation when no settings row exists yet.
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,0))")
        .bind(settings_key(state))
        .execute(&mut *tx)
        .await?;
    let value: Option<Value> =
        sqlx::query_scalar("SELECT value FROM settings WHERE key=$1 FOR UPDATE")
            .bind(settings_key(state))
            .fetch_optional(&mut *tx)
            .await?;
    let stored = value
        .map(|value| {
            let blob = value["encrypted"]
                .as_str()
                .ok_or_else(|| anyhow!("Stored SMTP configuration is invalid"))?
                .to_string();
            Ok::<_, anyhow::Error>(StoredSmtp {
                settings: SmtpSettings::decrypted(&blob, state)?,
                blob,
                value,
            })
        })
        .transpose()?;
    merge_password(&mut input, stored.as_ref())?;
    input.validate(&state.config.environment)?;
    let encrypted = input.encrypted(&state.config.credentials_encryption_key)?;
    let value =
        json!({"encrypted":encrypted,"last_verified_at":null,"last_error":null,"status":"pending"});
    sqlx::query("INSERT INTO settings(key,value,created_at,updated_at) VALUES($1,$2,NOW(),NOW()) ON CONFLICT(key) DO UPDATE SET value=EXCLUDED.value,updated_at=NOW()")
        .bind(settings_key(state)).bind(value).execute(&mut *tx).await?;
    super::crm_logs::record(
        &mut tx,
        "info",
        "email",
        "SMTP configuration saved",
        json!({"environment":super::credential_resolver::runtime_environment(&state.config)}),
        Some(actor),
    )
    .await?;
    tx.commit().await?;
    Ok(input.public_view())
}

pub async fn verify(state: &AppState, mut input: SmtpSettings, actor: i64) -> Result<Value> {
    let stored = load(state).await?;
    merge_password(&mut input, stored.as_ref())?;
    input.validate(&state.config.environment)?;
    let result = input.verify().await;
    let success = result.is_ok();
    let message = result.err().map(|e| e.to_string()).unwrap_or_else(|| {
        if input.username.is_empty() {
            "SMTP connection and NOOP passed; no email was sent".into()
        } else {
            "Authenticated SMTP connection and NOOP passed; no email was sent".into()
        }
    });
    if let Some(stored) = stored {
        let same = input.host == stored.settings.host
            && input.port == stored.settings.port
            && input.username == stored.settings.username
            && input.password == stored.settings.password
            && input.encryption == stored.settings.encryption
            && input.from_address == stored.settings.from_address
            && input.from_name == stored.settings.from_name;
        if same {
            let mut tx = state.db.pool.begin().await?;
            let updated=sqlx::query("UPDATE settings SET value=jsonb_set(jsonb_set(jsonb_set(value,'{status}',to_jsonb($3::text)),'{last_verified_at}',CASE WHEN $4 THEN to_jsonb(NOW()) ELSE 'null'::jsonb END),'{last_error}',CASE WHEN $4 THEN 'null'::jsonb ELSE to_jsonb($5::text) END),updated_at=NOW() WHERE key=$1 AND value->>'encrypted'=$2")
                .bind(settings_key(state)).bind(stored.blob).bind(if success{"verified"}else{"error"}).bind(success).bind(&message).execute(&mut *tx).await?;
            if updated.rows_affected() != 1 {
                bail!("SMTP configuration changed during verification; reload before retrying")
            }
            super::crm_logs::record(
                &mut tx,
                if success { "info" } else { "warning" },
                "email",
                "SMTP configuration verification recorded",
                json!({"success":success}),
                Some(actor),
            )
            .await?;
            tx.commit().await?;
        }
    }
    Ok(json!({"success":success,"message":message,"email_sent":false}))
}

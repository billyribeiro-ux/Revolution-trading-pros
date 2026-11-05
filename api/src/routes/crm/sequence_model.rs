//! Bounded sequence authoring; enrollment revisions will preserve this definition.
use crate::{services::campaign_mailer::Identity, utils::errors::ApiError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
fn patch<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

pub(super) fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
pub(super) fn text(value: &str, max: usize, required: bool, header: bool) -> Result<(), ApiError> {
    if value.contains('\0')
        || value.chars().count() > max
        || required && value.trim().is_empty()
        || header && value.contains(['\r', '\n'])
    {
        return Err(invalid(
            "Provide bounded text without null characters or multiline headers",
        ));
    }
    Ok(())
}
pub(super) fn object(value: &Value) -> Result<(), ApiError> {
    fn safe(value: &Value, depth: usize) -> bool {
        if depth > 16 {
            return false;
        }
        match value {
            Value::String(value) => !value.contains('\0'),
            Value::Array(values) => values.iter().all(|value| safe(value, depth + 1)),
            Value::Object(values) => values
                .iter()
                .all(|(key, value)| !key.contains('\0') && safe(value, depth + 1)),
            _ => true,
        }
    }
    if !value.is_object() || value.to_string().len() > 65_536 || !safe(value, 0) {
        return Err(invalid(
            "Provide configuration objects within64KiB and16levels without null characters",
        ));
    }
    Ok(())
}
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct Mailer {
    pub from_name: String,
    pub from_email: String,
    pub reply_to_name: String,
    pub reply_to_email: String,
    pub is_custom: bool,
}
impl Mailer {
    pub fn validate(&self) -> Result<(), ApiError> {
        for value in [
            &self.from_name,
            &self.from_email,
            &self.reply_to_name,
            &self.reply_to_email,
        ] {
            text(value, 255, false, true)?;
        }
        if self.is_custom || !self.from_email.is_empty() {
            Identity {
                name: self.from_name.clone(),
                email: self.from_email.clone(),
            }
            .formatted()
            .map_err(|_| invalid("Provide a valid sequence sender identity"))?;
        }
        if !self.reply_to_email.is_empty() {
            Identity {
                name: self.reply_to_name.clone(),
                email: self.reply_to_email.clone(),
            }
            .formatted()
            .map_err(|_| invalid("Provide a valid sequence reply-to identity"))?;
        }
        Ok(())
    }
}
#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct Settings {
    pub mailer_settings: Mailer,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Condition {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub value: Option<Value>,
    #[serde(default)]
    pub previous_email_id: Option<Uuid>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Mail {
    pub id: Uuid,
    pub sequence_id: String,
    pub title: String,
    pub email_subject: String,
    #[serde(default)]
    pub email_pre_header: String,
    pub email_body: String,
    #[serde(default)]
    pub text_body: Option<String>,
    pub delay_value: u32,
    pub delay_unit: String,
    pub delay: u64,
    pub status: String,
    #[serde(default)]
    pub settings: Mailer,
    pub position: usize,
    #[serde(default)]
    pub conditions: Vec<Condition>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct MailInput {
    pub id: Option<Uuid>,
    pub position: Option<usize>,
    #[serde(alias = "name")]
    pub title: Option<String>,
    #[serde(alias = "subject")]
    pub email_subject: Option<String>,
    pub email_pre_header: Option<String>,
    #[serde(alias = "html_content")]
    pub email_body: Option<String>,
    #[serde(alias = "plain_text_content", default, deserialize_with = "patch")]
    pub text_body: Option<Option<String>>,
    pub delay_value: Option<u32>,
    pub delay: Option<u64>,
    pub delay_unit: Option<String>,
    pub status: Option<String>,
    pub settings: Option<Mailer>,
    pub conditions: Option<Vec<Condition>>,
}
impl Mail {
    pub fn new(
        sequence: i64,
        actor: i64,
        input: MailInput,
        position: usize,
    ) -> Result<Self, ApiError> {
        let now = Utc::now();
        Self {
            id: input.id.unwrap_or_else(Uuid::new_v4),
            sequence_id: sequence.to_string(),
            title: String::new(),
            email_subject: String::new(),
            email_pre_header: String::new(),
            email_body: String::new(),
            text_body: None,
            delay_value: 0,
            delay_unit: "days".into(),
            delay: 0,
            status: "draft".into(),
            settings: Mailer::default(),
            position,
            conditions: Vec::new(),
            created_at: now,
            updated_at: now,
            created_by: Some(actor.to_string()),
        }
        .apply(input)
    }
    pub fn apply(mut self, input: MailInput) -> Result<Self, ApiError> {
        if input
            .position
            .is_some_and(|position| position != self.position)
        {
            return Err(invalid(
                "Sequence email position changed; use the reorder action or save the complete ordered definition",
            ));
        }
        if input.id.is_some_and(|id| id != self.id) {
            return Err(invalid("Sequence email identity cannot change"));
        }
        let explicit_delay_value = input.delay_value.is_some();
        if let Some(value) = input.title {
            self.title = value;
        }
        if let Some(value) = input.email_subject {
            self.email_subject = value;
        }
        if let Some(value) = input.email_pre_header {
            self.email_pre_header = value;
        }
        if let Some(value) = input.email_body {
            self.email_body = value;
        }
        if let Some(value) = input.text_body {
            self.text_body = value;
        }
        if let Some(value) = input.delay_value {
            self.delay_value = value;
        }
        if let Some(value) = input.delay_unit {
            self.delay_unit = value;
        }
        if let Some(value) = input.status {
            self.status = value;
        }
        if let Some(value) = input.settings {
            self.settings = value;
        }
        if let Some(value) = input.conditions {
            self.conditions = value;
        }
        if let Some(seconds) = input.delay {
            let multiplier = delay(1, &self.delay_unit)?;
            if explicit_delay_value {
                if delay(self.delay_value, &self.delay_unit)? != seconds {
                    return Err(invalid("Sequence delay does not match its value and unit"));
                }
            } else {
                if seconds % multiplier != 0 {
                    return Err(invalid(
                        "Specify a sequence delay value and unit that exactly represent its duration",
                    ));
                }
                self.delay_value = u32::try_from(seconds / multiplier)
                    .map_err(|_| invalid("Sequence delay is too large"))?;
            }
        }
        self.delay = delay(self.delay_value, &self.delay_unit)?;
        self.updated_at = Utc::now();
        self.validate(false)?;
        Ok(self)
    }
    pub fn validate(&self, submit: bool) -> Result<(), ApiError> {
        text(&self.title, 255, true, false)?;
        text(&self.email_subject, 500, submit, true)?;
        text(&self.email_pre_header, 500, false, false)?;
        text(&self.email_body, 1_000_000, submit, false)?;
        if let Some(value) = &self.text_body {
            text(value, 1_000_000, false, false)?;
        }
        if !matches!(self.status.as_str(), "draft" | "active" | "paused") {
            return Err(invalid("Invalid sequence email status"));
        }
        if self.id.is_nil()
            || self.position > 49
            || self
                .sequence_id
                .parse::<i64>()
                .ok()
                .is_none_or(|value| value < 1)
            || self.delay != delay(self.delay_value, &self.delay_unit)?
        {
            return Err(invalid(
                "Invalid sequence email identity, position or delay",
            ));
        }
        self.settings.validate()?;
        if self.conditions.len() > 20 {
            return Err(invalid(
                "At most20conditions are allowed per sequence email",
            ));
        }
        for condition in &self.conditions {
            if matches!(condition.kind.as_str(), "tag" | "custom")
                && condition.previous_email_id.is_some()
            {
                return Err(invalid(
                    "Tag and custom field conditions do not accept an email reference",
                ));
            }
            if matches!(
                condition.kind.as_str(),
                "opened" | "clicked" | "not_opened" | "not_clicked"
            ) && condition.value.is_some()
            {
                return Err(invalid(
                    "Email observation conditions use their type and earlier email reference only",
                ));
            }
            if !matches!(
                condition.kind.as_str(),
                "opened" | "clicked" | "not_opened" | "not_clicked" | "tag" | "custom"
            ) || condition
                .previous_email_id
                .is_some_and(|value| value.is_nil() || value == self.id)
            {
                return Err(invalid(
                    "Invalid sequence condition or previous email reference",
                ));
            }
            if let Some(value) = &condition.value
                && (value.to_string().len() > 4096 || value.to_string().contains("\\u0000"))
            {
                return Err(invalid("Sequence condition value exceeds its limit"));
            }
            if condition.kind == "tag"
                && condition
                    .value
                    .as_ref()
                    .and_then(Value::as_str)
                    .is_none_or(|value| value.trim().is_empty() || value.chars().count() > 255)
            {
                return Err(invalid("Tag conditions require a tag name"));
            }
            if condition.kind == "custom" {
                let value = condition
                    .value
                    .as_ref()
                    .ok_or_else(|| invalid("Custom conditions require a predicate object"))?;
                object(value)?;
                if value.as_object().is_some_and(|object| {
                    object.keys().any(|key| {
                        !matches!(key.as_str(), "field" | "operator" | "value" | "logic")
                    })
                }) {
                    return Err(invalid("Unsupported custom condition properties"));
                }
                let predicate: super::segment_rules::Condition =
                    serde_json::from_value(value.clone()).map_err(|_| {
                        invalid("Provide a contact field, operator and value for custom conditions")
                    })?;
                let _ = super::segment_rules::query(
                    "SELECT c.id FROM contacts c",
                    &[predicate],
                    "all",
                )?;
            }
        }
        Ok(())
    }
}
fn delay(value: u32, unit: &str) -> Result<u64, ApiError> {
    let multiplier = match unit {
        "minutes" => 60,
        "hours" => 3600,
        "days" => 86400,
        "weeks" => 604800,
        _ => return Err(invalid("Choose minutes, hours, days or weeks")),
    };
    let seconds = u64::from(value) * multiplier;
    if seconds > 366 * 86400 {
        return Err(invalid("A sequence email delay cannot exceed366days"));
    }
    Ok(seconds)
}
pub(super) fn mails(value: Value, sequence: i64) -> Result<Vec<Mail>, ApiError> {
    let mails:Vec<Mail>=serde_json::from_value(value).map_err(|_|invalid("Stored sequence steps need review before editing or activation; original data is preserved"))?;
    if mails.len() > 50
        || serde_json::to_vec(&mails)
            .map_err(|_| invalid("Sequence cannot be encoded"))?
            .len()
            > 8_000_000
    {
        return Err(invalid(
            "A sequence supports at most50emails and8MB of saved content",
        ));
    }
    for (index, mail) in mails.iter().enumerate() {
        mail.validate(false)?;
        if mail.sequence_id != sequence.to_string()
            || mail.position != index
            || mails[..index].iter().any(|other| other.id == mail.id)
        {
            return Err(invalid(
                "Stored sequence email identities or order are invalid; original data is preserved",
            ));
        }
        for condition in &mail.conditions {
            if let Some(previous) = condition.previous_email_id
                && !mails[..index].iter().any(|earlier| earlier.id == previous)
            {
                return Err(invalid(
                    "Sequence conditions may reference earlier emails only",
                ));
            }
        }
    }
    Ok(mails)
}

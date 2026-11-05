//! Strict native campaign variables. Stored snapshots supply recipient facts.
use anyhow::{Result, anyhow};
use serde_json::{Value, json};
pub struct Recipient<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub snapshot: &'a Value,
    pub app_url: &'a str,
    pub unsubscribe_url: &'a str,
    pub brand: &'a str,
}
pub fn render(definition: &Value, recipient: Recipient<'_>, subject: &str) -> Result<Value> {
    let mut resolved = definition.clone();
    resolved["email_subject"] = json!(subject);
    if definition["native_email_campaign_id"].as_i64().is_none() {
        return Ok(resolved);
    }
    let first = recipient.snapshot["first_name"].as_str().unwrap_or("");
    let last = recipient.snapshot["last_name"].as_str().unwrap_or("");
    let company = recipient.snapshot["company"].as_str().unwrap_or("");
    let person =
        json!({"email":recipient.email,"name":recipient.name,"first_name":first,"last_name":last});
    let model = json!({"email":recipient.email,"name":recipient.name,"full_name":recipient.name,"first_name":first,"last_name":last,"company":company,"user":person,"subscriber":person,"unsubscribe_url":recipient.unsubscribe_url});
    let plain = definition["type"] == "plain_text";
    let body = definition["email_body"]
        .as_str()
        .ok_or_else(|| anyhow!("Campaign content is missing"))?;
    let text = if plain {
        Some(body)
    } else {
        definition["text_body"].as_str()
    };
    let rendered = super::transactional_templates::render_content(
        subject,
        if plain { "" } else { body },
        text,
        model,
        recipient.app_url,
        recipient.brand,
    )?;
    resolved["email_subject"] = json!(rendered.subject);
    let (body, text) = if plain {
        (
            rendered
                .text
                .ok_or_else(|| anyhow!("Plain campaign content is missing"))?,
            None,
        )
    } else {
        (rendered.html, rendered.text)
    };
    resolved["email_body"] = json!(body);
    resolved["text_body"] = json!(text);
    Ok(resolved)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn recipient(snapshot: &Value) -> Recipient<'_> {
        Recipient {
            email: "member@example.test",
            name: "<Actual & fixture>",
            snapshot,
            app_url: "http://127.0.0.1:5274",
            unsubscribe_url: "http://127.0.0.1:5274/newsletter/unsubscribe?token=fixture",
            brand: "Fixture sender",
        }
    }
    #[test]
    fn native_variables_escape_html_preserve_text_and_use_the_frozen_variant_subject() {
        let snapshot = json!({"first_name":"<First>","last_name":"Last"});
        let definition = json!({"native_email_campaign_id":42,"type":"raw_html","email_subject":"Original A","email_body":"<p>{{first_name}} {{user.name}}</p><a href='{{unsubscribe_url}}'>Leave</a>","text_body":"{{full_name}} {{email}}"});
        let rendered = render(
            &definition,
            recipient(&snapshot),
            "Frozen B for {{first_name}}",
        )
        .unwrap();
        assert_eq!(rendered["email_subject"], "Frozen B for <First>");
        assert!(
            rendered["email_body"]
                .as_str()
                .unwrap()
                .contains("&lt;First&gt; &lt;Actual &amp; fixture&gt;")
        );
        assert_eq!(
            rendered["text_body"],
            "<Actual & fixture> member@example.test"
        );
        assert_eq!(definition["email_subject"], "Original A");
    }
    #[test]
    fn missing_variables_and_header_injection_fail_before_submission() {
        let snapshot = json!({"first_name":"First"});
        let definition = json!({"native_email_campaign_id":42,"type":"raw_html","email_body":"{{unknown_required_variable}}"});
        assert!(render(&definition, recipient(&snapshot), "Subject").is_err());
        let definition =
            json!({"native_email_campaign_id":42,"type":"raw_html","email_body":"<p>Body</p>"});
        assert!(
            render(
                &definition,
                recipient(&snapshot),
                "Subject\r\nInjected: header"
            )
            .is_err()
        );
    }
    #[test]
    fn plain_campaigns_keep_plain_content_and_regular_crm_content_is_unchanged() {
        let snapshot = json!({"first_name":"<First>"});
        let plain = json!({"native_email_campaign_id":42,"type":"plain_text","email_body":"Hello {{first_name}} <plain>","text_body":null});
        assert_eq!(
            render(&plain, recipient(&snapshot), "Plain").unwrap()["email_body"],
            "Hello <First> <plain>"
        );
        let crm = json!({"type":"raw_html","email_subject":"Original","email_body":"<p>{{literal_braces}}</p>"});
        let resolved = render(&crm, recipient(&snapshot), "Frozen subject").unwrap();
        assert_eq!(resolved["email_body"], crm["email_body"]);
        assert_eq!(resolved["email_subject"], "Frozen subject");
    }
}

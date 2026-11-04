//! Render persisted SMTP templates with strict variables and bounded output.
use anyhow::{Result, anyhow, bail};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::io::{self, Write};

pub struct Rendered {
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
}
struct Bounded {
    bytes: Vec<u8>,
    limit: usize,
}
impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(io::Error::other("Rendered email exceeds its limit"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn render(template: &str, model: &Value, html: bool, limit: usize) -> Result<String> {
    if template.len() > 4_000_000 {
        bail!("Email template exceeds its limit")
    }
    let mut registry = handlebars::Handlebars::new();
    registry.set_strict_mode(true);
    if !html {
        registry.register_escape_fn(handlebars::no_escape);
    }
    let mut output = Bounded {
        bytes: Vec::new(),
        limit,
    };
    registry
        .render_template_to_write(template, model, &mut output)
        .map_err(|_| {
            anyhow!("Email template has missing variables, invalid syntax, or excessive output")
        })?;
    Ok(String::from_utf8(output.bytes)?)
}
pub async fn load(
    pool: &PgPool,
    alias: &str,
    mut model: Value,
    app_url: &str,
    brand: &str,
) -> Result<Rendered> {
    let object = model
        .as_object_mut()
        .ok_or_else(|| anyhow!("Email template model must be an object"))?;
    object.entry("app_url").or_insert_with(|| json!(app_url));
    object.entry("brand_name").or_insert_with(|| json!(brand));
    let row:Option<(String,String,Option<String>)>=sqlx::query_as("SELECT subject,html_content,text_content FROM email_templates WHERE slug=$1 AND is_active=true").bind(alias).fetch_optional(pool).await?;
    let (subject, html, text) =
        row.ok_or_else(|| anyhow!("An active local email template is required for this alias"))?;
    let subject = render(&subject, &model, false, 4096)?;
    if subject.trim().is_empty()
        || subject.chars().count() > 500
        || subject.contains(['\r', '\n', '\0'])
    {
        bail!("Rendered email subject is invalid")
    }
    let html = render(&html, &model, true, 4_000_000)?;
    let text = text
        .map(|value| render(&value, &model, false, 4_000_000))
        .transpose()?;
    Ok(Rendered {
        subject,
        html,
        text,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn html_escaping_and_plain_text_keep_distinct_content_contracts() {
        let model = json!({"name":"<Customer & fixture>","url":"https://example.test/confirm?a=1&b=2","items":[{"title":"First"},{"title":"Second"}]});
        let html=render("<p>{{name}}</p><a href=\"{{url}}\">Confirm</a>{{#each items}}<b>{{title}}</b>{{/each}}",&model,true,4096).unwrap();
        assert!(html.contains("&lt;Customer &amp; fixture&gt;"));
        assert!(html.contains("a&#x3D;1&amp;b&#x3D;2") || html.contains("a=1&amp;b=2"));
        assert!(html.contains("<b>First</b><b>Second</b>"));
        assert_eq!(
            render("{{name}} {{url}}", &model, false, 4096).unwrap(),
            "<Customer & fixture> https://example.test/confirm?a=1&b=2"
        );
    }
    #[test]
    fn missing_variables_and_expansion_limits_are_errors() {
        assert!(render("Hello {{missing}}", &json!({}), true, 4096).is_err());
        assert!(
            render(
                "{{#each items}}{{this}}{{/each}}",
                &json!({"items":["12345678","12345678"]}),
                true,
                15
            )
            .is_err()
        );
        assert_eq!(
            render(
                "{{#if date}}{{date}}{{else}}No date supplied{{/if}}",
                &json!({"date":null}),
                true,
                4096
            )
            .unwrap(),
            "No date supplied"
        );
    }
}

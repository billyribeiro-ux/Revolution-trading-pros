//! Apply the campaign's saved preheader/tracking without stripping email design.
use anyhow::Result;
use lol_html::{RewriteStrSettings, element, html_content::ContentType, rewrite_str};
use serde_json::Value;
use std::{cell::Cell, rc::Rc};

pub fn render(definition: &Value, app_url: &str, unsubscribe: &str) -> Result<String> {
    let body = definition["email_body"].as_str().unwrap_or("");
    if definition["type"] == "plain_text" {
        return Ok(format!("{body}\n\nUnsubscribe: {unsubscribe}"));
    }
    let preheader = definition["email_pre_header"].as_str().unwrap_or("");
    let preheader = if preheader.is_empty() {
        String::new()
    } else {
        format!(
            "<div style=\"display:none;max-height:0;overflow:hidden;opacity:0\" aria-hidden=\"true\">{}</div>",
            html_escape::encode_text(preheader)
        )
    };
    let footer = format!(
        "<p><a href=\"{}\">Unsubscribe</a></p>",
        html_escape::encode_double_quoted_attribute(unsubscribe)
    );
    let base = reqwest::Url::parse(app_url)?;
    let utm = definition["utm_settings"]
        .as_object()
        .cloned()
        .unwrap_or_default();
    let has_body = Rc::new(Cell::new(false));
    let body_seen = has_body.clone();
    let callback = unsubscribe.to_string();
    let prefix = preheader.clone();
    let suffix = footer.clone();
    let rewritten = rewrite_str(
        body,
        RewriteStrSettings::new()
            .append_element_content_handler(element!("body", move |element| {
                body_seen.set(true);
                element.prepend(&prefix, ContentType::Html);
                element.append(&suffix, ContentType::Html);
                Ok(())
            }))
            .append_element_content_handler(element!("a[href]", move |element| {
                let Some(href) = element.get_attribute("href") else {
                    return Ok(());
                };
                let decoded = html_escape::decode_html_entities(&href);
                let decoded = decoded.trim();
                if decoded.starts_with('#') || decoded == callback {
                    return Ok(());
                }
                let Ok(mut url) = base.join(decoded) else {
                    return Ok(());
                };
                if !matches!(url.scheme(), "http" | "https")
                    || url.path() == "/newsletter/unsubscribe"
                {
                    return Ok(());
                }
                let pairs: Vec<(String, String)> = url
                    .query_pairs()
                    .filter(|(key, _)| {
                        !utm.get(key.as_ref())
                            .and_then(Value::as_str)
                            .is_some_and(|v| !v.is_empty())
                    })
                    .map(|(k, v)| (k.into_owned(), v.into_owned()))
                    .collect();
                if !utm.is_empty() {
                    url.set_query(None);
                    {
                        let mut query = url.query_pairs_mut();
                        for (key, value) in pairs {
                            query.append_pair(&key, &value);
                        }
                        for (key, value) in &utm {
                            if let Some(value) = value.as_str().filter(|v| !v.is_empty()) {
                                query.append_pair(key, value);
                            }
                        }
                    }
                }
                element.set_attribute("href", url.as_ref())?;
                Ok(())
            })),
    )?;
    Ok(if has_body.get() {
        rewritten
    } else {
        format!("{preheader}{rewritten}{footer}")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn preserves_design_and_tracks_actual_links_with_safe_preheader_and_unsubscribe() {
        let definition = json!({"type":"raw_html","email_body":"<!doctype html><html><head><style>.email{color:red}</style></head><body class='email'><!--keep--><a href='/pricing?a=1&amp;b=2#details'>Pricing</a><a href='mailto:support@example.test'>Email</a><a href='#section'>Jump</a></body></html>","email_pre_header":"<script>literal & text</script>","utm_settings":{"utm_source":"campaign & fixture","utm_campaign":"Example"}});
        let rendered = render(
            &definition,
            "https://frontend.example.test",
            "https://frontend.example.test/newsletter/unsubscribe?token=fixture",
        )
        .unwrap();
        assert!(rendered.contains("<style>.email{color:red}</style>"));
        assert!(rendered.contains("<!--keep-->"));
        assert!(rendered.contains("&lt;script&gt;literal &amp; text&lt;/script&gt;"));
        assert!(rendered.contains("utm_source=campaign+%26+fixture"));
        assert!(rendered.contains("a=1&b=2"));
        assert!(rendered.contains("#details"));
        assert!(rendered.contains("href='mailto:support@example.test'"));
        assert!(rendered.contains("href='#section'"));
        assert!(rendered.contains(
            "href=\"https://frontend.example.test/newsletter/unsubscribe?token=fixture\""
        ));
        assert!(rendered.find(">Unsubscribe</a>").unwrap() < rendered.find("</body>").unwrap());
    }
    #[test]
    fn plain_text_stays_text_and_contains_the_real_unsubscribe_link() {
        assert_eq!(
            render(
                &json!({"type":"plain_text","email_body":"Literal <html>"}),
                "https://frontend.example.test",
                "https://frontend.example.test/unsubscribe"
            )
            .unwrap(),
            "Literal <html>\n\nUnsubscribe: https://frontend.example.test/unsubscribe"
        );
    }
}

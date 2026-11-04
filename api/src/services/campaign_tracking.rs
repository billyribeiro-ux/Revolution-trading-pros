//! Native SMTP request observations. No inferred delivery or human engagement.
use anyhow::{Result, bail};
use lol_html::{RewriteStrSettings, element, html_content::ContentType, rewrite_str};
use sqlx::PgPool;
use std::{cell::RefCell, rc::Rc};
use uuid::Uuid;

fn callback(base: &str, kind: &str, token: Uuid) -> Result<String> {
    let mut url = super::campaign_delivery::callback_url(base)?;
    url.set_path(&format!("/api/newsletter/tracking/{kind}/{token}"));
    url.set_query(None);
    url.set_fragment(None);
    Ok(url.to_string())
}
fn target(value: &str, unsubscribe: &str) -> Option<reqwest::Url> {
    let url = reqwest::Url::parse(value).ok()?;
    (matches!(url.scheme(), "http" | "https")
        && url.host_str().is_some()
        && url.username().is_empty()
        && url.password().is_none()
        && value.len() <= 4096
        && url.as_str().len() <= 4096
        && value != unsubscribe
        && url.path() != "/newsletter/unsubscribe")
        .then_some(url)
}
struct Rendered {
    html: String,
    text: Option<String>,
    links: Vec<(Uuid, String)>,
}
fn rewrite(
    body: &str,
    text: Option<&str>,
    plain: bool,
    base: &str,
    unsubscribe: &str,
    token: Uuid,
) -> Result<Rendered> {
    let links = Rc::new(RefCell::new(Vec::new()));
    let rewrite_text = |input: &str| -> Result<String> {
        let pattern = regex::Regex::new(r#"https?://[^\s<>\"']+"#)?;
        let mut output = String::new();
        let mut end = 0;
        for found in pattern.find_iter(input) {
            output.push_str(&input[end..found.start()]);
            if let Some(url) = target(found.as_str(), unsubscribe) {
                if links.borrow().len() >= 5000 {
                    bail!("Campaign has too many tracking links");
                }
                let id = Uuid::new_v4();
                links.borrow_mut().push((id, url.to_string()));
                output.push_str(&callback(base, "click", id)?);
            } else {
                output.push_str(found.as_str());
            }
            end = found.end();
            if output.len() > 4_000_000 {
                bail!("Campaign tracking exceeds the message size limit");
            }
        }
        output.push_str(&input[end..]);
        Ok(output)
    };
    let rendered = if plain {
        rewrite_text(body)?
    } else {
        let captured = links.clone();
        let origin = base.to_string();
        let unsubscribe = unsubscribe.to_string();
        let pixel = format!(
            "<img src=\"{}\" width=\"1\" height=\"1\" alt=\"\" style=\"display:block;width:1px;height:1px\">",
            callback(base, "open", token)?
        );
        // Append before </body> where present, preserving email document design.
        let seen = Rc::new(std::cell::Cell::new(false));
        let body_seen = seen.clone();
        let image = pixel.clone();
        let result = rewrite_str(
            body,
            RewriteStrSettings::new()
                .append_element_content_handler(element!("body", move |element| {
                    body_seen.set(true);
                    element.append(&image, ContentType::Html);
                    Ok(())
                }))
                .append_element_content_handler(element!("a[href]", move |element| {
                    let Some(href) = element.get_attribute("href") else {
                        return Ok(());
                    };
                    let decoded = html_escape::decode_html_entities(&href);
                    if let Some(url) = target(decoded.trim(), &unsubscribe) {
                        if captured.borrow().len() >= 5000 {
                            return Err(
                                anyhow::anyhow!("Campaign has too many tracking links").into()
                            );
                        }
                        let id = Uuid::new_v4();
                        captured.borrow_mut().push((id, url.to_string()));
                        element.set_attribute("href", &callback(&origin, "click", id)?)?;
                    }
                    Ok(())
                })),
        )?;
        if seen.get() {
            result
        } else {
            format!("{result}{pixel}")
        }
    };
    let text = text.map(rewrite_text).transpose()?;
    let links = links.take();
    if links.len() > 5000
        || rendered.len() > 4_000_000
        || text.as_ref().is_some_and(|v| v.len() > 4_000_000)
    {
        bail!("Campaign tracking exceeds the supported message size or link count");
    }
    Ok(Rendered {
        html: rendered,
        text,
        links,
    })
}

pub async fn prepare(
    pool: &PgPool,
    delivery: Uuid,
    body: &str,
    text: Option<&str>,
    plain: bool,
    base: &str,
    unsubscribe: &str,
) -> Result<(String, Option<String>)> {
    let token =
        sqlx::query_scalar("SELECT tracking_token FROM crm_campaign_deliveries WHERE id=$1")
            .bind(delivery)
            .fetch_one(pool)
            .await?;
    let rendered = rewrite(body, text, plain, base, unsubscribe, token)?;
    let mut tx = pool.begin().await?;
    let (ids, targets): (Vec<_>, Vec<_>) = rendered.links.into_iter().unzip();
    sqlx::query("INSERT INTO crm_campaign_tracking_links(id,delivery_id,target) SELECT id,$2,target FROM UNNEST($1::uuid[],$3::text[]) AS link(id,target)")
        .bind(ids).bind(delivery).bind(targets).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok((rendered.html, rendered.text))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_design_and_unsubscribe_and_stores_only_safe_targets() {
        let unsubscribe = "https://example.test/newsletter/unsubscribe?token=private";
        let body = format!(
            "<!doctype html><html><head><style>.x{{color:red}}</style></head><body><a href='https://example.test/a?one=1&amp;two=2#details'>A</a><a href='mailto:a@example.test'>Mail</a><a href='https://user:pass@example.test/private'>Private</a><a href='{unsubscribe}'>Unsubscribe</a></body></html>"
        );
        let rendered = rewrite(
            &body,
            Some(&format!(
                "https://example.test/plain\nUnsubscribe: {unsubscribe}"
            )),
            false,
            "https://example.test",
            unsubscribe,
            Uuid::new_v4(),
        )
        .unwrap();
        assert_eq!(rendered.links.len(), 2);
        assert_eq!(
            rendered.links[0].1,
            "https://example.test/a?one=1&two=2#details"
        );
        assert!(rendered.html.contains("<style>.x{color:red}</style>"));
        assert!(rendered.html.contains("mailto:a@example.test"));
        assert!(rendered.html.contains(unsubscribe));
        assert!(rendered.html.contains("/tracking/open/"));
        assert!(
            rendered.html.find("/tracking/open/").unwrap() < rendered.html.find("</body>").unwrap()
        );
        assert!(rendered.text.unwrap().contains("/tracking/click/"));
        let plain = rewrite(
            "https://example.test/plain",
            None,
            true,
            "https://example.test",
            unsubscribe,
            Uuid::new_v4(),
        )
        .unwrap();
        assert!(!plain.html.contains("<img"));
        assert_eq!(plain.links.len(), 1);
    }
}

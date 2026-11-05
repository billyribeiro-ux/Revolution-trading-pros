//! Resolve the established audience filters against real contacts and confirmations.
use super::segment_rules::{self, Condition};
use crate::services::member_segmentation;
use crate::utils::errors::ApiError;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{PgPool, Postgres, QueryBuilder};

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Filter {
    list: String,
    tag: String,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Audience {
    subscribers: Vec<Filter>,
    excluded_subscribers: Vec<Filter>,
    sending_filter: Option<String>,
    dynamic_segment: Option<String>,
    advanced_filters: Vec<Condition>,
    newsletter_segment: Option<String>,
    eligibility: Option<String>,
    members: bool,
    member_segment: Option<i64>,
}
struct ResolvedFilter {
    list: Option<i64>,
    tag: Option<String>,
}
pub(crate) struct ResolvedAudience {
    includes: Vec<ResolvedFilter>,
    excludes: Vec<ResolvedFilter>,
    advanced: Vec<Condition>,
    segment: Vec<Condition>,
    segment_match: String,
    newsletter_segment: Option<String>,
    members: bool,
    member_segment: Option<(i64, member_segmentation::Definition)>,
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Campaign audience query failed");
    ApiError::database_error("Unable to resolve campaign audience")
}
fn id(value: &str) -> Result<i64, ApiError> {
    value
        .parse::<i64>()
        .ok()
        .filter(|v| *v > 0)
        .ok_or_else(|| invalid("Audience references must be positive record IDs"))
}
async fn resolve_filters(
    pool: &PgPool,
    filters: Vec<Filter>,
) -> Result<Vec<ResolvedFilter>, ApiError> {
    if filters.len() > 50 {
        return Err(invalid("At most 50 audience filter groups are allowed"));
    }
    let mut result = Vec::new();
    for filter in filters {
        if filter.list.is_empty() && filter.tag.is_empty() {
            return Err(invalid("Select a list or tag for each audience group"));
        }
        let list = if filter.list.is_empty() {
            None
        } else {
            let selected = id(&filter.list)?;
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_lists WHERE id=$1)")
                    .bind(selected)
                    .fetch_one(pool)
                    .await
                    .map_err(db)?;
            if !exists {
                return Err(invalid("An audience list no longer exists"));
            }
            Some(selected)
        };
        let tag = if filter.tag.is_empty() {
            None
        } else {
            let selected = id(&filter.tag)?;
            Some(
                sqlx::query_scalar("SELECT COALESCE(title,name) FROM crm_tags WHERE id=$1")
                    .bind(selected)
                    .fetch_optional(pool)
                    .await
                    .map_err(db)?
                    .ok_or_else(|| invalid("An audience tag no longer exists"))?,
            )
        };
        result.push(ResolvedFilter { list, tag });
    }
    Ok(result)
}
pub(crate) async fn resolve(
    pool: &PgPool,
    value: Option<Value>,
) -> Result<ResolvedAudience, ApiError> {
    let audience: Audience =
        serde_json::from_value(value.filter(|v| !v.is_null()).unwrap_or(json!({})))
            .map_err(|_| invalid("Unsupported audience configuration"))?;
    if audience
        .sending_filter
        .as_deref()
        .is_some_and(|v| !["all", "confirmed", "subscribed"].contains(&v))
    {
        return Err(invalid("Unsupported sending filter"));
    }
    if audience
        .eligibility
        .as_deref()
        .is_some_and(|policy| policy != "confirmed_only")
    {
        return Err(invalid(
            "Newsletter audiences require confirmed_only eligibility",
        ));
    }
    if audience.newsletter_segment.as_ref().is_some_and(|segment| {
        segment.trim().is_empty()
            || segment.contains('\0')
            || segment.chars().count() > 255
            || !audience.subscribers.is_empty()
            || !audience.excluded_subscribers.is_empty()
            || !audience.advanced_filters.is_empty()
            || audience
                .dynamic_segment
                .as_ref()
                .is_some_and(|value| !value.is_empty())
    }) {
        return Err(invalid(
            "Choose a newsletter segment or CRM contact filters, without mixing audience sources",
        ));
    }
    if audience.member_segment.is_some() && !audience.members {
        return Err(invalid(
            "Member segment requires the member audience source",
        ));
    }
    if audience.members
        && (audience.newsletter_segment.is_some()
            || !audience.subscribers.is_empty()
            || !audience.excluded_subscribers.is_empty()
            || !audience.advanced_filters.is_empty()
            || audience
                .dynamic_segment
                .as_ref()
                .is_some_and(|value| !value.is_empty()))
    {
        return Err(invalid(
            "Choose member audiences without mixing CRM or newsletter filters",
        ));
    }
    let member_segment = if let Some(id) = audience.member_segment {
        if id <= 0 {
            return Err(invalid("Member segment must be a positive record ID"));
        }
        Some((id, member_segmentation::load(pool, id, true).await?))
    } else {
        None
    };
    let includes = resolve_filters(pool, audience.subscribers).await?;
    let excludes = resolve_filters(pool, audience.excluded_subscribers).await?;
    let mut segment = Vec::new();
    let mut segment_match = "all".to_string();
    if let Some(selected) = audience.dynamic_segment.filter(|v| !v.is_empty()) {
        let selected = id(&selected)?;
        let row: Option<(Option<Value>, String)> = sqlx::query_as(
            "SELECT conditions,match_type FROM crm_segments WHERE id=$1 AND is_active=true",
        )
        .bind(selected)
        .fetch_optional(pool)
        .await
        .map_err(db)?;
        let (rules, matching) = row.ok_or_else(|| invalid("Select an active dynamic segment"))?;
        segment = serde_json::from_value(rules.unwrap_or(json!([]))).map_err(|_| {
            invalid("Selected segment has invalid rules; edit its definition first")
        })?;
        segment_match = matching;
    }
    let resolved = ResolvedAudience {
        includes,
        excludes,
        advanced: audience.advanced_filters,
        segment,
        segment_match,
        newsletter_segment: audience.newsletter_segment,
        members: audience.members,
        member_segment,
    };
    // Validate all conditions before constructing either count or data queries.
    resolved.query("SELECT COUNT(*) FROM contacts c", false)?;
    Ok(resolved)
}
fn append_filters(query: &mut QueryBuilder<Postgres>, filters: &[ResolvedFilter]) {
    for (index, filter) in filters.iter().enumerate() {
        if index > 0 {
            query.push(" OR ");
        }
        query.push("(");
        if let Some(list) = filter.list {
            query.push("EXISTS(SELECT 1 FROM crm_list_members m WHERE m.contact_id=c.id AND m.list_id=").push_bind(list).push(")");
        }
        if filter.list.is_some() && filter.tag.is_some() {
            query.push(" AND ");
        }
        if let Some(tag) = &filter.tag {
            query
                .push("COALESCE(c.tags,'[]'::jsonb) @> ")
                .push_bind(json!([tag]));
        }
        query.push(")");
    }
}
impl ResolvedAudience {
    pub(super) fn is_newsletter(&self) -> bool {
        self.newsletter_segment.is_some()
    }
    pub(super) async fn lock_member_definition(
        &self,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), ApiError> {
        if let Some((id, definition)) = &self.member_segment {
            let current:Option<(i64,bool)>=sqlx::query_as("SELECT authoring_version,COALESCE(is_active,false) FROM member_segments WHERE id=$1 FOR SHARE").bind(id).fetch_optional(connection).await.map_err(db)?;
            if current != Some((definition.version, true)) {
                return Err(crate::utils::errors::conflict(
                    "Member segment changed; reload before submitting",
                ));
            }
        }
        Ok(())
    }
    pub(super) fn is_member(&self) -> bool {
        self.members
    }
    pub(super) fn snapshot(&self) -> Value {
        if self.members {
            json!({"source":"members","segment":self.member_segment.as_ref().map(|(id, definition)|json!({"id":id,"authoring_version":definition.version,"conditions":definition.conditions,"static_id":definition.static_id})),"policy":"confirmed_only"})
        } else {
            json!({"source":if self.is_newsletter(){"newsletter"}else{"contacts"},"policy":"confirmed_only"})
        }
    }
    pub(super) fn query(
        &self,
        prefix: &'static str,
        eligible: bool,
    ) -> Result<QueryBuilder<Postgres>, ApiError> {
        const NEWSLETTER: &str = "FROM (SELECT n.id,n.email,n.name AS first_name,''::text AS last_name,NULL::text AS company,n.status,n.tags FROM (SELECT DISTINCT ON(LOWER(email)) * FROM newsletter_subscribers ORDER BY LOWER(email),updated_at DESC NULLS LAST,id DESC) n) c";
        const MEMBERS: &str = "FROM (SELECT id,email,COALESCE(NULLIF(first_name,''),name) AS first_name,COALESCE(last_name,'') AS last_name,NULL::text AS company,status,tags FROM admin_member_facts) c";
        let sql = if self.members {
            prefix.replace("FROM contacts c", MEMBERS)
        } else if self.newsletter_segment.is_some() {
            prefix.replace("FROM contacts c", NEWSLETTER)
        } else {
            prefix.to_string()
        };
        let mut query = QueryBuilder::new(sql);
        query.push(" WHERE ");
        if let Some((_, definition)) = &self.member_segment {
            query.push("EXISTS(SELECT 1 FROM admin_member_facts m WHERE m.id=c.id");
            member_segmentation::append_definition(&mut query, definition)?;
            query.push(") AND ");
        }
        segment_rules::append_group(&mut query, &self.advanced, "all")?;
        if let Some(tag) = self
            .newsletter_segment
            .as_ref()
            .filter(|tag| !matches!(tag.as_str(), "all" | "confirmed"))
        {
            query.push(" AND c.tags ? ").push_bind(tag.clone());
        }
        query.push(" AND (");
        segment_rules::append_group(&mut query, &self.segment, &self.segment_match)?;
        query.push(")");
        if !self.includes.is_empty() {
            query.push(" AND (");
            append_filters(&mut query, &self.includes);
            query.push(")");
        }
        if !self.excludes.is_empty() {
            query.push(" AND NOT (");
            append_filters(&mut query, &self.excludes);
            query.push(")");
        }
        if eligible {
            query.push(" AND EXISTS(SELECT 1 FROM newsletter_subscribers n WHERE n.id=(SELECT latest.id FROM newsletter_subscribers latest WHERE LOWER(latest.email)=LOWER(c.email) ORDER BY latest.updated_at DESC NULLS LAST,latest.id DESC LIMIT 1) AND n.status='confirmed' AND n.confirmed_at IS NOT NULL AND n.unsubscribed_at IS NULL)");
        }
        Ok(query)
    }
}
pub(super) async fn preview(
    pool: &PgPool,
    value: Option<Value>,
    page: i64,
    size: i64,
) -> Result<Value, ApiError> {
    if page < 1 || !(1..=100).contains(&size) {
        return Err(invalid("Page must be positive and page size must be 1–100"));
    }
    let audience = resolve(pool, value).await?;
    let mut candidates = audience.query(
        "SELECT COUNT(DISTINCT LOWER(c.email)) FROM contacts c",
        false,
    )?;
    let candidate_count: i64 = candidates
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(db)?;
    let mut count = audience.query(
        "SELECT COUNT(DISTINCT LOWER(c.email)) FROM contacts c",
        true,
    )?;
    let total: i64 = count
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(db)?;
    let mut wrapped=audience.query("SELECT to_jsonb(recipient) FROM (SELECT DISTINCT ON(LOWER(c.email)) c.id::text AS id,c.email,c.first_name,c.last_name,COALESCE(NULLIF(trim(concat_ws(' ',c.first_name,c.last_name)),''),c.email) AS full_name,c.company,c.status,c.tags FROM contacts c",true)?;
    wrapped
        .push(" ORDER BY LOWER(c.email),c.id LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind(page.saturating_sub(1).saturating_mul(size))
        .push(") recipient");
    let rows: Vec<Value> = wrapped
        .build_query_scalar()
        .fetch_all(pool)
        .await
        .map_err(db)?;
    Ok(
        json!({"data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":((total+size-1)/size).max(1)},"audience":{"source":if audience.is_member(){"members"}else if audience.is_newsletter(){"newsletter"}else{"live_preview"},"matching_contacts":candidate_count,"eligible_recipients":total,"excluded_without_confirmation":candidate_count-total,"policy":"Only confirmed newsletter subscribers are eligible; unsubscribe is checked again before delivery."}}),
    )
}

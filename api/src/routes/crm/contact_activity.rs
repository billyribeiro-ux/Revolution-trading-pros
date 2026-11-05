//! Recorded contact observations and explicitly configured scoring policies.
use super::super::segment_rules::{self, Condition};
use crate::{
    AppState,
    middleware::admin::AdminUser,
    services::crm_logs,
    utils::errors::{self, ApiError},
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgConnection;

pub(super) fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"CRM contact observation storage failed");
    if error
        .as_database_error()
        .is_some_and(|error| error.is_unique_violation())
    {
        errors::conflict("A contact record changed; reload before retrying")
    } else {
        errors::service_unavailable("Contact storage is unavailable")
    }
}
pub(super) async fn actor(connection: &mut PgConnection, id: i64) -> Result<(), ApiError> {
    sqlx::query("SELECT set_config('app.crm_actor_id',$1,true)")
        .bind(id.to_string())
        .execute(connection)
        .await
        .map_err(db)?;
    Ok(())
}
pub(super) async fn policy(connection: &mut PgConnection) -> Result<Value, ApiError> {
    Ok(sqlx::query_scalar::<_, Option<Value>>(
        "SELECT value->'scoring_settings' FROM settings WHERE key='crm.settings'",
    )
    .fetch_optional(connection)
    .await
    .map_err(db)?
    .flatten()
    .unwrap_or(Value::Null))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Rule {
    id: String,
    points: i32,
    #[serde(default = "all")]
    match_type: String,
    conditions: Vec<Condition>,
}
fn all() -> String {
    "all".into()
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Policy {
    enabled: Option<bool>,
    max_score: Option<i32>,
    rules: Vec<Rule>,
}
fn parse_policy(value: &Value) -> Result<Policy, ApiError> {
    if value.is_null() {
        return Ok(Policy::default());
    }
    if !value.is_object() || value.to_string().len() > 65_536 {
        return Err(errors::validation_error(
            "Scoring settings must be an object under64KiB",
        ));
    }
    if let Some(rules) = value["rules"].as_array() {
        for rule in rules {
            if let Some(conditions) = rule["conditions"].as_array() {
                for condition in conditions {
                    if !condition.is_object()
                        || condition.as_object().is_some_and(|values| {
                            values.keys().any(|key| {
                                !matches!(key.as_str(), "field" | "operator" | "value" | "logic")
                            })
                        })
                    {
                        return Err(errors::validation_error(
                            "Unsupported scoring condition properties",
                        ));
                    }
                }
            }
        }
    }
    let policy: Policy = serde_json::from_value(value.clone())
        .map_err(|_| errors::validation_error("Invalid scoring settings or rule field types"))?;
    if policy
        .max_score
        .is_some_and(|value| !(1..=100_000).contains(&value))
        || policy.rules.len() > 50
    {
        return Err(errors::validation_error(
            "Use max_score1–100000 and at most50scoring rules",
        ));
    }
    let mut ids = std::collections::BTreeSet::new();
    for rule in &policy.rules {
        if rule.id.trim().is_empty()
            || rule.id.chars().count() > 100
            || rule.id.contains(['\0', '\r', '\n'])
            || !ids.insert(&rule.id)
            || !(-100_000..=100_000).contains(&rule.points)
            || !(1..=20).contains(&rule.conditions.len())
            || rule
                .conditions
                .iter()
                .any(|condition| matches!(condition.field.as_str(), "score" | "lead_score"))
        {
            return Err(errors::validation_error(
                "Scoring rules require distinct bounded IDs, explicit points,1–20conditions and cannot depend on scores",
            ));
        }
        let _ = segment_rules::query(
            "SELECT c.id FROM contacts c",
            &rule.conditions,
            &rule.match_type,
        )?;
    }
    if policy.enabled == Some(true) && !policy.rules.is_empty() && policy.max_score.is_none() {
        return Err(errors::validation_error(
            "Configure max_score before calculating rule scores",
        ));
    }
    Ok(policy)
}
pub(super) fn dto(mut contact: Value, current_policy: &Value) -> Value {
    let recorded = contact["score"].clone();
    let assessment = contact["score_assessment"].clone();
    let parsed = parse_policy(current_policy);
    let (status, reason, max_score) = match &parsed {
        Err(_) => (
            "invalid_policy",
            Some("Review the stored scoring policy"),
            None,
        ),
        Ok(policy) if policy.enabled == Some(false) => (
            "disabled",
            Some("Contact scoring is disabled"),
            policy.max_score,
        ),
        Ok(policy) if policy.enabled != Some(true) || policy.rules.is_empty() => (
            "unconfigured",
            Some("No explicit scoring rules are configured"),
            policy.max_score,
        ),
        Ok(policy) => {
            let valid = assessment["status"] == "calculated"
                && assessment["policy"] == *current_policy
                && contact["score_assessed_at"].is_string()
                && assessment["score"] == recorded
                && recorded.as_i64().is_some_and(|score| {
                    score >= 0 && Some(score) <= policy.max_score.map(i64::from)
                });
            if valid {
                ("calculated", None, policy.max_score)
            } else if assessment.is_null() {
                (
                    "not_calculated",
                    Some("Stored raw scores have no current assessment"),
                    policy.max_score,
                )
            } else {
                (
                    "stale",
                    Some("Contact facts or scoring policy changed after this assessment"),
                    policy.max_score,
                )
            }
        }
    };
    let measured = if status == "calculated" {
        recorded.clone()
    } else {
        Value::Null
    };
    contact["recorded_score"] = recorded;
    contact["score"] = measured.clone();
    contact["lead_score"] = measured.clone();
    contact["scoring"] = json!({"status":status,"score":measured,"max_score":max_score,"evaluated_at":contact["score_assessed_at"],"policy_hash":assessment["policy_hash"],"contributions":if status=="calculated"{assessment["contributions"].clone()}else{json!([])},"reason":reason,"scope":"configured_contact_rules"});
    let name = [
        contact["first_name"].as_str().unwrap_or(""),
        contact["last_name"].as_str().unwrap_or(""),
    ]
    .join(" ")
    .trim()
    .to_string();
    contact["full_name"] = json!(if name.is_empty() {
        contact["email"].as_str().unwrap_or("")
    } else {
        &name
    });
    if let Some(values) = contact.as_object_mut() {
        values.remove("score_assessment");
    }
    contact
}
pub(super) async fn facts(connection: &mut PgConnection, id: i64) -> Result<Value, ApiError> {
    sqlx::query_scalar("SELECT jsonb_build_object('scope','recorded_contact_links','deals',jsonb_build_object('total',(SELECT COUNT(*) FROM crm_deals d WHERE d.contact_id=c.id),'open',(SELECT COUNT(*) FROM crm_deals d WHERE d.contact_id=c.id AND d.status='open'),'won',(SELECT COUNT(*) FROM crm_deals d WHERE d.contact_id=c.id AND d.status='won'),'activities',(SELECT COUNT(*) FROM crm_deal_activities a JOIN crm_deals d ON d.id=a.deal_id WHERE d.contact_id=c.id),'notes',(SELECT COUNT(*) FROM crm_deal_activities a JOIN crm_deals d ON d.id=a.deal_id WHERE d.contact_id=c.id AND a.activity_type='note')),'email',jsonb_build_object('scope','crm_outbox','accepted',(SELECT COUNT(*) FROM crm_campaign_deliveries d WHERE d.contact_id=c.id AND d.accepted_at IS NOT NULL),'open_signals',(SELECT COUNT(*) FROM crm_campaign_deliveries d WHERE d.contact_id=c.id AND d.accepted_at IS NOT NULL AND d.opened_at IS NOT NULL),'click_signals',(SELECT COUNT(*) FROM crm_campaign_deliveries d WHERE d.contact_id=c.id AND d.accepted_at IS NOT NULL AND d.clicked_at IS NOT NULL),'human_engagement',NULL),'orders',CASE WHEN c.user_id IS NULL THEN NULL ELSE jsonb_build_object('scope','linked_member_orders','total',(SELECT COUNT(*) FROM orders o WHERE o.user_id=c.user_id),'completed',(SELECT COUNT(*) FROM orders o WHERE o.user_id=c.user_id AND o.status='completed'),'refunded',(SELECT COUNT(*) FROM orders o WHERE o.user_id=c.user_id AND (o.refunded_at IS NOT NULL OR o.refund_amount>0))) END,'unmeasured',jsonb_build_object('health_score',NULL,'engagement_score',NULL,'value_score',NULL,'lifetime_value',NULL,'human_email_opens',NULL)) FROM contacts c WHERE c.id=$1")
        .bind(id).fetch_one(connection).await.map_err(db)
}
pub(super) async fn recalculate(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    // Policy before contact gives configuration changes a consistent lock order.
    let raw = sqlx::query_scalar::<_, Option<Value>>(
        "SELECT value->'scoring_settings' FROM settings WHERE key='crm.settings' FOR SHARE",
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(db)?
    .flatten()
    .unwrap_or(Value::Null);
    let policy = parse_policy(&raw)?;
    let _: i64 = sqlx::query_scalar("SELECT id FROM contacts WHERE id=$1 FOR NO KEY UPDATE")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(|| errors::not_found("Contact"))?;
    actor(&mut tx, admin.0.id).await?;
    let hash = hex::encode(Sha256::digest(
        serde_json::to_vec(&raw).map_err(|_| errors::validation_error("Invalid scoring policy"))?,
    ));
    let mut total = 0_i64;
    let mut contributions = Vec::new();
    let status = if policy.enabled == Some(false) {
        "disabled"
    } else if policy.enabled != Some(true) || policy.rules.is_empty() {
        "unconfigured"
    } else {
        "calculated"
    };
    if status == "calculated" {
        for rule in &policy.rules {
            let mut query = segment_rules::query(
                "SELECT c.id FROM contacts c",
                &rule.conditions,
                &rule.match_type,
            )?;
            query.push(" AND c.id=").push_bind(id);
            let matched = query
                .build_query_scalar::<i64>()
                .fetch_optional(&mut *tx)
                .await
                .map_err(db)?
                .is_some();
            if matched {
                total += i64::from(rule.points);
            }
            contributions.push(json!({"rule_id":rule.id,"matched":matched,"points":rule.points,"applied_points":if matched{rule.points}else{0}}));
        }
    }
    let score = if status == "calculated" {
        let maximum = policy.max_score.ok_or_else(|| {
            errors::validation_error("Configure max_score before calculating rule scores")
        })?;
        Some(total.clamp(0, i64::from(maximum)) as i32)
    } else {
        None
    };
    let assessment = json!({"status":status,"score":score,"policy":raw,"policy_hash":hash,"contributions":contributions,"raw_total":if status=="calculated"{Some(total)}else{None}});
    sqlx::query("SELECT set_config('app.crm_score_assessment','1',true)")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let saved:Value=sqlx::query_scalar("UPDATE contacts c SET score=CASE WHEN $2::integer IS NULL THEN score ELSE $2 END,score_assessment=$3,score_assessed_at=clock_timestamp() WHERE id=$1 RETURNING to_jsonb(c)")
        .bind(id).bind(score).bind(&assessment).fetch_one(&mut *tx).await.map_err(db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "Contact score assessed",
        json!({"contact_id":id,"status":status,"score":score,"policy_hash":hash}),
        Some(admin.0.id),
    )
    .await
    .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(dto(saved, &raw)))
}

macro_rules! timeline_sql { ($tail:literal) => { concat!(r#"WITH events AS (
 SELECT 'contact-'||e.id::text AS id,e.kind AS type,'contact'::text AS category,
 CASE e.kind WHEN 'created' THEN 'Contact record created' WHEN 'updated' THEN 'Contact fields updated' WHEN 'deleted' THEN 'Contact record deleted' ELSE 'Contact score assessed' END AS title,
 NULL::text AS description,e.metadata,e.occurred_at,e.created_by AS actor_id FROM crm_contact_events e WHERE e.contact_id=$1
 UNION ALL SELECT 'legacy-created-'||c.id,'created','contact','Stored contact creation',NULL,jsonb_build_object('source','contacts.created_at'),c.created_at AT TIME ZONE 'UTC',NULL FROM contacts c WHERE c.id=$1 AND c.created_at IS NOT NULL AND NOT EXISTS(SELECT 1 FROM crm_contact_events e WHERE e.contact_id=c.id AND e.kind='created')
 UNION ALL SELECT 'deal-activity-'||a.id,a.activity_type,'deal',COALESCE(a.title,'Recorded deal activity'),LEFT(a.description,10000),jsonb_build_object('deal_id',d.id::text,'description_truncated',length(a.description)>10000),a.created_at,a.created_by FROM crm_deal_activities a JOIN crm_deals d ON d.id=a.deal_id WHERE d.contact_id=$1 AND a.activity_type<>'stage_changed' AND a.created_at IS NOT NULL
 UNION ALL SELECT 'deal-stage-'||h.id,'deal_stage_changed','deal','Deal stage changed',LEFT(h.reason,10000),jsonb_build_object('deal_id',d.id::text,'from_stage_id',h.from_stage_id,'to_stage_id',h.to_stage_id),h.created_at,h.changed_by FROM crm_deal_stage_history h JOIN crm_deals d ON d.id=h.deal_id WHERE d.contact_id=$1 AND h.created_at IS NOT NULL
 UNION ALL SELECT 'delivery-'||d.id||'-'||signal.kind,signal.kind,'email',signal.title,NULL,jsonb_build_object('source','crm_outbox','delivery_id',d.id,'campaign_id',d.campaign_id::text,'human_engagement',NULL),signal.at,NULL FROM crm_campaign_deliveries d CROSS JOIN LATERAL (VALUES ('email_accepted',d.accepted_at,'Provider accepted message'),('email_open_signal',d.opened_at,'Provider recorded an open signal'),('email_click_signal',d.clicked_at,'Provider recorded a click signal')) AS signal(kind,at,title) WHERE d.contact_id=$1 AND d.accepted_at IS NOT NULL AND signal.at IS NOT NULL
 UNION ALL SELECT 'application-email-'||e.id||'-'||signal.kind,signal.kind,'email',signal.title,NULL,jsonb_build_object('source','linked_member_email_log','email_log_id',e.id::text,'human_engagement',NULL),signal.at,NULL FROM email_logs e JOIN contacts c ON c.user_id=e.user_id CROSS JOIN LATERAL (VALUES ('email_accepted',e.sent_at,'Provider accepted application message'),('email_open_signal',e.opened_at AT TIME ZONE 'UTC','Provider recorded an application open signal'),('email_click_signal',e.clicked_at AT TIME ZONE 'UTC','Provider recorded an application click signal')) AS signal(kind,at,title) WHERE c.id=$1 AND e.sent_at IS NOT NULL AND NULLIF(e.provider_message_id,'') IS NOT NULL AND e.status IN ('sent','delivered','opened','clicked','bounced') AND COALESCE(e.template_type,'') NOT IN ('native_campaign_test','sequence_test') AND signal.at IS NOT NULL
 UNION ALL SELECT 'order-'||o.id||'-'||signal.kind,signal.kind,'order',signal.title,NULL,jsonb_build_object('source','linked_member_orders','order_id',o.id::text,'order_number',o.order_number,'currency',o.currency),signal.at,NULL FROM orders o JOIN contacts c ON c.user_id=o.user_id CROSS JOIN LATERAL (VALUES ('order_created',o.created_at AT TIME ZONE 'UTC','Linked member order created'),('order_completed',o.completed_at AT TIME ZONE 'UTC','Order completion recorded'),('order_refunded',o.refunded_at AT TIME ZONE 'UTC','Order refund recorded')) AS signal(kind,at,title) WHERE c.id=$1 AND signal.at IS NOT NULL
) "#, $tail) }; }
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}
pub(super) async fn timeline(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<Pagination>,
) -> Result<Json<Value>, ApiError> {
    let page = query.page.unwrap_or(1);
    let size = query.per_page.unwrap_or(50);
    if !(1..=100_000).contains(&page) || !(1..=100).contains(&size) {
        return Err(errors::validation_error(
            "Choose a positive bounded page and page size1–100",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM contacts WHERE id=$1)")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
    if !exists {
        return Err(errors::not_found("Contact"));
    }
    let total: i64 = sqlx::query_scalar(timeline_sql!("SELECT COUNT(*) FROM events"))
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
    let rows:Vec<Value>=sqlx::query_scalar(timeline_sql!("SELECT jsonb_build_object('id',e.id,'type',e.type,'category',e.category,'title',e.title,'description',e.description,'metadata',e.metadata,'occurred_at',e.occurred_at,'created_by',CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id::text,'name',u.name) END) FROM events e LEFT JOIN users u ON u.id=e.actor_id ORDER BY e.occurred_at DESC,e.id DESC LIMIT $2 OFFSET $3"))
        .bind(id).bind(size).bind((page-1)*size).fetch_all(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"contact_id":id.to_string(),"scope":"recorded_contact_links","data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"last_page":((total+size-1)/size).max(1)},"coverage":{"sources":["contact_journal","linked_deal_activity","crm_outbox_receipts","linked_member_email_logs","linked_member_orders"],"human_email_engagement":null,"unlinked_address_history":null}}),
    ))
}

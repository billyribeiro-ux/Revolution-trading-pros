//! One factual member profile snapshot; account, billing and observations stay distinct.
use super::history::{self, HistoryQuery, Kind};
use crate::{AppState, utils::errors::ApiError};
use serde_json::{Value, json};
use sqlx::PgConnection;

const MEMBER: &str = r#"SELECT jsonb_build_object(
'id',u.id,'name',u.name,'first_name',m.first_name,'last_name',m.last_name,'email',u.email,
'role',u.role,'avatar',u.avatar_url,'email_verified',u.email_verified_at IS NOT NULL,
'created_at',u.created_at,'updated_at',u.updated_at,'joined_at',m.joined_at,
'status',m.status,'active_subscriptions_count',m.active_subscriptions_count,
'current_plan',m.current_plan,'current_product',m.current_product,'last_login',m.last_login,
'tags',m.tags,'tag_records',COALESCE((SELECT jsonb_agg(jsonb_build_object('id',t.id,'name',t.name,'slug',t.slug) ORDER BY t.name,t.id) FROM user_member_tags a JOIN member_tags t ON t.id=a.tag_id WHERE a.user_id=u.id),'[]'::jsonb),
'account',jsonb_build_object('version',u.account_version,'is_active',u.is_active,'mfa_enabled',u.mfa_enabled,
'recorded_restriction',s.status,'reason',s.status_reason,'ban_reason',s.ban_reason,
'banned_until',to_char(s.ban_expires_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"')),
'recorded_age_days',CASE WHEN isfinite(u.created_at) AND u.created_at<=NOW() AT TIME ZONE 'UTC' THEN FLOOR(EXTRACT(EPOCH FROM ((NOW() AT TIME ZONE 'UTC')-u.created_at))/86400)::bigint ELSE NULL END)
FROM users u JOIN admin_member_facts m ON m.id=u.id LEFT JOIN user_status s ON s.user_id=u.id
WHERE u.id=$1 AND u.deleted_at IS NULL AND COALESCE(LOWER(u.role),'')<>'deleted'"#;

const TIMELINE: &str = r#"WITH events AS(
 SELECT 'activity:'||a.id AS key,a.created_at AS sort_at,jsonb_build_object('id','activity:'||a.id,'type','activity','title',a.action,'date',to_char(a.created_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'timestamp_basis','recorded UTC instant','meta',a.details) AS item
 FROM user_activity_log a WHERE a.user_id=$1
 UNION ALL
 SELECT 'order:'||o.id,o.created_at AT TIME ZONE 'UTC',jsonb_build_object('id','order:'||o.id,'type','order','title','Order record created','date',o.created_at,'timestamp_basis','recorded without timezone','meta',jsonb_build_object('order_id',o.id,'order_number',o.order_number,'current_status',o.status,'currency',o.currency,'recorded_total_units',o.total::text))
 FROM orders o WHERE o.user_id=$1
 UNION ALL
 SELECT 'membership:'||m.id,m.created_at AT TIME ZONE 'UTC',jsonb_build_object('id','membership:'||m.id,'type','membership','title','Membership record created','date',m.created_at,'timestamp_basis','recorded without timezone','meta',jsonb_build_object('membership_id',m.id,'plan_id',m.plan_id,'current_status',m.status,'deleted_at',m.deleted_at))
 FROM user_memberships m WHERE m.user_id=$1
 UNION ALL
 SELECT 'cancellation:'||m.id,m.cancelled_at AT TIME ZONE 'UTC',jsonb_build_object('id','cancellation:'||m.id,'type','membership','title','Recorded membership cancellation','date',m.cancelled_at,'timestamp_basis','recorded without timezone','meta',jsonb_build_object('membership_id',m.id,'plan_id',m.plan_id))
 FROM user_memberships m WHERE m.user_id=$1 AND m.cancelled_at IS NOT NULL
 UNION ALL
 SELECT 'note:'||n.id,n.created_at,jsonb_build_object('id','note:'||n.id,'type','note','title','Member note created','date',to_char(n.created_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'timestamp_basis','recorded UTC instant','meta',jsonb_build_object('note_id',n.id,'created_by',n.created_by))
 FROM member_notes n WHERE n.user_id=$1 AND NOT EXISTS(SELECT 1 FROM user_activity_log a WHERE a.user_id=n.user_id AND a.entity_type='member_note' AND a.entity_id=n.id AND a.action='member_note_created')
) SELECT jsonb_build_object('schema_version',1,'metrics_scope','member_recorded_timeline','member_id',$1::bigint,
'data',COALESCE((SELECT jsonb_agg(item ORDER BY sort_at DESC NULLS LAST,key DESC) FROM(SELECT * FROM events ORDER BY sort_at DESC NULLS LAST,key DESC LIMIT $2 OFFSET $3)selected),'[]'::jsonb),
'total',(SELECT COUNT(*) FROM events),
'coverage','Recorded creation/cancellation facts and journal entries only; current status is not presented as a historical status change. Legacy naive dates use the UTC convention for ordering, not a verified original timezone.')"#;

pub(crate) async fn timeline_on(
    connection: &mut PgConnection,
    member: i64,
    page: i64,
    size: i64,
) -> Result<Value, ApiError> {
    if page < 1 || !(1..=100).contains(&size) {
        return Err(ApiError::validation_error(
            "Use a positive timeline page and a page size from 1 to 100",
        ));
    }
    let offset = (page - 1)
        .checked_mul(size)
        .ok_or_else(|| ApiError::validation_error("Requested timeline page is too large"))?;
    history::exists_on(connection, member).await?;
    let mut result: Value = sqlx::query_scalar(TIMELINE)
        .bind(member)
        .bind(size)
        .bind(offset)
        .fetch_one(connection)
        .await
        .map_err(history::db)?;
    let total = result["total"]
        .as_i64()
        .ok_or_else(|| ApiError::internal_error("Recorded timeline count is unavailable"))?;
    result["pagination"] = json!({"current_page":page,"per_page":size,"total":total,"last_page":(total/size+i64::from(total%size!=0)).max(1)});
    Ok(result)
}

pub(crate) async fn get(state: &AppState, member: i64) -> Result<Value, ApiError> {
    history::positive(member)?;
    let mut tx = state.db.pool.begin().await.map_err(history::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(history::db)?;
    let person: Value = sqlx::query_scalar(MEMBER)
        .bind(member)
        .fetch_optional(&mut *tx)
        .await
        .map_err(history::db)?
        .ok_or_else(|| ApiError::not_found("Member not found"))?;
    let money = crate::routes::member_statistics::recorded_order_value_on(&mut tx, member).await?;
    let query = HistoryQuery {
        page: Some(1),
        per_page: Some(25),
        action: None,
    };
    let memberships = history::read_on(&mut tx, member, &query, Kind::Memberships).await?;
    let orders = history::read_on(&mut tx, member, &query, Kind::Orders).await?;
    let notes = history::read_on(&mut tx, member, &query, Kind::Notes).await?;
    let activity = history::read_on(&mut tx, member, &query, Kind::Activity).await?;
    let emails = history::read_on(&mut tx, member, &query, Kind::Emails).await?;
    let timeline = timeline_on(&mut tx, member, 1, 25).await?;
    let observations:Value=sqlx::query_scalar("SELECT jsonb_build_object('linked_email_receipts',COUNT(*),'stored_open_signals',COUNT(*) FILTER(WHERE opened_at IS NOT NULL),'stored_click_signals',COUNT(*) FILTER(WHERE clicked_at IS NOT NULL),'scope','email_logs linked by recorded user_id; signals do not establish human engagement') FROM email_logs WHERE user_id=$1")
        .bind(member).fetch_one(&mut *tx).await.map_err(history::db)?;
    let evaluated_at: String = sqlx::query_scalar(
        r#"SELECT to_char(NOW() AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"')"#,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(history::db)?;
    tx.commit().await.map_err(history::db)?;
    Ok(
        json!({"schema_version":1,"metrics_scope":"member_profile","member_id":member,"evaluated_at":evaluated_at,
        "member":person,"recorded_order_value":money,"memberships":memberships,"orders":orders,
        "notes":notes,"activity":activity,"emails":emails,"timeline":timeline,
        "engagement":{"score":null,"reason":"No measured member engagement model is configured; journal entries and email observations are not a human engagement percentage","observations":observations},
        "timestamp_basis":"Member/order/membership legacy timestamps have no recorded timezone; no historical timezone is invented. Journal/notes and sent/queued receipt instants carry UTC explicitly.",
        "billing_coverage":"Mutable catalogue prices are not charged amounts. Entitlement expiry and current-period ends are not promised next-payment dates. Account restriction state is separate from membership state."}),
    )
}

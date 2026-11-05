//! Native campaign receipt cohorts and bounded UTC event observations.
use super::*;
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct EmailAnalyticsQuery {
    period: Option<String>,
}
impl EmailAnalyticsQuery {
    fn days(&self) -> Result<i32, ApiError> {
        match self.period.as_deref().unwrap_or("30d") {
            "7d" => Ok(7),
            "30d" => Ok(30),
            "90d" => Ok(90),
            "365d" => Ok(365),
            _ => Err(invalid("Choose a7d,30d,90d or365d reporting period")),
        }
    }
}
async fn aggregate_on(
    connection: &mut sqlx::PgConnection,
    id: Option<i64>,
    days: i32,
) -> Result<Value, ApiError> {
    sqlx::query_scalar(r#"
WITH bounds AS (SELECT NOW() finish,NOW()-make_interval(hours=>24*$2) start,NOW()-make_interval(hours=>48*$2) previous_start),
 c AS (SELECT c.id,c.name,c.delivery_campaign_id,c.created_at AT TIME ZONE 'UTC' created_at,COALESCE(engine.status,c.status,'draft') status FROM email_campaigns c LEFT JOIN crm_campaigns engine ON engine.id=c.delivery_campaign_id WHERE ($1::bigint IS NULL OR c.id=$1)),
 receipts AS (SELECT d.*,c.name,c.id native_id FROM crm_campaign_deliveries d JOIN c ON c.delivery_campaign_id=d.campaign_id CROSS JOIN bounds b WHERE d.accepted_at>=b.previous_start AND d.accepted_at<b.finish),
 current_cohort AS (SELECT COUNT(*) sent,COUNT(*) FILTER(WHERE d.opened_at<=b.finish) opened,COUNT(*) FILTER(WHERE d.clicked_at<=b.finish) clicked FROM receipts d CROSS JOIN bounds b WHERE d.accepted_at>=b.start),
 previous_cohort AS (SELECT COUNT(*) sent,COUNT(*) FILTER(WHERE d.opened_at<=b.finish) opened,COUNT(*) FILTER(WHERE d.clicked_at<=b.finish) clicked FROM receipts d CROSS JOIN bounds b WHERE d.accepted_at<b.start),
 e AS (SELECT e.*,COALESCE(e.details->>'original_link',l.target) url FROM crm_campaign_events e JOIN crm_campaign_deliveries d ON d.id=e.delivery_id AND d.attempt_count>0 JOIN c ON c.delivery_campaign_id=d.campaign_id CROSS JOIN bounds b LEFT JOIN crm_campaign_tracking_links l ON l.delivery_id=e.delivery_id AND l.id::text=e.details->>'link_id' WHERE e.occurred_at>=b.start AND e.occurred_at<b.finish AND e.record_type IN ('Open','Click')),
 daily AS (SELECT to_char(occurred_at AT TIME ZONE 'UTC','YYYY-MM-DD') date,COUNT(*) FILTER(WHERE record_type='Open') opens,COUNT(*) FILTER(WHERE record_type='Click') clicks FROM e GROUP BY 1),
 hourly AS (SELECT to_char(occurred_at AT TIME ZONE 'UTC','YYYY-MM-DD') date,EXTRACT(HOUR FROM occurred_at AT TIME ZONE 'UTC')::integer AS hour,CASE WHEN record_type='Open' THEN 'opened' ELSE 'clicked' END event_type,COUNT(*) count FROM e GROUP BY 1,2,3),
 top AS (SELECT native_id id,name,COUNT(*) sent,COUNT(*) FILTER(WHERE opened_at<=b.finish) opens,ROUND(100.0*COUNT(*) FILTER(WHERE opened_at<=b.finish)/COUNT(*),2) open_rate FROM receipts CROSS JOIN bounds b WHERE accepted_at>=b.start GROUP BY native_id,name ORDER BY opens DESC,sent DESC,id LIMIT 10),
 devices AS (SELECT COUNT(*) FILTER(WHERE LOWER(details->>'platform')='desktop') desktop,COUNT(*) FILTER(WHERE LOWER(details->>'platform')='mobile') mobile,COUNT(*) FILTER(WHERE LOWER(details->>'platform')='tablet') tablet,COUNT(*) FILTER(WHERE COALESCE(LOWER(details->>'platform'),'') NOT IN ('desktop','mobile','tablet')) unknown FROM e WHERE record_type='Open'),
 clients AS (SELECT details->>'client' client,COUNT(*) count FROM e WHERE record_type='Open' AND details->>'client' IS NOT NULL GROUP BY 1 ORDER BY count DESC,client LIMIT 100),
 links AS (SELECT url link_url,COUNT(*) clicks FROM e WHERE record_type='Click' AND url IS NOT NULL GROUP BY 1 ORDER BY clicks DESC,link_url LIMIT 100),
 geo AS (SELECT details->>'country' country,COUNT(*) count FROM e WHERE record_type='Open' AND details->>'country' IS NOT NULL GROUP BY 1 ORDER BY count DESC,country LIMIT 100)
SELECT jsonb_build_object('metrics_scope','native_campaign_outbox','period',$2::text||'d','window',jsonb_build_object('start',b.start,'end',b.finish,'previous_start',b.previous_start,'timezone','UTC'),
 'campaigns',(SELECT COUNT(*) FROM c WHERE status IN ('scheduled','sending')),
 'new_campaigns',(SELECT COUNT(*) FROM c WHERE created_at>=b.start AND created_at<b.finish),
 'emails_sent',cur.sent,'total_opens',(SELECT COUNT(*) FROM e WHERE record_type='Open'),'total_clicks',(SELECT COUNT(*) FROM e WHERE record_type='Click'),
 'average_open_rate',ROUND(100.0*cur.opened/NULLIF(cur.sent,0),2),'average_click_rate',ROUND(100.0*cur.clicked/NULLIF(cur.sent,0),2),
 'comparison',jsonb_build_object('emails_sent_change_pct',ROUND(100.0*(cur.sent-prev.sent)/NULLIF(prev.sent,0),2),
 'open_rate_change_points',ROUND(100.0*cur.opened/NULLIF(cur.sent,0)-100.0*prev.opened/NULLIF(prev.sent,0),2),
 'click_rate_change_points',ROUND(100.0*cur.clicked/NULLIF(cur.sent,0)-100.0*prev.clicked/NULLIF(prev.sent,0),2)),
 'top_campaigns',COALESCE((SELECT jsonb_agg(to_jsonb(top) ORDER BY opens DESC,sent DESC,id) FROM top),'[]'::jsonb),
 'engagement_trend',COALESCE((SELECT jsonb_agg(to_jsonb(daily) ORDER BY date) FROM daily),'[]'::jsonb),
 'hourly_stats',COALESCE((SELECT jsonb_agg(to_jsonb(hourly) ORDER BY date,hour,event_type) FROM hourly),'[]'::jsonb),
 'device_breakdown',(SELECT to_jsonb(devices) FROM devices),
 'client_breakdown',COALESCE((SELECT jsonb_object_agg(client,count) FROM clients),'{}'::jsonb),
 'unlisted_client_observations',(SELECT COUNT(*) FROM e WHERE record_type='Open')-COALESCE((SELECT SUM(count) FROM clients),0),
 'top_links',COALESCE((SELECT jsonb_agg(to_jsonb(links) ORDER BY clicks DESC,link_url) FROM links),'[]'::jsonb),
 'geo_stats',COALESCE((SELECT jsonb_agg(to_jsonb(geo) ORDER BY count DESC,country) FROM geo),'[]'::jsonb))
FROM bounds b CROSS JOIN current_cohort cur CROSS JOIN previous_cohort prev
"#).bind(id).bind(days).fetch_one(connection).await.map_err(db)
}
pub(crate) async fn global_analytics(
    State(state): State<AppState>,
    _: AdminUser,
    Query(query): Query<EmailAnalyticsQuery>,
) -> Result<Json<Value>, ApiError> {
    let days = query.days()?;
    let mut connection = state.db.pool.acquire().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"analytics":aggregate_on(&mut connection,None,days).await?}),
    ))
}
async fn campaign_analytics(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<EmailAnalyticsQuery>,
) -> Result<Json<Value>, ApiError> {
    let days = query.days()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let report = super::reports::data_on(&mut tx, id).await?;
    let window = aggregate_on(&mut tx, Some(id), days).await?;
    let metrics = &report["metrics"];
    let rates = &report["rates"];
    let devices = ["desktop", "mobile", "tablet", "unknown"]
        .map(|kind| json!({"device_type":kind,"count":window["device_breakdown"][kind]}));
    let data = json!({"campaign":{"id":report["id"],"name":report["name"],"sent_count":metrics["sent"],"opened_count":metrics["unique_opens"],"clicked_count":metrics["unique_clicks"],"bounced_count":metrics["bounced"],"unsubscribed_count":metrics["unsubscribed"],"open_rate":rates["open_rate"],"click_rate":rates["click_rate"],"bounce_rate":rates["bounce_rate"]},"hourly_stats":window["hourly_stats"],"device_stats":devices,"top_links":window["top_links"],"geo_stats":window["geo_stats"],"window":window["window"],"period":window["period"],"metrics_scope":"recorded_outbox"});
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"success":true,"data":data})))
}
pub fn router() -> Router<AppState> {
    Router::new().route("/{id}/analytics", get(campaign_analytics))
}

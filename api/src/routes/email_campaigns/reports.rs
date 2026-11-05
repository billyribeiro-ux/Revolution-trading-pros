//! Reports count durable receipts and observed events in a single database snapshot.
use super::*;

pub(super) async fn data_on(
    connection: &mut sqlx::PgConnection,
    id: i64,
) -> Result<Value, ApiError> {
    let report: Option<Value> = sqlx::query_scalar(r#"
WITH c AS (
 SELECT c.*,COALESCE(d.status,c.status,'draft') current_status,
 COALESCE(d.sent_at,c.sent_at AT TIME ZONE 'UTC') actual_sent_at,
 COALESCE(d.scheduled_at,c.scheduled_at AT TIME ZONE 'UTC') actual_scheduled_at,
 d.last_error,d.pause_reason,t.name template_name,s.name segment_name
 FROM email_campaigns c LEFT JOIN crm_campaigns d ON d.id=c.delivery_campaign_id
 LEFT JOIN email_templates t ON t.id=c.template_id LEFT JOIN member_segments s ON s.id=c.segment_id WHERE c.id=$1
), d AS (
 SELECT r.* FROM crm_campaign_deliveries r JOIN c ON c.delivery_campaign_id=r.campaign_id
), e AS (
 SELECT e.*,COALESCE(e.details->>'original_link',l.target) url
 FROM crm_campaign_events e JOIN d ON d.id=e.delivery_id AND d.attempt_count>0
 LEFT JOIN crm_campaign_tracking_links l ON l.delivery_id=e.delivery_id AND l.id::text=e.details->>'link_id'
), m AS (
 SELECT COUNT(*) FILTER(WHERE accepted_at IS NOT NULL) sent,
 COUNT(*) FILTER(WHERE delivered_at IS NOT NULL) delivered,
 COUNT(*) FILTER(WHERE opened_at IS NOT NULL) unique_opens,
 COUNT(*) FILTER(WHERE clicked_at IS NOT NULL) unique_clicks,
 COUNT(*) FILTER(WHERE bounced_at IS NOT NULL) bounced,
 COUNT(*) FILTER(WHERE complained_at IS NOT NULL) complained,
 COUNT(*) FILTER(WHERE unsubscribed_at IS NOT NULL) unsubscribed,
 COUNT(*) FILTER(WHERE status='queued') queued,COUNT(*) FILTER(WHERE status='processing') processing,
 COUNT(*) FILTER(WHERE status='failed') failed,COUNT(*) FILTER(WHERE status='unknown') unknown,
 COUNT(*) FILTER(WHERE status='skipped') skipped,COUNT(*) total,MIN(accepted_at) first_accepted_at FROM d
), ev AS (
 SELECT COUNT(*) FILTER(WHERE record_type='Open') opened,COUNT(*) FILTER(WHERE record_type='Click') clicked FROM e
), timeline AS (
 SELECT FLOOR(EXTRACT(EPOCH FROM (e.occurred_at-m.first_accepted_at))/3600)::integer AS hour,
 COUNT(*) FILTER(WHERE record_type='Open') opens,COUNT(*) FILTER(WHERE record_type='Click') clicks
 FROM e CROSS JOIN m WHERE record_type IN ('Open','Click') AND occurred_at>=m.first_accepted_at
 AND occurred_at<m.first_accepted_at+INTERVAL '48 hours' GROUP BY 1
), links AS (
 SELECT url,COUNT(*) clicks,COUNT(DISTINCT delivery_id) unique_clicks FROM e
 WHERE record_type='Click' AND url IS NOT NULL GROUP BY url ORDER BY clicks DESC,url LIMIT 100
), devices AS (
 SELECT COUNT(*) FILTER(WHERE LOWER(details->>'platform')='desktop') desktop,
 COUNT(*) FILTER(WHERE LOWER(details->>'platform')='mobile') mobile,
 COUNT(*) FILTER(WHERE LOWER(details->>'platform')='tablet') tablet,
 COUNT(*) FILTER(WHERE COALESCE(LOWER(details->>'platform'),'') NOT IN ('desktop','mobile','tablet')) unknown
 FROM e WHERE record_type='Open'
), clients AS (
 SELECT details->>'client' client,COUNT(*) count FROM e WHERE record_type='Open' AND details->>'client' IS NOT NULL
 GROUP BY 1 ORDER BY count DESC,client LIMIT 100
), geo AS (
 SELECT details->>'country' country,details->>'country_code' country_code,
 COUNT(*) FILTER(WHERE record_type='Open') opens,COUNT(*) FILTER(WHERE record_type='Click') clicks
 FROM e WHERE record_type IN ('Open','Click') AND details->>'country' IS NOT NULL AND details->>'country_code' IS NOT NULL
 GROUP BY 1,2 ORDER BY opens DESC,clicks DESC,country LIMIT 100
), variants AS (
 SELECT v.variant,jsonb_build_object('subject',COALESCE(MIN(d.subject_template),CASE WHEN v.variant='a' THEN c.subject ELSE c.authoring_config->'ab_test_config'->>'subject_b' END),
 'sent',COUNT(d.id) FILTER(WHERE accepted_at IS NOT NULL),'opens',COUNT(d.id) FILTER(WHERE opened_at IS NOT NULL),
 'clicks',COUNT(d.id) FILTER(WHERE clicked_at IS NOT NULL),
 'open_rate',ROUND(100.0*COUNT(d.id) FILTER(WHERE opened_at IS NOT NULL)/NULLIF(COUNT(d.id) FILTER(WHERE accepted_at IS NOT NULL),0),2),
 'click_rate',ROUND(100.0*COUNT(d.id) FILTER(WHERE clicked_at IS NOT NULL)/NULLIF(COUNT(d.id) FILTER(WHERE accepted_at IS NOT NULL),0),2)) value
 FROM c CROSS JOIN (VALUES('a'),('b')) v(variant) LEFT JOIN d ON d.variant=v.variant GROUP BY v.variant,c.subject,c.authoring_config
)
SELECT jsonb_build_object('id',c.id,'name',c.name,'subject',c.subject,'status',c.current_status,
 'created_at',c.created_at AT TIME ZONE 'UTC','sent_at',c.actual_sent_at,'scheduled_at',c.actual_scheduled_at,
 'segment_name',c.segment_name,'template_name',c.template_name,'metrics_scope','recorded_outbox',
 'last_error',c.last_error,'pause_reason',c.pause_reason,
 'delivery_stats',jsonb_build_object('total',m.total,'queued',m.queued,'processing',m.processing,'failed',m.failed,'unknown',m.unknown,'skipped',m.skipped),
 'historical_counts',jsonb_build_object('sent',c.sent_count,'opened',c.open_count,'clicked',c.click_count,'bounced',c.bounce_count,'unsubscribed',c.unsubscribe_count),
 'metrics',jsonb_build_object('sent',m.sent,'delivered',m.delivered,'opened',ev.opened,'unique_opens',m.unique_opens,'clicked',ev.clicked,'unique_clicks',m.unique_clicks,
 'bounced',m.bounced,'soft_bounced',CASE WHEN EXISTS(SELECT 1 FROM e WHERE details->>'bounce_type' IS NOT NULL) THEN (SELECT COUNT(*) FROM e WHERE record_type='Bounce' AND details->>'bounce_type'='SoftBounce') ELSE NULL END,
 'hard_bounced',CASE WHEN EXISTS(SELECT 1 FROM e WHERE details->>'bounce_type' IS NOT NULL) THEN (SELECT COUNT(*) FROM e WHERE record_type='Bounce' AND details->>'bounce_type'='HardBounce') ELSE NULL END,
 'unsubscribed',m.unsubscribed,'complained',m.complained),
 'rates',jsonb_build_object('delivery_rate',ROUND(100.0*m.delivered/NULLIF(m.sent,0),2),
 'open_rate',ROUND(100.0*m.unique_opens/NULLIF(m.sent,0),2),'unique_open_rate',ROUND(100.0*m.unique_opens/NULLIF(m.sent,0),2),
 'click_rate',ROUND(100.0*m.unique_clicks/NULLIF(m.sent,0),2),'unique_click_rate',ROUND(100.0*m.unique_clicks/NULLIF(m.sent,0),2),
 'click_to_open_rate',ROUND(100.0*m.unique_clicks/NULLIF(m.unique_opens,0),2),'bounce_rate',ROUND(100.0*m.bounced/NULLIF(m.sent,0),2),'unsubscribe_rate',ROUND(100.0*m.unsubscribed/NULLIF(m.sent,0),2)),
 'engagement_over_time',COALESCE((SELECT jsonb_agg(to_jsonb(timeline) ORDER BY hour) FROM timeline),'[]'::jsonb),
 'top_links',COALESCE((SELECT jsonb_agg(to_jsonb(links) ORDER BY clicks DESC,url) FROM links),'[]'::jsonb),
 'device_breakdown',(SELECT to_jsonb(devices) FROM devices),
 'email_client_breakdown',COALESCE((SELECT jsonb_agg(jsonb_build_object('client',client,'count',count,'percentage',ROUND(100.0*count/NULLIF(ev.opened,0),2)) ORDER BY count DESC,client) FROM clients),'[]'::jsonb),
 'geographic_distribution',COALESCE((SELECT jsonb_agg(to_jsonb(geo) ORDER BY opens DESC,clicks DESC,country) FROM geo),'[]'::jsonb),
 'ab_test_results',CASE WHEN c.authoring_config->'ab_test_config'->>'enabled'='true' THEN jsonb_build_object('enabled',true,'winner',NULL,'variant_a',(SELECT value FROM variants WHERE variant='a'),'variant_b',(SELECT value FROM variants WHERE variant='b')) ELSE NULL END
) FROM c CROSS JOIN m CROSS JOIN ev
"#).bind(id).fetch_optional(connection).await.map_err(db)?;
    report.ok_or_else(|| ApiError::not_found("Campaign"))
}
async fn report(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut connection = state.db.pool.acquire().await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"data":data_on(&mut connection,id).await?}),
    ))
}
pub fn router() -> Router<AppState> {
    Router::new().route("/{id}/report", get(report))
}

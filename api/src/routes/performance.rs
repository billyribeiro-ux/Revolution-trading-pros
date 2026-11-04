//! First-party performance observations and authenticated operational measurements.
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    Json, Router,
    body::Bytes,
    extract::{DefaultBodyLimit, Path, Query, State},
    http::HeaderMap,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    sync::{LazyLock, Mutex},
    time::{Duration, Instant},
};

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Performance persistence failed");
    ApiError::database_error("Unable to access performance observations")
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
const VITALS: &[(&str, f64, f64)] = &[
    ("LCP", 2500., 4000.),
    ("FID", 100., 300.),
    ("CLS", 0.1, 0.25),
    ("INP", 200., 500.),
    ("TTFB", 800., 1800.),
    ("FCP", 1800., 3000.),
];
fn rating(name: &str, value: f64) -> Option<&'static str> {
    VITALS
        .iter()
        .find(|(key, _, _)| key == &name)
        .map(|(_, good, poor)| {
            if value <= *good {
                "good"
            } else if value <= *poor {
                "needs_improvement"
            } else {
                "poor"
            }
        })
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Sample {
    #[serde(alias = "name")]
    metric: String,
    value: f64,
    id: Option<String>,
    url: Option<String>,
    timestamp: Option<i64>,
    #[serde(rename = "userAgent")]
    user_agent: Option<String>,
    #[serde(rename = "navigationType")]
    navigation_type: Option<String>,
    rating: Option<String>,
    delta: Option<f64>,
    connection: Option<String>,
    #[serde(rename = "deviceMemory")]
    device_memory: Option<f64>,
}
fn normalized_url(value: &str) -> Result<String, ApiError> {
    if value.len() > 2048 || value.chars().any(char::is_control) {
        return Err(invalid("Invalid observation URL"));
    }
    let mut url = reqwest::Url::parse(value)
        .map_err(|_| invalid("Observation URL must be absolute HTTP or HTTPS"))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(invalid("Invalid observation URL"));
    }
    url.set_query(None);
    url.set_fragment(None);
    Ok(url.to_string())
}
async fn ingest(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<Value>, ApiError> {
    let input: Sample = serde_json::from_slice(&body)
        .map_err(|_| invalid("Invalid performance observation JSON"))?;
    if input.metric.is_empty()
        || input.metric.len() > 80
        || !input
            .metric
            .bytes()
            .all(|v| v.is_ascii_alphanumeric() || b"_.:-".contains(&v))
        || !input.value.is_finite()
        || !(0.0..=86_400_000.0).contains(&input.value)
        || input
            .delta
            .is_some_and(|v| !v.is_finite() || v.abs() > 86_400_000.0)
        || input
            .device_memory
            .is_some_and(|v| !v.is_finite() || !(0.0..=1024.0).contains(&v))
        || input.rating.as_ref().is_some_and(|v| {
            !matches!(
                v.as_str(),
                "good" | "needs-improvement" | "needs_improvement" | "poor"
            )
        })
        || input
            .connection
            .as_ref()
            .is_some_and(|v| v.len() > 32 || v.contains('\0'))
        || input
            .user_agent
            .as_ref()
            .is_some_and(|v| v.len() > 2048 || v.contains('\0'))
        || input
            .navigation_type
            .as_ref()
            .is_some_and(|v| v.len() > 80 || v.contains('\0'))
    {
        return Err(invalid(
            "Invalid metric name, value or observation metadata",
        ));
    }
    let id = input.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    if id.is_empty() || id.len() > 128 || id.chars().any(char::is_control) {
        return Err(invalid(
            "Measurement ID must contain 1–128 bytes without control characters",
        ));
    }
    let now = Utc::now();
    let observed = input
        .timestamp
        .map(|time| {
            DateTime::from_timestamp_millis(time)
                .ok_or_else(|| invalid("Invalid observation timestamp"))
        })
        .transpose()?
        .unwrap_or(now);
    if observed < now - chrono::Duration::days(90) || observed > now + chrono::Duration::minutes(5)
    {
        return Err(invalid(
            "Observation timestamp must be within the last 90 days",
        ));
    }
    let url = input
        .url
        .as_deref()
        .or_else(|| headers.get("referer").and_then(|v| v.to_str().ok()))
        .map(normalized_url)
        .transpose()?;
    let user_agent = input
        .user_agent
        .as_deref()
        .or_else(|| headers.get("user-agent").and_then(|v| v.to_str().ok()));
    let device = match user_agent {
        Some(value) if value.to_ascii_lowercase().contains("mobile") => "mobile",
        Some(_) => "desktop",
        None => "unknown",
    };
    // Identical acknowledgements do not create extra samples. A newer value for
    // the same browser measurement identity updates that observation in place.
    let saved=sqlx::query("INSERT INTO performance_samples(measurement_id,metric,value,page_url,device,observed_at,navigation_type) VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(measurement_id,metric) DO UPDATE SET value=EXCLUDED.value,page_url=EXCLUDED.page_url,device=EXCLUDED.device,observed_at=EXCLUDED.observed_at,navigation_type=EXCLUDED.navigation_type WHERE EXCLUDED.observed_at>=performance_samples.observed_at AND (performance_samples.value,performance_samples.page_url,performance_samples.device,performance_samples.navigation_type) IS DISTINCT FROM (EXCLUDED.value,EXCLUDED.page_url,EXCLUDED.device,EXCLUDED.navigation_type)")
        .bind(&id).bind(&input.metric).bind(input.value).bind(url).bind(device).bind(observed).bind(input.navigation_type).execute(&state.db.pool).await.map_err(db)?;
    Ok(Json(
        json!({"success":true,"received":1,"persisted":saved.rows_affected(),"id":id}),
    ))
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Filters {
    url: Option<String>,
    device: Option<String>,
    days: Option<i64>,
}
async fn vitals(state: &AppState, filters: &Filters) -> Result<Value, ApiError> {
    let days = filters.days.unwrap_or(28);
    if !(1..=90).contains(&days)
        || filters
            .device
            .as_deref()
            .is_some_and(|v| !matches!(v, "all" | "mobile" | "desktop" | "unknown"))
    {
        return Err(invalid("Use a 1–90 day window and a valid device filter"));
    }
    let url = filters.url.as_deref().map(normalized_url).transpose()?;
    let device = filters.device.as_deref().filter(|v| *v != "all");
    let rows:Vec<(String,f64,i64)>=sqlx::query_as("SELECT metric,percentile_cont(0.75) WITHIN GROUP(ORDER BY value),COUNT(*) FROM performance_samples WHERE observed_at>=NOW()-make_interval(days=>$1) AND ($2::text IS NULL OR page_url=$2) AND ($3::text IS NULL OR device=$3) AND metric=ANY($4) GROUP BY metric")
        .bind(days as i32).bind(&url).bind(device).bind(VITALS.iter().map(|(key,_,_)|*key).collect::<Vec<_>>()).fetch_all(&state.db.pool).await.map_err(db)?;
    let mut result = json!({"url":url,"device":device.unwrap_or("all"),"window_days":days,"aggregation":"p75","source":"first_party_observations","sample_counts":{},"overall_score":null});
    for (key, _, _) in VITALS {
        let value = rows.iter().find(|(name, _, _)| name == key);
        result[key.to_ascii_lowercase()] = value.map_or(Value::Null, |(_, value, _)| json!(value));
        result[format!("{}_rating", key.to_ascii_lowercase())] = value
            .and_then(|(_, value, _)| rating(key, *value))
            .map_or(Value::Null, |rating| json!(rating));
        result["sample_counts"][*key] = json!(value.map_or(0, |(_, _, count)| *count));
    }
    Ok(result)
}
static HOST: LazyLock<Mutex<Option<(Instant, Value)>>> = LazyLock::new(|| Mutex::new(None));
pub async fn refresh_host_measurements() -> Result<(), ApiError> {
    // OS disk probes can be slow. Never hold the snapshot lock while probing.
    let value=tokio::task::spawn_blocking(|| {
        let mut system=sysinfo::System::new();system.refresh_cpu_usage();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);system.refresh_cpu_usage();system.refresh_memory();
        let disks=sysinfo::Disks::new_with_refreshed_list();
        let disk=disks.list().iter().find(|disk|disk.mount_point()==std::path::Path::new("/"));
        let memory=(system.total_memory()>0).then(||system.used_memory() as f64/system.total_memory() as f64*100.0);
        let disk=disk.filter(|disk|disk.total_space()>0).map(|disk|(disk.total_space().saturating_sub(disk.available_space())) as f64/disk.total_space() as f64*100.0);
        json!({"status":"available","cpu_usage":(!system.cpus().is_empty()).then(||system.global_cpu_usage()),"memory_usage":memory,"disk_usage":disk,"uptime":sysinfo::IS_SUPPORTED_SYSTEM.then(||format!("{}s",sysinfo::System::uptime())),"scope":"api_host","measured_at":Utc::now()})
    }).await.map_err(|_|ApiError::internal_error("Unable to collect host measurements"))?;
    *HOST
        .lock()
        .map_err(|_| ApiError::internal_error("Host measurement lock failed"))? =
        Some((Instant::now(), value));
    Ok(())
}
fn host() -> Result<Value, ApiError> {
    let cache = HOST
        .lock()
        .map_err(|_| ApiError::internal_error("Host measurement lock failed"))?;
    if let Some((at, value)) = &*cache {
        let mut value = value.clone();
        value["age_seconds"] = json!(at.elapsed().as_secs());
        if at.elapsed() > Duration::from_secs(60) {
            value["status"] = json!("stale");
        }
        return Ok(value);
    }
    Ok(
        json!({"status":"collecting","cpu_usage":null,"memory_usage":null,"disk_usage":null,"uptime":null,"scope":"api_host","measured_at":null,"age_seconds":null}),
    )
}
pub async fn run_host_sampler() {
    loop {
        if let Err(error) = refresh_host_measurements().await {
            tracing::error!(?error, "Host measurements unavailable; retrying");
            if let Ok(mut cache) = HOST.lock() {
                if let Some((_, value)) = &mut *cache {
                    value["status"] = json!("unavailable");
                } else {
                    *cache = Some((
                        Instant::now(),
                        json!({"status":"unavailable","cpu_usage":null,"memory_usage":null,"disk_usage":null,"uptime":null,"scope":"api_host","measured_at":null}),
                    ));
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

async fn database(state: &AppState) -> Result<Value, ApiError> {
    let available: bool =
        sqlx::query_scalar("SELECT to_regclass('pg_stat_statements') IS NOT NULL")
            .fetch_one(&state.db.pool)
            .await
            .map_err(db)?;
    let mut result = json!({"connection_pool":state.db.pool.size(),"query_count":null,"slow_queries":null,"avg_query_time":null,"query_statistics_status":"unavailable","query_statistics_requirement":"Enable PostgreSQL pg_stat_statements with shared_preload_libraries to collect statement statistics","slow_query_threshold_ms":100});
    if available {
        match sqlx::query_scalar::<_,Value>("SELECT jsonb_build_object('query_count',COALESCE(SUM(calls),0),'slow_queries',COALESCE(SUM(calls) FILTER(WHERE mean_exec_time>=100),0),'avg_query_time',CASE WHEN SUM(calls)>0 THEN SUM(total_exec_time)/SUM(calls) ELSE NULL END) FROM pg_stat_statements WHERE dbid=(SELECT oid FROM pg_database WHERE datname=current_database())").fetch_one(&state.db.pool).await{
            Ok(stats)=>{for key in ["query_count","slow_queries","avg_query_time"]{result[key]=stats[key].clone();}result["query_statistics_status"]=json!("available");result["query_statistics_requirement"]=Value::Null;},
            Err(error) if error.as_database_error().and_then(|e|e.code()).as_deref()==Some("55000")=>{},
            Err(error)=>return Err(db(error)),
        }
    }
    Ok(result)
}
async fn dashboard(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let server = host()?;
    let (database, vitals) = tokio::try_join!(database(&state), vitals(&state, &filters))?;
    let metrics = state.cache.metrics_snapshot();
    let hit_rate = (metrics.hits + metrics.misses > 0).then_some(metrics.hit_rate);
    let (keys, bytes) = state.cache.local_cache_stats().await;
    let cache = json!({"hit_rate":hit_rate,"keys_count":keys,"memory_used":format!("{} bytes",bytes),"evictions":metrics.evictions,"scope":"api_process_l1","redis_configured":state.cache.has_redis(),"metrics":metrics});
    Ok(Json(
        json!({"data":{"server":server,"database":database,"cache":cache,"core_web_vitals":vitals}}),
    ))
}
async fn get_vitals(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({"data":vitals(&state,&filters).await?})))
}
async fn recommendations(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let data = vitals(&state, &filters).await?;
    let mut items = Vec::new();
    for (key, _, _) in VITALS {
        let lower = key.to_ascii_lowercase();
        let state = data[format!("{lower}_rating")].as_str();
        if matches!(state, Some("poor" | "needs_improvement")) {
            let implementation = match *key {
                "LCP" => {
                    "Inspect the largest visible element, image dimensions and resource loading waterfall."
                }
                "CLS" => {
                    "Reserve layout space for images, embeds and asynchronously inserted content."
                }
                "INP" | "FID" => {
                    "Profile long tasks and reduce synchronous work in interaction handlers."
                }
                "TTFB" => "Inspect origin response timing, database work and cache misses.",
                _ => "Inspect render-blocking styles and the initial document loading waterfall.",
            };
            items.push(json!({"category":"Observed web vitals","priority":if state==Some("poor"){"high"}else{"medium"},"title":format!("Investigate {key}"),"description":format!("The recorded {}-day p75 observation is {}{} across {} samples.",data["window_days"],data[&lower],if *key=="CLS"{""}else{" ms"},data["sample_counts"][*key]),"impact":"Improve the measured user experience; validate with new observations.","implementation":implementation}));
        }
    }
    Ok(Json(
        json!({"data":{"recommendations":items,"cdn":null,"core_web_vitals":data}}),
    ))
}
async fn warm(State(state): State<AppState>, admin: AdminUser) -> Result<Json<Value>, ApiError> {
    let rooms: Vec<String> = sqlx::query_scalar(
        "SELECT slug FROM trading_rooms WHERE is_active=true ORDER BY id LIMIT 101",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(db)?;
    let plans: Vec<String> = sqlx::query_scalar(
        "SELECT slug FROM membership_plans WHERE is_active=true ORDER BY id LIMIT 501",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(db)?;
    if rooms.len() > 100 || plans.len() > 500 {
        return Err(invalid(
            "Warming supports at most 100 active rooms and 500 active plans per request",
        ));
    }
    let mut errors = Vec::new();
    let mut views = 0;
    let mut routes = 0;
    for room in &rooms {
        match super::room_content::stats::get_room_stats(
            State(state.clone()),
            admin.0.clone(),
            Path(room.clone()),
        )
        .await
        {
            Ok(_) => views += 1,
            Err(_) => errors.push(format!("Unable to warm {room} statistics")),
        };
        let query = super::room_content::TradeListQuery {
            page: Some(1),
            per_page: Some(20),
            status: None,
            ticker: None,
        };
        match super::room_content::trades::list_trades(
            State(state.clone()),
            admin.0.clone(),
            Path(room.clone()),
            Query(query),
        )
        .await
        {
            Ok(_) => views += 1,
            Err(_) => errors.push(format!("Unable to warm {room} trades")),
        };
    }
    match super::subscriptions::warm_list_plans(State(state.clone())).await {
        Ok(_) => routes += 1,
        Err(_) => errors.push("Unable to warm membership plan listing".into()),
    };
    for plan in &plans {
        match super::subscriptions::warm_get_plan(State(state.clone()), Path(plan.clone())).await {
            Ok(_) => routes += 1,
            Err(_) => errors.push(format!("Unable to warm membership plan {plan}")),
        };
    }
    Ok(Json(
        json!({"data":{"success":errors.is_empty(),"views":views,"routes":routes,"configs":0,"rooms":rooms.len(),"plans":plans.len(),"total":views+routes,"errors":errors}}),
    ))
}
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/web-vitals", post(ingest))
        .route("/performance", post(ingest))
        .layer(DefaultBodyLimit::max(16_384))
}
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/core-web-vitals", get(get_vitals))
        .route("/recommendations", get(recommendations))
        .route("/warm-caches", post(warm))
}

//! Explicit publication approval and fresh, factual news/video sitemap sources.
use crate::{
    AppState,
    middleware::admin::AdminUser,
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, Postgres, Transaction};

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Sitemap backing query failed");
    errors::service_unavailable("Sitemap content is unavailable")
}
fn invalid(message: &str) -> ApiError {
    errors::validation_error(message)
}
fn kind(value: &str) -> Result<(), ApiError> {
    if matches!(value, "news" | "video") {
        Ok(())
    } else {
        Err(errors::not_found("Sitemap"))
    }
}
#[derive(Deserialize)]
struct Author {
    slug: String,
    name: String,
}
fn profiles() -> Result<Value, ApiError> {
    let authors: Vec<Author> = serde_json::from_str(include_str!(
        "../../../frontend/src/lib/authors/authors.json"
    ))
    .map_err(|_| errors::service_unavailable("Author profile registry is unavailable"))?;
    let mut names = serde_json::Map::new();
    let mut slugs = serde_json::Map::new();
    for author in authors {
        names.insert(author.name.clone(), json!(author.slug));
        slugs.insert(author.slug, json!({"name":author.name}));
    }
    Ok(json!({"names":names,"slugs":slugs}))
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NewsSettings {
    enabled: bool,
    publication_name: String,
    language: String,
    max_age_hours: i32,
    exclude_categories: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct VideoSettings {
    enabled: bool,
    include_thumbnails: bool,
    include_uploader: bool,
    detect_from_content: bool,
    platforms: Vec<String>,
}
fn validate_config(kind: &str, value: Value) -> Result<Value, ApiError> {
    if kind == "news" {
        let mut config: NewsSettings = serde_json::from_value(value)
            .map_err(|_| invalid("Provide valid news sitemap settings"))?;
        config.publication_name = config.publication_name.trim().into();
        if !(1..=200).contains(&config.publication_name.chars().count())
            || config.publication_name.chars().any(char::is_control)
            || !(1..=48).contains(&config.max_age_hours)
            || !regex::Regex::new(r"^(?:[a-z]{2,3}|zh-(?:cn|tw))$")
                .map_err(|_| invalid("Language validation is unavailable"))?
                .is_match(&config.language)
            || config.exclude_categories.len() > 100
        {
            return Err(invalid(
                "Use a publication name, an ISO language code and an age of 1–48 hours",
            ));
        }
        config.exclude_categories.sort();
        config.exclude_categories.dedup();
        if config.exclude_categories.iter().any(|slug| {
            slug.is_empty()
                || slug.len() > 100
                || !slug
                    .bytes()
                    .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == b'-')
        }) {
            return Err(invalid("Choose actual category slugs"));
        }
        serde_json::to_value(config).map_err(|_| invalid("News settings could not be encoded"))
    } else {
        let mut config: VideoSettings = serde_json::from_value(value)
            .map_err(|_| invalid("Provide valid video sitemap settings"))?;
        if config.platforms.iter().any(|platform| {
            ![
                "self-hosted",
                "bunny",
                "youtube",
                "vimeo",
                "dailymotion",
                "ted",
                "wistia",
            ]
            .contains(&platform.as_str())
        }) {
            return Err(invalid("Choose supported video platforms"));
        }
        config.platforms.sort();
        config.platforms.dedup();
        if !config.include_thumbnails {
            return Err(invalid("Video sitemaps require actual video thumbnails"));
        }
        serde_json::to_value(config).map_err(|_| invalid("Video settings could not be encoded"))
    }
}
async fn settings(
    connection: &mut PgConnection,
    kind: &str,
    lock: bool,
) -> Result<(Value, i64), ApiError> {
    let sql = if lock {
        "SELECT config,version FROM seo_sitemap_settings WHERE kind=$1 FOR UPDATE"
    } else {
        "SELECT config,version FROM seo_sitemap_settings WHERE kind=$1"
    };
    let (config, version): (Value, i64) = sqlx::query_as(sqlx::AssertSqlSafe(sql))
        .bind(kind)
        .fetch_one(connection)
        .await
        .map_err(db)?;
    Ok((validate_config(kind, config)?, version))
}
const NEWS_SOURCE: &str = r#"SELECT jsonb_build_object('id',p.id::text,'title',p.title,'path','/blog/'||p.slug,'slug',p.slug,'published_at',to_char(p.published_at,'YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'author_name',u.name,'author_slug',$1::jsonb->'names'->>u.name,'canonical',p.canonical_url) AS item,
CASE WHEN p.status<>'published' THEN 'Not published'
WHEN NOT p.indexable THEN 'Post is marked noindex'
WHEN p.published_at IS NULL THEN 'Original publication date is unavailable'
WHEN p.published_at>NOW() AT TIME ZONE 'UTC' THEN 'Publication date is in the future'
WHEN p.published_at<(NOW() AT TIME ZONE 'UTC')-make_interval(hours=>(($2::jsonb->>'max_age_hours')::int)) THEN 'Outside the configured publication window'
WHEN NOT ($1::jsonb->'names' ? COALESCE(u.name,'')) THEN 'Actual author has no matching public profile'
WHEN p.slug !~ '^[a-z0-9]+(-[a-z0-9]+)*$' THEN 'Slug is not canonical'
WHEN NULLIF(btrim(p.title),'') IS NULL THEN 'Title is unavailable'
WHEN NULLIF(btrim(p.excerpt),'') IS NULL OR char_length(p.excerpt)<50 THEN 'Provide an actual article description'
WHEN NULLIF(btrim(p.featured_image),'') IS NULL THEN 'Article image is unavailable'
WHEN p.canonical_url IS NOT NULL AND btrim(p.canonical_url)<>'' AND p.canonical_url<>('/blog/'||p.slug) AND p.canonical_url<>($3::text||'/blog/'||p.slug) THEN 'Canonical points to a different URL'
WHEN EXISTS(SELECT 1 FROM post_categories pc JOIN categories c ON c.id=pc.category_id WHERE pc.post_id=p.id AND $2::jsonb->'exclude_categories' ? c.slug) THEN 'Category is excluded'
ELSE NULL END AS reason FROM posts p LEFT JOIN users u ON u.id=p.author_id"#;
const VIDEO_SNAPSHOT: &str = r#"jsonb_build_object('title',v.title,'slug',v.slug,'description',v.description,'video_url',v.video_url,'video_platform',v.video_platform,'video_id',v.video_id,'bunny_video_guid',v.bunny_video_guid,'bunny_library_id',v.bunny_library_id,'thumbnail_url',v.thumbnail_url,'duration',v.duration,'published_at',to_char(v.published_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'trader_id',v.trader_id,'trader_name',t.name)"#;
fn video_source() -> String {
    format!(
        r#"SELECT jsonb_build_object('id',v.id::text,'title',v.title,'path','/learning-center/'||v.slug,'slug',v.slug,'description',v.description,'video_url',v.video_url,'video_platform',v.video_platform,'video_id',v.video_id,'bunny_video_guid',v.bunny_video_guid,'bunny_library_id',v.bunny_library_id,'thumbnail_url',v.thumbnail_url,'duration',v.duration,'published_at',to_char(v.published_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'author_name',t.name,'trader_id',v.trader_id::text,'author_slug',p.author_slug,'mapping_id',p.id::text,'content_type',v.content_type,'is_published',v.is_published,'room_assignments',(SELECT COUNT(*) FROM video_room_assignments a WHERE a.video_id=v.id),'source',{VIDEO_SNAPSHOT}) AS item,
CASE WHEN v.deleted_at IS NOT NULL OR v.is_published IS DISTINCT FROM TRUE THEN 'Video is not published'
WHEN v.content_type<>'learning_center' THEN 'Only public learning-center videos can be approved'
WHEN EXISTS(SELECT 1 FROM video_room_assignments a WHERE a.video_id=v.id) THEN 'Room-assigned videos are private'
WHEN v.published_at IS NULL OR v.published_at>NOW() THEN 'Provide the actual nonfuture publication date'
WHEN v.duration IS NULL OR v.duration NOT BETWEEN 1 AND 28800 THEN 'Provide the actual duration between 1 and 28800 seconds'
WHEN char_length(COALESCE(v.title,'')) NOT BETWEEN 1 AND 100 OR char_length(COALESCE(v.description,'')) NOT BETWEEN 50 AND 2048 THEN 'Provide the actual title and complete video description'
WHEN $2::jsonb ? 'platforms' AND NOT ($2::jsonb->'platforms' ? v.video_platform) THEN 'Video platform is excluded by saved sitemap settings'
WHEN p.id IS NULL THEN 'Public watch page has not been approved'
WHEN NOT($1::jsonb->'slugs' ? p.author_slug) OR ($1::jsonb->'slugs'->p.author_slug->>'name') IS DISTINCT FROM t.name THEN 'Actual video author does not match the selected profile'
WHEN p.source_snapshot IS DISTINCT FROM {VIDEO_SNAPSHOT} THEN 'Source metadata changed; review this publication again'
ELSE NULL END AS reason FROM unified_videos v LEFT JOIN traders t ON t.id=v.trader_id LEFT JOIN seo_video_publications p ON p.video_id=v.id WHERE v.deleted_at IS NULL AND ($2::jsonb IS NOT NULL) AND ($3::text IS NOT NULL) AND (COALESCE(($2->>'detect_from_content')::boolean,true) OR p.id IS NOT NULL)"#
    )
}
/// Reuse the exact approved public gate in the legacy catalog; no parallel eligibility rules.
pub(crate) fn approved_video_source() -> Result<(String, Value), ApiError> {
    Ok((video_source(), profiles()?))
}
fn source_sql(kind: &str) -> String {
    if kind == "news" {
        NEWS_SOURCE.into()
    } else {
        video_source()
    }
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Page {
    page: Option<i64>,
    per_page: Option<i64>,
}
fn pagination(query: &Page, public: bool) -> Result<(i64, i64), ApiError> {
    let p = query.page.unwrap_or(1);
    let n = query.per_page.unwrap_or(if public { 1000 } else { 20 });
    if p < 1 || !(1..=if public { 1000 } else { 100 }).contains(&n) {
        Err(invalid("Choose a positive page and a bounded page size"))
    } else {
        Ok((p, n))
    }
}
async fn page_on(
    connection: &mut PgConnection,
    kind: &str,
    config: &Value,
    origin: &str,
    page: i64,
    size: i64,
    eligible_only: bool,
) -> Result<Value, ApiError> {
    let filter = if eligible_only {
        " WHERE reason IS NULL"
    } else {
        ""
    };
    let base = source_sql(kind);
    let registry = profiles()?;
    let count_sql = format!("WITH sources AS ({base}) SELECT COUNT(*) FROM sources{filter}");
    let total: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(count_sql.as_str()))
        .bind(&registry)
        .bind(config)
        .bind(origin.trim_end_matches('/'))
        .fetch_one(&mut *connection)
        .await
        .map_err(db)?;
    let sql = format!(
        "WITH sources AS ({base}) SELECT jsonb_build_object('record',item,'eligible',reason IS NULL,'exclusion_reason',reason) FROM sources{filter} ORDER BY (item->>'id')::bigint DESC LIMIT $4 OFFSET $5"
    );
    let mut data: Vec<Value> = sqlx::query_scalar(sqlx::AssertSqlSafe(sql.as_str()))
        .bind(&registry)
        .bind(config)
        .bind(origin.trim_end_matches('/'))
        .bind(size)
        .bind(page.saturating_sub(1).saturating_mul(size))
        .fetch_all(connection)
        .await
        .map_err(db)?;
    if kind == "video" {
        for row in &mut data {
            row["record"]["source_hash"] = json!(source_hash(&row["record"]["source"])?);
        }
    }
    Ok(
        json!({"data":data,"meta":{"total":total,"current_page":page,"per_page":size,"last_page":((total+size-1)/size).max(1)}}),
    )
}
async fn audit(
    tx: &mut Transaction<'_, Postgres>,
    kind: &str,
    action: &str,
    details: Value,
    actor: i64,
) -> Result<(), ApiError> {
    sqlx::query("INSERT INTO seo_sitemap_audit(kind,action,details,actor_id) VALUES($1,$2,$3,$4)")
        .bind(kind)
        .bind(action)
        .bind(details)
        .bind(actor)
        .execute(&mut **tx)
        .await
        .map_err(db)?;
    Ok(())
}
async fn get_admin(
    State(state): State<AppState>,
    _: AdminUser,
    Path(kind_name): Path<String>,
    Query(query): Query<Page>,
) -> Result<Json<Value>, ApiError> {
    kind(&kind_name)?;
    let (page, size) = pagination(&query, false)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let (config, version) = settings(&mut tx, &kind_name, false).await?;
    let data = page_on(
        &mut tx,
        &kind_name,
        &config,
        &state.config.app_url,
        page,
        size,
        false,
    )
    .await?;
    let last:Option<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',id,'created_at',created_at,'entry_count',entry_count,'settings_version',settings_version,'source_hash',source_hash) FROM seo_sitemap_generations WHERE kind=$1 ORDER BY created_at DESC,id DESC LIMIT 1").bind(&kind_name).fetch_optional(&mut *tx).await.map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"kind":kind_name,"metrics_scope":"saved_sitemap_sources","settings":config,"settings_version":version,"preview":data,"last_generation":last,"author_profiles":profiles()?}),
    ))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SettingsInput {
    expected_version: i64,
    config: Value,
}
async fn update_settings(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(kind_name): Path<String>,
    Json(input): Json<SettingsInput>,
) -> Result<Json<Value>, ApiError> {
    kind(&kind_name)?;
    let config = validate_config(&kind_name, input.config)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let (_, version) = settings(&mut tx, &kind_name, true).await?;
    if input.expected_version != version {
        return Err(errors::conflict(
            "Sitemap settings changed; reload before saving",
        ));
    }
    if kind_name == "news" {
        let wanted: Vec<String> = serde_json::from_value(config["exclude_categories"].clone())
            .map_err(|_| invalid("Choose actual category slugs"))?;
        let actual: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE slug=ANY($1)")
            .bind(&wanted)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
        if actual != wanted.len() as i64 {
            return Err(invalid("An excluded category no longer exists"));
        }
    }
    sqlx::query("UPDATE seo_sitemap_settings SET config=$2,version=version+1,updated_by=$3,updated_at=NOW() WHERE kind=$1").bind(&kind_name).bind(&config).bind(admin.0.id).execute(&mut *tx).await.map_err(db)?;
    audit(
        &mut tx,
        &kind_name,
        "settings_saved",
        json!({"settings_version":version+1,"config":config}),
        admin.0.id,
    )
    .await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"kind":kind_name,"settings":config,"settings_version":version+1,"saved":true}),
    ))
}
async fn public_feed(
    State(state): State<AppState>,
    Path(kind_name): Path<String>,
    Query(query): Query<Page>,
) -> Result<Json<Value>, ApiError> {
    kind(&kind_name)?;
    let (page, size) = pagination(&query, true)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let (config, version) = settings(&mut tx, &kind_name, false).await?;
    let mut data = if config["enabled"] == true {
        page_on(
            &mut tx,
            &kind_name,
            &config,
            &state.config.app_url,
            page,
            size,
            true,
        )
        .await?
    } else {
        json!({"data":[],"meta":{"total":0,"current_page":page,"per_page":size,"last_page":1}})
    };
    if kind_name == "video" {
        decorate_public_videos(&mut data, state.config.environment == "development")?;
    }
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"kind":kind_name,"metrics_scope":"published_sitemap_sources","settings":config,"settings_version":version,"entries":data}),
    ))
}
async fn regenerate(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(kind_name): Path<String>,
) -> Result<Json<Value>, ApiError> {
    kind(&kind_name)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let (config, version) = settings(&mut tx, &kind_name, true).await?;
    let mut hash = Sha256::new();
    let mut total = 0_i64;
    let mut page = 1_i64;
    if config["enabled"] == true {
        loop {
            let batch = page_on(
                &mut tx,
                &kind_name,
                &config,
                &state.config.app_url,
                page,
                1000,
                true,
            )
            .await?;
            let rows = batch["data"]
                .as_array()
                .ok_or_else(|| errors::service_unavailable("Sitemap source snapshot is invalid"))?;
            for row in rows {
                hash.update(
                    serde_json::to_vec(row)
                        .map_err(|_| invalid("Sitemap source could not be serialized"))?,
                );
            }
            total += rows.len() as i64;
            if page >= batch["meta"]["last_page"].as_i64().unwrap_or(1) {
                break;
            }
            page += 1;
        }
    }
    let source_hash = hex::encode(hash.finalize());
    let generation:Value=sqlx::query_scalar("INSERT INTO seo_sitemap_generations(kind,settings_version,entry_count,source_hash,created_by) VALUES($1,$2,$3,$4,$5) RETURNING jsonb_build_object('id',id,'created_at',created_at,'entry_count',entry_count,'settings_version',settings_version,'source_hash',source_hash)").bind(&kind_name).bind(version).bind(total).bind(&source_hash).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(db)?;
    audit(
        &mut tx,
        &kind_name,
        "generated",
        generation.clone(),
        admin.0.id,
    )
    .await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"kind":kind_name,"generated":true,"generation":generation}),
    ))
}
fn source_hash(value: &Value) -> Result<String, ApiError> {
    Ok(hex::encode(Sha256::digest(
        serde_json::to_vec(value).map_err(|_| invalid("Source metadata could not be encoded"))?,
    )))
}
#[derive(Deserialize)]
struct MediaPolicy {
    file_hosts: Vec<String>,
    iframe_hosts: Vec<String>,
    development_origin: String,
}
fn media_policy() -> Result<MediaPolicy, ApiError> {
    serde_json::from_str(include_str!(
        "../../../frontend/src/lib/seo/public-media.json"
    ))
    .map_err(|_| errors::service_unavailable("Public media policy is unavailable"))
}
fn allowed_host(host: &str, patterns: &[String]) -> bool {
    patterns.iter().any(|pattern| {
        pattern.strip_prefix("*.").map_or_else(
            || host == pattern,
            |suffix| host.ends_with(&format!(".{suffix}")),
        )
    })
}
fn public_url(value: &str, development: bool) -> Result<reqwest::Url, ApiError> {
    // Check the supplied path before URL parsing can remove dot segments or
    // reinterpret backslashes. A reviewed source must identify the same media
    // before and after browser URL normalization.
    let (_, rest) = value
        .split_once("://")
        .ok_or_else(|| invalid("Provide an absolute public media URL"))?;
    let authority_end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let raw_path = rest[authority_end..].split(['?', '#']).next().unwrap_or("");
    if value
        .chars()
        .any(|ch| ch.is_whitespace() || ch.is_control())
        || value.contains('\\')
        || rest[..authority_end].contains('@')
        || raw_path.split('/').any(|segment| {
            let Ok(decoded) = urlencoding::decode(segment) else {
                return true;
            };
            decoded == "."
                || decoded == ".."
                || decoded.contains(['/', '\\', '%'])
                || decoded.chars().any(char::is_control)
                || decoded.to_ascii_lowercase().contains("bcdn_token=")
        })
        || value.as_bytes().iter().enumerate().any(|(index, ch)| {
            *ch == b'%'
                && !value
                    .as_bytes()
                    .get(index + 1..index + 3)
                    .is_some_and(|pair| pair.iter().all(u8::is_ascii_hexdigit))
        })
    {
        return Err(invalid(
            "Use an unambiguous public media URL without malformed or traversing paths",
        ));
    }
    let parsed =
        reqwest::Url::parse(value).map_err(|_| invalid("Provide a valid public media URL"))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| invalid("Media URL requires a host"))?;
    let local = host == "localhost" || host == "127.0.0.1" || host == "[::1]";
    let fixture =
        development && parsed.origin().ascii_serialization() == media_policy()?.development_origin;
    if !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.fragment().is_some()
        || !(parsed.scheme() == "https" || fixture)
        || (parsed.scheme() == "https" && parsed.port().is_some())
        || (local && !fixture)
        || parsed.query_pairs().any(|(key, _)| {
            let key = key.to_ascii_lowercase();
            matches!(
                key.as_str(),
                "token"
                    | "access_token"
                    | "auth"
                    | "authorization"
                    | "jwt"
                    | "expires"
                    | "expiry"
                    | "expire"
                    | "signature"
                    | "sig"
                    | "exp"
                    | "policy"
                    | "key-pair-id"
                    | "awsaccesskeyid"
            ) || key.starts_with("token_")
                || key.ends_with("_token")
                || key.starts_with("x-amz-")
                || key.starts_with("x-goog-")
        })
    {
        return Err(invalid(
            "Use a stable public HTTPS media URL on port 443 without credentials, fragments or expiring tokens",
        ));
    }
    Ok(parsed)
}
fn provider_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 200
        && value
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && value
            .as_bytes()
            .last()
            .is_some_and(u8::is_ascii_alphanumeric)
        && value
            .bytes()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'-')
}
fn numeric_identifier(value: &str) -> bool {
    value.len() <= 20
        && value
            .as_bytes()
            .first()
            .is_some_and(|ch| matches!(ch, b'1'..=b'9'))
        && value.bytes().all(|ch| ch.is_ascii_digit())
}
fn youtube_identifier(value: &str) -> bool {
    value.len() == 11
        && value
            .bytes()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'-')
}
fn single_query(url: &reqwest::Url, name: &str) -> Result<Option<String>, ApiError> {
    let mut values = url.query_pairs().filter(|(key, _)| key == name);
    let value = values.next().map(|(_, value)| value.into_owned());
    if values.next().is_some() {
        return Err(invalid(
            "Provider URL contains an ambiguous repeated video identifier",
        ));
    }
    Ok(value)
}
fn playback(value: &str, development: bool) -> Result<(String, &'static str), ApiError> {
    let url = public_url(value, development)?;
    let host = url.host_str().unwrap_or("");
    let path = url.path();
    if [".mp4", ".webm", ".ogv", ".mov"]
        .iter()
        .any(|ext| path.to_ascii_lowercase().ends_with(ext))
    {
        if !path.rsplit('/').next().is_some_and(|filename| {
            filename
                .rsplit_once('.')
                .is_some_and(|(name, _)| !name.is_empty())
        }) {
            return Err(invalid(
                "Provide a public video file with an actual filename",
            ));
        }
        let policy = media_policy()?;
        if !allowed_host(host, &policy.file_hosts)
            && !(development && url.origin().ascii_serialization() == policy.development_origin)
        {
            return Err(invalid(
                "Use a public video file on the configured Bunny/R2 media allowlist, or a supported YouTube, Vimeo, Bunny, Dailymotion, TED or Wistia player URL",
            ));
        }
        return Ok((url.to_string(), "video"));
    }
    let path = path.strip_prefix('/').unwrap_or(path);
    let parts: Vec<_> = path.strip_suffix('/').unwrap_or(path).split('/').collect();
    let player = match (host, parts.as_slice()) {
        ("youtube.com" | "www.youtube.com" | "m.youtube.com" | "music.youtube.com", ["watch"]) => {
            single_query(&url, "v")?.filter(|id| youtube_identifier(id))
                .map(|id| format!("https://www.youtube-nocookie.com/embed/{id}"))
        }
        ("youtube.com" | "www.youtube.com" | "m.youtube.com" | "music.youtube.com", ["embed" | "shorts" | "live" | "v", id])
        | ("www.youtube-nocookie.com" | "youtube-nocookie.com", ["embed", id])
        | ("youtu.be", [id]) if youtube_identifier(id) && single_query(&url, "v")?.is_none() => {
            Some(format!("https://www.youtube-nocookie.com/embed/{id}"))
        }
        ("vimeo.com" | "www.vimeo.com", [id])
        | ("player.vimeo.com", ["video", id]) if numeric_identifier(id) && single_query(&url, "h")?.is_none() => {
            Some(format!("https://player.vimeo.com/video/{id}"))
        }
        ("vimeo.com" | "www.vimeo.com", ["channels", collection, id])
        | ("vimeo.com" | "www.vimeo.com", ["groups", collection, "videos", id])
            if provider_identifier(collection) && numeric_identifier(id) && single_query(&url, "h")?.is_none() => {
            Some(format!("https://player.vimeo.com/video/{id}"))
        }
        ("vimeo.com" | "www.vimeo.com", ["album" | "showcase", collection, "video", id])
            if numeric_identifier(collection) && numeric_identifier(id) && single_query(&url, "h")?.is_none() => {
            Some(format!("https://player.vimeo.com/video/{id}"))
        }
        ("iframe.mediadelivery.net" | "player.mediadelivery.net", ["embed", library, id])
            if numeric_identifier(library) && id.len() == 36 && uuid::Uuid::parse_str(id).is_ok_and(|id| !id.is_nil()) => {
            Some(url.to_string())
        }
        ("geo.dailymotion.com", ["player", player]) => {
            let player = player.strip_suffix(".html").filter(|id| provider_identifier(id));
            let video = single_query(&url, "video")?.filter(|id| provider_identifier(id));
            match (player, video) {
                (Some(player), Some(video)) if single_query(&url, "playlist")?.is_none() => {
                    Some(format!("https://geo.dailymotion.com/player/{player}.html?video={video}"))
                }
                _ => None,
            }
        }
        ("www.dailymotion.com" | "dailymotion.com" | "dai.ly", _) => {
            return Err(invalid("Dailymotion requires the actual full geo.dailymotion.com/player/PLAYER_ID.html?video=VIDEO_ID embed URL from Dailymotion Studio"));
        }
        ("www.ted.com" | "ted.com" | "embed.ted.com", ["talks", slug]) if provider_identifier(slug) => {
            Some(format!("https://embed.ted.com/talks/{slug}"))
        }
        ("www.ted.com" | "ted.com" | "embed.ted.com", ["talks", "lang", language, slug])
            if (2..=10).contains(&language.len()) && language.bytes().all(|ch| ch.is_ascii_alphabetic() || ch == b'-') && provider_identifier(slug) => {
            Some(format!("https://embed.ted.com/talks/lang/{language}/{slug}"))
        }
        ("fast.wistia.net" | "fast.wistia.com", ["embed", "iframe", id]) if id.len() == 10 && id.bytes().all(|ch| ch.is_ascii_alphanumeric()) => {
            Some(format!("https://{host}/embed/iframe/{id}"))
        }
        (_, ["medias" | "m", id] | ["medias", id, "manage"])
            if host.strip_suffix(".wistia.com").is_some_and(|account| !account.contains('.') && provider_identifier(account))
                && id.len() == 10 && id.bytes().all(|ch| ch.is_ascii_alphanumeric()) => {
            Some(format!("https://fast.wistia.net/embed/iframe/{id}"))
        }
        _ => None,
    }.ok_or_else(|| invalid("Use an actual supported public video file or provider player URL; private hashes and share links cannot be approved"))?;
    let normalized = public_url(&player, development)?;
    if !allowed_host(
        normalized.host_str().unwrap_or(""),
        &media_policy()?.iframe_hosts,
    ) {
        return Err(invalid(
            "The normalized video player is outside the configured CSP allowlist",
        ));
    }
    Ok((player, "iframe"))
}
/// Public catalogs and watch pages share one reviewed playback identity. Legacy
/// stored provider IDs remain supported privately, but cannot override a
/// different explicitly approved public source.
pub(crate) fn approved_playback(
    item: &Value,
    development: bool,
) -> Result<(String, &'static str), ApiError> {
    let actual = playback(item["video_url"].as_str().unwrap_or(""), development)?;
    let platform = item["video_platform"].as_str().unwrap_or("");
    let expected = match platform {
        "youtube" | "vimeo" if !item["video_id"].is_null() => {
            let id = item["video_id"].as_str().unwrap_or("");
            let source = if platform == "youtube" {
                format!("https://www.youtube.com/embed/{id}")
            } else {
                format!("https://player.vimeo.com/video/{id}")
            };
            Some(playback(&source, development)?)
        }
        "bunny" if !item["bunny_video_guid"].is_null() || !item["bunny_library_id"].is_null() => {
            let guid = item["bunny_video_guid"].as_str().ok_or_else(|| {
                invalid(
                    "Stored Bunny identifiers are incomplete; repair them before public approval",
                )
            })?;
            let library = item["bunny_library_id"].as_i64().filter(|id| *id > 0).ok_or_else(|| invalid("Stored Bunny identifiers are incomplete; repair them before public approval"))?;
            Some(playback(
                &format!("https://iframe.mediadelivery.net/embed/{library}/{guid}"),
                development,
            )?)
        }
        _ => None,
    };
    if let Some(expected) = expected {
        let same = if platform == "bunny" {
            let actual_url = reqwest::Url::parse(&actual.0)
                .map_err(|_| invalid("Public playback is invalid"))?;
            let expected_url = reqwest::Url::parse(&expected.0)
                .map_err(|_| invalid("Stored playback is invalid"))?;
            actual.1 == "iframe"
                && matches!(
                    actual_url.host_str(),
                    Some("iframe.mediadelivery.net" | "player.mediadelivery.net")
                )
                && actual_url
                    .path()
                    .trim_end_matches('/')
                    .eq_ignore_ascii_case(expected_url.path())
        } else {
            actual == expected
        };
        if !same {
            return Err(invalid(
                "The public video URL and stored provider identifiers refer to different videos; repair the source before approval",
            ));
        }
    }
    Ok(actual)
}
fn validate_video(item: &Value, author_slug: &str, development: bool) -> Result<(), ApiError> {
    if item["content_type"] != "learning_center"
        || item["is_published"] != true
        || item["room_assignments"].as_i64() != Some(0)
    {
        return Err(invalid(
            "Only published learning-center videos without room assignments can have a public watch page",
        ));
    }
    let registry = profiles()?;
    let name = registry["slugs"][author_slug]["name"]
        .as_str()
        .ok_or_else(|| invalid("Choose an existing public author profile"))?;
    if item["author_name"].as_str() != Some(name) {
        return Err(invalid(
            "The saved video trader must match the actual selected author profile",
        ));
    }
    let title = item["title"].as_str().unwrap_or("");
    let description = item["description"].as_str().unwrap_or("");
    if !(1..=100).contains(&title.chars().count())
        || !(50..=2048).contains(&description.chars().count())
        || !regex::Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$")
            .map_err(|_| invalid("Slug validation is unavailable"))?
            .is_match(item["slug"].as_str().unwrap_or(""))
    {
        return Err(invalid(
            "Provide a canonical slug, actual title (1–100 characters) and description (50–2048 characters)",
        ));
    }
    let date = chrono::DateTime::parse_from_rfc3339(item["published_at"].as_str().unwrap_or(""))
        .map_err(|_| invalid("Provide the actual video publication date"))?;
    if date > chrono::Utc::now() {
        return Err(invalid("Publication date cannot be in the future"));
    }
    if !item["duration"]
        .as_i64()
        .is_some_and(|seconds| (1..=28800).contains(&seconds))
    {
        return Err(invalid(
            "Provide the actual duration between 1 and 28800 seconds",
        ));
    }
    public_url(item["thumbnail_url"].as_str().unwrap_or(""), development)?;
    approved_playback(item, development)?;
    Ok(())
}
async fn video_on(connection: &mut PgConnection, id: i64, origin: &str) -> Result<Value, ApiError> {
    let sql = format!(
        "WITH sources AS ({}) SELECT item FROM sources WHERE item->>'id'=$4",
        video_source()
    );
    sqlx::query_scalar(sqlx::AssertSqlSafe(sql.as_str()))
        .bind(profiles()?)
        .bind(json!({}))
        .bind(origin)
        .bind(id.to_string())
        .fetch_optional(connection)
        .await
        .map_err(db)?
        .ok_or_else(|| errors::not_found("Video"))
}
#[derive(Deserialize)]
#[serde(untagged)]
enum RecordId {
    Number(i64),
    Text(String),
}
impl RecordId {
    fn value(&self) -> Result<i64, ApiError> {
        let value = match self {
            Self::Number(value) => Some(*value),
            Self::Text(value) => value.parse().ok(),
        };
        value
            .filter(|value| *value > 0)
            .ok_or_else(|| invalid("Choose a positive video identity"))
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Approval {
    video_id: RecordId,
    author_slug: String,
    expected_source_hash: String,
}
async fn approve(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<Approval>,
) -> Result<Json<Value>, ApiError> {
    let video_id = input.video_id.value()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SELECT id FROM unified_videos WHERE id=$1 FOR UPDATE")
        .bind(video_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(|| errors::not_found("Video"))?;
    let item = video_on(&mut tx, video_id, &state.config.app_url).await?;
    if source_hash(&item["source"])? != input.expected_source_hash {
        return Err(errors::conflict(
            "Video metadata changed; reload before approving its public page",
        ));
    }
    validate_video(
        &item,
        &input.author_slug,
        state.config.environment == "development",
    )?;
    let id:i64=sqlx::query_scalar("INSERT INTO seo_video_publications(video_id,author_slug,source_snapshot,created_by) VALUES($1,$2,$3,$4) ON CONFLICT(video_id) DO UPDATE SET author_slug=EXCLUDED.author_slug,source_snapshot=EXCLUDED.source_snapshot,updated_at=NOW() RETURNING id").bind(video_id).bind(&input.author_slug).bind(&item["source"]).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(db)?;
    audit(&mut tx,"video","public_watch_page_approved",json!({"video_id":video_id,"publication_id":id,"source_hash":input.expected_source_hash,"author_slug":input.author_slug}),admin.0.id).await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"approved":true,"video_id":video_id.to_string(),"publication_id":id.to_string(),"path":item["path"]}),
    ))
}
async fn remove_approval(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let deleted = sqlx::query("DELETE FROM seo_video_publications WHERE video_id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?
        .rows_affected();
    if deleted != 1 {
        return Err(errors::not_found("Video publication"));
    }
    audit(
        &mut tx,
        "video",
        "public_watch_page_removed",
        json!({"video_id":id}),
        admin.0.id,
    )
    .await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(json!({"removed":true,"video_id":id.to_string()})))
}
async fn watch(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, ApiError> {
    let source = video_source();
    let sql = format!(
        "WITH sources AS ({source}) SELECT item FROM sources WHERE reason IS NULL AND item->>'slug'=$4"
    );
    let mut item: Value = sqlx::query_scalar(sqlx::AssertSqlSafe(sql.as_str()))
        .bind(profiles()?)
        .bind(json!({}))
        .bind(&state.config.app_url)
        .bind(slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(db)?
        .ok_or_else(|| errors::not_found("Public video"))?;
    validate_video(
        &item,
        item["author_slug"].as_str().unwrap_or(""),
        state.config.environment == "development",
    )?;
    let (url, kind) = approved_playback(&item, state.config.environment == "development")?;
    item["playback_url"] = json!(url);
    item["playback_kind"] = json!(kind);
    item["source_hash"] = json!(source_hash(&item["source"])?);
    Ok(Json(item))
}
fn decorate_public_videos(page: &mut Value, development: bool) -> Result<(), ApiError> {
    let entries = page["data"]
        .as_array_mut()
        .ok_or_else(|| errors::service_unavailable("Video publication data is unavailable"))?;
    for entry in entries {
        let item = &mut entry["record"];
        validate_video(
            item,
            item["author_slug"].as_str().unwrap_or(""),
            development,
        )?;
        let (url, kind) = approved_playback(item, development)?;
        item["playback_url"] = json!(url);
        item["playback_kind"] = json!(kind);
    }
    Ok(())
}
async fn public_videos(
    State(state): State<AppState>,
    Query(query): Query<Page>,
) -> Result<Json<Value>, ApiError> {
    let (page, size) = pagination(&query, true)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let mut result = page_on(
        &mut tx,
        "video",
        &json!({}),
        &state.config.app_url,
        page,
        size,
        true,
    )
    .await?;
    decorate_public_videos(&mut result, state.config.environment == "development")?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"metrics_scope":"approved_public_videos","entries":result}),
    ))
}
async fn public_articles(
    State(state): State<AppState>,
    Query(query): Query<Page>,
) -> Result<Json<Value>, ApiError> {
    let (page, size) = pagination(&query, true)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let result = page_on(
        &mut tx,
        "news",
        &json!({"max_age_hours":8760000,"exclude_categories":[]}),
        &state.config.app_url,
        page,
        size,
        true,
    )
    .await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(
        json!({"metrics_scope":"indexable_public_articles","entries":result}),
    ))
}
async fn get_video_source(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut connection = state.db.pool.acquire().await.map_err(db)?;
    let mut item = video_on(&mut connection, id, &state.config.app_url).await?;
    item["source_hash"] = json!(source_hash(&item["source"])?);
    Ok(Json(json!({"record":item})))
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/admin/seo/sitemaps/{kind}",
            get(get_admin).put(update_settings),
        )
        .route("/admin/seo/sitemaps/{kind}/generate", post(regenerate))
        .route("/admin/seo/video-publications", post(approve))
        .route(
            "/admin/seo/video-publications/{id}",
            get(get_video_source).delete(remove_approval),
        )
        .route("/seo/sitemaps/{kind}", get(public_feed))
        .route("/seo/public-articles", get(public_articles))
        .route("/seo/public-videos", get(public_videos))
        .route("/seo/public-videos/{slug}", get(watch))
}

#[cfg(test)]
mod tests {
    use super::{allowed_host, approved_playback, media_policy, playback, public_url};

    #[test]
    fn stored_provider_identifiers_cannot_override_a_different_approved_source() {
        let mut source = serde_json::json!({
            "video_url":"https://player.mediadelivery.net/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe?autoplay=false",
            "video_platform":"bunny",
            "bunny_video_guid":"eb1c4f77-0cda-46be-b47d-1118ad7c2ffe",
            "bunny_library_id":123,
        });
        assert_eq!(
            approved_playback(&source, false).unwrap().0,
            source["video_url"]
        );
        source["bunny_video_guid"] = serde_json::json!("fb1c4f77-0cda-46be-b47d-1118ad7c2ffe");
        assert!(approved_playback(&source, false).is_err());
        source["bunny_video_guid"] = serde_json::Value::Null;
        assert!(approved_playback(&source, false).is_err());
        for (platform, raw, id) in [
            ("youtube", "https://youtu.be/abcdefghijk", "abcdefghijk"),
            ("vimeo", "https://vimeo.com/123456789", "123456789"),
        ] {
            let mut source =
                serde_json::json!({"video_platform":platform,"video_url":raw,"video_id":id});
            assert!(approved_playback(&source, false).is_ok());
            source["video_id"] = serde_json::json!("98765432100");
            assert!(approved_playback(&source, false).is_err());
        }
    }

    #[test]
    fn public_media_normalizes_only_explicit_provider_video_forms() {
        let youtube = "https://www.youtube-nocookie.com/embed/_bcdefghij-";
        for source in [
            "https://www.youtube.com/watch?v=_bcdefghij-",
            "https://youtube.com/watch?feature=shared&v=_bcdefghij-",
            "https://m.youtube.com/watch?v=_bcdefghij-",
            "https://youtu.be/_bcdefghij-?si=actual-share-reference",
            "https://www.youtube.com/embed/_bcdefghij-",
            "https://www.youtube.com/shorts/_bcdefghij-",
            "https://www.youtube.com/live/_bcdefghij-",
            "https://www.youtube.com/v/_bcdefghij-",
            "https://www.youtube-nocookie.com/embed/_bcdefghij-/",
            "https://youtube-nocookie.com/embed/_bcdefghij-",
        ] {
            assert_eq!(
                playback(source, false).unwrap(),
                (youtube.into(), "iframe"),
                "{source}"
            );
        }
        for source in [
            "https://vimeo.com/123456789",
            "https://www.vimeo.com/123456789/",
            "https://player.vimeo.com/video/123456789?autoplay=0",
            "https://vimeo.com/channels/staffpicks/123456789",
            "https://vimeo.com/groups/actual_group/videos/123456789",
            "https://vimeo.com/album/987654/video/123456789",
            "https://vimeo.com/showcase/987654/video/123456789",
        ] {
            assert_eq!(
                playback(source, false).unwrap(),
                ("https://player.vimeo.com/video/123456789".into(), "iframe"),
                "{source}"
            );
        }
        for source in [
            "https://www.ted.com/talks/actual_saved_talk",
            "https://ted.com/talks/actual_saved_talk/",
            "https://embed.ted.com/talks/actual_saved_talk",
        ] {
            assert_eq!(
                playback(source, false).unwrap(),
                (
                    "https://embed.ted.com/talks/actual_saved_talk".into(),
                    "iframe"
                ),
                "{source}"
            );
        }
        assert_eq!(
            playback("https://www.ted.com/talks/lang/en/actual_saved_talk", false).unwrap(),
            (
                "https://embed.ted.com/talks/lang/en/actual_saved_talk".into(),
                "iframe"
            )
        );
        for source in [
            "https://actual-account.wistia.com/medias/e4a27b971d",
            "https://actual-account.wistia.com/medias/e4a27b971d/manage",
            "https://actual-account.wistia.com/m/e4a27b971d",
            "https://fast.wistia.net/embed/iframe/e4a27b971d",
        ] {
            assert_eq!(
                playback(source, false).unwrap(),
                (
                    "https://fast.wistia.net/embed/iframe/e4a27b971d".into(),
                    "iframe"
                ),
                "{source}"
            );
        }
        for source in [
            "https://iframe.mediadelivery.net/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe?autoplay=false",
            "https://player.mediadelivery.net/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe",
            "https://fast.wistia.com/embed/iframe/e4a27b971d",
        ] {
            assert_eq!(playback(source, false).unwrap(), (source.into(), "iframe"));
        }
        assert_eq!(
            playback(
                "https://geo.dailymotion.com/player/x8lr5.html?video=x84sh87&mute=true",
                false
            )
            .unwrap(),
            (
                "https://geo.dailymotion.com/player/x8lr5.html?video=x84sh87".into(),
                "iframe"
            )
        );
    }

    #[test]
    fn accepted_playback_is_on_the_exact_shared_csp_sources_and_default_ports() {
        let policy = media_policy().unwrap();
        for source in [
            "https://fixture.b-cdn.net/actual.mp4",
            "https://fixture.b-cdn.net:443/folder/actual%20video.MP4?download=0",
            "https://vz-example.mediadelivery.net/actual.webm",
            "https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev/actual.ogv",
            "https://www.youtube.com/watch?v=abcdefghijk",
            "https://vimeo.com/123456789",
            "https://player.mediadelivery.net/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe",
            "https://geo.dailymotion.com/player/x8lr5.html?video=x84sh87",
            "https://www.ted.com/talks/actual_saved_talk",
            "https://fast.wistia.net/embed/iframe/e4a27b971d",
            "https://fast.wistia.com/embed/iframe/e4a27b971d",
        ] {
            let (normalized, kind) = playback(source, false).unwrap();
            let url = reqwest::Url::parse(&normalized).unwrap();
            let hosts = if kind == "video" {
                &policy.file_hosts
            } else {
                &policy.iframe_hosts
            };
            assert!(
                allowed_host(url.host_str().unwrap(), hosts),
                "Normalized playback must match CSP: {source} -> {normalized}"
            );
            assert_eq!(url.scheme(), "https");
            assert_eq!(url.port_or_known_default(), Some(443));
        }
    }

    #[test]
    fn empty_ambiguous_private_or_malformed_provider_paths_cannot_be_approved() {
        for source in [
            "https://www.youtube.com/",
            "https://www.youtube.com/watch",
            "https://www.youtube.com/watch?v=",
            "https://www.youtube.com/watch?v=abcdefghijk&v=lmnopqrstuv",
            "https://www.youtube.com/not-a-player/abcdefghijk",
            "https://www.youtube.com/embed/abcdefghijk/extra",
            "https://www.youtube.com//embed/abcdefghijk",
            "https://www.youtube.com/embed/abcdefghijk?v=lmnopqrstuv",
            "https://www.youtube.com/embed/abcdefghijk?x=1&v=lmnopqrstuv&v=abcdefghijk",
            "https://www.youtube.com/embed/abcdefghij!",
            "https://youtu.be/",
            "https://youtu.be/watch/abcdefghijk",
            "https://www.youtube-nocookie.com/watch?v=abcdefghijk",
            "https://www.youtube.com/embed/%61bcdefghijk",
            "https://vimeo.com/",
            "https://vimeo.com/0",
            "https://vimeo.com/manage/123456789",
            "https://vimeo.com/channels//123456789",
            "https://vimeo.com/123456789/actual-private-hash",
            "https://player.vimeo.com/video/123456789?h=actual-private-hash",
            "https://player.vimeo.com/123456789",
            "https://player.vimeo.com/video/123456789/extra",
            "https://iframe.mediadelivery.net/embed/",
            "https://iframe.mediadelivery.net/embed/123/",
            "https://iframe.mediadelivery.net/embed/0/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe",
            "https://iframe.mediadelivery.net/embed/123/not-a-video-guid",
            "https://iframe.mediadelivery.net/embed/123/00000000-0000-0000-0000-000000000000",
            "https://iframe.mediadelivery.net/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe/extra",
            "https://other.mediadelivery.net/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe",
            "https://www.dailymotion.com/embed/video/",
            "https://www.dailymotion.com/embed/video/x84sh87",
            "https://www.dailymotion.com/video/x84sh87",
            "https://dai.ly/x84sh87",
            "https://geo.dailymotion.com/player.html?video=x84sh87",
            "https://geo.dailymotion.com/player/.html?video=x84sh87",
            "https://geo.dailymotion.com/player/x8lr5.html?video=",
            "https://geo.dailymotion.com/player/x8lr5.html?video=x84sh87&video=other",
            "https://geo.dailymotion.com/player/x8lr5.html?video=x84sh87&playlist=x5zhzj",
            "https://geo.dailymotion.com/player/x8lr5.js?video=x84sh87",
            "https://geo.dailymotion.com/player/x8lr5.html/extra?video=x84sh87",
            "https://embed.ted.com/talks/",
            "https://embed.ted.com/talks/_",
            "https://embed.ted.com/talks/actual_saved_talk/transcript",
            "https://embed.ted.com/talks/lang//actual_saved_talk",
            "https://fast.wistia.net/embed/iframe/",
            "https://fast.wistia.net/embed/iframe/not-a-hashed-id",
            "https://fast.wistia.net/embed/iframe/e4a27b971d/extra",
            "https://actual-account.wistia.com/medias/",
            "https://actual-account.wistia.com/s/share-slug",
            "https://actual-account.wistia.com/channels/e4a27b971d",
        ] {
            assert!(
                playback(source, false).is_err(),
                "Invalid provider source must remain editable without public approval: {source}"
            );
        }
    }

    #[test]
    fn csp_mismatched_hosts_credentials_tokens_ports_and_path_normalization_are_rejected() {
        for source in [
            "https://unsupported.example.test/video.mp4",
            "https://fixture.b-cdn.net.evil.test/video.mp4",
            "https://www.youtube.com.evil.test/embed/abcdefghijk",
            "https://player.vimeo.com.evil.test/video/123456789",
            "https://iframe.mediadelivery.net.evil.test/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe",
            "https://geo.dailymotion.com.evil.test/player/x8lr5.html?video=x84sh87",
            "https://embed.ted.com.evil.test/talks/actual_saved_talk",
            "https://fast.wistia.net.evil.test/embed/iframe/e4a27b971d",
            "https://account.wistia.com.evil.test/medias/e4a27b971d",
            "https://fixture.b-cdn.net:444/video.mp4",
            "https://www.youtube.com:8443/watch?v=abcdefghijk",
            "https://vimeo.com:444/123456789",
            "https://iframe.mediadelivery.net:444/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe",
            "https://geo.dailymotion.com:444/player/x8lr5.html?video=x84sh87",
            "https://embed.ted.com:444/talks/actual_saved_talk",
            "https://fast.wistia.net:444/embed/iframe/e4a27b971d",
            "https://user:password@fixture.b-cdn.net/video.mp4",
            "https://user@www.youtube.com/watch?v=abcdefghijk",
            "https://@fixture.b-cdn.net/video.mp4",
            "https://fixture.b-cdn.net/video.mp4#private",
            "https://fixture.b-cdn.net/.mp4",
            "https://fixture.b-cdn.net/a/../video.mp4",
            "https://fixture.b-cdn.net/%2e%2e/video.mp4",
            "https://fixture.b-cdn.net/folder%2fvideo.mp4",
            "https://fixture.b-cdn.net/folder%5cvideo.mp4",
            "https://fixture.b-cdn.net/folder%252fvideo.mp4",
            "https://fixture.b-cdn.net/bad%zz.mp4",
            "https://fixture.b-cdn.net/bad%.mp4",
            "https://fixture.b-cdn.net/video%0a.mp4",
            "https://fixture.b-cdn.net/bcdn_token=HS256-fixture&expires=1234567890/video.mp4",
            "https://fixture.b-cdn.net/%62cdn_token=HS256-fixture&expires=1234567890/video.mp4",
            "https://fixture.b-cdn.net/BCDN_TOKEN%3DHS256-fixture%26expires%3D1234567890/video.mp4",
            "https://fixture.b-cdn.net/actual video.mp4",
            " https://fixture.b-cdn.net/video.mp4",
            "https://fixture.b-cdn.net/video.mp4\\extra",
        ] {
            assert!(
                playback(source, false).is_err(),
                "Unsafe or CSP-incompatible media must not be approved: {source}"
            );
        }
        for query in [
            "token=secret",
            "TOKEN=secret",
            "access_token=secret",
            "auth_token=secret",
            "token_path=secret",
            "authorization=secret",
            "jwt=secret",
            "expires=1",
            "expiry=1",
            "exp=1",
            "signature=secret",
            "sig=secret",
            "Policy=secret",
            "Key-Pair-Id=secret",
            "X-Amz-Signature=secret",
            "x-goog-credential=secret",
            "%74oken=secret",
        ] {
            for source in [
                format!("https://fixture.b-cdn.net/video.mp4?{query}"),
                format!("https://www.youtube.com/watch?v=abcdefghijk&{query}"),
                format!(
                    "https://iframe.mediadelivery.net/embed/123/eb1c4f77-0cda-46be-b47d-1118ad7c2ffe?{query}"
                ),
                format!("https://geo.dailymotion.com/player/x8lr5.html?video=x84sh87&{query}"),
            ] {
                assert!(
                    playback(&source, false).is_err(),
                    "Tokenized source must not be approved: {source}"
                );
            }
        }
        assert!(public_url("https://media.example.test:444/thumbnail.jpg", false).is_err());
    }

    #[test]
    fn owned_loopback_fixture_is_available_only_on_its_exact_development_origin() {
        assert!(playback("http://127.0.0.1:19030/actual.mp4", true).is_ok());
        assert!(playback("http://127.0.0.1:19030/actual.mp4", false).is_err());
        assert!(playback("http://127.0.0.1:19031/actual.mp4", true).is_err());
        assert!(playback("https://127.0.0.1:19031/actual.mp4", true).is_err());
        assert!(playback("http://localhost:19030/actual.mp4", true).is_err());
        assert!(playback("http://127.0.0.1:19030/actual.mp4?token=secret", true).is_err());
    }
}

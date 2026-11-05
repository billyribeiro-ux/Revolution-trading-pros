//! Real database publication eligibility, saved settings and approval withdrawal.
use super::request;
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/seo/sitemaps";
async fn get(api: &Router, path: &str, token: Option<&str>) -> Value {
    let (status, body) = request(api, "GET", path, token, Value::Null).await;
    assert_eq!(status, StatusCode::OK, "{path}: {body}");
    body
}
fn contains(feed: &Value, id: &str) -> bool {
    feed["entries"]["data"]
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| entry["record"]["id"] == id)
}
pub(super) async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = Some(tokens[0].as_str());
    video_installation_contract(api, pool, admin).await;
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(api, "GET", &format!("{ROOT}/news"), token, Value::Null)
                .await
                .0,
            status
        );
        assert_eq!(
            request(
                api,
                "POST",
                "/api/admin/seo/video-publications",
                token,
                json!({"video_id":1,"author_slug":"fixture","expected_source_hash":"0".repeat(64)})
            )
            .await
            .0,
            status
        );
    }
    let authors: Value = serde_json::from_str(include_str!(
        "../../../frontend/src/lib/authors/authors.json"
    ))
    .unwrap();
    let author = &authors[0];
    let source_admin:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) SELECT 'seo-author@example.test',$1,password_hash,'admin',NOW() FROM users WHERE role='admin' ORDER BY id LIMIT 1 RETURNING id").bind(author["name"].as_str().unwrap()).fetch_one(pool).await.unwrap();
    let author_token =
        revolution_api::utils::create_jwt(source_admin, &std::env::var("JWT_SECRET").unwrap(), 1)
            .unwrap();
    let date = (chrono::Utc::now() - chrono::Duration::minutes(5)).naive_utc();
    let post=request(api,"POST","/api/admin/posts",Some(&author_token),json!({"title":"Actual SEO publication fixture","excerpt":"This actual fixture article description is visible on the published source article.","content_blocks":[],"featured_image":"https://media.example.test/news.jpg","status":"published","published_at":date,"indexable":true})).await;
    assert_eq!(post.0, StatusCode::OK, "{post:?}");
    let post_id = post.1["id"].as_i64().unwrap();
    let post_key = post_id.to_string();
    let public = get(
        api,
        &format!("/api/posts/{}", post.1["slug"].as_str().unwrap()),
        None,
    )
    .await;
    assert_eq!(public["author"]["name"], author["name"]);
    let settings = get(api, &format!("{ROOT}/news"), admin).await;
    assert_eq!(settings["settings"]["enabled"], false);
    let version = settings["settings_version"].as_i64().unwrap();
    let config = json!({"enabled":true,"publication_name":"Actual fixture publication","language":"en","max_age_hours":48,"exclude_categories":[]});
    let saved = request(
        api,
        "PUT",
        &format!("{ROOT}/news"),
        admin,
        json!({"expected_version":version,"config":config}),
    )
    .await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{ROOT}/news"),
            admin,
            json!({"expected_version":version,"config":config})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let feed = get(api, "/api/seo/sitemaps/news", None).await;
    assert!(contains(&feed, &post_id.to_string()), "{feed}");
    let original = feed["entries"]["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["record"]["id"]
                .as_str()
                .is_some_and(|value| value == post_key)
        })
        .unwrap()["record"]["published_at"]
        .clone();
    for (field, value) in [
        ("indexable", json!(false)),
        ("published_at", json!(date - chrono::Duration::hours(49))),
        ("published_at", json!(date + chrono::Duration::days(1))),
    ] {
        assert_eq!(
            request(
                api,
                "PUT",
                &format!("/api/admin/posts/{post_id}"),
                Some(&author_token),
                json!({(field):value})
            )
            .await
            .0,
            StatusCode::OK
        );
        assert!(!contains(
            &get(api, "/api/seo/sitemaps/news", None).await,
            &post_id.to_string()
        ));
        assert_eq!(
            request(
                api,
                "PUT",
                &format!("/api/admin/posts/{post_id}"),
                Some(&author_token),
                json!({"indexable":true,"published_at":date})
            )
            .await
            .0,
            StatusCode::OK
        );
    }
    let generated = request(
        api,
        "POST",
        &format!("{ROOT}/news/generate"),
        admin,
        json!({}),
    )
    .await;
    assert_eq!(generated.0, StatusCode::OK, "{generated:?}");
    assert_eq!(
        generated.1["generation"]["entry_count"],
        feed["entries"]["meta"]["total"]
    );
    let again = get(api, "/api/seo/sitemaps/news", None).await;
    assert_eq!(
        again["entries"]["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["record"]["id"]
                .as_str()
                .is_some_and(|value| value == post_key))
            .unwrap()["record"]["published_at"],
        original
    );
    let page = get(api, &format!("{ROOT}/news?per_page=1"), admin).await;
    assert_eq!(page["preview"]["data"].as_array().unwrap().len(), 1);
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/news?per_page=1001"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    let trader: i64 = sqlx::query_scalar(
        "INSERT INTO traders(name,slug) VALUES($1,'seo-proof-author') RETURNING id",
    )
    .bind(author["name"].as_str().unwrap())
    .fetch_one(pool)
    .await
    .unwrap();
    let instant = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(date, chrono::Utc);
    let date_rfc = instant
        .with_timezone(&chrono::FixedOffset::east_opt(19_800).unwrap())
        .to_rfc3339();
    let previous_zone: String = sqlx::query_scalar("SHOW TIME ZONE")
        .fetch_one(pool)
        .await
        .unwrap();
    set_pool_timezone(pool, "Pacific/Auckland").await;
    let created=request(api,"POST","/api/admin/videos",admin,json!({"title":"Actual public SEO video fixture","description":"The actual saved video description appears unchanged on its public watch page and in its sitemap.","video_url":"https://fixture.b-cdn.net/public-video.mp4","video_platform":"self-hosted","thumbnail_url":"https://media.example.test/actual-video.jpg","duration":600,"video_date":date.date().to_string(),"published_at":date_rfc,"trader_id":trader,"content_type":"learning_center","is_published":true,"room_ids":[]})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["data"]["id"].as_i64().unwrap();
    let source = get(
        api,
        &format!("/api/admin/seo/video-publications/{id}"),
        admin,
    )
    .await;
    assert_eq!(source["record"]["duration"], 600);
    assert_eq!(source["record"]["trader_id"], trader.to_string());
    assert_eq!(
        chrono::DateTime::parse_from_rfc3339(source["record"]["published_at"].as_str().unwrap())
            .unwrap()
            .with_timezone(&chrono::Utc)
            .timestamp_micros(),
        instant.timestamp_micros(),
        "Publication JSON must preserve the supplied instant in a non-UTC database session"
    );
    let stored: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT published_at FROM unified_videos WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        stored.timestamp_micros(),
        instant.timestamp_micros(),
        "Offset-bearing publication dates must bind as TIMESTAMPTZ"
    );
    set_pool_timezone(pool, &previous_zone).await;
    let same_source = get(
        api,
        &format!("/api/admin/seo/video-publications/{id}"),
        admin,
    )
    .await;
    assert_eq!(
        same_source["record"]["source_hash"], source["record"]["source_hash"],
        "Approval fingerprints must be independent of the database session time zone"
    );
    let slug = source["record"]["slug"].as_str().unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/seo/public-videos/{slug}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let approve = json!({"video_id":id,"author_slug":author["slug"],"expected_source_hash":source["record"]["source_hash"]});
    let approved = request(
        api,
        "POST",
        "/api/admin/seo/video-publications",
        admin,
        approve.clone(),
    )
    .await;
    assert_eq!(approved.0, StatusCode::OK, "{approved:?}");
    let watch = get(api, &format!("/api/seo/public-videos/{slug}"), None).await;
    assert_eq!(watch["title"], source["record"]["title"]);
    assert_eq!(watch["description"], source["record"]["description"]);
    assert_eq!(watch["playback_kind"], "video");
    assert_eq!(watch["playback_url"], source["record"]["video_url"]);
    let settings = get(api, &format!("{ROOT}/video"), admin).await;
    let config = json!({"enabled":true,"include_thumbnails":true,"include_uploader":true,"detect_from_content":true,"platforms":["self-hosted","bunny","youtube","vimeo","dailymotion","ted","wistia"]});
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{ROOT}/video"),
            admin,
            json!({"expected_version":settings["settings_version"],"config":config})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert!(contains(
        &get(api, "/api/seo/sitemaps/video", None).await,
        &id.to_string()
    ));
    // Saved platform selection controls sitemap inclusion without inventing private/public status.
    let settings = get(api, &format!("{ROOT}/video"), admin).await;
    let mut excluded = settings["settings"].clone();
    excluded["platforms"] = json!(["youtube", "vimeo", "dailymotion", "ted", "wistia", "bunny"]);
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{ROOT}/video"),
            admin,
            json!({"expected_version":settings["settings_version"],"config":excluded})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert!(!contains(
        &get(api, "/api/seo/sitemaps/video", None).await,
        &id.to_string()
    ));
    get(api, &format!("/api/seo/public-videos/{slug}"), None).await;
    let settings = get(api, &format!("{ROOT}/video"), admin).await;
    let mut included = settings["settings"].clone();
    included["platforms"] = json!([
        "self-hosted",
        "bunny",
        "youtube",
        "vimeo",
        "dailymotion",
        "ted",
        "wistia"
    ]);
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{ROOT}/video"),
            admin,
            json!({"expected_version":settings["settings_version"],"config":included})
        )
        .await
        .0,
        StatusCode::OK
    );
    // A real metadata change withdraws approval and stale approval intent conflicts.
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("/api/admin/videos/{id}"),
            admin,
            json!({"duration":601})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/seo/public-videos/{slug}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert!(!contains(
        &get(api, "/api/seo/sitemaps/video", None).await,
        &id.to_string()
    ));
    assert_eq!(
        request(
            api,
            "POST",
            "/api/admin/seo/video-publications",
            admin,
            approve
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let source = get(
        api,
        &format!("/api/admin/seo/video-publications/{id}"),
        admin,
    )
    .await;
    assert_eq!(source["record"]["duration"], 601);
    let approve = json!({"video_id":id,"author_slug":author["slug"],"expected_source_hash":source["record"]["source_hash"]});
    assert_eq!(
        request(
            api,
            "POST",
            "/api/admin/seo/video-publications",
            admin,
            approve
        )
        .await
        .0,
        StatusCode::OK
    );
    let room: i64 = sqlx::query_scalar("SELECT id FROM trading_rooms ORDER BY id LIMIT 1")
        .fetch_one(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO video_room_assignments(video_id,room_id) VALUES($1,$2)")
        .bind(id)
        .bind(room)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/seo/public-videos/{slug}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert!(!contains(
        &get(api, "/api/seo/sitemaps/video", None).await,
        &id.to_string()
    ));
    sqlx::query("DELETE FROM video_room_assignments WHERE video_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("/api/admin/seo/video-publications/{id}"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/seo/public-videos/{slug}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let audit:i64=sqlx::query_scalar("SELECT COUNT(*) FROM seo_sitemap_audit WHERE action IN ('settings_saved','generated','public_watch_page_approved','public_watch_page_removed')").fetch_one(pool).await.unwrap();
    assert!(audit >= 6, "Actual saved actions must have audit records");
}

/// The fixture pool is owned by this proof. Hold every connection while changing
/// its session setting, so an API request cannot accidentally use an untested UTC
/// connection and conceal a naive timestamp binding or serialization defect.
async fn set_pool_timezone(pool: &PgPool, zone: &str) {
    let mut connections = Vec::new();
    for _ in 0..pool.options().get_max_connections() {
        let mut connection = pool.acquire().await.unwrap();
        sqlx::query("SELECT set_config('TimeZone',$1,false)")
            .bind(zone)
            .execute(&mut *connection)
            .await
            .unwrap();
        connections.push(connection);
    }
}

async fn video_installation_contract(api: &Router, pool: &PgPool, admin: Option<&str>) {
    let legacy: i64 = sqlx::query_scalar("INSERT INTO unified_videos(title,slug,content_type,video_url,video_platform,video_date,is_published,is_featured,views_count,created_at,updated_at) VALUES('Owned incomplete historical video','owned-incomplete-historical-video','learning_center',NULL,NULL,NULL,true,NULL,NULL,NULL,NULL) RETURNING id").fetch_one(pool).await.unwrap();
    for path in [
        format!("/api/admin/videos/{legacy}"),
        format!("/api/videos/{legacy}"),
    ] {
        let row = get(api, &path, admin).await;
        for field in [
            "video_url",
            "embed_url",
            "video_platform",
            "duration",
            "formatted_duration",
            "video_date",
            "formatted_date",
            "views_count",
            "created_at",
            "is_featured",
        ] {
            assert!(
                row["data"][field].is_null(),
                "Unknown legacy {field} must remain unknown: {path}: {row}"
            );
        }
    }
    let catalog = get(api, "/api/admin/videos?per_page=100", admin).await;
    assert!(
        catalog["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["id"] == legacy),
        "A nullable metadata row must not break or disappear from the admin catalog"
    );
    assert!(
        get(api, "/api/admin/videos/stats", admin).await["data"]["total_views"].is_null(),
        "An unknown historical view counter must not become a fabricated total of zero"
    );
    let candidate = get(
        api,
        &format!("/api/admin/seo/video-publications/{legacy}"),
        admin,
    )
    .await;
    assert!(candidate["record"]["published_at"].is_null());
    assert!(candidate["record"]["author_name"].is_null());
    assert!(candidate["record"]["trader_id"].is_null());
    assert_eq!(
        request(
            api,
            "GET",
            "/api/seo/public-videos/owned-incomplete-historical-video",
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );

    // A PostgreSQL DATE can exceed chrono's supported date range. Inject that
    // real decode failure after INSERT/UPDATE only for this owned fixture.
    sqlx::query("CREATE FUNCTION video_fixture_decode_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.title IN ('Owned video decode rollback fixture','Owned video update decode rollback fixture') THEN NEW.video_date=DATE '300000-01-01'; END IF; RETURN NEW; END $$").execute(pool).await.unwrap();
    sqlx::query("CREATE TRIGGER video_fixture_decode_failure BEFORE INSERT OR UPDATE ON unified_videos FOR EACH ROW EXECUTE FUNCTION video_fixture_decode_failure()").execute(pool).await.unwrap();
    let fields = json!({"title":"Owned video decode rollback fixture","video_url":"https://fixture.b-cdn.net/rollback.mp4","video_platform":"self-hosted","content_type":"learning_center","video_date":"2026-05-20","is_published":false,"room_ids":[]});
    let failed = request(api, "POST", "/api/admin/videos", admin, fields.clone()).await;
    assert_eq!(failed.0, StatusCode::INTERNAL_SERVER_ERROR, "{failed:?}");
    let persisted: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM unified_videos WHERE slug='owned-video-decode-rollback-fixture'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        persisted, 0,
        "A post-insert decode error must roll back the video instead of creating a duplicate on retry"
    );
    sqlx::query("DROP TRIGGER video_fixture_decode_failure ON unified_videos")
        .execute(pool)
        .await
        .unwrap();
    let saved = request(api, "POST", "/api/admin/videos", admin, fields).await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    let id = saved.1["data"]["id"].as_i64().unwrap();
    sqlx::query("CREATE TRIGGER video_fixture_decode_failure BEFORE UPDATE ON unified_videos FOR EACH ROW EXECUTE FUNCTION video_fixture_decode_failure()").execute(pool).await.unwrap();
    let failed = request(
        api,
        "PUT",
        &format!("/api/admin/videos/{id}"),
        admin,
        json!({"title":"Owned video update decode rollback fixture"}),
    )
    .await;
    assert_eq!(failed.0, StatusCode::INTERNAL_SERVER_ERROR, "{failed:?}");
    let actual: (String, chrono::NaiveDate) =
        sqlx::query_as("SELECT title,video_date FROM unified_videos WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(actual.0, "Owned video decode rollback fixture");
    assert_eq!(
        actual.1,
        chrono::NaiveDate::from_ymd_opt(2026, 5, 20).unwrap()
    );
    sqlx::query("DROP TRIGGER video_fixture_decode_failure ON unified_videos")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DROP FUNCTION video_fixture_decode_failure()")
        .execute(pool)
        .await
        .unwrap();
    // Unsupported historical values must produce an ordinary HTTP error, not
    // unwind through SQLx's unchecked Chrono decoder or become fake NULLs.
    let malformed: i64 = sqlx::query_scalar("INSERT INTO unified_videos(title,slug,content_type,video_date,is_published) VALUES('Owned unsupported calendar fixture','owned-unsupported-calendar-fixture','learning_center',DATE '300000-01-01',true) RETURNING id").fetch_one(pool).await.unwrap();
    for timestamp_infinity in [false, true] {
        if timestamp_infinity {
            sqlx::query("UPDATE unified_videos SET video_date=NULL,created_at='infinity'::TIMESTAMPTZ WHERE id=$1").bind(malformed).execute(pool).await.unwrap();
        }
        for path in [
            format!("/api/admin/videos/{malformed}"),
            format!("/api/videos/{malformed}"),
        ] {
            let response = request(api, "GET", &path, admin, Value::Null).await;
            assert_eq!(
                response.0,
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unsupported stored temporal metadata must be a controlled request error: {path}: {response:?}"
            );
        }
    }
    // Source fixtures remain private and retain NULL original publication facts.
    for id in [legacy, id, malformed] {
        assert_eq!(
            request(
                api,
                "DELETE",
                &format!("/api/admin/videos/{id}"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::OK
        );
    }
}

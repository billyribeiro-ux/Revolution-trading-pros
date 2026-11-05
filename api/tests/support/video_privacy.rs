//! Public publication and exact current room-membership boundaries across every video entry point.
use super::request;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
async fn call(api: &Router, path: &str, token: Option<&str>) -> Value {
    let result = request(api, "GET", path, token, Value::Null).await;
    assert_eq!(result.0, StatusCode::OK, "{path}: {result:?}");
    result.1
}
fn has(value: &Value, id: i64) -> bool {
    value["data"]
        .as_array()
        .unwrap()
        .iter()
        .any(|row| row["id"] == id || row["video_id"] == id)
}
async fn video(
    api: &Router,
    admin: &str,
    title: &str,
    kind: &str,
    room: Option<i64>,
    trader: i64,
) -> Value {
    let result=request(api,"POST","/api/admin/videos",Some(admin),json!({"title":title,"description":"The actual owned privacy fixture video has a complete saved public description.","video_url":"https://fixture.b-cdn.net/privacy.mp4","video_platform":"self-hosted","thumbnail_url":"https://fixture.b-cdn.net/privacy.jpg","duration":2,"video_date":chrono::Utc::now().date_naive().to_string(),"published_at":chrono::Utc::now().to_rfc3339(),"trader_id":trader,"content_type":kind,"is_published":true,"room_ids":room.into_iter().collect::<Vec<_>>()})).await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    result.1["data"].clone()
}
pub(super) async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = &tokens[0];
    let authors: Value = serde_json::from_str(include_str!(
        "../../../frontend/src/lib/authors/authors.json"
    ))
    .unwrap();
    let author = &authors[0];
    let trader: i64 = sqlx::query_scalar(
        "INSERT INTO traders(name,slug) VALUES($1,'video-privacy-fixture-author') RETURNING id",
    )
    .bind(author["name"].as_str().unwrap())
    .fetch_one(pool)
    .await
    .unwrap();
    let mut rooms = Vec::new();
    for letter in ["a", "b"] {
        rooms.push(
            sqlx::query_scalar::<_, i64>(
                "INSERT INTO trading_rooms(name,slug) VALUES($1,$2) RETURNING id",
            )
            .bind(format!("Actual video privacy {letter}"))
            .bind(format!("video-privacy-fixture-{letter}"))
            .fetch_one(pool)
            .await
            .unwrap(),
        );
    }
    let mut people = Vec::new();
    let mut memberships = Vec::new();
    for (suffix, room) in [
        ("room-a", Some(rooms[0])),
        ("room-b", Some(rooms[1])),
        ("unmapped", None),
    ] {
        let user:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) SELECT $1,'Actual video member',password_hash,'user',NOW() FROM users WHERE role='admin' ORDER BY id LIMIT 1 RETURNING id").bind(format!("privacy-{suffix}@example.test")).fetch_one(pool).await.unwrap();
        let token =
            revolution_api::utils::create_jwt(user, &std::env::var("JWT_SECRET").unwrap(), 1)
                .unwrap();
        people.push((user, token));
        let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle,room_id,metadata) VALUES($1,$2,0,'monthly',$3,'{}') RETURNING id").bind(format!("Actual private plan {suffix}")).bind(format!("privacy-{suffix}")).bind(room).fetch_one(pool).await.unwrap();
        memberships.push(sqlx::query_scalar::<_,i64>("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at) VALUES($1,$2,'active',NOW()-INTERVAL '1 minute',NOW()+INTERVAL '1 day') RETURNING id").bind(user).bind(plan).fetch_one(pool).await.unwrap());
    }
    let a = video(
        api,
        admin,
        "Privacy room A learning",
        "learning_center",
        Some(rooms[0]),
        trader,
    )
    .await;
    let b = video(
        api,
        admin,
        "Privacy room B learning",
        "learning_center",
        Some(rooms[1]),
        trader,
    )
    .await;
    let weekly_a = video(
        api,
        admin,
        "Privacy room A weekly",
        "weekly_watchlist",
        Some(rooms[0]),
        trader,
    )
    .await;
    let weekly_b = video(
        api,
        admin,
        "Privacy room B weekly",
        "weekly_watchlist",
        Some(rooms[1]),
        trader,
    )
    .await;
    let private = video(
        api,
        admin,
        "Privacy unmapped learning",
        "learning_center",
        None,
        trader,
    )
    .await;
    let public = video(
        api,
        admin,
        "Privacy approved public learning",
        "learning_center",
        None,
        trader,
    )
    .await;
    let public_id = public["id"].as_i64().unwrap();
    let source = call(
        api,
        &format!("/api/admin/seo/video-publications/{public_id}"),
        Some(admin),
    )
    .await;
    let result=request(api,"POST","/api/admin/seo/video-publications",Some(admin),json!({"video_id":public_id,"author_slug":author["slug"],"expected_source_hash":source["record"]["source_hash"]})).await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    let public_page = call(api, "/api/videos?per_page=100", None).await;
    assert!(has(&public_page, public_id));
    public_playback_identity(api, pool, admin, author, trader, public_id).await;
    for item in [&a, &b, &weekly_a, &weekly_b, &private] {
        let id = item["id"].as_i64().unwrap();
        assert!(!has(&public_page, id));
        for identity in [id.to_string(), item["slug"].as_str().unwrap().to_string()] {
            assert_eq!(
                request(
                    api,
                    "GET",
                    &format!("/api/videos/{identity}"),
                    None,
                    Value::Null
                )
                .await
                .0,
                StatusCode::NOT_FOUND
            );
        }
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/videos/{id}/related"),
                None,
                Value::Null
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            request(
                api,
                "POST",
                &format!("/api/videos/{id}/track"),
                None,
                json!({"event_type":"play"})
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/videos/{id}"),
                Some(&tokens[1]),
                Value::Null
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/videos/{id}"),
                Some(admin),
                Value::Null
            )
            .await
            .0,
            StatusCode::OK
        );
    }
    call(api, &format!("/api/videos/{public_id}"), None).await;
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/videos/{public_id}"),
            Some("invalid-fixture-token"),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let a_id = a["id"].as_i64().unwrap();
    let b_id = b["id"].as_i64().unwrap();
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
        (Some(people[1].1.as_str()), StatusCode::FORBIDDEN),
        (Some(people[2].1.as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/videos?room_id={}", rooms[0]),
                token,
                Value::Null
            )
            .await
            .0,
            status
        );
    }
    let member = Some(people[0].1.as_str());
    let scoped = call(
        api,
        &format!("/api/videos?room_id={}&per_page=1", rooms[0]),
        member,
    )
    .await;
    assert_eq!(scoped["meta"]["total"], 2);
    assert_eq!(scoped["meta"]["last_page"], 2);
    assert_eq!(scoped["data"].as_array().unwrap().len(), 1);
    let catalog = call(api, "/api/videos?per_page=100", member).await;
    assert!(has(&catalog, a_id));
    assert!(has(&catalog, public_id));
    assert!(!has(&catalog, b_id));
    assert!(!has(&catalog, private["id"].as_i64().unwrap()));
    let related = call(api, &format!("/api/videos/{a_id}/related"), member).await;
    assert!(!has(&related, b_id));
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/videos/{b_id}/related"),
            member,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let weekly = call(api, "/api/videos/weekly", member).await;
    assert!(has(&weekly, weekly_a["id"].as_i64().unwrap()));
    assert!(!has(&weekly, weekly_b["id"].as_i64().unwrap()));
    assert_eq!(
        request(api, "GET", "/api/videos/weekly", None, Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let date = chrono::Utc::now().date_naive();
    let by_slug = call(
        api,
        &format!("/api/videos?room_slug=video-privacy-fixture-a&video_date={date}"),
        member,
    )
    .await;
    assert_eq!(by_slug["meta"]["total"], 2);
    for id in [a_id, b_id] {
        sqlx::query("INSERT INTO video_watch_progress(user_id,video_id,current_time_seconds,completion_percent) VALUES($1,$2,1,50)").bind(people[0].0).bind(id).execute(pool).await.unwrap();
    }
    let history = call(api, "/api/videos/history", member).await;
    assert!(has(&history, a_id));
    assert!(!has(&history, b_id));
    assert_eq!(
        request(
            api,
            "POST",
            &format!("/api/videos/{a_id}/track"),
            member,
            json!({"event_type":"play"})
        )
        .await
        .0,
        StatusCode::OK
    );
    for path in [
        "/api/videos".to_string(),
        format!("/api/videos/{a_id}"),
        format!("/api/videos/{a_id}/related"),
        "/api/videos/weekly".into(),
        "/api/videos/history".into(),
    ] {
        let response = api
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header("authorization", format!("Bearer {}", people[0].1))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["cache-control"], "private, no-store");
        assert_eq!(response.headers()["vary"], "Authorization, Cookie");
    }
    // Trial status requires a current trial window even when general expiry is still future.
    for status in ["trial", "trialing"] {
        sqlx::query("UPDATE user_memberships SET status=$2,trial_ends_at=(NOW() AT TIME ZONE 'UTC')+INTERVAL '1 hour' WHERE id=$1").bind(memberships[0]).bind(status).execute(pool).await.unwrap();
        let current = call(api, &format!("/api/videos?room_id={}", rooms[0]), member).await;
        assert!(has(&current, a_id));
        call(api, &format!("/api/videos/{a_id}"), member).await;
        call(api, &format!("/api/videos/{a_id}/related"), member).await;
        assert!(has(&call(api, "/api/videos/history", member).await, a_id));
        assert_eq!(
            request(
                api,
                "POST",
                &format!("/api/videos/{a_id}/track"),
                member,
                json!({"event_type":"play"})
            )
            .await
            .0,
            StatusCode::OK
        );
        sqlx::query("UPDATE user_memberships SET trial_ends_at=(NOW() AT TIME ZONE 'UTC')-INTERVAL '1 minute' WHERE id=$1").bind(memberships[0]).execute(pool).await.unwrap();
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/videos?room_id={}", rooms[0]),
                member,
                Value::Null
            )
            .await
            .0,
            StatusCode::FORBIDDEN
        );
        assert!(!has(
            &call(api, "/api/videos?per_page=100", member).await,
            a_id
        ));
        assert!(!has(&call(api, "/api/videos/history", member).await, a_id));
        assert!(!has(
            &call(api, "/api/videos/weekly", member).await,
            weekly_a["id"].as_i64().unwrap()
        ));
        for suffix in ["", "/related"] {
            assert_eq!(
                request(
                    api,
                    "GET",
                    &format!("/api/videos/{a_id}{suffix}"),
                    member,
                    Value::Null
                )
                .await
                .0,
                StatusCode::NOT_FOUND
            );
        }
        assert_eq!(
            request(
                api,
                "POST",
                &format!("/api/videos/{a_id}/track"),
                member,
                json!({"event_type":"play"})
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
    }
    sqlx::query("UPDATE user_memberships SET status='active' WHERE id=$1")
        .bind(memberships[0])
        .execute(pool)
        .await
        .unwrap();
    call(api, &format!("/api/videos/{a_id}"), member).await;
    // Immediate membership revocation affects list, direct, related, weekly, history and writes.
    sqlx::query("UPDATE user_memberships SET status='cancelled' WHERE id=$1")
        .bind(memberships[0])
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/videos?room_id={}", rooms[0]),
            member,
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert!(!has(
        &call(api, "/api/videos?per_page=100", member).await,
        a_id
    ));
    assert!(!has(&call(api, "/api/videos/history", member).await, a_id));
    assert!(!has(
        &call(api, "/api/videos/weekly", member).await,
        weekly_a["id"].as_i64().unwrap()
    ));
    for suffix in ["", "/related"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/videos/{a_id}{suffix}"),
                member,
                Value::Null
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
    }
    assert_eq!(
        request(
            api,
            "POST",
            &format!("/api/videos/{a_id}/track"),
            member,
            json!({"event_type":"play"})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    call(api, &format!("/api/videos/{public_id}"), member).await;
    // Current-period expiration and deleted grants do not act as a global membership.
    sqlx::query("UPDATE user_memberships SET status='active',expires_at=NOW()-INTERVAL '1 minute' WHERE id=$1").bind(memberships[0]).execute(pool).await.unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/videos/{a_id}"),
            member,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    sqlx::query("UPDATE user_memberships SET expires_at=NOW()+INTERVAL '1 day',deleted_at=NOW() WHERE id=$1").bind(memberships[0]).execute(pool).await.unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/videos/{a_id}"),
            member,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    // Source edits withdraw public approval from the legacy catalog too.
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("/api/admin/videos/{public_id}"),
            Some(admin),
            json!({"duration":3})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/videos/{public_id}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
}

async fn public_playback_identity(
    api: &Router,
    pool: &PgPool,
    admin: &str,
    author: &Value,
    trader: i64,
    related_source: i64,
) {
    let source = video(
        api,
        admin,
        "Privacy Bunny source identity",
        "learning_center",
        None,
        trader,
    )
    .await;
    let id = source["id"].as_i64().unwrap();
    let slug = source["slug"].as_str().unwrap();
    let guid = "eb1c4f77-0cda-46be-b47d-1118ad7c2ffe";
    let other_guid = "fb1c4f77-0cda-46be-b47d-1118ad7c2ffe";
    let raw = format!("https://player.mediadelivery.net/embed/123/{guid}?autoplay=false");
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("/api/admin/videos/{id}"),
            Some(admin),
            json!({"video_platform":"bunny","video_url":raw})
        )
        .await
        .0,
        StatusCode::OK
    );
    // These established legacy fields are populated by the Bunny upload path,
    // rather than by the metadata CRUD. A controlled fixture recreates that state.
    sqlx::query("UPDATE unified_videos SET bunny_video_guid=$2,bunny_library_id=123 WHERE id=$1")
        .bind(id)
        .bind(other_guid)
        .execute(pool)
        .await
        .unwrap();
    let path = format!("/api/admin/seo/video-publications/{id}");
    let mismatched = call(api, &path, Some(admin)).await;
    let result = request(api,"POST","/api/admin/seo/video-publications",Some(admin),json!({"video_id":id,"author_slug":author["slug"],"expected_source_hash":mismatched["record"]["source_hash"]})).await;
    assert_eq!(
        result.0,
        StatusCode::UNPROCESSABLE_ENTITY,
        "An inconsistent stored Bunny identity must not approve: {result:?}"
    );
    sqlx::query("UPDATE unified_videos SET bunny_video_guid=$2 WHERE id=$1")
        .bind(id)
        .bind(guid)
        .execute(pool)
        .await
        .unwrap();
    let reviewed = call(api, &path, Some(admin)).await;
    assert_ne!(
        reviewed["record"]["source_hash"],
        mismatched["record"]["source_hash"]
    );
    let result=request(api,"POST","/api/admin/seo/video-publications",Some(admin),json!({"video_id":id,"author_slug":author["slug"],"expected_source_hash":reviewed["record"]["source_hash"]})).await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    let watch = call(api, &format!("/api/seo/public-videos/{slug}"), None).await;
    assert_eq!(watch["playback_url"], raw);
    for token in [None, Some(admin)] {
        let detail = call(api, &format!("/api/videos/{id}"), token).await;
        assert_eq!(detail["data"]["video_url"], watch["playback_url"]);
        assert_eq!(detail["data"]["embed_url"], watch["playback_url"]);
    }
    for page in [
        call(api, "/api/videos?per_page=100", None).await,
        call(api, &format!("/api/videos/{related_source}/related"), None).await,
    ] {
        let row = page["data"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["id"] == id)
            .expect("Approved video should appear in the actual public catalog and related result");
        assert_eq!(row["video_url"], watch["playback_url"]);
        assert_eq!(row["embed_url"], watch["playback_url"]);
    }
    for change_library in [false, true] {
        if change_library {
            sqlx::query("UPDATE unified_videos SET bunny_library_id=124 WHERE id=$1")
                .bind(id)
                .execute(pool)
                .await
                .unwrap();
        } else {
            sqlx::query("UPDATE unified_videos SET bunny_video_guid=$2 WHERE id=$1")
                .bind(id)
                .bind(other_guid)
                .execute(pool)
                .await
                .unwrap();
        }
        let changed = call(api, &path, Some(admin)).await;
        assert_ne!(
            changed["record"]["source_hash"],
            reviewed["record"]["source_hash"]
        );
        assert!(!has(&call(api, "/api/videos?per_page=100", None).await, id));
        assert!(!has(
            &call(api, &format!("/api/videos/{related_source}/related"), None).await,
            id
        ));
        for path in [
            format!("/api/videos/{id}"),
            format!("/api/seo/public-videos/{slug}"),
        ] {
            assert_eq!(
                request(api, "GET", &path, None, Value::Null).await.0,
                StatusCode::NOT_FOUND,
                "Changing a playback-affecting stored field must withdraw every public entry point: {path}"
            );
        }
        let result = request(api,"POST","/api/admin/seo/video-publications",Some(admin),json!({"video_id":id,"author_slug":author["slug"],"expected_source_hash":changed["record"]["source_hash"]})).await;
        assert_eq!(result.0, StatusCode::UNPROCESSABLE_ENTITY);
        let staff = call(api, &format!("/api/videos/{id}"), Some(admin)).await;
        let private_expected = if change_library {
            format!("https://iframe.mediadelivery.net/embed/124/{guid}")
        } else {
            format!("https://iframe.mediadelivery.net/embed/123/{other_guid}")
        };
        assert_eq!(
            staff["data"]["embed_url"], private_expected,
            "Staff access preserves established private playback"
        );
        sqlx::query(
            "UPDATE unified_videos SET bunny_video_guid=$2,bunny_library_id=123 WHERE id=$1",
        )
        .bind(id)
        .bind(guid)
        .execute(pool)
        .await
        .unwrap();
        assert_eq!(
            call(api, &format!("/api/videos/{id}"), None).await["data"]["embed_url"],
            watch["playback_url"]
        );
    }
}

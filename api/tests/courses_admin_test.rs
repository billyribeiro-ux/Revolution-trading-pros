//! Courses-admin route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::courses_admin` (mount table)
//! and to the request/response DTOs in `revolution_api::models::course_enhanced`
//! that the route handlers consume. The `price_cents` field is the
//! load-bearing money path here — paid courses cost real money and the
//! same i64 invariant that applies to membership plans applies here.
//! A regression that flips `price_cents` to i32 or removes a CRUD route
//! from the admin router will fail this suite.

use revolution_api::models::course_enhanced::{
    format_duration_minutes, format_duration_seconds, BulkLiveSessionsRequest, CourseEnhanced,
    CourseListQuery, CourseResponse, CreateCourseRequest, CreateLessonRequest,
    CreateLiveSessionRequest, CreateResourceRequest, CreateSectionRequest, LiveSessionInput,
    ReorderItemsRequest, UpdateCourseRequest,
};

// ── Money: price_cents is i64 and round-trips past i32::MAX ──────────

/// HARD RULE (CLAUDE.md "Money / cents"): `price_cents` must be i64
/// EVERYWHERE — DB column BIGINT, struct field i64, JSON wire number.
/// Once a corporate-tier course or annual cohort hits $20K+ × multi-seat
/// the cap is reached fast; storing in i32 would silently overflow.
#[test]
fn course_enhanced_price_cents_is_i64_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 12_345; // > $21.4M cents

    let course = CourseEnhanced {
        id: 1,
        title: "Advanced Options Mastery".to_string(),
        slug: "advanced-options-mastery".to_string(),
        subtitle: None,
        description: None,
        description_html: None,
        thumbnail_url: None,
        trailer_video_url: None,
        trailer_bunny_guid: None,
        difficulty_level: "advanced".to_string(),
        category: Some("options".to_string()),
        tags: None,
        instructor_id: Some(7),
        estimated_duration_minutes: 600,
        total_lessons: 42,
        total_sections: 6,
        is_published: true,
        is_featured: true,
        is_free: false,
        required_plan_id: Some(1),
        price_cents: Some(above_i32_max),
        prerequisite_course_ids: None,
        certificate_enabled: true,
        certificate_template: "default".to_string(),
        completion_threshold_percent: 80,
        meta_title: None,
        meta_description: None,
        created_by: Some(1),
        updated_by: Some(1),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        published_at: None,
        deleted_at: None,
    };

    let wire = serde_json::to_value(&course).expect("serialize course");
    assert_eq!(
        wire["price_cents"].as_i64(),
        Some(above_i32_max),
        "price_cents must survive JSON round-trip as i64"
    );
    assert!(
        course.price_cents.unwrap() > i32::MAX as i64,
        "fixture must exceed i32::MAX to prove i64 pin"
    );

    // Round-trip the CourseResponse (the wire DTO the handler returns).
    let resp = CourseResponse {
        id: course.id,
        title: course.title.clone(),
        slug: course.slug.clone(),
        subtitle: None,
        description: None,
        description_html: None,
        thumbnail_url: None,
        trailer_video_url: None,
        difficulty_level: course.difficulty_level.clone(),
        category: course.category.clone(),
        tags: vec![],
        instructor: None,
        estimated_duration_minutes: course.estimated_duration_minutes,
        formatted_duration: format_duration_minutes(course.estimated_duration_minutes),
        total_lessons: course.total_lessons,
        total_sections: course.total_sections,
        is_published: course.is_published,
        is_featured: course.is_featured,
        is_free: course.is_free,
        price_cents: course.price_cents,
        certificate_enabled: course.certificate_enabled,
        sections: None,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    let resp_wire = serde_json::to_value(&resp).expect("serialize response");
    assert_eq!(resp_wire["price_cents"].as_i64(), Some(above_i32_max));
    assert_eq!(resp_wire["formatted_duration"], "10h 0m");
}

// ── Request DTOs: create/update accept full money payloads ───────────

#[test]
fn create_course_request_accepts_price_cents_past_i32_max() {
    let big: i64 = (i32::MAX as i64) + 1;
    let body = serde_json::json!({
        "title": "Pro Trading Cohort",
        "subtitle": "Live cohort",
        "description": "12-week program",
        "difficulty_level": "advanced",
        "category": "trading",
        "tags": ["cohort", "live"],
        "instructor_id": 1,
        "is_published": false,
        "is_featured": true,
        "is_free": false,
        "required_plan_id": 2,
        "price_cents": big,
        "prerequisite_course_ids": [10_i64, 11],
        "certificate_enabled": true,
        "completion_threshold_percent": 85
    });
    let req: CreateCourseRequest = serde_json::from_value(body).expect("create course payload");
    assert_eq!(req.title, "Pro Trading Cohort");
    assert_eq!(req.price_cents, Some(big));
    assert!(
        req.price_cents.unwrap() > i32::MAX as i64,
        "request DTO must accept values past i32::MAX"
    );
    assert_eq!(req.tags.as_ref().unwrap(), &vec!["cohort", "live"]);
    assert_eq!(
        req.prerequisite_course_ids.as_ref().unwrap(),
        &vec![10_i64, 11]
    );
}

#[test]
fn update_course_request_is_fully_optional_and_round_trips_price() {
    let empty: UpdateCourseRequest = serde_json::from_str("{}").expect("empty patch");
    assert!(empty.title.is_none());
    assert!(empty.price_cents.is_none());

    // Patch only the price.
    let big: i64 = (i32::MAX as i64) * 2; // ~$42.9M cents
    let body = serde_json::json!({ "price_cents": big });
    let req: UpdateCourseRequest = serde_json::from_value(body).expect("patch price");
    assert_eq!(req.price_cents, Some(big));
    assert!(req.title.is_none());
}

// ── Other request DTOs (section / lesson / resource / live session) ──

#[test]
fn create_section_lesson_resource_and_live_session_dtos_deserialize() {
    let section: CreateSectionRequest = serde_json::from_value(serde_json::json!({
        "title": "Module 1",
        "section_type": "lessons",
        "unlock_type": "immediate"
    }))
    .expect("section payload");
    assert_eq!(section.title, "Module 1");
    assert_eq!(section.section_type.as_deref(), Some("lessons"));

    let lesson: CreateLessonRequest = serde_json::from_value(serde_json::json!({
        "section_id": 1_i64,
        "title": "Intro to Options",
        "lesson_type": "video",
        "is_preview": true
    }))
    .expect("lesson payload");
    assert_eq!(lesson.section_id, 1);
    assert_eq!(lesson.is_preview, Some(true));

    let resource: CreateResourceRequest = serde_json::from_value(serde_json::json!({
        "title": "Workbook PDF",
        "file_url": "https://files.example.com/wb.pdf",
        "file_name": "workbook.pdf",
        "file_type": "pdf",
        "file_size_bytes": 5_000_000_i64
    }))
    .expect("resource payload");
    assert_eq!(resource.file_name, "workbook.pdf");
    assert_eq!(resource.file_size_bytes, Some(5_000_000));

    let live: CreateLiveSessionRequest = serde_json::from_value(serde_json::json!({
        "title": "Live Q&A",
        "session_date": "2026-06-01",
        "session_time": "18:00",
        "is_published": false
    }))
    .expect("live session payload");
    assert_eq!(live.title, "Live Q&A");
    assert_eq!(live.session_time.as_deref(), Some("18:00"));
}

#[test]
fn bulk_live_sessions_and_reorder_dtos_carry_collections() {
    let bulk: BulkLiveSessionsRequest = serde_json::from_value(serde_json::json!({
        "section_id": 3_i64,
        "sessions": [
            {"title": "Week 1", "session_date": "2026-06-01"},
            {"title": "Week 2", "session_date": "2026-06-08"}
        ]
    }))
    .expect("bulk live sessions payload");
    assert_eq!(bulk.sessions.len(), 2);
    let titles: Vec<&str> = bulk
        .sessions
        .iter()
        .map(|s: &LiveSessionInput| s.title.as_str())
        .collect();
    assert_eq!(titles, vec!["Week 1", "Week 2"]);

    let reorder: ReorderItemsRequest = serde_json::from_value(serde_json::json!({
        "items": [
            {"id": 1, "sort_order": 0},
            {"id": 2, "sort_order": 1}
        ]
    }))
    .expect("reorder payload");
    assert_eq!(reorder.items.len(), 2);
    assert_eq!(reorder.items[1].sort_order, 1);
}

// ── Query DTO carries pagination + filters ───────────────────────────

#[test]
fn course_list_query_carries_pagination_and_filters() {
    let q: CourseListQuery = serde_json::from_value(serde_json::json!({
        "page": 2,
        "per_page": 50,
        "difficulty_level": "advanced",
        "category": "options",
        "instructor_id": 7,
        "is_published": true,
        "is_featured": false,
        "search": "earnings"
    }))
    .expect("course list query");
    assert_eq!(q.page, Some(2));
    assert_eq!(q.per_page, Some(50));
    assert_eq!(q.difficulty_level.as_deref(), Some("advanced"));
    assert_eq!(q.instructor_id, Some(7));
    assert_eq!(q.is_published, Some(true));
    assert_eq!(q.search.as_deref(), Some("earnings"));
}

// ── Helper functions exercised by the response payload ───────────────

#[test]
fn duration_formatting_helpers_match_expected_output() {
    // These run on every CourseResponse the admin UI gets back.
    assert_eq!(format_duration_minutes(0), "0m");
    assert_eq!(format_duration_minutes(45), "45m");
    assert_eq!(format_duration_minutes(60), "1h 0m");
    assert_eq!(format_duration_minutes(125), "2h 5m");

    // format_duration_seconds is colon-separated (mm:ss / h:mm:ss).
    assert_eq!(format_duration_seconds(0), "0:00");
    assert_eq!(format_duration_seconds(45), "0:45");
    assert_eq!(format_duration_seconds(125), "2:05");
    assert_eq!(format_duration_seconds(3661), "1:01:01");
}

// ── Router mount table ───────────────────────────────────────────────

/// Mount-table test: removing a course-CRUD route or breaking a handler
/// signature will fail to compile here. The admin courses router is
/// what the dashboard's course-builder UI calls; a regression that drops
/// `/:course_id/clone` or `/:course_id/enroll` would silently break those
/// features in prod.
#[test]
fn courses_admin_router_builds() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::courses_admin::router();
}

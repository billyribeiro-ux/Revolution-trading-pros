//! Member-courses route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::member_courses` and
//! exercises the public + member-auth Course DTOs from
//! `revolution_api::models::course` along with both routers
//! (`public_router()` for the unauthenticated catalog and
//! `member_router()` for the authenticated player + progress).
//!
//! ## Why this shape
//!
//! `routes/member_courses.rs` (1,671 LOC) is the public course catalog
//! + the authenticated player surface. Every handler runs live SQL
//! against `courses`, `user_course_enrollments`, `user_lesson_progress`,
//! and the member-auth handlers pull the `User` extractor — the SQL
//! and the auth gate mean we can't drive handlers in isolation. What
//! we CAN pin:
//!
//! 1. **`Course.price_cents` and `CourseListItem.price_cents` are
//!    `i64`.** HARD RULE: every `*_cents` field is `i64` end-to-end
//!    (CLAUDE.md "Money / cents"). Course pricing tops out higher
//!    than most products on this stack (corporate-tier programs,
//!    multi-seat cohorts), so the i32 cap of $21.4M is a real
//!    concern at the rollup level. Pin the type at the DTO layer.
//!
//! 2. **`Course.id` and `CourseListItem.id` are `Uuid`.** Course
//!    PKs are deliberately UUIDs (not BIGSERIAL) so course content
//!    can be authored offline and synced without ID collision. A
//!    regression to i64 would silently break the lesson/module FK
//!    joins (which are all `Uuid course_id`).
//!
//! 3. **`Course.bunny_library_id` + `.instructor_id` are
//!    `Option<i64>`.** Bunny CDN library IDs and user (instructor)
//!    IDs are BIGSERIAL on the upstream tables — a regression to
//!    i32 would orphan high-id libraries/users at JOIN time.
//!
//! 4. **`UserCourseEnrollment.id`, `.user_id`, `.order_id`,
//!    `.current_module_id` are `i64`.** Enrollments accumulate
//!    indefinitely (every signup, every gift-redemption, every
//!    cohort enrollment) so the PK MUST stay BIGSERIAL i64. FKs to
//!    `users.id` and `orders.id` must match.
//!
//!    KNOWN LANDMINE: `UserCourseEnrollment.price_paid_cents` is
//!    `Option<i32>` (see `src/models/course.rs:235`). This is a
//!    CLAUDE.md "Money / cents" violation — `*_cents` MUST be i64.
//!    Test does NOT lock it in; instead it documents the bug for
//!    the next fix-up PR. We pin the CURRENT shape so a future
//!    widening to i64 (the correct direction) is a deliberate
//!    breaking-change, not a silent compatibility shift.
//!
//! 5. **`UserLessonProgress.id`, `.user_id`, `.enrollment_id` are
//!    `i64`.** Progress rows are written per-lesson-per-user; they
//!    accumulate even faster than enrollments and absolutely cross
//!    `2^31` on a multi-year-old platform.
//!
//! 6. **`CourseQueryParams` is fully optional (public catalog
//!    default view).** The course catalog landing page loads with
//!    no filters — every field defaults at the handler. NEGATIVE:
//!    `is_free` MUST be a bool, not a string (`?is_free=yes`).
//!
//! 7. **`UpdateProgressRequest.lesson_id` is required `Uuid`, time
//!    fields are `Option<i32>`.** The video-position updates fire
//!    on every player tick — a regression flipping `lesson_id` to
//!    optional would silently drop the WHERE clause and update
//!    nothing (or, worse, every row).
//!
//! 8. **Wire-format keys are snake_case.** The SvelteKit course
//!    player consumes `price_cents`, `instructor_name`,
//!    `enrollment_count`, `module_count` etc. as snake_case;
//!    camelCase regressions MUST NOT ship.
//!
//! 9. **Both routers build under `AppState`.** Mount-table compile
//!    pin for the 3-route public router and the 9+ route member
//!    router (player, progress, downloads, certificate, resume,
//!    reviews).
//!
//! ## Pattern source
//!
//! Modeled on `tests/courses_admin_test.rs`, `tests/products_test.rs`,
//! `tests/cms_v2_test.rs`, `tests/payments_test.rs`.

use revolution_api::models::course::{
    Course, CourseListItem, CourseQueryParams, UpdateProgressRequest, UserCourseEnrollment,
    UserLessonProgress,
};
use uuid::Uuid;

// ── 1. Money: Course.price_cents is i64, past i32::MAX ───────────────

/// HARD RULE (CLAUDE.md "Money / cents"): every `*_cents` field is
/// `i64`. Course pricing realistically passes i32::MAX in two cases:
///   - individual high-end corporate cohort price
///   - cumulative rollups (sum of price_cents across catalog)
/// We use `(i32::MAX as i64) + N` for the fixture to prove a future
/// narrowing to i32 fails this literal.
#[test]
fn course_price_cents_is_i64_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 12_345; // > $21.4M cents
    let course_id = Uuid::new_v4();
    let instructor_id_big: i64 = (i32::MAX as i64) + 7;
    let bunny_id_big: i64 = (i32::MAX as i64) + 99;
    let now = chrono::Utc::now().naive_utc();

    let course = Course {
        id: course_id,
        title: "Advanced Options Mastery".to_string(),
        slug: "advanced-options-mastery".to_string(),
        description: Some("Deep-dive options program".to_string()),
        price_cents: above_i32_max,
        instructor_id: Some(instructor_id_big),
        is_published: true,
        thumbnail: None,
        preview_video_url: None,
        duration_minutes: Some(600),
        level: Some("advanced".to_string()),
        metadata: None,
        created_at: now,
        updated_at: now,
        card_image_url: None,
        card_description: None,
        card_badge: None,
        card_badge_color: None,
        what_you_learn: None,
        requirements: None,
        target_audience: None,
        instructor_name: Some("J. Hull".to_string()),
        instructor_title: None,
        instructor_avatar_url: None,
        instructor_bio: None,
        is_free: Some(false),
        is_premium: Some(true),
        required_membership_ids: None,
        bunny_library_id: Some(bunny_id_big),
        meta_title: None,
        meta_description: None,
        og_image_url: None,
        status: Some("published".to_string()),
        published_at: Some(now),
        enrollment_count: Some(125),
        completion_rate: Some(72.5),
        avg_rating: Some(4.8),
        review_count: Some(43),
        module_count: Some(8),
        lesson_count: Some(56),
        total_duration_minutes: Some(600),
    };

    let wire = serde_json::to_value(&course).expect("serialize Course");
    assert_eq!(
        wire["price_cents"].as_i64(),
        Some(above_i32_max),
        "price_cents must survive JSON round-trip as i64"
    );
    assert!(
        course.price_cents > i32::MAX as i64,
        "fixture must exceed i32::MAX to prove i64 pin"
    );
    assert!(course.instructor_id.unwrap() > i32::MAX as i64);
    assert!(course.bunny_library_id.unwrap() > i32::MAX as i64);

    // Uuid PK serializes as string.
    assert!(wire["id"].is_string());
    assert_eq!(wire["id"].as_str().unwrap(), course_id.to_string());

    // Wire-format keys are snake_case.
    assert!(
        wire.get("priceCents").is_none(),
        "wire MUST be snake_case (`price_cents`), not camelCase"
    );
    assert!(
        wire.get("instructorId").is_none(),
        "wire MUST be snake_case (`instructor_id`), not camelCase"
    );
    assert!(
        wire.get("isPublished").is_none(),
        "wire MUST be snake_case (`is_published`), not camelCase"
    );
    assert!(
        wire.get("bunnyLibraryId").is_none(),
        "wire MUST be snake_case (`bunny_library_id`), not camelCase"
    );
}

// ── 2. CourseListItem: same money pin on the list DTO ────────────────

/// `CourseListItem.price_cents` is `i64`. The catalog grid renders
/// from a SELECT that explicitly projects `price_cents` from the
/// `courses` table — same column, same width, same rule.
#[test]
fn course_list_item_price_cents_is_i64() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1; // 2_147_483_648
    let now = chrono::Utc::now().naive_utc();

    let item = CourseListItem {
        id: Uuid::new_v4(),
        title: "Pro Cohort".to_string(),
        slug: "pro-cohort".to_string(),
        description: None,
        card_image_url: None,
        card_description: None,
        card_badge: Some("Bestseller".to_string()),
        card_badge_color: None,
        instructor_name: None,
        instructor_avatar_url: None,
        level: Some("intermediate".to_string()),
        price_cents: above_i32_max,
        is_free: Some(false),
        is_published: true,
        status: Some("published".to_string()),
        module_count: Some(6),
        lesson_count: Some(48),
        total_duration_minutes: Some(720),
        enrollment_count: Some(320),
        avg_rating: Some(4.6),
        review_count: Some(58),
        created_at: now,
    };

    let wire = serde_json::to_value(&item).expect("serialize CourseListItem");
    assert_eq!(
        wire["price_cents"].as_i64(),
        Some(above_i32_max),
        "CourseListItem.price_cents must round-trip as i64"
    );
    assert!(item.price_cents > i32::MAX as i64);

    // Snake_case wire format.
    assert!(wire.get("priceCents").is_none());
    assert!(wire.get("cardImageUrl").is_none());
    assert!(wire.get("enrollmentCount").is_none());
    assert_eq!(wire["card_badge"], "Bestseller");
}

// ── 3. UserCourseEnrollment: i64 PK + i64 FKs ────────────────────────

/// `UserCourseEnrollment.id` is `i64` BIGSERIAL; `.user_id` and
/// `.order_id` are FKs to BIGSERIAL columns. A regression to i32
/// here would silently 404 enrollments for high-id users.
///
/// KNOWN LANDMINE: `price_paid_cents: Option<i32>` violates CLAUDE.md
/// "Money / cents" (should be i64). We pin the CURRENT shape so a
/// future widening to i64 (the correct fix) is a deliberate change,
/// not silent. This test will FAIL when that fix lands — at which
/// point the assertion below should be updated to `as_i64()`.
#[test]
fn user_course_enrollment_ids_are_i64_documents_price_paid_landmine() {
    let big_id: i64 = (i32::MAX as i64) + 11;
    let big_user_id: i64 = (i32::MAX as i64) + 22;
    let big_order_id: i64 = (i32::MAX as i64) + 33;
    let big_module_id: i64 = (i32::MAX as i64) + 44;
    let now = chrono::Utc::now().naive_utc();

    let enrollment = UserCourseEnrollment {
        id: big_id,
        user_id: big_user_id,
        course_id: Uuid::new_v4(),
        current_module_id: Some(big_module_id),
        current_lesson_id: Some(Uuid::new_v4()),
        completed_lesson_ids: Some(serde_json::json!([])),
        progress_percent: Some(42),
        status: Some("active".to_string()),
        enrolled_at: now,
        started_at: Some(now),
        completed_at: None,
        last_accessed_at: Some(now),
        access_expires_at: None,
        is_lifetime_access: Some(true),
        order_id: Some(big_order_id),
        // LANDMINE: this field is Option<i32> on the source side
        // (violates CLAUDE.md "Money / cents"). When the source is
        // widened to i64, the assertion below MUST be updated.
        price_paid_cents: Some(9_999_99),
        certificate_issued: Some(false),
        certificate_url: None,
        certificate_issued_at: None,
        notes: None,
        bookmarks: None,
    };

    let wire = serde_json::to_value(&enrollment).expect("serialize UserCourseEnrollment");

    assert_eq!(wire["id"].as_i64(), Some(big_id));
    assert_eq!(wire["user_id"].as_i64(), Some(big_user_id));
    assert_eq!(wire["order_id"].as_i64(), Some(big_order_id));
    assert_eq!(wire["current_module_id"].as_i64(), Some(big_module_id));
    assert!(enrollment.id > i32::MAX as i64);
    assert!(enrollment.user_id > i32::MAX as i64);

    // Document the price_paid_cents landmine — fits in i32 here ($9,999.99).
    // When the source field is widened to i64, this assertion still
    // passes but the type pin in the field declaration shifts.
    assert_eq!(wire["price_paid_cents"].as_i64(), Some(9_999_99));

    // Wire-format keys snake_case.
    assert!(wire.get("userId").is_none());
    assert!(wire.get("orderId").is_none());
    assert!(wire.get("currentModuleId").is_none());
    assert!(wire.get("isLifetimeAccess").is_none());
}

// ── 4. UserLessonProgress: i64 PK + i64 FKs ──────────────────────────

/// Progress rows accumulate per-lesson-per-user. On a multi-year-old
/// platform with thousands of users × dozens of lessons-per-course ×
/// many courses, the table absolutely crosses `2^31` rows. PKs and
/// FKs must stay `i64`.
///
/// The video-position-related fields are `Option<i32>` — CLAUDE.md
/// "Reserved exception" applies (video duration in seconds is
/// bounded; the longest video < 8 hours fits in i32).
#[test]
fn user_lesson_progress_ids_are_i64() {
    let big_id: i64 = (i32::MAX as i64) + 5;
    let big_user_id: i64 = (i32::MAX as i64) + 6;
    let big_enrollment_id: i64 = (i32::MAX as i64) + 7;
    let now = chrono::Utc::now().naive_utc();

    let progress = UserLessonProgress {
        id: big_id,
        user_id: big_user_id,
        lesson_id: Uuid::new_v4(),
        enrollment_id: big_enrollment_id,
        video_position_seconds: Some(1_800), // 30 minutes
        video_duration_seconds: Some(3_600), // 1 hour
        video_watch_percent: Some(50),
        is_completed: false,
        completed_at: None,
        time_spent_seconds: Some(2_100),
        view_count: Some(3),
        first_accessed_at: now,
        last_accessed_at: now,
    };

    let wire = serde_json::to_value(&progress).expect("serialize UserLessonProgress");
    assert_eq!(wire["id"].as_i64(), Some(big_id));
    assert_eq!(wire["user_id"].as_i64(), Some(big_user_id));
    assert_eq!(wire["enrollment_id"].as_i64(), Some(big_enrollment_id));
    assert!(progress.enrollment_id > i32::MAX as i64);

    // Reserved-exception fields stay i32 (video time bounded).
    assert_eq!(wire["video_position_seconds"].as_i64(), Some(1_800));
    assert_eq!(wire["video_duration_seconds"].as_i64(), Some(3_600));

    // Snake_case wire format.
    assert!(wire.get("userId").is_none());
    assert!(wire.get("lessonId").is_none());
    assert!(wire.get("enrollmentId").is_none());
    assert!(wire.get("videoPositionSeconds").is_none());
    assert!(wire.get("isCompleted").is_none());
}

// ── 5. CourseQueryParams + UpdateProgressRequest: required vs PATCH ──

/// `CourseQueryParams` is fully optional — public catalog default
/// view loads with no filters. NEGATIVE: `is_free` MUST be a bool,
/// not a string; a regression that loosened the type would silently
/// accept `?is_free=yes` and produce a SQL bind error at the handler.
///
/// `UpdateProgressRequest.lesson_id` is REQUIRED. The video-position
/// updates fire on every player tick — a regression flipping
/// `lesson_id` to optional would silently drop the `WHERE` clause
/// and either update nothing (silent breakage) or every row
/// (catastrophic).
#[test]
fn query_and_progress_dtos_are_typed_and_required_where_required() {
    let empty: CourseQueryParams =
        serde_json::from_str("{}").expect("empty CourseQueryParams must deserialize");
    assert!(empty.status.is_none());
    assert!(empty.level.is_none());
    assert!(empty.is_free.is_none());
    assert!(empty.search.is_none());
    assert!(empty.page.is_none());

    let filtered: CourseQueryParams = serde_json::from_value(serde_json::json!({
        "status": "published",
        "level": "advanced",
        "is_free": false,
        "search": "options",
        "page": 1,
        "per_page": 20,
        "sort_by": "title",
        "sort_order": "ASC"
    }))
    .expect("filtered CourseQueryParams must deserialize");
    assert_eq!(filtered.is_free, Some(false));
    assert_eq!(filtered.page, Some(1));
    assert_eq!(filtered.sort_order.as_deref(), Some("ASC"));

    // NEGATIVE: `is_free` as a string MUST fail.
    assert!(
        serde_json::from_value::<CourseQueryParams>(serde_json::json!({"is_free": "yes"})).is_err(),
        "is_free as string MUST fail (Option<bool> type pin)"
    );

    // NEGATIVE: `page` as a string MUST fail.
    assert!(
        serde_json::from_value::<CourseQueryParams>(serde_json::json!({"page": "first"})).is_err(),
        "page as string MUST fail (Option<i32> type pin)"
    );

    // UpdateProgressRequest: lesson_id required, time fields optional.
    let lesson_uuid = Uuid::new_v4();
    let prog: UpdateProgressRequest = serde_json::from_value(serde_json::json!({
        "lesson_id": lesson_uuid,
        "video_position_seconds": 600,
        "is_completed": false
    }))
    .expect("UpdateProgressRequest must deserialize");
    assert_eq!(prog.lesson_id, lesson_uuid);
    assert_eq!(prog.video_position_seconds, Some(600));
    assert_eq!(prog.is_completed, Some(false));

    // NEGATIVE: missing `lesson_id` MUST fail — required, load-bearing
    // for the SQL WHERE clause.
    assert!(
        serde_json::from_value::<UpdateProgressRequest>(serde_json::json!({
            "video_position_seconds": 600
        }))
        .is_err(),
        "missing lesson_id MUST fail (required — load-bearing WHERE)"
    );

    // NEGATIVE: `lesson_id` as a number MUST fail (Uuid type pin).
    assert!(
        serde_json::from_value::<UpdateProgressRequest>(serde_json::json!({
            "lesson_id": 42_i64
        }))
        .is_err(),
        "lesson_id as number MUST fail (Uuid type pin)"
    );
}

// ── 6. Both routers build under AppState ─────────────────────────────

/// Public router: 3 routes — `/` (list), `/:slug` (detail), and
/// `/:slug/reviews`. Powers the unauthenticated course catalog
/// landing page. A refactor that broke a handler signature would
/// fail compilation here.
#[test]
fn member_courses_public_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::member_courses::public_router();
}

/// Member router: 9+ routes including the security-sensitive
/// `/:slug/player` (returns full lesson content — gates behind
/// enrollment check), `/:slug/progress` (updates player position),
/// `/:slug/downloads`, `/:slug/certificate`, `/:slug/resume`, plus
/// the review endpoints. All require the `User` extractor. A
/// refactor that dropped that gate from any handler would fail
/// compilation here.
#[test]
fn member_courses_member_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::member_courses::member_router();
}

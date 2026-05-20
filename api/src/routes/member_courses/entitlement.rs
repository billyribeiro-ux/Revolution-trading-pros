//! Course entitlement (paywall) helpers — ICT 7 Grade
//!
//! Pure, DB-free predicates that decide whether a viewer is entitled to a
//! lesson's protected payload (Bunny GUIDs, downloads, etc.). Kept tiny and
//! exhaustively unit-tested so the P0-7 paywall cannot silently regress.

use crate::models::User;

/// Admin-role gate, matching the canonical role set used by
/// `crate::middleware::admin::AdminUser` so the player handler stays
/// consistent with the rest of the codebase. The player route uses the
/// plain `User` extractor (legitimate enrolled members are not admins),
/// so the admin check has to be derived inline from `user.role`.
pub(super) fn is_course_admin(user: &User) -> bool {
    let role = user.role.as_deref().unwrap_or("user");
    matches!(role, "admin" | "super_admin" | "super-admin" | "developer")
}

/// Pure entitlement predicate for a single lesson — no DB, no network.
///
/// Determines whether `bunny_video_guid` (and other protected lesson
/// fields) may be serialized for the caller. A viewer is entitled to a
/// lesson's protected video payload when ANY of the following holds:
///
/// * the caller is enrolled in the course, or
/// * the caller is an admin/super-admin/developer, or
/// * the course itself is free, or
/// * the lesson is individually marked free, or
/// * the lesson is a public preview.
///
/// This is intentionally the *only* place the rule is expressed so it can
/// be regression-tested without a database (Bunny CDN is down, no live
/// playback test is possible — structural correctness is the bar).
pub(super) fn is_lesson_entitled(
    course_is_free: bool,
    enrolled: bool,
    is_admin: bool,
    lesson_is_free: bool,
    lesson_is_preview: bool,
) -> bool {
    enrolled || is_admin || course_is_free || lesson_is_free || lesson_is_preview
}

// ═══════════════════════════════════════════════════════════════════════════════════
// TESTS — pure entitlement predicate (P0-7 paywall regression guard)
//
// No DB / no network: the Bunny GUID is an opaque string, so the
// authorization branch is fully reasoned structurally. These tests pin the
// truth table so the paywall cannot silently regress.
// ═══════════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::is_lesson_entitled;

    /// A non-enrolled, non-admin viewer on a PAID course whose lesson is
    /// neither free nor preview is the exact P0-7 leak vector — it must be
    /// denied the protected video payload.
    #[test]
    fn paid_course_anonymous_viewer_is_not_entitled() {
        assert!(!is_lesson_entitled(
            false, // course_is_free
            false, // enrolled
            false, // is_admin
            false, // lesson_is_free
            false, // lesson_is_preview
        ));
    }

    #[test]
    fn enrolled_user_is_entitled_even_on_paid_locked_lesson() {
        assert!(is_lesson_entitled(false, true, false, false, false));
    }

    #[test]
    fn admin_is_entitled_even_when_not_enrolled() {
        assert!(is_lesson_entitled(false, false, true, false, false));
    }

    #[test]
    fn free_course_is_entitled_for_anyone() {
        assert!(is_lesson_entitled(true, false, false, false, false));
    }

    #[test]
    fn free_lesson_on_paid_course_is_entitled() {
        assert!(is_lesson_entitled(false, false, false, true, false));
    }

    #[test]
    fn preview_lesson_on_paid_course_is_entitled() {
        assert!(is_lesson_entitled(false, false, false, false, true));
    }

    /// Enrolled + admin + free flags all true must still be entitled
    /// (no accidental inversion of the predicate).
    #[test]
    fn fully_privileged_viewer_is_entitled() {
        assert!(is_lesson_entitled(true, true, true, true, true));
    }
}

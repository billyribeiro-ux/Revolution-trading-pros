// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM SCHEDULES — HELPER FUNCTIONS
// `day_name`, `parse_time`, `parse_date` — verbatim from the monolithic
// `schedules.rs` (R28-B4 split). Used by both the public upcoming-events
// handler (day_name) and every admin write path (parse_time / parse_date).
// ═══════════════════════════════════════════════════════════════════════════

use chrono::{NaiveDate, NaiveTime};

pub(super) fn day_name(day: i32) -> String {
    match day {
        0 => "Sunday".to_string(),
        1 => "Monday".to_string(),
        2 => "Tuesday".to_string(),
        3 => "Wednesday".to_string(),
        4 => "Thursday".to_string(),
        5 => "Friday".to_string(),
        6 => "Saturday".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub(super) fn parse_time(time_str: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(time_str, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%H:%M:%S"))
        .map_err(|e| format!("Invalid time format: {e}"))
}

pub(super) fn parse_date(date_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| format!("Invalid date format: {e}"))
}

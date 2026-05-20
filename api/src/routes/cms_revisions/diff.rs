//! Pure-Rust diff computation helpers for CMS revisions.
//!
//! R27-B4 split (2026-05-20) — extracted verbatim from the original
//! `routes/cms_revisions.rs` (lines 704-1093). The in-mod `#[cfg(test)]`
//! tests previously living in `cms_revisions.rs` are preserved at the
//! bottom of this file because they directly exercise the private
//! `compute_diff` and `count_words` helpers defined here (R15-D ref).

use serde_json::Value as JsonValue;

use super::dtos::{BlockChange, DiffStats, FieldChange};

// ═══════════════════════════════════════════════════════════════════════════════════════
// DIFF COMPUTATION
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Compute comprehensive diff between two revision data snapshots
pub(super) fn compute_diff(from: &JsonValue, to: &JsonValue, include_blocks: bool) -> DiffStats {
    let mut field_changes = Vec::new();
    let mut block_changes = None;

    // Fields to compare for changes
    let simple_fields = [
        "title",
        "subtitle",
        "excerpt",
        "content",
        "slug",
        "meta_title",
        "meta_description",
        "canonical_url",
        "robots_directives",
        "template",
    ];

    let seo_fields = [
        "meta_title",
        "meta_description",
        "meta_keywords",
        "canonical_url",
        "robots_directives",
        "structured_data",
    ];

    let mut title_changed = false;
    let mut content_changed = false;
    let mut seo_changed = false;
    let mut custom_fields_changed = false;

    // Compare simple fields
    for field in &simple_fields {
        let from_val = from.get(field);
        let to_val = to.get(field);

        if from_val != to_val {
            let change_type = match (from_val, to_val) {
                (None, Some(_)) => "added",
                (Some(_), None) => "removed",
                _ => "modified",
            };

            field_changes.push(FieldChange {
                field: field.to_string(),
                from: from_val.cloned(),
                to: to_val.cloned(),
                change_type: change_type.to_string(),
            });

            if *field == "title" {
                title_changed = true;
            }
            if *field == "content" {
                content_changed = true;
            }
            if seo_fields.contains(field) {
                seo_changed = true;
            }
        }
    }

    // Check SEO fields separately
    for field in &seo_fields {
        if !simple_fields.contains(field) {
            let from_val = from.get(field);
            let to_val = to.get(field);
            if from_val != to_val {
                seo_changed = true;
                let change_type = match (from_val, to_val) {
                    (None, Some(_)) => "added",
                    (Some(_), None) => "removed",
                    _ => "modified",
                };
                field_changes.push(FieldChange {
                    field: field.to_string(),
                    from: from_val.cloned(),
                    to: to_val.cloned(),
                    change_type: change_type.to_string(),
                });
            }
        }
    }

    // Check custom_fields
    let from_custom = from.get("custom_fields");
    let to_custom = to.get("custom_fields");
    if from_custom != to_custom {
        custom_fields_changed = true;
        field_changes.push(FieldChange {
            field: "custom_fields".to_string(),
            from: from_custom.cloned(),
            to: to_custom.cloned(),
            change_type: "modified".to_string(),
        });
    }

    // Compute block-level changes
    let (blocks_added, blocks_removed, blocks_modified, blocks_reordered) = if include_blocks {
        let (added, removed, modified, reordered, changes) = compute_block_diff(from, to);
        block_changes = Some(changes);
        (added, removed, modified, reordered)
    } else {
        // Quick check without detailed diff
        let from_blocks = from.get("content_blocks").and_then(|v| v.as_array());
        let to_blocks = to.get("content_blocks").and_then(|v| v.as_array());

        match (from_blocks, to_blocks) {
            (Some(f), Some(t)) if f != t => {
                content_changed = true;
                // Rough estimate
                let diff = (t.len() as i32 - f.len() as i32).abs();
                if t.len() > f.len() {
                    (diff, 0, 0, 0)
                } else if t.len() < f.len() {
                    (0, diff, 0, 0)
                } else {
                    (0, 0, diff, 0)
                }
            }
            (None, Some(t)) => (t.len() as i32, 0, 0, 0),
            (Some(f), None) => (0, f.len() as i32, 0, 0),
            _ => (0, 0, 0, 0),
        }
    };

    if blocks_added > 0 || blocks_removed > 0 || blocks_modified > 0 {
        content_changed = true;
    }

    // Calculate word count difference
    let from_word_count = count_words(from);
    let to_word_count = count_words(to);
    let word_count_diff = to_word_count - from_word_count;

    DiffStats {
        title_changed,
        content_changed,
        seo_changed,
        custom_fields_changed,
        blocks_added,
        blocks_removed,
        blocks_modified,
        blocks_reordered,
        total_fields_changed: field_changes.len() as i32,
        word_count_diff,
        field_changes,
        block_changes,
    }
}

/// Compute detailed block-level diff
fn compute_block_diff(from: &JsonValue, to: &JsonValue) -> (i32, i32, i32, i32, Vec<BlockChange>) {
    let mut changes = Vec::new();
    let mut added = 0;
    let mut removed = 0;
    let mut modified = 0;
    let mut reordered = 0;

    let from_blocks = from
        .get("content_blocks")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let to_blocks = to
        .get("content_blocks")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    // Create maps by block ID for efficient lookup
    let from_map: std::collections::HashMap<String, (usize, &JsonValue)> = from_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            b.get("id")
                .and_then(|id| id.as_str())
                .map(|id| (id.to_string(), (i, b)))
        })
        .collect();

    let to_map: std::collections::HashMap<String, (usize, &JsonValue)> = to_blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            b.get("id")
                .and_then(|id| id.as_str())
                .map(|id| (id.to_string(), (i, b)))
        })
        .collect();

    // Find removed and modified blocks
    for (id, (old_pos, old_block)) in &from_map {
        let block_type = old_block
            .get("block_type")
            .and_then(|t| t.as_str())
            .unwrap_or("unknown")
            .to_string();

        if let Some((new_pos, new_block)) = to_map.get(id) {
            // Block exists in both - check for modifications or reordering
            if *old_block != *new_block {
                modified += 1;
                let field_changes = compute_block_field_changes(old_block, new_block);
                changes.push(BlockChange {
                    block_id: id.clone(),
                    block_type: block_type.clone(),
                    change_type: "modified".to_string(),
                    old_position: Some(*old_pos as i32),
                    new_position: Some(*new_pos as i32),
                    changes: if field_changes.is_empty() {
                        None
                    } else {
                        Some(field_changes)
                    },
                });
            } else if old_pos != new_pos {
                reordered += 1;
                changes.push(BlockChange {
                    block_id: id.clone(),
                    block_type,
                    change_type: "reordered".to_string(),
                    old_position: Some(*old_pos as i32),
                    new_position: Some(*new_pos as i32),
                    changes: None,
                });
            }
        } else {
            // Block was removed
            removed += 1;
            changes.push(BlockChange {
                block_id: id.clone(),
                block_type,
                change_type: "removed".to_string(),
                old_position: Some(*old_pos as i32),
                new_position: None,
                changes: None,
            });
        }
    }

    // Find added blocks
    for (id, (new_pos, new_block)) in &to_map {
        if !from_map.contains_key(id) {
            added += 1;
            let block_type = new_block
                .get("block_type")
                .and_then(|t| t.as_str())
                .unwrap_or("unknown")
                .to_string();

            changes.push(BlockChange {
                block_id: id.clone(),
                block_type,
                change_type: "added".to_string(),
                old_position: None,
                new_position: Some(*new_pos as i32),
                changes: None,
            });
        }
    }

    // Sort changes by position for consistent output
    changes.sort_by(|a, b| {
        let a_pos = a.new_position.or(a.old_position).unwrap_or(999);
        let b_pos = b.new_position.or(b.old_position).unwrap_or(999);
        a_pos.cmp(&b_pos)
    });

    (added, removed, modified, reordered, changes)
}

/// Compute field-level changes within a block
fn compute_block_field_changes(old: &JsonValue, new: &JsonValue) -> Vec<FieldChange> {
    let mut changes = Vec::new();

    // Get the data objects from blocks
    let old_data = old.get("data");
    let new_data = new.get("data");

    if old_data == new_data {
        return changes;
    }

    // If both are objects, compare fields
    if let (Some(old_obj), Some(new_obj)) = (
        old_data.and_then(|d| d.as_object()),
        new_data.and_then(|d| d.as_object()),
    ) {
        // Check all fields in old
        for (key, old_val) in old_obj {
            let new_val = new_obj.get(key);
            if new_val != Some(old_val) {
                changes.push(FieldChange {
                    field: key.clone(),
                    from: Some(old_val.clone()),
                    to: new_val.cloned(),
                    change_type: if new_val.is_none() {
                        "removed"
                    } else {
                        "modified"
                    }
                    .to_string(),
                });
            }
        }

        // Check for new fields
        for (key, new_val) in new_obj {
            if !old_obj.contains_key(key) {
                changes.push(FieldChange {
                    field: key.clone(),
                    from: None,
                    to: Some(new_val.clone()),
                    change_type: "added".to_string(),
                });
            }
        }
    } else {
        // Data changed type or one is null
        changes.push(FieldChange {
            field: "data".to_string(),
            from: old_data.cloned(),
            to: new_data.cloned(),
            change_type: "modified".to_string(),
        });
    }

    // Check settings
    let old_settings = old.get("settings");
    let new_settings = new.get("settings");
    if old_settings != new_settings {
        changes.push(FieldChange {
            field: "settings".to_string(),
            from: old_settings.cloned(),
            to: new_settings.cloned(),
            change_type: "modified".to_string(),
        });
    }

    changes
}

/// Count approximate words in content
fn count_words(data: &JsonValue) -> i32 {
    let mut count = 0;

    // Count words in title
    if let Some(title) = data.get("title").and_then(|t| t.as_str()) {
        count += title.split_whitespace().count() as i32;
    }

    // Count words in content
    if let Some(content) = data.get("content").and_then(|c| c.as_str()) {
        count += content.split_whitespace().count() as i32;
    }

    // Count words in excerpt
    if let Some(excerpt) = data.get("excerpt").and_then(|e| e.as_str()) {
        count += excerpt.split_whitespace().count() as i32;
    }

    // Count words in content blocks
    if let Some(blocks) = data.get("content_blocks").and_then(|b| b.as_array()) {
        for block in blocks {
            if let Some(block_data) = block.get("data") {
                count += count_words_in_json(block_data);
            }
        }
    }

    count
}

/// Recursively count words in JSON values
fn count_words_in_json(value: &JsonValue) -> i32 {
    match value {
        JsonValue::String(s) => s.split_whitespace().count() as i32,
        JsonValue::Array(arr) => arr.iter().map(count_words_in_json).sum(),
        JsonValue::Object(obj) => obj.values().map(count_words_in_json).sum(),
        _ => 0,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// IN-MOD TESTS (R15-D — preserved verbatim from `cms_revisions.rs:1153-1209`)
//
// These tests exercise the PRIVATE helpers `compute_diff` and
// `count_words` defined above. They are kept in this sub-file so the
// `use super::*` import resolves the private items directly (the same
// way the original `#[cfg(test)] mod tests` did inside the monolithic
// file). External public-API contracts are pinned separately in
// `tests/cms_revisions_test.rs` (R15-D).
// ═══════════════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compute_diff_title_change() {
        let from = json!({
            "title": "Old Title",
            "content": "Same content"
        });
        let to = json!({
            "title": "New Title",
            "content": "Same content"
        });

        let diff = compute_diff(&from, &to, false);
        assert!(diff.title_changed);
        assert!(!diff.content_changed);
        assert_eq!(diff.total_fields_changed, 1);
    }

    #[test]
    fn test_compute_diff_block_added() {
        let from = json!({
            "title": "Test",
            "content_blocks": [
                {"id": "block1", "block_type": "rich-text", "data": {"text": "Hello"}}
            ]
        });
        let to = json!({
            "title": "Test",
            "content_blocks": [
                {"id": "block1", "block_type": "rich-text", "data": {"text": "Hello"}},
                {"id": "block2", "block_type": "image", "data": {"src": "test.jpg"}}
            ]
        });

        let diff = compute_diff(&from, &to, true);
        assert!(diff.content_changed);
        assert_eq!(diff.blocks_added, 1);
        assert_eq!(diff.blocks_removed, 0);
    }

    #[test]
    fn test_count_words() {
        let data = json!({
            "title": "Hello World",
            "content": "This is a test content with some words",
            "content_blocks": [
                {"data": {"text": "Block text here"}}
            ]
        });

        let count = count_words(&data);
        assert!(count > 0);
    }
}

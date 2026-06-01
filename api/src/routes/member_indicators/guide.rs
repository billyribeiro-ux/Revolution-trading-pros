//! Update notifications + installation guides.
//!
//! - `check_updates` — paywall-gated, diff a user's owned indicators
//!   against the latest current-version file
//! - `get_installation_guide` — public, returns platform-specific install
//!   instructions (DB row or built-in fallback)
//! - `get_default_instructions` — built-in fallback copy

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::models::User;
use crate::AppState;

/// ICT 7: Check for indicator updates for owned indicators
pub(super) async fn check_updates(
    user: User,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // Get user's owned indicators with update info
    #[allow(clippy::type_complexity)]
    let updates: Vec<(
        i64,
        String,
        String,
        Option<String>,
        Option<String>,
        chrono::NaiveDateTime,
    )> = sqlx::query_as(
        r"
        SELECT
            i.id,
            i.name,
            i.slug,
            i.version,
            (SELECT f.version FROM indicator_files f
             WHERE f.indicator_id = i.id AND f.is_current_version = true
             LIMIT 1) as latest_file_version,
            COALESCE(i.updated_at, i.created_at) as last_updated
        FROM user_indicators ui
        JOIN indicators i ON ui.indicator_id = i.id
        WHERE ui.user_id = $1 AND i.is_active = true
        ORDER BY i.updated_at DESC NULLS LAST
        ",
    )
    .bind(user_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let updates_list: Vec<serde_json::Value> = updates
        .into_iter()
        .filter_map(|(id, name, slug, version, file_version, updated_at)| {
            // Check if there's a newer version available
            let has_update = version
                .as_ref()
                .and_then(|v| file_version.as_ref().map(|fv| fv != v))
                .unwrap_or(false);

            if has_update {
                Some(json!({
                    "indicator_id": id,
                    "name": name,
                    "slug": slug,
                    "current_version": version,
                    "latest_version": file_version,
                    "updated_at": updated_at.and_utc().to_rfc3339()
                }))
            } else {
                None
            }
        })
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "updates_available": !updates_list.is_empty(),
            "count": updates_list.len(),
            "updates": updates_list
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// INSTALLATION GUIDES (ICT 7 Grade - February 2026)
// ═══════════════════════════════════════════════════════════════════════════════════

/// ICT 7: Get installation guide for a specific platform
pub(super) async fn get_installation_guide(
    State(state): State<AppState>,
    Path((slug, platform)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get indicator
    let indicator: Option<(i64, Option<String>)> =
        sqlx::query_as("SELECT id, documentation_url FROM indicators WHERE slug = $1")
            .bind(&slug)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten();

    let (indicator_id, doc_url) = match indicator {
        Some(data) => data,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Indicator not found"})),
            ));
        }
    };

    // Get platform-specific installation guide from trading_platforms table
    let platform_guide: Option<(String, Option<String>, Option<String>)> = sqlx::query_as(
        r"
        SELECT name, install_instructions, documentation_url
        FROM trading_platforms
        WHERE LOWER(slug) = LOWER($1) OR LOWER(name) = LOWER($1)
        ",
    )
    .bind(&platform)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    // Get platform-specific files for this indicator
    let files: Vec<(String, Option<String>, Option<String>)> = sqlx::query_as(
        r"
        SELECT file_name, display_name, version
        FROM indicator_files
        WHERE indicator_id = $1 AND LOWER(platform) = LOWER($2) AND is_active = true
        ORDER BY is_current_version DESC, display_order
        ",
    )
    .bind(indicator_id)
    .bind(&platform)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let guide = match platform_guide {
        Some((name, instructions, platform_doc_url)) => {
            json!({
                "platform": name,
                "instructions": instructions.unwrap_or_else(|| get_default_instructions(&platform)),
                "documentation_url": platform_doc_url.or(doc_url),
                "files": files.into_iter().map(|(file_name, display_name, version)| {
                    json!({
                        "file_name": file_name,
                        "display_name": display_name,
                        "version": version
                    })
                }).collect::<Vec<_>>()
            })
        }
        None => {
            // Return default instructions for the platform
            json!({
                "platform": platform,
                "instructions": get_default_instructions(&platform),
                "documentation_url": doc_url,
                "files": files.into_iter().map(|(file_name, display_name, version)| {
                    json!({
                        "file_name": file_name,
                        "display_name": display_name,
                        "version": version
                    })
                }).collect::<Vec<_>>()
            })
        }
    };

    Ok(Json(json!({
        "success": true,
        "data": guide
    })))
}

/// Get default installation instructions for a platform
fn get_default_instructions(platform: &str) -> String {
    match platform.to_lowercase().as_str() {
        "thinkorswim" | "tos" => r"
## ThinkorSwim Installation

1. Download the indicator file (.ts extension)
2. Open ThinkorSwim and go to Setup > Open Shared Item
3. Paste the shareable link or import the .ts file
4. The indicator will appear in your Studies menu
5. Apply to any chart by right-clicking and selecting Studies > Add Study

**Tip:** Make sure to save the workspace after adding the indicator.
".to_string(),
        "tradingview" => r#"
## TradingView Installation

1. This indicator requires TradingView access
2. Go to your TradingView account
3. Open the Pine Script editor (at the bottom of your chart)
4. Click "Indicators" and search for the indicator name
5. If it's invite-only, ensure your TradingView username is registered

**Note:** TradingView indicators are cloud-based and don't require file downloads.
"#.to_string(),
        "metatrader" | "mt4" | "mt5" => r"
## MetaTrader Installation

1. Download the indicator file (.mq4/.mq5 or .ex4/.ex5)
2. Open MetaTrader and go to File > Open Data Folder
3. Navigate to MQL4/Indicators (or MQL5/Indicators)
4. Copy the downloaded file into this folder
5. Restart MetaTrader or refresh the Navigator panel
6. Find the indicator in Navigator > Indicators

**Tip:** Make sure to compile .mq4/.mq5 files if needed.
".to_string(),
        "ninjatrader" => r"
## NinjaTrader Installation

1. Download the indicator .zip file
2. Open NinjaTrader Control Center
3. Go to Tools > Import > NinjaScript Add-On
4. Select the downloaded .zip file
5. Follow the import wizard
6. The indicator will appear in your Indicators list

**Note:** Restart NinjaTrader if the indicator doesn't appear immediately.
".to_string(),
        "tradestation" => r"
## TradeStation Installation

1. Download the indicator .eld file
2. Open TradeStation
3. Go to File > Import/Export EasyLanguage
4. Select Import EasyLanguage file
5. Browse to the downloaded .eld file
6. Follow the import wizard

**Tip:** You may need to verify the indicator in the TradeStation scanner.
".to_string(),
        _ => format!(
            "## {platform} Installation\n\n1. Download the indicator file\n2. Follow the platform's standard import process\n3. Consult the platform's documentation for detailed steps\n\nContact support if you need assistance."
        ),
    }
}

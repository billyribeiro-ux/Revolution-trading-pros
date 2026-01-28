//! Unified Event Broadcasting Service - Apple Principal Engineer ICT Level 7+ Grade
//! ═══════════════════════════════════════════════════════════════════════════════════
//! January 2026
//!
//! This service provides a unified interface for broadcasting events to:
//! - WebSocket clients (real-time, bidirectional)
//! - SSE clients (real-time, unidirectional - for backwards compatibility)
//!
//! Design principles:
//! - Single source of truth for all real-time events
//! - Protocol-agnostic broadcasting interface
//! - Room-based subscription model
//! - Built for horizontal scaling (Redis pub/sub ready)
//!
//! Future extensibility:
//! - Redis pub/sub for multi-instance deployment
//! - Webhook delivery for external integrations
//! - Event persistence for replay/audit

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::routes::{
    realtime::EventBroadcaster as SseEventBroadcaster,
    websocket::{
        AlertPayload, StatsPayload, TradePayload, TradePlanPayload, VideoPayload,
        WsConnectionManager,
    },
};

// ═══════════════════════════════════════════════════════════════════════════════════
// UNIFIED EVENT TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

/// Unified event types for all real-time channels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "data")]
pub enum TradingRoomEvent {
    /// Alert events
    AlertCreated(AlertEventData),
    AlertUpdated(AlertEventData),
    AlertDeleted {
        room_slug: String,
        alert_id: i64,
    },

    /// Trade events
    TradeCreated(TradeEventData),
    TradeClosed(TradeEventData),
    TradeUpdated(TradeEventData),
    TradeInvalidated(TradeEventData),

    /// Stats events
    StatsUpdated(StatsEventData),

    /// Trade plan events
    TradePlanCreated(TradePlanEventData),
    TradePlanUpdated(TradePlanEventData),
    TradePlanDeleted {
        room_slug: String,
        entry_id: i64,
    },

    /// Video events
    VideoPublished(VideoEventData),
}

/// Alert event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEventData {
    pub id: i64,
    pub room_slug: String,
    pub alert_type: String,
    pub ticker: String,
    pub title: Option<String>,
    pub message: String,
    pub notes: Option<String>,
    pub tos_string: Option<String>,
    pub is_new: bool,
    pub is_pinned: bool,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Trade event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeEventData {
    pub id: i64,
    pub room_slug: String,
    pub ticker: String,
    pub direction: String,
    pub status: String,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub pnl_percent: Option<f64>,
    pub result: Option<String>,
    pub invalidation_reason: Option<String>,
    pub was_updated: Option<bool>,
    pub entry_date: String,
    pub exit_date: Option<String>,
}

/// Stats event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsEventData {
    pub room_slug: String,
    pub win_rate: Option<f64>,
    pub weekly_profit: Option<String>,
    pub active_trades: Option<i32>,
    pub closed_this_week: Option<i32>,
    pub total_trades: Option<i32>,
    pub current_streak: Option<i32>,
}

/// Trade plan event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePlanEventData {
    pub id: i64,
    pub room_slug: String,
    pub ticker: String,
    pub bias: String,
    pub entry: Option<String>,
    pub target1: Option<String>,
    pub target2: Option<String>,
    pub target3: Option<String>,
    pub runner: Option<String>,
    pub stop: Option<String>,
    pub options_strike: Option<String>,
    pub options_exp: Option<String>,
    pub notes: Option<String>,
}

/// Video event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoEventData {
    pub id: i64,
    pub room_slug: String,
    pub week_title: String,
    pub video_title: String,
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<String>,
    pub published_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// UNIFIED BROADCASTER
// ═══════════════════════════════════════════════════════════════════════════════════

/// Unified event broadcaster that sends events to both WebSocket and SSE clients
#[derive(Clone)]
pub struct UnifiedEventBroadcaster {
    /// WebSocket connection manager
    ws_manager: WsConnectionManager,

    /// SSE event broadcaster (for backwards compatibility)
    sse_broadcaster: SseEventBroadcaster,
}

impl UnifiedEventBroadcaster {
    /// Create a new unified event broadcaster
    pub fn new(ws_manager: WsConnectionManager, sse_broadcaster: SseEventBroadcaster) -> Self {
        Self {
            ws_manager,
            sse_broadcaster,
        }
    }

    /// Get the WebSocket connection manager
    pub fn ws_manager(&self) -> &WsConnectionManager {
        &self.ws_manager
    }

    /// Get the SSE broadcaster
    pub fn sse_broadcaster(&self) -> &SseEventBroadcaster {
        &self.sse_broadcaster
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // ALERT BROADCASTING
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Broadcast alert created event
    pub async fn broadcast_alert_created(&self, data: AlertEventData) {
        let room_slug = data.room_slug.clone();

        // Broadcast to WebSocket clients
        self.ws_manager
            .broadcast_alert_created(AlertPayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                alert_type: data.alert_type.clone(),
                ticker: data.ticker.clone(),
                title: data.title.clone(),
                message: data.message.clone(),
                notes: data.notes.clone(),
                tos_string: data.tos_string.clone(),
                is_new: data.is_new,
                is_pinned: data.is_pinned,
                published_at: data.published_at,
                created_at: data.created_at,
            })
            .await;

        // Broadcast to SSE clients (using CMS content type for backwards compatibility)
        crate::routes::realtime::emit_content_created(
            &self.sse_broadcaster,
            &format!("alert:{}", room_slug),
            data.id,
            data.title,
            0, // No user tracking in trading room alerts
        );

        tracing::info!(
            room_slug = %room_slug,
            alert_id = data.id,
            "Alert created event broadcast"
        );
    }

    /// Broadcast alert updated event
    pub async fn broadcast_alert_updated(&self, data: AlertEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_alert_updated(AlertPayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                alert_type: data.alert_type.clone(),
                ticker: data.ticker.clone(),
                title: data.title.clone(),
                message: data.message.clone(),
                notes: data.notes.clone(),
                tos_string: data.tos_string.clone(),
                is_new: data.is_new,
                is_pinned: data.is_pinned,
                published_at: data.published_at,
                created_at: data.created_at,
            })
            .await;

        crate::routes::realtime::emit_content_updated(
            &self.sse_broadcaster,
            &format!("alert:{}", room_slug),
            data.id,
            data.title,
            0,
            None,
        );

        tracing::info!(
            room_slug = %room_slug,
            alert_id = data.id,
            "Alert updated event broadcast"
        );
    }

    /// Broadcast alert deleted event
    pub async fn broadcast_alert_deleted(&self, room_slug: &str, alert_id: i64) {
        self.ws_manager
            .broadcast_alert_deleted(room_slug, alert_id)
            .await;

        crate::routes::realtime::emit_content_deleted(
            &self.sse_broadcaster,
            &format!("alert:{}", room_slug),
            alert_id,
            0,
        );

        tracing::info!(
            room_slug = %room_slug,
            alert_id = alert_id,
            "Alert deleted event broadcast"
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // TRADE BROADCASTING
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Broadcast trade created event
    pub async fn broadcast_trade_created(&self, data: TradeEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_trade_created(TradePayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                ticker: data.ticker.clone(),
                direction: data.direction.clone(),
                status: data.status.clone(),
                entry_price: data.entry_price,
                exit_price: data.exit_price,
                pnl_percent: data.pnl_percent,
                result: data.result.clone(),
                invalidation_reason: data.invalidation_reason.clone(),
                was_updated: data.was_updated,
                entry_date: data.entry_date.clone(),
                exit_date: data.exit_date.clone(),
            })
            .await;

        crate::routes::realtime::emit_content_created(
            &self.sse_broadcaster,
            &format!("trade:{}", room_slug),
            data.id,
            Some(format!("{} {}", data.ticker, data.direction)),
            0,
        );

        tracing::info!(
            room_slug = %room_slug,
            trade_id = data.id,
            ticker = %data.ticker,
            "Trade created event broadcast"
        );
    }

    /// Broadcast trade closed event
    pub async fn broadcast_trade_closed(&self, data: TradeEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_trade_closed(TradePayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                ticker: data.ticker.clone(),
                direction: data.direction.clone(),
                status: data.status.clone(),
                entry_price: data.entry_price,
                exit_price: data.exit_price,
                pnl_percent: data.pnl_percent,
                result: data.result.clone(),
                invalidation_reason: data.invalidation_reason.clone(),
                was_updated: data.was_updated,
                entry_date: data.entry_date.clone(),
                exit_date: data.exit_date.clone(),
            })
            .await;

        crate::routes::realtime::emit_content_published(
            &self.sse_broadcaster,
            &format!("trade:{}", room_slug),
            data.id,
            Some(format!(
                "{} {} - {}",
                data.ticker,
                data.result.as_deref().unwrap_or("CLOSED"),
                data.pnl_percent
                    .map(|p| format!("{:+.2}%", p))
                    .unwrap_or_default()
            )),
            0,
        );

        tracing::info!(
            room_slug = %room_slug,
            trade_id = data.id,
            ticker = %data.ticker,
            result = ?data.result,
            pnl_percent = ?data.pnl_percent,
            "Trade closed event broadcast"
        );
    }

    /// Broadcast trade updated event
    pub async fn broadcast_trade_updated(&self, data: TradeEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_trade_updated(TradePayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                ticker: data.ticker.clone(),
                direction: data.direction.clone(),
                status: data.status.clone(),
                entry_price: data.entry_price,
                exit_price: data.exit_price,
                pnl_percent: data.pnl_percent,
                result: data.result.clone(),
                invalidation_reason: data.invalidation_reason.clone(),
                was_updated: data.was_updated,
                entry_date: data.entry_date.clone(),
                exit_date: data.exit_date.clone(),
            })
            .await;

        crate::routes::realtime::emit_content_updated(
            &self.sse_broadcaster,
            &format!("trade:{}", room_slug),
            data.id,
            Some(format!("{} UPDATE", data.ticker)),
            0,
            Some(vec!["position".to_string()]),
        );

        tracing::info!(
            room_slug = %room_slug,
            trade_id = data.id,
            ticker = %data.ticker,
            "Trade updated event broadcast"
        );
    }

    /// Broadcast trade invalidated event
    pub async fn broadcast_trade_invalidated(&self, data: TradeEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_trade_invalidated(TradePayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                ticker: data.ticker.clone(),
                direction: data.direction.clone(),
                status: data.status.clone(),
                entry_price: data.entry_price,
                exit_price: data.exit_price,
                pnl_percent: data.pnl_percent,
                result: data.result.clone(),
                invalidation_reason: data.invalidation_reason.clone(),
                was_updated: data.was_updated,
                entry_date: data.entry_date.clone(),
                exit_date: data.exit_date.clone(),
            })
            .await;

        tracing::info!(
            room_slug = %room_slug,
            trade_id = data.id,
            ticker = %data.ticker,
            reason = ?data.invalidation_reason,
            "Trade invalidated event broadcast"
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // STATS BROADCASTING
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Broadcast stats updated event
    pub async fn broadcast_stats_updated(&self, data: StatsEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_stats_updated(StatsPayload {
                room_slug: data.room_slug.clone(),
                win_rate: data.win_rate,
                weekly_profit: data.weekly_profit.clone(),
                active_trades: data.active_trades,
                closed_this_week: data.closed_this_week,
                total_trades: data.total_trades,
                current_streak: data.current_streak,
            })
            .await;

        tracing::info!(
            room_slug = %room_slug,
            win_rate = ?data.win_rate,
            "Stats updated event broadcast"
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // TRADE PLAN BROADCASTING
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Broadcast trade plan created event
    pub async fn broadcast_trade_plan_created(&self, data: TradePlanEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_trade_plan_created(TradePlanPayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                ticker: data.ticker.clone(),
                bias: data.bias.clone(),
                entry: data.entry.clone(),
                target1: data.target1.clone(),
                target2: data.target2.clone(),
                target3: data.target3.clone(),
                runner: data.runner.clone(),
                stop: data.stop.clone(),
                options_strike: data.options_strike.clone(),
                options_exp: data.options_exp.clone(),
                notes: data.notes.clone(),
            })
            .await;

        crate::routes::realtime::emit_content_created(
            &self.sse_broadcaster,
            &format!("trade_plan:{}", room_slug),
            data.id,
            Some(format!("{} {}", data.ticker, data.bias)),
            0,
        );

        tracing::info!(
            room_slug = %room_slug,
            entry_id = data.id,
            ticker = %data.ticker,
            "Trade plan created event broadcast"
        );
    }

    /// Broadcast trade plan updated event
    pub async fn broadcast_trade_plan_updated(&self, data: TradePlanEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_trade_plan_updated(TradePlanPayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                ticker: data.ticker.clone(),
                bias: data.bias.clone(),
                entry: data.entry.clone(),
                target1: data.target1.clone(),
                target2: data.target2.clone(),
                target3: data.target3.clone(),
                runner: data.runner.clone(),
                stop: data.stop.clone(),
                options_strike: data.options_strike.clone(),
                options_exp: data.options_exp.clone(),
                notes: data.notes.clone(),
            })
            .await;

        crate::routes::realtime::emit_content_updated(
            &self.sse_broadcaster,
            &format!("trade_plan:{}", room_slug),
            data.id,
            Some(format!("{} {}", data.ticker, data.bias)),
            0,
            None,
        );

        tracing::info!(
            room_slug = %room_slug,
            entry_id = data.id,
            ticker = %data.ticker,
            "Trade plan updated event broadcast"
        );
    }

    /// Broadcast trade plan deleted event
    pub async fn broadcast_trade_plan_deleted(&self, room_slug: &str, entry_id: i64) {
        self.ws_manager
            .broadcast_trade_plan_deleted(room_slug, entry_id)
            .await;

        crate::routes::realtime::emit_content_deleted(
            &self.sse_broadcaster,
            &format!("trade_plan:{}", room_slug),
            entry_id,
            0,
        );

        tracing::info!(
            room_slug = %room_slug,
            entry_id = entry_id,
            "Trade plan deleted event broadcast"
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // VIDEO BROADCASTING
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Broadcast video published event
    pub async fn broadcast_video_published(&self, data: VideoEventData) {
        let room_slug = data.room_slug.clone();

        self.ws_manager
            .broadcast_video_published(VideoPayload {
                id: data.id,
                room_slug: data.room_slug.clone(),
                week_title: data.week_title.clone(),
                video_title: data.video_title.clone(),
                video_url: data.video_url.clone(),
                thumbnail_url: data.thumbnail_url.clone(),
                duration: data.duration.clone(),
                published_at: data.published_at,
            })
            .await;

        crate::routes::realtime::emit_content_published(
            &self.sse_broadcaster,
            &format!("video:{}", room_slug),
            data.id,
            Some(data.video_title.clone()),
            0,
        );

        tracing::info!(
            room_slug = %room_slug,
            video_id = data.id,
            title = %data.video_title,
            "Video published event broadcast"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Convert a room_content::RoomAlert to AlertEventData
impl From<&crate::routes::room_content::RoomAlert> for AlertEventData {
    fn from(alert: &crate::routes::room_content::RoomAlert) -> Self {
        Self {
            id: alert.id,
            room_slug: alert.room_slug.clone(),
            alert_type: alert.alert_type.clone(),
            ticker: alert.ticker.clone(),
            title: alert.title.clone(),
            message: alert.message.clone(),
            notes: alert.notes.clone(),
            tos_string: alert.tos_string.clone(),
            is_new: alert.is_new,
            is_pinned: alert.is_pinned,
            published_at: alert.published_at,
            created_at: alert.created_at,
        }
    }
}

/// Convert a room_content::RoomTrade to TradeEventData
impl From<&crate::routes::room_content::RoomTrade> for TradeEventData {
    fn from(trade: &crate::routes::room_content::RoomTrade) -> Self {
        Self {
            id: trade.id,
            room_slug: trade.room_slug.clone(),
            ticker: trade.ticker.clone(),
            direction: trade.direction.clone(),
            status: trade.status.clone(),
            entry_price: trade.entry_price,
            exit_price: trade.exit_price,
            pnl_percent: trade.pnl_percent,
            result: trade.result.clone(),
            invalidation_reason: trade.invalidation_reason.clone(),
            was_updated: trade.was_updated,
            entry_date: trade.entry_date.to_string(),
            exit_date: trade.exit_date.map(|d| d.to_string()),
        }
    }
}

/// Convert a room_content::TradePlanEntry to TradePlanEventData
impl From<&crate::routes::room_content::TradePlanEntry> for TradePlanEventData {
    fn from(entry: &crate::routes::room_content::TradePlanEntry) -> Self {
        Self {
            id: entry.id,
            room_slug: entry.room_slug.clone(),
            ticker: entry.ticker.clone(),
            bias: entry.bias.clone(),
            entry: entry.entry.clone(),
            target1: entry.target1.clone(),
            target2: entry.target2.clone(),
            target3: entry.target3.clone(),
            runner: entry.runner.clone(),
            stop: entry.stop.clone(),
            options_strike: entry.options_strike.clone(),
            options_exp: entry.options_exp.map(|d| d.to_string()),
            notes: entry.notes.clone(),
        }
    }
}

/// Convert a room_content::WeeklyVideo to VideoEventData
impl From<&crate::routes::room_content::WeeklyVideo> for VideoEventData {
    fn from(video: &crate::routes::room_content::WeeklyVideo) -> Self {
        Self {
            id: video.id,
            room_slug: video.room_slug.clone(),
            week_title: video.week_title.clone(),
            video_title: video.video_title.clone(),
            video_url: video.video_url.clone(),
            thumbnail_url: video.thumbnail_url.clone(),
            duration: video.duration.clone(),
            published_at: video.published_at,
        }
    }
}

/// Convert a room_content::RoomStats to StatsEventData
impl From<&crate::routes::room_content::RoomStats> for StatsEventData {
    fn from(stats: &crate::routes::room_content::RoomStats) -> Self {
        Self {
            room_slug: stats.room_slug.clone(),
            win_rate: stats.win_rate,
            weekly_profit: stats.weekly_profit.clone(),
            active_trades: stats.active_trades,
            closed_this_week: stats.closed_this_week,
            total_trades: stats.total_trades,
            current_streak: stats.current_streak,
        }
    }
}

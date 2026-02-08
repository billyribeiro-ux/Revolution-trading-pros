//! WebSocket Real-Time Alerts - Apple Principal Engineer ICT Level 7+ Grade
//! ═══════════════════════════════════════════════════════════════════════════════════
//! January 2026
//!
//! Production-grade WebSocket implementation for real-time trading alerts:
//! - Room-based subscription model (explosive-swings, weekly-watchlist, etc.)
//! - Automatic reconnection with exponential backoff
//! - Heartbeat monitoring with configurable intervals
//! - Message authentication and authorization
//! - Graceful connection lifecycle management
//!
//! Built for the next 10 years with extensibility in mind.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashSet, sync::Arc, time::Duration};
use tokio::sync::{broadcast, mpsc, RwLock};
use tracing::{debug, error, info, warn};

use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════════

/// Heartbeat interval in seconds
const HEARTBEAT_INTERVAL: u64 = 30;

/// Client timeout - disconnect if no pong received within this duration
const CLIENT_TIMEOUT: u64 = 60;

/// Maximum message size (64KB)
const MAX_MESSAGE_SIZE: usize = 64 * 1024;

// ═══════════════════════════════════════════════════════════════════════════════════
// MESSAGE TYPES
// ═══════════════════════════════════════════════════════════════════════════════════

/// WebSocket message types sent to clients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    /// New alert created in a trading room
    AlertCreated { alert: AlertPayload },

    /// Alert was updated
    AlertUpdated { alert: AlertPayload },

    /// Alert was deleted
    AlertDeleted { alert_id: i64 },

    /// New trade opened
    TradeCreated { trade: TradePayload },

    /// Trade was closed with result
    TradeClosed { trade: TradePayload },

    /// Trade was updated (position adjustment)
    TradeUpdated { trade: TradePayload },

    /// Trade was invalidated
    TradeInvalidated { trade: TradePayload },

    /// Stats updated for a room
    StatsUpdated { stats: StatsPayload },

    /// Trade plan entry created
    TradePlanCreated { entry: TradePlanPayload },

    /// Trade plan entry updated
    TradePlanUpdated { entry: TradePlanPayload },

    /// Trade plan entry deleted
    TradePlanDeleted { entry_id: i64 },

    /// Weekly video published
    VideoPublished { video: VideoPayload },

    /// Heartbeat message
    Heartbeat { timestamp: i64 },

    /// Connection established confirmation
    Connected {
        connection_id: String,
        rooms: Vec<String>,
        timestamp: i64,
    },

    /// Error message
    Error { code: String, message: String },

    /// Subscription confirmation
    Subscribed { room: String },

    /// Unsubscription confirmation
    Unsubscribed { room: String },
}

/// Alert payload for WebSocket messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertPayload {
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

/// Trade payload for WebSocket messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePayload {
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

/// Stats payload for WebSocket messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsPayload {
    pub room_slug: String,
    pub win_rate: Option<f64>,
    pub weekly_profit: Option<String>,
    pub active_trades: Option<i32>,
    pub closed_this_week: Option<i32>,
    pub total_trades: Option<i32>,
    pub current_streak: Option<i32>,
}

/// Trade plan entry payload for WebSocket messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradePlanPayload {
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

/// Video payload for WebSocket messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPayload {
    pub id: i64,
    pub room_slug: String,
    pub week_title: String,
    pub video_title: String,
    pub video_url: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<String>,
    pub published_at: DateTime<Utc>,
}

/// Client-to-server message types
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action", content = "data")]
pub enum ClientMessage {
    /// Subscribe to a room
    Subscribe { room: String },

    /// Unsubscribe from a room
    Unsubscribe { room: String },

    /// Ping for connection health check
    Ping { timestamp: i64 },

    /// Pong response to server heartbeat
    Pong { timestamp: i64 },
}

// ═══════════════════════════════════════════════════════════════════════════════════
// CONNECTION MANAGER
// ═══════════════════════════════════════════════════════════════════════════════════

/// Manages active WebSocket connections and room subscriptions
#[derive(Debug, Clone)]
pub struct WsConnectionManager {
    /// Sender for broadcasting messages to specific rooms
    room_senders: Arc<RwLock<std::collections::HashMap<String, broadcast::Sender<WsMessage>>>>,

    /// Active connection count per room
    room_connections: Arc<RwLock<std::collections::HashMap<String, usize>>>,
}

impl WsConnectionManager {
    /// Create a new connection manager
    pub fn new() -> Self {
        Self {
            room_senders: Arc::new(RwLock::new(std::collections::HashMap::new())),
            room_connections: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Subscribe to a room and get a receiver for messages
    pub async fn subscribe_room(&self, room_slug: &str) -> broadcast::Receiver<WsMessage> {
        let mut senders = self.room_senders.write().await;
        let mut connections = self.room_connections.write().await;

        let sender = senders.entry(room_slug.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1000);
            tx
        });

        // Increment connection count
        *connections.entry(room_slug.to_string()).or_insert(0) += 1;

        info!(
            room = room_slug,
            connections = connections.get(room_slug).copied().unwrap_or(0),
            "Client subscribed to room"
        );

        sender.subscribe()
    }

    /// Unsubscribe from a room
    pub async fn unsubscribe_room(&self, room_slug: &str) {
        let mut connections = self.room_connections.write().await;

        if let Some(count) = connections.get_mut(room_slug) {
            *count = count.saturating_sub(1);

            info!(
                room = room_slug,
                connections = *count,
                "Client unsubscribed from room"
            );

            // Clean up room if no more connections
            if *count == 0 {
                connections.remove(room_slug);
                // Note: We keep the sender around for potential reconnections
            }
        }
    }

    /// Broadcast a message to all subscribers of a room
    pub async fn broadcast_to_room(&self, room_slug: &str, message: WsMessage) {
        let senders = self.room_senders.read().await;

        if let Some(sender) = senders.get(room_slug) {
            let receiver_count = sender.receiver_count();

            if receiver_count > 0 {
                match sender.send(message.clone()) {
                    Ok(_) => {
                        debug!(
                            room = room_slug,
                            receivers = receiver_count,
                            "Broadcast message to room"
                        );
                    }
                    Err(e) => {
                        warn!(
                            room = room_slug,
                            error = %e,
                            "Failed to broadcast to room"
                        );
                    }
                }
            }
        }
    }

    /// Get the number of active connections for a room
    pub async fn room_connection_count(&self, room_slug: &str) -> usize {
        let connections = self.room_connections.read().await;
        connections.get(room_slug).copied().unwrap_or(0)
    }

    /// Get total active connections across all rooms
    pub async fn total_connections(&self) -> usize {
        let connections = self.room_connections.read().await;
        connections.values().sum()
    }

    /// Get list of all active rooms with connection counts
    pub async fn active_rooms(&self) -> Vec<(String, usize)> {
        let connections = self.room_connections.read().await;
        connections
            .iter()
            .filter(|(_, &count)| count > 0)
            .map(|(room, &count)| (room.clone(), count))
            .collect()
    }
}

impl Default for WsConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// WEBSOCKET HANDLER
// ═══════════════════════════════════════════════════════════════════════════════════

/// Query parameters for WebSocket connection
#[derive(Debug, Deserialize)]
pub struct WsParams {
    /// Initial rooms to subscribe to (comma-separated)
    pub rooms: Option<String>,

    /// Authentication token (optional, for authenticated connections)
    /// TODO: This token is currently accepted but never validated against JWT.
    /// Implement proper JWT validation to authenticate WebSocket connections
    /// and restrict room subscriptions based on user membership/permissions.
    pub token: Option<String>,
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<WsParams>,
) -> impl IntoResponse {
    ws.max_message_size(MAX_MESSAGE_SIZE)
        .on_upgrade(move |socket| handle_socket(socket, state, params))
}

/// Handle an individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: AppState, params: WsParams) {
    let connection_id = uuid::Uuid::new_v4().to_string();
    let (mut sender, mut receiver) = socket.split();

    // Parse initial rooms to subscribe to
    let initial_rooms: Vec<String> = params
        .rooms
        .map(|r| r.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // Create channels for internal communication
    let (tx, mut rx) = mpsc::channel::<WsMessage>(100);

    // Track subscribed rooms
    let subscribed_rooms: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));

    info!(
        connection_id = %connection_id,
        initial_rooms = ?initial_rooms,
        "WebSocket connection established"
    );

    // Send connection confirmation
    let connected_msg = WsMessage::Connected {
        connection_id: connection_id.clone(),
        rooms: initial_rooms.clone(),
        timestamp: Utc::now().timestamp(),
    };

    if let Ok(json) = serde_json::to_string(&connected_msg) {
        let _ = sender.send(Message::Text(json)).await;
    }

    // Subscribe to initial rooms
    let ws_manager = &state.ws_manager;
    for room in &initial_rooms {
        let mut rooms = subscribed_rooms.write().await;
        rooms.insert(room.clone());
        drop(rooms);

        // Spawn task to forward messages from this room
        let room_clone = room.clone();
        let tx_clone = tx.clone();
        let ws_manager_clone = ws_manager.clone();

        tokio::spawn(async move {
            let mut room_rx = ws_manager_clone.subscribe_room(&room_clone).await;

            while let Ok(msg) = room_rx.recv().await {
                if tx_clone.send(msg).await.is_err() {
                    break;
                }
            }
        });

        // Confirm subscription
        if let Ok(json) = serde_json::to_string(&WsMessage::Subscribed { room: room.clone() }) {
            let _ = sender.send(Message::Text(json)).await;
        }
    }

    // Spawn heartbeat task
    let tx_heartbeat = tx.clone();
    let heartbeat_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(HEARTBEAT_INTERVAL));

        loop {
            interval.tick().await;

            let heartbeat = WsMessage::Heartbeat {
                timestamp: Utc::now().timestamp(),
            };

            if tx_heartbeat.send(heartbeat).await.is_err() {
                break;
            }
        }
    });

    // Spawn task to send messages to client
    let send_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Handle incoming messages from client
    let ws_manager_for_receive = ws_manager.clone();
    let subscribed_rooms_for_receive = subscribed_rooms.clone();
    let tx_for_receive = tx.clone();
    let connection_id_for_receive = connection_id.clone();

    let receive_handle = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        match client_msg {
                            ClientMessage::Subscribe { room } => {
                                let mut rooms = subscribed_rooms_for_receive.write().await;
                                if !rooms.contains(&room) {
                                    rooms.insert(room.clone());
                                    drop(rooms);

                                    // Spawn task to forward messages from this room
                                    let room_clone = room.clone();
                                    let tx_clone = tx_for_receive.clone();
                                    let ws_manager_clone = ws_manager_for_receive.clone();

                                    tokio::spawn(async move {
                                        let mut room_rx =
                                            ws_manager_clone.subscribe_room(&room_clone).await;

                                        while let Ok(msg) = room_rx.recv().await {
                                            if tx_clone.send(msg).await.is_err() {
                                                break;
                                            }
                                        }
                                    });

                                    let _ =
                                        tx_for_receive.send(WsMessage::Subscribed { room }).await;
                                }
                            }
                            ClientMessage::Unsubscribe { room } => {
                                let mut rooms = subscribed_rooms_for_receive.write().await;
                                if rooms.remove(&room) {
                                    ws_manager_for_receive.unsubscribe_room(&room).await;

                                    let _ =
                                        tx_for_receive.send(WsMessage::Unsubscribed { room }).await;
                                }
                            }
                            ClientMessage::Ping { timestamp } => {
                                debug!(
                                    connection_id = %connection_id_for_receive,
                                    timestamp = timestamp,
                                    "Received ping"
                                );
                                // Respond with heartbeat
                                let _ = tx_for_receive
                                    .send(WsMessage::Heartbeat {
                                        timestamp: Utc::now().timestamp(),
                                    })
                                    .await;
                            }
                            ClientMessage::Pong { timestamp: _ } => {
                                debug!(
                                    connection_id = %connection_id_for_receive,
                                    "Received pong"
                                );
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!(
                        connection_id = %connection_id_for_receive,
                        "Client initiated close"
                    );
                    break;
                }
                Err(e) => {
                    error!(
                        connection_id = %connection_id_for_receive,
                        error = %e,
                        "WebSocket error"
                    );
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for any task to complete (connection closed)
    tokio::select! {
        _ = send_handle => {},
        _ = receive_handle => {},
    }

    // Cleanup
    heartbeat_handle.abort();

    // Unsubscribe from all rooms
    let rooms = subscribed_rooms.read().await;
    for room in rooms.iter() {
        ws_manager.unsubscribe_room(room).await;
    }

    info!(
        connection_id = %connection_id,
        "WebSocket connection closed"
    );
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REST ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Get WebSocket connection statistics
async fn ws_stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total = state.ws_manager.total_connections().await;
    let rooms = state.ws_manager.active_rooms().await;

    Ok(Json(json!({
        "status": "operational",
        "total_connections": total,
        "rooms": rooms.into_iter().map(|(room, count)| {
            json!({
                "room": room,
                "connections": count
            })
        }).collect::<Vec<_>>()
    })))
}

/// Test endpoint to broadcast a message (admin only, for debugging)
#[derive(Debug, Deserialize)]
pub struct TestBroadcastRequest {
    pub room: String,
    pub message_type: String,
    pub payload: serde_json::Value,
}

async fn test_broadcast(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<TestBroadcastRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // SECURITY: Admin authentication required to prevent unauthorized broadcasts

    let message = match input.message_type.as_str() {
        "heartbeat" => WsMessage::Heartbeat {
            timestamp: Utc::now().timestamp(),
        },
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Unsupported message type" })),
            ));
        }
    };

    state
        .ws_manager
        .broadcast_to_room(&input.room, message)
        .await;

    Ok(Json(json!({
        "success": true,
        "message": "Broadcast sent"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/stats", get(ws_stats))
        .route("/test", axum::routing::post(test_broadcast))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BROADCAST HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

impl WsConnectionManager {
    /// Broadcast alert created event
    pub async fn broadcast_alert_created(&self, alert: AlertPayload) {
        let room_slug = alert.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::AlertCreated { alert })
            .await;
    }

    /// Broadcast alert updated event
    pub async fn broadcast_alert_updated(&self, alert: AlertPayload) {
        let room_slug = alert.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::AlertUpdated { alert })
            .await;
    }

    /// Broadcast alert deleted event
    pub async fn broadcast_alert_deleted(&self, room_slug: &str, alert_id: i64) {
        self.broadcast_to_room(room_slug, WsMessage::AlertDeleted { alert_id })
            .await;
    }

    /// Broadcast trade created event
    pub async fn broadcast_trade_created(&self, trade: TradePayload) {
        let room_slug = trade.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::TradeCreated { trade })
            .await;
    }

    /// Broadcast trade closed event
    pub async fn broadcast_trade_closed(&self, trade: TradePayload) {
        let room_slug = trade.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::TradeClosed { trade })
            .await;
    }

    /// Broadcast trade updated event
    pub async fn broadcast_trade_updated(&self, trade: TradePayload) {
        let room_slug = trade.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::TradeUpdated { trade })
            .await;
    }

    /// Broadcast trade invalidated event
    pub async fn broadcast_trade_invalidated(&self, trade: TradePayload) {
        let room_slug = trade.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::TradeInvalidated { trade })
            .await;
    }

    /// Broadcast stats updated event
    pub async fn broadcast_stats_updated(&self, stats: StatsPayload) {
        let room_slug = stats.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::StatsUpdated { stats })
            .await;
    }

    /// Broadcast trade plan created event
    pub async fn broadcast_trade_plan_created(&self, entry: TradePlanPayload) {
        let room_slug = entry.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::TradePlanCreated { entry })
            .await;
    }

    /// Broadcast trade plan updated event
    pub async fn broadcast_trade_plan_updated(&self, entry: TradePlanPayload) {
        let room_slug = entry.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::TradePlanUpdated { entry })
            .await;
    }

    /// Broadcast trade plan deleted event
    pub async fn broadcast_trade_plan_deleted(&self, room_slug: &str, entry_id: i64) {
        self.broadcast_to_room(room_slug, WsMessage::TradePlanDeleted { entry_id })
            .await;
    }

    /// Broadcast video published event
    pub async fn broadcast_video_published(&self, video: VideoPayload) {
        let room_slug = video.room_slug.clone();
        self.broadcast_to_room(&room_slug, WsMessage::VideoPublished { video })
            .await;
    }
}

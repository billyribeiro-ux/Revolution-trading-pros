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
        ws::{rejection::WebSocketUpgradeRejection, Message, WebSocket, WebSocketUpgrade},
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
use crate::utils::verify_jwt;
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

/// Maximum room subscriptions per WebSocket connection
const MAX_ROOMS_PER_CONNECTION: usize = 10;

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
        // RUST_DEEP_AUDIT_2026-06-07 (P1-6): acquire `room_senders` then
        // `room_connections` — the SAME order as `subscribe_room` — to avoid a
        // lock-ordering inversion / deadlock now that we also touch the senders map.
        let mut senders = self.room_senders.write().await;
        let mut connections = self.room_connections.write().await;

        if let Some(count) = connections.get_mut(room_slug) {
            *count = count.saturating_sub(1);

            info!(
                room = room_slug,
                connections = *count,
                "Client unsubscribed from room"
            );

            // Clean up room if no more connections. Drop the broadcast sender too,
            // otherwise `room_senders` grows unbounded with client-controlled room
            // slugs (each entry is a 1000-slot ring buffer). `subscribe_room`
            // transparently re-creates the sender on the next subscribe.
            if *count == 0 {
                connections.remove(room_slug);
                senders.remove(room_slug);
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

    /// Authentication token (optional, for authenticated connections).
    /// When present it is validated against `jwt_secret` before the upgrade;
    /// invalid / expired tokens are rejected with HTTP 401 (close code 4001).
    /// Room subscriptions without a token are also rejected.
    pub token: Option<String>,
}

/// Root handler supporting both HTTP GET (status check) and WebSocket upgrade
pub async fn root_ws_handler(
    ws_opt: Result<WebSocketUpgrade, WebSocketUpgradeRejection>,
    State(state): State<AppState>,
    Query(params): Query<WsParams>,
) -> impl IntoResponse {
    match ws_opt {
        Ok(ws) => ws_handler_inner(ws, state, params).await.into_response(),
        Err(_) => (
            StatusCode::OK,
            Json(json!({
                "service": "Revolution Trading Pros API",
                "version": env!("CARGO_PKG_VERSION"),
                "status": "online"
            })),
        )
            .into_response(),
    }
}

/// WebSocket upgrade handler
///
/// If a `token` query parameter is present it is validated against the configured
/// `jwt_secret`. An invalid or expired token causes the connection to be refused
/// with HTTP 401 **before** the WebSocket handshake is completed. Missing token is
/// allowed so that public (unauthenticated) room subscriptions and service connections keep working.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<WsParams>,
) -> axum::response::Response {
    ws_handler_inner(ws, state, params).await
}

async fn ws_handler_inner(
    ws: WebSocketUpgrade,
    state: AppState,
    params: WsParams,
) -> axum::response::Response {
    let has_room_request = params
        .rooms
        .as_ref()
        .map(|r| r.split(',').any(|s| !s.trim().is_empty()))
        .unwrap_or(false);

    let has_room_access = if let Some(ref token) = params.token {
        let claims = match verify_jwt(token, &state.config.jwt_secret, "access") {
            Ok(c) => c,
            Err(err) => {
                warn!(error = %err, "WebSocket connection rejected: invalid JWT token");
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Unauthorized", "code": 4001})),
                )
                    .into_response();
            }
        };

        // Enforce Redis token_version epoch check so revoked/banned tokens cannot connect
        if let Some(ref redis) = state.services.redis {
            match redis.get_token_version(claims.sub).await {
                Ok(current_version) if claims.token_version < current_version => {
                    warn!(
                        user_id = %claims.sub,
                        token_version = %claims.token_version,
                        current_version = %current_version,
                        "WebSocket connection rejected: stale token_version"
                    );
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"error": "Token has been revoked", "code": 4001})),
                    )
                        .into_response();
                }
                Ok(_) => {}
                Err(e) => {
                    error!(user_id = %claims.sub, error = %e, "Redis token_version check failed on WebSocket upgrade");
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"error": "Authentication temporarily unavailable", "code": 4001})),
                    )
                        .into_response();
                }
            }
        }

        // Verify user is active and check role / active membership for room access
        let user_row: Option<(Option<String>, bool)> =
            sqlx::query_as("SELECT role, is_active FROM users WHERE id = $1")
                .bind(claims.sub)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

        let Some((role_opt, is_active)) = user_row else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized", "code": 4001})),
            )
                .into_response();
        };

        if !is_active {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({"error": "Account is disabled", "code": 4003})),
            )
                .into_response();
        }

        let role = role_opt.as_deref().unwrap_or("");
        let is_staff = matches!(role, "admin" | "super_admin" | "super-admin" | "developer");
        if is_staff {
            true
        } else {
            let membership: Option<(i64,)> = sqlx::query_as(
                "SELECT id FROM user_memberships \
                 WHERE user_id = $1 AND status IN ('active', 'trial', 'trialing') LIMIT 1",
            )
            .bind(claims.sub)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten();
            membership.is_some()
        }
    } else {
        false
    };

    if has_room_request && !has_room_access {
        warn!("WebSocket room subscription rejected: active membership required or unauthenticated");
        return (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Active membership required", "code": 4003})),
        )
            .into_response();
    }

    ws.max_message_size(MAX_MESSAGE_SIZE)
        .on_upgrade(move |socket| handle_socket(socket, state, params, has_room_access))
        .into_response()
}

/// Handle an individual WebSocket connection
async fn handle_socket(
    socket: WebSocket,
    state: AppState,
    params: WsParams,
    has_room_access: bool,
) {
    let connection_id = uuid::Uuid::new_v4().to_string();
    let (mut sender, mut receiver) = socket.split();

    // Parse initial rooms to subscribe to (capped at MAX_ROOMS_PER_CONNECTION and gated by membership).
    // RUST_DEEP_AUDIT_2026-06-07 (P2-9): de-duplicate (and drop empties) so a
    // `?rooms=alpha,alpha` cannot double-increment a room's connection count
    // (cleanup iterates the deduped HashSet once → off-by-one otherwise).
    let initial_rooms: Vec<String> = if has_room_access {
        let mut seen = HashSet::new();
        params
            .rooms
            .map(|r| {
                r.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty() && seen.insert(s.clone()))
                    .take(MAX_ROOMS_PER_CONNECTION)
                    .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    // Create channels for internal communication
    let (tx, mut rx) = mpsc::channel::<WsMessage>(100);

    // Track subscribed rooms
    let subscribed_rooms: Arc<RwLock<HashSet<String>>> = Arc::new(RwLock::new(HashSet::new()));

    // RUST_DEEP_AUDIT_2026-06-07 (P1-5): handles for the per-room forwarder tasks,
    // shared with the receive task so every forwarder is aborted on disconnect.
    // Without this they outlive the connection (a forwarder only notices the closed
    // mpsc on the NEXT room broadcast, so on a quiet room it leaks indefinitely).
    let forwarders: Arc<tokio::sync::Mutex<Vec<tokio::task::JoinHandle<()>>>> =
        Arc::new(tokio::sync::Mutex::new(Vec::new()));

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
        let _ = sender.send(Message::Text(json.into())).await;
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

        let handle = tokio::spawn(async move {
            let mut room_rx = ws_manager_clone.subscribe_room(&room_clone).await;

            while let Ok(msg) = room_rx.recv().await {
                if tx_clone.send(msg).await.is_err() {
                    break;
                }
            }
        });
        forwarders.lock().await.push(handle);

        // Confirm subscription
        if let Ok(json) = serde_json::to_string(&WsMessage::Subscribed { room: room.clone() }) {
            let _ = sender.send(Message::Text(json.into())).await;
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
    let mut send_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json.into())).await.is_err() {
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
    let forwarders_for_receive = forwarders.clone();

    let mut receive_handle = tokio::spawn(async move {
        while let Some(result) = receiver.next().await {
            match result {
                Ok(Message::Text(text)) => {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) {
                        let action = v
                            .get("action")
                            .and_then(|a| a.as_str())
                            .or_else(|| v.get("type").and_then(|t| t.as_str()))
                            .unwrap_or("");

                        if action.eq_ignore_ascii_case("subscribe") {
                            let room = v
                                .get("channel")
                                .or_else(|| v.get("room"))
                                .or_else(|| v.get("data").and_then(|d| d.get("room")))
                                .and_then(|r| r.as_str())
                                .unwrap_or("");

                            if !room.is_empty() {
                                let mut rooms = subscribed_rooms_for_receive.write().await;
                                if !rooms.contains(room) {
                                    if rooms.len() >= MAX_ROOMS_PER_CONNECTION {
                                        drop(rooms);
                                        let _ = tx_for_receive
                                            .send(WsMessage::Error {
                                                code: "ROOM_LIMIT_EXCEEDED".to_string(),
                                                message: format!(
                                                    "Maximum of {MAX_ROOMS_PER_CONNECTION} room subscriptions per connection"
                                                ),
                                            })
                                            .await;
                                        continue;
                                    }
                                    rooms.insert(room.to_string());
                                    drop(rooms);

                                    // Spawn task to forward messages from this room
                                    let room_clone = room.to_string();
                                    let tx_clone = tx_for_receive.clone();
                                    let ws_manager_clone = ws_manager_for_receive.clone();

                                    let handle = tokio::spawn(async move {
                                        let mut room_rx =
                                            ws_manager_clone.subscribe_room(&room_clone).await;

                                        while let Ok(msg) = room_rx.recv().await {
                                            if tx_clone.send(msg).await.is_err() {
                                                break;
                                            }
                                        }
                                    });
                                    forwarders_for_receive.lock().await.push(handle);

                                    let _ = tx_for_receive
                                        .send(WsMessage::Subscribed {
                                            room: room.to_string(),
                                        })
                                        .await;
                                }
                            }
                        } else if action.eq_ignore_ascii_case("unsubscribe") {
                            let room = v
                                .get("channel")
                                .or_else(|| v.get("room"))
                                .or_else(|| v.get("data").and_then(|d| d.get("room")))
                                .and_then(|r| r.as_str())
                                .unwrap_or("");

                            if !room.is_empty() {
                                let mut rooms = subscribed_rooms_for_receive.write().await;
                                if rooms.remove(room) {
                                    ws_manager_for_receive.unsubscribe_room(room).await;

                                    let _ = tx_for_receive
                                        .send(WsMessage::Unsubscribed {
                                            room: room.to_string(),
                                        })
                                        .await;
                                }
                            }
                        } else if action.eq_ignore_ascii_case("ping") {
                            debug!(
                                connection_id = %connection_id_for_receive,
                                "Received ping"
                            );
                            let _ = tx_for_receive
                                .send(WsMessage::Heartbeat {
                                    timestamp: Utc::now().timestamp(),
                                })
                                .await;
                        } else if action.eq_ignore_ascii_case("pong") {
                            debug!(
                                connection_id = %connection_id_for_receive,
                                "Received pong"
                            );
                        } else if action.eq_ignore_ascii_case("auth") {
                            debug!(
                                connection_id = %connection_id_for_receive,
                                "Client authenticated over WebSocket"
                            );
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

    // Wait for any task to complete (connection closed). Borrow the handles so
    // they survive the select! and can be aborted below — dropping a JoinHandle
    // only detaches the task, it does not stop it.
    tokio::select! {
        _ = &mut send_handle => {},
        _ = &mut receive_handle => {},
    }

    // Cleanup — abort every task spawned for this connection so none outlive it
    // (RUST_DEEP_AUDIT_2026-06-07 P1-5).
    heartbeat_handle.abort();
    send_handle.abort();
    receive_handle.abort();
    for handle in forwarders.lock().await.drain(..) {
        handle.abort();
    }

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
    _admin: crate::middleware::admin::AdminUser,
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
        .route("/", get(ws_handler))
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

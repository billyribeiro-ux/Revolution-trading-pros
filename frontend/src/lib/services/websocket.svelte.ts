/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * WebSocket Service - Svelte 5 Runes Implementation
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Production-grade WebSocket service for real-time trading alerts
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+ | Svelte 5 Runes Syntax
 *
 * Features:
 * - Automatic reconnection with exponential backoff
 * - Heartbeat monitoring and connection health tracking
 * - Room-based subscriptions for trading rooms
 * - Type-safe message handling with TypeScript
 * - Graceful degradation when WebSocket unavailable
 *
 * Built for the next 10 years with extensibility in mind.
 */

import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/** Base reconnection delay in milliseconds */
const BASE_RECONNECT_DELAY = 1000;

/** Maximum reconnection delay in milliseconds (30 seconds) */
const MAX_RECONNECT_DELAY = 30000;

/** Maximum reconnection attempts before giving up */
const MAX_RECONNECT_ATTEMPTS = 10;

/** Heartbeat timeout - disconnect if no heartbeat received within this time */
const HEARTBEAT_TIMEOUT = 45000;

/** Reconnection jitter factor to prevent thundering herd */
const JITTER_FACTOR = 0.3;

// ═══════════════════════════════════════════════════════════════════════════════
// MESSAGE TYPES - Match backend WebSocket message types
// ═══════════════════════════════════════════════════════════════════════════════

/** Alert payload from WebSocket */
export interface AlertPayload {
	id: number;
	room_slug: string;
	alert_type: string;
	ticker: string;
	title: string | null;
	message: string;
	notes: string | null;
	tos_string: string | null;
	is_new: boolean;
	is_pinned: boolean;
	published_at: string;
	created_at: string;
}

/** Trade payload from WebSocket */
export interface TradePayload {
	id: number;
	room_slug: string;
	ticker: string;
	direction: string;
	status: string;
	entry_price: number;
	exit_price: number | null;
	pnl_percent: number | null;
	result: string | null;
	invalidation_reason: string | null;
	was_updated: boolean | null;
	entry_date: string;
	exit_date: string | null;
}

/** Stats payload from WebSocket */
export interface StatsPayload {
	room_slug: string;
	win_rate: number | null;
	weekly_profit: string | null;
	active_trades: number | null;
	closed_this_week: number | null;
	total_trades: number | null;
	current_streak: number | null;
}

/** Trade plan entry payload from WebSocket */
export interface TradePlanPayload {
	id: number;
	room_slug: string;
	ticker: string;
	bias: string;
	entry: string | null;
	target1: string | null;
	target2: string | null;
	target3: string | null;
	runner: string | null;
	stop: string | null;
	options_strike: string | null;
	options_exp: string | null;
	notes: string | null;
}

/** Video payload from WebSocket */
export interface VideoPayload {
	id: number;
	room_slug: string;
	week_title: string;
	video_title: string;
	video_url: string;
	thumbnail_url: string | null;
	duration: string | null;
	published_at: string;
}

/** WebSocket message types */
export type WsMessage =
	| { type: 'AlertCreated'; payload: AlertPayload }
	| { type: 'AlertUpdated'; payload: AlertPayload }
	| { type: 'AlertDeleted'; payload: { alert_id: number } }
	| { type: 'TradeCreated'; payload: TradePayload }
	| { type: 'TradeClosed'; payload: TradePayload }
	| { type: 'TradeUpdated'; payload: TradePayload }
	| { type: 'TradeInvalidated'; payload: TradePayload }
	| { type: 'StatsUpdated'; payload: StatsPayload }
	| { type: 'TradePlanCreated'; payload: TradePlanPayload }
	| { type: 'TradePlanUpdated'; payload: TradePlanPayload }
	| { type: 'TradePlanDeleted'; payload: { entry_id: number } }
	| { type: 'VideoPublished'; payload: VideoPayload }
	| { type: 'Heartbeat'; payload: { timestamp: number } }
	| { type: 'Connected'; payload: { connection_id: string; rooms: string[]; timestamp: number } }
	| { type: 'Error'; payload: { code: string; message: string } }
	| { type: 'Subscribed'; payload: { room: string } }
	| { type: 'Unsubscribed'; payload: { room: string } };

/** Client-to-server message types */
type ClientMessage =
	| { action: 'Subscribe'; data: { room: string } }
	| { action: 'Unsubscribe'; data: { room: string } }
	| { action: 'Ping'; data: { timestamp: number } }
	| { action: 'Pong'; data: { timestamp: number } };

// ═══════════════════════════════════════════════════════════════════════════════
// CONNECTION STATE
// ═══════════════════════════════════════════════════════════════════════════════

export interface WebSocketState {
	/** Whether the WebSocket is currently connected */
	isConnected: boolean;
	/** Whether the WebSocket is attempting to reconnect */
	isReconnecting: boolean;
	/** The last message received from the server */
	lastMessage: WsMessage | null;
	/** Any connection error that occurred */
	error: Error | null;
	/** Connection ID assigned by server */
	connectionId: string | null;
	/** Currently subscribed rooms */
	subscribedRooms: Set<string>;
	/** Number of reconnection attempts */
	reconnectAttempts: number;
	/** Last heartbeat timestamp */
	lastHeartbeat: number | null;
}

/** Message handler callback type */
type MessageHandler = (message: WsMessage) => void;

// ═══════════════════════════════════════════════════════════════════════════════
// WEBSOCKET SERVICE FACTORY
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates a reactive WebSocket service for a specific trading room.
 *
 * @param initialRooms - Initial rooms to subscribe to on connection
 * @returns WebSocket service with reactive state and methods
 *
 * @example
 * ```typescript
 * const ws = createWebSocketService('explosive-swings');
 *
 * $effect(() => {
 *   ws.connect();
 *
 *   const unsubscribe = ws.subscribe((message) => {
 *     if (message.type === 'AlertCreated') {
 *       logger.info('New alert:', message.payload);
 *     }
 *   });
 *
 *   return () => {
 *     unsubscribe();
 *     ws.disconnect();
 *   };
 * });
 * ```
 */
export function createWebSocketService(initialRooms: string | string[] = []) {
	// Normalize initial rooms to array
	const rooms = Array.isArray(initialRooms) ? initialRooms : [initialRooms];

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let state = $state<WebSocketState>({
		isConnected: false,
		isReconnecting: false,
		lastMessage: null,
		error: null,
		connectionId: null,
		subscribedRooms: new Set(rooms),
		reconnectAttempts: 0,
		lastHeartbeat: null
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// INTERNAL STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let socket: WebSocket | null = null;
	let reconnectTimer: ReturnType<typeof setTimeout> | null = null;
	let heartbeatTimer: ReturnType<typeof setTimeout> | null = null;
	const messageHandlers = new Set<MessageHandler>();
	let messageQueue: ClientMessage[] = [];

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get WebSocket URL based on current environment
	 */
	function getWebSocketUrl(): string {
		if (!browser) return '';

		// ICT 7: Only connect when VITE_WS_URL is explicitly configured.
		// Do NOT construct a fallback URL — the API server may not have a WS endpoint,
		// which causes 10 failed reconnection attempts and noisy console errors.
		const envWsUrl = import.meta.env['VITE_WS_URL'];
		if (!envWsUrl) return '';

		// Add rooms as query parameter
		const roomsParam = rooms.length > 0 ? `?rooms=${rooms.join(',')}` : '';
		return `${envWsUrl}${roomsParam}`;
	}

	/**
	 * Calculate reconnection delay with exponential backoff and jitter
	 */
	function getReconnectDelay(): number {
		const exponentialDelay = Math.min(
			BASE_RECONNECT_DELAY * Math.pow(2, state.reconnectAttempts),
			MAX_RECONNECT_DELAY
		);

		// Add jitter to prevent thundering herd
		const jitter = exponentialDelay * JITTER_FACTOR * Math.random();
		return exponentialDelay + jitter;
	}

	/**
	 * Clear all timers
	 */
	function clearTimers(): void {
		if (reconnectTimer) {
			clearTimeout(reconnectTimer);
			reconnectTimer = null;
		}
		if (heartbeatTimer) {
			clearTimeout(heartbeatTimer);
			heartbeatTimer = null;
		}
	}

	/**
	 * Send a message to the server
	 */
	function send(message: ClientMessage): void {
		if (socket?.readyState === WebSocket.OPEN) {
			socket.send(JSON.stringify(message));
		} else {
			// Queue message for when connection is restored
			messageQueue.push(message);
		}
	}

	/**
	 * Flush queued messages after reconnection
	 */
	function flushMessageQueue(): void {
		while (messageQueue.length > 0 && socket?.readyState === WebSocket.OPEN) {
			const message = messageQueue.shift();
			if (message) {
				socket.send(JSON.stringify(message));
			}
		}
	}

	/**
	 * Reset heartbeat timer
	 */
	function resetHeartbeatTimer(): void {
		if (heartbeatTimer) {
			clearTimeout(heartbeatTimer);
		}

		heartbeatTimer = setTimeout(() => {
			logger.warn('[WebSocket] Heartbeat timeout - connection may be stale');

			// Try to send a ping
			send({ action: 'Ping', data: { timestamp: Date.now() } });

			// If still no response, reconnect
			heartbeatTimer = setTimeout(() => {
				logger.warn('[WebSocket] No heartbeat response - reconnecting');
				socket?.close();
			}, 10000);
		}, HEARTBEAT_TIMEOUT);
	}

	/**
	 * Handle incoming WebSocket message
	 */
	function handleMessage(event: MessageEvent): void {
		try {
			const message = JSON.parse(event.data) as WsMessage;

			// Update state
			state.lastMessage = message;

			// Handle specific message types
			switch (message.type) {
				case 'Connected':
					state.connectionId = message.payload.connection_id;
					state.reconnectAttempts = 0;
					logger.info(
						'[WebSocket] Connected:',
						message.payload.connection_id,
						'Rooms:',
						message.payload.rooms
					);
					break;

				case 'Heartbeat':
					state.lastHeartbeat = message.payload.timestamp;
					resetHeartbeatTimer();
					break;

				case 'Subscribed':
					state.subscribedRooms.add(message.payload.room);
					logger.info('[WebSocket] Subscribed to room:', message.payload.room);
					break;

				case 'Unsubscribed':
					state.subscribedRooms.delete(message.payload.room);
					logger.info('[WebSocket] Unsubscribed from room:', message.payload.room);
					break;

				case 'Error':
					logger.error('[WebSocket] Server error:', message.payload.code, message.payload.message);
					state.error = new Error(`${message.payload.code}: ${message.payload.message}`);
					break;
			}

			// Notify all message handlers
			messageHandlers.forEach((handler) => {
				try {
					handler(message);
				} catch (err) {
					logger.error('[WebSocket] Error in message handler:', err);
				}
			});
		} catch (err) {
			logger.error('[WebSocket] Failed to parse message:', err);
		}
	}

	/**
	 * Handle WebSocket connection open
	 */
	function handleOpen(): void {
		state.isConnected = true;
		state.isReconnecting = false;
		state.error = null;
		state.reconnectAttempts = 0;

		// Start heartbeat monitoring
		resetHeartbeatTimer();

		// Flush any queued messages
		flushMessageQueue();

		// Re-subscribe to rooms (in case of reconnection)
		state.subscribedRooms.forEach((room) => {
			send({ action: 'Subscribe', data: { room } });
		});

		logger.info('[WebSocket] Connection opened');
	}

	/**
	 * Handle WebSocket connection close
	 */
	function handleClose(event: CloseEvent): void {
		state.isConnected = false;
		clearTimers();

		logger.info('[WebSocket] Connection closed:', event.code, event.reason);

		// Attempt reconnection if not a deliberate close
		if (event.code !== 1000 && state.reconnectAttempts < MAX_RECONNECT_ATTEMPTS) {
			scheduleReconnect();
		} else if (state.reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
			state.error = new Error('Maximum reconnection attempts reached');
			logger.error('[WebSocket] Max reconnection attempts reached');
		}
	}

	/**
	 * Handle WebSocket error
	 */
	function handleError(event: Event): void {
		logger.error('[WebSocket] Error:', event);
		state.error = new Error('WebSocket connection error');
	}

	/**
	 * Schedule a reconnection attempt
	 */
	function scheduleReconnect(): void {
		if (state.isReconnecting) return;

		state.isReconnecting = true;
		const delay = getReconnectDelay();
		state.reconnectAttempts++;

		logger.info(
			`[WebSocket] Reconnecting in ${Math.round(delay)}ms (attempt ${state.reconnectAttempts}/${MAX_RECONNECT_ATTEMPTS})`
		);

		reconnectTimer = setTimeout(() => {
			connect();
		}, delay);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PUBLIC METHODS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Connect to the WebSocket server
	 */
	function connect(): void {
		if (!browser) return;

		// Don't connect if already connected or connecting
		if (socket?.readyState === WebSocket.OPEN || socket?.readyState === WebSocket.CONNECTING) {
			return;
		}

		const url = getWebSocketUrl();
		if (!url) {
			logger.debug('[WebSocket] No VITE_WS_URL configured — skipping connection');
			return;
		}

		try {
			socket = new WebSocket(url);

			socket.onopen = handleOpen;
			socket.onclose = handleClose;
			socket.onerror = handleError;
			socket.onmessage = handleMessage;

			logger.info('[WebSocket] Connecting to:', url);
		} catch (err) {
			logger.error('[WebSocket] Failed to create connection:', err);
			state.error = err instanceof Error ? err : new Error('Failed to connect');
			scheduleReconnect();
		}
	}

	/**
	 * Disconnect from the WebSocket server
	 */
	function disconnect(): void {
		clearTimers();
		state.reconnectAttempts = MAX_RECONNECT_ATTEMPTS; // Prevent auto-reconnect

		if (socket) {
			socket.close(1000, 'Client disconnect');
			socket = null;
		}

		state.isConnected = false;
		state.isReconnecting = false;
		state.connectionId = null;

		logger.info('[WebSocket] Disconnected');
	}

	/**
	 * Subscribe to a trading room
	 */
	function subscribeRoom(room: string): void {
		if (!state.subscribedRooms.has(room)) {
			state.subscribedRooms.add(room);
			send({ action: 'Subscribe', data: { room } });
		}
	}

	/**
	 * Unsubscribe from a trading room
	 */
	function unsubscribeRoom(room: string): void {
		if (state.subscribedRooms.has(room)) {
			state.subscribedRooms.delete(room);
			send({ action: 'Unsubscribe', data: { room } });
		}
	}

	/**
	 * Subscribe to WebSocket messages
	 *
	 * @param handler - Callback function to handle messages
	 * @returns Unsubscribe function
	 */
	function subscribe(handler: MessageHandler): () => void {
		messageHandlers.add(handler);

		return () => {
			messageHandlers.delete(handler);
		};
	}

	/**
	 * Manually trigger a reconnection
	 */
	function reconnect(): void {
		state.reconnectAttempts = 0;
		disconnect();
		connect();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// Reactive state accessors
		get isConnected() {
			return state.isConnected;
		},
		get isReconnecting() {
			return state.isReconnecting;
		},
		get lastMessage() {
			return state.lastMessage;
		},
		get error() {
			return state.error;
		},
		get connectionId() {
			return state.connectionId;
		},
		get subscribedRooms() {
			return state.subscribedRooms;
		},
		get reconnectAttempts() {
			return state.reconnectAttempts;
		},
		get lastHeartbeat() {
			return state.lastHeartbeat;
		},

		// Methods
		connect,
		disconnect,
		reconnect,
		subscribeRoom,
		unsubscribeRoom,
		subscribe
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE EXPORT
// ═══════════════════════════════════════════════════════════════════════════════

export type WebSocketService = ReturnType<typeof createWebSocketService>;

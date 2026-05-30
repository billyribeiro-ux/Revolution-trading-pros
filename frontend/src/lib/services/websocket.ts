import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';
import type { JsonValue } from '$lib/api/_types';

interface WebSocketMessage {
	event: string;
	widget_id?: string;
	dashboard_id?: string;
	user_id?: string;
	cart_id?: string;
	notification?: NotificationPayload;
	// `data` / `changes` are wire-level polymorphic payloads keyed off `event`.
	// The handler narrows per-branch (`data as RoomAlertPayload` etc.).
	data?: JsonValue;
	changes?: JsonValue;
	timestamp: string;
}

/**
 * Server-bound control messages. The backend's subscription protocol only
 * accepts `{ action: 'subscribe' | 'unsubscribe', channel: string }`.
 */
interface WebSocketControlMessage {
	action: 'subscribe' | 'unsubscribe';
	channel: string;
}

/**
 * Erased callback signature stored in the subscriptions map. Each typed
 * `subscribeTo*` method wraps its caller's callback in this erased shape;
 * the handler dispatches a per-channel narrowed payload via a localized
 * cast.
 */
type ChannelCallback = (payload: unknown) => void;

/**
 * Narrow a `JsonValue` to a plain object form so handlers can safely read
 * string-keyed fields like `data.room_slug` or `data.session_id`. Returns
 * `null` for arrays / primitives / null.
 */
function asObject(value: JsonValue | undefined): { [k: string]: JsonValue | undefined } | null {
	if (value !== null && typeof value === 'object' && !Array.isArray(value)) {
		return value;
	}
	return null;
}

export interface NotificationPayload {
	id: string;
	type: 'info' | 'success' | 'warning' | 'error' | 'system';
	priority: 'low' | 'normal' | 'high' | 'urgent';
	title: string;
	message: string;
	action?: {
		label: string;
		href?: string;
	};
	metadata?: Record<string, unknown>;
	timestamp: string;
	read: boolean;
	dismissed: boolean;
}

export interface CartUpdatePayload {
	cart_id: string;
	action: string;
	product_id?: string;
	items_count: number;
	subtotal: number;
	total: number;
	items: Array<{
		id: string;
		product_id: string;
		quantity: number;
		price: number;
		total: number;
		reserved_until?: string;
	}>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Explosive Swings Real-time Payload Types
// ═══════════════════════════════════════════════════════════════════════════════

export interface RoomAlertPayload {
	id: number;
	room_slug: string;
	alert_type: 'ENTRY' | 'EXIT' | 'UPDATE';
	ticker: string;
	title: string;
	message?: string;
	tos_string?: string;
	published_at: string;
}

export interface TradeUpdatePayload {
	id: number;
	room_slug: string;
	ticker: string;
	status: 'open' | 'closed' | 'invalidated';
	pnl?: number;
	pnl_percent?: number;
}

export interface StatsUpdatePayload {
	room_slug: string;
	win_rate: number;
	active_trades: number;
	closed_this_week: number;
}

class WebSocketService {
	private ws: WebSocket | null = null;
	private reconnectAttempts = 0;
	private maxReconnectAttempts = 5;
	private reconnectDelay = 1000;
	private subscriptions: Map<string, Set<ChannelCallback>> = new Map();

	public connected: Writable<boolean> = writable(false);
	public error: Writable<string | null> = writable(null);

	constructor(private url: string) {}

	/**
	 * Register a callback for a channel, lazily creating its callback set.
	 * Returns the (now guaranteed-present) set so callers avoid a
	 * non-null-asserted `Map.get`.
	 */
	private addCallback(channel: string, callback: ChannelCallback): Set<ChannelCallback> {
		let callbacks = this.subscriptions.get(channel);
		if (!callbacks) {
			callbacks = new Set();
			this.subscriptions.set(channel, callbacks);
		}
		callbacks.add(callback);
		return callbacks;
	}

	/**
	 * Connect to WebSocket server
	 */
	connect(): void {
		if (!browser || this.ws?.readyState === WebSocket.OPEN) {
			return;
		}

		try {
			this.ws = new WebSocket(this.url);

			this.ws.onopen = () => {
				console.info('WebSocket connected');
				this.connected.set(true);
				this.error.set(null);
				this.reconnectAttempts = 0;
			};

			this.ws.onmessage = (event) => {
				try {
					const message: WebSocketMessage = JSON.parse(event.data);
					this.handleMessage(message);
				} catch (err) {
					console.error('Failed to parse WebSocket message:', err);
				}
			};

			this.ws.onerror = (event) => {
				console.error('WebSocket error:', event);
				this.error.set('WebSocket connection error');
			};

			this.ws.onclose = () => {
				console.info('WebSocket disconnected');
				this.connected.set(false);
				this.attemptReconnect();
			};
		} catch (err) {
			console.error('Failed to create WebSocket:', err);
			this.error.set('Failed to connect to WebSocket');
		}
	}

	/**
	 * Disconnect from WebSocket
	 */
	disconnect(): void {
		if (this.ws) {
			this.ws.close();
			this.ws = null;
			this.connected.set(false);
		}
	}

	/**
	 * Subscribe to widget updates
	 */
	subscribeToWidget(widgetId: string, callback: (data: JsonValue) => void): () => void {
		const channel = `widget:${widgetId}`;

		this.addCallback(channel, callback as ChannelCallback);

		// Send subscription message to server
		this.send({
			action: 'subscribe',
			channel: `dashboard:widget:${widgetId}`
		});

		// Return unsubscribe function
		return () => {
			this.subscriptions.get(channel)?.delete(callback as ChannelCallback);
			if (this.subscriptions.get(channel)?.size === 0) {
				this.send({
					action: 'unsubscribe',
					channel: `dashboard:widget:${widgetId}`
				});
			}
		};
	}

	/**
	 * Subscribe to dashboard updates
	 */
	subscribeToDashboard(dashboardId: string, callback: (changes: JsonValue) => void): () => void {
		const channel = `dashboard:${dashboardId}`;

		this.addCallback(channel, callback as ChannelCallback);

		this.send({
			action: 'subscribe',
			channel: `dashboard:dashboard:${dashboardId}`
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback as ChannelCallback);
			if (this.subscriptions.get(channel)?.size === 0) {
				this.send({
					action: 'unsubscribe',
					channel: `dashboard:dashboard:${dashboardId}`
				});
			}
		};
	}

	/**
	 * Subscribe to user notifications
	 */
	subscribeToNotifications(
		userId: string,
		callback: (notification: NotificationPayload) => void
	): () => void {
		const channel = `user:${userId}:notifications`;

		this.addCallback(channel, callback as ChannelCallback);

		this.send({
			action: 'subscribe',
			channel: `private-user.${userId}`
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback as ChannelCallback);
			if (this.subscriptions.get(channel)?.size === 0) {
				this.send({
					action: 'unsubscribe',
					channel: `private-user.${userId}`
				});
			}
		};
	}

	/**
	 * Subscribe to cart updates
	 */
	subscribeToCart(
		userId: string | null,
		sessionId: string,
		callback: (cart: CartUpdatePayload) => void
	): () => void {
		const channel = userId ? `user:${userId}:cart` : `session:${sessionId}:cart`;
		const wsChannel = userId ? `private-user.${userId}.cart` : `private-session.${sessionId}.cart`;

		this.addCallback(channel, callback as ChannelCallback);

		this.send({
			action: 'subscribe',
			channel: wsChannel
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback as ChannelCallback);
			if (this.subscriptions.get(channel)?.size === 0) {
				this.send({
					action: 'unsubscribe',
					channel: wsChannel
				});
			}
		};
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Explosive Swings Subscription Methods
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Subscribe to explosive swings room alerts
	 */
	subscribeToRoomAlerts(roomSlug: string, callback: (alert: RoomAlertPayload) => void): () => void {
		const channel = `room:${roomSlug}:alerts`;
		const wsChannel = `room.${roomSlug}.alerts`;

		this.addCallback(channel, callback as ChannelCallback);

		this.send({
			action: 'subscribe',
			channel: wsChannel
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback as ChannelCallback);
			if (this.subscriptions.get(channel)?.size === 0) {
				this.send({
					action: 'unsubscribe',
					channel: wsChannel
				});
			}
		};
	}

	/**
	 * Subscribe to explosive swings trade updates
	 */
	subscribeToTradeUpdates(
		roomSlug: string,
		callback: (trade: TradeUpdatePayload) => void
	): () => void {
		const channel = `room:${roomSlug}:trades`;
		const wsChannel = `room.${roomSlug}.trades`;

		this.addCallback(channel, callback as ChannelCallback);

		this.send({
			action: 'subscribe',
			channel: wsChannel
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback as ChannelCallback);
			if (this.subscriptions.get(channel)?.size === 0) {
				this.send({
					action: 'unsubscribe',
					channel: wsChannel
				});
			}
		};
	}

	/**
	 * Subscribe to explosive swings stats updates
	 */
	subscribeToStatsUpdates(
		roomSlug: string,
		callback: (stats: StatsUpdatePayload) => void
	): () => void {
		const channel = `room:${roomSlug}:stats`;
		const wsChannel = `room.${roomSlug}.stats`;

		this.addCallback(channel, callback as ChannelCallback);

		this.send({
			action: 'subscribe',
			channel: wsChannel
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback as ChannelCallback);
			if (this.subscriptions.get(channel)?.size === 0) {
				this.send({
					action: 'unsubscribe',
					channel: wsChannel
				});
			}
		};
	}

	/**
	 * Handle incoming message
	 */
	private handleMessage(message: WebSocketMessage): void {
		const { event, widget_id, dashboard_id, user_id, notification, data, changes } = message;

		if (event === 'widget:update' && widget_id) {
			const channel = `widget:${widget_id}`;
			this.subscriptions.get(channel)?.forEach((callback) => callback(data));
		}

		if (event === 'dashboard:update' && dashboard_id) {
			const channel = `dashboard:${dashboard_id}`;
			this.subscriptions.get(channel)?.forEach((callback) => callback(changes));
		}

		if (event === 'widget:refresh' && widget_id) {
			const channel = `widget:${widget_id}`;
			this.subscriptions.get(channel)?.forEach((callback) => callback({ refresh: true }));
		}

		// Handle user notifications
		if (event === 'notification' && user_id && notification) {
			const channel = `user:${user_id}:notifications`;
			this.subscriptions.get(channel)?.forEach((callback) => callback(notification));
		}

		// Handle cart updates
		if (event === 'cart.updated' && data) {
			const dataObj = asObject(data);
			// Try user channel first, then session channel
			if (user_id) {
				const userChannel = `user:${user_id}:cart`;
				this.subscriptions.get(userChannel)?.forEach((callback) => callback(data));
			}
			const sessionId = dataObj?.session_id;
			if (typeof sessionId === 'string') {
				const sessionChannel = `session:${sessionId}:cart`;
				this.subscriptions.get(sessionChannel)?.forEach((callback) => callback(data));
			}
		}

		// ═══════════════════════════════════════════════════════════════════════════
		// Handle Explosive Swings room events
		// ═══════════════════════════════════════════════════════════════════════════

		const dataObj = asObject(data);
		const roomSlug = dataObj?.room_slug;

		// Handle room alert created
		if (
			(event === 'room:alert:created' || event === 'room:alert:updated') &&
			typeof roomSlug === 'string'
		) {
			const channel = `room:${roomSlug}:alerts`;
			this.subscriptions
				.get(channel)
				?.forEach((callback) => callback(data as unknown as RoomAlertPayload));
		}

		// Handle room trade opened/closed
		if (
			(event === 'room:trade:opened' || event === 'room:trade:closed') &&
			typeof roomSlug === 'string'
		) {
			const channel = `room:${roomSlug}:trades`;
			this.subscriptions
				.get(channel)
				?.forEach((callback) => callback(data as unknown as TradeUpdatePayload));
		}

		// Handle room stats updated
		if (event === 'room:stats:updated' && typeof roomSlug === 'string') {
			const channel = `room:${roomSlug}:stats`;
			this.subscriptions
				.get(channel)
				?.forEach((callback) => callback(data as unknown as StatsUpdatePayload));
		}

		if (event === 'system:notification') {
			// Handle system-wide notifications
			console.info('System notification:', message);
		}
	}

	/**
	 * Send a subscribe/unsubscribe control message to the server.
	 *
	 * The backend's subscription protocol only accepts the
	 * `WebSocketControlMessage` shape; previously typed as `any` which
	 * permitted accidentally posting arbitrary payloads. Tightening to
	 * the control union catches typos at compile time.
	 */
	private send(data: WebSocketControlMessage): void {
		if (this.ws?.readyState === WebSocket.OPEN) {
			this.ws.send(JSON.stringify(data));
		}
	}

	/**
	 * Attempt to reconnect
	 */
	private attemptReconnect(): void {
		if (this.reconnectAttempts >= this.maxReconnectAttempts) {
			this.error.set('Max reconnection attempts reached');
			return;
		}

		this.reconnectAttempts++;
		const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);

		console.info(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);

		setTimeout(() => {
			this.connect();
		}, delay);
	}

	/**
	 * Check if connected
	 */
	isConnected(): boolean {
		return this.ws?.readyState === WebSocket.OPEN;
	}
}

// Create singleton instance - use wss:// in production to avoid Mixed Content errors
// NOTE: Cloudflare Pages does NOT support WebSockets - only connect if WS URL is explicitly configured
const configuredWsUrl = browser ? import.meta.env['VITE_WS_URL'] : '';
// Only use WebSocket if explicitly configured via environment variable
const wsUrl = configuredWsUrl || '';
export const websocketService = new WebSocketService(wsUrl);

// Auto-connect in browser only if WebSocket URL is configured
if (browser && wsUrl) {
	websocketService.connect();
}

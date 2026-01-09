import { writable, type Writable } from 'svelte/store';
import { browser } from '$app/environment';

interface WebSocketMessage {
	event: string;
	widget_id?: string;
	dashboard_id?: string;
	user_id?: string;
	cart_id?: string;
	notification?: NotificationPayload;
	data?: any;
	changes?: any;
	timestamp: string;
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

class WebSocketService {
	private ws: WebSocket | null = null;
	private reconnectAttempts = 0;
	private maxReconnectAttempts = 5;
	private reconnectDelay = 1000;
	private subscriptions: Map<string, Set<(data: any) => void>> = new Map();

	public connected: Writable<boolean> = writable(false);
	public error: Writable<string | null> = writable(null);

	constructor(private url: string) {}

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
				console.log('WebSocket connected');
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
				console.log('WebSocket disconnected');
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
	subscribeToWidget(widgetId: string, callback: (data: any) => void): () => void {
		const channel = `widget:${widgetId}`;

		if (!this.subscriptions.has(channel)) {
			this.subscriptions.set(channel, new Set());
		}

		this.subscriptions.get(channel)!.add(callback);

		// Send subscription message to server
		this.send({
			action: 'subscribe',
			channel: `dashboard:widget:${widgetId}`
		});

		// Return unsubscribe function
		return () => {
			this.subscriptions.get(channel)?.delete(callback);
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
	subscribeToDashboard(dashboardId: string, callback: (changes: any) => void): () => void {
		const channel = `dashboard:${dashboardId}`;

		if (!this.subscriptions.has(channel)) {
			this.subscriptions.set(channel, new Set());
		}

		this.subscriptions.get(channel)!.add(callback);

		this.send({
			action: 'subscribe',
			channel: `dashboard:dashboard:${dashboardId}`
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback);
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
	subscribeToNotifications(userId: string, callback: (notification: NotificationPayload) => void): () => void {
		const channel = `user:${userId}:notifications`;

		if (!this.subscriptions.has(channel)) {
			this.subscriptions.set(channel, new Set());
		}

		this.subscriptions.get(channel)!.add(callback);

		this.send({
			action: 'subscribe',
			channel: `private-user.${userId}`
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback);
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
	subscribeToCart(userId: string | null, sessionId: string, callback: (cart: CartUpdatePayload) => void): () => void {
		const channel = userId ? `user:${userId}:cart` : `session:${sessionId}:cart`;
		const wsChannel = userId ? `private-user.${userId}.cart` : `private-session.${sessionId}.cart`;

		if (!this.subscriptions.has(channel)) {
			this.subscriptions.set(channel, new Set());
		}

		this.subscriptions.get(channel)!.add(callback);

		this.send({
			action: 'subscribe',
			channel: wsChannel
		});

		return () => {
			this.subscriptions.get(channel)?.delete(callback);
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
			// Try user channel first, then session channel
			if (user_id) {
				const userChannel = `user:${user_id}:cart`;
				this.subscriptions.get(userChannel)?.forEach((callback) => callback(data));
			}
			if (data.session_id) {
				const sessionChannel = `session:${data.session_id}:cart`;
				this.subscriptions.get(sessionChannel)?.forEach((callback) => callback(data));
			}
		}

		if (event === 'system:notification') {
			// Handle system-wide notifications
			console.log('System notification:', message);
		}
	}

	/**
	 * Send message to server
	 */
	private send(data: any): void {
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

		console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);

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
const wsProtocol = browser && window.location.protocol === 'https:' ? 'wss:' : 'ws:';
const configuredWsUrl = browser ? import.meta.env['VITE_WS_URL'] : '';
// Only use WebSocket if explicitly configured via environment variable
const wsUrl = configuredWsUrl || '';
export const websocketService = new WebSocketService(wsUrl);

// Auto-connect in browser only if WebSocket URL is configured
if (browser && wsUrl) {
	websocketService.connect();
}

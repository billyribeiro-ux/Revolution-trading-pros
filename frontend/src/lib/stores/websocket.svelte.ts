import { browser } from '$app/environment';

/**
 * WebSocket Store - Real-time Updates (Svelte 5 Runes)
 *
 * Provides real-time communication matching Fluent Cart Pro features:
 * - Cart updates
 * - Inventory alerts
 * - Order status changes
 * - Notifications
 */

// Types
export interface WebSocketMessage {
	event: string;
	channel: string;
	data: Record<string, unknown>;
	timestamp: string;
}

export interface ConnectionState {
	connected: boolean;
	connecting: boolean;
	error: string | null;
	reconnectAttempts: number;
}

export interface CartUpdate {
	cart_id: string;
	action: 'added' | 'removed' | 'updated' | 'cleared';
	items_count: number;
	subtotal: number;
	total: number;
	items: CartItem[];
}

export interface CartItem {
	id: number;
	product_id: number;
	quantity: number;
	price: number;
	total: number;
	reserved_until?: string;
}

export interface InventoryUpdate {
	product_id: number;
	available_stock: number;
	reservation?: {
		reservation_id: string;
		quantity: number;
		expires_at: string;
	};
}

export interface OrderStatusUpdate {
	order_id: number;
	status: string;
	message: string;
}

export interface Notification {
	id: string;
	type: string;
	title: string;
	message: string;
	data: Record<string, unknown>;
	read: boolean;
	timestamp: string;
}

// Configuration
const WS_CONFIG = {
	reconnectDelay: 1000,
	maxReconnectDelay: 30000,
	maxReconnectAttempts: 10,
	heartbeatInterval: 30000
};

/**
 * WebSocket State Manager using Svelte 5 Runes
 */
class WebSocketState {
	// Reactive state using $state rune
	connection = $state<ConnectionState>({
		connected: false,
		connecting: false,
		error: null,
		reconnectAttempts: 0
	});

	messages = $state<WebSocketMessage[]>([]);
	cart = $state<CartUpdate | null>(null);
	inventory = $state<Map<number, InventoryUpdate>>(new Map());
	orders = $state<Map<number, OrderStatusUpdate>>(new Map());
	notifications = $state<Notification[]>([]);

	// Derived state using $derived rune
	unreadCount = $derived(this.notifications.filter((n) => !n.read).length);
	isConnected = $derived(this.connection.connected);

	// Private state
	private ws: WebSocket | null = null;
	private reconnectTimeout: ReturnType<typeof setTimeout> | null = null;
	private heartbeatInterval: ReturnType<typeof setInterval> | null = null;
	private subscribedChannels: Set<string> = new Set();
	private config: { wsUrl: string; authToken?: string } | null = null;

	// Calculate reconnect delay with exponential backoff
	private getReconnectDelay(attempts: number): number {
		const delay = WS_CONFIG.reconnectDelay * Math.pow(2, attempts);
		return Math.min(delay, WS_CONFIG.maxReconnectDelay);
	}

	// Connect to WebSocket server
	connect(config: { wsUrl: string; authToken?: string }): void {
		if (!browser) return;

		this.config = config;
		this.connection.connecting = true;
		this.connection.error = null;

		try {
			const url = new URL(config.wsUrl);
			if (config.authToken) {
				url.searchParams.set('token', config.authToken);
			}

			this.ws = new WebSocket(url.toString());

			this.ws.onopen = () => {
				this.connection = {
					connected: true,
					connecting: false,
					error: null,
					reconnectAttempts: 0
				};

				this.startHeartbeat();

				// Resubscribe to channels
				this.subscribedChannels.forEach((channel) => {
					this.sendSubscribe(channel);
				});

				console.log('[WebSocket] Connected');
			};

			this.ws.onmessage = (event) => {
				try {
					const message = JSON.parse(event.data) as WebSocketMessage;
					this.handleMessage(message);
				} catch (e) {
					console.error('[WebSocket] Failed to parse message:', e);
				}
			};

			this.ws.onerror = (error) => {
				console.error('[WebSocket] Error:', error);
				this.connection.error = 'Connection error';
			};

			this.ws.onclose = (event) => {
				this.connection.connected = false;
				this.connection.connecting = false;

				this.stopHeartbeat();

				// Attempt reconnection if not intentionally closed
				if (event.code !== 1000) {
					this.scheduleReconnect();
				}

				console.log('[WebSocket] Disconnected:', event.code, event.reason);
			};
		} catch (error) {
			this.connection.connecting = false;
			this.connection.error = error instanceof Error ? error.message : 'Failed to connect';
		}
	}

	// Disconnect from WebSocket server
	disconnect(): void {
		if (this.reconnectTimeout) {
			clearTimeout(this.reconnectTimeout);
			this.reconnectTimeout = null;
		}

		this.stopHeartbeat();

		if (this.ws) {
			this.ws.close(1000, 'Client disconnect');
			this.ws = null;
		}

		this.subscribedChannels.clear();

		this.connection = {
			connected: false,
			connecting: false,
			error: null,
			reconnectAttempts: 0
		};
	}

	// Schedule reconnection
	private scheduleReconnect(): void {
		if (!this.config) return;

		if (this.connection.reconnectAttempts >= WS_CONFIG.maxReconnectAttempts) {
			this.connection.error = 'Max reconnection attempts reached';
			return;
		}

		const delay = this.getReconnectDelay(this.connection.reconnectAttempts);
		console.log(
			`[WebSocket] Reconnecting in ${delay}ms (attempt ${this.connection.reconnectAttempts + 1})`
		);

		this.reconnectTimeout = setTimeout(() => {
			if (this.config) {
				this.connect(this.config);
			}
		}, delay);

		this.connection.reconnectAttempts++;
	}

	// Start heartbeat
	private startHeartbeat(): void {
		this.heartbeatInterval = setInterval(() => {
			if (this.ws?.readyState === WebSocket.OPEN) {
				this.ws.send(JSON.stringify({ event: 'ping' }));
			}
		}, WS_CONFIG.heartbeatInterval);
	}

	// Stop heartbeat
	private stopHeartbeat(): void {
		if (this.heartbeatInterval) {
			clearInterval(this.heartbeatInterval);
			this.heartbeatInterval = null;
		}
	}

	// Send subscribe message
	private sendSubscribe(channel: string): void {
		if (this.ws?.readyState === WebSocket.OPEN) {
			this.ws.send(
				JSON.stringify({
					event: 'subscribe',
					channel
				})
			);
		}
	}

	// Subscribe to a channel
	subscribe(channel: string): void {
		this.subscribedChannels.add(channel);
		this.sendSubscribe(channel);
	}

	// Unsubscribe from a channel
	unsubscribe(channel: string): void {
		this.subscribedChannels.delete(channel);
		if (this.ws?.readyState === WebSocket.OPEN) {
			this.ws.send(
				JSON.stringify({
					event: 'unsubscribe',
					channel
				})
			);
		}
	}

	// Handle incoming message
	private handleMessage(message: WebSocketMessage): void {
		// Store all messages (keep last 100)
		this.messages = [...this.messages.slice(-99), message];

		// Route to specific handlers based on event type
		switch (message.event) {
			case 'cart.updated':
				this.handleCartUpdate(message.data as unknown as CartUpdate);
				break;

			case 'inventory.updated':
			case 'inventory.reserved':
				this.handleInventoryUpdate(message.data as unknown as InventoryUpdate);
				break;

			case 'order.status_changed':
				this.handleOrderUpdate(message.data as unknown as OrderStatusUpdate);
				break;

			case 'notification':
				this.handleNotification(message.data as unknown as Notification);
				break;

			case 'pong':
				// Heartbeat response - no action needed
				break;

			default:
				console.log('[WebSocket] Unhandled event:', message.event, message.data);
		}
	}

	// Handle cart update
	private handleCartUpdate(update: CartUpdate): void {
		this.cart = update;

		// Dispatch custom event for components to listen to
		if (browser) {
			window.dispatchEvent(new CustomEvent('cartUpdated', { detail: update }));
		}
	}

	// Handle inventory update
	private handleInventoryUpdate(update: InventoryUpdate): void {
		const newMap = new Map(this.inventory);
		newMap.set(update.product_id, update);
		this.inventory = newMap;

		// Dispatch custom event
		if (browser) {
			window.dispatchEvent(new CustomEvent('inventoryUpdated', { detail: update }));
		}
	}

	// Handle order update
	private handleOrderUpdate(update: OrderStatusUpdate): void {
		const newMap = new Map(this.orders);
		newMap.set(update.order_id, update);
		this.orders = newMap;

		// Dispatch custom event
		if (browser) {
			window.dispatchEvent(new CustomEvent('orderUpdated', { detail: update }));
		}
	}

	// Handle notification
	private handleNotification(notification: Notification): void {
		this.notifications = [notification, ...this.notifications.slice(0, 49)];

		// Dispatch custom event
		if (browser) {
			window.dispatchEvent(new CustomEvent('notificationReceived', { detail: notification }));
		}

		// Show browser notification if permitted
		if (browser && Notification.permission === 'granted') {
			new Notification(notification.title, {
				body: notification.message,
				icon: '/favicon.png'
			});
		}
	}

	// Mark notification as read
	markNotificationRead(id: string): void {
		this.notifications = this.notifications.map((n) => (n.id === id ? { ...n, read: true } : n));
	}

	// Clear all notifications
	clearNotifications(): void {
		this.notifications = [];
	}

	// Get inventory for product
	getInventory(productId: number): InventoryUpdate | undefined {
		return this.inventory.get(productId);
	}

	// Get order status
	getOrderStatus(orderId: number): OrderStatusUpdate | undefined {
		return this.orders.get(orderId);
	}
}

// Export singleton instance
export const websocket = new WebSocketState();

// Auto-reconnect on visibility change
if (browser) {
	document.addEventListener('visibilitychange', () => {
		if (document.visibilityState === 'visible') {
			if (!websocket.connection.connected && !websocket.connection.connecting) {
				console.log('[WebSocket] Page visible, checking connection...');
			}
		}
	});
}

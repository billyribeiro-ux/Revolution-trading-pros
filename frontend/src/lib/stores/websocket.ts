/**
 * Revolution Trading Pros - Enterprise WebSocket Store
 * ====================================================
 * Real-time data synchronization with automatic reconnection,
 * heartbeat monitoring, and event-based architecture.
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';
import { authStore } from './auth';

export type WebSocketStatus = 'connecting' | 'connected' | 'disconnected' | 'reconnecting' | 'error';

export interface WebSocketMessage<T = unknown> {
	type: string;
	channel?: string;
	data: T;
	timestamp: number;
}

interface WebSocketState {
	status: WebSocketStatus;
	lastMessage: WebSocketMessage | null;
	lastError: string | null;
	reconnectAttempts: number;
	latency: number | null;
}

// Configuration
const WS_URL = browser ? `${window.location.protocol === 'https:' ? 'wss:' : 'ws:'}//${window.location.host}/ws` : '';
const RECONNECT_DELAY_BASE = 1000; // 1 second
const RECONNECT_MAX_DELAY = 30000; // 30 seconds
const MAX_RECONNECT_ATTEMPTS = 10;
const HEARTBEAT_INTERVAL = 30000; // 30 seconds
const HEARTBEAT_TIMEOUT = 10000; // 10 seconds

function createWebSocketStore() {
	const { subscribe, set, update } = writable<WebSocketState>({
		status: 'disconnected',
		lastMessage: null,
		lastError: null,
		reconnectAttempts: 0,
		latency: null
	});

	let ws: WebSocket | null = null;
	let heartbeatInterval: NodeJS.Timeout | null = null;
	let heartbeatTimeout: NodeJS.Timeout | null = null;
	let reconnectTimeout: NodeJS.Timeout | null = null;
	let pingTimestamp: number | null = null;

	// Event listeners map
	const listeners = new Map<string, Set<(data: unknown) => void>>();

	// Channel subscriptions
	const subscriptions = new Set<string>();

	function emit(type: string, data: unknown) {
		const typeListeners = listeners.get(type);
		if (typeListeners) {
			typeListeners.forEach((callback) => callback(data));
		}

		// Also emit to wildcard listeners
		const wildcardListeners = listeners.get('*');
		if (wildcardListeners) {
			wildcardListeners.forEach((callback) => callback({ type, data }));
		}
	}

	function startHeartbeat() {
		stopHeartbeat();

		heartbeatInterval = setInterval(() => {
			if (ws?.readyState === WebSocket.OPEN) {
				pingTimestamp = Date.now();
				ws.send(JSON.stringify({ type: 'ping', timestamp: pingTimestamp }));

				// Set timeout for pong response
				heartbeatTimeout = setTimeout(() => {
					console.warn('WebSocket heartbeat timeout - reconnecting...');
					reconnect();
				}, HEARTBEAT_TIMEOUT);
			}
		}, HEARTBEAT_INTERVAL);
	}

	function stopHeartbeat() {
		if (heartbeatInterval) {
			clearInterval(heartbeatInterval);
			heartbeatInterval = null;
		}
		if (heartbeatTimeout) {
			clearTimeout(heartbeatTimeout);
			heartbeatTimeout = null;
		}
	}

	function calculateReconnectDelay(attempts: number): number {
		// Exponential backoff with jitter
		const delay = Math.min(
			RECONNECT_DELAY_BASE * Math.pow(2, attempts) + Math.random() * 1000,
			RECONNECT_MAX_DELAY
		);
		return delay;
	}

	function connect() {
		if (!browser) return;

		// Get session ID for WebSocket authentication
		// SECURITY: Token is NOT sent in URL - uses cookie-based auth instead
		const sessionId = authStore.getSessionId();

		// Build WebSocket URL with session ID only (token via cookies)
		let url = WS_URL;
		if (sessionId) {
			url += `?session_id=${sessionId}`;
		}

		update((state) => ({ ...state, status: 'connecting' }));

		try {
			ws = new WebSocket(url);

			ws.onopen = () => {
				console.log('WebSocket connected');
				update((state) => ({
					...state,
					status: 'connected',
					reconnectAttempts: 0,
					lastError: null
				}));

				startHeartbeat();

				// Resubscribe to all channels
				subscriptions.forEach((channel) => {
					ws?.send(JSON.stringify({ type: 'subscribe', channel }));
				});

				emit('connected', {});
			};

			ws.onmessage = (event) => {
				try {
					const message: WebSocketMessage = JSON.parse(event.data);

					// Handle pong response
					if (message.type === 'pong') {
						if (heartbeatTimeout) {
							clearTimeout(heartbeatTimeout);
							heartbeatTimeout = null;
						}
						if (pingTimestamp) {
							const latency = Date.now() - pingTimestamp;
							update((state) => ({ ...state, latency }));
						}
						return;
					}

					// Update last message
					update((state) => ({
						...state,
						lastMessage: message
					}));

					// Emit to listeners
					emit(message.type, message.data);

					// Emit channel-specific events
					if (message.channel) {
						emit(`channel:${message.channel}`, message.data);
					}
				} catch (error) {
					console.error('Failed to parse WebSocket message:', error);
				}
			};

			ws.onerror = (error) => {
				console.error('WebSocket error:', error);
				update((state) => ({
					...state,
					status: 'error',
					lastError: 'WebSocket connection error'
				}));
			};

			ws.onclose = (event) => {
				console.log('WebSocket closed:', event.code, event.reason);
				stopHeartbeat();

				update((state) => ({
					...state,
					status: 'disconnected'
				}));

				emit('disconnected', { code: event.code, reason: event.reason });

				// Auto-reconnect if not intentionally closed
				if (event.code !== 1000 && event.code !== 1001) {
					reconnect();
				}
			};
		} catch (error) {
			console.error('Failed to create WebSocket:', error);
			update((state) => ({
				...state,
				status: 'error',
				lastError: 'Failed to create WebSocket connection'
			}));
		}
	}

	function reconnect() {
		if (reconnectTimeout) {
			clearTimeout(reconnectTimeout);
		}

		update((state) => {
			const attempts = state.reconnectAttempts + 1;

			if (attempts > MAX_RECONNECT_ATTEMPTS) {
				return {
					...state,
					status: 'error',
					lastError: 'Max reconnection attempts reached'
				};
			}

			const delay = calculateReconnectDelay(attempts);
			console.log(`Reconnecting in ${delay}ms (attempt ${attempts}/${MAX_RECONNECT_ATTEMPTS})`);

			reconnectTimeout = setTimeout(() => {
				connect();
			}, delay);

			return {
				...state,
				status: 'reconnecting',
				reconnectAttempts: attempts
			};
		});
	}

	function disconnect() {
		if (reconnectTimeout) {
			clearTimeout(reconnectTimeout);
			reconnectTimeout = null;
		}

		stopHeartbeat();

		if (ws) {
			ws.close(1000, 'Client disconnect');
			ws = null;
		}

		update((state) => ({
			...state,
			status: 'disconnected',
			reconnectAttempts: 0
		}));
	}

	function send<T>(type: string, data: T, channel?: string) {
		if (ws?.readyState === WebSocket.OPEN) {
			ws.send(
				JSON.stringify({
					type,
					channel,
					data,
					timestamp: Date.now()
				})
			);
		} else {
			console.warn('WebSocket not connected - message not sent');
		}
	}

	function subscribeChannel(channel: string) {
		subscriptions.add(channel);

		if (ws?.readyState === WebSocket.OPEN) {
			ws.send(JSON.stringify({ type: 'subscribe', channel }));
		}
	}

	function unsubscribeChannel(channel: string) {
		subscriptions.delete(channel);

		if (ws?.readyState === WebSocket.OPEN) {
			ws.send(JSON.stringify({ type: 'unsubscribe', channel }));
		}
	}

	function on<T>(type: string, callback: (data: T) => void): () => void {
		if (!listeners.has(type)) {
			listeners.set(type, new Set());
		}
		listeners.get(type)!.add(callback as (data: unknown) => void);

		// Return unsubscribe function
		return () => {
			listeners.get(type)?.delete(callback as (data: unknown) => void);
		};
	}

	function off(type: string, callback?: (data: unknown) => void) {
		if (callback) {
			listeners.get(type)?.delete(callback);
		} else {
			listeners.delete(type);
		}
	}

	return {
		subscribe,
		connect,
		disconnect,
		reconnect,
		send,
		subscribeChannel,
		unsubscribeChannel,
		on,
		off,

		// Convenience method to check if connected
		get isConnected() {
			return ws?.readyState === WebSocket.OPEN;
		}
	};
}

export const websocketStore = createWebSocketStore();

// Derived stores for convenience
export const wsStatus = derived(websocketStore, ($ws) => $ws.status);
export const wsLatency = derived(websocketStore, ($ws) => $ws.latency);
export const wsLastMessage = derived(websocketStore, ($ws) => $ws.lastMessage);
export const wsIsConnected = derived(websocketStore, ($ws) => $ws.status === 'connected');

// Real-time dashboard data store
interface DashboardRealtimeData {
	activeUsers: number;
	todayRevenue: number;
	newSignups: number;
	activeSubscriptions: number;
	lastUpdated: Date | null;
}

function createDashboardRealtimeStore() {
	const { subscribe, set, update } = writable<DashboardRealtimeData>({
		activeUsers: 0,
		todayRevenue: 0,
		newSignups: 0,
		activeSubscriptions: 0,
		lastUpdated: null
	});

	let unsubscribe: (() => void) | null = null;

	function init() {
		if (!browser) return;

		// Subscribe to dashboard channel
		websocketStore.subscribeChannel('dashboard');

		// Listen for dashboard updates
		unsubscribe = websocketStore.on<DashboardRealtimeData>('dashboard:update', (data) => {
			update((state) => ({
				...state,
				...data,
				lastUpdated: new Date()
			}));
		});
	}

	function destroy() {
		websocketStore.unsubscribeChannel('dashboard');
		if (unsubscribe) {
			unsubscribe();
			unsubscribe = null;
		}
	}

	return {
		subscribe,
		init,
		destroy
	};
}

export const dashboardRealtime = createDashboardRealtimeStore();

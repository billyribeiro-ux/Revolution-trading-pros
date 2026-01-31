/**
 * Yjs Provider - Real-time Collaborative Document Synchronization
 * ═══════════════════════════════════════════════════════════════════════════
 * Production-grade Yjs CRDT integration for collaborative blog editing
 *
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+ | Svelte 5 Runes Syntax
 *
 * Features:
 * - Y-WebSocket for real-time synchronization
 * - Y-IndexedDB for offline persistence
 * - Automatic reconnection with exponential backoff
 * - Graceful connection/disconnection handling
 * - Document state synchronization
 */

import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';
import { IndexeddbPersistence } from 'y-indexeddb';
import { browser } from '$app/environment';
import { WS_URL } from '$lib/api/config';
import type { Block } from '../types';

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/** Base reconnection delay in milliseconds */
const BASE_RECONNECT_DELAY = 1000;

/** Maximum reconnection delay in milliseconds (30 seconds) */
const MAX_RECONNECT_DELAY = 30000;

/** Maximum reconnection attempts before giving up */
const MAX_RECONNECT_ATTEMPTS = 15;

/** Reconnection jitter factor to prevent thundering herd */
const JITTER_FACTOR = 0.3;

/** Sync timeout in milliseconds */
const SYNC_TIMEOUT = 10000;

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/** Connection status enumeration */
export type ConnectionStatus =
	| 'disconnected'
	| 'connecting'
	| 'connected'
	| 'syncing'
	| 'synced'
	| 'error';

/** Yjs provider state */
export interface YjsProviderState {
	/** Current connection status */
	status: ConnectionStatus;
	/** Whether the document is synced with the server */
	isSynced: boolean;
	/** Whether the document is persisted locally */
	isPersisted: boolean;
	/** Number of reconnection attempts */
	reconnectAttempts: number;
	/** Last sync timestamp */
	lastSyncAt: number | null;
	/** Any connection error */
	error: Error | null;
}

/** Yjs provider options */
export interface YjsProviderOptions {
	/** Room/document identifier (typically post ID) */
	roomId: string;
	/** Optional user information for awareness */
	user?: {
		id: string;
		name: string;
		color: string;
		avatar?: string;
	};
	/** Whether to enable offline persistence */
	enablePersistence?: boolean;
	/** WebSocket URL override */
	wsUrl?: string;
	/** Callback when connection status changes */
	onStatusChange?: (status: ConnectionStatus) => void;
	/** Callback when sync completes */
	onSync?: () => void;
	/** Callback when an error occurs */
	onError?: (error: Error) => void;
}

/** Provider instance interface */
export interface YjsProviderInstance {
	/** The Yjs document */
	doc: Y.Doc;
	/** The Y.Array for blocks */
	yBlocks: Y.Array<Block>;
	/** The Y.Map for document metadata */
	yMeta: Y.Map<any>;
	/** WebSocket provider */
	wsProvider: WebsocketProvider | null;
	/** IndexedDB persistence */
	persistence: IndexeddbPersistence | null;
	/** Current state */
	state: YjsProviderState;
	/** Connect to the server */
	connect: () => void;
	/** Disconnect from the server */
	disconnect: () => void;
	/** Force reconnection */
	reconnect: () => void;
	/** Destroy the provider and cleanup */
	destroy: () => void;
	/** Get the awareness instance */
	getAwareness: () => any;
}

// ═══════════════════════════════════════════════════════════════════════════════
// YJS PROVIDER FACTORY
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates a Yjs provider instance for collaborative document editing.
 *
 * @param options - Provider configuration options
 * @returns YjsProviderInstance with document and connection management
 *
 * @example
 * ```typescript
 * const provider = createYjsProvider({
 *   roomId: 'post-123',
 *   user: { id: 'user-1', name: 'John', color: '#ff0000' },
 *   enablePersistence: true,
 *   onSync: () => console.log('Synced!')
 * });
 *
 * // Access the blocks array
 * const blocks = provider.yBlocks.toArray();
 *
 * // Cleanup on unmount
 * provider.destroy();
 * ```
 */
export function createYjsProvider(options: YjsProviderOptions): YjsProviderInstance {
	const {
		roomId,
		user,
		enablePersistence = true,
		wsUrl = WS_URL,
		onStatusChange,
		onSync,
		onError
	} = options;

	// ═══════════════════════════════════════════════════════════════════════════
	// DOCUMENT SETUP
	// ═══════════════════════════════════════════════════════════════════════════

	/** Yjs document instance */
	const doc = new Y.Doc();

	/** Shared array for blocks - the core collaborative data structure */
	const yBlocks = doc.getArray<Block>('blocks');

	/** Shared map for document metadata (title, settings, etc.) */
	const yMeta = doc.getMap<any>('meta');

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE MANAGEMENT
	// ═══════════════════════════════════════════════════════════════════════════

	/** Provider state (mutable) */
	let state: YjsProviderState = {
		status: 'disconnected',
		isSynced: false,
		isPersisted: false,
		reconnectAttempts: 0,
		lastSyncAt: null,
		error: null
	};

	/** WebSocket provider reference */
	let wsProvider: WebsocketProvider | null = null;

	/** IndexedDB persistence reference */
	let persistence: IndexeddbPersistence | null = null;

	/** Reconnection timer */
	let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

	/** Manual disconnect flag */
	let isManualDisconnect = false;

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Update state and notify listeners
	 */
	function updateState(updates: Partial<YjsProviderState>): void {
		state = { ...state, ...updates };

		if (updates.status && onStatusChange) {
			onStatusChange(updates.status);
		}

		if (updates.error && onError) {
			onError(updates.error);
		}
	}

	/**
	 * Calculate reconnection delay with exponential backoff
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
	 * Get the WebSocket URL for the collaboration room
	 */
	function getWebSocketUrl(): string {
		// Convert HTTPS to WSS, HTTP to WS
		const baseUrl = wsUrl.replace(/^http/, 'ws');

		// Construct the collaboration endpoint
		// Format: wss://domain/api/ws/collab/{roomId}
		return `${baseUrl}/api/ws/collab/${roomId}`;
	}

	/**
	 * Clear reconnection timer
	 */
	function clearReconnectTimer(): void {
		if (reconnectTimer) {
			clearTimeout(reconnectTimer);
			reconnectTimer = null;
		}
	}

	/**
	 * Schedule reconnection with backoff
	 */
	function scheduleReconnect(): void {
		if (isManualDisconnect) return;
		if (state.reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
			updateState({
				status: 'error',
				error: new Error('Maximum reconnection attempts reached')
			});
			console.error('[YjsProvider] Max reconnection attempts reached');
			return;
		}

		const delay = getReconnectDelay();
		updateState({
			status: 'connecting',
			reconnectAttempts: state.reconnectAttempts + 1
		});

		console.log(
			`[YjsProvider] Reconnecting in ${Math.round(delay)}ms (attempt ${state.reconnectAttempts}/${MAX_RECONNECT_ATTEMPTS})`
		);

		reconnectTimer = setTimeout(() => {
			connect();
		}, delay);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PERSISTENCE SETUP
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Initialize IndexedDB persistence
	 */
	function initPersistence(): void {
		if (!browser || !enablePersistence) return;

		try {
			persistence = new IndexeddbPersistence(`collab-${roomId}`, doc);

			persistence.on('synced', () => {
				console.log('[YjsProvider] IndexedDB synced');
				updateState({ isPersisted: true });
			});

			persistence.whenSynced.then(() => {
				console.log('[YjsProvider] IndexedDB initial sync complete');
			}).catch((err: unknown) => {
				console.warn('[YjsProvider] IndexedDB sync failed:', err);
			});
		} catch (err) {
			console.warn('[YjsProvider] Failed to initialize IndexedDB persistence:', err);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// WEBSOCKET PROVIDER SETUP
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Initialize WebSocket provider
	 */
	function initWebSocketProvider(): void {
		if (!browser) return;

		try {
			const wsUrlFull = getWebSocketUrl();

			console.log('[YjsProvider] Connecting to:', wsUrlFull);

			wsProvider = new WebsocketProvider(
				wsUrlFull,
				roomId,
				doc,
				{
					connect: true,
					params: user ? { userId: user.id } : {},
					// WebSocket connection options
					WebSocketPolyfill: undefined,
					resyncInterval: 30000, // Resync every 30 seconds
					maxBackoffTime: MAX_RECONNECT_DELAY,
					disableBc: false // Enable broadcast channel for same-origin tabs
				}
			);

			// Set up awareness with user info
			if (user) {
				wsProvider.awareness.setLocalStateField('user', {
					id: user.id,
					name: user.name,
					color: user.color,
					avatar: user.avatar,
					cursor: null,
					selection: null,
					lastActive: Date.now()
				});
			}

			// Connection status handlers
			wsProvider.on('status', (event: { status: string }) => {
				console.log('[YjsProvider] Status:', event.status);

				if (event.status === 'connected') {
					updateState({
						status: 'syncing',
						reconnectAttempts: 0,
						error: null
					});
				} else if (event.status === 'disconnected') {
					updateState({
						status: 'disconnected',
						isSynced: false
					});

					if (!isManualDisconnect) {
						scheduleReconnect();
					}
				}
			});

			// Sync status handler
			wsProvider.on('sync', (isSynced: boolean) => {
				console.log('[YjsProvider] Sync status:', isSynced);

				if (isSynced) {
					updateState({
						status: 'synced',
						isSynced: true,
						lastSyncAt: Date.now()
					});

					if (onSync) {
						onSync();
					}
				}
			});

			// Error handler
			wsProvider.on('connection-error', (event: Event) => {
				console.error('[YjsProvider] Connection error:', event);
				updateState({
					status: 'error',
					error: new Error('WebSocket connection error')
				});
			});

			// Connection close handler
			wsProvider.on('connection-close', (event: CloseEvent | null, _provider: WebsocketProvider) => {
				console.log('[YjsProvider] Connection closed:', event?.code, event?.reason);

				if (!isManualDisconnect && event?.code !== 1000) {
					scheduleReconnect();
				}
			});

			updateState({ status: 'connecting' });
		} catch (err) {
			console.error('[YjsProvider] Failed to create WebSocket provider:', err);
			updateState({
				status: 'error',
				error: err instanceof Error ? err : new Error('Failed to connect')
			});
			scheduleReconnect();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PUBLIC METHODS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Connect to the collaboration server
	 */
	function connect(): void {
		if (!browser) return;

		isManualDisconnect = false;
		clearReconnectTimer();

		// Initialize persistence if not already done
		if (!persistence && enablePersistence) {
			initPersistence();
		}

		// Initialize WebSocket provider
		if (!wsProvider) {
			initWebSocketProvider();
		} else if (!wsProvider.wsconnected) {
			wsProvider.connect();
		}
	}

	/**
	 * Disconnect from the collaboration server
	 */
	function disconnect(): void {
		isManualDisconnect = true;
		clearReconnectTimer();

		if (wsProvider) {
			wsProvider.disconnect();
		}

		updateState({
			status: 'disconnected',
			isSynced: false
		});

		console.log('[YjsProvider] Disconnected');
	}

	/**
	 * Force reconnection
	 */
	function reconnect(): void {
		disconnect();

		// Small delay before reconnecting
		setTimeout(() => {
			isManualDisconnect = false;
			updateState({ reconnectAttempts: 0 });
			connect();
		}, 100);
	}

	/**
	 * Destroy the provider and cleanup all resources
	 */
	function destroy(): void {
		console.log('[YjsProvider] Destroying provider');

		isManualDisconnect = true;
		clearReconnectTimer();

		// Disconnect and destroy WebSocket provider
		if (wsProvider) {
			wsProvider.destroy();
			wsProvider = null;
		}

		// Destroy persistence
		if (persistence) {
			persistence.destroy();
			persistence = null;
		}

		// Destroy the document
		doc.destroy();

		updateState({
			status: 'disconnected',
			isSynced: false,
			isPersisted: false
		});
	}

	/**
	 * Get the awareness instance for presence tracking
	 */
	function getAwareness(): any {
		return wsProvider?.awareness ?? null;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PROVIDER INSTANCE
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		doc,
		yBlocks,
		yMeta,
		get wsProvider() { return wsProvider; },
		get persistence() { return persistence; },
		get state() { return state; },
		connect,
		disconnect,
		reconnect,
		destroy,
		getAwareness
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Convert a Y.Array to a regular array with type safety
 */
export function yArrayToArray<T>(yArray: Y.Array<T>): T[] {
	return yArray.toArray();
}

/**
 * Convert a Y.Map to a plain object
 */
export function yMapToObject<T extends Record<string, any>>(yMap: Y.Map<any>): T {
	const result: Record<string, any> = {};
	yMap.forEach((value: any, key: string) => {
		result[key] = value;
	});
	return result as T;
}

/**
 * Generate a unique client ID for this session
 */
export function generateClientId(): string {
	return `${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
}

/**
 * Generate a random user color for presence
 */
export function generateUserColor(): string {
	const colors = [
		'#f87171', // red
		'#fb923c', // orange
		'#fbbf24', // amber
		'#a3e635', // lime
		'#34d399', // emerald
		'#22d3ee', // cyan
		'#60a5fa', // blue
		'#a78bfa', // violet
		'#f472b6', // pink
		'#e879f9'  // fuchsia
	];
	return colors[Math.floor(Math.random() * colors.length)];
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE EXPORT
// ═══════════════════════════════════════════════════════════════════════════════

export type { Y };

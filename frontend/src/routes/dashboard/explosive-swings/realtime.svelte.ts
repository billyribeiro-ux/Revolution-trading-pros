/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Real-Time Updates Integration (Phase 3)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description WebSocket integration for real-time trading alerts on Explosive Swings
 * @version 2.0.0 - January 2026 (Phase 3 WebSocket Implementation)
 * @standards Apple Principal Engineer ICT Level 7+ | Svelte 5 Runes Syntax
 *
 * This module bridges the WebSocket service with the page state, providing:
 * - Automatic alert updates (new alerts appear instantly)
 * - Trade status updates (closed trades, P&L updates)
 * - Stats refresh (win rate, active trades count)
 * - Visual notifications for new content
 *
 * Built for the next 10 years with extensibility in mind.
 */

import { browser } from '$app/environment';
import {
	createWebSocketService,
	type AlertPayload,
	type TradePayload,
	type StatsPayload,
	type TradePlanPayload,
	type WsMessage,
	type WebSocketService
} from '$lib/services/websocket.svelte';
import { formatTimeAgo } from './utils/formatters';

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/** Room slug for Explosive Swings */
const ROOM_SLUG = 'explosive-swings';

/** Duration to show "new" indicator on alerts (ms) */
const NEW_ALERT_INDICATOR_DURATION = 30000;

/** Sound notification enabled by default */
const ENABLE_SOUND_NOTIFICATIONS = false;

// Re-export payload types for convenience
export type { AlertPayload, TradePayload, StatsPayload, TradePlanPayload };

// ═══════════════════════════════════════════════════════════════════════════════
// REAL-TIME STATE
// ═══════════════════════════════════════════════════════════════════════════════

export interface RealtimeState {
	/** Whether WebSocket is connected */
	isConnected: boolean;
	/** Whether WebSocket is reconnecting */
	isReconnecting: boolean;
	/** IDs of alerts that arrived via WebSocket (for "new" indicator) */
	realtimeAlertIds: Set<number>;
	/** Last error from WebSocket */
	error: Error | null;
	/** Count of unread alerts since last interaction */
	unreadCount: number;
	/** Latest received alert */
	latestAlert: AlertPayload | null;
	/** Latest received trade update */
	latestTrade: TradePayload | null;
	/** Latest received stats */
	latestStats: StatsPayload | null;
	/** Recent alerts for notification display */
	recentAlerts: AlertPayload[];
}

// ═══════════════════════════════════════════════════════════════════════════════
// TOAST NOTIFICATIONS
// ═══════════════════════════════════════════════════════════════════════════════

/** Toast notification types */
type ToastType = 'alert' | 'trade' | 'stats' | 'error' | 'info';

interface ToastOptions {
	type: ToastType;
	title: string;
	message: string;
	duration?: number;
}

/**
 * Show a toast notification
 * This can be replaced with your preferred toast library
 */
function showToast(options: ToastOptions): void {
	if (!browser) return;

	// Check if the browser supports notifications and user has granted permission
	if ('Notification' in window && Notification.permission === 'granted') {
		// Use browser notifications for important alerts when tab is not focused
		if (document.hidden && options.type === 'alert') {
			new Notification(options.title, {
				body: options.message,
				icon: '/favicon.png',
				tag: 'explosive-swings-alert'
			});
		}
	}

	// Dispatch custom event for toast UI component
	const event = new CustomEvent('show-toast', {
		detail: {
			...options,
			id: crypto.randomUUID(),
			timestamp: Date.now()
		}
	});
	window.dispatchEvent(event);

	// Console log for development
	console.log(`[Toast] ${options.type.toUpperCase()}: ${options.title} - ${options.message}`);
}

/**
 * Play notification sound
 */
function playNotificationSound(): void {
	if (!browser || !ENABLE_SOUND_NOTIFICATIONS) return;

	try {
		const audio = new Audio('/sounds/notification.mp3');
		audio.volume = 0.5;
		audio.play().catch(() => {
			// Ignore autoplay errors
		});
	} catch {
		// Ignore audio errors
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// FORMATTED ALERT TYPE (for page state integration)
// ═══════════════════════════════════════════════════════════════════════════════

export interface FormattedAlert {
	id: number;
	type: 'ENTRY' | 'EXIT' | 'UPDATE';
	ticker: string;
	title: string;
	time: string;
	message: string;
	isNew: boolean;
	notes: string;
	tosString?: string;
}

/**
 * Format alert payload to page state format
 */
function formatAlertFromPayload(payload: AlertPayload): FormattedAlert {
	return {
		id: payload.id,
		type: payload.alert_type as 'ENTRY' | 'EXIT' | 'UPDATE',
		ticker: payload.ticker,
		title: payload.title || payload.ticker,
		time: formatTimeAgo(payload.published_at),
		message: payload.message,
		isNew: payload.is_new,
		notes: payload.notes || '',
		tosString: payload.tos_string || undefined
	};
}

// formatTimeAgo imported from './utils/formatters' - ICT 7 Single Source of Truth

// ═══════════════════════════════════════════════════════════════════════════════
// PAGE STATE INTERFACE (for type safety)
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Interface for page state methods used by realtime updates
 */
export interface PageStateForRealtime {
	prependAlert?: (alert: FormattedAlert) => void;
	updateAlert?: (alert: FormattedAlert) => void;
	removeAlert?: (alertId: number) => void;
	fetchAllTrades?: () => void | Promise<void>;
	fetchStats?: () => void | Promise<void>;
	fetchTradePlan?: () => void | Promise<void>;
	setStats?: (stats: {
		winRate: number;
		weeklyProfit: string;
		activeTrades: number;
		closedThisWeek: number;
	}) => void;
}

// ═══════════════════════════════════════════════════════════════════════════════
// REAL-TIME UPDATES FACTORY
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates real-time updates integration for the Explosive Swings page.
 *
 * @param pageState - The page state object from createPageState() (optional)
 * @returns Real-time state and control methods
 *
 * @example
 * ```typescript
 * const ps = createPageState();
 * const realtime = createRealtimeState('explosive-swings');
 *
 * // Set up callbacks for page state integration
 * realtime.setOnAlertReceived((alert) => ps.prependAlert(formatAlert(alert)));
 *
 * $effect(() => {
 *   realtime.connect();
 *   return () => realtime.disconnect();
 * });
 * ```
 */
export function createRealtimeState(roomSlug: string = ROOM_SLUG) {
	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let state = $state<RealtimeState>({
		isConnected: false,
		isReconnecting: false,
		realtimeAlertIds: new Set(),
		error: null,
		unreadCount: 0,
		latestAlert: null,
		latestTrade: null,
		latestStats: null,
		recentAlerts: []
	});

	// WebSocket service instance
	let ws: WebSocketService | null = null;
	let unsubscribe: (() => void) | null = null;
	
	// ICT 7 Fix: Track sync interval to prevent memory leaks
	let syncIntervalId: ReturnType<typeof setInterval> | null = null;

	// Timer for clearing "new" indicators
	const newAlertTimers = new Map<number, ReturnType<typeof setTimeout>>();

	// External callbacks for integration
	let onAlertReceived: ((alert: AlertPayload) => void) | null = null;
	let onTradeReceived: ((trade: TradePayload) => void) | null = null;
	let onStatsReceived: ((stats: StatsPayload) => void) | null = null;

	// Page state reference (optional)
	let pageStateRef: PageStateForRealtime | null = null;

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const hasRealtimeUpdates = $derived(state.unreadCount > 0);
	const totalNewNotifications = $derived(state.unreadCount);

	// ═══════════════════════════════════════════════════════════════════════════
	// MESSAGE HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Handle new alert created
	 */
	function handleAlertCreated(payload: AlertPayload): void {
		// Only process alerts for our room
		if (payload.room_slug !== roomSlug) return;

		state.latestAlert = payload;

		// Add to realtime alert IDs (for "new" indicator)
		state.realtimeAlertIds.add(payload.id);

		// Add to recent alerts (keep last 10)
		state.recentAlerts = [payload, ...state.recentAlerts.slice(0, 9)];

		// Schedule removal of "new" indicator
		const timer = setTimeout(() => {
			state.realtimeAlertIds.delete(payload.id);
			newAlertTimers.delete(payload.id);
		}, NEW_ALERT_INDICATOR_DURATION);
		newAlertTimers.set(payload.id, timer);

		// Prepend to page state if available
		if (pageStateRef?.prependAlert) {
			const formattedAlert = formatAlertFromPayload(payload);
			pageStateRef.prependAlert(formattedAlert);
		}

		// Call external callback
		if (onAlertReceived) {
			onAlertReceived(payload);
		}

		// Increment unread count
		state.unreadCount++;

		// Show toast notification
		showToast({
			type: 'alert',
			title: `New ${payload.alert_type} Alert`,
			message: `${payload.ticker}: ${payload.message.slice(0, 100)}...`,
			duration: 5000
		});

		// Play notification sound
		playNotificationSound();
	}

	/**
	 * Handle alert updated
	 */
	function handleAlertUpdated(payload: AlertPayload): void {
		if (payload.room_slug !== roomSlug) return;

		if (pageStateRef?.updateAlert) {
			const formattedAlert = formatAlertFromPayload(payload);
			pageStateRef.updateAlert(formattedAlert);
		}
	}

	/**
	 * Handle alert deleted
	 */
	function handleAlertDeleted(alertId: number): void {
		if (pageStateRef?.removeAlert) {
			pageStateRef.removeAlert(alertId);
		}

		// Clean up timers
		const timer = newAlertTimers.get(alertId);
		if (timer) {
			clearTimeout(timer);
			newAlertTimers.delete(alertId);
		}
		state.realtimeAlertIds.delete(alertId);
	}

	/**
	 * Handle trade created
	 */
	function handleTradeCreated(payload: TradePayload): void {
		if (payload.room_slug !== roomSlug) return;

		state.latestTrade = payload;

		// Refresh trades list
		if (pageStateRef?.fetchAllTrades) {
			pageStateRef.fetchAllTrades();
		}

		// Call external callback
		if (onTradeReceived) {
			onTradeReceived(payload);
		}

		showToast({
			type: 'trade',
			title: 'New Trade Opened',
			message: `${payload.ticker} ${payload.direction} @ $${payload.entry_price.toFixed(2)}`,
			duration: 4000
		});
	}

	/**
	 * Handle trade closed
	 */
	function handleTradeClosed(payload: TradePayload): void {
		if (payload.room_slug !== roomSlug) return;

		state.latestTrade = payload;

		// Refresh trades and stats
		if (pageStateRef?.fetchAllTrades) {
			pageStateRef.fetchAllTrades();
		}
		if (pageStateRef?.fetchStats) {
			pageStateRef.fetchStats();
		}

		// Call external callback
		if (onTradeReceived) {
			onTradeReceived(payload);
		}

		const pnl = payload.pnl_percent;
		const resultIcon = payload.result === 'win' ? '+' : payload.result === 'loss' ? '' : '~';

		showToast({
			type: 'trade',
			title: `Trade Closed: ${payload.result?.toUpperCase() || 'CLOSED'}`,
			message: `${payload.ticker} ${resultIcon}${pnl?.toFixed(2) || 0}%`,
			duration: 5000
		});
	}

	/**
	 * Handle trade updated
	 */
	function handleTradeUpdated(payload: TradePayload): void {
		if (payload.room_slug !== roomSlug) return;

		state.latestTrade = payload;

		// Refresh trades
		if (pageStateRef?.fetchAllTrades) {
			pageStateRef.fetchAllTrades();
		}

		// Call external callback
		if (onTradeReceived) {
			onTradeReceived(payload);
		}

		if (payload.was_updated) {
			showToast({
				type: 'trade',
				title: 'Position Updated',
				message: `${payload.ticker} position has been adjusted`,
				duration: 4000
			});
		}
	}

	/**
	 * Handle trade invalidated
	 */
	function handleTradeInvalidated(payload: TradePayload): void {
		if (payload.room_slug !== roomSlug) return;

		state.latestTrade = payload;

		// Refresh trades and stats
		if (pageStateRef?.fetchAllTrades) {
			pageStateRef.fetchAllTrades();
		}
		if (pageStateRef?.fetchStats) {
			pageStateRef.fetchStats();
		}

		showToast({
			type: 'info',
			title: 'Trade Invalidated',
			message: `${payload.ticker}: ${payload.invalidation_reason || 'Trade invalidated'}`,
			duration: 5000
		});
	}

	/**
	 * Handle stats updated
	 */
	function handleStatsUpdated(payload: StatsPayload): void {
		if (payload.room_slug !== roomSlug) return;

		state.latestStats = payload;

		if (pageStateRef?.setStats) {
			pageStateRef.setStats({
				winRate: payload.win_rate ?? 0,
				weeklyProfit: payload.weekly_profit ?? '0%',
				activeTrades: payload.active_trades ?? 0,
				closedThisWeek: payload.closed_this_week ?? 0
			});
		}

		// Call external callback
		if (onStatsReceived) {
			onStatsReceived(payload);
		}
	}

	/**
	 * Handle trade plan events
	 */
	function handleTradePlanEvent(): void {
		if (pageStateRef?.fetchTradePlan) {
			pageStateRef.fetchTradePlan();
		}
	}

	/**
	 * Main message handler
	 */
	function handleMessage(message: WsMessage): void {
		switch (message.type) {
			case 'AlertCreated':
				handleAlertCreated(message.payload);
				break;

			case 'AlertUpdated':
				handleAlertUpdated(message.payload);
				break;

			case 'AlertDeleted':
				handleAlertDeleted(message.payload.alert_id);
				break;

			case 'TradeCreated':
				handleTradeCreated(message.payload);
				break;

			case 'TradeClosed':
				handleTradeClosed(message.payload);
				break;

			case 'TradeUpdated':
				handleTradeUpdated(message.payload);
				break;

			case 'TradeInvalidated':
				handleTradeInvalidated(message.payload);
				break;

			case 'StatsUpdated':
				handleStatsUpdated(message.payload);
				break;

			case 'TradePlanCreated':
			case 'TradePlanUpdated':
			case 'TradePlanDeleted':
				handleTradePlanEvent();
				break;

			case 'Error':
				state.error = new Error(message.payload.message);
				showToast({
					type: 'error',
					title: 'Connection Error',
					message: message.payload.message,
					duration: 5000
				});
				break;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// CONNECTION STATE SYNC
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Sync WebSocket connection state to local state
	 */
	function syncConnectionState(): void {
		if (ws) {
			state.isConnected = ws.isConnected;
			state.isReconnecting = ws.isReconnecting;
			state.error = ws.error;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PUBLIC METHODS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Connect to WebSocket server and start receiving updates
	 * @description ICT 7 Fix: Prevents memory leaks by clearing existing intervals before creating new ones
	 */
	function connect(): void {
		if (!browser) return;
		
		// ICT 7 Fix: Prevent duplicate connections and memory leaks
		if (state.isConnected || state.isReconnecting) {
			console.warn('[realtime] Already connected or reconnecting, skipping connect()');
			return;
		}

		// Create WebSocket service if not exists
		if (!ws) {
			ws = createWebSocketService(roomSlug);
		}

		// Connect
		ws.connect();

		// Subscribe to messages
		unsubscribe = ws.subscribe(handleMessage);

		// ICT 7 Fix: Clear existing interval before creating new one to prevent memory leaks
		if (syncIntervalId) {
			clearInterval(syncIntervalId);
			syncIntervalId = null;
		}
		
		// Set up connection state sync
		syncIntervalId = setInterval(syncConnectionState, 1000);

		// Store cleanup for disconnect
		const originalUnsubscribe = unsubscribe;
		unsubscribe = () => {
			originalUnsubscribe();
			if (syncIntervalId) {
				clearInterval(syncIntervalId);
				syncIntervalId = null;
			}
		};

		// Request notification permission
		if ('Notification' in window && Notification.permission === 'default') {
			Notification.requestPermission();
		}
	}

	/**
	 * Disconnect from WebSocket server
	 * @description ICT 7 Fix: Ensures all intervals and timers are properly cleaned up
	 */
	function disconnect(): void {
		// Unsubscribe from messages
		if (unsubscribe) {
			unsubscribe();
			unsubscribe = null;
		}
		
		// ICT 7 Fix: Explicitly clear sync interval
		if (syncIntervalId) {
			clearInterval(syncIntervalId);
			syncIntervalId = null;
		}

		// Disconnect WebSocket
		if (ws) {
			ws.disconnect();
			ws = null;
		}

		// Clear timers
		newAlertTimers.forEach((timer) => clearTimeout(timer));
		newAlertTimers.clear();

		// Reset state
		state.isConnected = false;
		state.isReconnecting = false;
		state.realtimeAlertIds.clear();
	}

	/**
	 * Mark all alerts as read (clear unread count)
	 */
	function acknowledgeAlerts(): void {
		state.unreadCount = 0;
	}

	/**
	 * Clear all recent alerts
	 */
	function clearRecentAlerts(): void {
		state.recentAlerts = [];
		state.latestAlert = null;
	}

	/**
	 * Check if an alert arrived via real-time (for "new" indicator)
	 */
	function isRealtimeAlert(alertId: number): boolean {
		return state.realtimeAlertIds.has(alertId);
	}

	/**
	 * Force reconnection
	 */
	function reconnect(): void {
		if (ws) {
			ws.reconnect();
		}
	}

	/**
	 * Set page state reference for automatic updates
	 */
	function setPageState(pageState: PageStateForRealtime): void {
		pageStateRef = pageState;
	}

	/**
	 * Register callback for new alerts
	 */
	function setOnAlertReceived(callback: (alert: AlertPayload) => void): void {
		onAlertReceived = callback;
	}

	/**
	 * Register callback for trade updates
	 */
	function setOnTradeReceived(callback: (trade: TradePayload) => void): void {
		onTradeReceived = callback;
	}

	/**
	 * Register callback for stats updates
	 */
	function setOnStatsReceived(callback: (stats: StatsPayload) => void): void {
		onStatsReceived = callback;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// Connection state (read-only)
		get isConnected() {
			return state.isConnected;
		},
		get isReconnecting() {
			return state.isReconnecting;
		},
		get connectionError() {
			return state.error?.message || null;
		},
		get error() {
			return state.error;
		},

		// Real-time data (read-only)
		get latestAlert() {
			return state.latestAlert;
		},
		get latestTrade() {
			return state.latestTrade;
		},
		get latestStats() {
			return state.latestStats;
		},
		get recentAlerts() {
			return state.recentAlerts;
		},

		// Notification state (read-only)
		get unreadCount() {
			return state.unreadCount;
		},
		get alertCount() {
			return state.unreadCount;
		},
		get hasNewAlert() {
			return state.unreadCount > 0;
		},
		get hasNewTrade() {
			return state.latestTrade !== null;
		},

		// Derived state (read-only)
		get hasRealtimeUpdates() {
			return hasRealtimeUpdates;
		},
		get totalNewNotifications() {
			return totalNewNotifications;
		},

		// Actions
		connect,
		disconnect,
		reconnect,
		acknowledgeAlerts,
		acknowledgeTrades: acknowledgeAlerts, // Alias for backward compatibility
		clearRecentAlerts,
		isRealtimeAlert,

		// Page state integration
		setPageState,

		// Callback registration
		setOnAlertReceived,
		setOnTradeReceived,
		setOnStatsReceived,

		// Constants
		roomSlug
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

export type RealtimeStateReturn = ReturnType<typeof createRealtimeState>;

// Backward compatibility alias
export type RealtimeState_v1 = RealtimeStateReturn;

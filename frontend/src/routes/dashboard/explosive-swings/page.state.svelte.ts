/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Page State Module
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Centralized state management using Svelte 5 runes
 * @version 1.0.0
 * @standards Svelte 5 January 2026 Syntax
 *
 * This module exports a factory function that creates all reactive state
 * for the Explosive Swings dashboard. Components consume this via context.
 */

import type {
	AlertFilter,
	TradePlanEntry,
	ActivePosition,
	ClosedTrade,
	WeeklyPerformance,
	WeeklyContent,
	ApiTrade,
	ApiWeeklyVideo,
	PaginationState,
	QuickStats
} from './types';
import { fallbackData, ROOM_SLUG, ALERTS_PER_PAGE } from './data/fallbacks';
import * as api from './page.api';
import type { FormattedAlert } from './page.api';
import type { RoomAlert } from '$lib/types/trading';
import type { TradePlanEntry as ApiTradePlanEntry } from '$lib/types/trading';
import {
	performanceMonitor,
	analyticsTracker,
	trackFilterApplied,
	trackPagination,
	trackModalOpened
} from './monitoring';

/**
 * Creates the reactive state store for the Explosive Swings dashboard.
 * Call this once in +page.svelte and provide via setContext if needed.
 */
export function createPageState() {
	// ═══════════════════════════════════════════════════════════════════════════
	// FILTER & PAGINATION STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let selectedFilter = $state<AlertFilter>('all');
	let currentPage = $state(1);
	let pagination = $state<PaginationState>({ total: 0, limit: ALERTS_PER_PAGE, offset: 0 });

	// ═══════════════════════════════════════════════════════════════════════════
	// ADMIN STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isAdmin = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// MODAL STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isAlertModalOpen = $state(false);
	let editingAlert = $state<RoomAlert | null>(null);
	let isTradeEntryModalOpen = $state(false);
	let editingTradeEntry = $state<ApiTradePlanEntry | null>(null);
	let isVideoUploadModalOpen = $state(false);
	let isClosePositionModalOpen = $state(false);
	let closingPosition = $state<ActivePosition | null>(null);
	let isAddTradeModalOpen = $state(false);
	let isUpdatePositionModalOpen = $state(false);
	let updatingPosition = $state<ActivePosition | null>(null);
	let isInvalidatePositionModalOpen = $state(false);
	let invalidatingPosition = $state<ActivePosition | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let apiAlerts = $state<FormattedAlert[]>([]);
	let apiTradePlan = $state<ApiTradePlanEntry[]>([]);
	let apiStats = $state<QuickStats | null>(null);
	let apiOpenTrades = $state<ApiTrade[]>([]);
	let apiClosedTrades = $state<ApiTrade[]>([]);
	let apiWeeklyVideo = $state<ApiWeeklyVideo | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// LOADING STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let isLoadingAlerts = $state(false);
	let isLoadingTradePlan = $state(false);
	let isLoadingStats = $state(false);
	let isLoadingTrades = $state(false);
	let isLoadingVideos = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// ERROR STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let alertsError = $state<string | null>(null);
	let tradePlanError = $state<string | null>(null);
	let statsError = $state<string | null>(null);
	let tradesError = $state<string | null>(null);
	let videosError = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// UI STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let expandedNotes = $state<Set<number>>(new Set());
	let copiedAlertId = $state<number | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const totalPages = $derived(Math.ceil(pagination.total / pagination.limit) || 1);
	const showingFrom = $derived(pagination.total > 0 ? (currentPage - 1) * ALERTS_PER_PAGE + 1 : 0);
	const showingTo = $derived(Math.min(currentPage * ALERTS_PER_PAGE, pagination.total));

	const alerts = $derived<FormattedAlert[]>(apiAlerts.length > 0 ? apiAlerts : fallbackData.alerts);

	const filteredAlerts = $derived(
		selectedFilter === 'all'
			? alerts
			: alerts.filter((a) => a.type.toLowerCase() === selectedFilter)
	);

	const tradePlan = $derived<TradePlanEntry[]>(
		apiTradePlan.length > 0
			? apiTradePlan.map((e) => ({
					ticker: e.ticker,
					bias: e.bias as 'BULLISH' | 'BEARISH' | 'NEUTRAL',
					entry: e.entry,
					target1: e.target1,
					target2: e.target2,
					target3: e.target3,
					runner: e.runner,
					stop: e.stop,
					optionsStrike: e.options_strike || '-',
					optionsExp: e.options_exp || '-',
					notes: e.notes || ''
				}))
			: fallbackData.tradePlan
	);

	const stats = $derived<QuickStats>(apiStats ?? fallbackData.stats);

	const weeklyPerformance = $derived<WeeklyPerformance>(
		apiStats
			? {
					winRate: apiStats.winRate,
					totalTrades: apiStats.activeTrades + apiStats.closedThisWeek,
					winningTrades: Math.round(
						(apiStats.winRate / 100) * (apiStats.activeTrades + apiStats.closedThisWeek)
					),
					avgWinPercent: 5.7,
					avgLossPercent: 2.1,
					riskRewardRatio: 2.7
				}
			: fallbackData.weeklyPerformance
	);

	const closedTrades = $derived<ClosedTrade[]>(
		apiClosedTrades.length > 0
			? apiClosedTrades.slice(0, 10).map((t) => ({
					id: String(t.id),
					ticker: t.ticker,
					percentageGain: t.pnl_percent ?? 0,
					isWinner: (t.pnl_percent ?? 0) > 0,
					closedAt: new Date(t.exit_date ?? t.entry_date),
					entryPrice: t.entry_price,
					exitPrice: t.exit_price ?? t.entry_price
				}))
			: fallbackData.closedTrades
	);

	const activePositions = $derived<ActivePosition[]>(
		apiOpenTrades.length > 0
			? apiOpenTrades.map((t) => ({
					id: String(t.id),
					ticker: t.ticker,
					status: 'ACTIVE' as const,
					entryPrice: t.entry_price,
					currentPrice: t.entry_price * 1.01,
					unrealizedPercent: 1.0,
					targets: [],
					stopLoss: { price: t.entry_price * 0.95, percentFromEntry: -5 },
					progressToTarget1: 0,
					triggeredAt: new Date(t.entry_date),
					notes: t.notes,
					wasUpdated: (t as any).was_updated ?? false,
					updatedAt: (t as any).updated_at ? new Date((t as any).updated_at) : undefined
				}))
			: fallbackData.activePositions
	);

	// Helper to safely validate dates before calling toLocaleDateString
	function isValidDate(dateValue: any): boolean {
		if (!dateValue) return false;
		const date = new Date(dateValue);
		return date instanceof Date && !isNaN(date.getTime());
	}

	const weeklyContent = $derived.by<WeeklyContent>(() => {
		// Use fallback if no video or invalid published_at
		if (!apiWeeklyVideo || !isValidDate(apiWeeklyVideo.published_at)) {
			return fallbackData.weeklyContent;
		}

		const publishedDate = new Date(apiWeeklyVideo.published_at);

		return {
			title:
				apiWeeklyVideo.week_title ||
				`Week of ${publishedDate.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}`,
			videoTitle: apiWeeklyVideo.video_title ?? '',
			videoUrl: apiWeeklyVideo.video_url ?? '',
			thumbnail: apiWeeklyVideo.thumbnail_url ?? fallbackData.weeklyContent.thumbnail,
			duration: apiWeeklyVideo.duration ?? '',
			publishedDate:
				publishedDate.toLocaleDateString('en-US', {
					month: 'long',
					day: 'numeric',
					year: 'numeric'
				}) +
				' at ' +
				publishedDate.toLocaleTimeString('en-US', {
					hour: 'numeric',
					minute: '2-digit',
					hour12: true
				}) +
				' ET'
		};
	});

	const hasAnyError = $derived(
		alertsError || tradePlanError || statsError || tradesError || videosError
	);

	const isInitialLoading = $derived(isLoadingAlerts && isLoadingStats && isLoadingTrades);

	// ═══════════════════════════════════════════════════════════════════════════
	// ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchAlerts() {
		isLoadingAlerts = true;
		alertsError = null;

		try {
			const result = await performanceMonitor.measureAsync('fetch-alerts', () =>
				api.fetchAlerts(ROOM_SLUG, currentPage, ALERTS_PER_PAGE)
			);
			apiAlerts = result.alerts;
			pagination = result.pagination;

			// Complete filter/pagination tracking if applicable
			analyticsTracker.trackFilterCompleted();
			analyticsTracker.trackPaginationCompleted();
		} catch (err) {
			alertsError = err instanceof Error ? err.message : 'Failed to load alerts';
			console.error('Failed to fetch alerts:', err);
			analyticsTracker.trackError('fetch_alerts', alertsError, { page: currentPage });
		} finally {
			isLoadingAlerts = false;
		}
	}

	async function fetchTradePlan() {
		isLoadingTradePlan = true;
		tradePlanError = null;

		try {
			const result = await api.fetchTradePlan(ROOM_SLUG);
			apiTradePlan = result as unknown as ApiTradePlanEntry[];
		} catch (err) {
			tradePlanError = err instanceof Error ? err.message : 'Failed to load trade plan';
			console.error('Failed to fetch trade plan:', err);
		} finally {
			isLoadingTradePlan = false;
		}
	}

	async function fetchStats() {
		isLoadingStats = true;
		statsError = null;

		try {
			apiStats = await performanceMonitor.measureAsync('fetch-stats', () =>
				api.fetchStats(ROOM_SLUG)
			);
		} catch (err) {
			statsError = err instanceof Error ? err.message : 'Failed to load stats';
			console.error('Failed to fetch stats:', err);
			analyticsTracker.trackError('fetch_stats', statsError);
		} finally {
			isLoadingStats = false;
		}
	}

	async function fetchAllTrades() {
		isLoadingTrades = true;
		tradesError = null;

		try {
			const trades = await performanceMonitor.measureAsync('fetch-trades', () =>
				api.fetchAllTrades(ROOM_SLUG)
			);
			apiOpenTrades = trades.filter((t) => t.status === 'open');
			apiClosedTrades = trades.filter((t) => t.status === 'closed');
		} catch (err) {
			tradesError = err instanceof Error ? err.message : 'Failed to load trades';
			console.error('Failed to fetch trades:', err);
			analyticsTracker.trackError('fetch_trades', tradesError);
		} finally {
			isLoadingTrades = false;
		}
	}

	async function fetchWeeklyVideo() {
		isLoadingVideos = true;
		videosError = null;

		try {
			apiWeeklyVideo = await api.fetchWeeklyVideo(ROOM_SLUG);
		} catch (err) {
			videosError = err instanceof Error ? err.message : 'Failed to load video';
			console.error('Failed to fetch weekly video:', err);
		} finally {
			isLoadingVideos = false;
		}
	}

	async function checkAdminStatus() {
		try {
			isAdmin = await api.checkAdminStatus();
		} catch {
			isAdmin = false;
		}
	}

	function setFilter(filter: AlertFilter) {
		if (filter === selectedFilter) return;
		trackFilterApplied('alert_type', filter);
		selectedFilter = filter;
		currentPage = 1;
		fetchAlerts();
	}

	async function goToPage(page: number) {
		if (page < 1 || page > totalPages || page === currentPage) return;
		trackPagination(page, totalPages);
		currentPage = page;
		await fetchAlerts();
	}

	function toggleNotes(alertId: number) {
		const newExpanded = new Set(expandedNotes);
		if (newExpanded.has(alertId)) {
			newExpanded.delete(alertId);
		} else {
			newExpanded.add(alertId);
		}
		expandedNotes = newExpanded;
	}

	async function copyTradeDetails(alert: FormattedAlert) {
		const details = `${alert.ticker} ${alert.type}\n${alert.title}\n${alert.message}${alert.tosString ? '\nTOS: ' + alert.tosString : ''}`;
		try {
			await navigator.clipboard.writeText(details);
			copiedAlertId = alert.id;
			setTimeout(() => {
				copiedAlertId = null;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy:', err);
		}
	}

	function initializeData() {
		// Start page load performance tracking
		performanceMonitor.startMark('page-load');
		analyticsTracker.trackPageView();

		checkAdminStatus();
		fetchAlerts();
		fetchTradePlan();
		fetchStats();
		fetchAllTrades();
		fetchWeeklyVideo();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// REAL-TIME UPDATE METHODS (ICT 7+ Phase 3)
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Prepend a new alert to the top of the alerts list
	 * Called by WebSocket real-time updates
	 */
	function prependAlert(alert: FormattedAlert) {
		// Check if alert already exists (avoid duplicates)
		if (apiAlerts.some((a) => a.id === alert.id)) {
			return;
		}
		apiAlerts = [alert, ...apiAlerts];

		// Update pagination total
		pagination = { ...pagination, total: pagination.total + 1 };

		// Track the real-time update
		analyticsTracker.trackEvent('realtime', 'alert_received', alert.type);
	}

	/**
	 * Update an existing alert in the list
	 * Called by WebSocket real-time updates
	 */
	function updateAlert(alert: FormattedAlert) {
		const index = apiAlerts.findIndex((a) => a.id === alert.id);
		if (index !== -1) {
			apiAlerts = [...apiAlerts.slice(0, index), alert, ...apiAlerts.slice(index + 1)];
		}
	}

	/**
	 * Remove an alert from the list
	 * Called by WebSocket real-time updates
	 */
	function removeAlert(alertId: number) {
		const index = apiAlerts.findIndex((a) => a.id === alertId);
		if (index !== -1) {
			apiAlerts = [...apiAlerts.slice(0, index), ...apiAlerts.slice(index + 1)];

			// Update pagination total
			pagination = { ...pagination, total: Math.max(0, pagination.total - 1) };
		}
	}

	/**
	 * Set stats directly from WebSocket update
	 * Called by WebSocket real-time updates
	 */
	function setStats(newStats: QuickStats) {
		apiStats = newStats;
	}

	/**
	 * Get current performance metrics for the page
	 */
	function getPerformanceMetrics() {
		return performanceMonitor.reportMetrics();
	}

	/**
	 * Get session analytics metrics
	 */
	function getSessionMetrics() {
		return analyticsTracker.getSessionMetrics();
	}

	/**
	 * Track page unload for cleanup
	 */
	function onPageLeave() {
		performanceMonitor.endMark('page-load');
		analyticsTracker.trackPageLeave();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// MODAL ACTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function openAlertModal(alert?: RoomAlert) {
		trackModalOpened(alert ? 'edit_alert' : 'create_alert');
		editingAlert = alert ?? null;
		isAlertModalOpen = true;
	}

	function closeAlertModal() {
		isAlertModalOpen = false;
		editingAlert = null;
	}

	function openTradeEntryModal(entry?: ApiTradePlanEntry) {
		trackModalOpened(entry ? 'edit_trade_entry' : 'create_trade_entry');
		editingTradeEntry = entry ?? null;
		isTradeEntryModalOpen = true;
	}

	function closeTradeEntryModal() {
		isTradeEntryModalOpen = false;
		editingTradeEntry = null;
	}

	function openClosePositionModal(position: ActivePosition) {
		trackModalOpened('close_position');
		closingPosition = position;
		isClosePositionModalOpen = true;
	}

	function closeClosePositionModal() {
		isClosePositionModalOpen = false;
		closingPosition = null;
	}

	function openVideoUploadModal() {
		trackModalOpened('video_upload');
		isVideoUploadModalOpen = true;
	}

	function closeVideoUploadModal() {
		isVideoUploadModalOpen = false;
	}

	function openAddTradeModal() {
		trackModalOpened('add_trade');
		isAddTradeModalOpen = true;
	}

	function closeAddTradeModal() {
		isAddTradeModalOpen = false;
	}

	function openUpdatePositionModal(position: ActivePosition) {
		trackModalOpened('update_position');
		updatingPosition = position;
		isUpdatePositionModalOpen = true;
	}

	function closeUpdatePositionModal() {
		isUpdatePositionModalOpen = false;
		updatingPosition = null;
	}

	function handlePositionUpdated() {
		// Refresh positions data
		fetchAllTrades();
		fetchStats();
	}

	function openInvalidatePositionModal(position: ActivePosition) {
		trackModalOpened('invalidate_position');
		invalidatingPosition = position;
		isInvalidatePositionModalOpen = true;
	}

	function closeInvalidatePositionModal() {
		isInvalidatePositionModalOpen = false;
		invalidatingPosition = null;
	}

	async function deletePosition(position: ActivePosition) {
		try {
			const response = await fetch(`/api/admin/trades/${position.id}`, {
				method: 'DELETE',
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error('Failed to delete position');
			}

			// Refresh data
			fetchAllTrades();
			fetchStats();
		} catch (err) {
			console.error('Failed to delete position:', err);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN PUBLIC API
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		// Reactive state (getters for read, setters where mutation is allowed)
		get selectedFilter() {
			return selectedFilter;
		},
		get currentPage() {
			return currentPage;
		},
		get pagination() {
			return pagination;
		},
		get isAdmin() {
			return isAdmin;
		},
		get expandedNotes() {
			return expandedNotes;
		},
		get copiedAlertId() {
			return copiedAlertId;
		},

		// Loading states
		get isLoadingAlerts() {
			return isLoadingAlerts;
		},
		get isLoadingTradePlan() {
			return isLoadingTradePlan;
		},
		get isLoadingStats() {
			return isLoadingStats;
		},
		get isLoadingTrades() {
			return isLoadingTrades;
		},
		get isLoadingVideos() {
			return isLoadingVideos;
		},
		get isInitialLoading() {
			return isInitialLoading;
		},

		// Error states
		get alertsError() {
			return alertsError;
		},
		get tradePlanError() {
			return tradePlanError;
		},
		get statsError() {
			return statsError;
		},
		get tradesError() {
			return tradesError;
		},
		get videosError() {
			return videosError;
		},
		get hasAnyError() {
			return hasAnyError;
		},

		// Derived data
		get totalPages() {
			return totalPages;
		},
		get showingFrom() {
			return showingFrom;
		},
		get showingTo() {
			return showingTo;
		},
		get alerts() {
			return alerts;
		},
		get filteredAlerts() {
			return filteredAlerts;
		},
		get tradePlan() {
			return tradePlan;
		},
		get stats() {
			return stats;
		},
		get weeklyPerformance() {
			return weeklyPerformance;
		},
		get closedTrades() {
			return closedTrades;
		},
		get activePositions() {
			return activePositions;
		},
		get weeklyContent() {
			return weeklyContent;
		},
		get apiWeeklyVideo() {
			return apiWeeklyVideo;
		},
		get apiTradePlan() {
			return apiTradePlan;
		},

		// Modal state
		get isAlertModalOpen() {
			return isAlertModalOpen;
		},
		get editingAlert() {
			return editingAlert;
		},
		get isTradeEntryModalOpen() {
			return isTradeEntryModalOpen;
		},
		get editingTradeEntry() {
			return editingTradeEntry;
		},
		get isVideoUploadModalOpen() {
			return isVideoUploadModalOpen;
		},
		get isClosePositionModalOpen() {
			return isClosePositionModalOpen;
		},
		get closingPosition() {
			return closingPosition;
		},
		get isAddTradeModalOpen() {
			return isAddTradeModalOpen;
		},
		get isUpdatePositionModalOpen() {
			return isUpdatePositionModalOpen;
		},
		get updatingPosition() {
			return updatingPosition;
		},
		get isInvalidatePositionModalOpen() {
			return isInvalidatePositionModalOpen;
		},
		get invalidatingPosition() {
			return invalidatingPosition;
		},

		// Actions
		initializeData,
		setFilter,
		goToPage,
		toggleNotes,
		copyTradeDetails,
		fetchAlerts,
		fetchTradePlan,
		fetchStats,
		fetchAllTrades,
		fetchWeeklyVideo,

		// Real-time update methods (ICT 7+ Phase 3)
		prependAlert,
		updateAlert,
		removeAlert,
		setStats,

		// Modal actions
		openAlertModal,
		closeAlertModal,
		openTradeEntryModal,
		closeTradeEntryModal,
		openClosePositionModal,
		closeClosePositionModal,
		openVideoUploadModal,
		closeVideoUploadModal,
		openAddTradeModal,
		closeAddTradeModal,
		openUpdatePositionModal,
		closeUpdatePositionModal,
		handlePositionUpdated,
		openInvalidatePositionModal,
		closeInvalidatePositionModal,
		deletePosition,

		// Performance monitoring
		getPerformanceMetrics,
		getSessionMetrics,
		onPageLeave,

		// Constants
		ROOM_SLUG
	};
}

export type PageState = ReturnType<typeof createPageState>;

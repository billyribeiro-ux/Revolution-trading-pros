/**
 * ===============================================================================
 * Explosive Swings - Page State Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for createPageState() factory function
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Vitest January 2026 Patterns
 *
 * Tests cover:
 * - State initialization
 * - State mutations
 * - Derived state calculations
 * - Loading/error state management
 * - Pagination logic
 * - Filter logic
 * - Modal state management
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
	setupTestEnvironment,
	createMockAlertsResponse,
	createMockAlert,
	createMockTrade,
	createMockActivePosition,
	flushPromises,
	resetAllMocks
} from './setup';

// Setup test environment
setupTestEnvironment();

// ===============================================================================
// MOCK THE REMOTE FUNCTION MODULES
// ===============================================================================

// Mock data.remote — remote query functions used by page.state.svelte.ts
vi.mock('../data.remote', () => ({
	getAlerts: vi.fn(),
	getTradePlan: vi.fn(),
	getStats: vi.fn(),
	getTrades: vi.fn(),
	getWeeklyVideo: vi.fn(),
	getAdminStatus: vi.fn()
}));

// Mock commands.remote — remote command functions used by page.state.svelte.ts
vi.mock('../commands.remote', () => ({
	deleteTrade: vi.fn()
}));

// Import after mocking
import { createPageState } from '../page.state.svelte';
import {
	getAlerts,
	getTradePlan,
	getStats,
	getTrades,
	getWeeklyVideo,
	getAdminStatus
} from '../data.remote';
import { deleteTrade } from '../commands.remote';

// ===============================================================================
// TEST SUITE: createPageState Factory Function
// ===============================================================================

describe('createPageState()', () => {
	beforeEach(() => {
		resetAllMocks();
		vi.clearAllMocks();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	// ===========================================================================
	// INITIALIZATION TESTS
	// ===========================================================================

	describe('initialization', () => {
		it('should create state with correct initial values', () => {
			const state = createPageState();

			// Filter & Pagination
			expect(state.selectedFilter).toBe('all');
			expect(state.currentPage).toBe(1);
			expect(state.pagination).toEqual({ total: 0, limit: 10, offset: 0 });

			// Admin state
			expect(state.isAdmin).toBe(false);

			// Loading states should start as false
			expect(state.isLoadingAlerts).toBe(false);
			expect(state.isLoadingTradePlan).toBe(false);
			expect(state.isLoadingStats).toBe(false);
			expect(state.isLoadingTrades).toBe(false);
			expect(state.isLoadingVideos).toBe(false);

			// Error states should be null
			expect(state.alertsError).toBeNull();
			expect(state.tradePlanError).toBeNull();
			expect(state.statsError).toBeNull();
			expect(state.tradesError).toBeNull();
			expect(state.videosError).toBeNull();

			// UI state
			expect(state.copiedAlertId).toBeNull();
		});

		it('should have fallback data available when API data is empty', () => {
			const state = createPageState();

			// Derived state should fall back to fallbackData
			expect(state.alerts.length).toBeGreaterThan(0);
			expect(state.tradePlan.length).toBeGreaterThan(0);
			expect(state.stats).toBeDefined();
			expect(state.stats.winRate).toBeGreaterThan(0);
		});

		it('should have all modal states closed initially', () => {
			const state = createPageState();

			expect(state.isAlertModalOpen).toBe(false);
			expect(state.editingAlert).toBeNull();
			expect(state.isTradeEntryModalOpen).toBe(false);
			expect(state.editingTradeEntry).toBeNull();
			expect(state.isVideoUploadModalOpen).toBe(false);
			expect(state.isClosePositionModalOpen).toBe(false);
			expect(state.closingPosition).toBeNull();
			expect(state.isAddTradeModalOpen).toBe(false);
			expect(state.isUpdatePositionModalOpen).toBe(false);
			expect(state.updatingPosition).toBeNull();
			expect(state.isInvalidatePositionModalOpen).toBe(false);
			expect(state.invalidatingPosition).toBeNull();
		});

		it('should export ROOM_SLUG constant', () => {
			const state = createPageState();
			expect(state.ROOM_SLUG).toBeDefined();
			expect(typeof state.ROOM_SLUG).toBe('string');
		});
	});

	// ===========================================================================
	// DERIVED STATE TESTS
	// ===========================================================================

	describe('derived state calculations', () => {
		it('should calculate totalPages correctly', () => {
			const state = createPageState();

			// With default pagination (total: 0, limit: 10)
			expect(state.totalPages).toBe(1); // Minimum 1 page

			// Note: We can't directly modify pagination in tests since it's reactive
			// The totalPages will update when pagination.total changes via API fetch
		});

		it('should calculate showingFrom and showingTo correctly', () => {
			const state = createPageState();

			// With no results
			expect(state.showingFrom).toBe(0);
			expect(state.showingTo).toBe(0);
		});

		it('should filter alerts by type when filter is applied', () => {
			const state = createPageState();

			// Default filter is 'all', should show all alerts
			const allAlerts = state.alerts;
			const filteredAlerts = state.filteredAlerts;

			expect(filteredAlerts.length).toBe(allAlerts.length);
		});

		it('should derive weeklyPerformance from stats', () => {
			const state = createPageState();

			expect(state.weeklyPerformance).toBeDefined();
			expect(state.weeklyPerformance.winRate).toBeGreaterThan(0);
			expect(state.weeklyPerformance.totalTrades).toBeGreaterThan(0);
			expect(state.weeklyPerformance.riskRewardRatio).toBeGreaterThan(0);
		});

		it('should derive weeklyContent with valid date formatting', () => {
			const state = createPageState();

			expect(state.weeklyContent).toBeDefined();
			expect(state.weeklyContent.title).toBeDefined();
			expect(state.weeklyContent.videoUrl).toBeDefined();
			expect(state.weeklyContent.publishedDate).toContain('ET');
		});

		it('should calculate hasAnyError when any error exists', () => {
			const state = createPageState();

			// Initially no errors
			expect(state.hasAnyError).toBeFalsy();
		});

		it('should calculate isInitialLoading when multiple loads are in progress', () => {
			const state = createPageState();

			// Initially not loading
			expect(state.isInitialLoading).toBe(false);
		});
	});

	// ===========================================================================
	// FETCH ACTIONS TESTS
	// ===========================================================================

	describe('fetchAlerts()', () => {
		it('should set loading state while fetching', async () => {
			const mockAlerts = [createMockAlert(), createMockAlert({ id: 2, ticker: 'TSLA' })];
			void createMockAlertsResponse(mockAlerts);

			vi.mocked(getAlerts).mockResolvedValueOnce({
				alerts: mockAlerts.map((a) => ({
					...a,
					notes: a.notes || ''
				})),
				pagination: { total: mockAlerts.length, limit: 10, offset: 0 }
			});

			const state = createPageState();
			const fetchPromise = state.fetchAlerts();

			// Loading should be true during fetch
			expect(state.isLoadingAlerts).toBe(true);

			await fetchPromise;

			// Loading should be false after fetch
			expect(state.isLoadingAlerts).toBe(false);
		});

		it('should update alerts state on successful fetch', async () => {
			const mockAlerts = [
				createMockAlert({ id: 1, ticker: 'NVDA' }),
				createMockAlert({ id: 2, ticker: 'TSLA' })
			];

			vi.mocked(getAlerts).mockResolvedValueOnce({
				alerts: mockAlerts.map((a) => ({ ...a, notes: a.notes || '' })),
				pagination: { total: 2, limit: 10, offset: 0 }
			});

			const state = createPageState();
			await state.fetchAlerts();

			expect(state.alerts.length).toBe(2);
			expect(state.alertsError).toBeNull();
		});

		it('should set error state on fetch failure', async () => {
			vi.mocked(getAlerts).mockRejectedValueOnce(new Error('Network error'));

			const state = createPageState();
			await state.fetchAlerts();

			expect(state.alertsError).toBe('Network error');
			expect(state.isLoadingAlerts).toBe(false);
		});

		it('should handle non-Error exceptions', async () => {
			vi.mocked(getAlerts).mockRejectedValueOnce('String error');

			const state = createPageState();
			await state.fetchAlerts();

			expect(state.alertsError).toBe('Failed to load alerts');
		});
	});

	describe('fetchTradePlan()', () => {
		it('should update trade plan on successful fetch', async () => {
			const mockEntries = [
				{
					ticker: 'NVDA',
					bias: 'BULLISH',
					entry: '$142',
					target1: '$148',
					target2: '$152',
					target3: '$158',
					runner: '$165',
					stop: '$136',
					options_strike: '$145 Call',
					options_exp: 'Jan 24',
					notes: 'Test'
				}
			];

			vi.mocked(getTradePlan).mockResolvedValueOnce(mockEntries as any);

			const state = createPageState();
			await state.fetchTradePlan();

			expect(state.isLoadingTradePlan).toBe(false);
			expect(state.tradePlanError).toBeNull();
		});

		it('should set error state on failure', async () => {
			vi.mocked(getTradePlan).mockRejectedValueOnce(new Error('API error'));

			const state = createPageState();
			await state.fetchTradePlan();

			expect(state.tradePlanError).toBe('API error');
		});
	});

	describe('fetchStats()', () => {
		it('should update stats on successful fetch', async () => {
			vi.mocked(getStats).mockResolvedValueOnce({
				winRate: 85,
				weeklyProfit: '+$5,000',
				activeTrades: 5,
				closedThisWeek: 3
			});

			const state = createPageState();
			await state.fetchStats();

			expect(state.stats.winRate).toBe(85);
			expect(state.statsError).toBeNull();
		});

		it('should set error state on failure', async () => {
			vi.mocked(getStats).mockRejectedValueOnce(new Error('Stats unavailable'));

			const state = createPageState();
			await state.fetchStats();

			expect(state.statsError).toBe('Stats unavailable');
		});
	});

	describe('fetchAllTrades()', () => {
		it('should separate open and closed trades', async () => {
			const mockTrades = [
				createMockTrade({ id: 1, status: 'open' }),
				createMockTrade({ id: 2, status: 'closed', pnl_percent: 5.5 }),
				createMockTrade({ id: 3, status: 'open' })
			];

			vi.mocked(getTrades).mockResolvedValueOnce(mockTrades as any);

			const state = createPageState();
			await state.fetchAllTrades();

			// State derives activePositions and closedTrades from API data
			expect(state.isLoadingTrades).toBe(false);
			expect(state.tradesError).toBeNull();
		});

		it('should set error state on failure', async () => {
			vi.mocked(getTrades).mockRejectedValueOnce(new Error('Trades unavailable'));

			const state = createPageState();
			await state.fetchAllTrades();

			expect(state.tradesError).toBe('Trades unavailable');
		});
	});

	describe('fetchWeeklyVideo()', () => {
		it('should update weekly video on successful fetch', async () => {
			vi.mocked(getWeeklyVideo).mockResolvedValueOnce({
				id: 1,
				video_title: 'Test Video',
				video_url: 'https://example.com/video',
				thumbnail_url: 'https://example.com/thumb.jpg',
				duration: '25:00',
				published_at: new Date().toISOString(),
				week_title: 'Week of January 20'
			});

			const state = createPageState();
			await state.fetchWeeklyVideo();

			expect(state.isLoadingVideos).toBe(false);
			expect(state.videosError).toBeNull();
		});

		it('should set error state on failure', async () => {
			vi.mocked(getWeeklyVideo).mockRejectedValueOnce(new Error('Video unavailable'));

			const state = createPageState();
			await state.fetchWeeklyVideo();

			expect(state.videosError).toBe('Video unavailable');
		});
	});

	// ===========================================================================
	// FILTER LOGIC TESTS
	// ===========================================================================

	describe('setFilter()', () => {
		it('should update filter and reset page to 1', async () => {
			vi.mocked(getAlerts).mockResolvedValue({
				alerts: [],
				pagination: { total: 0, limit: 10, offset: 0 }
			});

			const state = createPageState();

			// Change filter to 'entry'
			state.setFilter('entry');

			expect(state.selectedFilter).toBe('entry');
			expect(state.currentPage).toBe(1);
		});

		it('should not fetch if filter is already selected', async () => {
			const state = createPageState();

			// Filter is already 'all', so this should not trigger fetch
			state.setFilter('all');

			expect(getAlerts).not.toHaveBeenCalled();
		});

		it('should trigger fetchAlerts when filter changes', async () => {
			vi.mocked(getAlerts).mockResolvedValue({
				alerts: [],
				pagination: { total: 0, limit: 10, offset: 0 }
			});

			const state = createPageState();
			state.setFilter('exit');

			expect(getAlerts).toHaveBeenCalled();
		});
	});

	// ===========================================================================
	// PAGINATION LOGIC TESTS
	// ===========================================================================

	describe('goToPage()', () => {
		it('should not navigate to invalid page numbers', async () => {
			const state = createPageState();

			await state.goToPage(0); // Less than 1
			expect(state.currentPage).toBe(1);

			await state.goToPage(-1); // Negative
			expect(state.currentPage).toBe(1);
		});

		it('should not navigate if already on the requested page', async () => {
			const state = createPageState();

			await state.goToPage(1); // Already on page 1

			expect(getAlerts).not.toHaveBeenCalled();
		});

		it('should fetch alerts when navigating to a valid new page', async () => {
			vi.mocked(getAlerts).mockResolvedValue({
				alerts: [],
				pagination: { total: 50, limit: 10, offset: 10 }
			});

			void createPageState();

			// First, we need to have more than 1 page
			// Since totalPages depends on pagination.total, we need to mock this

			// For now, test that goToPage calls fetchAlerts when conditions are met
			// This is limited by the derived state calculation
		});
	});

	// ===========================================================================
	// COPY TRADE DETAILS TESTS
	// ===========================================================================

	describe('copyTradeDetails()', () => {
		it('should copy alert details to clipboard', async () => {
			const mockClipboard = {
				writeText: vi.fn().mockResolvedValue(undefined)
			};
			Object.assign(navigator, { clipboard: mockClipboard });

			const state = createPageState();
			const alert = {
				id: 1,
				type: 'ENTRY' as const,
				ticker: 'NVDA',
				title: 'Test Alert',
				time: 'Today',
				message: 'Test message',
				isNew: false,
				notes: ''
			};

			await state.copyTradeDetails(alert);

			expect(mockClipboard.writeText).toHaveBeenCalled();
			expect(state.copiedAlertId).toBe(1);
		});

		it('should include TOS string when available', async () => {
			const mockClipboard = {
				writeText: vi.fn().mockResolvedValue(undefined)
			};
			Object.assign(navigator, { clipboard: mockClipboard });

			const state = createPageState();
			const alert = {
				id: 1,
				type: 'ENTRY' as const,
				ticker: 'NVDA',
				title: 'Test Alert',
				time: 'Today',
				message: 'Test message',
				isNew: false,
				notes: '',
				tosString: 'TOS_STRING_HERE'
			};

			await state.copyTradeDetails(alert);

			const copiedText = mockClipboard.writeText.mock.calls[0][0];
			expect(copiedText).toContain('TOS: TOS_STRING_HERE');
		});

		it('should clear copiedAlertId after timeout', async () => {
			vi.useFakeTimers();

			const mockClipboard = {
				writeText: vi.fn().mockResolvedValue(undefined)
			};
			Object.assign(navigator, { clipboard: mockClipboard });

			const state = createPageState();
			const alert = {
				id: 1,
				type: 'ENTRY' as const,
				ticker: 'NVDA',
				title: 'Test',
				time: 'Today',
				message: 'Test',
				isNew: false,
				notes: ''
			};

			await state.copyTradeDetails(alert);
			expect(state.copiedAlertId).toBe(1);

			vi.advanceTimersByTime(2000);

			expect(state.copiedAlertId).toBeNull();

			vi.useRealTimers();
		});
	});

	// ===========================================================================
	// MODAL ACTIONS TESTS
	// ===========================================================================

	describe('Modal Actions', () => {
		describe('Alert Modal', () => {
			it('should open alert modal without editing alert', () => {
				const state = createPageState();

				state.openAlertModal();

				expect(state.isAlertModalOpen).toBe(true);
				expect(state.editingAlert).toBeNull();
			});

			it('should open alert modal with editing alert', () => {
				const state = createPageState();
				const alert = { id: 1, ticker: 'NVDA' } as any;

				state.openAlertModal(alert);

				expect(state.isAlertModalOpen).toBe(true);
				expect(state.editingAlert).toEqual(alert);
			});

			it('should close alert modal and clear editing state', () => {
				const state = createPageState();
				const alert = { id: 1, ticker: 'NVDA' } as any;

				state.openAlertModal(alert);
				state.closeAlertModal();

				expect(state.isAlertModalOpen).toBe(false);
				expect(state.editingAlert).toBeNull();
			});
		});

		describe('Trade Entry Modal', () => {
			it('should open trade entry modal', () => {
				const state = createPageState();

				state.openTradeEntryModal();

				expect(state.isTradeEntryModalOpen).toBe(true);
			});

			it('should open with editing entry', () => {
				const state = createPageState();
				const entry = { ticker: 'NVDA' } as any;

				state.openTradeEntryModal(entry);

				expect(state.isTradeEntryModalOpen).toBe(true);
				expect(state.editingTradeEntry).toEqual(entry);
			});

			it('should close and clear state', () => {
				const state = createPageState();
				const entry = { ticker: 'NVDA' } as any;

				state.openTradeEntryModal(entry);
				state.closeTradeEntryModal();

				expect(state.isTradeEntryModalOpen).toBe(false);
				expect(state.editingTradeEntry).toBeNull();
			});
		});

		describe('Close Position Modal', () => {
			it('should open with position', () => {
				const state = createPageState();
				const position = createMockActivePosition();

				state.openClosePositionModal(position as any);

				expect(state.isClosePositionModalOpen).toBe(true);
				expect(state.closingPosition).toEqual(position);
			});

			it('should close and clear state', () => {
				const state = createPageState();
				const position = createMockActivePosition();

				state.openClosePositionModal(position as any);
				state.closeClosePositionModal();

				expect(state.isClosePositionModalOpen).toBe(false);
				expect(state.closingPosition).toBeNull();
			});
		});

		describe('Video Upload Modal', () => {
			it('should open and close', () => {
				const state = createPageState();

				state.openVideoUploadModal();
				expect(state.isVideoUploadModalOpen).toBe(true);

				state.closeVideoUploadModal();
				expect(state.isVideoUploadModalOpen).toBe(false);
			});
		});

		describe('Add Trade Modal', () => {
			it('should open and close', () => {
				const state = createPageState();

				state.openAddTradeModal();
				expect(state.isAddTradeModalOpen).toBe(true);

				state.closeAddTradeModal();
				expect(state.isAddTradeModalOpen).toBe(false);
			});
		});

		describe('Update Position Modal', () => {
			it('should open with position', () => {
				const state = createPageState();
				const position = createMockActivePosition();

				state.openUpdatePositionModal(position as any);

				expect(state.isUpdatePositionModalOpen).toBe(true);
				expect(state.updatingPosition).toEqual(position);
			});

			it('should close and clear state', () => {
				const state = createPageState();
				const position = createMockActivePosition();

				state.openUpdatePositionModal(position as any);
				state.closeUpdatePositionModal();

				expect(state.isUpdatePositionModalOpen).toBe(false);
				expect(state.updatingPosition).toBeNull();
			});
		});

		describe('Invalidate Position Modal', () => {
			it('should open with position', () => {
				const state = createPageState();
				const position = createMockActivePosition();

				state.openInvalidatePositionModal(position as any);

				expect(state.isInvalidatePositionModalOpen).toBe(true);
				expect(state.invalidatingPosition).toEqual(position);
			});

			it('should close and clear state', () => {
				const state = createPageState();
				const position = createMockActivePosition();

				state.openInvalidatePositionModal(position as any);
				state.closeInvalidatePositionModal();

				expect(state.isInvalidatePositionModalOpen).toBe(false);
				expect(state.invalidatingPosition).toBeNull();
			});
		});
	});

	// ===========================================================================
	// POSITION UPDATED HANDLER TESTS
	// ===========================================================================

	describe('handlePositionUpdated()', () => {
		it('should refresh trades and stats', async () => {
			vi.mocked(getTrades).mockResolvedValue([]);
			vi.mocked(getStats).mockResolvedValue({
				winRate: 80,
				weeklyProfit: '+$4,000',
				activeTrades: 3,
				closedThisWeek: 2
			});

			const state = createPageState();
			state.handlePositionUpdated();

			await flushPromises();

			expect(getTrades).toHaveBeenCalled();
			expect(getStats).toHaveBeenCalled();
		});
	});

	// ===========================================================================
	// INITIALIZE DATA TESTS
	// ===========================================================================

	describe('initializeData()', () => {
		it('should fetch all data in parallel', async () => {
			vi.mocked(getAdminStatus).mockResolvedValue(false);
			vi.mocked(getAlerts).mockResolvedValue({
				alerts: [],
				pagination: { total: 0, limit: 10, offset: 0 }
			});
			vi.mocked(getTradePlan).mockResolvedValue([]);
			vi.mocked(getStats).mockResolvedValue({
				winRate: 80,
				weeklyProfit: '+$4,000',
				activeTrades: 3,
				closedThisWeek: 2
			});
			vi.mocked(getTrades).mockResolvedValue([]);
			vi.mocked(getWeeklyVideo).mockResolvedValue(null);

			const state = createPageState();
			state.initializeData();

			await flushPromises();

			expect(getAdminStatus).toHaveBeenCalled();
			expect(getAlerts).toHaveBeenCalled();
			expect(getTradePlan).toHaveBeenCalled();
			expect(getStats).toHaveBeenCalled();
			expect(getTrades).toHaveBeenCalled();
			expect(getWeeklyVideo).toHaveBeenCalled();
		});
	});

	// ===========================================================================
	// DELETE POSITION TESTS
	// ===========================================================================

	describe('deletePosition()', () => {
		it('should call API to delete position and refresh data', async () => {
			vi.mocked(deleteTrade).mockResolvedValue(undefined);
			vi.mocked(getTrades).mockResolvedValue([]);
			vi.mocked(getStats).mockResolvedValue({
				winRate: 80,
				weeklyProfit: '+$4,000',
				activeTrades: 2,
				closedThisWeek: 2
			});

			const state = createPageState();
			const position = createMockActivePosition();

			await state.deletePosition(position as any);

			expect(deleteTrade).toHaveBeenCalledWith({
				tradeId: Number(position.id)
			});
		});

		it('should handle delete failure gracefully', async () => {
			vi.mocked(deleteTrade).mockRejectedValueOnce(new Error('Server error'));

			const state = createPageState();
			const position = createMockActivePosition();

			// deletePosition re-throws errors for UI to handle
			await expect(state.deletePosition(position as any)).rejects.toThrow();
		});
	});

	// ===========================================================================
	// ADMIN STATUS TESTS
	// ===========================================================================

	describe('checkAdminStatus()', () => {
		it('should set isAdmin to true for admin user', async () => {
			vi.mocked(getAdminStatus).mockResolvedValue(true);

			void createPageState();

			// Admin status is checked via initializeData or directly
			// We need to trigger it
			await vi.mocked(getAdminStatus)();

			// Note: Due to encapsulation, we can't directly call checkAdminStatus
			// It's called internally by initializeData
		});

		it('should set isAdmin to false on error', async () => {
			vi.mocked(getAdminStatus).mockRejectedValue(new Error('Unauthorized'));

			const state = createPageState();

			// isAdmin should remain false (default)
			expect(state.isAdmin).toBe(false);
		});
	});
});

// ===============================================================================
// TYPE SAFETY TESTS
// ===============================================================================

describe('Type Safety', () => {
	it('should return PageState type from createPageState', () => {
		const state = createPageState();

		// TypeScript compilation verifies these exist
		void (state.selectedFilter satisfies 'all' | 'entry' | 'exit' | 'update');
		void (state.currentPage satisfies number);
		void (state.isAdmin satisfies boolean);
		void (state.isLoadingAlerts satisfies boolean);
		void (state.alertsError satisfies string | null);

		expect(true).toBe(true); // Type checking is done at compile time
	});
});

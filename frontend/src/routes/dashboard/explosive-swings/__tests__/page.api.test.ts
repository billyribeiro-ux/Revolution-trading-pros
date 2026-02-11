/**
 * ===============================================================================
 * Explosive Swings - Page API Unit Tests
 * ===============================================================================
 *
 * @description Comprehensive tests for API functions using mocked api client
 * @version 2.0.0
 * @standards Apple Principal Engineer ICT 7+ | Vitest January 2026 Patterns
 *
 * Tests cover:
 * - fetchAlerts() with mocked api.get responses
 * - fetchTradePlan() with data transformation
 * - fetchStats() with snake_case to camelCase
 * - fetchAllTrades()
 * - fetchWeeklyVideo()
 * - checkAdminStatus()
 * - Error handling
 * - Response transformation
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { setupTestEnvironment, resetAllMocks } from './setup';

// Setup test environment (registers vi.mock for $lib/api/client etc.)
setupTestEnvironment();

// Import the mocked api module
import { api } from '$lib/api/client';

// Import after setup to ensure mocks are in place
import {
	fetchAlerts,
	fetchTradePlan,
	fetchStats,
	fetchAllTrades,
	fetchWeeklyVideo,
	checkAdminStatus
} from '../page.api';

// Typed reference to the mocked api methods
const mockApiGet = api.get as ReturnType<typeof vi.fn>;

// ===============================================================================
// TEST SUITE: fetchAlerts
// ===============================================================================

describe('fetchAlerts()', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	afterEach(() => {
		vi.restoreAllMocks();
	});

	describe('successful requests', () => {
		it('should fetch alerts with correct URL parameters', async () => {
			mockApiGet.mockResolvedValueOnce({
				data: [
					{
						id: 1,
						alert_type: 'ENTRY',
						ticker: 'NVDA',
						title: 'Test Alert',
						published_at: new Date().toISOString(),
						message: 'Test message',
						is_new: true,
						notes: 'Test notes',
						tos_string: 'TOS_STRING'
					}
				],
				total: 1
			});

			const result = await fetchAlerts('explosive-swings', 1, 10);

			expect(mockApiGet).toHaveBeenCalledWith(
				'/api/alerts/explosive-swings',
				expect.objectContaining({
					params: { limit: 10, offset: 0 }
				})
			);
			expect(result.alerts).toHaveLength(1);
			expect(result.pagination.total).toBe(1);
		});

		it('should calculate correct offset for page 2', async () => {
			mockApiGet.mockResolvedValueOnce({ data: [], total: 0 });

			await fetchAlerts('explosive-swings', 2, 10);

			expect(mockApiGet).toHaveBeenCalledWith(
				'/api/alerts/explosive-swings',
				expect.objectContaining({
					params: { limit: 10, offset: 10 }
				})
			);
		});

		it('should calculate correct offset for page 3 with custom limit', async () => {
			mockApiGet.mockResolvedValueOnce({ data: [], total: 0 });

			await fetchAlerts('explosive-swings', 3, 15);

			expect(mockApiGet).toHaveBeenCalledWith(
				'/api/alerts/explosive-swings',
				expect.objectContaining({
					params: { limit: 15, offset: 30 }
				})
			);
		});

		it('should transform API response to FormattedAlert type', async () => {
			const publishedAt = new Date('2026-01-20T14:30:00Z').toISOString();
			mockApiGet.mockResolvedValueOnce({
				data: [
					{
						id: 123,
						alert_type: 'ENTRY',
						ticker: 'NVDA',
						title: 'Opening Position',
						published_at: publishedAt,
						message: 'Entry at $142.50',
						is_new: true,
						notes: 'Strong breakout',
						tos_string: 'TOS:NVDA'
					}
				],
				total: 1
			});

			const result = await fetchAlerts('explosive-swings', 1, 10);

			expect(result.alerts[0]).toMatchObject({
				id: 123,
				type: 'ENTRY',
				ticker: 'NVDA',
				title: 'Opening Position',
				message: 'Entry at $142.50',
				isNew: true,
				notes: 'Strong breakout',
				tosString: 'TOS:NVDA'
			});
			expect(result.alerts[0].time).toBeDefined();
		});

		it('should handle alerts without optional fields', async () => {
			mockApiGet.mockResolvedValueOnce({
				data: [
					{
						id: 1,
						alert_type: 'UPDATE',
						ticker: 'TSLA',
						title: 'Update',
						published_at: new Date().toISOString(),
						message: 'Test',
						is_new: false,
						notes: null,
						tos_string: null
					}
				],
				total: 1
			});

			const result = await fetchAlerts('explosive-swings', 1, 10);

			expect(result.alerts[0].notes).toBe('');
			expect(result.alerts[0].tosString).toBeUndefined();
		});

		it('should return correct pagination state', async () => {
			mockApiGet.mockResolvedValueOnce({
				data: Array(10).fill({
					id: 1,
					alert_type: 'ENTRY',
					ticker: 'NVDA',
					title: 'Test',
					published_at: new Date().toISOString(),
					message: 'Test',
					is_new: false,
					notes: '',
					tos_string: null
				}),
				total: 45
			});

			const result = await fetchAlerts('explosive-swings', 2, 10);

			expect(result.pagination).toEqual({
				total: 45,
				limit: 10,
				offset: 10
			});
		});
	});

	describe('error handling', () => {
		it('should propagate api.get errors', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('API request failed'));

			await expect(fetchAlerts('explosive-swings', 1, 10)).rejects.toThrow('API request failed');
		});

		it('should handle network errors', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('Connection refused'));

			await expect(fetchAlerts('explosive-swings', 1, 10)).rejects.toThrow('Connection refused');
		});
	});
});

// ===============================================================================
// TEST SUITE: fetchTradePlan
// ===============================================================================

describe('fetchTradePlan()', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	describe('successful requests', () => {
		it('should fetch trade plan with correct URL', async () => {
			mockApiGet.mockResolvedValueOnce([
				{
					ticker: 'NVDA',
					bias: 'BULLISH',
					entry: '$142.50',
					target1: '$148.00',
					target2: '$152.00',
					target3: '$158.00',
					runner: '$165.00+',
					stop: '$136.00',
					options_strike: '$145 Call',
					options_exp: 'Jan 24, 2026',
					notes: 'Breakout setup'
				}
			]);

			await fetchTradePlan('explosive-swings');

			expect(mockApiGet).toHaveBeenCalledWith(
				'/api/trade-plans/explosive-swings',
				expect.any(Object)
			);
		});

		it('should transform API response to TradePlanEntry type', async () => {
			mockApiGet.mockResolvedValueOnce([
				{
					ticker: 'NVDA',
					bias: 'BULLISH',
					entry: '$142.50',
					target1: '$148.00',
					target2: '$152.00',
					target3: '$158.00',
					runner: '$165.00+',
					stop: '$136.00',
					options_strike: '$145 Call',
					options_exp: 'Jan 24, 2026',
					notes: 'Strong momentum'
				}
			]);

			const result = await fetchTradePlan('explosive-swings');

			expect(result[0]).toMatchObject({
				ticker: 'NVDA',
				bias: 'BULLISH',
				entry: '$142.50',
				target1: '$148.00',
				target2: '$152.00',
				target3: '$158.00',
				runner: '$165.00+',
				stop: '$136.00',
				optionsStrike: '$145 Call',
				optionsExp: 'Jan 24, 2026',
				notes: 'Strong momentum'
			});
		});

		it('should handle empty notes', async () => {
			mockApiGet.mockResolvedValueOnce([
				{
					ticker: 'TSLA',
					bias: 'BEARISH',
					entry: '$248.00',
					target1: '$240.00',
					target2: '$235.00',
					target3: '$230.00',
					runner: '$220.00',
					stop: '$255.00',
					options_strike: null,
					options_exp: null,
					notes: null
				}
			]);

			const result = await fetchTradePlan('explosive-swings');

			expect(result[0].optionsStrike).toBe('-');
			expect(result[0].optionsExp).toBe('-');
			expect(result[0].notes).toBe('');
		});
	});

	describe('error handling', () => {
		it('should propagate api.get errors', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('Unauthorized'));

			await expect(fetchTradePlan('explosive-swings')).rejects.toThrow('Unauthorized');
		});
	});
});

// ===============================================================================
// TEST SUITE: fetchStats
// ===============================================================================

describe('fetchStats()', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	describe('successful requests', () => {
		it('should fetch stats with correct URL', async () => {
			mockApiGet.mockResolvedValueOnce({
				win_rate: 82,
				weekly_profit: '+$4,850',
				active_trades: 4,
				closed_this_week: 3
			});

			await fetchStats('explosive-swings');

			expect(mockApiGet).toHaveBeenCalledWith('/api/stats/explosive-swings', expect.any(Object));
		});

		it('should transform snake_case to camelCase', async () => {
			mockApiGet.mockResolvedValueOnce({
				win_rate: 85,
				weekly_profit: '+$5,200',
				active_trades: 5,
				closed_this_week: 2
			});

			const result = await fetchStats('explosive-swings');

			expect(result).toEqual({
				winRate: 85,
				weeklyProfit: '+$5,200',
				activeTrades: 5,
				closedThisWeek: 2
			});
		});
	});

	describe('error handling', () => {
		it('should propagate api.get errors', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('Database error'));

			await expect(fetchStats('explosive-swings')).rejects.toThrow('Database error');
		});
	});
});

// ===============================================================================
// TEST SUITE: fetchAllTrades
// ===============================================================================

describe('fetchAllTrades()', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	describe('successful requests', () => {
		it('should fetch trades with per_page parameter', async () => {
			mockApiGet.mockResolvedValueOnce([
				{
					id: 1,
					ticker: 'NVDA',
					status: 'open',
					entry_price: 142.5,
					exit_price: null,
					pnl_percent: null,
					entry_date: '2026-01-20T10:00:00Z',
					exit_date: null,
					direction: 'long'
				}
			]);

			await fetchAllTrades('explosive-swings');

			expect(mockApiGet).toHaveBeenCalledWith(
				'/api/trades/explosive-swings',
				expect.objectContaining({
					params: { per_page: 100 }
				})
			);
		});

		it('should return trades array directly', async () => {
			const trades = [
				{
					id: 1,
					ticker: 'NVDA',
					status: 'open',
					entry_price: 142.5,
					exit_price: null,
					pnl_percent: null,
					entry_date: '2026-01-20T10:00:00Z',
					exit_date: null,
					direction: 'long'
				},
				{
					id: 2,
					ticker: 'MSFT',
					status: 'closed',
					entry_price: 425.0,
					exit_price: 460.0,
					pnl_percent: 8.2,
					entry_date: '2026-01-15T10:00:00Z',
					exit_date: '2026-01-20T15:00:00Z',
					direction: 'long'
				}
			];
			mockApiGet.mockResolvedValueOnce(trades);

			const result = await fetchAllTrades('explosive-swings');

			expect(result).toHaveLength(2);
			expect(result[0].status).toBe('open');
			expect(result[1].status).toBe('closed');
		});

		it('should handle empty trades array', async () => {
			mockApiGet.mockResolvedValueOnce([]);

			const result = await fetchAllTrades('explosive-swings');

			expect(result).toEqual([]);
		});
	});

	describe('error handling', () => {
		it('should propagate api.get errors', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('Forbidden'));

			await expect(fetchAllTrades('explosive-swings')).rejects.toThrow('Forbidden');
		});
	});
});

// ===============================================================================
// TEST SUITE: fetchWeeklyVideo
// ===============================================================================

describe('fetchWeeklyVideo()', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	describe('successful requests', () => {
		it('should fetch weekly video with correct URL', async () => {
			mockApiGet.mockResolvedValueOnce({
				id: 1,
				video_title: 'Weekly Breakdown',
				video_url: 'https://iframe.mediadelivery.net/embed/585929/video-id',
				thumbnail_url: 'https://placehold.co/640x360',
				duration: '24:35',
				published_at: '2026-01-20T09:00:00Z',
				week_title: 'Week of January 20, 2026'
			});

			await fetchWeeklyVideo('explosive-swings');

			expect(mockApiGet).toHaveBeenCalledWith(
				'/api/weekly-video/explosive-swings',
				expect.any(Object)
			);
		});

		it('should return video data directly', async () => {
			const videoData = {
				id: 1,
				video_title: 'Weekly Breakdown',
				video_url: 'https://iframe.mediadelivery.net/embed/585929/video-id',
				thumbnail_url: 'https://placehold.co/640x360',
				duration: '24:35',
				published_at: '2026-01-20T09:00:00Z',
				week_title: 'Week of January 20, 2026'
			};
			mockApiGet.mockResolvedValueOnce(videoData);

			const result = await fetchWeeklyVideo('explosive-swings');

			expect(result).toMatchObject({
				id: 1,
				video_title: 'Weekly Breakdown',
				video_url: 'https://iframe.mediadelivery.net/embed/585929/video-id'
			});
		});
	});

	describe('error handling', () => {
		it('should return null on 404 error', async () => {
			const error = new Error('Not found');
			(error as any).name = 'ApiError';
			(error as any).statusCode = 404;
			mockApiGet.mockRejectedValueOnce(error);

			const result = await fetchWeeklyVideo('explosive-swings');

			expect(result).toBeNull();
		});

		it('should propagate non-404 errors', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('Server error'));

			await expect(fetchWeeklyVideo('explosive-swings')).rejects.toThrow('Server error');
		});
	});
});

// ===============================================================================
// TEST SUITE: checkAdminStatus
// ===============================================================================

describe('checkAdminStatus()', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	describe('successful requests', () => {
		it('should call correct endpoint', async () => {
			mockApiGet.mockResolvedValueOnce({ is_admin: false, role: 'user' });

			await checkAdminStatus();

			expect(mockApiGet).toHaveBeenCalledWith('/api/auth/me', expect.any(Object));
		});

		it('should return true for is_admin=true', async () => {
			mockApiGet.mockResolvedValueOnce({ is_admin: true, role: 'user' });

			const result = await checkAdminStatus();

			expect(result).toBe(true);
		});

		it('should return true for role=admin', async () => {
			mockApiGet.mockResolvedValueOnce({ is_admin: false, role: 'admin' });

			const result = await checkAdminStatus();

			expect(result).toBe(true);
		});

		it('should return true for role=super_admin', async () => {
			mockApiGet.mockResolvedValueOnce({ is_admin: false, role: 'super_admin' });

			const result = await checkAdminStatus();

			expect(result).toBe(true);
		});

		it('should return false for regular user', async () => {
			mockApiGet.mockResolvedValueOnce({ is_admin: false, role: 'user' });

			const result = await checkAdminStatus();

			expect(result).toBe(false);
		});
	});

	describe('error handling', () => {
		it('should return false on error', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('Unauthorized'));

			const result = await checkAdminStatus();

			expect(result).toBe(false);
		});

		it('should return false on network error', async () => {
			mockApiGet.mockRejectedValueOnce(new Error('Network error'));

			const result = await checkAdminStatus();
			expect(result).toBe(false);
		});
	});
});

// ===============================================================================
// TEST SUITE: Time Formatting
// ===============================================================================

describe('formatTimeAgo (internal function tested via fetchAlerts)', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	it('should format recent time as minutes ago', async () => {
		const fiveMinutesAgo = new Date(Date.now() - 5 * 60 * 1000).toISOString();
		mockApiGet.mockResolvedValueOnce({
			data: [
				{
					id: 1,
					alert_type: 'ENTRY',
					ticker: 'NVDA',
					title: 'Test',
					published_at: fiveMinutesAgo,
					message: 'Test',
					is_new: true,
					notes: '',
					tos_string: null
				}
			],
			total: 1
		});

		const result = await fetchAlerts('explosive-swings', 1, 10);

		expect(result.alerts[0].time).toMatch(/\d+ min ago/);
	});

	it('should format hours ago', async () => {
		const threeHoursAgo = new Date(Date.now() - 3 * 60 * 60 * 1000).toISOString();
		mockApiGet.mockResolvedValueOnce({
			data: [
				{
					id: 1,
					alert_type: 'ENTRY',
					ticker: 'NVDA',
					title: 'Test',
					published_at: threeHoursAgo,
					message: 'Test',
					is_new: false,
					notes: '',
					tos_string: null
				}
			],
			total: 1
		});

		const result = await fetchAlerts('explosive-swings', 1, 10);

		expect(result.alerts[0].time).toMatch(/3h ago/);
	});

	it('should format yesterday', async () => {
		const yesterday = new Date(Date.now() - 24 * 60 * 60 * 1000 - 1000).toISOString();
		mockApiGet.mockResolvedValueOnce({
			data: [
				{
					id: 1,
					alert_type: 'ENTRY',
					ticker: 'NVDA',
					title: 'Test',
					published_at: yesterday,
					message: 'Test',
					is_new: false,
					notes: '',
					tos_string: null
				}
			],
			total: 1
		});

		const result = await fetchAlerts('explosive-swings', 1, 10);

		expect(result.alerts[0].time).toMatch(/Yesterday at/);
	});

	it('should format older dates with month and day', async () => {
		const lastWeek = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString();
		mockApiGet.mockResolvedValueOnce({
			data: [
				{
					id: 1,
					alert_type: 'EXIT',
					ticker: 'MSFT',
					title: 'Test',
					published_at: lastWeek,
					message: 'Test',
					is_new: false,
					notes: '',
					tos_string: null
				}
			],
			total: 1
		});

		const result = await fetchAlerts('explosive-swings', 1, 10);

		// Should contain month (e.g., "Jan 13 at 10:30 AM")
		expect(result.alerts[0].time).toMatch(/\w{3} \d+ at/);
	});
});

// ===============================================================================
// TEST SUITE: Edge Cases
// ===============================================================================

describe('Edge Cases', () => {
	beforeEach(() => {
		resetAllMocks();
	});

	it('should handle very large page numbers', async () => {
		mockApiGet.mockResolvedValueOnce({ data: [], total: 0 });

		await fetchAlerts('explosive-swings', 1000, 10);

		expect(mockApiGet).toHaveBeenCalledWith(
			'/api/alerts/explosive-swings',
			expect.objectContaining({
				params: { limit: 10, offset: 9990 }
			})
		);
	});

	it('should handle API returning empty data array', async () => {
		mockApiGet.mockResolvedValueOnce([]);

		const result = await fetchTradePlan('explosive-swings');

		expect(result).toEqual([]);
	});

	it('should handle malformed date strings gracefully', async () => {
		mockApiGet.mockResolvedValueOnce({
			data: [
				{
					id: 1,
					alert_type: 'ENTRY',
					ticker: 'NVDA',
					title: 'Test',
					published_at: 'invalid-date',
					message: 'Test',
					is_new: false,
					notes: '',
					tos_string: null
				}
			],
			total: 1
		});

		// Should not throw, but time formatting may produce unexpected results
		const result = await fetchAlerts('explosive-swings', 1, 10);
		expect(result.alerts[0].time).toBeDefined();
	});
});

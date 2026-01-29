/**
 * ===============================================================================
 * Explosive Swings - Test Setup Module
 * ===============================================================================
 *
 * @description Comprehensive test setup with mocks for Explosive Swings tests
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Built for the next 10 years
 *
 * This module provides:
 * - Global mocks for browser APIs
 * - SvelteKit module mocks
 * - Test fixtures and factory functions
 * - Utility functions for common test scenarios
 */

import { vi, beforeEach, afterEach } from 'vitest';

// ===============================================================================
// BROWSER API MOCKS
// ===============================================================================

/**
 * Mock localStorage with full Storage interface
 */
export const mockLocalStorage = {
	data: {} as Record<string, string>,
	getItem: vi.fn((key: string) => mockLocalStorage.data[key] || null),
	setItem: vi.fn((key: string, value: string) => {
		mockLocalStorage.data[key] = value;
	}),
	removeItem: vi.fn((key: string) => {
		delete mockLocalStorage.data[key];
	}),
	clear: vi.fn(() => {
		mockLocalStorage.data = {};
	}),
	get length() {
		return Object.keys(mockLocalStorage.data).length;
	},
	key: vi.fn((index: number) => Object.keys(mockLocalStorage.data)[index] || null)
};

/**
 * Mock navigator with clipboard support
 */
export const mockNavigator = {
	userAgent: 'Mozilla/5.0 (Test Environment)',
	language: 'en-US',
	clipboard: {
		writeText: vi.fn(() => Promise.resolve()),
		readText: vi.fn(() => Promise.resolve(''))
	},
	sendBeacon: vi.fn(() => true)
};

/**
 * Mock window object with commonly used properties
 */
export const mockWindow = {
	location: {
		href: 'http://localhost:5174/dashboard/explosive-swings',
		pathname: '/dashboard/explosive-swings',
		origin: 'http://localhost:5174',
		search: '',
		hash: ''
	},
	history: {
		pushState: vi.fn(),
		replaceState: vi.fn(),
		back: vi.fn(),
		forward: vi.fn()
	},
	addEventListener: vi.fn(),
	removeEventListener: vi.fn(),
	dispatchEvent: vi.fn(),
	scrollTo: vi.fn(),
	innerWidth: 1280,
	innerHeight: 720,
	localStorage: mockLocalStorage,
	navigator: mockNavigator,
	matchMedia: vi.fn((query: string) => ({
		matches: false,
		media: query,
		onchange: null,
		addListener: vi.fn(),
		removeListener: vi.fn(),
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn()
	}))
};

/**
 * Mock fetch with configurable responses
 */
export const mockFetch = vi.fn();

/**
 * Configure mock fetch to return a successful response
 */
export function mockFetchSuccess<T>(data: T, options: Partial<Response> = {}) {
	mockFetch.mockResolvedValueOnce({
		ok: true,
		status: 200,
		json: () => Promise.resolve(data),
		text: () => Promise.resolve(JSON.stringify(data)),
		headers: new Headers(),
		...options
	});
}

/**
 * Configure mock fetch to return an error response
 */
export function mockFetchError(status: number, message: string) {
	mockFetch.mockResolvedValueOnce({
		ok: false,
		status,
		json: () => Promise.resolve({ error: message }),
		text: () => Promise.resolve(message),
		headers: new Headers()
	});
}

/**
 * Configure mock fetch to throw a network error
 */
export function mockFetchNetworkError(message = 'Network error') {
	mockFetch.mockRejectedValueOnce(new Error(message));
}

// ===============================================================================
// SVELTEKIT MODULE MOCKS
// ===============================================================================

/**
 * Mock SvelteKit navigation functions
 */
vi.mock('$app/navigation', () => ({
	goto: vi.fn(() => Promise.resolve()),
	invalidate: vi.fn(() => Promise.resolve()),
	invalidateAll: vi.fn(() => Promise.resolve()),
	preloadCode: vi.fn(() => Promise.resolve()),
	preloadData: vi.fn(() => Promise.resolve()),
	beforeNavigate: vi.fn(),
	afterNavigate: vi.fn()
}));

/**
 * Mock SvelteKit stores
 */
vi.mock('$app/stores', () => ({
	page: {
		subscribe: vi.fn((fn) => {
			fn({
				url: new URL('http://localhost:5174/dashboard/explosive-swings'),
				params: {},
				route: { id: '/dashboard/explosive-swings' },
				status: 200,
				error: null,
				data: {},
				form: null
			});
			return () => {};
		})
	},
	navigating: {
		subscribe: vi.fn((fn) => {
			fn(null);
			return () => {};
		})
	},
	updated: {
		subscribe: vi.fn((fn) => {
			fn(false);
			return () => {};
		}),
		check: vi.fn(() => Promise.resolve(false))
	}
}));

/**
 * Mock SvelteKit environment
 */
vi.mock('$app/environment', () => ({
	browser: true,
	dev: true,
	building: false,
	version: 'test-1.0.0'
}));

// ===============================================================================
// TEST FIXTURES - Factory Functions
// ===============================================================================

/**
 * Create a mock alert with customizable properties
 */
export function createMockAlert(overrides: Partial<MockAlert> = {}): MockAlert {
	return {
		id: Math.floor(Math.random() * 10000),
		type: 'ENTRY',
		ticker: 'NVDA',
		title: 'Test Alert',
		time: 'Today at 10:30 AM',
		message: 'This is a test alert message',
		isNew: false,
		notes: '',
		tosString: undefined,
		...overrides
	};
}

export interface MockAlert {
	id: number;
	type: 'ENTRY' | 'EXIT' | 'UPDATE';
	ticker: string;
	title: string;
	time: string;
	message: string;
	isNew: boolean;
	notes: string;
	tosString?: string;
	entryPrice?: number;
	targetPrice?: number;
	stopPrice?: number;
	resultPercent?: number;
}

/**
 * Create a mock trade with customizable properties
 */
export function createMockTrade(overrides: Partial<MockTrade> = {}): MockTrade {
	return {
		id: Math.floor(Math.random() * 10000),
		ticker: 'NVDA',
		status: 'open',
		entry_price: 142.5,
		exit_price: null,
		pnl_percent: null,
		entry_date: new Date().toISOString(),
		exit_date: null,
		direction: 'long',
		setup: 'breakout',
		notes: 'Test trade',
		...overrides
	};
}

export interface MockTrade {
	id: number;
	ticker: string;
	status: 'open' | 'closed';
	entry_price: number;
	exit_price: number | null;
	pnl_percent: number | null;
	entry_date: string;
	exit_date: string | null;
	direction: string;
	setup?: string;
	notes?: string;
}

/**
 * Create a mock trade plan entry
 */
export function createMockTradePlanEntry(
	overrides: Partial<MockTradePlanEntry> = {}
): MockTradePlanEntry {
	return {
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
		notes: 'Breakout above consolidation',
		...overrides
	};
}

export interface MockTradePlanEntry {
	ticker: string;
	bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	options_strike: string;
	options_exp: string;
	notes: string;
}

/**
 * Create mock stats
 */
export function createMockStats(overrides: Partial<MockStats> = {}): MockStats {
	return {
		win_rate: 82,
		weekly_profit: '+$4,850',
		active_trades: 4,
		closed_this_week: 2,
		...overrides
	};
}

export interface MockStats {
	win_rate: number;
	weekly_profit: string;
	active_trades: number;
	closed_this_week: number;
}

/**
 * Create a mock weekly video
 */
export function createMockWeeklyVideo(overrides: Partial<MockWeeklyVideo> = {}): MockWeeklyVideo {
	return {
		id: 1,
		video_title: 'Weekly Breakdown: Top Swing Setups',
		video_url: 'https://iframe.mediadelivery.net/embed/585929/test-video-id',
		thumbnail_url: 'https://placehold.co/640x360',
		duration: '24:35',
		published_at: new Date().toISOString(),
		week_title: 'Week of January 20, 2026',
		...overrides
	};
}

export interface MockWeeklyVideo {
	id: number;
	video_title: string;
	video_url: string;
	thumbnail_url: string | null;
	duration: string | null;
	published_at: string;
	week_title: string;
}

/**
 * Create a mock active position
 */
export function createMockActivePosition(
	overrides: Partial<MockActivePosition> = {}
): MockActivePosition {
	return {
		id: '1',
		ticker: 'NVDA',
		status: 'ACTIVE',
		entryPrice: 142.5,
		currentPrice: 145.0,
		unrealizedPercent: 1.75,
		targets: [
			{ price: 156, percentFromEntry: 9.5, label: 'Target 1' },
			{ price: 168, percentFromEntry: 17.9, label: 'Target 2' }
		],
		stopLoss: { price: 136, percentFromEntry: -4.6 },
		progressToTarget1: 18.5,
		triggeredAt: new Date(),
		notes: undefined,
		wasUpdated: false,
		updatedAt: undefined,
		...overrides
	};
}

export interface MockActivePosition {
	id: string;
	ticker: string;
	status: 'ENTRY' | 'WATCHING' | 'ACTIVE';
	entryPrice: number | null;
	entryZone?: { low: number; high: number };
	currentPrice: number;
	unrealizedPercent: number | null;
	targets: Array<{ price: number; percentFromEntry: number; label: string }>;
	stopLoss: { price: number; percentFromEntry: number };
	progressToTarget1: number;
	triggeredAt?: Date;
	notes?: string;
	wasUpdated?: boolean;
	updatedAt?: Date;
}

/**
 * Create a mock closed trade
 */
export function createMockClosedTrade(overrides: Partial<MockClosedTrade> = {}): MockClosedTrade {
	return {
		id: '1',
		ticker: 'MSFT',
		percentageGain: 8.2,
		isWinner: true,
		closedAt: new Date(),
		entryPrice: 425,
		exitPrice: 460,
		...overrides
	};
}

export interface MockClosedTrade {
	id: string;
	ticker: string;
	percentageGain: number;
	isWinner: boolean;
	closedAt: Date;
	entryPrice: number;
	exitPrice: number;
}

/**
 * Create mock weekly performance
 */
export function createMockWeeklyPerformance(
	overrides: Partial<MockWeeklyPerformance> = {}
): MockWeeklyPerformance {
	return {
		winRate: 82,
		totalTrades: 7,
		winningTrades: 6,
		avgWinPercent: 5.7,
		avgLossPercent: 2.1,
		riskRewardRatio: 3.2,
		...overrides
	};
}

export interface MockWeeklyPerformance {
	winRate: number;
	totalTrades: number;
	winningTrades: number;
	avgWinPercent: number;
	avgLossPercent: number;
	riskRewardRatio: number;
}

// ===============================================================================
// API RESPONSE MOCKS
// ===============================================================================

/**
 * Create a mock API response for alerts
 */
export function createMockAlertsResponse(
	alerts: MockAlert[] = [createMockAlert()],
	total = alerts.length
) {
	return {
		success: true,
		data: alerts.map((alert) => ({
			id: alert.id,
			alert_type: alert.type,
			ticker: alert.ticker,
			title: alert.title,
			published_at: new Date().toISOString(),
			message: alert.message,
			is_new: alert.isNew,
			notes: alert.notes,
			tos_string: alert.tosString
		})),
		total
	};
}

/**
 * Create a mock API response for trades
 */
export function createMockTradesResponse(trades: MockTrade[] = [createMockTrade()]) {
	return {
		success: true,
		data: trades
	};
}

/**
 * Create a mock API response for trade plans
 */
export function createMockTradePlansResponse(
	entries: MockTradePlanEntry[] = [createMockTradePlanEntry()]
) {
	return {
		success: true,
		data: entries
	};
}

/**
 * Create a mock API response for stats
 */
export function createMockStatsResponse(stats: MockStats = createMockStats()) {
	return {
		success: true,
		data: stats
	};
}

/**
 * Create a mock API response for weekly video
 */
export function createMockWeeklyVideoResponse(video: MockWeeklyVideo = createMockWeeklyVideo()) {
	return {
		success: true,
		data: video
	};
}

/**
 * Create a mock admin status response
 */
export function createMockAdminResponse(isAdmin = false) {
	return {
		is_admin: isAdmin,
		role: isAdmin ? 'admin' : 'user'
	};
}

// ===============================================================================
// TEST LIFECYCLE HOOKS
// ===============================================================================

/**
 * Reset all mocks to their initial state
 */
export function resetAllMocks() {
	mockLocalStorage.data = {};
	mockFetch.mockClear();
	vi.clearAllMocks();
}

/**
 * Setup global test environment
 * Call this in your test file's top-level or in vitest.setup.ts
 */
export function setupTestEnvironment() {
	vi.stubGlobal('localStorage', mockLocalStorage);
	vi.stubGlobal('navigator', mockNavigator);
	vi.stubGlobal('fetch', mockFetch);

	// Partial window mock (avoid replacing entire window in JSDOM)
	if (typeof window !== 'undefined') {
		Object.defineProperty(window, 'matchMedia', {
			writable: true,
			value: mockWindow.matchMedia
		});
	}

	beforeEach(() => {
		resetAllMocks();
	});

	afterEach(() => {
		vi.clearAllTimers();
	});
}

// ===============================================================================
// TESTING UTILITIES
// ===============================================================================

/**
 * Wait for a specified number of milliseconds
 */
export function wait(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Wait for all pending promises to resolve
 */
export function flushPromises(): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, 0));
}

/**
 * Create a deferred promise for testing async flows
 */
export function createDeferred<T>() {
	let resolve!: (value: T) => void;
	let reject!: (reason?: unknown) => void;
	const promise = new Promise<T>((res, rej) => {
		resolve = res;
		reject = rej;
	});
	return { promise, resolve, reject };
}

/**
 * Assert that a value is defined (narrows type)
 */
export function assertDefined<T>(
	value: T | undefined | null,
	message = 'Value is not defined'
): asserts value is T {
	if (value === undefined || value === null) {
		throw new Error(message);
	}
}

// ===============================================================================
// CONSTANTS
// ===============================================================================

export const TEST_ROOM_SLUG = 'explosive-swings';
export const TEST_ALERTS_PER_PAGE = 10;
export const TEST_BASE_URL = 'http://localhost:5174';
export const TEST_API_URL = 'http://localhost:8000';

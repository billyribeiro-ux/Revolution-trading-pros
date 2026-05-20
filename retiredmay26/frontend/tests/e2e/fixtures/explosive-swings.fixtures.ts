/**
 * ===============================================================================
 * Explosive Swings - E2E Test Fixtures
 * ===============================================================================
 *
 * @description Comprehensive test data fixtures for E2E tests
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Playwright January 2026 Patterns
 *
 * This module provides:
 * - Mock API responses for MSW or route interception
 * - Test data factories
 * - Common test scenarios
 * - Admin vs user contexts
 */

// ===============================================================================
// TYPE DEFINITIONS
// ===============================================================================

export interface AlertFixture {
	id: number;
	alert_type: 'ENTRY' | 'EXIT' | 'UPDATE';
	ticker: string;
	title: string;
	published_at: string;
	message: string;
	is_new: boolean;
	notes: string | null;
	tos_string: string | null;
}

export interface TradeFixture {
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

export interface TradePlanFixture {
	ticker: string;
	bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
	entry: string;
	target1: string;
	target2: string;
	target3: string;
	runner: string;
	stop: string;
	options_strike: string | null;
	options_exp: string | null;
	notes: string | null;
}

export interface StatsFixture {
	win_rate: number;
	weekly_profit: string;
	active_trades: number;
	closed_this_week: number;
}

export interface WeeklyVideoFixture {
	id: number;
	video_title: string;
	video_url: string;
	thumbnail_url: string | null;
	duration: string | null;
	published_at: string;
	week_title: string;
}

// ===============================================================================
// FIXTURE FACTORIES
// ===============================================================================

/**
 * Create a mock alert fixture
 */
export function createAlertFixture(overrides: Partial<AlertFixture> = {}): AlertFixture {
	return {
		id: Math.floor(Math.random() * 10000),
		alert_type: 'ENTRY',
		ticker: 'NVDA',
		title: 'Opening NVDA Swing Position',
		published_at: new Date().toISOString(),
		message: 'Entering NVDA at $142.50. First target $148, stop at $136.',
		is_new: false,
		notes: 'Strong momentum into earnings.',
		tos_string: null,
		...overrides
	};
}

/**
 * Create a mock trade fixture
 */
export function createTradeFixture(overrides: Partial<TradeFixture> = {}): TradeFixture {
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

/**
 * Create a mock trade plan fixture
 */
export function createTradePlanFixture(
	overrides: Partial<TradePlanFixture> = {}
): TradePlanFixture {
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

/**
 * Create mock stats fixture
 */
export function createStatsFixture(overrides: Partial<StatsFixture> = {}): StatsFixture {
	return {
		win_rate: 82,
		weekly_profit: '+$4,850',
		active_trades: 4,
		closed_this_week: 3,
		...overrides
	};
}

/**
 * Create mock weekly video fixture
 */
export function createWeeklyVideoFixture(
	overrides: Partial<WeeklyVideoFixture> = {}
): WeeklyVideoFixture {
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

// ===============================================================================
// MOCK API RESPONSES
// ===============================================================================

/**
 * Mock response for GET /api/alerts/:room
 */
export function mockAlertsResponse(alerts: AlertFixture[] = [], total?: number) {
	return {
		success: true,
		data: alerts,
		total: total ?? alerts.length
	};
}

/**
 * Mock response for GET /api/trades/:room
 */
export function mockTradesResponse(trades: TradeFixture[] = []) {
	return {
		success: true,
		data: trades
	};
}

/**
 * Mock response for GET /api/trade-plans/:room
 */
export function mockTradePlansResponse(plans: TradePlanFixture[] = []) {
	return {
		success: true,
		data: plans
	};
}

/**
 * Mock response for GET /api/stats/:room
 */
export function mockStatsResponse(stats: StatsFixture = createStatsFixture()) {
	return {
		success: true,
		data: stats
	};
}

/**
 * Mock response for GET /api/weekly-video/:room
 */
export function mockWeeklyVideoResponse(
	video: WeeklyVideoFixture | null = createWeeklyVideoFixture()
) {
	return {
		success: true,
		data: video
	};
}

/**
 * Mock response for GET /api/auth/me (admin)
 */
export function mockAdminUserResponse() {
	return {
		id: 1,
		email: 'admin@test.com',
		name: 'Test Admin',
		is_admin: true,
		role: 'admin'
	};
}

/**
 * Mock response for GET /api/auth/me (regular user)
 */
export function mockRegularUserResponse() {
	return {
		id: 2,
		email: 'user@test.com',
		name: 'Test User',
		is_admin: false,
		role: 'user'
	};
}

/**
 * Mock error response
 */
export function mockErrorResponse(message: string, status = 500) {
	return {
		success: false,
		error: message,
		status
	};
}

// ===============================================================================
// TEST SCENARIOS
// ===============================================================================

/**
 * Scenario: Dashboard with active trades and alerts
 */
export const activeDashboardScenario = {
	alerts: [
		createAlertFixture({ id: 1, alert_type: 'ENTRY', ticker: 'NVDA', is_new: true }),
		createAlertFixture({ id: 2, alert_type: 'UPDATE', ticker: 'TSLA', is_new: true }),
		createAlertFixture({ id: 3, alert_type: 'EXIT', ticker: 'MSFT', is_new: false }),
		createAlertFixture({ id: 4, alert_type: 'ENTRY', ticker: 'META', is_new: false }),
		createAlertFixture({ id: 5, alert_type: 'UPDATE', ticker: 'AMD', is_new: false })
	],
	trades: [
		createTradeFixture({ id: 1, ticker: 'NVDA', status: 'open', entry_price: 142.5 }),
		createTradeFixture({ id: 2, ticker: 'META', status: 'open', entry_price: 585.0 }),
		createTradeFixture({
			id: 3,
			ticker: 'MSFT',
			status: 'closed',
			entry_price: 425,
			exit_price: 460,
			pnl_percent: 8.2
		}),
		createTradeFixture({
			id: 4,
			ticker: 'AAPL',
			status: 'closed',
			entry_price: 185,
			exit_price: 194.4,
			pnl_percent: 5.1
		})
	],
	tradePlans: [
		createTradePlanFixture({ ticker: 'NVDA', bias: 'BULLISH' }),
		createTradePlanFixture({ ticker: 'TSLA', bias: 'BULLISH' }),
		createTradePlanFixture({ ticker: 'AMZN', bias: 'BULLISH' }),
		createTradePlanFixture({ ticker: 'AMD', bias: 'BEARISH' })
	],
	stats: createStatsFixture({ win_rate: 82, active_trades: 4, closed_this_week: 3 }),
	weeklyVideo: createWeeklyVideoFixture()
};

/**
 * Scenario: Empty dashboard (new week, no trades)
 */
export const emptyDashboardScenario = {
	alerts: [],
	trades: [],
	tradePlans: [],
	stats: createStatsFixture({
		win_rate: 0,
		active_trades: 0,
		closed_this_week: 0,
		weekly_profit: '$0'
	}),
	weeklyVideo: createWeeklyVideoFixture()
};

/**
 * Scenario: Dashboard with only closed trades
 */
export const closedTradesOnlyScenario = {
	alerts: [
		createAlertFixture({ id: 1, alert_type: 'EXIT', ticker: 'MSFT' }),
		createAlertFixture({ id: 2, alert_type: 'EXIT', ticker: 'AAPL' })
	],
	trades: [
		createTradeFixture({
			id: 1,
			ticker: 'MSFT',
			status: 'closed',
			entry_price: 425,
			exit_price: 460,
			pnl_percent: 8.2
		}),
		createTradeFixture({
			id: 2,
			ticker: 'AAPL',
			status: 'closed',
			entry_price: 185,
			exit_price: 194.4,
			pnl_percent: 5.1
		})
	],
	tradePlans: [],
	stats: createStatsFixture({ win_rate: 100, active_trades: 0, closed_this_week: 2 }),
	weeklyVideo: createWeeklyVideoFixture()
};

/**
 * Scenario: Dashboard with many alerts (pagination test)
 */
export const paginatedAlertsScenario = {
	alerts: Array.from({ length: 25 }, (_, i) =>
		createAlertFixture({
			id: i + 1,
			alert_type: ['ENTRY', 'UPDATE', 'EXIT'][i % 3] as 'ENTRY' | 'UPDATE' | 'EXIT',
			ticker: ['NVDA', 'TSLA', 'META', 'MSFT', 'AAPL'][i % 5],
			is_new: i < 3
		})
	),
	trades: [],
	tradePlans: [],
	stats: createStatsFixture(),
	weeklyVideo: createWeeklyVideoFixture()
};

// ===============================================================================
// TEST DATA - Specific Tickers
// ===============================================================================

export const testTickers = {
	NVDA: {
		name: 'NVIDIA Corporation',
		price: 142.5,
		targetPrice: 148.0,
		stopPrice: 136.0,
		bias: 'BULLISH' as const
	},
	TSLA: {
		name: 'Tesla Inc',
		price: 248.0,
		targetPrice: 265.0,
		stopPrice: 235.0,
		bias: 'BULLISH' as const
	},
	META: {
		name: 'Meta Platforms Inc',
		price: 585.0,
		targetPrice: 620.0,
		stopPrice: 565.0,
		bias: 'BULLISH' as const
	},
	AMD: {
		name: 'Advanced Micro Devices',
		price: 125.0,
		targetPrice: 115.0,
		stopPrice: 132.0,
		bias: 'BEARISH' as const
	},
	MSFT: {
		name: 'Microsoft Corporation',
		price: 425.0,
		targetPrice: 445.0,
		stopPrice: 410.0,
		bias: 'BULLISH' as const
	}
};

// ===============================================================================
// API ROUTE PATTERNS
// ===============================================================================

export const apiRoutes = {
	alerts: '/api/alerts/explosive-swings*',
	trades: '/api/trades/explosive-swings*',
	tradePlans: '/api/trade-plans/explosive-swings*',
	stats: '/api/stats/explosive-swings*',
	weeklyVideo: '/api/weekly-video/explosive-swings*',
	authMe: '/api/auth/me',
	adminAlerts: '/api/admin/alerts*',
	adminTrades: '/api/admin/trades*',
	adminTradePlans: '/api/admin/trade-plans*',
	adminVideos: '/api/admin/videos*'
};

// ===============================================================================
// SELECTORS - Data Test IDs (for E2E tests)
// ===============================================================================

export const selectors = {
	// Dashboard sections
	performanceSummary: '[data-testid="performance-summary"]',
	alertsFeed: '[data-testid="alerts-feed"]',
	tradePlan: '[data-testid="trade-plan"]',
	weeklyVideo: '[data-testid="weekly-video"]',

	// Performance metrics
	winRate: '[data-testid="win-rate"]',
	winLossRatio: '[data-testid="win-loss-ratio"]',

	// Alert cards
	alertCard: '[data-testid="alert-card"]',
	alertBadge: '[data-testid="alert-badge"]',
	newAlertIndicator: '[data-testid="new-alert-indicator"]',

	// Position cards
	positionCard: '[data-testid="position-card"]',
	tickerPill: '[data-testid="ticker-pill"]',

	// Filters
	filterAll: '[data-testid="filter-all"]',
	filterEntry: '[data-testid="filter-entry"]',
	filterExit: '[data-testid="filter-exit"]',
	filterUpdate: '[data-testid="filter-update"]',

	// Pagination
	pagination: '[data-testid="pagination"]',
	prevPage: '[data-testid="prev-page"]',
	nextPage: '[data-testid="next-page"]',

	// Admin actions
	addAlertBtn: '[data-testid="add-alert-btn"]',
	addTradeBtn: '[data-testid="add-trade-btn"]',
	editBtn: '[data-testid="edit-btn"]',
	deleteBtn: '[data-testid="delete-btn"]',

	// Modals
	alertModal: '[data-testid="alert-modal"]',
	tradeModal: '[data-testid="trade-modal"]',
	videoModal: '[data-testid="video-modal"]',

	// Loading states
	skeleton: '.skeleton',
	loadingSpinner: '[data-testid="loading-spinner"]',

	// Error states
	errorBanner: '[data-testid="error-banner"]',
	retryBtn: '[data-testid="retry-btn"]'
};

// ===============================================================================
// TEST USERS
// ===============================================================================

export const testUsers = {
	admin: {
		email: 'admin@revolutiontradingpros.com',
		password: 'AdminTest123!',
		role: 'admin'
	},
	subscriber: {
		email: 'subscriber@test.com',
		password: 'SubscriberTest123!',
		role: 'subscriber'
	},
	guest: {
		email: null,
		password: null,
		role: 'guest'
	}
};

// ===============================================================================
// VIEWPORT CONFIGURATIONS
// ===============================================================================

export const viewports = {
	mobile: { width: 375, height: 667 },
	tablet: { width: 768, height: 1024 },
	desktop: { width: 1280, height: 720 },
	widescreen: { width: 1920, height: 1080 }
};

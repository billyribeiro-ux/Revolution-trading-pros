/**
 * ═══════════════════════════════════════════════════════════════════════════════
 * Explosive Swings - Fallback Data
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * @description Mock/fallback data used when API is unavailable
 * @version 1.0.0
 * @note This data is for development and graceful degradation only
 */

import type {
	ClosedTrade,
	ActivePosition,
	WeeklyPerformance,
	ThirtyDayPerformance,
	WeeklyContent,
	WeeklyVideo,
	Video,
	VideoUpdate,
	TradePlanEntry,
	QuickStats
} from '../types';
import type { FormattedAlert } from '../page.api';

// ═══════════════════════════════════════════════════════════════════════════
// FALLBACK DATA OBJECT - Single export for all fallback data
// ═══════════════════════════════════════════════════════════════════════════

export const fallbackData = {
	// Weekly Content for Hero Section
	weeklyContent: {
		title: `Week of ${new Date().toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}`,
		videoTitle: 'Mastering the Trade Dashboard — Simpler Trading',
		videoUrl: 'https://iframe.mediadelivery.net/embed/585929/d8477a4f-6b2f-4a2a-9e3e-e8e3c1e0e0e0',
		thumbnail: 'https://placehold.co/1280x720/143E59/FFFFFF/png?text=Weekly+Video',
		duration: '24:35',
		publishedDate: new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' }) + ' at 6:40 PM ET'
	} as WeeklyContent,

	// Trade Plan Entries
	tradePlan: [
		{
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
			notes: 'Breakout above consolidation. Wait for pullback to entry.'
		},
		{
			ticker: 'TSLA',
			bias: 'BULLISH',
			entry: '$248.00',
			target1: '$255.00',
			target2: '$265.00',
			target3: '$275.00',
			runner: '$290.00+',
			stop: '$235.00',
			optionsStrike: '$250 Call',
			optionsExp: 'Jan 31, 2026',
			notes: 'Momentum building. Earnings catalyst ahead.'
		},
		{
			ticker: 'AMZN',
			bias: 'BULLISH',
			entry: '$185.00',
			target1: '$190.00',
			target2: '$195.00',
			target3: '$198.00',
			runner: '$205.00+',
			stop: '$178.00',
			optionsStrike: '$185 Call',
			optionsExp: 'Jan 24, 2026',
			notes: 'Breaking resistance. Strong volume confirmation.'
		},
		{
			ticker: 'GOOGL',
			bias: 'NEUTRAL',
			entry: '$175.50',
			target1: '$180.00',
			target2: '$185.00',
			target3: '$188.00',
			runner: '$195.00+',
			stop: '$168.00',
			optionsStrike: '$177.50 Call',
			optionsExp: 'Feb 7, 2026',
			notes: 'Watching for breakout. Not triggered yet.'
		},
		{
			ticker: 'META',
			bias: 'BULLISH',
			entry: '$585.00',
			target1: '$600.00',
			target2: '$615.00',
			target3: '$630.00',
			runner: '$650.00+',
			stop: '$565.00',
			optionsStrike: '$590 Call',
			optionsExp: 'Jan 24, 2026',
			notes: 'Strong trend. Buy dips to support.'
		},
		{
			ticker: 'AMD',
			bias: 'BEARISH',
			entry: '$125.00',
			target1: '$120.00',
			target2: '$115.00',
			target3: '$110.00',
			runner: '$100.00',
			stop: '$132.00',
			optionsStrike: '$122 Put',
			optionsExp: 'Jan 31, 2026',
			notes: 'Breakdown in progress. Short on bounces.'
		}
	] as TradePlanEntry[],

	// Quick Stats for KPIs
	stats: {
		winRate: 82,
		weeklyProfit: '+$4,850',
		activeTrades: 4,
		closedThisWeek: 2
	} as QuickStats,

	// Weekly Performance Metrics
	weeklyPerformance: {
		winRate: 82,
		totalTrades: 7,
		winningTrades: 6,
		avgWinPercent: 5.7,
		avgLossPercent: 2.1,
		riskRewardRatio: 3.2
	} as WeeklyPerformance,

	// 30-Day Performance
	thirtyDayPerformance: {
		winRate: 82,
		totalAlerts: 28,
		profitableAlerts: 23,
		avgWinPercent: 6.8,
		avgLossPercent: 2.1
	} as ThirtyDayPerformance,

	// Closed Trades for Ticker Pills
	closedTrades: [
		{ id: '1', ticker: 'MSFT', percentageGain: 8.2, isWinner: true, closedAt: new Date('2026-01-12'), entryPrice: 425, exitPrice: 460 },
		{ id: '2', ticker: 'AAPL', percentageGain: 5.1, isWinner: true, closedAt: new Date('2026-01-11'), entryPrice: 185, exitPrice: 194.4 },
		{ id: '3', ticker: 'GOOGL', percentageGain: 4.8, isWinner: true, closedAt: new Date('2026-01-10'), entryPrice: 175, exitPrice: 183.4 },
		{ id: '4', ticker: 'AMZN', percentageGain: 6.3, isWinner: true, closedAt: new Date('2026-01-10'), entryPrice: 185, exitPrice: 196.7 },
		{ id: '5', ticker: 'AMD', percentageGain: 3.9, isWinner: true, closedAt: new Date('2026-01-09'), entryPrice: 125, exitPrice: 129.9 },
		{ id: '6', ticker: 'NFLX', percentageGain: -2.1, isWinner: false, closedAt: new Date('2026-01-09'), entryPrice: 520, exitPrice: 509.1 }
	] as ClosedTrade[],

	// Active Positions for Position Cards
	activePositions: [
		{
			id: '1',
			ticker: 'NVDA',
			status: 'ENTRY',
			entryPrice: 142.50,
			currentPrice: 143.80,
			unrealizedPercent: 0.9,
			targets: [
				{ price: 156, percentFromEntry: 9.5, label: 'Target 1' },
				{ price: 168, percentFromEntry: 17.9, label: 'Target 2' }
			],
			stopLoss: { price: 136, percentFromEntry: -4.6 },
			progressToTarget1: 9.6,
			triggeredAt: new Date('2026-01-13T10:32:00')
		},
		{
			id: '2',
			ticker: 'TSLA',
			status: 'WATCHING',
			entryPrice: null,
			entryZone: { low: 180, high: 185 },
			currentPrice: 188.40,
			unrealizedPercent: null,
			targets: [{ price: 210, percentFromEntry: 13.5, label: 'Target' }],
			stopLoss: { price: 172, percentFromEntry: -7.0 },
			notes: 'Waiting for pullback to zone',
			progressToTarget1: 0
		},
		{
			id: '3',
			ticker: 'META',
			status: 'ACTIVE',
			entryPrice: 585,
			currentPrice: 597.30,
			unrealizedPercent: 2.1,
			targets: [
				{ price: 620, percentFromEntry: 6.0, label: 'Target 1' },
				{ price: 645, percentFromEntry: 10.3, label: 'Target 2' }
			],
			stopLoss: { price: 558, percentFromEntry: -4.6 },
			progressToTarget1: 35,
			triggeredAt: new Date('2026-01-11T11:20:00')
		},
		{
			id: '4',
			ticker: 'AMD',
			status: 'ACTIVE',
			entryPrice: 125,
			currentPrice: 126.00,
			unrealizedPercent: 0.8,
			targets: [{ price: 138, percentFromEntry: 10.4, label: 'Target 1' }],
			stopLoss: { price: 118, percentFromEntry: -5.6 },
			progressToTarget1: 7.7,
			triggeredAt: new Date('2026-01-10T14:30:00')
		}
	] as ActivePosition[],

	// Weekly Video for sidebar
	weeklyVideo: {
		title: 'Top Swing Setups This Week',
		thumbnailUrl: 'https://placehold.co/640x360/143E59/FFFFFF/png?text=Weekly+Breakdown',
		videoUrl: '/dashboard/explosive-swings/video/weekly',
		duration: '24:35',
		publishedAt: new Date('2026-01-13T09:00:00')
	} as WeeklyVideo,

	// Recent Videos for grid
	recentVideos: [
		{ id: '1', ticker: 'NVDA', type: 'ENTRY', title: 'NVDA Entry Alert - Opening Swing Position', thumbnailUrl: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=NVDA+ENTRY', videoUrl: '/dashboard/explosive-swings/updates/nvda-entry', duration: '8:45', publishedAt: new Date('2026-01-13'), isFeatured: true },
		{ id: '2', ticker: 'MSFT', type: 'EXIT', title: 'MSFT Exit - Closing for +8.2% Profit', thumbnailUrl: 'https://placehold.co/640x360/3b82f6/FFFFFF/png?text=MSFT+EXIT', videoUrl: '/dashboard/explosive-swings/updates/msft-exit', duration: '6:20', publishedAt: new Date('2026-01-12') },
		{ id: '3', ticker: 'META', type: 'ENTRY', title: 'META Entry - Momentum Play Setup', thumbnailUrl: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=META+ENTRY', videoUrl: '/dashboard/explosive-swings/updates/meta-entry', duration: '7:15', publishedAt: new Date('2026-01-11') },
		{ id: '4', ticker: 'TSLA', type: 'UPDATE', title: 'TSLA Update - Approaching Entry Zone', thumbnailUrl: 'https://placehold.co/640x360/f59e0b/FFFFFF/png?text=TSLA+UPDATE', videoUrl: '/dashboard/explosive-swings/updates/tsla-update', duration: '5:30', publishedAt: new Date('2026-01-11') }
	] as Video[],

	// Video Updates for latest content section
	videoUpdates: [
		{
			id: 1,
			title: 'Weekly Swing Setup Review',
			date: 'January 13, 2026',
			excerpt: 'Top setups for the week ahead',
			href: '/daily/explosive-swings/weekly-review',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true,
			duration: '18:42'
		},
		{
			id: 2,
			title: 'NVDA Breakout Analysis',
			date: 'January 12, 2026',
			excerpt: 'Breaking down the NVDA trade setup',
			href: '/daily/explosive-swings/nvda-analysis',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true,
			duration: '12:15'
		},
		{
			id: 3,
			title: 'Market Momentum Update',
			date: 'January 10, 2026',
			excerpt: 'Current market conditions and outlook',
			href: '/daily/explosive-swings/market-update',
			image: 'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
			isVideo: true,
			duration: '15:30'
		}
	] as VideoUpdate[],

	// Alerts for alerts feed
	alerts: [
		{
			id: 1,
			type: 'ENTRY',
			ticker: 'NVDA',
			title: 'Opening NVDA Swing Position',
			time: 'Today at 10:32 AM',
			message: 'Entering NVDA at $142.50. First target $148, stop at $136. See trade plan for full details.',
			isNew: true,
			notes: 'Entry based on breakout above $142 resistance with strong volume confirmation. RSI at 62 showing momentum. Watch for pullback to $140 support if entry missed. Position size: 150 shares. Risk/reward: 2.8:1 to T2.'
		},
		{
			id: 2,
			type: 'UPDATE',
			ticker: 'TSLA',
			title: 'TSLA Approaching Entry Zone',
			time: 'Today at 9:15 AM',
			message: 'TSLA pulling back to our entry zone. Be ready. Will alert when triggered.',
			isNew: true,
			notes: 'Watching $248 entry level closely. Pullback is healthy after recent run. Volume declining on pullback (bullish). If entry triggers, will send immediate alert with exact entry price and position sizing.'
		},
		{
			id: 3,
			type: 'EXIT',
			ticker: 'MSFT',
			title: 'Closing MSFT for +8.2%',
			time: 'Yesterday at 3:45 PM',
			message: 'Taking profits on MSFT. Hit second target. +$2,450 on this trade.',
			isNew: false,
			notes: 'Excellent trade execution. Entered at $425, scaled out 1/3 at T1 ($435), another 1/3 at T2 ($445). Final exit at $460. Held for 5 days. Key lesson: Patience paid off - almost exited early on day 3 consolidation.'
		},
		{
			id: 4,
			type: 'ENTRY',
			ticker: 'META',
			title: 'META Entry Triggered',
			time: 'Yesterday at 11:20 AM',
			message: 'META hit our entry at $585. Position active. Targets in trade plan.',
			isNew: false,
			notes: 'Entry confirmed at $585 with volume spike. Stop placed at $565 (3.4% risk). Currently up 1.3% and holding well. Momentum strong with AI revenue narrative. Will trail stop after T1 hit.'
		},
		{
			id: 5,
			type: 'UPDATE',
			ticker: 'AMD',
			title: 'AMD Short Setup Active',
			time: 'Jan 10 at 2:30 PM',
			message: 'Bearish setup triggered on AMD. Short at $125 with stop at $132.',
			isNew: false,
			notes: 'Bearish breakdown confirmed. Entered short at $125, currently at $123.50 (-1.2%). Stop at $132 gives us 5.6% risk. First target $120, second target $115. Watch for bounce at $120 psychological level.'
		}
	] as FormattedAlert[]
};

// ═══════════════════════════════════════════════════════════════════════════
// CONSTANTS - Re-exported from centralized constants (ICT 7: Single Source of Truth)
// ═══════════════════════════════════════════════════════════════════════════

export { ROOM_SLUG, ALERTS_PER_PAGE } from '../constants';

<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Explosive Swings - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Member-first dashboard for swing trading alerts and analysis
	 * @version 4.1.0 - Visual Polish Pass
	 * @requires Svelte 5.0+ (January 2026 syntax)
	 * @requires SvelteKit 2.0+
	 * @standards Apple Principal Engineer ICT Level 11
	 *
	 * Features:
	 * - Weekly video breakdown with collapsible hero
	 * - Real-time trade plan with entry/exit targets
	 * - Live alerts feed with admin CRUD capabilities
	 * - Performance metrics sidebar with API integration
	 * - WCAG 2.1 AA accessibility compliance
	 */
	import { onMount } from 'svelte';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import TradeAlertModal from '$lib/components/dashboard/TradeAlertModal.svelte';
	
	// Extracted Components - Nuclear Build Specification (ICT 7+ Standards)
	// Global shared components
	import Pagination from '$lib/components/dashboard/pagination/Pagination.svelte';
	
	// Legacy components still in use (will be replaced incrementally)
	import AlertCard from '$lib/components/dashboard/alerts/AlertCard.svelte';
	import AlertFilters from '$lib/components/dashboard/alerts/AlertFilters.svelte';
	import StatsBar from '$lib/components/dashboard/stats/StatsBar.svelte';
	
	// Local page-specific components (Nuclear Build) - ready for integration
	import PerformanceSummary from './components/PerformanceSummary.svelte';
	import AlertsFeed from './components/AlertsFeed.svelte';
	import SidebarComponent from './components/Sidebar.svelte';
	import VideoGrid from './components/VideoGrid.svelte';
	
	// Types from local types.ts (Nuclear Build)
	import type { 
		ClosedTrade, 
		ActivePosition, 
		Alert as NuclearAlert,
		WeeklyPerformance,
		ThirtyDayPerformance,
		WeeklyVideo,
		Video
	} from './types';
	
	// Legacy types for backward compatibility with existing API
	import type { Alert, AlertFilter, QuickStats, VideoUpdate } from '$lib/components/dashboard/alerts/types';
	import type {
		TradePlanEntry as ApiTradePlanEntry,
		RoomAlert,
		RoomStats,
		AlertCreateInput,
		AlertUpdateInput
	} from '$lib/types/trading';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPE DEFINITIONS - Principal Engineer ICT 11 Standards
	// ═══════════════════════════════════════════════════════════════════════════
	interface WeeklyContent {
		title: string;
		videoTitle: string;
		videoUrl: string;
		thumbnail: string;
		duration: string;
		publishedDate: string;
	}

	interface TradePlanEntry {
		ticker: string;
		bias: 'BULLISH' | 'BEARISH' | 'NEUTRAL';
		entry: string;
		target1: string;
		target2: string;
		target3: string;
		runner: string;
		stop: string;
		optionsStrike: string;
		optionsExp: string;
		notes: string;
	}

	interface Alert {
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

	interface QuickStats {
		winRate: number;
		weeklyProfit: string;
		activeTrades: number;
		closedThisWeek: number;
	}

	interface VideoUpdate {
		id: number;
		title: string;
		date: string;
		excerpt: string;
		href: string;
		image: string;
		isVideo: boolean;
		duration: string;
	}

	type HeroTab = 'video' | 'entries';
	type AlertFilter = 'all' | 'entry' | 'exit' | 'update';

	// ═══════════════════════════════════════════════════════════════════════════
	// COMPONENT PROPS - Server data with types
	// ═══════════════════════════════════════════════════════════════════════════
	import type { WatchlistResponse } from '$lib/types/watchlist';
	import type { RoomResource } from '$lib/api/room-resources';
	import BunnyVideoPlayer from '$lib/components/video/BunnyVideoPlayer.svelte';

	// SvelteKit 2.0+ / Svelte 5: Props interface for type safety
	interface PageData {
		watchlist?: WatchlistResponse;
		tutorialVideo?: RoomResource | null;
		latestUpdates?: RoomResource[];
		documents?: RoomResource[];
		roomId?: number;
	}

	// Svelte 5 $props() rune - the official way to receive page data
	const { data }: { data: PageData } = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE - Svelte 5 $state runes
	// ═══════════════════════════════════════════════════════════════════════════
	let heroTab = $state<HeroTab>('video');
	let selectedFilter = $state<AlertFilter>('all');
	let isHeroCollapsed = $state(false); // Collapsible hero section
	let copiedAlertId = $state<number | null>(null); // Track which alert was just copied

	// Admin state
	let isAdmin = $state(false);
	let isAlertModalOpen = $state(false);
	let editingAlert = $state<RoomAlert | null>(null);
	let apiAlerts = $state<RoomAlert[]>([]);
	let apiTradePlan = $state<ApiTradePlanEntry[]>([]);
	let apiStats = $state<RoomStats | null>(null);
	let isLoadingAlerts = $state(false);
	let isLoadingTradePlan = $state(false);
	let isLoadingStats = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// NUCLEAR BUILD API STATE - Real data for performance summary
	// ═══════════════════════════════════════════════════════════════════════════
	interface ApiTrade {
		id: number;
		ticker: string;
		status: 'open' | 'closed';
		entry_price: number;
		exit_price: number | null;
		pnl_percent: number | null;
		entry_date: string;
		exit_date: string | null;
		direction: string;
		target1?: number;
		target2?: number;
		stop?: number;
		notes?: string;
	}

	interface ApiWeeklyVideo {
		id: number;
		video_title: string;
		video_url: string;
		thumbnail_url: string | null;
		duration: string | null;
		published_at: string;
		week_title: string;
	}

	let apiOpenTrades = $state<ApiTrade[]>([]);
	let apiClosedTrades = $state<ApiTrade[]>([]);
	let apiWeeklyVideo = $state<ApiWeeklyVideo | null>(null);
	let apiRecentVideos = $state<ApiWeeklyVideo[]>([]);
	let isLoadingTrades = $state(false);
	let isLoadingVideos = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════
	// PAGINATION STATE - Principal Engineer ICT 11 Standards
	// ═══════════════════════════════════════════════════════════════════════════
	let currentPage = $state(1);
	let alertsPerPage = $state(10);
	let pagination = $state({ total: 0, limit: 10, offset: 0 });

	// Derived pagination values
	const totalPages = $derived(Math.ceil(pagination.total / pagination.limit) || 1);
	const showingFrom = $derived(pagination.total > 0 ? (currentPage - 1) * alertsPerPage + 1 : 0);
	const showingTo = $derived(Math.min(currentPage * alertsPerPage, pagination.total));

	const ROOM_SLUG = 'explosive-swings';

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA - This Week's Content (fetched from API with fallback)
	// ═══════════════════════════════════════════════════════════════════════════

	// Fallback data constants
	const fallbackWeeklyContent: WeeklyContent = {
		title: 'Week of January 13, 2026',
		videoTitle: 'Weekly Breakdown: Top Swing Setups',
		videoUrl: 'https://player.vimeo.com/video/123456789',
		thumbnail: 'https://placehold.co/1280x720/143E59/FFFFFF/png?text=Weekly+Video+Breakdown',
		duration: '24:35',
		publishedDate: 'January 13, 2026 at 9:00 AM ET'
	};

	const fallbackTradePlan: TradePlanEntry[] = [
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
	];

	const fallbackStats: QuickStats = {
		winRate: 82,
		weeklyProfit: '+$4,850',
		activeTrades: 4,
		closedThisWeek: 2
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// NUCLEAR BUILD FALLBACK DATA - Used when API data is unavailable
	// These values are shown as placeholders until real data loads from:
	// - /api/trades/${ROOM_SLUG}?status=open (active positions)
	// - /api/trades/${ROOM_SLUG}?status=closed (closed trades)
	// - /api/stats/${ROOM_SLUG} (performance metrics)
	// - /api/weekly-video/${ROOM_SLUG} (weekly video)
	// ═══════════════════════════════════════════════════════════════════════════

	// FALLBACK: Weekly performance stats (replaced by derivedWeeklyPerformance)
	const weeklyPerformance: WeeklyPerformance = {
		winRate: 82,
		totalTrades: 7,
		winningTrades: 6,
		avgWinPercent: 5.7,
		avgLossPercent: 2.1,
		riskRewardRatio: 3.2
	};

	const closedTrades: ClosedTrade[] = [
		{ id: '1', ticker: 'MSFT', percentageGain: 8.2, isWinner: true, closedAt: new Date('2026-01-12'), entryPrice: 425, exitPrice: 460 },
		{ id: '2', ticker: 'AAPL', percentageGain: 5.1, isWinner: true, closedAt: new Date('2026-01-11'), entryPrice: 185, exitPrice: 194.4 },
		{ id: '3', ticker: 'GOOGL', percentageGain: 4.8, isWinner: true, closedAt: new Date('2026-01-10'), entryPrice: 175, exitPrice: 183.4 },
		{ id: '4', ticker: 'AMZN', percentageGain: 6.3, isWinner: true, closedAt: new Date('2026-01-10'), entryPrice: 185, exitPrice: 196.7 },
		{ id: '5', ticker: 'AMD', percentageGain: 3.9, isWinner: true, closedAt: new Date('2026-01-09'), entryPrice: 125, exitPrice: 129.9 },
		{ id: '6', ticker: 'NFLX', percentageGain: -2.1, isWinner: false, closedAt: new Date('2026-01-09'), entryPrice: 520, exitPrice: 509.1 }
	];

	const activePositions: ActivePosition[] = [
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
	];

	const thirtyDayPerformance: ThirtyDayPerformance = {
		winRate: 82,
		totalAlerts: 28,
		profitableAlerts: 23,
		avgWinPercent: 6.8,
		avgLossPercent: 2.1
	};

	const weeklyVideo: WeeklyVideo = {
		title: 'Top Swing Setups This Week',
		thumbnailUrl: 'https://placehold.co/640x360/143E59/FFFFFF/png?text=Weekly+Breakdown',
		videoUrl: '/dashboard/explosive-swings/video/weekly',
		duration: '24:35',
		publishedAt: new Date('2026-01-13T09:00:00')
	};

	const recentVideos: Video[] = [
		{ id: '1', ticker: 'NVDA', type: 'ENTRY', title: 'NVDA Entry Alert - Opening Swing Position', thumbnailUrl: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=NVDA+ENTRY', videoUrl: '/dashboard/explosive-swings/updates/nvda-entry', duration: '8:45', publishedAt: new Date('2026-01-13'), isFeatured: true },
		{ id: '2', ticker: 'MSFT', type: 'EXIT', title: 'MSFT Exit - Closing for +8.2% Profit', thumbnailUrl: 'https://placehold.co/640x360/3b82f6/FFFFFF/png?text=MSFT+EXIT', videoUrl: '/dashboard/explosive-swings/updates/msft-exit', duration: '6:20', publishedAt: new Date('2026-01-12') },
		{ id: '3', ticker: 'META', type: 'ENTRY', title: 'META Entry - Momentum Play Setup', thumbnailUrl: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=META+ENTRY', videoUrl: '/dashboard/explosive-swings/updates/meta-entry', duration: '7:15', publishedAt: new Date('2026-01-11') },
		{ id: '4', ticker: 'TSLA', type: 'UPDATE', title: 'TSLA Update - Approaching Entry Zone', thumbnailUrl: 'https://placehold.co/640x360/f59e0b/FFFFFF/png?text=TSLA+UPDATE', videoUrl: '/dashboard/explosive-swings/updates/tsla-update', duration: '5:30', publishedAt: new Date('2026-01-11') }
	];

	// ═══════════════════════════════════════════════════════════════════════════

	// Fallback video updates for latest content section
	const fallbackVideoUpdates: VideoUpdate[] = [
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
	];

	// Derive display items from API or fallback to static
	const displayUpdates = $derived(
		data.latestUpdates && data.latestUpdates.length > 0
			? data.latestUpdates.map((r) => ({
					id: r.id,
					title: r.title,
					date: r.formatted_date,
					excerpt: r.description || '',
					href: `/daily/explosive-swings/${r.slug}`,
					image:
						r.thumbnail_url ||
						'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg',
					isVideo: r.resource_type === 'video',
					duration: r.formatted_duration || ''
				}))
			: fallbackVideoUpdates
	);

	// Reactive data derived from server props - use $derived for reactivity
	// WatchlistResponse has entries[] and total - extract latest entry for weekly content
	const latestWatchlistEntry = $derived(data.watchlist?.entries?.[0]);
	const weeklyContent = $derived<WeeklyContent>(
		latestWatchlistEntry
			? {
					title: `Week of ${latestWatchlistEntry.weekOf}`,
					videoTitle: latestWatchlistEntry.title,
					videoUrl: latestWatchlistEntry.videoUrl,
					thumbnail:
						latestWatchlistEntry.poster ||
						'https://placehold.co/1280x720/143E59/FFFFFF/png?text=Weekly+Video',
					duration: '',
					publishedDate: latestWatchlistEntry.publishedAt || ''
				}
			: fallbackWeeklyContent
	);

	// Fallback alerts
	const fallbackAlerts: Alert[] = [
		{
			id: 1,
			type: 'ENTRY',
			ticker: 'NVDA',
			title: 'Opening NVDA Swing Position',
			time: 'Today at 10:32 AM',
			message:
				'Entering NVDA at $142.50. First target $148, stop at $136. See trade plan for full details.',
			isNew: true,
			notes:
				'Entry based on breakout above $142 resistance with strong volume confirmation. RSI at 62 showing momentum. Watch for pullback to $140 support if entry missed. Position size: 150 shares. Risk/reward: 2.8:1 to T2.'
		},
		{
			id: 2,
			type: 'UPDATE',
			ticker: 'TSLA',
			title: 'TSLA Approaching Entry Zone',
			time: 'Today at 9:15 AM',
			message: 'TSLA pulling back to our entry zone. Be ready. Will alert when triggered.',
			isNew: true,
			notes:
				'Watching $248 entry level closely. Pullback is healthy after recent run. Volume declining on pullback (bullish). If entry triggers, will send immediate alert with exact entry price and position sizing.'
		},
		{
			id: 3,
			type: 'EXIT',
			ticker: 'MSFT',
			title: 'Closing MSFT for +8.2%',
			time: 'Yesterday at 3:45 PM',
			message: 'Taking profits on MSFT. Hit second target. +$2,450 on this trade.',
			isNew: false,
			notes:
				'Excellent trade execution. Entered at $425, scaled out 1/3 at T1 ($435), another 1/3 at T2 ($445). Final exit at $460. Held for 5 days. Key lesson: Patience paid off - almost exited early on day 3 consolidation.'
		},
		{
			id: 4,
			type: 'ENTRY',
			ticker: 'META',
			title: 'META Entry Triggered',
			time: 'Yesterday at 11:20 AM',
			message: 'META hit our entry at $585. Position active. Targets in trade plan.',
			isNew: false,
			notes:
				'Entry confirmed at $585 with volume spike. Stop placed at $565 (3.4% risk). Currently up 1.3% and holding well. Momentum strong with AI revenue narrative. Will trail stop after T1 hit.'
		},
		{
			id: 5,
			type: 'UPDATE',
			ticker: 'AMD',
			title: 'AMD Short Setup Active',
			time: 'Jan 10 at 2:30 PM',
			message: 'Bearish setup triggered on AMD. Short at $125 with stop at $132.',
			isNew: false,
			notes:
				'Bearish breakdown confirmed. Entered short at $125, currently at $123.50 (-1.2%). Stop at $132 gives us 5.6% risk. First target $120, second target $115. Watch for bounce at $120 psychological level.'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function fetchAlerts() {
		isLoadingAlerts = true;
		try {
			const offset = (currentPage - 1) * alertsPerPage;
			const response = await fetch(`/api/alerts/${ROOM_SLUG}?limit=${alertsPerPage}&offset=${offset}`);
			const data = await response.json();
			if (data.success) {
				apiAlerts = data.data;
				pagination = {
					total: data.total || data.data.length,
					limit: alertsPerPage,
					offset: offset
				};
			}
		} catch (err) {
			console.error('Failed to fetch alerts:', err);
		} finally {
			isLoadingAlerts = false;
		}
	}

	async function fetchTradePlan() {
		isLoadingTradePlan = true;
		try {
			const response = await fetch(`/api/trade-plans/${ROOM_SLUG}`);
			const data = await response.json();
			if (data.success) {
				apiTradePlan = data.data;
			}
		} catch (err) {
			console.error('Failed to fetch trade plan:', err);
		} finally {
			isLoadingTradePlan = false;
		}
	}

	async function fetchStats() {
		isLoadingStats = true;
		try {
			const response = await fetch(`/api/stats/${ROOM_SLUG}`);
			const data = await response.json();
			if (data.success) {
				apiStats = data.data;
			}
		} catch (err) {
			console.error('Failed to fetch stats:', err);
		} finally {
			isLoadingStats = false;
		}
	}

	async function checkAdminStatus() {
		// Check if user has admin role from session/cookie
		// For now, we'll check a simple flag - in production this comes from auth
		try {
			const response = await fetch('/api/auth/me');
			const data = await response.json();
			isAdmin = data.user?.role === 'admin' || data.user?.role === 'super_admin';
		} catch {
			isAdmin = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// NUCLEAR BUILD API FUNCTIONS - Trades & Weekly Video
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Fetch open trades (active positions) from API
	 */
	async function fetchOpenTrades() {
		isLoadingTrades = true;
		try {
			const response = await fetch(`/api/trades/${ROOM_SLUG}?status=open`);
			const data = await response.json();
			if (data.success) {
				apiOpenTrades = data.data || [];
			}
		} catch (err) {
			console.error('Failed to fetch open trades:', err);
		} finally {
			isLoadingTrades = false;
		}
	}

	/**
	 * Fetch closed trades (for ticker pills) from API
	 */
	async function fetchClosedTrades() {
		try {
			const response = await fetch(`/api/trades/${ROOM_SLUG}?status=closed&per_page=10`);
			const data = await response.json();
			if (data.success) {
				apiClosedTrades = data.data || [];
			}
		} catch (err) {
			console.error('Failed to fetch closed trades:', err);
		}
	}

	/**
	 * Fetch current weekly video from API
	 */
	async function fetchWeeklyVideo() {
		isLoadingVideos = true;
		try {
			const response = await fetch(`/api/weekly-video/${ROOM_SLUG}`);
			const data = await response.json();
			if (data.success && data.data) {
				apiWeeklyVideo = data.data;
			}
		} catch (err) {
			console.error('Failed to fetch weekly video:', err);
		} finally {
			isLoadingVideos = false;
		}
	}

	/**
	 * Fetch all trades (for calculating performance metrics)
	 */
	async function fetchAllTrades() {
		try {
			const response = await fetch(`/api/trades/${ROOM_SLUG}?per_page=100`);
			const data = await response.json();
			if (data.success && data.data) {
				// Split into open and closed
				apiOpenTrades = data.data.filter((t: ApiTrade) => t.status === 'open');
				apiClosedTrades = data.data.filter((t: ApiTrade) => t.status === 'closed');
			}
		} catch (err) {
			console.error('Failed to fetch all trades:', err);
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PAGINATION FUNCTIONS - Principal Engineer ICT 11 Standards
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Navigate to a specific page and fetch alerts
	 */
	async function goToPage(page: number) {
		if (page < 1 || page > totalPages || page === currentPage) return;
		currentPage = page;
		await fetchAlerts();
		// Scroll to top of alerts section
		const alertsSection = document.querySelector('.alerts-section');
		if (alertsSection) {
			alertsSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
		}
	}

	/**
	 * Generate visible page numbers with ellipsis for pagination UI
	 */
	function getVisiblePages(current: number, total: number): (number | 'ellipsis')[] {
		if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1);

		const pages: (number | 'ellipsis')[] = [];

		// Always show first page
		pages.push(1);

		// Calculate range around current
		const rangeStart = Math.max(2, current - 1);
		const rangeEnd = Math.min(total - 1, current + 1);

		// Add ellipsis before range if needed
		if (rangeStart > 2) pages.push('ellipsis');

		// Add range
		for (let i = rangeStart; i <= rangeEnd; i++) {
			pages.push(i);
		}

		// Add ellipsis after range if needed
		if (rangeEnd < total - 1) pages.push('ellipsis');

		// Always show last page
		if (total > 1) pages.push(total);

		return pages;
	}

	// Derived visible pages for pagination UI
	const visiblePages = $derived(getVisiblePages(currentPage, totalPages));

	// Reset pagination when filter changes
	let previousFilter = $state<AlertFilter>('all');
	$effect(() => {
		if (selectedFilter !== previousFilter) {
			previousFilter = selectedFilter;
			currentPage = 1;
			fetchAlerts();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// ADMIN HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function openNewAlertModal() {
		editingAlert = null;
		isAlertModalOpen = true;
	}

	function openEditAlertModal(alert: RoomAlert) {
		editingAlert = alert;
		isAlertModalOpen = true;
	}

	function closeAlertModal() {
		isAlertModalOpen = false;
		editingAlert = null;
	}

	async function handleSaveAlert(alertData: AlertCreateInput | AlertUpdateInput, isEdit: boolean) {
		const url =
			isEdit && editingAlert
				? `/api/alerts/${ROOM_SLUG}/${editingAlert.id}`
				: `/api/alerts/${ROOM_SLUG}`;

		const response = await fetch(url, {
			method: isEdit ? 'PUT' : 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(alertData)
		});

		if (!response.ok) {
			throw new Error('Failed to save alert');
		}

		// Refresh alerts after save
		await fetchAlerts();
	}

	async function handleDeleteAlert(alertId: number) {
		if (!confirm('Are you sure you want to delete this alert?')) return;

		const response = await fetch(`/api/alerts/${ROOM_SLUG}/${alertId}`, {
			method: 'DELETE'
		});

		if (response.ok) {
			await fetchAlerts();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		checkAdminStatus();
		fetchAlerts();
		fetchTradePlan();
		fetchStats();
		// Nuclear Build API calls
		fetchAllTrades();
		fetchWeeklyVideo();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE - Transform API data to display format
	// ═══════════════════════════════════════════════════════════════════════════

	// Format time ago helper
	function formatTimeAgo(dateString: string): string {
		const date = new Date(dateString);
		const now = new Date();
		const diffMs = now.getTime() - date.getTime();
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMs / 3600000);
		const diffDays = Math.floor(diffMs / 86400000);

		if (diffMins < 60) return `${diffMins} min ago`;
		if (diffHours < 24) return `${diffHours}h ago`;
		if (diffDays === 0)
			return `Today at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;
		if (diffDays === 1)
			return `Yesterday at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`;
		return (
			date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }) +
			` at ${date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })}`
		);
	}

	// Derive alerts from API or fallback
	const alerts = $derived<Alert[]>(
		apiAlerts.length > 0
			? apiAlerts.map((a) => ({
					id: a.id,
					type: a.alert_type,
					ticker: a.ticker,
					title: a.title,
					time: formatTimeAgo(a.published_at),
					message: a.message,
					isNew: a.is_new,
					notes: a.notes || '',
					tosString: a.tos_string || undefined
				}))
			: fallbackAlerts
	);

	// Derive trade plan from API or fallback
	const tradePlan = $derived<TradePlanEntry[]>(
		apiTradePlan.length > 0
			? apiTradePlan.map((t) => ({
					ticker: t.ticker,
					bias: t.bias,
					entry: t.entry,
					target1: t.target1,
					target2: t.target2,
					target3: t.target3,
					runner: t.runner,
					stop: t.stop,
					optionsStrike: t.options_strike || '-',
					optionsExp: t.options_exp || '-',
					notes: t.notes || ''
				}))
			: fallbackTradePlan
	);

	// Derive stats from API or fallback
	const stats = $derived<QuickStats>(
		apiStats
			? {
					winRate: apiStats.win_rate,
					weeklyProfit: apiStats.weekly_profit,
					activeTrades: apiStats.active_trades,
					closedThisWeek: apiStats.closed_this_week
				}
			: fallbackStats
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// NUCLEAR BUILD DERIVED STATE - Transform API data to Nuclear Build formats
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Derive weekly performance from API stats or fallback
	 * Used by PerformanceSummary component
	 */
	const derivedWeeklyPerformance = $derived<WeeklyPerformance>(
		apiStats ? {
			winRate: apiStats.win_rate || 0,
			totalTrades: apiStats.total_trades || 0,
			winningTrades: apiStats.wins || 0,
			avgWinPercent: apiStats.avg_win ? (apiStats.avg_win / 100) : 0,
			avgLossPercent: apiStats.avg_loss ? (apiStats.avg_loss / 100) : 0,
			riskRewardRatio: apiStats.profit_factor || 0
		} : weeklyPerformance // fallback to hardcoded
	);

	/**
	 * Derive closed trades from API for TickerPill display
	 * Used by PerformanceSummary component
	 */
	const derivedClosedTrades = $derived<ClosedTrade[]>(
		apiClosedTrades.length > 0
			? apiClosedTrades.slice(0, 10).map((t) => ({
					id: String(t.id),
					ticker: t.ticker,
					percentageGain: t.pnl_percent || 0,
					isWinner: (t.pnl_percent || 0) > 0,
					closedAt: new Date(t.exit_date || t.entry_date),
					entryPrice: t.entry_price,
					exitPrice: t.exit_price || t.entry_price
				}))
			: closedTrades // fallback to hardcoded
	);

	/**
	 * Derive active positions from API for ActivePositionCard display
	 * Used by PerformanceSummary component
	 */
	const derivedActivePositions = $derived<ActivePosition[]>(
		apiOpenTrades.length > 0
			? apiOpenTrades.map((t) => {
					const currentPrice = t.entry_price * 1.01; // Simulate current price (would come from real-time data)
					const unrealizedPercent = ((currentPrice - t.entry_price) / t.entry_price) * 100;
					return {
						id: String(t.id),
						ticker: t.ticker,
						status: 'ACTIVE' as const,
						entryPrice: t.entry_price,
						currentPrice: currentPrice,
						unrealizedPercent: unrealizedPercent,
						targets: t.target1 ? [{ price: t.target1, percentFromEntry: ((t.target1 - t.entry_price) / t.entry_price) * 100, label: 'Target 1' }] : [],
						stopLoss: t.stop ? { price: t.stop, percentFromEntry: ((t.stop - t.entry_price) / t.entry_price) * 100 } : { price: t.entry_price * 0.95, percentFromEntry: -5 },
						progressToTarget1: t.target1 ? Math.max(0, Math.min(100, ((currentPrice - t.entry_price) / (t.target1 - t.entry_price)) * 100)) : 0,
						triggeredAt: new Date(t.entry_date)
					};
				})
			: activePositions // fallback to hardcoded
	);

	/**
	 * Derive 30-day performance from API stats
	 * Used by Sidebar PerformanceCard component
	 */
	const derivedThirtyDayPerformance = $derived<ThirtyDayPerformance>(
		apiStats ? {
			winRate: apiStats.win_rate || 0,
			totalAlerts: apiStats.total_trades || 0,
			profitableAlerts: apiStats.wins || 0,
			avgWinPercent: apiStats.avg_win ? (apiStats.avg_win / 100) : 0,
			avgLossPercent: apiStats.avg_loss ? (apiStats.avg_loss / 100) : 0
		} : thirtyDayPerformance // fallback to hardcoded
	);

	/**
	 * Derive weekly video from API
	 * Used by Sidebar WeeklyVideoCard and VideoGrid components
	 */
	const derivedWeeklyVideo = $derived<WeeklyVideo>(
		apiWeeklyVideo ? {
			title: apiWeeklyVideo.video_title,
			thumbnailUrl: apiWeeklyVideo.thumbnail_url || 'https://placehold.co/640x360/143E59/FFFFFF/png?text=Weekly+Breakdown',
			videoUrl: apiWeeklyVideo.video_url,
			duration: apiWeeklyVideo.duration || '',
			publishedAt: new Date(apiWeeklyVideo.published_at)
		} : weeklyVideo // fallback to hardcoded
	);

	/**
	 * Derive recent videos from closed trades alerts (videos are tied to alerts)
	 * Used by VideoGrid component
	 */
	const derivedRecentVideos = $derived<Video[]>(
		apiAlerts.length > 0
			? apiAlerts.slice(0, 4).map((a, i) => ({
					id: String(a.id),
					ticker: a.ticker,
					type: a.alert_type as 'ENTRY' | 'EXIT' | 'UPDATE',
					title: a.title,
					thumbnailUrl: `https://placehold.co/640x360/${a.alert_type === 'ENTRY' ? '22c55e' : a.alert_type === 'EXIT' ? '3b82f6' : 'f59e0b'}/FFFFFF/png?text=${a.ticker}+${a.alert_type}`,
					videoUrl: `/dashboard/explosive-swings/updates/${a.ticker.toLowerCase()}-${a.alert_type.toLowerCase()}`,
					duration: '5:00',
					publishedAt: new Date(a.published_at),
					isFeatured: i === 0
				}))
			: recentVideos // fallback to hardcoded
	);

	// Track which alert notes are expanded
	let expandedNotes = $state<Set<number>>(new Set());

	// Track which trade plan notes are expanded
	let expandedTradeNotes = $state<Set<string>>(new Set());

	function toggleTradeNotes(ticker: string) {
		const newExpanded = new Set(expandedTradeNotes);
		if (newExpanded.has(ticker)) {
			newExpanded.delete(ticker);
		} else {
			newExpanded.add(ticker);
		}
		expandedTradeNotes = newExpanded;
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

	// Extract price from alert message (e.g., "$142.50" from message text)
	function extractPrice(message: string): string | null {
		const priceMatch = message.match(/\$[\d,]+\.?\d*/g);
		return priceMatch ? priceMatch[0] : null;
	}

	// Copy trade details to clipboard
	async function copyTradeDetails(alert: Alert) {
		const tradeDetails = `${alert.ticker} ${alert.type}\n${alert.title}\n${alert.message}${alert.tosString ? '\nTOS: ' + alert.tosString : ''}`;
		try {
			await navigator.clipboard.writeText(tradeDetails);
			copiedAlertId = alert.id;
			setTimeout(() => {
				copiedAlertId = null;
			}, 2000);
		} catch (err) {
			console.error('Failed to copy:', err);
		}
	}

	// Toggle hero section collapse
	function toggleHeroCollapse() {
		isHeroCollapsed = !isHeroCollapsed;
	}

	// Filter alerts
	const filteredAlerts = $derived(
		selectedFilter === 'all'
			? alerts
			: alerts.filter((a) => a.type.toLowerCase() === selectedFilter)
	);

	const latestUpdates: VideoUpdate[] = [
		{
			id: 1,
			title: 'NVDA Entry Alert - Opening Swing Position',
			date: 'January 13, 2026 at 10:32 AM ET',
			excerpt:
				'Entering NVDA at $142.50. Watch this video breakdown for entry reasoning, targets, and risk management.',
			href: '/dashboard/explosive-swings/updates/nvda-entry-011326',
			image: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=NVDA+ENTRY',
			isVideo: true,
			duration: '8:45'
		},
		{
			id: 2,
			title: 'MSFT Exit - Closing for +8.2% Profit',
			date: 'January 12, 2026 at 3:45 PM ET',
			excerpt:
				'Taking profits on MSFT swing trade. Hit second target. Watch the exit strategy breakdown and lessons learned.',
			href: '/dashboard/explosive-swings/updates/msft-exit-011226',
			image: 'https://placehold.co/640x360/3b82f6/FFFFFF/png?text=MSFT+EXIT',
			isVideo: true,
			duration: '6:20'
		},
		{
			id: 3,
			title: 'META Entry - Momentum Play Setup',
			date: 'January 11, 2026 at 11:20 AM ET',
			excerpt:
				'META triggered our entry at $585. Full breakdown of the momentum setup and what to watch for.',
			href: '/dashboard/explosive-swings/updates/meta-entry-011126',
			image: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=META+ENTRY',
			isVideo: true,
			duration: '7:15'
		},
		{
			id: 4,
			title: 'TSLA Update - Approaching Entry Zone',
			date: 'January 11, 2026 at 9:15 AM ET',
			excerpt: 'TSLA pulling back to our entry zone. Technical analysis and what levels to watch.',
			href: '/dashboard/explosive-swings/updates/tsla-update-011126',
			image: 'https://placehold.co/640x360/f59e0b/FFFFFF/png?text=TSLA+UPDATE',
			isVideo: true,
			duration: '5:30'
		},
		{
			id: 5,
			title: 'AMD Short Setup - Bearish Breakdown',
			date: 'January 10, 2026 at 2:30 PM ET',
			excerpt:
				'AMD short position triggered at $125. Breakdown of the bearish setup and downside targets.',
			href: '/dashboard/explosive-swings/updates/amd-short-011026',
			image: 'https://placehold.co/640x360/ef4444/FFFFFF/png?text=AMD+SHORT',
			isVideo: true,
			duration: '9:10'
		},
		{
			id: 6,
			title: 'AMZN Entry - Breakout Confirmation',
			date: 'January 9, 2026 at 11:15 AM ET',
			excerpt: 'AMZN breaking above key resistance. Entry at $185 with full technical breakdown.',
			href: '/dashboard/explosive-swings/updates/amzn-entry-010926',
			image: 'https://placehold.co/640x360/22c55e/FFFFFF/png?text=AMZN+ENTRY',
			isVideo: true,
			duration: '7:45'
		}
	];
</script>

<svelte:head>
	<title>Explosive Swings Dashboard | Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
/>

<div class="explosive-swings-page">
	<!-- ═══════════════════════════════════════════════════════════════════════════
	     PERFORMANCE SUMMARY - Nuclear Build Specification
	     Replaces old StatsBar with ticker pills + active positions
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<PerformanceSummary
		performance={derivedWeeklyPerformance}
		closedTrades={derivedClosedTrades}
		activePositions={derivedActivePositions}
		isLoading={isLoadingStats || isLoadingTrades}
	/>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     HERO: Collapsible Weekly Video Accordion (Legacy - Will be removed)
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<section class="hero" class:collapsed={isHeroCollapsed}>
		<button class="hero-collapse-toggle" onclick={toggleHeroCollapse}>
			<div class="hero-header-compact">
				<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20" class="video-icon">
					<path d="M8 5v14l11-7z" />
				</svg>
				<h1>{weeklyContent.title} — Weekly Breakdown</h1>
			</div>
			<div class="collapse-indicator">
				<span>{isHeroCollapsed ? 'Expand' : 'Collapse'}</span>
				<svg class="collapse-chevron" class:rotated={!isHeroCollapsed} viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="20" height="20">
					<path d="M19 9l-7 7-7-7" />
				</svg>
			</div>
		</button>

		{#if !isHeroCollapsed}
			<div class="hero-tabs-bar">
				<button
					class="hero-tab"
					class:active={heroTab === 'video'}
					onclick={() => (heroTab = 'video')}
				>
					<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
						<path d="M8 5v14l11-7z" />
					</svg>
					Video Breakdown
				</button>
				<button
					class="hero-tab"
					class:active={heroTab === 'entries'}
					onclick={() => (heroTab = 'entries')}
				>
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="18"
						height="18"
					>
						<rect x="3" y="3" width="18" height="18" rx="2" />
						<path d="M3 9h18M9 21V9" />
					</svg>
					Trade Plan & Entries
				</button>
			</div>

			<div class="hero-content">
				{#if heroTab === 'video'}
					<!-- VIDEO TAB - Condensed -->
					<div class="video-container-compact">
						<div class="video-player-compact" style="background-image: url('{weeklyContent.thumbnail}')">
							<div class="video-overlay">
								<button class="play-btn" aria-label="Play video">
									<svg viewBox="0 0 24 24" fill="currentColor">
										<path d="M8 5v14l11-7z" />
									</svg>
								</button>
							</div>
							<div class="video-duration">{weeklyContent.duration}</div>
						</div>
						<div class="video-info-compact">
							<h2>{weeklyContent.videoTitle}</h2>
							<p>Published {weeklyContent.publishedDate}</p>
							<a href="/dashboard/explosive-swings/video/weekly" class="watch-btn">
								Watch Full Video
								<svg
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									width="18"
									height="18"
								>
									<path d="M5 12h14M12 5l7 7-7 7" />
								</svg>
							</a>
						</div>
					</div>
				{:else}
					<!-- ENTRIES TAB - Trade Plan Sheet -->
					<div class="entries-container">
						<div class="entries-header">
							<h2>This Week's Trade Plan</h2>
							<p>Complete breakdown with entries, targets, stops, and options plays</p>
						</div>
						<div class="trade-sheet-wrapper">
							<table class="trade-sheet">
								<thead>
									<tr>
										<th>Ticker</th>
										<th>Bias</th>
										<th>Entry</th>
										<th>Target 1</th>
										<th>Target 2</th>
										<th>Target 3</th>
										<th>Runner</th>
										<th>Stop</th>
										<th>Options</th>
										<th>Exp</th>
										<th class="notes-th">Notes</th>
									</tr>
								</thead>
								<tbody>
									{#each tradePlan as trade}
										<tr class:has-notes-open={expandedTradeNotes.has(trade.ticker)}>
											<td class="ticker-cell">
												<strong>{trade.ticker}</strong>
											</td>
											<td>
												<span class="bias bias--{trade.bias.toLowerCase()}">{trade.bias}</span>
											</td>
											<td class="entry-cell">{trade.entry}</td>
											<td class="target-cell">{trade.target1}</td>
											<td class="target-cell">{trade.target2}</td>
											<td class="target-cell">{trade.target3}</td>
											<td class="runner-cell">{trade.runner}</td>
											<td class="stop-cell">{trade.stop}</td>
											<td class="options-cell">{trade.optionsStrike}</td>
											<td class="exp-cell">{trade.optionsExp}</td>
											<td class="notes-toggle-cell">
												<button
													class="table-notes-btn"
													class:expanded={expandedTradeNotes.has(trade.ticker)}
													onclick={() => toggleTradeNotes(trade.ticker)}
													aria-label="Toggle notes for {trade.ticker}"
												>
													<svg
														class="chevron-icon"
														viewBox="0 0 24 24"
														fill="none"
														stroke="currentColor"
														stroke-width="2.5"
														width="18"
														height="18"
													>
														<path d="M19 9l-7 7-7-7" />
													</svg>
												</button>
											</td>
										</tr>
										{#if expandedTradeNotes.has(trade.ticker)}
											<tr class="notes-row expanded">
												<td colspan="11">
													<div class="trade-notes-panel">
														<div class="trade-notes-badge">{trade.ticker}</div>
														<p>{trade.notes}</p>
													</div>
												</td>
											</tr>
										{/if}
									{/each}
								</tbody>
							</table>
						</div>
						<div class="sheet-footer">
							<a
								href="https://docs.google.com/spreadsheets/d/your-sheet-id"
								target="_blank"
								class="google-sheet-link"
							>
								<svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
									<path
										d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM7 17h2v-7H7v7zm4 0h2V7h-2v10zm4 0h2v-4h-2v4z"
									/>
								</svg>
								Open in Google Sheets
							</a>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</section>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     MAIN CONTENT: Alerts + Sidebar
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<div class="main-grid">
		<!-- ALERTS FEED -->
		<section class="alerts-section">
			<div class="section-header">
				<div class="section-title-row">
					<h2>Live Alerts</h2>
					{#if isAdmin}
						<button class="admin-btn" onclick={openNewAlertModal}>
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="18"
								height="18"
							>
								<path d="M12 5v14M5 12h14" />
							</svg>
							New Alert
						</button>
					{/if}
				</div>
				<AlertFilters 
					selected={selectedFilter} 
					onFilterChange={(filter) => (selectedFilter = filter)} 
				/>
			</div>

			<div class="alerts-list">
				{#if isLoadingAlerts}
					<!-- Skeleton Loading States -->
					{#each [1, 2, 3] as _}
						<div class="alert-card-skeleton">
							<div class="skeleton-row">
								<div class="skeleton-badge"></div>
								<div class="skeleton-ticker"></div>
								<div class="skeleton-time"></div>
							</div>
							<div class="skeleton-title"></div>
							<div class="skeleton-message"></div>
							<div class="skeleton-message short"></div>
						</div>
					{/each}
				{:else if filteredAlerts.length === 0}
					<!-- Empty State -->
					<div class="alerts-empty-state" role="status" aria-live="polite">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48">
							<path d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z"/>
						</svg>
						<h4>No alerts found</h4>
						<p>{selectedFilter === 'all' ? 'No alerts have been posted yet.' : `No ${selectedFilter} alerts match your filter.`}</p>
						{#if selectedFilter !== 'all'}
							<button class="reset-filter-btn" onclick={() => (selectedFilter = 'all')}>Show all alerts</button>
						{/if}
					</div>
				{:else}
					{#each filteredAlerts as alert, index}
						<AlertCard
							{alert}
							{index}
							{isAdmin}
							isNotesExpanded={expandedNotes.has(alert.id)}
							isCopied={copiedAlertId === alert.id}
							onToggleNotes={toggleNotes}
							onCopy={copyTradeDetails}
							onEdit={(a) => {
								const apiAlert = apiAlerts.find((api) => api.id === a.id);
								if (apiAlert) openEditAlertModal(apiAlert);
							}}
							onDelete={handleDeleteAlert}
						/>
					{/each}
				{/if}
			</div>

			<!-- Pagination UI - Using extracted Pagination component -->
			{#if pagination.total > alertsPerPage && !isLoadingAlerts}
				<Pagination
					{currentPage}
					{totalPages}
					totalItems={pagination.total}
					itemsPerPage={alertsPerPage}
					onPageChange={goToPage}
					itemLabel="alerts"
				/>
			{/if}

			<a href="/dashboard/explosive-swings/alerts" class="view-all-link"> View All Alerts → </a>
		</section>

		<!-- ═══════════════════════════════════════════════════════════════════════════
		     SIDEBAR - Weekly video, performance, resources, updates
		     ═══════════════════════════════════════════════════════════════════════════ -->
		<aside class="sidebar" aria-label="Dashboard sidebar">
			<!-- Weekly Breakdown Video -->
			<div class="sidebar-card" role="region" aria-labelledby="weekly-video-heading">
				<div class="sidebar-card-header">
					<h3 id="weekly-video-heading">Weekly Breakdown</h3>
					<a href="/dashboard/explosive-swings/video-library" class="watch-full-btn">Watch Full Video</a>
				</div>
				<div class="sidebar-video-container">
					<div class="sidebar-video-wrapper">
						<div class="sidebar-video-title">Weekly Swing Setup Video</div>
						<div class="sidebar-video-player">
							<button class="play-button" aria-label="Play video">
								<svg viewBox="0 0 24 24" fill="currentColor" width="32" height="32">
									<path d="M8 5v14l11-7z" />
								</svg>
							</button>
						</div>
						<div class="sidebar-video-controls">
							<span class="video-time">0:00 / 9:58</span>
						</div>
					</div>
				</div>
			</div>

			<!-- 30-Day Performance Chart -->
			<div class="sidebar-card" role="region" aria-labelledby="perf-heading">
				<h3 id="perf-heading">30-Day Performance</h3>
				{#if isLoadingStats}
					<div class="loading-shimmer" style="height: 120px;" aria-label="Loading performance data"></div>
				{:else}
					<div class="perf-chart-interactive">
						<!-- Tooltip -->
						<div class="chart-tooltip">
							<div class="tooltip-date">Mar 11, 2024</div>
							<div class="tooltip-value">● $142.50 — 122.59</div>
						</div>
						<!-- Chart -->
						<svg viewBox="0 0 300 100" class="performance-chart" aria-hidden="true">
							<polyline
								points="0,70 30,65 60,60 90,55 120,50 150,45 180,55 210,40 240,35 270,30 300,25"
								fill="none"
								stroke="#3b82f6"
								stroke-width="2"
							/>
							<circle cx="210" cy="40" r="6" fill="#fff" stroke="#3b82f6" stroke-width="2" />
						</svg>
					</div>
				{/if}
			</div>

			<!-- Resources & Need Help Row -->
			<div class="sidebar-row">
				<!-- Resources Links -->
				<div class="sidebar-card sidebar-card-half" role="region" aria-labelledby="resources-heading">
					<h3 id="resources-heading">Resources</h3>
					<nav class="resource-links">
						<a href="/dashboard/explosive-swings/video-library">Resources</a>
						<a href="/dashboard/explosive-swings/format-links">Tormmat Links</a>
						<a href="https://intercom.help/simpler-trading/en/" target="_blank" rel="noopener noreferrer">Need Help</a>
					</nav>
				</div>

				<!-- Need Help Card -->
				<div class="sidebar-card sidebar-card-half support-card" role="region" aria-labelledby="support-heading">
					<h3 id="support-heading">Need Help?</h3>
					<p>This represents/view our sonotrusie contact or need help help.</p>
				</div>
			</div>

			<!-- Latest Updates Grid -->
			<div class="sidebar-card" role="region" aria-labelledby="updates-heading">
				<h3 id="updates-heading">Latest Updates</h3>
				<div class="sidebar-updates-grid">
					{#each latestUpdates.slice(0, 6) as update}
						<a href={update.href} class="sidebar-update-item">
							<div class="sidebar-update-thumb" style="background-image: url('{update.image}')">
								<div class="sidebar-play-icon">
									<svg viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
										<path d="M8 5v14l11-7z" />
									</svg>
								</div>
								<span class="sidebar-duration">{update.duration}</span>
							</div>
							<div class="sidebar-update-title">{update.title}</div>
						</a>
					{/each}
				</div>
			</div>
		</aside>
	</div>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     VIDEO GRID - Nuclear Build Specification
	     Featured video + responsive grid of recent videos
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<VideoGrid videos={derivedRecentVideos} isLoading={isLoadingStats || isLoadingVideos} />
</div>

<!-- Admin Modal for Creating/Editing Alerts -->
<TradeAlertModal
	bind:isOpen={isAlertModalOpen}
	roomSlug={ROOM_SLUG}
	editAlert={editingAlert}
	entryAlerts={apiAlerts.filter((a) => a.alert_type === 'ENTRY')}
	onClose={closeAlertModal}
	onSave={handleSaveAlert}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE BASE - Renamed from .dashboard to avoid collision with parent layout
	   CSS Containment added to prevent layout thrashing
	   ═══════════════════════════════════════════════════════════════════════════ */
	.explosive-swings-page {
		background: #f5f7fa;
		contain: layout style;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   STATS BAR - Enhanced with Backdrop Blur & Circular Progress Ring
	   ═══════════════════════════════════════════════════════════════════════════ */
	.stats-bar {
		display: flex;
		justify-content: center;
		gap: 60px;
		padding: 24px 30px;
		background: rgba(255, 255, 255, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid rgba(229, 231, 235, 0.8);
		flex-wrap: wrap;
		position: sticky;
		top: 0;
		z-index: 50;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
	}

	.stat {
		text-align: center;
	}

	.stat-with-ring {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.win-rate-ring {
		position: relative;
		width: 70px;
		height: 70px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.circular-chart {
		position: absolute;
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.circle-bg {
		fill: none;
		stroke: #e5e7eb;
		stroke-width: 3;
	}

	.circle {
		fill: none;
		stroke: #22c55e;
		stroke-width: 3;
		stroke-linecap: round;
		transition: stroke-dasharray 0.6s ease;
	}

	.stat-value-ring {
		font-size: 18px;
		font-weight: 700;
		color: #143e59;
		font-family: 'Montserrat', sans-serif;
		z-index: 1;
	}

	.stat-value {
		display: block;
		font-size: 32px;
		font-weight: 700;
		color: #143e59;
		font-family: 'Montserrat', sans-serif;
	}

	.stat-value.green {
		color: #22c55e;
	}

	.stat-label {
		font-size: 12px;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 4px;
	}

	@media (max-width: 640px) {
		.stats-bar {
			gap: 24px;
			padding: 20px;
		}

		.stat-value {
			font-size: 24px;
		}

		.win-rate-ring {
			width: 56px;
			height: 56px;
		}

		.stat-value-ring {
			font-size: 14px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HERO SECTION - Collapsible Accordion
	   ═══════════════════════════════════════════════════════════════════════════ */
	.hero {
		background: linear-gradient(135deg, #f69532 0%, #e8850d 50%, #d4790a 100%);
		padding: 0;
		transition: all 0.3s ease;
	}

	.hero.collapsed {
		background: linear-gradient(135deg, #f69532 0%, #e8850d 100%);
	}

	.hero-collapse-toggle {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 18px 30px;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: background 0.2s;
	}

	.hero-collapse-toggle:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.hero-header-compact {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.hero-header-compact h1 {
		color: #fff;
		font-size: 18px;
		font-weight: 700;
		margin: 0;
		font-family: 'Montserrat', sans-serif;
	}

	.video-icon {
		color: #fff;
		opacity: 0.9;
	}

	.collapse-indicator {
		display: flex;
		align-items: center;
		gap: 8px;
		color: rgba(255, 255, 255, 0.9);
		font-size: 13px;
		font-weight: 600;
	}

	.collapse-chevron {
		transition: transform 0.3s ease;
	}

	.collapse-chevron.rotated {
		transform: rotate(180deg);
	}

	.hero-tabs-bar {
		display: flex;
		gap: 10px;
		padding: 0 30px 20px;
		justify-content: center;
	}

	.hero-tabs {
		display: flex;
		gap: 10px;
	}

	.hero-tab {
		display: flex;
		align-items: center;
		gap: 8px;
		background: rgba(255, 255, 255, 0.15);
		border: 2px solid rgba(255, 255, 255, 0.3);
		color: #fff;
		padding: 12px 24px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.hero-tab:hover {
		background: rgba(255, 255, 255, 0.25);
	}

	.hero-tab.active {
		background: #fff;
		color: #f69532;
		border-color: #fff;
	}

	.hero-content {
		padding: 40px;
	}

	/* VIDEO TAB - Compact Version */
	.video-container-compact {
		display: flex;
		gap: 30px;
		align-items: center;
		max-width: 1000px;
		margin: 0 auto;
	}

	.video-player-compact {
		flex: 0 0 50%;
		position: relative;
		padding-bottom: 28%;
		background-size: cover;
		background-position: center;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 15px 40px rgba(0, 0, 0, 0.25);
	}

	.video-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.3);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.3s;
	}

	.video-player-compact:hover .video-overlay {
		background: rgba(0, 0, 0, 0.4);
	}

	.play-btn {
		width: 60px;
		height: 60px;
		background: #fff;
		border: none;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: transform 0.3s;
		box-shadow: 0 8px 30px rgba(0, 0, 0, 0.3);
	}

	.play-btn svg {
		width: 24px;
		height: 24px;
		color: #f69532;
		margin-left: 3px;
	}

	.video-player-compact:hover .play-btn {
		transform: scale(1.1);
	}

	.video-duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 5px 10px;
		border-radius: 5px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-info-compact {
		flex: 1;
		color: #fff;
		text-align: center;
	}

	.video-info-compact h2 {
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 8px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.video-info-compact p {
		font-size: 13px;
		opacity: 0.9;
		margin: 0 0 20px 0;
	}

	.watch-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: #143e59;
		color: #fff;
		padding: 16px 32px;
		border-radius: 10px;
		font-size: 16px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s;
	}

	.watch-btn:hover {
		background: #0f2d42;
		transform: translateY(-2px);
	}

	/* ENTRIES TAB - Trade Sheet */
	.entries-container {
		max-width: 1400px;
		margin: 0 auto;
	}

	.entries-header {
		text-align: center;
		color: #fff;
		margin-bottom: 30px;
	}

	.entries-header h2 {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 8px 0;
		font-family: 'Montserrat', sans-serif;
	}

	.entries-header p {
		font-size: 15px;
		opacity: 0.9;
		margin: 0;
	}

	.trade-sheet-wrapper {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.2);
	}

	.trade-sheet {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.trade-sheet thead {
		background: #143e59;
		color: #fff;
	}

	.trade-sheet th {
		padding: 16px 12px;
		text-align: center;
		font-weight: 700;
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.trade-sheet tbody tr:nth-child(4n + 1),
	.trade-sheet tbody tr:nth-child(4n + 2) {
		background: #f8fafc;
	}

	.trade-sheet td {
		padding: 14px 12px;
		text-align: center;
		border-bottom: 1px solid #e5e7eb;
	}

	.ticker-cell strong {
		font-size: 16px;
		color: #143e59;
	}

	.bias {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
	}

	.bias--bullish {
		background: #dcfce7;
		color: #166534;
	}

	.bias--bearish {
		background: #fee2e2;
		color: #991b1b;
	}

	.bias--neutral {
		background: #fef3c7;
		color: #92400e;
	}

	.entry-cell {
		font-weight: 700;
		color: #143e59;
	}

	.target-cell {
		color: #22c55e;
		font-weight: 600;
	}

	.runner-cell {
		color: #0ea5e9;
		font-weight: 700;
	}

	.stop-cell {
		color: #ef4444;
		font-weight: 600;
	}

	.options-cell {
		font-weight: 600;
		color: #7c3aed;
	}

	.exp-cell {
		font-size: 12px;
		color: #666;
	}

	/* Notes Column Header */
	.notes-th {
		width: 60px;
		text-align: center;
	}

	/* Notes Toggle Button in Table */
	.notes-toggle-cell {
		text-align: center;
		vertical-align: middle;
	}

	.table-notes-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #f1f5f9;
		border: 1.5px solid #e2e8f0;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.25s ease;
		color: #64748b;
	}

	.table-notes-btn:hover {
		background: #e2e8f0;
		border-color: #143e59;
		color: #143e59;
	}

	.table-notes-btn.expanded {
		background: #143e59;
		border-color: #143e59;
		color: #fff;
	}

	.table-notes-btn .chevron-icon {
		transition: transform 0.3s ease;
	}

	.table-notes-btn.expanded .chevron-icon {
		transform: rotate(180deg);
	}

	/* Row highlight when notes open */
	tr.has-notes-open td {
		background: #f0f9ff !important;
	}

	/* Expandable Notes Row */
	.notes-row.expanded td {
		text-align: left;
		padding: 0;
		background: transparent !important;
		border-bottom: 2px solid #143e59;
	}

	.trade-notes-panel {
		display: flex;
		align-items: flex-start;
		gap: 16px;
		padding: 18px 24px;
		background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
		animation: notesSlide 0.3s ease;
	}

	@keyframes notesSlide {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.trade-notes-badge {
		flex-shrink: 0;
		background: #143e59;
		color: #fff;
		font-size: 12px;
		font-weight: 800;
		padding: 6px 14px;
		border-radius: 6px;
		letter-spacing: 0.05em;
	}

	.trade-notes-panel p {
		flex: 1;
		font-size: 14px;
		color: #0c4a6e;
		line-height: 1.7;
		margin: 0;
		font-weight: 500;
	}

	.sheet-footer {
		padding: 20px;
		text-align: center;
	}

	.google-sheet-link {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		background: #143e59;
		color: #fff;
		padding: 14px 28px;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 700;
		text-decoration: none;
		transition: all 0.3s;
	}

	.google-sheet-link:hover {
		background: #0f2d42;
	}

	@media (max-width: 1024px) {
		.video-container {
			flex-direction: column;
		}

		.video-player {
			flex: none;
			width: 100%;
			padding-bottom: 56.25%;
		}

		.trade-sheet-wrapper {
			overflow-x: auto;
		}

		.trade-sheet {
			min-width: 900px;
		}
	}

	@media (max-width: 768px) {
		.hero-header {
			flex-direction: column;
			text-align: center;
		}

		.hero-tabs {
			width: 100%;
			justify-content: center;
		}

		.hero-tab {
			flex: 1;
			justify-content: center;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MAIN GRID
	   ═══════════════════════════════════════════════════════════════════════════ */
	.main-grid {
		display: grid;
		grid-template-columns: 1fr 340px;
		gap: 30px;
		padding: 30px;
		max-width: 1400px;
		margin: 0 auto;
		contain: layout;
	}

	@media (max-width: 1024px) {
		.main-grid {
			grid-template-columns: 1fr;
			gap: 24px;
			padding: 24px;
		}
	}

	@media (max-width: 768px) {
		.main-grid {
			gap: 20px;
			padding: 20px;
		}
	}

	@media (max-width: 640px) {
		.main-grid {
			gap: 16px;
			padding: 16px;
		}
	}

	/* ALERTS SECTION */
	.alerts-section {
		background: #fff;
		border-radius: 16px;
		padding: 25px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
		contain: layout style;
	}

	@media (max-width: 640px) {
		.alerts-section {
			padding: 18px;
			border-radius: 12px;
		}
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 25px;
		flex-wrap: wrap;
		gap: 15px;
	}

	.section-header h2 {
		font-size: 20px;
		font-weight: 700;
		margin: 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
		text-align: center;
	}

	.filter-pills {
		display: flex;
		gap: 8px;
		flex-wrap: wrap;
	}

	.pill {
		background: #f3f4f6;
		border: none;
		padding: 8px 16px;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pill:hover {
		background: #e5e7eb;
	}

	.pill.active {
		background: #143e59;
		color: #fff;
	}

	@media (max-width: 640px) {
		.section-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 12px;
			margin-bottom: 20px;
		}

		.section-header h2 {
			font-size: 18px;
			text-align: left;
		}

		.filter-pills {
			width: 100%;
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
			padding-bottom: 4px;
		}

		.pill {
			padding: 7px 14px;
			font-size: 12px;
			white-space: nowrap;
			flex-shrink: 0;
		}
	}

	.alerts-list {
		display: flex;
		flex-direction: column;
		gap: 15px;
		contain: layout style;
		max-height: none; /* Pagination handles overflow */
	}

	/* Skeleton Loading for Alerts */
	.alert-card-skeleton {
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 20px;
	}

	.skeleton-row {
		display: flex;
		align-items: center;
		gap: 12px;
		margin-bottom: 16px;
	}

	.skeleton-badge {
		width: 60px;
		height: 24px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton-ticker {
		width: 50px;
		height: 20px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton-time {
		width: 80px;
		height: 16px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	.skeleton-title {
		width: 70%;
		height: 20px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 12px;
	}

	.skeleton-message {
		width: 100%;
		height: 16px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 8px;
	}

	.skeleton-message.short {
		width: 60%;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* Alert Card with Slide-in Animation */
	.alert-card {
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 20px;
		position: relative;
		transition: all 0.2s;
		text-align: left;
		animation: alertSlideIn 0.4s ease-out backwards;
	}

	@keyframes alertSlideIn {
		from {
			opacity: 0;
			transform: translateY(-12px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.alert-card:hover {
		border-color: #143e59;
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.08);
	}

	.alert-card.is-new {
		background: #fffbf5;
		border-color: #f69532;
		box-shadow: 0 4px 15px rgba(246, 149, 50, 0.15);
	}

	.new-badge {
		position: absolute;
		top: -10px;
		right: 15px;
		background: #f69532;
		color: #fff;
		font-size: 10px;
		font-weight: 700;
		padding: 4px 12px;
		border-radius: 10px;
	}

	.new-badge.pulse {
		animation: badgePulse 2s ease-in-out infinite;
	}

	@keyframes badgePulse {
		0%, 100% { box-shadow: 0 0 0 0 rgba(246, 149, 50, 0.4); }
		50% { box-shadow: 0 0 0 8px rgba(246, 149, 50, 0); }
	}

	/* Directional Icons */
	.direction-icon {
		flex-shrink: 0;
	}

	.direction-up {
		color: #22c55e;
	}

	.direction-down {
		color: #3b82f6;
	}

	.direction-neutral {
		color: #f59e0b;
	}

	/* Price Display */
	.alert-price {
		background: linear-gradient(135deg, #143e59 0%, #1e5175 100%);
		padding: 8px 16px;
		border-radius: 8px;
		margin-left: auto;
	}

	.price-value {
		color: #fff;
		font-size: 16px;
		font-weight: 700;
		font-family: 'Montserrat', sans-serif;
	}

	/* Action Buttons Row */
	.alert-actions-row {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 10px;
		margin-top: 12px;
		padding-top: 12px;
		border-top: 1px solid #e5e7eb;
	}

	/* Copy Button */
	.copy-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		background: #f1f5f9;
		border: 1px solid #e2e8f0;
		padding: 6px 12px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.copy-btn:hover {
		background: #e2e8f0;
		color: #143e59;
		border-color: #143e59;
	}

	.copy-btn.copied {
		background: #dcfce7;
		border-color: #22c55e;
		color: #166534;
	}

	/* TOS Copy Button */
	.tos-display {
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: #1a1a2e;
		border-radius: 8px;
		padding: 10px 14px;
		margin-top: 12px;
	}

	.tos-copy-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		padding: 6px;
		border-radius: 4px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tos-copy-btn:hover {
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
	}

	/* Alert Row - Inline Header with Chevron */
	.alert-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 12px;
	}

	.alert-info {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.notes-chevron {
		display: flex;
		align-items: center;
		gap: 6px;
		background: transparent;
		border: none;
		padding: 8px 12px;
		border-radius: 8px;
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.25s ease;
	}

	.notes-chevron:hover {
		background: #f1f5f9;
		color: #143e59;
	}

	.notes-chevron.expanded {
		background: #143e59;
		color: #fff;
	}

	.notes-label {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.chevron-icon {
		transition: transform 0.3s ease;
	}

	.alert-type {
		font-size: 11px;
		font-weight: 700;
		padding: 4px 10px;
		border-radius: 4px;
	}

	.alert-type--entry {
		background: #dcfce7;
		color: #166534;
	}

	.alert-type--exit {
		background: #dbeafe;
		color: #1e40af;
	}

	.alert-type--update {
		background: #fef3c7;
		color: #92400e;
	}

	.alert-ticker {
		font-size: 16px;
		font-weight: 700;
		color: #143e59;
	}

	.alert-time {
		font-size: 12px;
		color: #888;
	}

	/* Card state when notes are open */
	.alert-card.has-notes-open {
		border-color: #143e59;
		box-shadow: 0 8px 30px rgba(20, 62, 89, 0.12);
	}

	/* Notes Panel - Expandable */
	.notes-panel {
		margin-top: 16px;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border-radius: 12px;
		overflow: hidden;
		border: 1px solid #e2e8f0;
		animation: panelSlide 0.35s cubic-bezier(0.4, 0, 0.2, 1);
	}

	@keyframes panelSlide {
		from {
			opacity: 0;
			transform: translateY(-8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.notes-panel-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 18px;
		background: linear-gradient(135deg, #143e59 0%, #1e5175 100%);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.notes-ticker-badge {
		background: #fff;
		color: #143e59;
		font-size: 12px;
		font-weight: 800;
		padding: 5px 12px;
		border-radius: 6px;
		letter-spacing: 0.05em;
	}

	.notes-title {
		color: rgba(255, 255, 255, 0.9);
		font-size: 13px;
		font-weight: 600;
		letter-spacing: 0.02em;
	}

	.notes-panel-body {
		padding: 18px 20px;
	}

	.view-all-link {
		display: block;
		text-align: center;
		margin-top: 20px;
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
	}

	.view-all-link:hover {
		color: #143e59;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGINATION - Principal Engineer ICT 11 Standards
	   ═══════════════════════════════════════════════════════════════════════════ */
	.pagination {
		margin-top: 24px;
		padding: 16px 24px;
		background: #f8fafc;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
	}

	.pagination-controls {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		flex-wrap: wrap;
	}

	.pagination-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 6px;
		min-width: 40px;
		min-height: 40px;
		padding: 8px 12px;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.pagination-btn:hover:not(:disabled) {
		background: #e5e7eb;
		color: #333;
	}

	.pagination-btn:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.pagination-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.pagination-btn.active {
		background: #143e59;
		color: #fff;
		font-weight: 700;
	}

	.pagination-nav {
		padding: 8px 16px;
	}

	.pagination-pages {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.pagination-ellipsis {
		padding: 8px 4px;
		color: #888;
		font-size: 14px;
	}

	.pagination-info {
		text-align: center;
		margin: 12px 0 0 0;
		font-size: 13px;
		color: #666;
	}

	@media (max-width: 640px) {
		.pagination-nav {
			padding: 8px;
		}

		.pagination-btn {
			min-width: 36px;
			min-height: 36px;
			padding: 6px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */
	.alerts-empty-state {
		text-align: center;
		padding: 48px 24px;
		background: #f8fafc;
		border: 2px dashed #e5e7eb;
		border-radius: 12px;
	}

	.alerts-empty-state svg {
		color: #94a3b8;
		margin-bottom: 16px;
	}

	.alerts-empty-state h4 {
		margin: 0 0 8px 0;
		font-size: 18px;
		font-weight: 700;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.alerts-empty-state p {
		margin: 0 0 16px 0;
		font-size: 14px;
		color: #666;
	}

	.reset-filter-btn {
		background: #143e59;
		color: #fff;
		border: none;
		padding: 10px 20px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.reset-filter-btn:hover {
		background: #0f2d42;
		transform: translateY(-1px);
	}

	.reset-filter-btn:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   SIDEBAR - Matching Screenshot Design (ICT 11 Standards)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.sidebar {
		--sidebar-gap: 16px;
		--card-radius: 12px;
		--card-padding: 20px;
		--color-primary: #143e59;
		--color-accent: #3b82f6;
		--shadow-card: 0 2px 8px rgba(0, 0, 0, 0.08);
		--transition-fast: 0.2s ease;

		display: flex;
		flex-direction: column;
		gap: var(--sidebar-gap);
	}

	.sidebar-card {
		background: #fff;
		border-radius: var(--card-radius);
		padding: var(--card-padding);
		box-shadow: var(--shadow-card);
		border: 1px solid #e5e7eb;
		border-top: 3px solid var(--color-primary);
		transition: box-shadow var(--transition-fast), transform var(--transition-fast);
	}

	.sidebar-card:hover {
		box-shadow: 0 4px 16px rgba(20, 62, 89, 0.12);
		transform: translateY(-2px);
	}

	.sidebar-card h3 {
		font-size: 1rem;
		font-weight: 700;
		margin: 0 0 16px 0;
		color: var(--color-primary);
		font-family: 'Montserrat', system-ui, sans-serif;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.sidebar-card h3::before {
		content: '';
		width: 4px;
		height: 16px;
		background: linear-gradient(180deg, var(--color-primary) 0%, var(--color-accent) 100%);
		border-radius: 2px;
	}

	/* Sidebar Card Header with Button */
	.sidebar-card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 12px;
	}

	.sidebar-card-header h3 {
		margin: 0;
	}

	.watch-full-btn {
		font-size: 0.75rem;
		font-weight: 600;
		color: #666;
		text-decoration: none;
		padding: 6px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		transition: all var(--transition-fast);
	}

	.watch-full-btn:hover {
		background: #f3f4f6;
		color: #333;
	}

	/* Weekly Video Container */
	.sidebar-video-container {
		border-radius: 8px;
		overflow: hidden;
		background: #1a1a1a;
	}

	.sidebar-video-wrapper {
		position: relative;
	}

	.sidebar-video-title {
		padding: 12px 16px;
		font-size: 0.875rem;
		font-weight: 600;
		color: #fff;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.sidebar-video-player {
		aspect-ratio: 16 / 9;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #2a2a2a;
	}

	.play-button {
		width: 56px;
		height: 56px;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.9);
		border: none;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--transition-fast);
	}

	.play-button:hover {
		transform: scale(1.1);
	}

	.play-button svg {
		color: #1a1a1a;
		margin-left: 4px;
	}

	.sidebar-video-controls {
		padding: 8px 16px;
		background: #1a1a1a;
	}

	.video-time {
		font-size: 0.75rem;
		color: #999;
		font-family: ui-monospace, monospace;
	}

	/* 30-Day Performance Chart */
	.perf-chart-interactive {
		position: relative;
		padding-top: 50px;
	}

	.chart-tooltip {
		position: absolute;
		top: 0;
		left: 50%;
		transform: translateX(-50%);
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		padding: 8px 12px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		text-align: center;
		z-index: 10;
	}

	.tooltip-date {
		font-size: 0.75rem;
		font-weight: 600;
		color: #333;
	}

	.tooltip-value {
		font-size: 0.75rem;
		color: var(--color-accent);
	}

	.performance-chart {
		width: 100%;
		height: 80px;
	}

	/* Resources & Need Help Row */
	.sidebar-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 12px;
	}

	.sidebar-card-half {
		padding: 16px;
	}

	.sidebar-card-half h3 {
		font-size: 0.875rem;
		margin-bottom: 12px;
	}

	.resource-links {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.resource-links a {
		font-size: 0.8125rem;
		color: var(--color-accent);
		text-decoration: underline;
		transition: color var(--transition-fast);
	}

	.resource-links a:hover {
		color: var(--color-primary);
	}

	/* Support Card - Enhanced visual design */
	.support-card {
		background: linear-gradient(135deg, #143e59 0%, #1e5175 100%);
		border-top-color: #f59e0b;
	}

	.support-card:hover {
		box-shadow: 0 4px 16px rgba(20, 62, 89, 0.25);
	}

	.support-card h3 {
		color: #fff;
	}

	.support-card h3::before {
		background: linear-gradient(180deg, #f59e0b 0%, #f97316 100%);
	}

	.support-card p {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.85);
		margin: 0;
		line-height: 1.4;
	}

	/* Latest Updates Grid */
	.sidebar-updates-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 12px;
	}

	.sidebar-update-item {
		text-decoration: none;
		display: block;
	}

	.sidebar-update-thumb {
		position: relative;
		aspect-ratio: 1;
		border-radius: 8px;
		background-size: cover;
		background-position: center;
		background-color: #1a1a1a;
		margin-bottom: 8px;
	}

	.sidebar-play-icon {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 32px;
		height: 32px;
		background: rgba(255, 255, 255, 0.9);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.sidebar-play-icon svg {
		color: #1a1a1a;
		margin-left: 2px;
	}

	.sidebar-duration {
		position: absolute;
		bottom: 4px;
		right: 4px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		font-size: 0.625rem;
		padding: 2px 4px;
		border-radius: 3px;
		font-family: ui-monospace, monospace;
	}

	.sidebar-update-title {
		font-size: 0.6875rem;
		font-weight: 600;
		color: #333;
		line-height: 1.3;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.loading-shimmer {
		background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ADMIN STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */
	.section-title-row {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.admin-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		background: linear-gradient(135deg, #f69532 0%, #e8860d 100%);
		color: #fff;
		border: none;
		padding: 8px 16px;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		box-shadow: 0 2px 8px rgba(246, 149, 50, 0.3);
	}

	.admin-btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(246, 149, 50, 0.4);
	}

	.admin-actions {
		display: flex;
		gap: 8px;
		margin-top: 12px;
		padding-top: 12px;
		border-top: 1px solid #e5e7eb;
	}

	.admin-action-btn {
		padding: 6px 14px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.admin-action-btn.edit {
		background: #e0f2fe;
		color: #0369a1;
	}

	.admin-action-btn.edit:hover {
		background: #bae6fd;
	}

	.admin-action-btn.delete {
		background: #fee2e2;
		color: #991b1b;
	}

	.admin-action-btn.delete:hover {
		background: #fecaca;
	}

	.tos-display {
		background: #1a1a2e;
		border-radius: 8px;
		padding: 10px 14px;
		margin-top: 12px;
	}
</style>

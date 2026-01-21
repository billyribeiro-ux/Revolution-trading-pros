<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * Explosive Swings - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Member-first dashboard for swing trading alerts and analysis
	 * @version 3.0.0 - January 2026 - ICT Level 11 Standards
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
	     QUICK STATS - Moved to top for immediate status visibility
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<section class="stats-bar">
		<!-- Win Rate with Circular Progress Ring -->
		<div class="stat stat-with-ring">
			<div class="win-rate-ring">
				<svg viewBox="0 0 36 36" class="circular-chart">
					<path class="circle-bg" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
					<path class="circle" stroke-dasharray="{stats.winRate}, 100" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" />
				</svg>
				<span class="stat-value-ring">{stats.winRate}%</span>
			</div>
			<span class="stat-label">Win Rate</span>
		</div>
		<div class="stat">
			<span class="stat-value green">{stats.weeklyProfit}</span>
			<span class="stat-label">This Week</span>
		</div>
		<div class="stat">
			<span class="stat-value">{stats.activeTrades}</span>
			<span class="stat-label">Active Trades</span>
		</div>
		<div class="stat">
			<span class="stat-value">{stats.closedThisWeek}</span>
			<span class="stat-label">Closed This Week</span>
		</div>
	</section>

	<!-- ═══════════════════════════════════════════════════════════════════════════
	     HERO: Collapsible Weekly Video Accordion
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
				<div class="filter-pills">
					<button
						class="pill"
						class:active={selectedFilter === 'all'}
						onclick={() => (selectedFilter = 'all')}>All</button
					>
					<button
						class="pill"
						class:active={selectedFilter === 'entry'}
						onclick={() => (selectedFilter = 'entry')}>Entries</button
					>
					<button
						class="pill"
						class:active={selectedFilter === 'exit'}
						onclick={() => (selectedFilter = 'exit')}>Exits</button
					>
					<button
						class="pill"
						class:active={selectedFilter === 'update'}
						onclick={() => (selectedFilter = 'update')}>Updates</button
					>
				</div>
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
						<div
							class="alert-card"
							class:is-new={alert.isNew}
							class:has-notes-open={expandedNotes.has(alert.id)}
							style="animation-delay: {index * 50}ms"
						>
							{#if alert.isNew}
								<span class="new-badge pulse">NEW</span>
							{/if}

							<!-- Alert Header Row with Directional Icon -->
							<div class="alert-row">
								<div class="alert-info">
									<!-- Directional Arrow Icon -->
									{#if alert.type === 'ENTRY'}
										<svg class="direction-icon direction-up" viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
											<path d="M7 14l5-5 5 5H7z"/>
										</svg>
									{:else if alert.type === 'EXIT'}
										<svg class="direction-icon direction-down" viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
											<path d="M7 10l5 5 5-5H7z"/>
										</svg>
									{:else}
										<svg class="direction-icon direction-neutral" viewBox="0 0 24 24" fill="currentColor" width="20" height="20">
											<circle cx="12" cy="12" r="4"/>
										</svg>
									{/if}
									<span class="alert-type alert-type--{alert.type.toLowerCase()}">{alert.type}</span>
									<span class="alert-ticker">{alert.ticker}</span>
									<span class="alert-time">{alert.time}</span>
								</div>
								<!-- Price Display (Bold, Prominent) -->
								{#if extractPrice(alert.message)}
									<div class="alert-price">
										<span class="price-value">{extractPrice(alert.message)}</span>
									</div>
								{/if}
							</div>

							<h3>{alert.title}</h3>
							<p class="alert-message">{alert.message}</p>

							<!-- Action Buttons Row -->
							<div class="alert-actions-row">
								<button
									class="copy-btn"
									class:copied={copiedAlertId === alert.id}
									onclick={() => copyTradeDetails(alert)}
									aria-label="Copy trade details"
								>
									{#if copiedAlertId === alert.id}
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" width="16" height="16">
											<path d="M20 6L9 17l-5-5"/>
										</svg>
										<span>Copied!</span>
									{:else}
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
											<rect x="9" y="9" width="13" height="13" rx="2"/>
											<path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
										</svg>
										<span>Copy</span>
									{/if}
								</button>
								<button
									class="notes-chevron"
									class:expanded={expandedNotes.has(alert.id)}
									onclick={() => toggleNotes(alert.id)}
									aria-label="Toggle trade notes"
								>
									<span class="notes-label">Notes</span>
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
							</div>

							<!-- TOS String (if available) -->
							{#if alert.tosString}
								<div class="tos-display">
									<code>{alert.tosString}</code>
									<button 
										class="tos-copy-btn"
										onclick={() => navigator.clipboard.writeText(alert.tosString || '')}
										aria-label="Copy TOS string"
									>
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14">
											<rect x="9" y="9" width="13" height="13" rx="2"/>
											<path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
										</svg>
									</button>
								</div>
							{/if}

						<!-- Expandable Notes Panel -->
						{#if expandedNotes.has(alert.id)}
							<div class="notes-panel">
								<div class="notes-panel-header">
									<div class="notes-ticker-badge">{alert.ticker}</div>
									<span class="notes-title">Trade Analysis & Notes</span>
								</div>
								<div class="notes-panel-body">
									<p>{alert.notes}</p>
								</div>
							</div>
						{/if}

						<!-- Admin Actions -->
						{#if isAdmin}
							<div class="admin-actions">
								<button
									class="admin-action-btn edit"
									onclick={() => {
										const apiAlert = apiAlerts.find((a) => a.id === alert.id);
										if (apiAlert) openEditAlertModal(apiAlert);
									}}
								>
									Edit
								</button>
								<button class="admin-action-btn delete" onclick={() => handleDeleteAlert(alert.id)}>
									Delete
								</button>
							</div>
						{/if}
					</div>
				{/each}
				{/if}
			</div>

			<!-- Pagination UI -->
			{#if pagination.total > alertsPerPage && !isLoadingAlerts}
				<nav class="pagination" aria-label="Alerts pagination">
					<div class="pagination-controls">
						<!-- Previous Button -->
						<button
							class="pagination-btn pagination-nav"
							disabled={currentPage === 1}
							onclick={() => goToPage(currentPage - 1)}
							aria-label="Go to previous page"
						>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
								<path d="M15 19l-7-7 7-7"/>
							</svg>
							<span>Previous</span>
						</button>

						<!-- Page Numbers -->
						<div class="pagination-pages">
							{#each visiblePages as page, idx}
								{#if page === 'ellipsis'}
									<span class="pagination-ellipsis" aria-hidden="true">…</span>
								{:else}
									<button
										class="pagination-btn pagination-page"
										class:active={currentPage === page}
										onclick={() => goToPage(page)}
										aria-label="Go to page {page}"
										aria-current={currentPage === page ? 'page' : undefined}
									>
										{page}
									</button>
								{/if}
							{/each}
						</div>

						<!-- Next Button -->
						<button
							class="pagination-btn pagination-nav"
							disabled={currentPage === totalPages}
							onclick={() => goToPage(currentPage + 1)}
							aria-label="Go to next page"
						>
							<span>Next</span>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
								<path d="M9 5l7 7-7 7"/>
							</svg>
						</button>
					</div>

					<!-- Results Count -->
					<p class="pagination-info" aria-live="polite">
						Showing {showingFrom}-{showingTo} of {pagination.total} alerts
					</p>
				</nav>
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
	     LATEST UPDATES - Video Updates for Entries/Exits
	     ═══════════════════════════════════════════════════════════════════════════ -->
	<section class="latest-updates">
		<div class="updates-header">
			<h2>Latest Updates</h2>
			<p>Video breakdowns as we enter and exit trades</p>
		</div>
		<div class="updates-grid">
			{#each latestUpdates as update}
				<a href={update.href} class="update-card">
					<div class="update-thumbnail" style="background-image: url('{update.image}')">
						<div class="play-overlay">
							<svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
								<path d="M8 5v14l11-7z" />
							</svg>
						</div>
						<div class="update-duration">{update.duration}</div>
					</div>
					<div class="update-content">
						<h3>{update.title}</h3>
						<p class="update-date">{update.date}</p>
						<p class="update-excerpt">{update.excerpt}</p>
					</div>
				</a>
			{/each}
		</div>
	</section>
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

	.tos-display code {
		color: #22c55e;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 13px;
		font-weight: 600;
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

	.notes-chevron.expanded .chevron-icon {
		transform: rotate(180deg);
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

	.alert-card h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
	}

	.alert-card p.alert-message {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
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

	.notes-panel-body p {
		font-size: 14px;
		color: #334155;
		line-height: 1.75;
		margin: 0;
		font-weight: 500;
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
		.pagination-nav span {
			display: none;
		}

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
	}

	.sidebar-card h3 {
		font-size: 1rem;
		font-weight: 700;
		margin: 0 0 16px 0;
		color: #1a1a1a;
		font-family: 'Montserrat', system-ui, sans-serif;
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

	/* Support Card */
	.support-card {
		background: #f8f9fa;
	}

	.support-card h3 {
		color: #1a1a1a;
	}

	.support-card p {
		font-size: 0.75rem;
		color: #666;
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
	   LATEST UPDATES SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.latest-updates {
		background: #fff;
		padding: 60px 30px;
		border-top: 1px solid #e5e7eb;
	}

	.updates-header {
		text-align: center;
		max-width: 800px;
		margin: 0 auto 50px;
	}

	.updates-header h2 {
		font-size: 32px;
		font-weight: 700;
		margin: 0 0 12px 0;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	.updates-header p {
		font-size: 16px;
		color: #666;
		margin: 0;
	}

	.updates-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 30px;
		max-width: 1400px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		.updates-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.updates-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.update-card {
		background: #fff;
		border-radius: 16px;
		overflow: hidden;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
		transition: all 0.3s ease;
		text-decoration: none;
		color: inherit;
		display: block;
	}

	.update-card:hover {
		transform: translateY(-8px);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
	}

	.update-thumbnail {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.4);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: background 0.3s;
	}

	.update-card:hover .play-overlay {
		background: rgba(0, 0, 0, 0.6);
	}

	.play-overlay svg {
		color: #fff;
		opacity: 0.9;
		transition: all 0.3s;
	}

	.update-card:hover .play-overlay svg {
		opacity: 1;
		transform: scale(1.1);
	}

	.update-duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		background: rgba(0, 0, 0, 0.85);
		color: #fff;
		padding: 5px 10px;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
	}

	.update-content {
		padding: 20px;
		text-align: center;
	}

	.update-content h3 {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 8px 0;
		color: #333;
		line-height: 1.4;
	}

	.update-date {
		font-size: 12px;
		color: #888;
		margin: 0 0 12px 0;
	}

	.update-excerpt {
		font-size: 14px;
		color: #666;
		line-height: 1.6;
		margin: 0;
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

	.tos-display code {
		color: #22c55e;
		font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
		font-size: 13px;
		font-weight: 600;
	}
</style>

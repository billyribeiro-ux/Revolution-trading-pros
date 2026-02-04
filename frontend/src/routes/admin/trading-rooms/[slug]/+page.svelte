<!--
	Trading Room Content Management - Per-Room Admin
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 13, 2026
	
	Tabbed interface for managing:
	- Trade Plan (watchlist table with ticker, bias, entry, targets, stop, options, notes)
	- Alerts (entry/exit/update with expandable notes)
	- Weekly Video (featured video with auto-archive)
	
	@version 1.0.0
-->
<script lang="ts">
	import { page } from '$app/state';
	import { browser } from '$app/environment';
	import { untrack } from 'svelte';
	import { getRoomById, type Room } from '$lib/config/rooms';
	import {
		tradePlanApi,
		alertsApi,
		weeklyVideoApi,
		roomStatsApi,
		tradesApi,
		type TradePlanEntry,
		type RoomAlert,
		type WeeklyVideo,
		type RoomStats,
		type RoomTrade,
		type Bias,
		type AlertType,
		type TradeStatus
	} from '$lib/api/room-content';
	import {
		roomResourcesApi,
		type RoomResource,
		type ResourceListQuery
	} from '$lib/api/room-resources';

	// Icons
	import IconTable from '@tabler/icons-svelte-runes/icons/table';
	import IconBell from '@tabler/icons-svelte-runes/icons/bell';
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconPin from '@tabler/icons-svelte-runes/icons/pin';
	import IconPinFilled from '@tabler/icons-svelte-runes/icons/pin-filled';
	import IconChartBar from '@tabler/icons-svelte-runes/icons/chart-bar';
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconCurrencyDollar from '@tabler/icons-svelte-runes/icons/currency-dollar';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// ═══════════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════════

	type Tab = 'trade-plan' | 'alerts' | 'weekly-video' | 'trades' | 'video-library';

	// ═══════════════════════════════════════════════════════════════════════════════
	// ROUTE DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	const slug = $derived(page.params.slug ?? '');
	const room = $derived(getRoomById(slug));

	// ═══════════════════════════════════════════════════════════════════════════════
	// UI STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let activeTab = $state<Tab>('trade-plan');
	let successMessage = $state('');
	let errorMessage = $state('');

	// Delete confirmation modal state
	let showDeleteTradePlanModal = $state(false);
	let showDeleteAlertModal = $state(false);
	let showDeleteTradeModal = $state(false);
	let showDeleteVideoModal = $state(false);
	let pendingDeleteTradePlan = $state<TradePlanEntry | null>(null);
	let pendingDeleteAlert = $state<RoomAlert | null>(null);
	let pendingDeleteTrade = $state<RoomTrade | null>(null);
	let pendingDeleteVideoId = $state<number | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════════
	// TRADE PLAN STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let tradePlanEntries = $state<TradePlanEntry[]>([]);
	let isLoadingTradePlan = $state(true);
	let showTradePlanModal = $state(false);
	let editingTradePlan = $state<TradePlanEntry | null>(null);
	let isSavingTradePlan = $state(false);

	let tradePlanForm = $state({
		ticker: '',
		bias: 'BULLISH' as Bias,
		entry: '',
		target1: '',
		target2: '',
		target3: '',
		runner: '',
		stop: '',
		options_strike: '',
		options_exp: '',
		notes: ''
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// ALERTS STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let alerts = $state<RoomAlert[]>([]);
	let isLoadingAlerts = $state(true);
	let showAlertModal = $state(false);
	let editingAlert = $state<RoomAlert | null>(null);
	let isSavingAlert = $state(false);

	let alertForm = $state({
		alert_type: 'ENTRY' as AlertType,
		ticker: '',
		title: '',
		message: '',
		notes: '',
		// TOS Format Fields
		trade_type: '' as 'options' | 'shares' | '',
		action: '' as 'BUY' | 'SELL' | '',
		quantity: '',
		option_type: '' as 'CALL' | 'PUT' | '',
		strike: '',
		expiration: '',
		contract_type: '' as 'Weeklys' | 'Monthly' | 'LEAPS' | '',
		order_type: '' as 'MKT' | 'LMT' | '',
		limit_price: '',
		fill_price: '',
		tos_string: '',
		is_new: true,
		is_published: true
	});

	// Alert type filter for Market Updates
	let alertTypeFilter = $state<'all' | 'ENTRY' | 'EXIT' | 'UPDATE' | 'MARKET_UPDATE'>('all');

	// ═══════════════════════════════════════════════════════════════════════════════
	// WEEKLY VIDEO STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let currentVideo = $state<WeeklyVideo | null>(null);
	let archivedVideos = $state<WeeklyVideo[]>([]);
	let isLoadingVideo = $state(true);
	let showVideoModal = $state(false);
	let isSavingVideo = $state(false);

	let videoForm = $state({
		week_of: new Date().toISOString().split('T')[0],
		week_title: '',
		video_title: '',
		video_url: '',
		video_platform: 'bunny',
		thumbnail_url: '',
		duration: '',
		description: ''
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// ROOM STATS STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let roomStats = $state<RoomStats | null>(null);
	let isLoadingStats = $state(true);

	// ═══════════════════════════════════════════════════════════════════════════════
	// TRADE TRACKER STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let trades = $state<RoomTrade[]>([]);
	let isLoadingTrades = $state(true);
	let tradeFilter = $state<TradeStatus | 'all'>('all');
	let showCloseTradeModal = $state(false);
	let closingTrade = $state<RoomTrade | null>(null);
	let isClosingTrade = $state(false);

	let closeTradeForm = $state({
		exit_price: '',
		exit_date: new Date().toISOString().split('T')[0],
		notes: ''
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// VIDEO LIBRARY STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let videoResources = $state<RoomResource[]>([]);
	let isLoadingVideos = $state(true);
	let videoFilter = $state<string>('all');
	let showVideoResourceModal = $state(false);
	let editingVideoResource = $state<RoomResource | null>(null);
	let isSavingVideoResource = $state(false);

	let videoResourceForm = $state({
		title: '',
		description: '',
		video_url: '',
		video_platform: 'bunny',
		thumbnail_url: '',
		duration: 0,
		category: 'weekly',
		is_published: true,
		is_featured: false
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// DERIVED COMPUTED VALUES
	// ═══════════════════════════════════════════════════════════════════════════════

	/** Sorted alerts with pinned items first, then by published date descending */
	const sortedAlerts = $derived(
		[...alerts].sort((a, b) => {
			if (a.is_pinned && !b.is_pinned) return -1;
			if (!a.is_pinned && b.is_pinned) return 1;
			return new Date(b.published_at).getTime() - new Date(a.published_at).getTime();
		})
	);

	/** Count of trade plan entries for tab badge */
	const tradePlanCount = $derived(tradePlanEntries.length);

	/** Count of alerts for tab badge */
	const alertsCount = $derived(alerts.length);

	/** Whether any data is currently loading */
	const isLoadingAny = $derived(
		isLoadingTradePlan || isLoadingAlerts || isLoadingVideo || isLoadingStats
	);

	/** Whether a save operation is in progress */
	const isSavingAny = $derived(isSavingTradePlan || isSavingAlert || isSavingVideo);

	/** Trade plan form validation */
	const isTradePlanFormValid = $derived(
		tradePlanForm.ticker.trim() !== '' && tradePlanForm.bias !== null
	);

	/** Alert form validation */
	const isAlertFormValid = $derived(
		alertForm.ticker.trim() !== '' &&
			alertForm.title.trim() !== '' &&
			alertForm.message.trim() !== ''
	);

	/** Video form validation */
	const isVideoFormValid = $derived(
		videoForm.video_url.trim() !== '' && videoForm.video_title.trim() !== ''
	);

	/** Whether room has a current video */
	const hasCurrentVideo = $derived(currentVideo !== null);

	/** Whether there are trade plan entries */
	const hasTradePlanEntries = $derived(tradePlanEntries.length > 0);

	/** Whether there are alerts */
	const hasAlerts = $derived(alerts.length > 0);

	/** Filtered alerts based on type filter */
	const filteredAlerts = $derived(
		alertTypeFilter === 'all'
			? sortedAlerts
			: sortedAlerts.filter((a) => a.alert_type === alertTypeFilter)
	);

	/** Whether there are archived videos */
	const hasArchivedVideos = $derived(archivedVideos.length > 0);

	/** Filtered trades based on status filter */
	const filteredTrades = $derived(
		tradeFilter === 'all' ? trades : trades.filter((t) => t.status === tradeFilter)
	);

	/** Trades count for tab badge */
	const tradesCount = $derived(trades.length);

	/** Active trades count */
	const activeTradesCount = $derived(trades.filter((t) => t.status === 'open').length);

	/** Video resources count for tab badge */
	const videosCount = $derived(videoResources.length);

	/** Filtered videos based on category */
	const filteredVideos = $derived(
		videoFilter === 'all'
			? videoResources
			: videoResources.filter((v) => v.category === videoFilter)
	);

	/** Close trade form validation */
	const isCloseTradeFormValid = $derived(
		closeTradeForm.exit_price.trim() !== '' && !isNaN(parseFloat(closeTradeForm.exit_price))
	);

	// ═══════════════════════════════════════════════════════════════════════════════
	// DATA FETCHING
	// ═══════════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (!browser || !slug) return;

		untrack(() => {
			loadTradePlan();
			loadAlerts();
			loadWeeklyVideo();
			loadRoomStats();
			loadTrades();
			loadVideoResources();
		});
	});

	async function loadTradePlan() {
		isLoadingTradePlan = true;
		try {
			const response = await tradePlanApi.list(slug);
			tradePlanEntries = response.data || [];
		} catch (err) {
			console.error('Failed to load trade plan:', err);
			errorMessage = 'Failed to load trade plan entries';
		} finally {
			isLoadingTradePlan = false;
		}
	}

	async function loadAlerts() {
		isLoadingAlerts = true;
		try {
			const response = await alertsApi.list(slug, { per_page: 50 });
			alerts = response.data || [];
		} catch (err) {
			console.error('Failed to load alerts:', err);
			errorMessage = 'Failed to load alerts';
		} finally {
			isLoadingAlerts = false;
		}
	}

	async function loadWeeklyVideo() {
		isLoadingVideo = true;
		try {
			const [currentRes, archiveRes] = await Promise.all([
				weeklyVideoApi.getCurrent(slug),
				weeklyVideoApi.list(slug, { per_page: 10 })
			]);
			currentVideo = currentRes.data;
			archivedVideos = (archiveRes.data || []).filter((v) => !v.is_current);
		} catch (err) {
			console.error('Failed to load weekly video:', err);
		} finally {
			isLoadingVideo = false;
		}
	}

	async function loadRoomStats() {
		isLoadingStats = true;
		try {
			const response = await roomStatsApi.get(slug);
			roomStats = response.data;
		} catch (err) {
			console.error('Failed to load room stats:', err);
		} finally {
			isLoadingStats = false;
		}
	}

	async function loadTrades() {
		isLoadingTrades = true;
		try {
			const response = await tradesApi.list(slug, { per_page: 100 });
			trades = response.data || [];
		} catch (err) {
			console.error('Failed to load trades:', err);
			errorMessage = 'Failed to load trades';
		} finally {
			isLoadingTrades = false;
		}
	}

	async function loadVideoResources() {
		isLoadingVideos = true;
		try {
			const response = await roomResourcesApi.adminList({
				room_slug: slug,
				resource_type: 'video',
				per_page: 100
			} as ResourceListQuery & { room_slug: string });
			videoResources = response.data || [];
		} catch (err) {
			console.error('Failed to load video resources:', err);
		} finally {
			isLoadingVideos = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// TRADE PLAN HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function openAddTradePlan() {
		editingTradePlan = null;
		tradePlanForm = {
			ticker: '',
			bias: 'BULLISH',
			entry: '',
			target1: '',
			target2: '',
			target3: '',
			runner: '',
			stop: '',
			options_strike: '',
			options_exp: '',
			notes: ''
		};
		showTradePlanModal = true;
	}

	function openEditTradePlan(entry: TradePlanEntry) {
		editingTradePlan = entry;
		tradePlanForm = {
			ticker: entry.ticker,
			bias: entry.bias as Bias,
			entry: entry.entry || '',
			target1: entry.target1 || '',
			target2: entry.target2 || '',
			target3: entry.target3 || '',
			runner: entry.runner || '',
			stop: entry.stop || '',
			options_strike: entry.options_strike || '',
			options_exp: entry.options_exp || '',
			notes: entry.notes || ''
		};
		showTradePlanModal = true;
	}

	async function saveTradePlan() {
		if (!isTradePlanFormValid) {
			errorMessage = 'Ticker and Bias are required';
			return;
		}

		isSavingTradePlan = true;
		errorMessage = '';

		try {
			if (editingTradePlan) {
				await tradePlanApi.update(editingTradePlan.id, tradePlanForm);
				successMessage = 'Trade plan entry updated';
			} else {
				await tradePlanApi.create({
					room_slug: slug,
					...tradePlanForm
				});
				successMessage = 'Trade plan entry created';
			}
			showTradePlanModal = false;
			await loadTradePlan();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to save trade plan';
		} finally {
			isSavingTradePlan = false;
		}
	}

	function deleteTradePlan(entry: TradePlanEntry) {
		pendingDeleteTradePlan = entry;
		showDeleteTradePlanModal = true;
	}

	async function confirmDeleteTradePlan() {
		if (!pendingDeleteTradePlan) return;
		showDeleteTradePlanModal = false;
		const entry = pendingDeleteTradePlan;
		pendingDeleteTradePlan = null;
		try {
			await tradePlanApi.delete(entry.id);
			successMessage = 'Trade plan entry deleted';
			await loadTradePlan();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to delete entry';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// ALERTS HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function openAddAlert() {
		editingAlert = null;
		alertForm = {
			alert_type: 'ENTRY',
			ticker: '',
			title: '',
			message: '',
			notes: '',
			trade_type: '',
			action: '',
			quantity: '',
			option_type: '',
			strike: '',
			expiration: '',
			contract_type: '',
			order_type: '',
			limit_price: '',
			fill_price: '',
			tos_string: '',
			is_new: true,
			is_published: true
		};
		showAlertModal = true;
	}

	function openEditAlert(alert: RoomAlert) {
		editingAlert = alert;
		alertForm = {
			alert_type: alert.alert_type as AlertType,
			ticker: alert.ticker,
			title: alert.title || '',
			message: alert.message,
			notes: alert.notes || '',
			trade_type: (alert as any).trade_type || '',
			action: (alert as any).action || '',
			quantity: (alert as any).quantity?.toString() || '',
			option_type: (alert as any).option_type || '',
			strike: (alert as any).strike?.toString() || '',
			expiration: (alert as any).expiration || '',
			contract_type: (alert as any).contract_type || '',
			order_type: (alert as any).order_type || '',
			limit_price: (alert as any).limit_price?.toString() || '',
			fill_price: (alert as any).fill_price?.toString() || '',
			tos_string: (alert as any).tos_string || '',
			is_new: alert.is_new,
			is_published: alert.is_published
		};
		showAlertModal = true;
	}

	async function saveAlert() {
		if (!isAlertFormValid) {
			errorMessage = 'Ticker, Title, and Message are required';
			return;
		}

		isSavingAlert = true;
		errorMessage = '';

		try {
			if (editingAlert) {
				await alertsApi.update(editingAlert.id, alertForm);
				successMessage = 'Alert updated';
			} else {
				await alertsApi.create({
					room_slug: slug,
					...alertForm
				});
				successMessage = 'Alert created';
			}
			showAlertModal = false;
			await loadAlerts();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to save alert';
		} finally {
			isSavingAlert = false;
		}
	}

	function deleteAlert(alert: RoomAlert) {
		pendingDeleteAlert = alert;
		showDeleteAlertModal = true;
	}

	async function confirmDeleteAlert() {
		if (!pendingDeleteAlert) return;
		showDeleteAlertModal = false;
		const alert = pendingDeleteAlert;
		pendingDeleteAlert = null;
		try {
			await alertsApi.delete(alert.id);
			successMessage = 'Alert deleted';
			await loadAlerts();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to delete alert';
		}
	}

	async function toggleAlertPin(alert: RoomAlert) {
		try {
			await alertsApi.update(alert.id, { is_pinned: !alert.is_pinned });
			successMessage = alert.is_pinned ? 'Alert unpinned' : 'Alert pinned';
			await loadAlerts();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to update alert';
		}
	}

	async function toggleAlertNew(alert: RoomAlert) {
		try {
			await alertsApi.update(alert.id, { is_new: !alert.is_new });
			successMessage = alert.is_new ? 'Alert marked as read' : 'Alert marked as new';
			await loadAlerts();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to update alert';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// WEEKLY VIDEO HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function openAddVideo() {
		const today = new Date();
		const weekTitle = `Week of ${today.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' })}`;

		videoForm = {
			week_of: today.toISOString().split('T')[0],
			week_title: weekTitle,
			video_title: 'Weekly Breakdown',
			video_url: '',
			video_platform: 'bunny',
			thumbnail_url: '',
			duration: '',
			description: ''
		};
		showVideoModal = true;
	}

	async function saveVideo() {
		if (!isVideoFormValid) {
			errorMessage = 'Video URL and Title are required';
			return;
		}

		isSavingVideo = true;
		errorMessage = '';

		try {
			await weeklyVideoApi.create({
				room_slug: slug,
				...videoForm
			});
			successMessage = 'Weekly video published (previous video archived)';
			showVideoModal = false;
			await loadWeeklyVideo();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to save video';
		} finally {
			isSavingVideo = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// TRADE TRACKER HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function openCloseTrade(trade: RoomTrade) {
		closingTrade = trade;
		closeTradeForm = {
			exit_price: '',
			exit_date: new Date().toISOString().split('T')[0],
			notes: ''
		};
		showCloseTradeModal = true;
	}

	async function closeTrade() {
		if (!closingTrade || !isCloseTradeFormValid) {
			errorMessage = 'Exit price is required';
			return;
		}

		isClosingTrade = true;
		errorMessage = '';

		try {
			await tradesApi.close(closingTrade.id, {
				exit_price: parseFloat(closeTradeForm.exit_price),
				exit_date: closeTradeForm.exit_date,
				notes: closeTradeForm.notes || undefined
			});
			successMessage = `Trade ${closingTrade.ticker} closed successfully`;
			showCloseTradeModal = false;
			closingTrade = null;
			await loadTrades();
			await loadRoomStats();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to close trade';
		} finally {
			isClosingTrade = false;
		}
	}

	function deleteTrade(trade: RoomTrade) {
		pendingDeleteTrade = trade;
		showDeleteTradeModal = true;
	}

	async function confirmDeleteTrade() {
		if (!pendingDeleteTrade) return;
		showDeleteTradeModal = false;
		const trade = pendingDeleteTrade;
		pendingDeleteTrade = null;
		try {
			await tradesApi.delete(trade.id);
			successMessage = 'Trade deleted';
			await loadTrades();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to delete trade';
		}
	}

	async function confirmDeleteVideo() {
		if (pendingDeleteVideoId === null) return;
		showDeleteVideoModal = false;
		const videoId = pendingDeleteVideoId;
		pendingDeleteVideoId = null;
		try {
			await roomResourcesApi.delete(videoId);
			successMessage = 'Video deleted';
			await loadVideoResources();
		} catch (err: any) {
			errorMessage = err.message || 'Failed to delete video';
		}
	}

	// Trade result color helper
	function getTradeResultColor(result: string | null): string {
		switch (result) {
			case 'WIN':
				return '#22c55e';
			case 'LOSS':
				return '#ef4444';
			default:
				return '#64748b';
		}
	}

	// Format currency
	function formatCurrency(value: number | null): string {
		if (value === null) return '-';
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 2
		}).format(value);
	}

	// Format percent
	function formatPercent(value: number | null): string {
		if (value === null) return '-';
		const sign = value >= 0 ? '+' : '';
		return `${sign}${value.toFixed(2)}%`;
	}

	// Clear messages after delay
	$effect(() => {
		if (successMessage) {
			const timeout = setTimeout(() => (successMessage = ''), 3000);
			return () => clearTimeout(timeout);
		}
		return undefined;
	});

	$effect(() => {
		if (errorMessage) {
			const timeout = setTimeout(() => (errorMessage = ''), 5000);
			return () => clearTimeout(timeout);
		}
		return undefined;
	});

	// Bias color helper
	function getBiasColor(bias: string): string {
		switch (bias) {
			case 'BULLISH':
				return '#22c55e';
			case 'BEARISH':
				return '#ef4444';
			case 'NEUTRAL':
				return '#f59e0b';
			default:
				return '#64748b';
		}
	}

	// Alert type color helper
	function getAlertTypeColor(type: string): string {
		switch (type) {
			case 'ENTRY':
				return '#22c55e';
			case 'EXIT':
				return '#3b82f6';
			case 'UPDATE':
				return '#f59e0b';
			default:
				return '#64748b';
		}
	}
</script>

<svelte:head>
	<title>{room?.name || 'Trading Room'} Content | Admin</title>
</svelte:head>

<div class="admin-page">
	<!-- Header -->
	<header class="page-header">
		<a href="/admin/trading-rooms" class="back-link">
			<IconChevronLeft size={20} />
			<span>All Trading Rooms</span>
		</a>
		<div class="header-row">
			<div class="header-content">
				<div class="room-badge" style="background: {room?.color || '#143E59'}">
					{room?.icon || '📊'}
				</div>
				<div>
					<h1>{room?.name || 'Trading Room'}</h1>
					<p>Manage trade plans, alerts, and weekly videos</p>
				</div>
			</div>
			{#if roomStats && !isLoadingStats}
				<div class="stats-panel">
					<div class="stat-item">
						<span class="stat-value" style="color: #22c55e">{roomStats.win_rate ?? '-'}%</span>
						<span class="stat-label">Win Rate</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{roomStats.active_trades ?? 0}</span>
						<span class="stat-label">Active</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{roomStats.closed_this_week ?? 0}</span>
						<span class="stat-label">This Week</span>
					</div>
					<div class="stat-item">
						<span class="stat-value">{roomStats.total_trades ?? 0}</span>
						<span class="stat-label">Total</span>
					</div>
				</div>
			{/if}
		</div>
	</header>

	<!-- Messages -->
	{#if successMessage}
		<div class="message success">
			<IconCheck size={20} />
			{successMessage}
		</div>
	{/if}
	{#if errorMessage}
		<div class="message error">
			<IconX size={20} />
			{errorMessage}
			<button onclick={() => (errorMessage = '')}>×</button>
		</div>
	{/if}

	<!-- Tabs -->
	<nav class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'trade-plan'}
			onclick={() => (activeTab = 'trade-plan')}
		>
			<IconTable size={20} />
			<span>Trade Plan</span>
			<span class="badge">{tradePlanCount}</span>
		</button>
		<button
			class="tab"
			class:active={activeTab === 'alerts'}
			onclick={() => (activeTab = 'alerts')}
		>
			<IconBell size={20} />
			<span>Alerts</span>
			<span class="badge">{alertsCount}</span>
		</button>
		<button
			class="tab"
			class:active={activeTab === 'weekly-video'}
			onclick={() => (activeTab = 'weekly-video')}
		>
			<IconVideo size={20} />
			<span>Weekly Video</span>
			{#if hasCurrentVideo}
				<span class="badge active">1</span>
			{/if}
		</button>
		<button
			class="tab"
			class:active={activeTab === 'trades'}
			onclick={() => (activeTab = 'trades')}
		>
			<IconChartLine size={20} />
			<span>Trade Tracker</span>
			{#if activeTradesCount > 0}
				<span class="badge active">{activeTradesCount}</span>
			{:else}
				<span class="badge">{tradesCount}</span>
			{/if}
		</button>
		<button
			class="tab"
			class:active={activeTab === 'video-library'}
			onclick={() => (activeTab = 'video-library')}
		>
			<IconPlayerPlay size={20} />
			<span>Video Library</span>
			<span class="badge">{videosCount}</span>
		</button>
	</nav>

	<!-- Tab Content -->
	<div class="tab-content">
		<!-- ═══════════════════════════════════════════════════════════════════════════
		     TRADE PLAN TAB
		     ═══════════════════════════════════════════════════════════════════════════ -->
		{#if activeTab === 'trade-plan'}
			<div class="section-header">
				<h2>Trade Plan Entries</h2>
				<button class="btn-primary" onclick={openAddTradePlan}>
					<IconPlus size={18} />
					Add Entry
				</button>
			</div>

			{#if isLoadingTradePlan}
				<div class="loading">Loading trade plan...</div>
			{:else if !hasTradePlanEntries}
				<div class="empty-state">
					<IconTable size={48} />
					<h3>No Trade Plan Entries</h3>
					<p>Add your first trade plan entry to get started</p>
					<button class="btn-primary" onclick={openAddTradePlan}>
						<IconPlus size={18} />
						Add First Entry
					</button>
				</div>
			{:else}
				<div class="table-wrapper">
					<table class="data-table trade-plan-table">
						<thead>
							<tr>
								<th>Ticker</th>
								<th>Bias</th>
								<th>Entry</th>
								<th>T1</th>
								<th>T2</th>
								<th>T3</th>
								<th>Runner</th>
								<th>Stop</th>
								<th>Options</th>
								<th>Exp</th>
								<th>Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each tradePlanEntries as entry}
								<tr>
									<td class="ticker-cell"><strong>{entry.ticker}</strong></td>
									<td>
										<span class="bias-badge" style="background: {getBiasColor(entry.bias)}"
											>{entry.bias}</span
										>
									</td>
									<td>{entry.entry || '-'}</td>
									<td class="target-cell">{entry.target1 || '-'}</td>
									<td class="target-cell">{entry.target2 || '-'}</td>
									<td class="target-cell">{entry.target3 || '-'}</td>
									<td class="runner-cell">{entry.runner || '-'}</td>
									<td class="stop-cell">{entry.stop || '-'}</td>
									<td>{entry.options_strike || '-'}</td>
									<td>{entry.options_exp || '-'}</td>
									<td class="actions-cell">
										<button class="icon-btn" onclick={() => openEditTradePlan(entry)} title="Edit">
											<IconEdit size={16} />
										</button>
										<button
											class="icon-btn danger"
											onclick={() => deleteTradePlan(entry)}
											title="Delete"
										>
											<IconTrash size={16} />
										</button>
									</td>
								</tr>
								{#if entry.notes}
									<tr class="notes-row">
										<td colspan="11">
											<span class="notes-label">Notes:</span>
											{entry.notes}
										</td>
									</tr>
								{/if}
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════════
		     ALERTS TAB
		     ═══════════════════════════════════════════════════════════════════════════ -->
		{#if activeTab === 'alerts'}
			<div class="section-header">
				<h2>Trading Alerts</h2>
				<div class="header-actions">
					<div class="filter-pills">
						<button
							class="pill"
							class:active={alertTypeFilter === 'all'}
							onclick={() => (alertTypeFilter = 'all')}
						>
							All
						</button>
						<button
							class="pill"
							class:active={alertTypeFilter === 'ENTRY'}
							onclick={() => (alertTypeFilter = 'ENTRY')}
						>
							Entry
						</button>
						<button
							class="pill"
							class:active={alertTypeFilter === 'EXIT'}
							onclick={() => (alertTypeFilter = 'EXIT')}
						>
							Exit
						</button>
						<button
							class="pill"
							class:active={alertTypeFilter === 'UPDATE'}
							onclick={() => (alertTypeFilter = 'UPDATE')}
						>
							Update
						</button>
					</div>
					<button class="btn-primary" onclick={openAddAlert}>
						<IconPlus size={18} />
						New Alert
					</button>
				</div>
			</div>

			{#if isLoadingAlerts}
				<div class="loading">Loading alerts...</div>
			{:else if !hasAlerts}
				<div class="empty-state">
					<IconBell size={48} />
					<h3>No Alerts</h3>
					<p>Create your first trading alert</p>
					<button class="btn-primary" onclick={openAddAlert}>
						<IconPlus size={18} />
						Create Alert
					</button>
				</div>
			{:else if filteredAlerts.length === 0}
				<div class="empty-state">
					<IconBell size={48} />
					<h3>No {alertTypeFilter} Alerts</h3>
					<p>No alerts match the selected filter</p>
				</div>
			{:else}
				<div class="alerts-list">
					{#each filteredAlerts as alert}
						<div class="alert-card" class:is-new={alert.is_new} class:is-pinned={alert.is_pinned}>
							<div class="alert-header">
								<div class="alert-meta">
									<span class="alert-type" style="background: {getAlertTypeColor(alert.alert_type)}"
										>{alert.alert_type}</span
									>
									<span class="alert-ticker">{alert.ticker}</span>
									{#if alert.is_pinned}
										<span class="pinned-badge">PINNED</span>
									{/if}
									{#if alert.is_new}
										<span class="new-badge">NEW</span>
									{/if}
								</div>
								<div class="alert-actions">
									<button
										class="icon-btn"
										class:active={alert.is_pinned}
										onclick={() => toggleAlertPin(alert)}
										title={alert.is_pinned ? 'Unpin' : 'Pin'}
									>
										{#if alert.is_pinned}
											<IconPinFilled size={16} />
										{:else}
											<IconPin size={16} />
										{/if}
									</button>
									<button
										class="icon-btn"
										class:active={alert.is_new}
										onclick={() => toggleAlertNew(alert)}
										title={alert.is_new ? 'Mark as read' : 'Mark as new'}
									>
										<IconCheck size={16} />
									</button>
									<button class="icon-btn" onclick={() => openEditAlert(alert)} title="Edit">
										<IconEdit size={16} />
									</button>
									<button class="icon-btn danger" onclick={() => deleteAlert(alert)} title="Delete">
										<IconTrash size={16} />
									</button>
								</div>
							</div>
							<h3>{alert.title}</h3>
							<p class="alert-message">{alert.message}</p>
							{#if alert.notes}
								<div class="alert-notes">
									<span class="notes-label">Notes:</span>
									<p>{alert.notes}</p>
								</div>
							{/if}
							<div class="alert-footer">
								<span class="alert-time">{new Date(alert.published_at).toLocaleString()}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════════
		     WEEKLY VIDEO TAB
		     ═══════════════════════════════════════════════════════════════════════════ -->
		{#if activeTab === 'weekly-video'}
			<div class="section-header">
				<h2>Weekly Video</h2>
				<button class="btn-primary" onclick={openAddVideo}>
					<IconPlus size={18} />
					Publish New Video
				</button>
			</div>

			{#if isLoadingVideo}
				<div class="loading">Loading video...</div>
			{:else}
				<!-- Current Video -->
				{#if currentVideo}
					<div class="current-video-card">
						<div class="video-badge">CURRENT</div>
						<div class="video-content">
							{#if currentVideo.thumbnail_url}
								<div
									class="video-thumbnail"
									style="background-image: url('{currentVideo.thumbnail_url}')"
								>
									{#if currentVideo.duration}
										<span class="duration">{currentVideo.duration}</span>
									{/if}
								</div>
							{/if}
							<div class="video-info">
								<h3>{currentVideo.week_title}</h3>
								<h4>{currentVideo.video_title}</h4>
								<p class="video-url">{currentVideo.video_url}</p>
								<span class="video-date"
									>Published: {new Date(currentVideo.published_at).toLocaleDateString()}</span
								>
							</div>
						</div>
					</div>
				{:else}
					<div class="empty-state">
						<IconVideo size={48} />
						<h3>No Weekly Video</h3>
						<p>Publish your first weekly video for this room</p>
						<button class="btn-primary" onclick={openAddVideo}>
							<IconPlus size={18} />
							Publish Video
						</button>
					</div>
				{/if}

				<!-- Archived Videos -->
				{#if hasArchivedVideos}
					<div class="archived-section">
						<h3>Archive</h3>
						<div class="archived-list">
							{#each archivedVideos as video}
								<div class="archived-video">
									<span class="archived-week">{video.week_title}</span>
									<span class="archived-title">{video.video_title}</span>
									<span class="archived-date"
										>{new Date(video.published_at).toLocaleDateString()}</span
									>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			{/if}
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════════
		     TRADE TRACKER TAB
		     ═══════════════════════════════════════════════════════════════════════════ -->
		{#if activeTab === 'trades'}
			<div class="section-header">
				<h2>Trade Tracker</h2>
				<div class="filter-pills">
					<button
						class="pill"
						class:active={tradeFilter === 'all'}
						onclick={() => (tradeFilter = 'all')}
					>
						All ({tradesCount})
					</button>
					<button
						class="pill"
						class:active={tradeFilter === 'open'}
						onclick={() => (tradeFilter = 'open')}
					>
						Active ({activeTradesCount})
					</button>
					<button
						class="pill"
						class:active={tradeFilter === 'closed'}
						onclick={() => (tradeFilter = 'closed')}
					>
						Closed ({tradesCount - activeTradesCount})
					</button>
				</div>
			</div>

			{#if isLoadingTrades}
				<div class="loading">Loading trades...</div>
			{:else if filteredTrades.length === 0}
				<div class="empty-state">
					<IconChartLine size={48} />
					<h3>No Trades Found</h3>
					<p>Trades are created automatically from entry alerts</p>
				</div>
			{:else}
				<div class="table-wrapper">
					<table class="data-table trades-table">
						<thead>
							<tr>
								<th>Ticker</th>
								<th>Type</th>
								<th>Direction</th>
								<th>Entry</th>
								<th>Entry Date</th>
								<th>Exit</th>
								<th>P&L</th>
								<th>Status</th>
								<th>Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each filteredTrades as trade}
								<tr class:is-open={trade.status === 'open'}>
									<td class="ticker-cell"><strong>{trade.ticker}</strong></td>
									<td>
										<span class="type-badge">{trade.trade_type}</span>
									</td>
									<td>
										<span
											class="direction-badge"
											class:long={trade.direction === 'long'}
											class:short={trade.direction === 'short'}
										>
											{trade.direction.toUpperCase()}
										</span>
									</td>
									<td class="entry-cell">{formatCurrency(trade.entry_price)}</td>
									<td>{new Date(trade.entry_date).toLocaleDateString()}</td>
									<td>
										{#if trade.exit_price}
											{formatCurrency(trade.exit_price)}
										{:else}
											<span class="pending">-</span>
										{/if}
									</td>
									<td>
										{#if trade.pnl !== null}
											<span class="pnl-value" style="color: {getTradeResultColor(trade.result)}">
												{formatCurrency(trade.pnl)}
												<small>({formatPercent(trade.pnl_percent)})</small>
											</span>
										{:else}
											<span class="pending">-</span>
										{/if}
									</td>
									<td>
										<span
											class="status-badge"
											class:open={trade.status === 'open'}
											class:closed={trade.status === 'closed'}
											class:win={trade.result === 'WIN'}
											class:loss={trade.result === 'LOSS'}
										>
											{#if trade.status === 'closed'}
												{trade.result || 'CLOSED'}
											{:else}
												OPEN
											{/if}
										</span>
									</td>
									<td class="actions-cell">
										{#if trade.status === 'open'}
											<button
												class="btn-close-trade"
												onclick={() => openCloseTrade(trade)}
												title="Close Trade"
											>
												<IconCurrencyDollar size={16} />
												Close
											</button>
										{/if}
										<button
											class="icon-btn danger"
											onclick={() => deleteTrade(trade)}
											title="Delete"
										>
											<IconTrash size={16} />
										</button>
									</td>
								</tr>
								{#if trade.notes}
									<tr class="notes-row">
										<td colspan="9">
											<span class="notes-label">Notes:</span>
											{trade.notes}
										</td>
									</tr>
								{/if}
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		{/if}

		<!-- ═══════════════════════════════════════════════════════════════════════════
		     VIDEO LIBRARY TAB
		     ═══════════════════════════════════════════════════════════════════════════ -->
		{#if activeTab === 'video-library'}
			<div class="section-header">
				<h2>Video Library</h2>
				<div class="filter-pills">
					<button
						class="pill"
						class:active={videoFilter === 'all'}
						onclick={() => (videoFilter = 'all')}
					>
						All
					</button>
					<button
						class="pill"
						class:active={videoFilter === 'weekly'}
						onclick={() => (videoFilter = 'weekly')}
					>
						Weekly
					</button>
					<button
						class="pill"
						class:active={videoFilter === 'entry'}
						onclick={() => (videoFilter = 'entry')}
					>
						Entry
					</button>
					<button
						class="pill"
						class:active={videoFilter === 'exit'}
						onclick={() => (videoFilter = 'exit')}
					>
						Exit
					</button>
					<button
						class="pill"
						class:active={videoFilter === 'education'}
						onclick={() => (videoFilter = 'education')}
					>
						Education
					</button>
				</div>
			</div>

			{#if isLoadingVideos}
				<div class="loading">Loading videos...</div>
			{:else if filteredVideos.length === 0}
				<div class="empty-state">
					<IconPlayerPlay size={48} />
					<h3>No Videos Found</h3>
					<p>Videos will appear here when uploaded through the Resources page</p>
					<a href="/admin/resources" class="btn-primary"> Go to Resources </a>
				</div>
			{:else}
				<div class="video-grid">
					{#each filteredVideos as video}
						<div class="video-card">
							<div
								class="video-card-thumbnail"
								style="background-image: url('{video.thumbnail_url || '/placeholder-video.jpg'}')"
							>
								{#if video.formatted_duration}
									<span class="duration">{video.formatted_duration}</span>
								{/if}
								{#if video.is_featured}
									<span class="featured-badge">Featured</span>
								{/if}
							</div>
							<div class="video-card-content">
								<h4>{video.title}</h4>
								<p class="video-card-date">{video.formatted_date}</p>
								{#if video.category}
									<span class="category-badge">{video.category}</span>
								{/if}
								<div class="video-card-stats">
									<span>{video.views_count} views</span>
								</div>
							</div>
							<div class="video-card-actions">
								<a href="/admin/resources/{video.id}" class="icon-btn" title="Edit">
									<IconEdit size={16} />
								</a>
								<button
									class="icon-btn danger"
									onclick={() => {
										pendingDeleteVideoId = video.id;
										showDeleteVideoModal = true;
									}}
									title="Delete"
								>
									<IconTrash size={16} />
								</button>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     TRADE PLAN MODAL
     ═══════════════════════════════════════════════════════════════════════════════════ -->
{#if showTradePlanModal}
	<div
		class="modal-overlay"
		onclick={() => (showTradePlanModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showTradePlanModal = false)}
		role="button"
		tabindex="0"
		aria-label="Close modal"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="trade-plan-modal-title"
		>
			<div class="modal-header">
				<h2 id="trade-plan-modal-title">{editingTradePlan ? 'Edit' : 'Add'} Trade Plan Entry</h2>
				<button class="close-btn" onclick={() => (showTradePlanModal = false)}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					saveTradePlan();
				}}
			>
				<div class="form-row">
					<div class="form-group">
						<label for="ticker">Ticker *</label>
						<input
							id="ticker" name="ticker"
							type="text"
							bind:value={tradePlanForm.ticker}
							placeholder="NVDA"
							style="text-transform: uppercase"
						/>
					</div>
					<div class="form-group">
						<label for="bias">Bias *</label>
						<select id="bias" bind:value={tradePlanForm.bias}>
							<option value="BULLISH">BULLISH</option>
							<option value="BEARISH">BEARISH</option>
							<option value="NEUTRAL">NEUTRAL</option>
						</select>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="entry">Entry</label>
						<input id="entry" name="entry" type="text" bind:value={tradePlanForm.entry} placeholder="$142.50" />
					</div>
					<div class="form-group">
						<label for="stop">Stop</label>
						<input id="stop" name="stop" type="text" bind:value={tradePlanForm.stop} placeholder="$136.00" />
					</div>
				</div>

				<div class="form-row targets">
					<div class="form-group">
						<label for="target1">Target 1</label>
						<input
							id="target1" name="target1"
							type="text"
							bind:value={tradePlanForm.target1}
							placeholder="$148.00"
						/>
					</div>
					<div class="form-group">
						<label for="target2">Target 2</label>
						<input
							id="target2" name="target2"
							type="text"
							bind:value={tradePlanForm.target2}
							placeholder="$152.00"
						/>
					</div>
					<div class="form-group">
						<label for="target3">Target 3</label>
						<input
							id="target3" name="target3"
							type="text"
							bind:value={tradePlanForm.target3}
							placeholder="$158.00"
						/>
					</div>
					<div class="form-group">
						<label for="runner">Runner</label>
						<input
							id="runner" name="runner"
							type="text"
							bind:value={tradePlanForm.runner}
							placeholder="$165.00+"
						/>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="options_strike">Options Strike</label>
						<input
							id="options_strike" name="options_strike"
							type="text"
							bind:value={tradePlanForm.options_strike}
							placeholder="$145 Call"
						/>
					</div>
					<div class="form-group">
						<label for="options_exp">Options Exp</label>
						<input id="options_exp" name="options_exp" type="date" bind:value={tradePlanForm.options_exp} />
					</div>
				</div>

				<div class="form-group full-width">
					<label for="notes">Notes</label>
					<textarea
						id="notes"
						bind:value={tradePlanForm.notes}
						placeholder="Breakout above consolidation. Wait for pullback to entry..."
						rows="3"
					></textarea>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={() => (showTradePlanModal = false)}
						>Cancel</button
					>
					<button
						type="submit"
						class="btn-primary"
						disabled={isSavingTradePlan || !isTradePlanFormValid}
					>
						{isSavingTradePlan ? 'Saving...' : editingTradePlan ? 'Update' : 'Add Entry'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     ALERT MODAL
     ═══════════════════════════════════════════════════════════════════════════════════ -->
{#if showAlertModal}
	<div
		class="modal-overlay"
		onclick={() => (showAlertModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showAlertModal = false)}
		role="button"
		tabindex="0"
		aria-label="Close modal"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="alert-modal-title"
		>
			<div class="modal-header">
				<h2 id="alert-modal-title">{editingAlert ? 'Edit' : 'Create'} Alert</h2>
				<button class="close-btn" onclick={() => (showAlertModal = false)}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					saveAlert();
				}}
			>
				<div class="form-row">
					<div class="form-group">
						<label for="alert_type">Type *</label>
						<select id="alert_type" bind:value={alertForm.alert_type}>
							<option value="ENTRY">ENTRY</option>
							<option value="EXIT">EXIT</option>
							<option value="UPDATE">UPDATE</option>
						</select>
					</div>
					<div class="form-group">
						<label for="alert_ticker">Ticker *</label>
						<input
							id="alert_ticker" name="alert_ticker"
							type="text"
							bind:value={alertForm.ticker}
							placeholder="NVDA"
							style="text-transform: uppercase"
						/>
					</div>
				</div>

				<div class="form-group full-width">
					<label for="alert_title">Title *</label>
					<input
						id="alert_title" name="alert_title"
						type="text"
						bind:value={alertForm.title}
						placeholder="Opening NVDA Swing Position"
					/>
				</div>

				<div class="form-group full-width">
					<label for="alert_message">Message *</label>
					<textarea
						id="alert_message"
						bind:value={alertForm.message}
						placeholder="Entering NVDA at $142.50. First target $148, stop at $136..."
						rows="3"
					></textarea>
				</div>

				<div class="form-group full-width">
					<label for="alert_notes">Detailed Notes (for dropdown)</label>
					<textarea
						id="alert_notes"
						bind:value={alertForm.notes}
						placeholder="Entry based on breakout above $142 resistance with strong volume confirmation..."
						rows="4"
					></textarea>
				</div>

				<!-- TOS Format Section -->
				{#if alertForm.alert_type === 'ENTRY' || alertForm.alert_type === 'EXIT'}
					<div class="tos-section">
						<h4 class="section-title">TOS Format (Optional)</h4>
						<div class="form-row">
							<div class="form-group">
								<label for="trade_type">Trade Type</label>
								<select id="trade_type" bind:value={alertForm.trade_type}>
									<option value="">Select...</option>
									<option value="options">Options</option>
									<option value="shares">Shares</option>
								</select>
							</div>
							<div class="form-group">
								<label for="action">Action</label>
								<select id="action" bind:value={alertForm.action}>
									<option value="">Select...</option>
									<option value="BUY">BUY</option>
									<option value="SELL">SELL</option>
								</select>
							</div>
							<div class="form-group">
								<label for="quantity">Quantity</label>
								<input
									id="quantity" name="quantity"
									type="number"
									bind:value={alertForm.quantity}
									placeholder="10"
								/>
							</div>
						</div>

						{#if alertForm.trade_type === 'options'}
							<div class="form-row">
								<div class="form-group">
									<label for="option_type">Option Type</label>
									<select id="option_type" bind:value={alertForm.option_type}>
										<option value="">Select...</option>
										<option value="CALL">CALL</option>
										<option value="PUT">PUT</option>
									</select>
								</div>
								<div class="form-group">
									<label for="strike">Strike</label>
									<input
										id="strike" name="strike"
										type="number"
										step="0.5"
										bind:value={alertForm.strike}
										placeholder="145"
									/>
								</div>
								<div class="form-group">
									<label for="expiration">Expiration</label>
									<input id="expiration" name="expiration" type="date" bind:value={alertForm.expiration} />
								</div>
							</div>
							<div class="form-row">
								<div class="form-group">
									<label for="contract_type">Contract Type</label>
									<select id="contract_type" bind:value={alertForm.contract_type}>
										<option value="">Select...</option>
										<option value="Weeklys">Weeklys</option>
										<option value="Monthly">Monthly</option>
										<option value="LEAPS">LEAPS</option>
									</select>
								</div>
								<div class="form-group">
									<label for="order_type">Order Type</label>
									<select id="order_type" bind:value={alertForm.order_type}>
										<option value="">Select...</option>
										<option value="MKT">Market (MKT)</option>
										<option value="LMT">Limit (LMT)</option>
									</select>
								</div>
							</div>
						{/if}

						<div class="form-row">
							<div class="form-group">
								<label for="limit_price">Limit Price</label>
								<input
									id="limit_price" name="limit_price"
									type="number"
									step="0.01"
									bind:value={alertForm.limit_price}
									placeholder="2.50"
								/>
							</div>
							<div class="form-group">
								<label for="fill_price">Fill Price</label>
								<input
									id="fill_price" name="fill_price"
									type="number"
									step="0.01"
									bind:value={alertForm.fill_price}
									placeholder="2.48"
								/>
							</div>
						</div>

						<div class="form-group full-width">
							<label for="tos_string">TOS String (auto-generated or manual)</label>
							<input
								id="tos_string" name="tos_string"
								type="text"
								bind:value={alertForm.tos_string}
								placeholder="BUY +10 NVDA 100 (Weeklys) 17 JAN 25 145 CALL @2.50 LMT"
							/>
						</div>
					</div>
				{/if}

				<div class="form-row checkboxes">
					<label class="checkbox-label">
						<input
							id="alert-is-new"
							name="alert-is-new"
							type="checkbox"
							bind:checked={alertForm.is_new}
						/>
						<span>Mark as NEW</span>
					</label>
					<label class="checkbox-label">
						<input
							id="alert-is-published"
							name="alert-is-published"
							type="checkbox"
							bind:checked={alertForm.is_published}
						/>
						<span>Published</span>
					</label>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={() => (showAlertModal = false)}
						>Cancel</button
					>
					<button type="submit" class="btn-primary" disabled={isSavingAlert || !isAlertFormValid}>
						{isSavingAlert ? 'Saving...' : editingAlert ? 'Update' : 'Create Alert'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     WEEKLY VIDEO MODAL
     ═══════════════════════════════════════════════════════════════════════════════════ -->
{#if showVideoModal}
	<div
		class="modal-overlay"
		onclick={() => (showVideoModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showVideoModal = false)}
		role="button"
		tabindex="0"
		aria-label="Close modal"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="video-modal-title"
		>
			<div class="modal-header">
				<h2 id="video-modal-title">Publish Weekly Video</h2>
				<button class="close-btn" onclick={() => (showVideoModal = false)}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					saveVideo();
				}}
			>
				<div class="form-row">
					<div class="form-group">
						<label for="week_of">Week Of *</label>
						<input id="week_of" name="week_of" type="date" bind:value={videoForm.week_of} />
					</div>
					<div class="form-group">
						<label for="week_title">Week Title *</label>
						<input
							id="week_title" name="week_title"
							type="text"
							bind:value={videoForm.week_title}
							placeholder="Week of January 13, 2026"
						/>
					</div>
				</div>

				<div class="form-group full-width">
					<label for="video_title">Video Title *</label>
					<input
						id="video_title" name="video_title"
						type="text"
						bind:value={videoForm.video_title}
						placeholder="Weekly Breakdown: Top Swing Setups"
					/>
				</div>

				<div class="form-row">
					<div class="form-group flex-2">
						<label for="video_url">Video URL *</label>
						<input
							id="video_url" name="video_url"
							type="url"
							bind:value={videoForm.video_url}
							placeholder="https://player.vimeo.com/video/..."
						/>
					</div>
					<div class="form-group">
						<label for="duration">Duration</label>
						<input id="duration" name="duration" type="text" bind:value={videoForm.duration} placeholder="24:35" />
					</div>
				</div>

				<div class="form-group full-width">
					<label for="thumbnail_url">Thumbnail URL</label>
					<input
						id="thumbnail_url" name="thumbnail_url"
						type="url"
						bind:value={videoForm.thumbnail_url}
						placeholder="https://..."
					/>
				</div>

				<div class="form-group full-width">
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={videoForm.description}
						placeholder="This week's breakdown covers..."
						rows="3"
					></textarea>
				</div>

				<div class="info-box">
					<strong>Note:</strong> Publishing a new video will automatically archive the current video.
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={() => (showVideoModal = false)}
						>Cancel</button
					>
					<button type="submit" class="btn-primary" disabled={isSavingVideo || !isVideoFormValid}>
						{isSavingVideo ? 'Publishing...' : 'Publish Video'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     CLOSE TRADE MODAL
     ═══════════════════════════════════════════════════════════════════════════════════ -->
{#if showCloseTradeModal && closingTrade}
	<div
		class="modal-overlay"
		onclick={() => (showCloseTradeModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showCloseTradeModal = false)}
		role="button"
		tabindex="0"
		aria-label="Close modal"
	>
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			aria-labelledby="close-trade-modal-title"
		>
			<div class="modal-header">
				<h2 id="close-trade-modal-title">Close Trade: {closingTrade.ticker}</h2>
				<button class="close-btn" onclick={() => (showCloseTradeModal = false)}>
					<IconX size={24} />
				</button>
			</div>
			<form
				class="modal-body"
				onsubmit={(e) => {
					e.preventDefault();
					closeTrade();
				}}
			>
				<div class="trade-summary">
					<div class="trade-summary-row">
						<span class="label">Ticker:</span>
						<span class="value"><strong>{closingTrade.ticker}</strong></span>
					</div>
					<div class="trade-summary-row">
						<span class="label">Direction:</span>
						<span
							class="value direction-badge"
							class:long={closingTrade.direction === 'long'}
							class:short={closingTrade.direction === 'short'}
						>
							{closingTrade.direction.toUpperCase()}
						</span>
					</div>
					<div class="trade-summary-row">
						<span class="label">Entry Price:</span>
						<span class="value">{formatCurrency(closingTrade.entry_price)}</span>
					</div>
					<div class="trade-summary-row">
						<span class="label">Entry Date:</span>
						<span class="value">{new Date(closingTrade.entry_date).toLocaleDateString()}</span>
					</div>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="exit_price">Exit Price *</label>
						<input
							id="exit_price" name="exit_price"
							type="number"
							step="0.01"
							bind:value={closeTradeForm.exit_price}
							placeholder="145.50"
							required
						/>
					</div>
					<div class="form-group">
						<label for="exit_date">Exit Date</label>
						<input id="exit_date" name="exit_date" type="date" bind:value={closeTradeForm.exit_date} />
					</div>
				</div>

				<div class="form-group full-width">
					<label for="close_notes">Notes</label>
					<textarea
						id="close_notes"
						bind:value={closeTradeForm.notes}
						placeholder="Optional notes about the exit..."
						rows="2"
					></textarea>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-secondary" onclick={() => (showCloseTradeModal = false)}
						>Cancel</button
					>
					<button
						type="submit"
						class="btn-primary"
						disabled={isClosingTrade || !isCloseTradeFormValid}
					>
						{isClosingTrade ? 'Closing...' : 'Close Trade'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<style>
	/* Base Styles */
	.admin-page {
		padding: 32px;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		margin-bottom: 32px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #64748b;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		margin-bottom: 16px;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #143e59;
	}

	.header-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 24px;
		flex-wrap: wrap;
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 20px;
	}

	.room-badge {
		width: 64px;
		height: 64px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 32px;
	}

	.header-content h1 {
		font-size: 28px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 4px 0;
	}

	.header-content p {
		font-size: 15px;
		color: #64748b;
		margin: 0;
	}

	/* Stats Panel */
	.stats-panel {
		display: flex;
		gap: 24px;
		background: #f8fafc;
		padding: 16px 24px;
		border-radius: 12px;
		border: 1px solid #e2e8f0;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-width: 60px;
	}

	.stat-value {
		font-size: 24px;
		font-weight: 700;
		color: #1e293b;
		line-height: 1;
	}

	.stat-label {
		font-size: 11px;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 4px;
	}

	/* Messages */
	.message {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 14px 18px;
		border-radius: 10px;
		margin-bottom: 20px;
		font-weight: 500;
	}

	.message.success {
		background: #dcfce7;
		color: #166534;
	}

	.message.error {
		background: #fef2f2;
		color: #991b1b;
	}

	.message button {
		margin-left: auto;
		background: none;
		border: none;
		font-size: 20px;
		cursor: pointer;
		opacity: 0.7;
	}

	/* Tabs */
	.tabs {
		display: flex;
		gap: 8px;
		border-bottom: 2px solid #e2e8f0;
		margin-bottom: 32px;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 14px 24px;
		background: none;
		border: none;
		font-size: 15px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		border-bottom: 3px solid transparent;
		margin-bottom: -2px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #1e293b;
	}

	.tab.active {
		color: #143e59;
		border-bottom-color: #143e59;
	}

	.tab .badge {
		background: #e2e8f0;
		color: #64748b;
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 12px;
	}

	.tab.active .badge {
		background: #143e59;
		color: #fff;
	}

	.tab .badge.active {
		background: #22c55e;
		color: #fff;
	}

	/* Section Header */
	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 24px;
	}

	.section-header h2 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	/* Buttons */
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #143e59;
		color: #fff;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		background: #0f2d42;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: #f1f5f9;
		color: #475569;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #e2e8f0;
	}

	.icon-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: #f1f5f9;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.icon-btn:hover {
		background: #e2e8f0;
		color: #143e59;
	}

	.icon-btn.danger:hover {
		background: #fef2f2;
		color: #dc2626;
	}

	.icon-btn.active {
		background: #dbeafe;
		color: #2563eb;
	}

	/* Loading & Empty States */
	.loading {
		text-align: center;
		padding: 60px 20px;
		color: #64748b;
		font-size: 16px;
	}

	.empty-state {
		text-align: center;
		padding: 80px 20px;
		background: #f8fafc;
		border-radius: 16px;
		color: #64748b;
	}

	.empty-state :global(svg) {
		opacity: 0.4;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		font-size: 20px;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 8px 0;
	}

	.empty-state p {
		margin: 0 0 24px 0;
	}

	/* Trade Plan Table */
	.table-wrapper {
		overflow-x: auto;
		background: #fff;
		border-radius: 12px;
		border: 1px solid #e2e8f0;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.data-table th {
		text-align: left;
		padding: 14px 16px;
		background: #143e59;
		color: #fff;
		font-weight: 600;
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.data-table th:first-child {
		border-radius: 12px 0 0 0;
	}

	.data-table th:last-child {
		border-radius: 0 12px 0 0;
	}

	.data-table td {
		padding: 14px 16px;
		border-bottom: 1px solid #e2e8f0;
		vertical-align: middle;
	}

	.data-table tbody tr:hover {
		background: #f8fafc;
	}

	.ticker-cell {
		font-weight: 700;
		color: #1e293b;
	}

	.target-cell {
		color: #22c55e;
	}

	.runner-cell {
		color: #3b82f6;
		font-weight: 600;
	}

	.stop-cell {
		color: #ef4444;
	}

	.bias-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 6px;
		color: #fff;
		font-size: 11px;
		font-weight: 700;
		letter-spacing: 0.05em;
	}

	.actions-cell {
		display: flex;
		gap: 8px;
	}

	.notes-row td {
		padding: 12px 16px;
		background: #f8fafc;
		font-size: 13px;
		color: #64748b;
		border-bottom: 2px solid #e2e8f0;
	}

	.notes-label {
		font-weight: 600;
		color: #475569;
	}

	/* Alerts List */
	.alerts-list {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.alert-card {
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		transition: all 0.2s;
	}

	.alert-card:hover {
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
	}

	.alert-card.is-new {
		border-left: 4px solid #22c55e;
	}

	.alert-card.is-pinned {
		border-left: 4px solid #2563eb;
		background: #fafbff;
	}

	.alert-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 12px;
	}

	.alert-meta {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.alert-type {
		padding: 4px 10px;
		border-radius: 6px;
		color: #fff;
		font-size: 11px;
		font-weight: 700;
	}

	.alert-ticker {
		font-weight: 700;
		color: #1e293b;
		font-size: 16px;
	}

	.new-badge {
		background: #dcfce7;
		color: #166534;
		padding: 3px 8px;
		border-radius: 4px;
		font-size: 10px;
		font-weight: 700;
	}

	.pinned-badge {
		background: #dbeafe;
		color: #1d4ed8;
		padding: 3px 8px;
		border-radius: 4px;
		font-size: 10px;
		font-weight: 700;
	}

	.alert-actions {
		display: flex;
		gap: 8px;
	}

	.alert-card h3 {
		font-size: 18px;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 8px 0;
	}

	.alert-message {
		color: #64748b;
		margin: 0 0 12px 0;
		line-height: 1.6;
	}

	.alert-notes {
		background: #f8fafc;
		padding: 12px 16px;
		border-radius: 8px;
		margin-bottom: 12px;
	}

	.alert-notes .notes-label {
		display: block;
		font-size: 11px;
		text-transform: uppercase;
		color: #64748b;
		margin-bottom: 6px;
	}

	.alert-notes p {
		margin: 0;
		font-size: 14px;
		color: #475569;
		line-height: 1.6;
	}

	.alert-footer {
		display: flex;
		justify-content: flex-end;
	}

	.alert-time {
		font-size: 13px;
		color: #94a3b8;
	}

	/* Weekly Video */
	.current-video-card {
		background: #fff;
		border: 2px solid #143e59;
		border-radius: 16px;
		overflow: hidden;
		position: relative;
	}

	.video-badge {
		position: absolute;
		top: 16px;
		left: 16px;
		background: #143e59;
		color: #fff;
		padding: 6px 14px;
		border-radius: 6px;
		font-size: 11px;
		font-weight: 700;
		z-index: 1;
	}

	.video-content {
		display: flex;
		gap: 24px;
		padding: 24px;
	}

	.video-thumbnail {
		width: 320px;
		height: 180px;
		background-size: cover;
		background-position: center;
		background-color: #f1f5f9;
		border-radius: 12px;
		flex-shrink: 0;
		position: relative;
	}

	.video-thumbnail .duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-info {
		flex: 1;
	}

	.video-info h3 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 8px 0;
	}

	.video-info h4 {
		font-size: 16px;
		font-weight: 500;
		color: #475569;
		margin: 0 0 16px 0;
	}

	.video-url {
		font-size: 13px;
		color: #3b82f6;
		word-break: break-all;
		margin: 0 0 12px 0;
	}

	.video-date {
		font-size: 13px;
		color: #94a3b8;
	}

	.archived-section {
		margin-top: 32px;
	}

	.archived-section h3 {
		font-size: 16px;
		font-weight: 600;
		color: #64748b;
		margin: 0 0 16px 0;
	}

	.archived-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.archived-video {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 14px 18px;
		background: #f8fafc;
		border-radius: 10px;
	}

	.archived-week {
		font-weight: 600;
		color: #1e293b;
		min-width: 200px;
	}

	.archived-title {
		flex: 1;
		color: #64748b;
	}

	.archived-date {
		font-size: 13px;
		color: #94a3b8;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		background: #fff;
		border-radius: 16px;
		width: 100%;
		max-width: 640px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h2 {
		font-size: 20px;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		padding: 4px;
	}

	.close-btn:hover {
		color: #1e293b;
	}

	.modal-body {
		padding: 24px;
	}

	.form-row {
		display: flex;
		gap: 16px;
		margin-bottom: 20px;
	}

	.form-row.targets {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
	}

	.form-group {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group.full-width {
		margin-bottom: 20px;
	}

	.form-group.flex-2 {
		flex: 2;
	}

	.form-group label {
		font-size: 13px;
		font-weight: 600;
		color: #475569;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 12px 14px;
		border: 1.5px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
	}

	.form-group textarea {
		resize: vertical;
	}

	.form-row.checkboxes {
		display: flex;
		gap: 24px;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 14px;
		color: #475569;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: #143e59;
	}

	.info-box {
		background: #f0f9ff;
		border: 1px solid #bae6fd;
		border-radius: 10px;
		padding: 14px 18px;
		font-size: 14px;
		color: #0369a1;
		margin-bottom: 20px;
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding-top: 20px;
		border-top: 1px solid #e2e8f0;
	}

	/* Filter Pills */
	.filter-pills {
		display: flex;
		gap: 8px;
	}

	.pill {
		padding: 8px 16px;
		background: #f1f5f9;
		border: none;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.pill:hover {
		background: #e2e8f0;
	}

	.pill.active {
		background: #143e59;
		color: #fff;
	}

	/* Trade Tracker Styles */
	.trades-table tr.is-open {
		background: #fafbff;
	}

	.type-badge {
		display: inline-block;
		padding: 3px 8px;
		background: #f1f5f9;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
	}

	.direction-badge {
		display: inline-block;
		padding: 3px 8px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
	}

	.direction-badge.long {
		background: #dcfce7;
		color: #166534;
	}

	.direction-badge.short {
		background: #fef2f2;
		color: #991b1b;
	}

	.entry-cell {
		font-weight: 600;
		color: #1e293b;
	}

	.pending {
		color: #94a3b8;
	}

	.pnl-value {
		font-weight: 600;
	}

	.pnl-value small {
		font-weight: 500;
		opacity: 0.8;
	}

	.status-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 6px;
		font-size: 11px;
		font-weight: 700;
	}

	.status-badge.open {
		background: #fef3c7;
		color: #92400e;
	}

	.status-badge.win {
		background: #dcfce7;
		color: #166534;
	}

	.status-badge.loss {
		background: #fef2f2;
		color: #991b1b;
	}

	.btn-close-trade {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 6px 12px;
		background: #22c55e;
		color: #fff;
		border: none;
		border-radius: 6px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-close-trade:hover {
		background: #16a34a;
	}

	/* Trade Summary in Modal */
	.trade-summary {
		background: #f8fafc;
		border-radius: 10px;
		padding: 16px;
		margin-bottom: 20px;
	}

	.trade-summary-row {
		display: flex;
		justify-content: space-between;
		padding: 8px 0;
		border-bottom: 1px solid #e2e8f0;
	}

	.trade-summary-row:last-child {
		border-bottom: none;
	}

	.trade-summary-row .label {
		color: #64748b;
		font-size: 14px;
	}

	.trade-summary-row .value {
		color: #1e293b;
		font-size: 14px;
	}

	/* Video Library Grid */
	.video-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	.video-card {
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		overflow: hidden;
		transition: all 0.2s;
	}

	.video-card:hover {
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
	}

	.video-card-thumbnail {
		height: 160px;
		background-size: cover;
		background-position: center;
		background-color: #f1f5f9;
		position: relative;
	}

	.video-card-thumbnail .duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 3px 8px;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-card-thumbnail .featured-badge {
		position: absolute;
		top: 8px;
		left: 8px;
		background: #f59e0b;
		color: #fff;
		padding: 3px 10px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 700;
	}

	.video-card-content {
		padding: 16px;
	}

	.video-card-content h4 {
		font-size: 15px;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 8px 0;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.video-card-date {
		font-size: 13px;
		color: #64748b;
		margin: 0 0 8px 0;
	}

	.category-badge {
		display: inline-block;
		padding: 3px 10px;
		background: #e2e8f0;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		color: #475569;
		text-transform: capitalize;
	}

	.video-card-stats {
		margin-top: 8px;
		font-size: 12px;
		color: #94a3b8;
	}

	.video-card-actions {
		display: flex;
		gap: 8px;
		padding: 12px 16px;
		border-top: 1px solid #e2e8f0;
		background: #fafafa;
	}

	/* Header Actions */
	.header-actions {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	/* TOS Section */
	.tos-section {
		background: #f8fafc;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		margin-bottom: 20px;
	}

	.section-title {
		font-size: 14px;
		font-weight: 600;
		color: #475569;
		margin: 0 0 16px 0;
		padding-bottom: 12px;
		border-bottom: 1px solid #e2e8f0;
	}

	@media (max-width: 768px) {
		.admin-page {
			padding: 20px;
		}

		.header-row {
			flex-direction: column;
			align-items: flex-start;
		}

		.stats-panel {
			width: 100%;
			justify-content: space-between;
		}

		.form-row {
			flex-direction: column;
		}

		.form-row.targets {
			grid-template-columns: repeat(2, 1fr);
		}

		.video-content {
			flex-direction: column;
		}

		.video-thumbnail {
			width: 100%;
			height: 200px;
		}

		.filter-pills {
			flex-wrap: wrap;
		}

		.video-grid {
			grid-template-columns: 1fr;
		}

		.section-header {
			flex-direction: column;
			gap: 16px;
			align-items: flex-start;
		}
	}
</style>

<ConfirmationModal
	isOpen={showDeleteTradePlanModal}
	title="Delete Trade Plan Entry"
	message={pendingDeleteTradePlan ? `Delete ${pendingDeleteTradePlan.ticker} from trade plan?` : ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteTradePlan}
	onCancel={() => { showDeleteTradePlanModal = false; pendingDeleteTradePlan = null; }}
/>

<ConfirmationModal
	isOpen={showDeleteAlertModal}
	title="Delete Alert"
	message={pendingDeleteAlert ? `Delete alert "${pendingDeleteAlert.title}"?` : ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteAlert}
	onCancel={() => { showDeleteAlertModal = false; pendingDeleteAlert = null; }}
/>

<ConfirmationModal
	isOpen={showDeleteTradeModal}
	title="Delete Trade"
	message={pendingDeleteTrade ? `Delete trade ${pendingDeleteTrade.ticker}? This cannot be undone.` : ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteTrade}
	onCancel={() => { showDeleteTradeModal = false; pendingDeleteTrade = null; }}
/>

<ConfirmationModal
	isOpen={showDeleteVideoModal}
	title="Delete Video"
	message="Delete this video?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteVideo}
	onCancel={() => { showDeleteVideoModal = false; pendingDeleteVideoId = null; }}
/>

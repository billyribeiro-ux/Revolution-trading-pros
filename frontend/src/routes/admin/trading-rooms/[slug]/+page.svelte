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
	import type { PageData } from './$types';
	import { getRoomById } from '$lib/config/rooms';
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
	import { logger } from '$lib/utils/logger';

	// Icons
	import IconTable from '@tabler/icons-svelte-runes/icons/table';
	import IconBell from '@tabler/icons-svelte-runes/icons/bell';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconPin from '@tabler/icons-svelte-runes/icons/pin';
	import IconPinFilled from '@tabler/icons-svelte-runes/icons/pin-filled';
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconCurrencyDollar from '@tabler/icons-svelte-runes/icons/currency-dollar';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import PageHeader from './_components/PageHeader.svelte';
	import MessagesBanner from './_components/MessagesBanner.svelte';
	import TabNavigation from './_components/TabNavigation.svelte';
	import WeeklyVideoPanel from './_components/WeeklyVideoPanel.svelte';
	import TradePlanModal from './_components/TradePlanModal.svelte';
	import WeeklyVideoModal from './_components/WeeklyVideoModal.svelte';
	import CloseTradeModal from './_components/CloseTradeModal.svelte';
	import AlertModal from './_components/AlertModal.svelte';

	// ═══════════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════════

	type Tab = 'trade-plan' | 'alerts' | 'weekly-video' | 'trades' | 'video-library';

	// ═══════════════════════════════════════════════════════════════════════════════
	// PAGE DATA FROM LOAD FUNCTION (SSR)
	// ═══════════════════════════════════════════════════════════════════════════════

	const { data }: { data: PageData } = $props();

	// ═══════════════════════════════════════════════════════════════════════════════
	// ROUTE DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	const slug = $derived(data.slug ?? page.params.slug ?? '');
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

	// FIX-2026-04-26-audit (P0-7): seed local state from SSR `data.initialData` ONCE
	// at construction. The previous `$effect` shadow pattern (banned by CLAUDE.md)
	// re-clobbered local edits any time `data` changed. `untrack` silences the
	// `state_referenced_locally` warning since we DO want a one-shot initial value
	// (mutated by load functions / optimistic updates afterwards, not synced).
	let tradePlanEntries = $state<TradePlanEntry[]>(untrack(() => data.initialData?.tradePlan ?? []));
	let isLoadingTradePlan = $state(false);
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

	// FIX-2026-04-26-audit (P0-7): seed from SSR data.initialData via untrack.
	let alerts = $state<RoomAlert[]>(untrack(() => data.initialData?.alerts ?? []));
	let isLoadingAlerts = $state(false);
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

	// FIX-2026-04-26-audit (P0-7): seed currentVideo from SSR via untrack.
	let currentVideo = $state<WeeklyVideo | null>(
		untrack(() => data.initialData?.weeklyVideo ?? null)
	);
	let archivedVideos = $state<WeeklyVideo[]>([]);
	let isLoadingVideo = $state(false);
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

	// FIX-2026-04-26-audit (P0-7): seed roomStats from SSR via untrack.
	let roomStats = $state<RoomStats | null>(untrack(() => data.initialData?.roomStats ?? null));
	let isLoadingStats = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════════
	// TRADE TRACKER STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	// FIX-2026-04-26-audit (P0-7): seed trades from SSR via untrack.
	let trades = $state<RoomTrade[]>(untrack(() => data.initialData?.trades ?? []));
	let isLoadingTrades = $state(false);
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

	// FIX-2026-04-26-audit (P0-7): seed videoResources from SSR via untrack.
	let videoResources = $state<RoomResource[]>(
		untrack(() => data.initialData?.videoResources ?? [])
	);
	let isLoadingVideos = $state(false);
	let videoFilter = $state<string>('all');

	// Video resource management removed - functionality not implemented

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

	// Removed unused derived values - functionality not implemented
	// const isLoadingAny = $derived(
	// 	isLoadingTradePlan || isLoadingAlerts || isLoadingVideo || isLoadingStats
	// );
	// const isSavingAny = $derived(isSavingTradePlan || isSavingAlert || isSavingVideo);

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
	// NOTE: Shadow-state $effect blocks removed 2026-04-26-audit (P0-7).
	// Each state variable is seeded from SSR data via `untrack()` in its declaration.
	// Re-syncing through $effects would clobber optimistic local mutations whenever
	// SvelteKit re-runs the load function. To reload from the server after a mutation,
	// call the individual load*() functions explicitly.
	// ═══════════════════════════════════════════════════════════════════════════════

	// ═══════════════════════════════════════════════════════════════════════════════
	// DATA FETCHING
	// ═══════════════════════════════════════════════════════════════════════════════

	// SSR data is already loaded from +page.server.ts
	// Only refresh on client when slug changes (navigation)
	//
	// FIX-2026-04-26-audit (P1-12): the previous condition was
	// `data.slug !== currentSlug`, but `slug` is `$derived(data.slug ?? …)` —
	// so `currentSlug === data.slug` whenever `data.slug` is set, meaning the
	// branch was unreachable in normal navigation. Track the last loaded slug
	// in a local module variable instead, so we refetch exactly once per
	// real navigation transition. Seed it with the initial SSR slug (read via
	// untrack to silence Svelte's state_referenced_locally warning) so the
	// first effect run is a no-op (SSR already populated all six lists).
	let lastLoadedSlug: string | null = untrack(() => data.slug ?? null);

	$effect(() => {
		if (!browser) return;
		const currentSlug = slug;
		if (!currentSlug) return;
		if (currentSlug === lastLoadedSlug) return;

		lastLoadedSlug = currentSlug;
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
			logger.error('Failed to load trade plan', { error: err });
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
			logger.error('Failed to load alerts', { error: err });
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
			logger.error('Failed to load weekly video', { error: err });
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
			logger.error('Failed to load room stats', { error: err });
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
			logger.error('Failed to load trades', { error: err });
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
			logger.error('Failed to load video resources', { error: err });
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
				await tradePlanApi.update(editingTradePlan.id, slug, tradePlanForm);
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to save trade plan';
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to delete entry';
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
			trade_type: (alert.trade_type || '') as '' | 'options' | 'shares',
			action: (alert.action || '') as '' | 'BUY' | 'SELL',
			quantity: alert.quantity?.toString() || '',
			option_type: (alert.option_type || '') as '' | 'CALL' | 'PUT',
			strike: alert.strike?.toString() || '',
			expiration: alert.expiration || '',
			contract_type: (alert.contract_type || '') as '' | 'Weeklys' | 'Monthly' | 'LEAPS',
			order_type: (alert.order_type || '') as '' | 'MKT' | 'LMT',
			limit_price: alert.limit_price?.toString() || '',
			fill_price: alert.fill_price?.toString() || '',
			tos_string: alert.tos_string || '',
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to save alert';
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to delete alert';
		}
	}

	async function toggleAlertPin(alert: RoomAlert) {
		try {
			await alertsApi.update(alert.id, { is_pinned: !alert.is_pinned });
			successMessage = alert.is_pinned ? 'Alert unpinned' : 'Alert pinned';
			await loadAlerts();
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to update alert';
		}
	}

	async function toggleAlertNew(alert: RoomAlert) {
		try {
			await alertsApi.update(alert.id, { is_new: !alert.is_new });
			successMessage = alert.is_new ? 'Alert marked as read' : 'Alert marked as new';
			await loadAlerts();
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to update alert';
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to save video';
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to close trade';
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to delete trade';
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
		} catch (err) {
			const error = err as { message?: string };
			errorMessage = error.message || 'Failed to delete video';
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
	<PageHeader {room} {roomStats} {isLoadingStats} />

	<!-- Messages -->
	<MessagesBanner {successMessage} {errorMessage} onDismissError={() => (errorMessage = '')} />

	<!-- Tabs -->
	<TabNavigation
		bind:activeTab
		{tradePlanCount}
		{alertsCount}
		{hasCurrentVideo}
		{tradesCount}
		{activeTradesCount}
		{videosCount}
	/>

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
							{#each tradePlanEntries as entry (entry.id)}
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
					{#each filteredAlerts as alert (alert.id)}
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

			<WeeklyVideoPanel
				{isLoadingVideo}
				{currentVideo}
				{archivedVideos}
				{hasArchivedVideos}
				onAddVideo={openAddVideo}
			/>
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
							{#each filteredTrades as trade (trade.id)}
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
					{#each filteredVideos as video (video.id)}
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
<TradePlanModal
	open={showTradePlanModal}
	{editingTradePlan}
	bind:form={tradePlanForm}
	isSaving={isSavingTradePlan}
	isValid={isTradePlanFormValid}
	onSave={saveTradePlan}
	onClose={() => (showTradePlanModal = false)}
/>

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     ALERT MODAL
     ═══════════════════════════════════════════════════════════════════════════════════ -->
<AlertModal
	open={showAlertModal}
	{editingAlert}
	bind:form={alertForm}
	isSaving={isSavingAlert}
	isValid={isAlertFormValid}
	onSave={saveAlert}
	onClose={() => (showAlertModal = false)}
/>

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     WEEKLY VIDEO MODAL
     ═══════════════════════════════════════════════════════════════════════════════════ -->
<WeeklyVideoModal
	open={showVideoModal}
	bind:form={videoForm}
	isSaving={isSavingVideo}
	isValid={isVideoFormValid}
	onSave={saveVideo}
	onClose={() => (showVideoModal = false)}
/>

<!-- ═══════════════════════════════════════════════════════════════════════════════════
     CLOSE TRADE MODAL
     ═══════════════════════════════════════════════════════════════════════════════════ -->
<CloseTradeModal
	open={showCloseTradeModal}
	{closingTrade}
	bind:form={closeTradeForm}
	isClosing={isClosingTrade}
	isValid={isCloseTradeFormValid}
	{formatCurrency}
	onClose={() => (showCloseTradeModal = false)}
	onSubmit={closeTrade}
/>

<ConfirmationModal
	isOpen={showDeleteTradePlanModal}
	title="Delete Trade Plan Entry"
	message={pendingDeleteTradePlan ? `Delete ${pendingDeleteTradePlan.ticker} from trade plan?` : ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteTradePlan}
	onCancel={() => {
		showDeleteTradePlanModal = false;
		pendingDeleteTradePlan = null;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteAlertModal}
	title="Delete Alert"
	message={pendingDeleteAlert ? `Delete alert "${pendingDeleteAlert.title}"?` : ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteAlert}
	onCancel={() => {
		showDeleteAlertModal = false;
		pendingDeleteAlert = null;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteTradeModal}
	title="Delete Trade"
	message={pendingDeleteTrade
		? `Delete trade ${pendingDeleteTrade.ticker}? This cannot be undone.`
		: ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteTrade}
	onCancel={() => {
		showDeleteTradeModal = false;
		pendingDeleteTrade = null;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteVideoModal}
	title="Delete Video"
	message="Delete this video?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteVideo}
	onCancel={() => {
		showDeleteVideoModal = false;
		pendingDeleteVideoId = null;
	}}
/>

<style>
	/* Base Styles */
	.admin-page {
		padding: 32px;
		max-width: 1400px;
		margin: 0 auto;
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

	@media (max-width: 767.98px) {
		.admin-page {
			padding: 20px;
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

<script lang="ts">
	/**
	 * Explosive Swings - Member Dashboard
	 * @version 6.0.0 - WebSocket Real-Time Alerts (Phase 3)
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import { onMount, onDestroy } from 'svelte';
	import { createPageState } from './page.state.svelte';
	import { createRealtimeState } from './realtime.svelte';

	// Layout Components
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import Pagination from '$lib/components/dashboard/pagination/Pagination.svelte';

	// Page Components
	import PerformanceSummary from './components/PerformanceSummary.svelte';
	import WeeklyHero from './components/WeeklyHero.svelte';
	import AlertCard from './components/AlertCard.svelte';
	import AlertFilters from '$lib/components/dashboard/alerts/AlertFilters.svelte';
	import SidebarComponent from './components/Sidebar.svelte';
	import NewAlertPulse from './components/NewAlertPulse.svelte';

	// Modals
	import TradeAlertModal from '$lib/components/dashboard/TradeAlertModal.svelte';
	import TradeEntryModal from './components/TradeEntryModal.svelte';
	import VideoUploadModal from './components/VideoUploadModal.svelte';
	import PublishWeeklyModal from './components/PublishWeeklyModal.svelte';
	import ClosePositionModal from './components/ClosePositionModal.svelte';
	import AddTradeModal from './components/AddTradeModal.svelte';
	import UpdatePositionModal from './components/UpdatePositionModal.svelte';
	import InvalidatePositionModal from './components/InvalidatePositionModal.svelte';

	// Types
	import type { AlertCreateInput, AlertUpdateInput } from '$lib/types/trading';
	import type { WatchlistData } from '$lib/server/watchlist';
	import type { RoomResource } from '$lib/api/room-resources';

	// Props from +page.server.ts - ICT 7 Fix: Proper type definitions (no any)
	interface PageData {
		watchlist: WatchlistData | null;
		tutorialVideo: RoomResource | null;
		latestUpdates: RoomResource[];
		documents: RoomResource[];
		roomId: number;
	}
	let props: { data: PageData } = $props();
	let data = $derived(props.data);
	
	// ICT 7: Local error state for user feedback
	let saveAlertError = $state<string | null>(null);

	// Initialize state module (named 'ps' to avoid conflict with $state rune)
	const ps = createPageState();
	
	// ICT 7: Transform WeeklyContent to WeeklyVideo for Sidebar component
	// Must be defined after ps initialization
	const weeklyVideoForSidebar = $derived.by(() => {
		const wc = ps.weeklyContent;
		return {
			title: wc.videoTitle || wc.title,
			thumbnailUrl: wc.thumbnail,
			videoUrl: wc.videoUrl,
			duration: wc.duration,
			publishedAt: new Date() // WeeklyContent has formatted string, Sidebar expects Date
		};
	});
	
	// ICT 7: Transform RoomResource[] to Video[] for Sidebar component
	const latestUpdatesForSidebar = $derived.by(() => {
		return (data.latestUpdates ?? []).map((resource) => ({
			id: String(resource.id),
			title: resource.title,
			thumbnailUrl: resource.thumbnail_url ?? '',
			videoUrl: resource.embed_url || resource.file_url,
			duration: resource.formatted_duration ?? '',
			publishedAt: new Date(resource.created_at)
		}));
	});

	// Initialize real-time state for WebSocket updates (ICT 7+ Phase 3)
	const realtime = createRealtimeState('explosive-swings');

	// Connect real-time updates to page state
	realtime.setPageState({
		prependAlert: ps.prependAlert,
		updateAlert: ps.updateAlert,
		removeAlert: ps.removeAlert,
		fetchAllTrades: ps.fetchAllTrades,
		fetchStats: ps.fetchStats,
		fetchTradePlan: ps.fetchTradePlan,
		setStats: ps.setStats
	});

	// Local modal state (bind: requires local variables)
	let alertModalOpen = $state(false);

	// Expanded notes tracking (local UI state)
	let expandedNotes = $state(new Set<number>());

	// Sync modal state from state module
	$effect(() => {
		alertModalOpen = ps.isAlertModalOpen;
	});

	function toggleNotes(alertId: number) {
		const newSet = new Set(expandedNotes);
		if (newSet.has(alertId)) {
			newSet.delete(alertId);
		} else {
			newSet.add(alertId);
		}
		expandedNotes = newSet;
	}

	/**
	 * Handle saving alerts with proper error handling and user feedback
	 * @description ICT 7 Fix: Added try/catch with user-friendly error messages
	 */
	async function handleSaveAlert(alertData: AlertCreateInput | AlertUpdateInput, isEdit: boolean): Promise<void> {
		saveAlertError = null;
		
		const url =
			isEdit && ps.editingAlert
				? `/api/alerts/${ps.ROOM_SLUG}/${ps.editingAlert.id}`
				: `/api/alerts/${ps.ROOM_SLUG}`;

		try {
			const response = await fetch(url, {
				method: isEdit ? 'PUT' : 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify(alertData)
			});

			if (!response.ok) {
				const errorData = await response.json().catch(() => ({}));
				throw new Error(errorData.message || `Failed to ${isEdit ? 'update' : 'create'} alert`);
			}
			
			await ps.fetchAlerts();
			
			// Close modal on success
			alertModalOpen = false;
			ps.closeAlertModal();
		} catch (err) {
			const message = err instanceof Error ? err.message : 'An unexpected error occurred';
			saveAlertError = message;
			console.error('Failed to save alert:', err);
			// Don't re-throw - show error in UI instead
		}
	}

	async function handleDeleteAlert(alertId: number) {
		if (!confirm('Are you sure you want to delete this alert?')) return;
		const response = await fetch(`/api/alerts/${ps.ROOM_SLUG}/${alertId}`, {
			method: 'DELETE',
			credentials: 'include'
		});
		if (response.ok) await ps.fetchAlerts();
	}

	onMount(() => {
		ps.initializeData();

		// ICT 7+ Phase 3: Connect to WebSocket for real-time alerts
		realtime.connect();
	});

	onDestroy(() => {
		// ICT 7+ Phase 3: Disconnect WebSocket when leaving page
		realtime.disconnect();
	});
</script>

<svelte:head>
	<title>Explosive Swings Dashboard | Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader
	roomName="Explosive Swings"
	startHereUrl="/dashboard/explosive-swings/start-here"
/>

<div class="page">
	<!-- Performance Summary -->
	<PerformanceSummary
		performance={ps.weeklyPerformance}
		closedTrades={ps.closedTrades}
		activePositions={ps.activePositions}
		isLoading={ps.isLoadingStats || ps.isLoadingTrades}
		isAdmin={ps.isAdmin}
		onClosePosition={ps.openClosePositionModal}
		onUpdatePosition={ps.openUpdatePositionModal}
		onInvalidatePosition={ps.openInvalidatePositionModal}
		onDeletePosition={ps.deletePosition}
		onAddTrade={ps.openAddTradeModal}
	/>

	<!-- Weekly Hero -->
	<WeeklyHero
		weeklyContent={ps.weeklyContent}
		tradePlan={ps.tradePlan}
		isAdmin={ps.isAdmin}
		roomSlug={ps.ROOM_SLUG}
		onAddEntry={() => ps.openTradeEntryModal()}
		onEditEntry={(entry: any) => ps.openTradeEntryModal(entry)}
		onUploadVideo={ps.openPublishWeeklyModal}
	/>

	<!-- Main Grid -->
	<div class="main-grid">
		<!-- Alerts Section -->
		<section class="alerts-section">
			<div class="section-header">
				<div class="header-left">
					<h2>Live Alerts</h2>
					<!-- ICT 7+ Phase 3: Real-time connection status -->
					<div
						class="connection-status"
						class:connected={realtime.isConnected}
						class:reconnecting={realtime.isReconnecting}
					>
						<span class="status-dot"></span>
						<span class="status-text">
							{#if realtime.isConnected}
								Live
							{:else if realtime.isReconnecting}
								Reconnecting...
							{:else}
								Offline
							{/if}
						</span>
					</div>
					{#if realtime.unreadCount > 0}
						<NewAlertPulse
							size="sm"
							variant="entry"
							active={true}
							label="{realtime.unreadCount} new"
						/>
					{/if}
				</div>
				{#if ps.isAdmin}
					<button class="admin-btn" onclick={() => ps.openAlertModal()}>+ New Alert</button>
				{/if}
			</div>

			<AlertFilters selected={ps.selectedFilter} onFilterChange={ps.setFilter} />

			{#if ps.alertsError}
				<div class="error-banner">
					<p>{ps.alertsError}</p>
					<button onclick={ps.fetchAlerts}>Retry</button>
				</div>
			{/if}

			<div class="alerts-list">
				{#if ps.isLoadingAlerts}
					{#each Array(3) as _}
						<div class="alert-skeleton"></div>
					{/each}
				{:else if ps.filteredAlerts.length === 0}
					<div class="empty-state">
						<p>No alerts found</p>
						{#if ps.selectedFilter !== 'all'}
							<button onclick={() => ps.setFilter('all')}>Show all</button>
						{/if}
					</div>
				{:else}
					{#each ps.filteredAlerts as alert, index (alert.id)}
						<AlertCard
							{alert}
							{index}
							isAdmin={ps.isAdmin}
							isNotesExpanded={expandedNotes.has(alert.id)}
							isCopied={ps.copiedAlertId === alert.id}
							onToggleNotes={toggleNotes}
							onCopy={(a: any) => ps.copyTradeDetails(a)}
							onEdit={(a: any) => ps.openAlertModal(a)}
							onDelete={handleDeleteAlert}
						/>
					{/each}
				{/if}
			</div>

			{#if ps.pagination.total > 10}
				<Pagination
					currentPage={ps.currentPage}
					totalPages={ps.totalPages}
					totalItems={ps.pagination.total}
					itemsPerPage={10}
					onPageChange={ps.goToPage}
					itemLabel="alerts"
				/>
			{/if}

			<a href="/dashboard/explosive-swings/alerts" class="view-all">View All Alerts →</a>
		</section>

		<!-- Sidebar -->
		<!-- ICT 7 Fix: Use actual stats from API, transform types for Sidebar component -->
		<SidebarComponent
			thirtyDayPerformance={{
				winRate: ps.stats?.winRate ?? 0,
				totalAlerts: (ps.stats?.activeTrades ?? 0) + (ps.stats?.closedThisWeek ?? 0),
				profitableAlerts: Math.round(
					((ps.stats?.winRate ?? 0) / 100) * ((ps.stats?.activeTrades ?? 0) + (ps.stats?.closedThisWeek ?? 0))
				),
				avgWinPercent: ps.weeklyPerformance?.avgWinPercent ?? 0,
				avgLossPercent: ps.weeklyPerformance?.avgLossPercent ?? 0
			}}
			weeklyVideo={weeklyVideoForSidebar}
			latestUpdates={latestUpdatesForSidebar}
			isLoading={ps.isLoadingStats}
		/>
	</div>
</div>

<!-- Modals -->
<TradeAlertModal
	bind:isOpen={alertModalOpen}
	roomSlug={ps.ROOM_SLUG}
	editAlert={ps.editingAlert as any}
	entryAlerts={ps.alerts.filter((a) => a.type === 'ENTRY') as any}
	onClose={() => {
		alertModalOpen = false;
		ps.closeAlertModal();
	}}
	onSave={handleSaveAlert}
/>

<TradeEntryModal
	isOpen={ps.isTradeEntryModalOpen}
	roomSlug={ps.ROOM_SLUG}
	editEntry={ps.editingTradeEntry}
	onClose={ps.closeTradeEntryModal}
	onSuccess={ps.fetchTradePlan}
/>

<VideoUploadModal
	isOpen={ps.isVideoUploadModalOpen}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeVideoUploadModal}
	onSuccess={ps.fetchWeeklyVideo}
/>

<PublishWeeklyModal
	isOpen={ps.isPublishWeeklyModalOpen}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closePublishWeeklyModal}
	onSuccess={() => {
		ps.fetchWeeklyVideo();
		ps.fetchTradePlan();
	}}
/>

<ClosePositionModal
	isOpen={ps.isClosePositionModalOpen}
	position={ps.closingPosition}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeClosePositionModal}
	onSuccess={() => {
		ps.fetchAllTrades();
		ps.fetchStats();
	}}
/>

<AddTradeModal
	isOpen={ps.isAddTradeModalOpen}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeAddTradeModal}
	onSuccess={() => {
		ps.fetchAllTrades();
		ps.fetchStats();
	}}
/>

<UpdatePositionModal
	isOpen={ps.isUpdatePositionModalOpen}
	position={ps.updatingPosition}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeUpdatePositionModal}
	onSuccess={ps.handlePositionUpdated}
/>

<InvalidatePositionModal
	isOpen={ps.isInvalidatePositionModalOpen}
	position={ps.invalidatingPosition}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeInvalidatePositionModal}
	onSuccess={() => {
		ps.fetchAllTrades();
		ps.fetchStats();
	}}
/>

<style>
	.page {
		background: var(--color-bg-page);
		min-height: 100vh;
		min-height: 100dvh;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   MAIN GRID - Mobile-first responsive layout
	   Uses CSS custom property for sidebar width from design-tokens.css
	   ═══════════════════════════════════════════════════════════════════════ */
	.main-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-2);
		padding: var(--space-2);
		max-width: 1600px;
		margin: 0 auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   ALERTS SECTION - Card container with responsive padding
	   ═══════════════════════════════════════════════════════════════════════ */
	.alerts-section {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: var(--card-padding-sm);
		box-shadow: var(--shadow-sm);
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--space-2);
		flex-wrap: wrap;
		gap: var(--space-1);
	}

	.section-header h2 {
		font-size: var(--text-lg);
		font-weight: 700;
		margin: 0;
		color: var(--color-text-primary);
	}

	/* Header layout and connection status */
	.header-left {
		display: flex;
		align-items: center;
		gap: var(--space-1-5);
		flex-wrap: wrap;
	}

	.connection-status {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: var(--text-xs);
		font-weight: 500;
		color: var(--color-text-tertiary);
		padding: var(--space-0-5) var(--space-1);
		background: var(--color-bg-subtle);
		border-radius: 9999px;
	}

	.connection-status.connected {
		color: var(--color-profit, #10b981);
		background: var(--color-profit-bg, #d1fae5);
	}

	.connection-status.reconnecting {
		color: var(--color-watching, #f59e0b);
		background: var(--color-watching-bg, #fef3c7);
	}

	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: currentColor;
	}

	.connection-status.connected .status-dot {
		animation: pulse-dot 2s ease-in-out infinite;
	}

	@keyframes pulse-dot {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.status-text {
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.admin-btn {
		font-size: var(--text-sm);
		font-weight: 600;
		padding: var(--space-1) var(--space-2);
		min-height: var(--touch-target-min);
		background: var(--color-brand-secondary);
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: background 0.15s;
	}

	.admin-btn:hover {
		background: var(--color-brand-secondary-hover);
	}

	.alerts-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-1-5);
		margin-bottom: var(--space-2);
	}

	.alert-skeleton {
		height: 100px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	.empty-state {
		text-align: center;
		padding: var(--space-5) var(--space-3);
		color: var(--color-text-tertiary);
	}

	.empty-state button {
		margin-top: var(--space-1-5);
		padding: var(--space-1) var(--space-2);
		min-height: var(--touch-target-min);
		background: var(--color-brand-primary);
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
	}

	.error-banner {
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss-border);
		border-radius: 8px;
		padding: var(--space-1-5) var(--space-2);
		margin-bottom: var(--space-2);
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: var(--space-1);
		flex-wrap: wrap;
	}

	.error-banner p {
		margin: 0;
		color: var(--color-loss);
		font-size: var(--text-sm);
	}

	.error-banner button {
		padding: 6px var(--space-1-5);
		min-height: var(--touch-target-min);
		background: var(--color-loss);
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: var(--text-sm);
	}

	.view-all {
		display: block;
		text-align: center;
		padding: var(--space-1-5);
		min-height: var(--touch-target-min);
		color: var(--color-brand-primary);
		font-weight: 600;
		text-decoration: none;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.view-all:hover {
		text-decoration: underline;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile-first (min-width)
	   ═══════════════════════════════════════════════════════════════════════ */

	/* Tablet (768px+) */
	@media (min-width: 768px) {
		.main-grid {
			padding: var(--space-3);
			gap: var(--space-3);
		}

		.alerts-section {
			padding: var(--card-padding-md);
		}

		.section-header h2 {
			font-size: var(--text-xl);
		}
	}

	/* Desktop - Sidebar appears (1024px+) */
	@media (min-width: 1024px) {
		.main-grid {
			grid-template-columns: 1fr var(--sidebar-width);
			padding: var(--space-3);
			gap: var(--space-3);
		}

		.alerts-section {
			padding: var(--card-padding-lg);
		}
	}

	/* Large Desktop (1440px+) */
	@media (min-width: 1440px) {
		.main-grid {
			gap: var(--space-4);
			padding: var(--space-4);
		}
	}
</style>

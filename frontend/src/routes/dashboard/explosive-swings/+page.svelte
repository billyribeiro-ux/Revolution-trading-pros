<script lang="ts">
	/**
	 * Explosive Swings - Member Dashboard
	 * @version 5.0.0 - State Module Integration
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import { onMount } from 'svelte';
	import { createPageState } from './page.state.svelte';

	// Layout Components
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import Pagination from '$lib/components/dashboard/pagination/Pagination.svelte';

	// Page Components
	import PerformanceSummary from './components/PerformanceSummary.svelte';
	import WeeklyHero from './components/WeeklyHero.svelte';
	import AlertCard from '$lib/components/dashboard/alerts/AlertCard.svelte';
	import AlertFilters from '$lib/components/dashboard/alerts/AlertFilters.svelte';
	import SidebarComponent from './components/Sidebar.svelte';

	// Modals
	import TradeAlertModal from '$lib/components/dashboard/TradeAlertModal.svelte';
	import TradeEntryModal from './components/TradeEntryModal.svelte';
	import VideoUploadModal from './components/VideoUploadModal.svelte';
	import ClosePositionModal from './components/ClosePositionModal.svelte';
	import AddTradeModal from './components/AddTradeModal.svelte';
	import UpdatePositionModal from './components/UpdatePositionModal.svelte';

	// Types
	import type { AlertCreateInput, AlertUpdateInput } from '$lib/types/trading';

	// Props from +page.ts
	interface PageData {
		watchlist?: any;
		tutorialVideo?: any;
		latestUpdates?: any[];
		documents?: any[];
		roomId?: number;
	}
	const { data }: { data: PageData } = $props();

	// Initialize state module (named 'ps' to avoid conflict with $state rune)
	const ps = createPageState();

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

	// Alert handlers
	async function handleSaveAlert(alertData: AlertCreateInput | AlertUpdateInput, isEdit: boolean) {
		const url = isEdit && ps.editingAlert
			? `/api/alerts/${ps.ROOM_SLUG}/${ps.editingAlert.id}`
			: `/api/alerts/${ps.ROOM_SLUG}`;

		const response = await fetch(url, {
method: isEdit ? 'PUT' : 'POST',
headers: { 'Content-Type': 'application/json' },
body: JSON.stringify(alertData)
});

		if (!response.ok) throw new Error('Failed to save alert');
		await ps.fetchAlerts();
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
	});
</script>

<svelte:head>
	<title>Explosive Swings Dashboard | Revolution Trading Pros</title>
</svelte:head>

<TradingRoomHeader roomName="Explosive Swings" startHereUrl="/dashboard/explosive-swings/start-here" />

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
		onUploadVideo={ps.openVideoUploadModal}
	/>

	<!-- Main Grid -->
	<div class="main-grid">
		<!-- Alerts Section -->
		<section class="alerts-section">
			<div class="section-header">
				<h2>Live Alerts</h2>
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
		<SidebarComponent
			thirtyDayPerformance={{
				winRate: ps.stats?.winRate ?? 75,
				totalAlerts: ps.stats?.closedThisWeek ?? 0,
				profitableAlerts: Math.round((ps.stats?.winRate ?? 75) / 100 * (ps.stats?.closedThisWeek ?? 0)),
				avgWinPercent: 5.7,
				avgLossPercent: 2.1
			}}
			weeklyVideo={ps.weeklyContent as any}
			latestUpdates={[]}
			isLoading={ps.isLoadingStats}
		/>
	</div>
</div>

<!-- Modals -->
<TradeAlertModal
	bind:isOpen={alertModalOpen}
	roomSlug={ps.ROOM_SLUG}
	editAlert={ps.editingAlert as any}
	entryAlerts={ps.alerts.filter(a => a.type === 'ENTRY') as any}
	onClose={() => { alertModalOpen = false; ps.closeAlertModal(); }}
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

<ClosePositionModal
	isOpen={ps.isClosePositionModalOpen}
	position={ps.closingPosition}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeClosePositionModal}
	onSuccess={() => { ps.fetchAllTrades(); ps.fetchStats(); }}
/>

<AddTradeModal
	isOpen={ps.isAddTradeModalOpen}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeAddTradeModal}
	onSuccess={() => { ps.fetchAllTrades(); ps.fetchStats(); }}
/>

<UpdatePositionModal
	isOpen={ps.isUpdatePositionModalOpen}
	position={ps.updatingPosition}
	roomSlug={ps.ROOM_SLUG}
	onClose={ps.closeUpdatePositionModal}
	onSuccess={ps.handlePositionUpdated}
/>

<style>
	.page {
		background: var(--color-bg-page);
		min-height: 100vh;
	}

	.main-grid {
		display: grid;
		grid-template-columns: 1fr 340px;
		gap: 24px;
		padding: 24px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.alerts-section {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
	}

	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
	}

	.section-header h2 {
		font-size: 18px;
		font-weight: 700;
		margin: 0;
		color: var(--color-text-primary);
	}

	.admin-btn {
		font-size: 13px;
		font-weight: 600;
		padding: 8px 14px;
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
		gap: 12px;
		margin-bottom: 16px;
	}

	.alert-skeleton {
		height: 100px;
		background: linear-gradient(90deg, var(--color-bg-subtle) 25%, var(--color-bg-muted) 50%, var(--color-bg-subtle) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 8px;
	}

	.empty-state {
		text-align: center;
		padding: 40px 20px;
		color: var(--color-text-tertiary);
	}

	.empty-state button {
		margin-top: 12px;
		padding: 8px 16px;
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
		padding: 12px 16px;
		margin-bottom: 16px;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.error-banner p {
		margin: 0;
		color: var(--color-loss);
		font-size: 14px;
	}

	.error-banner button {
		padding: 6px 12px;
		background: var(--color-loss);
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		font-size: 13px;
	}

	.view-all {
		display: block;
		text-align: center;
		padding: 12px;
		color: var(--color-brand-primary);
		font-weight: 600;
		text-decoration: none;
	}

	.view-all:hover {
		text-decoration: underline;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	@media (max-width: 1024px) {
		.main-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 768px) {
		.main-grid {
			padding: 16px;
			gap: 16px;
		}
		.alerts-section {
			padding: 16px;
		}
	}
</style>

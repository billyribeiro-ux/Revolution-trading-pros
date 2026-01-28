<!--
	/admin/crm/deals - Deal Pipeline & Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Pipeline/Kanban view with drag-and-drop
	- Deal listing with filters
	- Stage progression tracking
	- Win/Loss management
	- Revenue forecasting
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	/**
	 * CRM Deals - FluentCRM Pro Pipeline Management
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Full deal pipeline with:
	 * - Kanban board view
	 * - List view toggle
	 * - Drag and drop stage changes
	 * - Win/Loss actions
	 * - Revenue forecasting
	 *
	 * @version 1.0.0 (January 2026)
	 */

	import { onMount } from 'svelte';
	// Svelte 5 individual icon imports (Dec 2025 pattern)
	import IconBriefcase from '@tabler/icons-svelte/icons/briefcase';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconFilter from '@tabler/icons-svelte/icons/filter';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconLayoutKanban from '@tabler/icons-svelte/icons/layout-kanban';
	import IconList from '@tabler/icons-svelte/icons/list';
	import IconCurrencyDollar from '@tabler/icons-svelte/icons/currency-dollar';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconTarget from '@tabler/icons-svelte/icons/target';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconTrophy from '@tabler/icons-svelte/icons/trophy';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import { crmAPI } from '$lib/api/crm';
	import type { Deal, Pipeline, Stage, DealFilters, DealForecast } from '$lib/crm/types';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let deals = $state<Deal[]>([]);
	let pipelines = $state<Pipeline[]>([]);
	let selectedPipeline = $state<Pipeline | null>(null);
	let forecast = $state<DealForecast | null>(null);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let viewMode = $state<'kanban' | 'list'>('kanban');
	let selectedStatus = $state<'open' | 'won' | 'lost' | 'all'>('open');

	// Stats
	let stats = $state({
		totalDeals: 0,
		openDeals: 0,
		wonDeals: 0,
		lostDeals: 0,
		totalValue: 0,
		weightedValue: 0,
		avgDealSize: 0,
		winRate: 0
	});

	// Modal states
	let showWinModal = $state(false);
	let showLoseModal = $state(false);
	let selectedDeal = $state<Deal | null>(null);
	let winDetails = $state('');
	let lostReason = $state('');
	let processingAction = $state(false);

	// Drag and drop state
	let draggingDeal = $state<Deal | null>(null);
	let dragOverStage = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE (Svelte 5 $derived)
	// ═══════════════════════════════════════════════════════════════════════════

	let stages = $derived(selectedPipeline?.stages || []);

	let filteredDeals = $derived(
		deals.filter((deal) => {
			const matchesSearch =
				!searchQuery ||
				deal.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
				deal.contact?.full_name?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesStatus = selectedStatus === 'all' || deal.status === selectedStatus;
			const matchesPipeline = !selectedPipeline || deal.pipeline_id === selectedPipeline.id;
			return matchesSearch && matchesStatus && matchesPipeline;
		})
	);

	let dealsByStage = $derived(
		stages.reduce(
			(acc, stage) => {
				acc[stage.id] = filteredDeals.filter((deal) => deal.stage_id === stage.id);
				return acc;
			},
			{} as Record<string, Deal[]>
		)
	);

	let pipelineStats = $derived({
		totalValue: filteredDeals.reduce((sum, d) => sum + d.amount, 0),
		weightedValue: filteredDeals.reduce((sum, d) => sum + d.weighted_value, 0),
		dealsCount: filteredDeals.length
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadData() {
		isLoading = true;
		error = '';

		try {
			const [dealsRes, pipelinesRes, forecastRes] = await Promise.allSettled([
				crmAPI.getDeals({ status: selectedStatus !== 'all' ? selectedStatus : undefined }),
				crmAPI.getPipelines(),
				crmAPI.getDealForecast('this_month')
			]);

			if (dealsRes.status === 'fulfilled') {
				deals = dealsRes.value?.data || [];
			}

			if (pipelinesRes.status === 'fulfilled') {
				pipelines = pipelinesRes.value || [];
				if (pipelines.length > 0 && !selectedPipeline) {
					selectedPipeline = pipelines.find((p) => p.is_default) || pipelines[0];
				}
			}

			if (forecastRes.status === 'fulfilled') {
				forecast = forecastRes.value;
			}

			// Calculate stats
			const openDeals = deals.filter((d) => d.status === 'open');
			const wonDeals = deals.filter((d) => d.status === 'won');
			const lostDeals = deals.filter((d) => d.status === 'lost');

			stats = {
				totalDeals: deals.length,
				openDeals: openDeals.length,
				wonDeals: wonDeals.length,
				lostDeals: lostDeals.length,
				totalValue: openDeals.reduce((sum, d) => sum + d.amount, 0),
				weightedValue: openDeals.reduce((sum, d) => sum + d.weighted_value, 0),
				avgDealSize:
					openDeals.length > 0
						? openDeals.reduce((sum, d) => sum + d.amount, 0) / openDeals.length
						: 0,
				winRate:
					wonDeals.length + lostDeals.length > 0
						? (wonDeals.length / (wonDeals.length + lostDeals.length)) * 100
						: 0
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load deals';
		} finally {
			isLoading = false;
		}
	}

	async function updateDealStage(dealId: string, stageId: string) {
		try {
			await crmAPI.updateDealStage(dealId, stageId);
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to update deal stage';
		}
	}

	async function winDeal() {
		if (!selectedDeal) return;
		processingAction = true;

		try {
			await crmAPI.winDeal(selectedDeal.id, winDetails || undefined);
			showWinModal = false;
			selectedDeal = null;
			winDetails = '';
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to mark deal as won';
		} finally {
			processingAction = false;
		}
	}

	async function loseDeal() {
		if (!selectedDeal || !lostReason.trim()) return;
		processingAction = true;

		try {
			await crmAPI.loseDeal(selectedDeal.id, lostReason);
			showLoseModal = false;
			selectedDeal = null;
			lostReason = '';
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to mark deal as lost';
		} finally {
			processingAction = false;
		}
	}

	async function deleteDeal(deal: Deal) {
		if (!confirm(`Are you sure you want to delete "${deal.name}"? This action cannot be undone.`))
			return;

		try {
			await crmAPI.updateDeal(deal.id, { status: 'abandoned' } as any);
			await loadData();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete deal';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DRAG AND DROP HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleDragStart(event: DragEvent, deal: Deal) {
		draggingDeal = deal;
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
			event.dataTransfer.setData('text/plain', deal.id);
		}
	}

	function handleDragOver(event: DragEvent, stageId: string) {
		event.preventDefault();
		if (event.dataTransfer) {
			event.dataTransfer.dropEffect = 'move';
		}
		dragOverStage = stageId;
	}

	function handleDragLeave() {
		dragOverStage = null;
	}

	async function handleDrop(event: DragEvent, stageId: string) {
		event.preventDefault();
		dragOverStage = null;

		if (draggingDeal && draggingDeal.stage_id !== stageId) {
			await updateDealStage(draggingDeal.id, stageId);
		}
		draggingDeal = null;
	}

	function handleDragEnd() {
		draggingDeal = null;
		dragOverStage = null;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatCurrency(amount: number): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
			minimumFractionDigits: 0,
			maximumFractionDigits: 0
		}).format(amount);
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric'
		});
	}

	function getStageColor(stage: Stage): string {
		if (stage.is_closed_won) return '#22c55e';
		if (stage.is_closed_lost) return '#ef4444';
		return stage.color || '#6366f1';
	}

	function getPriorityColor(priority: string): string {
		const colors: Record<string, string> = {
			low: '#64748b',
			normal: '#3b82f6',
			high: '#f59e0b',
			urgent: '#ef4444'
		};
		return colors[priority] || colors.normal;
	}

	function getDaysColor(days: number): string {
		if (days <= 7) return '#22c55e';
		if (days <= 30) return '#f59e0b';
		return '#ef4444';
	}

	function openWinModal(deal: Deal) {
		selectedDeal = deal;
		winDetails = '';
		showWinModal = true;
	}

	function openLoseModal(deal: Deal) {
		selectedDeal = deal;
		lostReason = '';
		showLoseModal = true;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		loadData();
	});
</script>

<svelte:head>
	<title>Deals | CRM - Admin Dashboard</title>
</svelte:head>

<div class="admin-crm-deals">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header">
		<h1>
			<IconBriefcase size={28} />
			Deal Pipeline
		</h1>
		<p class="subtitle">Manage your sales pipeline and track deal progress</p>
		<div class="header-actions">
			<div class="view-toggle">
				<button
					class="toggle-btn"
					class:active={viewMode === 'kanban'}
					onclick={() => (viewMode = 'kanban')}
				>
					<IconLayoutKanban size={18} />
				</button>
				<button
					class="toggle-btn"
					class:active={viewMode === 'list'}
					onclick={() => (viewMode = 'list')}
				>
					<IconList size={18} />
				</button>
			</div>
			<button class="btn-secondary" onclick={() => loadData()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/deals/new" class="btn-primary">
				<IconPlus size={18} />
				New Deal
			</a>
		</div>
	</header>

	<!-- Stats Cards -->
	<section class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconBriefcase size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.openDeals}</span>
				<span class="stat-label">Open Deals</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconCurrencyDollar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatCurrency(stats.totalValue)}</span>
				<span class="stat-label">Pipeline Value</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon cyan">
				<IconTarget size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatCurrency(stats.weightedValue)}</span>
				<span class="stat-label">Weighted Value</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconTrendingUp size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.winRate.toFixed(1)}%</span>
				<span class="stat-label">Win Rate</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconChartBar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatCurrency(stats.avgDealSize)}</span>
				<span class="stat-label">Avg Deal Size</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon emerald">
				<IconTrophy size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.wonDeals}</span>
				<span class="stat-label">Won This Period</span>
			</div>
		</div>
	</section>

	<!-- Filters Bar -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" id="search-deals" name="search" placeholder="Search deals..." bind:value={searchQuery} />
		</div>

		{#if pipelines.length > 0}
			<select class="filter-select" bind:value={selectedPipeline}>
				{#each pipelines as pipeline}
					<option value={pipeline}>{pipeline.name}</option>
				{/each}
			</select>
		{/if}

		<select class="filter-select" bind:value={selectedStatus}>
			<option value="all">All Status</option>
			<option value="open">Open</option>
			<option value="won">Won</option>
			<option value="lost">Lost</option>
		</select>

		<div class="pipeline-stats">
			<span class="stat-item">
				<IconBriefcase size={14} />
				{pipelineStats.dealsCount} deals
			</span>
			<span class="stat-item">
				<IconCurrencyDollar size={14} />
				{formatCurrency(pipelineStats.totalValue)}
			</span>
		</div>
	</div>

	<!-- Main Content -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading deals...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<IconAlertTriangle size={48} />
			<p>{error}</p>
			<button class="btn-primary" onclick={() => loadData()}>Try Again</button>
		</div>
	{:else if viewMode === 'kanban'}
		<!-- Kanban Board -->
		<div class="kanban-board">
			{#each stages as stage (stage.id)}
				{@const stageDeals = dealsByStage[stage.id] || []}
				{@const stageValue = stageDeals.reduce((sum, d) => sum + d.amount, 0)}
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class="kanban-column"
					class:drag-over={dragOverStage === stage.id}
					ondragover={(e) => handleDragOver(e, stage.id)}
					ondragleave={handleDragLeave}
					ondrop={(e) => handleDrop(e, stage.id)}
				>
					<div class="column-header" style="border-color: {getStageColor(stage)}">
						<div class="column-title">
							<span class="stage-name">{stage.name}</span>
							<span class="stage-count">{stageDeals.length}</span>
						</div>
						<div class="column-value">{formatCurrency(stageValue)}</div>
						{#if stage.probability > 0}
							<div class="column-probability">{stage.probability}% probability</div>
						{/if}
					</div>

					<div class="column-cards">
						{#each stageDeals as deal (deal.id)}
							<!-- svelte-ignore a11y_no_static_element_interactions -->
							<div
								class="deal-card"
								draggable="true"
								class:dragging={draggingDeal?.id === deal.id}
								ondragstart={(e) => handleDragStart(e, deal)}
								ondragend={handleDragEnd}
								role="listitem"
							>
								<div class="card-grip">
									<IconGripVertical size={14} />
								</div>

								<div class="card-content">
									<a href="/admin/crm/deals/{deal.id}" class="deal-name">
										{deal.name}
									</a>

									<div class="deal-amount">
										{formatCurrency(deal.amount)}
									</div>

									{#if deal.contact}
										<div class="deal-contact">
											<IconUser size={12} />
											<span>{deal.contact.full_name}</span>
										</div>
									{/if}

									<div class="deal-meta">
										<span class="meta-item" style="color: {getDaysColor(deal.days_in_stage)}">
											<IconClock size={12} />
											{deal.days_in_stage}d in stage
										</span>
										{#if deal.expected_close_date}
											<span class="meta-item">
												<IconCalendar size={12} />
												{formatDate(deal.expected_close_date)}
											</span>
										{/if}
									</div>

									{#if deal.priority && deal.priority !== 'normal'}
										<span
											class="priority-badge"
											style="background: {getPriorityColor(
												deal.priority
											)}15; color: {getPriorityColor(deal.priority)}"
										>
											{deal.priority}
										</span>
									{/if}
								</div>

								<div class="card-actions">
									{#if deal.status === 'open'}
										<button
											class="action-btn success"
											title="Mark as Won"
											onclick={() => openWinModal(deal)}
										>
											<IconCheck size={14} />
										</button>
										<button
											class="action-btn danger"
											title="Mark as Lost"
											onclick={() => openLoseModal(deal)}
										>
											<IconX size={14} />
										</button>
									{/if}
									<a href="/admin/crm/deals/{deal.id}/edit" class="action-btn" title="Edit">
										<IconEdit size={14} />
									</a>
								</div>
							</div>
						{/each}

						{#if stageDeals.length === 0}
							<div class="empty-column">
								<p>No deals</p>
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<!-- List View -->
		<div class="table-container">
			{#if filteredDeals.length === 0}
				<div class="empty-state">
					<IconBriefcase size={48} />
					<h3>No deals found</h3>
					<p>Create your first deal to start tracking your sales pipeline</p>
					<a href="/admin/crm/deals/new" class="btn-primary">
						<IconPlus size={18} />
						Create Deal
					</a>
				</div>
			{:else}
				<table class="data-table">
					<thead>
						<tr>
							<th>Deal</th>
							<th>Contact</th>
							<th>Stage</th>
							<th>Amount</th>
							<th>Probability</th>
							<th>Close Date</th>
							<th>Days in Stage</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each filteredDeals as deal (deal.id)}
							<tr>
								<td>
									<a href="/admin/crm/deals/{deal.id}" class="deal-link">
										<span class="deal-title">{deal.name}</span>
										{#if deal.priority && deal.priority !== 'normal'}
											<span
												class="priority-badge small"
												style="background: {getPriorityColor(
													deal.priority
												)}15; color: {getPriorityColor(deal.priority)}"
											>
												{deal.priority}
											</span>
										{/if}
									</a>
								</td>
								<td>
									{#if deal.contact}
										<a href="/admin/crm/contacts/{deal.contact_id}" class="contact-link">
											{deal.contact.full_name}
										</a>
									{:else}
										<span class="text-muted">-</span>
									{/if}
								</td>
								<td>
									<span
										class="stage-badge"
										style="background: {getStageColor(
											deal.stage || stages.find((s) => s.id === deal.stage_id) || ({} as Stage)
										)}15; color: {getStageColor(
											deal.stage || stages.find((s) => s.id === deal.stage_id) || ({} as Stage)
										)}"
									>
										{deal.stage?.name ||
											stages.find((s) => s.id === deal.stage_id)?.name ||
											'Unknown'}
									</span>
								</td>
								<td class="amount-cell">
									{formatCurrency(deal.amount)}
								</td>
								<td>
									<div class="probability-bar">
										<div class="bar-fill" style="width: {deal.probability}%"></div>
										<span class="bar-text">{deal.probability}%</span>
									</div>
								</td>
								<td>
									{formatDate(deal.expected_close_date)}
								</td>
								<td>
									<span style="color: {getDaysColor(deal.days_in_stage)}">
										{deal.days_in_stage} days
									</span>
								</td>
								<td>
									<div class="action-buttons">
										{#if deal.status === 'open'}
											<button
												class="btn-icon success"
												title="Mark as Won"
												onclick={() => openWinModal(deal)}
											>
												<IconCheck size={16} />
											</button>
											<button
												class="btn-icon danger"
												title="Mark as Lost"
												onclick={() => openLoseModal(deal)}
											>
												<IconX size={16} />
											</button>
										{/if}
										<a href="/admin/crm/deals/{deal.id}" class="btn-icon" title="View">
											<IconEye size={16} />
										</a>
										<a href="/admin/crm/deals/{deal.id}/edit" class="btn-icon" title="Edit">
											<IconEdit size={16} />
										</a>
										<button class="btn-icon danger" title="Delete" onclick={() => deleteDeal(deal)}>
											<IconTrash size={16} />
										</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>

				<div class="table-footer">
					<span class="results-count">
						Showing {filteredDeals.length} of {deals.length} deals
					</span>
				</div>
			{/if}
		</div>
	{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Win Deal Modal -->
{#if showWinModal && selectedDeal}
	<div
		class="modal-overlay"
		onclick={() => {
			showWinModal = false;
			selectedDeal = null;
		}}
		onkeydown={(e) => e.key === 'Escape' && (showWinModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header success">
				<IconTrophy size={24} />
				<h3>Mark Deal as Won</h3>
				<button
					class="modal-close"
					onclick={() => {
						showWinModal = false;
						selectedDeal = null;
					}}
				>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="deal-summary">
					<p class="deal-name">{selectedDeal.name}</p>
					<p class="deal-value">{formatCurrency(selectedDeal.amount)}</p>
				</div>
				<div class="form-group">
					<label for="win-details">Win Details (optional)</label>
					<textarea
						id="win-details"
						bind:value={winDetails}
						placeholder="Add notes about how this deal was won..."
						rows="4"
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button
					class="btn-secondary"
					onclick={() => {
						showWinModal = false;
						selectedDeal = null;
					}}
					disabled={processingAction}
				>
					Cancel
				</button>
				<button class="btn-success" onclick={winDeal} disabled={processingAction}>
					{#if processingAction}
						<div class="btn-spinner"></div>
					{:else}
						<IconCheck size={18} />
					{/if}
					Mark as Won
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Lose Deal Modal -->
{#if showLoseModal && selectedDeal}
	<div
		class="modal-overlay"
		onclick={() => {
			showLoseModal = false;
			selectedDeal = null;
		}}
		onkeydown={(e) => e.key === 'Escape' && (showLoseModal = false)}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="modal-header danger">
				<IconX size={24} />
				<h3>Mark Deal as Lost</h3>
				<button
					class="modal-close"
					onclick={() => {
						showLoseModal = false;
						selectedDeal = null;
					}}
				>
					<IconX size={20} />
				</button>
			</div>
			<div class="modal-body">
				<div class="deal-summary">
					<p class="deal-name">{selectedDeal.name}</p>
					<p class="deal-value">{formatCurrency(selectedDeal.amount)}</p>
				</div>
				<div class="form-group">
					<label for="lost-reason">Lost Reason <span class="required">*</span></label>
					<textarea
						id="lost-reason"
						bind:value={lostReason}
						placeholder="Why was this deal lost? (required)"
						rows="4"
						required
					></textarea>
				</div>
			</div>
			<div class="modal-footer">
				<button
					class="btn-secondary"
					onclick={() => {
						showLoseModal = false;
						selectedDeal = null;
					}}
					disabled={processingAction}
				>
					Cancel
				</button>
				<button
					class="btn-danger"
					onclick={loseDeal}
					disabled={processingAction || !lostReason.trim()}
				>
					{#if processingAction}
						<div class="btn-spinner"></div>
					{:else}
						<IconX size={18} />
					{/if}
					Mark as Lost
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.admin-crm-deals {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 12px;
		margin: 0 0 0.5rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.page-header h1 :global(svg) {
		color: #e6b800;
	}

	.subtitle {
		margin: 0 0 1.5rem;
		color: #64748b;
		font-size: 0.875rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		align-items: center;
	}

	.view-toggle {
		display: flex;
		background: #1e293b;
		border-radius: 8px;
		padding: 4px;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.toggle-btn:hover {
		color: #e2e8f0;
	}

	.toggle-btn.active {
		background: linear-gradient(135deg, #e6b800, #b38f00);
		color: white;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 0.75rem 1rem;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 8px;
		color: #cbd5e1;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.5);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #e6b800, #b38f00);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 16px;
		margin-bottom: 24px;
	}

	@media (max-width: 1400px) {
		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 900px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid #334155;
		border-radius: 8px;
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.stat-icon.purple {
		background: rgba(139, 92, 246, 0.15);
		color: #a78bfa;
	}
	.stat-icon.cyan {
		background: rgba(6, 182, 212, 0.15);
		color: #22d3ee;
	}
	.stat-icon.amber {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}
	.stat-icon.emerald {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Filters Bar */
	.filters-bar {
		display: flex;
		gap: 12px;
		margin-bottom: 24px;
		flex-wrap: wrap;
		align-items: center;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 0 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 10px;
		flex: 1;
		max-width: 320px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 12px 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-select {
		padding: 12px 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.filter-select:focus {
		outline: none;
		border-color: #e6b800;
	}

	.pipeline-stats {
		display: flex;
		gap: 16px;
		margin-left: auto;
	}

	.pipeline-stats .stat-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	/* Kanban Board */
	.kanban-board {
		display: flex;
		gap: 16px;
		overflow-x: auto;
		padding-bottom: 16px;
		min-height: 600px;
	}

	.kanban-column {
		flex: 0 0 300px;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid #334155;
		border-radius: 8px;
		display: flex;
		flex-direction: column;
		max-height: calc(100vh - 350px);
		transition: all 0.2s;
	}

	.kanban-column.drag-over {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.5);
	}

	.column-header {
		padding: 16px;
		border-bottom: 1px solid #334155;
		border-left: 3px solid;
	}

	.column-title {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 4px;
	}

	.stage-name {
		font-weight: 600;
		color: white;
		font-size: 0.9rem;
	}

	.stage-count {
		padding: 2px 8px;
		background: rgba(99, 102, 241, 0.15);
		border-radius: 12px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #818cf8;
	}

	.column-value {
		font-size: 1.1rem;
		font-weight: 700;
		color: #e2e8f0;
	}

	.column-probability {
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 4px;
	}

	.column-cards {
		flex: 1;
		overflow-y: auto;
		padding: 12px;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.deal-card {
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 10px;
		padding: 12px;
		cursor: grab;
		transition: all 0.2s;
		display: grid;
		grid-template-columns: 16px 1fr;
		gap: 8px;
	}

	.deal-card:hover {
		border-color: rgba(99, 102, 241, 0.5);
		transform: translateY(-2px);
	}

	.deal-card.dragging {
		opacity: 0.5;
		cursor: grabbing;
	}

	.card-grip {
		color: #475569;
		cursor: grab;
	}

	.card-content {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.deal-name {
		font-weight: 600;
		color: white;
		font-size: 0.85rem;
		text-decoration: none;
	}

	.deal-name:hover {
		color: #818cf8;
	}

	.deal-amount {
		font-size: 1rem;
		font-weight: 700;
		color: #4ade80;
	}

	.deal-contact {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.deal-meta {
		display: flex;
		gap: 12px;
		flex-wrap: wrap;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 0.7rem;
		color: #64748b;
	}

	.priority-badge {
		display: inline-block;
		padding: 2px 8px;
		border-radius: 4px;
		font-size: 0.65rem;
		font-weight: 600;
		text-transform: uppercase;
		width: fit-content;
	}

	.priority-badge.small {
		padding: 1px 6px;
		font-size: 0.6rem;
		margin-left: 8px;
	}

	.card-actions {
		grid-column: span 2;
		display: flex;
		gap: 4px;
		margin-top: 8px;
		padding-top: 8px;
		border-top: 1px solid #334155;
	}

	.action-btn {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid #334155;
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.action-btn:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
		border-color: rgba(99, 102, 241, 0.3);
	}

	.action-btn.success:hover {
		background: rgba(34, 197, 94, 0.1);
		color: #4ade80;
		border-color: rgba(34, 197, 94, 0.3);
	}

	.action-btn.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.empty-column {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 40px 20px;
		color: #475569;
		font-size: 0.85rem;
	}

	/* Table View */
	.table-container {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid #334155;
		border-radius: 8px;
		overflow: hidden;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th {
		padding: 14px 16px;
		text-align: left;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		background: #0f172a;
		border-bottom: 1px solid #334155;
	}

	.data-table td {
		padding: 14px 16px;
		font-size: 0.875rem;
		color: #e2e8f0;
		border-bottom: 1px solid #1e293b;
	}

	.data-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.deal-link {
		display: flex;
		align-items: center;
		text-decoration: none;
	}

	.deal-title {
		font-weight: 600;
		color: white;
	}

	.deal-link:hover .deal-title {
		color: #818cf8;
	}

	.contact-link {
		color: #60a5fa;
		text-decoration: none;
	}

	.contact-link:hover {
		text-decoration: underline;
	}

	.stage-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 20px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.amount-cell {
		font-weight: 600;
		color: #4ade80;
	}

	.probability-bar {
		position: relative;
		width: 80px;
		height: 20px;
		background: #0f172a;
		border-radius: 10px;
		overflow: hidden;
	}

	.bar-fill {
		position: absolute;
		left: 0;
		top: 0;
		height: 100%;
		background: linear-gradient(135deg, #e6b800, #b38f00);
		transition: width 0.3s;
	}

	.bar-text {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		font-size: 0.7rem;
		font-weight: 600;
		color: white;
	}

	.text-muted {
		color: #64748b;
	}

	.action-buttons {
		display: flex;
		gap: 4px;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
		border-color: rgba(99, 102, 241, 0.3);
	}

	.btn-icon.success:hover {
		background: rgba(34, 197, 94, 0.1);
		color: #4ade80;
		border-color: rgba(34, 197, 94, 0.3);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.table-footer {
		padding: 16px;
		border-top: 1px solid #334155;
	}

	.results-count {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* States */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 60px 20px;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 16px;
	}

	.empty-state h3 {
		color: white;
		margin: 0 0 8px;
	}

	.empty-state p {
		margin: 0 0 20px;
	}

	.error-state :global(svg) {
		color: #f87171;
		margin-bottom: 16px;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #e6b800;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 20px;
	}

	.modal {
		width: 100%;
		max-width: 480px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		overflow: hidden;
	}

	.modal-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 20px;
		border-bottom: 1px solid #334155;
	}

	.modal-header.success {
		background: rgba(34, 197, 94, 0.1);
	}

	.modal-header.success :global(svg) {
		color: #4ade80;
	}

	.modal-header.danger {
		background: rgba(239, 68, 68, 0.1);
	}

	.modal-header.danger :global(svg) {
		color: #f87171;
	}

	.modal-header h3 {
		flex: 1;
		margin: 0;
		font-size: 1.1rem;
		color: white;
	}

	.modal-close {
		display: flex;
		padding: 8px;
		background: transparent;
		border: none;
		color: #64748b;
		cursor: pointer;
	}

	.modal-close:hover {
		color: white;
	}

	.modal-body {
		padding: 20px;
	}

	.deal-summary {
		padding: 16px;
		background: #0f172a;
		border-radius: 10px;
		margin-bottom: 20px;
	}

	.deal-summary .deal-name {
		margin: 0 0 4px;
		font-weight: 600;
		color: white;
	}

	.deal-summary .deal-value {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #4ade80;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group label {
		font-size: 0.85rem;
		font-weight: 600;
		color: #e2e8f0;
	}

	.form-group .required {
		color: #f87171;
	}

	.form-group textarea {
		padding: 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
	}

	.form-group textarea:focus {
		outline: none;
		border-color: #e6b800;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 12px;
		padding: 20px;
		border-top: 1px solid #334155;
	}

	.modal .btn-secondary {
		padding: 10px 20px;
	}

	.btn-success {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #22c55e, #16a34a);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-success:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(34, 197, 94, 0.4);
	}

	.btn-success:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #ef4444, #dc2626);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(239, 68, 68, 0.4);
	}

	.btn-danger:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.admin-crm-deals {
			padding: 1rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
		}

		.header-actions {
			flex-wrap: wrap;
		}

		.filters-bar {
			flex-direction: column;
		}

		.search-box {
			max-width: none;
		}

		.pipeline-stats {
			margin-left: 0;
			justify-content: center;
		}

		.kanban-board {
			min-height: 400px;
		}

		.kanban-column {
			flex: 0 0 280px;
		}
	}
</style>

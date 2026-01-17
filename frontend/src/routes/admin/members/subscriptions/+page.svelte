<script lang="ts">
	/**
	 * Member Subscriptions - Apple ICT7 Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 runes implementation with:
	 * - $state for reactive state management
	 * - $effect for lifecycle and data loading
	 * - $derived for computed values
	 * - Proper TypeScript types
	 * - Enhanced error handling
	 *
	 * @version 2.0.0 - Svelte 5 Migration (Dec 2025)
	 */

	import { goto } from '$app/navigation';
	import { toastStore } from '$lib/stores/toast.svelte';
	import {
		IconRefresh,
		IconSearch,
		IconFilter,
		IconDownload,
		IconEye,
		IconEdit,
		IconX,
		IconCreditCard
	} from '$lib/icons';
	import {
		getSubscriptions,
		cancelSubscription,
		pauseSubscription,
		resumeSubscription,
		exportSubscriptions,
		type EnhancedSubscription
	} from '$lib/api/subscriptions';

	// ═══════════════════════════════════════════════════════════════════════════════
	// State - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════════

	let subscriptions = $state<EnhancedSubscription[]>([]);
	let loading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let statusFilter = $state<string>('');
	let intervalFilter = $state<string>('');
	let showFilters = $state(false);
	let exporting = $state(false);

	// ═══════════════════════════════════════════════════════════════════════════════
	// Derived State
	// ═══════════════════════════════════════════════════════════════════════════════

	let filteredSubscriptions = $derived(
		subscriptions.filter((sub) => {
			// Search filter
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				const matchesSearch =
					sub.customer?.name?.toLowerCase().includes(query) ||
					sub.customer?.email?.toLowerCase().includes(query) ||
					sub.id?.toLowerCase().includes(query) ||
					sub.planId?.toLowerCase().includes(query);
				if (!matchesSearch) return false;
			}

			// Status filter
			if (statusFilter && sub.status !== statusFilter) {
				return false;
			}

			// Interval filter
			if (intervalFilter && sub.billingCycle?.interval !== intervalFilter) {
				return false;
			}

			return true;
		})
	);

	let subscriptionCounts = $derived({
		total: subscriptions.length,
		active: subscriptions.filter((s) => s.status === 'active').length,
		trial: subscriptions.filter((s) => s.status === 'trial').length,
		paused: subscriptions.filter((s) => s.status === 'on-hold').length,
		cancelled: subscriptions.filter((s) => s.status === 'cancelled').length
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════════

	$effect(() => {
		loadSubscriptions();
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════════

	async function loadSubscriptions() {
		loading = true;
		error = '';
		try {
			const filters: any = {};
			if (statusFilter) {
				filters.status = [statusFilter];
			}
			if (intervalFilter) {
				filters.interval = [intervalFilter];
			}
			subscriptions = await getSubscriptions(filters, true);
		} catch (e: unknown) {
			if (e instanceof Error) {
				if (e.message.includes('401') || e.message.includes('Unauthorized')) {
					goto('/login');
					return;
				}
				error = e.message;
			} else {
				error = 'Failed to load subscriptions. Please refresh the page or contact support.';
			}
			console.error('Failed to load subscriptions:', e);
		} finally {
			loading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Actions
	// ═══════════════════════════════════════════════════════════════════════════════

	async function handlePause(sub: EnhancedSubscription) {
		if (!confirm(`Pause subscription for ${sub.customer?.name || sub.customer?.email}?`)) return;
		try {
			await pauseSubscription(sub.id, 'Admin paused');
			toastStore.success('Subscription paused successfully');
			await loadSubscriptions();
		} catch (e) {
			toastStore.error('Failed to pause subscription');
			console.error(e);
		}
	}

	async function handleResume(sub: EnhancedSubscription) {
		try {
			await resumeSubscription(sub.id);
			toastStore.success('Subscription resumed successfully');
			await loadSubscriptions();
		} catch (e) {
			toastStore.error('Failed to resume subscription');
			console.error(e);
		}
	}

	async function handleCancel(sub: EnhancedSubscription) {
		if (!confirm(`Cancel subscription for ${sub.customer?.name || sub.customer?.email}? This cannot be undone.`)) return;
		try {
			await cancelSubscription(sub.id, 'Admin cancelled', true);
			toastStore.success('Subscription cancelled successfully');
			await loadSubscriptions();
		} catch (e) {
			toastStore.error('Failed to cancel subscription');
			console.error(e);
		}
	}

	async function handleExport() {
		exporting = true;
		try {
			const blob = await exportSubscriptions('csv');
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `subscriptions-export-${new Date().toISOString().split('T')[0]}.csv`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
			toastStore.success('Subscriptions exported successfully');
		} catch (e) {
			toastStore.error('Failed to export subscriptions');
			console.error(e);
		} finally {
			exporting = false;
		}
	}

	function clearFilters() {
		searchQuery = '';
		statusFilter = '';
		intervalFilter = '';
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// Formatters
	// ═══════════════════════════════════════════════════════════════════════════════

	function formatCurrency(amount: number, currency = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency,
			minimumFractionDigits: 0,
			maximumFractionDigits: 2
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		if (!dateString) return '-';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function getStatusClass(status: string): string {
		switch (status) {
			case 'active':
				return 'status-active';
			case 'trial':
				return 'status-trial';
			case 'on-hold':
			case 'paused':
				return 'status-paused';
			case 'cancelled':
			case 'pending-cancel':
				return 'status-cancelled';
			default:
				return 'status-default';
		}
	}

	function getStatusLabel(status: string): string {
		switch (status) {
			case 'active':
				return 'Active';
			case 'trial':
				return 'Trial';
			case 'on-hold':
				return 'Paused';
			case 'pending-cancel':
				return 'Pending Cancel';
			case 'cancelled':
				return 'Cancelled';
			default:
				return status;
		}
	}
</script>

<svelte:head>
	<title>Member Subscriptions | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="page">
	<!-- Header - CENTERED -->
	<div class="page-header">
		<h1>Member Subscriptions</h1>
		<p class="subtitle">Manage and monitor all member subscription plans</p>
	</div>

	<!-- Actions Row - CENTERED -->
	<div class="actions-row">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search by name, email, or ID..."
				bind:value={searchQuery}
				class="search-input"
			/>
		</div>
		<button class="btn-secondary" onclick={() => (showFilters = !showFilters)}>
			<IconFilter size={18} />
			Filters
		</button>
		<button class="btn-secondary" onclick={handleExport} disabled={exporting}>
			<IconDownload size={18} />
			{exporting ? 'Exporting...' : 'Export'}
		</button>
		<button class="btn-secondary" onclick={loadSubscriptions} disabled={loading}>
			<IconRefresh size={18} />
			Refresh
		</button>
	</div>

	<!-- Filters Panel -->
	{#if showFilters}
		<div class="filters-panel">
			<div class="filter-group">
				<label for="status-filter">Status</label>
				<select id="status-filter" bind:value={statusFilter} onchange={loadSubscriptions}>
					<option value="">All Statuses</option>
					<option value="active">Active</option>
					<option value="trial">Trial</option>
					<option value="on-hold">Paused</option>
					<option value="pending-cancel">Pending Cancel</option>
					<option value="cancelled">Cancelled</option>
				</select>
			</div>
			<div class="filter-group">
				<label for="interval-filter">Billing Interval</label>
				<select id="interval-filter" bind:value={intervalFilter} onchange={loadSubscriptions}>
					<option value="">All Intervals</option>
					<option value="monthly">Monthly</option>
					<option value="quarterly">Quarterly</option>
					<option value="yearly">Yearly</option>
				</select>
			</div>
			<button class="btn-clear" onclick={clearFilters}>
				<IconX size={16} />
				Clear All
			</button>
		</div>
	{/if}

	<!-- Stats Summary -->
	<div class="stats-row">
		<div class="stat-item">
			<span class="stat-label">Total</span>
			<span class="stat-value">{subscriptionCounts.total}</span>
		</div>
		<div class="stat-item active">
			<span class="stat-label">Active</span>
			<span class="stat-value">{subscriptionCounts.active}</span>
		</div>
		<div class="stat-item trial">
			<span class="stat-label">Trial</span>
			<span class="stat-value">{subscriptionCounts.trial}</span>
		</div>
		<div class="stat-item paused">
			<span class="stat-label">Paused</span>
			<span class="stat-value">{subscriptionCounts.paused}</span>
		</div>
		<div class="stat-item cancelled">
			<span class="stat-label">Cancelled</span>
			<span class="stat-value">{subscriptionCounts.cancelled}</span>
		</div>
	</div>

	<!-- Content -->
	{#if loading}
		<div class="loading-state">
			<div class="loader"></div>
			<p>Loading subscriptions...</p>
		</div>
	{:else if error}
		<div class="alert alert-error">
			<div class="error-content">
				<span>{error}</span>
				<button class="btn-retry" onclick={loadSubscriptions}>Retry</button>
			</div>
		</div>
	{:else if filteredSubscriptions.length === 0}
		<div class="empty-state">
			{#if searchQuery || statusFilter || intervalFilter}
				<IconCreditCard size={48} />
				<p>No subscriptions match your filters</p>
				<button class="btn-secondary" onclick={clearFilters}>Clear Filters</button>
			{:else}
				<IconCreditCard size={48} />
				<p>No subscriptions found</p>
				<p class="empty-hint">Subscriptions will appear here when members subscribe to plans.</p>
			{/if}
		</div>
	{:else}
		<div class="table-info">
			Showing {filteredSubscriptions.length} of {subscriptions.length} subscriptions
		</div>
		<table class="subscriptions-table">
			<thead>
				<tr>
					<th>Customer</th>
					<th>Plan</th>
					<th>Status</th>
					<th>Interval</th>
					<th>MRR</th>
					<th>Next Billing</th>
					<th>Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each filteredSubscriptions as sub}
					<tr>
						<td class="customer-cell">
							<div class="customer-name">{sub.customer?.name || 'Unknown'}</div>
							<div class="customer-email">{sub.customer?.email || '-'}</div>
						</td>
						<td>
							<span class="plan-badge">{sub.planId || 'N/A'}</span>
						</td>
						<td>
							<span class="status-badge {getStatusClass(sub.status)}">
								{getStatusLabel(sub.status)}
							</span>
						</td>
						<td>
							<span class="interval-badge">{sub.billingCycle?.interval || '-'}</span>
						</td>
						<td class="mrr-cell">
							{formatCurrency(sub.mrr || 0, sub.pricing?.currency)}
						</td>
						<td class="date-cell">
							{formatDate(sub.billingCycle?.currentPeriodEnd || '')}
						</td>
						<td class="actions-cell">
							<button
								class="btn-icon"
								onclick={() => goto(`/admin/members/${sub.customer?.id}`)}
								title="View Member"
							>
								<IconEye size={18} />
							</button>
							{#if sub.status === 'active'}
								<button
									class="btn-icon"
									onclick={() => handlePause(sub)}
									title="Pause Subscription"
								>
									<IconEdit size={18} />
								</button>
							{:else if sub.status === 'on-hold'}
								<button
									class="btn-icon success"
									onclick={() => handleResume(sub)}
									title="Resume Subscription"
								>
									<IconRefresh size={18} />
								</button>
							{/if}
							{#if sub.status !== 'cancelled'}
								<button
									class="btn-icon danger"
									onclick={() => handleCancel(sub)}
									title="Cancel Subscription"
								>
									<IconX size={18} />
								</button>
							{/if}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Page Layout - Email Templates Style
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Header - CENTERED
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Actions Row - CENTERED
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.actions-row {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
	}

	.search-input {
		width: 240px;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.875rem;
		outline: none;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Buttons
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: linear-gradient(135deg, #e6b800, #b38f00);
		color: white;
		border: none;
		padding: 0.625rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
		padding: 0.625rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
		border-color: rgba(100, 116, 139, 0.4);
	}

	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-clear {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.2);
		padding: 0.5rem 0.875rem;
		border-radius: 6px;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-clear:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	.btn-retry {
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
		padding: 0.4rem 1rem;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 600;
		white-space: nowrap;
	}

	.btn-retry:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.btn-icon.success:hover {
		background: rgba(34, 197, 94, 0.15);
		border-color: rgba(34, 197, 94, 0.3);
		color: #22c55e;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Filters Panel
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.filters-panel {
		display: flex;
		justify-content: center;
		align-items: flex-end;
		gap: 1rem;
		padding: 1rem 1.5rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.filter-group {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.filter-group label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.filter-group select {
		padding: 0.5rem 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #f1f5f9;
		font-size: 0.875rem;
		cursor: pointer;
		min-width: 150px;
	}

	.filter-group select:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Stats Row
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.stats-row {
		display: flex;
		justify-content: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		min-width: 100px;
	}

	.stat-item.active {
		border-color: rgba(34, 197, 94, 0.3);
	}

	.stat-item.trial {
		border-color: rgba(99, 102, 241, 0.3);
	}

	.stat-item.paused {
		border-color: rgba(251, 191, 36, 0.3);
	}

	.stat-item.cancelled {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.stat-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-item.active .stat-value {
		color: #22c55e;
	}

	.stat-item.trial .stat-value {
		color: #a5b4fc;
	}

	.stat-item.paused .stat-value {
		color: #fbbf24;
	}

	.stat-item.cancelled .stat-value {
		color: #f87171;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Table
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.table-info {
		font-size: 0.8125rem;
		color: #64748b;
		margin-bottom: 1rem;
		text-align: center;
	}

	.subscriptions-table {
		width: 100%;
		border-collapse: collapse;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 8px;
		overflow: hidden;
	}

	.subscriptions-table th,
	.subscriptions-table td {
		padding: 0.75rem 1rem;
		text-align: left;
		color: #e2e8f0;
	}

	.subscriptions-table th {
		background: rgba(15, 23, 42, 0.6);
		font-weight: 600;
		font-size: 0.8125rem;
		color: #94a3b8;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.subscriptions-table tbody tr {
		border-top: 1px solid rgba(148, 163, 184, 0.1);
		transition: background 0.2s;
	}

	.subscriptions-table tbody tr:hover {
		background: rgba(99, 102, 241, 0.05);
	}

	.customer-cell {
		min-width: 200px;
	}

	.customer-name {
		font-weight: 600;
		color: #f1f5f9;
	}

	.customer-email {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.mrr-cell {
		font-weight: 600;
		color: #22c55e;
	}

	.date-cell {
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.actions-cell {
		display: flex;
		gap: 0.5rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Badges
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.plan-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.interval-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: capitalize;
	}

	.status-badge {
		padding: 0.25rem 0.5rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.status-badge.status-active {
		background: rgba(34, 197, 94, 0.15);
		color: #22c55e;
	}

	.status-badge.status-trial {
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
	}

	.status-badge.status-paused {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}

	.status-badge.status-cancelled {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.status-badge.status-default {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Loading State - Centered
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #94a3b8;
	}

	.loader {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: #e6b800;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Empty State - Centered
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(30, 41, 59, 0.4);
		border-radius: 12px;
		border: 1px solid rgba(148, 163, 184, 0.1);
		color: #64748b;
	}

	.empty-state p {
		color: #94a3b8;
		margin: 1rem 0;
	}

	.empty-hint {
		font-size: 0.875rem;
		color: #64748b !important;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Error State
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		margin-bottom: 1rem;
		border: 1px solid rgba(239, 68, 68, 0.3);
	}

	.error-content {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	 * Responsive
	 * ═══════════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.actions-row {
			flex-direction: column;
		}

		.search-box {
			width: 100%;
		}

		.search-input {
			width: 100%;
		}

		.filters-panel {
			flex-direction: column;
			align-items: stretch;
		}

		.filter-group {
			width: 100%;
		}

		.filter-group select {
			width: 100%;
		}

		.stats-row {
			flex-direction: column;
		}

		.stat-item {
			width: 100%;
		}

		.subscriptions-table {
			display: block;
			overflow-x: auto;
		}

		.actions-cell {
			flex-wrap: wrap;
		}
	}
</style>

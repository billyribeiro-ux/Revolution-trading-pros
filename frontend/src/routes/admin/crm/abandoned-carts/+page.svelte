<!--
	/admin/crm/abandoned-carts - Abandoned Cart Recovery
	Apple Principal Engineer ICT 7 Grade - January 2026
	
	Features:
	- Cart tracking with stats dashboard
	- Bulk delete functionality
	- Status filtering (draft, processing, recovered, lost, opt_out)
	- Recovery URL tracking
	- Automation integration warning
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconCurrencyDollar from '@tabler/icons-svelte/icons/currency-dollar';
	import IconReceipt from '@tabler/icons-svelte/icons/receipt';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconTrendingDown from '@tabler/icons-svelte/icons/trending-down';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconUser from '@tabler/icons-svelte/icons/user';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import { crmAPI } from '$lib/api/crm';
	import type { AbandonedCart, AbandonedCartStatus, AbandonedCartStats } from '$lib/crm/types';

	let carts = $state<AbandonedCart[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let statusFilter = $state<AbandonedCartStatus | ''>('');
	let haveAutomation = $state(false);
	let selectedCarts = $state<string[]>([]);

	let stats = $state<AbandonedCartStats | null>(null);

	let dateRange = $state<string[]>([]);

	async function loadCarts() {
		isLoading = true;
		error = '';

		try {
			const [cartsResponse, statsResponse] = await Promise.all([
				crmAPI.getAbandonedCarts({
					status: statusFilter || undefined,
					search: searchQuery || undefined,
					date_range: dateRange.length ? dateRange : undefined
				}),
				crmAPI.getAbandonedCartStats(dateRange.length ? dateRange : undefined)
			]);

			carts = cartsResponse.data || [];
			haveAutomation = cartsResponse.haveAutomation;
			stats = statsResponse.widgets;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load abandoned carts';
		} finally {
			isLoading = false;
		}
	}

	async function deleteCart(id: string) {
		if (!confirm('Are you sure you want to delete this cart?')) return;

		try {
			await crmAPI.deleteAbandonedCart(id);
			await loadCarts();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete cart';
		}
	}

	async function bulkDelete() {
		if (selectedCarts.length === 0) return;
		if (!confirm(`Are you sure you want to delete ${selectedCarts.length} carts?`)) return;

		try {
			await crmAPI.bulkDeleteAbandonedCarts(selectedCarts);
			selectedCarts = [];
			await loadCarts();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete carts';
		}
	}

	function toggleCartSelection(id: string) {
		if (selectedCarts.includes(id)) {
			selectedCarts = selectedCarts.filter((c) => c !== id);
		} else {
			selectedCarts = [...selectedCarts, id];
		}
	}

	function toggleAllSelection() {
		if (selectedCarts.length === carts.length) {
			selectedCarts = [];
		} else {
			selectedCarts = carts.map((c) => c.id);
		}
	}

	function formatCurrency(amount: number, currency: string = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: currency
		}).format(amount);
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStatusColor(status: AbandonedCartStatus): string {
		const colors: Record<AbandonedCartStatus, string> = {
			draft: 'gray',
			processing: 'blue',
			recovered: 'green',
			lost: 'red',
			opt_out: 'orange'
		};
		return colors[status] || 'gray';
	}

	function getStatusLabel(status: AbandonedCartStatus): string {
		const labels: Record<AbandonedCartStatus, string> = {
			draft: 'Draft',
			processing: 'Processing',
			recovered: 'Recovered',
			lost: 'Lost',
			opt_out: 'Opted Out'
		};
		return labels[status] || status;
	}

	let filteredCarts = $derived(
		carts.filter((cart) => {
			if (statusFilter && cart.status !== statusFilter) return false;
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				return (
					cart.email.toLowerCase().includes(query) ||
					(cart.full_name && cart.full_name.toLowerCase().includes(query))
				);
			}
			return true;
		})
	);

	onMount(() => {
		loadCarts();
	});
</script>

<svelte:head>
	<title>Abandoned Carts - FluentCRM Pro</title>
</svelte:head>

<div class="abandoned-carts-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Abandoned Carts</h1>
			<p class="page-description">Track and recover abandoned shopping carts</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadCarts()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/abandoned-carts/settings" class="btn-secondary">
				<IconSettings size={18} />
				Settings
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	{#if stats}
		<div class="stats-grid">
			<div class="stat-card recovered">
				<div class="stat-icon green">
					<IconTrendingUp size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{@html stats.recovered_revenue.value}</span>
					<span class="stat-label">{stats.recovered_revenue.title}</span>
					<span class="stat-count">{stats.recovered_revenue.count} carts</span>
				</div>
			</div>
			<div class="stat-card processing">
				<div class="stat-icon blue">
					<IconShoppingCart size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{@html stats.processing_revenue.value}</span>
					<span class="stat-label">{stats.processing_revenue.title}</span>
					<span class="stat-count">{stats.processing_revenue.count} carts</span>
				</div>
			</div>
			<div class="stat-card lost">
				<div class="stat-icon red">
					<IconTrendingDown size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{@html stats.lost_revenue.value}</span>
					<span class="stat-label">{stats.lost_revenue.title}</span>
					<span class="stat-count">{stats.lost_revenue.count} carts</span>
				</div>
			</div>
			<div class="stat-card rate">
				<div class="stat-icon purple">
					<IconReceipt size={24} />
				</div>
				<div class="stat-content">
					<span class="stat-value">{stats.recovery_rate.value}</span>
					<span class="stat-label">{stats.recovery_rate.title}</span>
				</div>
			</div>
		</div>
	{/if}

	{#if !haveAutomation}
		<div class="warning-banner">
			<IconShoppingCart size={20} />
			<span
				>No active abandoned cart automation found. Create an automation to start recovering lost
				revenue.</span
			>
			<a href="/admin/crm/automations/new?trigger=fc_ab_cart_simulation_woo" class="btn-link">
				Create Automation
			</a>
		</div>
	{/if}

	<!-- Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" id="search-abandoned-carts" name="search" placeholder="Search by email or name..." bind:value={searchQuery} />
		</div>
		<select bind:value={statusFilter} class="filter-select">
			<option value="">All Statuses</option>
			<option value="draft">Draft</option>
			<option value="processing">Processing</option>
			<option value="recovered">Recovered</option>
			<option value="lost">Lost</option>
			<option value="opt_out">Opted Out</option>
		</select>
		{#if selectedCarts.length > 0}
			<button class="btn-danger" onclick={bulkDelete}>
				<IconTrash size={16} />
				Delete ({selectedCarts.length})
			</button>
		{/if}
	</div>

	<!-- Carts Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading abandoned carts...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadCarts()}>Try Again</button>
		</div>
	{:else if filteredCarts.length === 0}
		<div class="empty-state">
			<IconShoppingCart size={48} />
			<h3>No abandoned carts found</h3>
			<p>Abandoned carts will appear here when customers leave items in their cart</p>
		</div>
	{:else}
		<div class="table-container">
			<table class="carts-table">
				<thead>
					<tr>
						<th class="checkbox-col">
							<input
								type="checkbox"
								checked={selectedCarts.length === carts.length}
								onchange={toggleAllSelection}
							/>
						</th>
						<th>Customer</th>
						<th>Cart Total</th>
						<th>Items</th>
						<th>Status</th>
						<th>Created</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredCarts as cart}
						<tr>
							<td class="checkbox-col">
								<input
									type="checkbox"
									checked={selectedCarts.includes(cart.id)}
									onchange={() => toggleCartSelection(cart.id)}
								/>
							</td>
							<td>
								<div class="customer-cell">
									{#if cart.customer_avatar}
										<img src={cart.customer_avatar} alt="" class="customer-avatar" />
									{:else}
										<div class="customer-avatar-placeholder">
											<IconUser size={16} />
										</div>
									{/if}
									<div class="customer-info">
										<span class="customer-name">{cart.full_name || 'Unknown'}</span>
										<span class="customer-email">{cart.email || ''}</span>
									</div>
								</div>
							</td>
							<td>
								<span class="cart-total">{formatCurrency(cart.total, cart.currency)}</span>
							</td>
							<td>
								<span class="items-count">
									{cart.cart?.cart_contents?.length || 0} items
								</span>
							</td>
							<td>
								<span class="status-badge {getStatusColor(cart.status)}">
									{getStatusLabel(cart.status)}
								</span>
							</td>
							<td>
								<span class="date">{formatDate(cart.created_at)}</span>
							</td>
							<td>
								<div class="actions">
									{#if cart.recovery_url}
										<a
											href={cart.recovery_url}
											target="_blank"
											class="btn-icon"
											title="Recovery URL"
										>
											<IconExternalLink size={16} />
										</a>
									{/if}
									{#if cart.order_url}
										<a href={cart.order_url} target="_blank" class="btn-icon" title="View Order">
											<IconReceipt size={16} />
										</a>
									{/if}
									<button
										class="btn-icon danger"
										title="Delete"
										onclick={() => deleteCart(cart.id)}
									>
										<IconTrash size={16} />
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.abandoned-carts-page {
		max-width: 1600px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-400);
	}

	.btn-refresh :global(.spinning) {
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

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		color: #e2e8f0;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-secondary:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.4);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1024px) {
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
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-500);
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.15rem;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.stat-count {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.warning-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		background: rgba(251, 191, 36, 0.1);
		border: 1px solid rgba(251, 191, 36, 0.3);
		border-radius: 10px;
		color: #fbbf24;
		margin-bottom: 1.5rem;
	}

	.warning-banner .btn-link {
		margin-left: auto;
		color: #fbbf24;
		font-weight: 600;
		text-decoration: underline;
	}

	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
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
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.table-container {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		overflow: hidden;
	}

	.carts-table {
		width: 100%;
		border-collapse: collapse;
	}

	.carts-table th,
	.carts-table td {
		padding: 1rem;
		text-align: left;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.carts-table th {
		background: rgba(30, 41, 59, 0.5);
		color: #94a3b8;
		font-weight: 600;
		font-size: 0.8rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.carts-table tr:last-child td {
		border-bottom: none;
	}

	.checkbox-col {
		width: 40px;
	}

	.customer-cell {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.customer-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		object-fit: cover;
	}

	.customer-avatar-placeholder {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: rgba(230, 184, 0, 0.2);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--primary-400);
	}

	.customer-info {
		display: flex;
		flex-direction: column;
	}

	.customer-name {
		color: #f1f5f9;
		font-weight: 600;
	}

	.customer-email {
		color: #64748b;
		font-size: 0.85rem;
	}

	.cart-total {
		color: #4ade80;
		font-weight: 600;
	}

	.items-count {
		color: #94a3b8;
	}

	.status-badge {
		display: inline-flex;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.status-badge.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.status-badge.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.status-badge.orange {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.status-badge.gray {
		background: rgba(100, 116, 139, 0.15);
		color: #94a3b8;
	}

	.date {
		color: #64748b;
		font-size: 0.85rem;
	}

	.actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-400);
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>

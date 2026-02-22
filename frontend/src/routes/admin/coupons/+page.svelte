<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { goto } from '$app/navigation';
	import { couponsApi, AdminApiError, type Coupon } from '$lib/api/admin';
	import { IconEdit, IconTrash, IconPlus, IconRefresh, IconSearch, IconFilter } from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let loading = $state(true);
	let deleting = $state<number | null>(null);
	let coupons = $state<Coupon[]>([]);
	let error = $state('');
	let searchQuery = $state('');
	let filterStatus = $state<'all' | 'active' | 'inactive'>('all');

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived State - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredCoupons = $derived.by(() => {
		let result = coupons;

		// Filter by status
		if (filterStatus === 'active') {
			result = result.filter((c) => c.is_active);
		} else if (filterStatus === 'inactive') {
			result = result.filter((c) => !c.is_active);
		}

		// Filter by search query
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(c) =>
					c.code.toLowerCase().includes(query) || (c.type && c.type.toLowerCase().includes(query))
			);
		}

		return result;
	});

	let activeCouponsCount = $derived(coupons.filter((c) => c.is_active).length);
	let totalCouponsCount = $derived(coupons.length);

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle - Svelte 5 $effect
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		loadCoupons();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API Functions
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadCoupons() {
		loading = true;
		error = '';
		try {
			const response = await couponsApi.list();
			coupons = response.data || [];
		} catch (err) {
			if (err instanceof AdminApiError) {
				if (err.status === 401) {
					goto('/login');
					return;
				} else if (err.status === 403) {
					error = 'You are not authorized to view coupons.';
				} else {
					error = err.message;
				}
			} else {
				error = 'Error connecting to server';
			}
			logger.error('Failed to load coupons:', err);
		} finally {
			loading = false;
		}
	}

	async function deleteCoupon(id: number) {
		if (!confirm('Are you sure you want to delete this coupon? This action cannot be undone.'))
			return;

		deleting = id;
		try {
			await couponsApi.delete(id);
			coupons = coupons.filter((c) => c.id !== id);
		} catch (err) {
			if (err instanceof AdminApiError) {
				alert(`Failed to delete coupon: ${err.message}`);
			} else {
				alert('Failed to delete coupon. Please try again.');
			}
			logger.error('Delete coupon error:', err);
		} finally {
			deleting = null;
		}
	}

	async function toggleCouponStatus(coupon: Coupon) {
		try {
			await couponsApi.update(coupon.id, { is_active: !coupon.is_active });
			coupons = coupons.map((c) => (c.id === coupon.id ? { ...c, is_active: !c.is_active } : c));
		} catch (err) {
			logger.error('Toggle status error:', err);
			alert('Failed to update coupon status');
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helper Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function formatDiscountValue(coupon: Coupon): string {
		if (coupon.type === 'percentage') {
			return `${coupon.value}% off`;
		} else if (coupon.type === 'fixed') {
			return `$${coupon.value} off`;
		} else if (coupon.type === 'free_shipping') {
			return 'Free Shipping';
		}
		return `${coupon.value}`;
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'No expiry';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function isExpired(dateString: string | undefined): boolean {
		if (!dateString) return false;
		return new Date(dateString) < new Date();
	}
</script>

<div class="admin-coupons">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Page Header -->
		<header class="page-header">
			<h1>Coupons Management</h1>
			<p class="subtitle">Manage discount codes and promotional offers</p>
			<div class="header-actions">
				<button class="btn-secondary" onclick={loadCoupons} disabled={loading}>
					<IconRefresh size={18} class={loading ? 'spinning' : ''} />
					Refresh
				</button>
				<button class="btn-primary" onclick={() => goto('/admin/coupons/create')}>
					<IconPlus size={18} />
					Create Coupon
				</button>
			</div>
		</header>

		<!-- Stats Bar -->
		{#if !loading && !error}
			<div class="stats-bar">
				<div class="stat-item">
					<span class="stat-value">{totalCouponsCount}</span>
					<span class="stat-label">Total Coupons</span>
				</div>
				<div class="stat-item">
					<span class="stat-value stat-active">{activeCouponsCount}</span>
					<span class="stat-label">Active</span>
				</div>
				<div class="stat-item">
					<span class="stat-value stat-inactive">{totalCouponsCount - activeCouponsCount}</span>
					<span class="stat-label">Inactive</span>
				</div>
			</div>
		{/if}

		<!-- Filters -->
		{#if !loading && coupons.length > 0}
			<div class="filters-bar">
				<div class="search-box">
					<IconSearch size={18} />
					<input
						type="text"
						id="search-coupons"
						name="search"
						placeholder="Search by code or type..."
						bind:value={searchQuery}
					/>
				</div>
				<div class="filter-tabs">
					<button
						class="filter-tab"
						class:active={filterStatus === 'all'}
						onclick={() => (filterStatus = 'all')}
					>
						All ({totalCouponsCount})
					</button>
					<button
						class="filter-tab"
						class:active={filterStatus === 'active'}
						onclick={() => (filterStatus = 'active')}
					>
						Active ({activeCouponsCount})
					</button>
					<button
						class="filter-tab"
						class:active={filterStatus === 'inactive'}
						onclick={() => (filterStatus = 'inactive')}
					>
						Inactive ({totalCouponsCount - activeCouponsCount})
					</button>
				</div>
			</div>
		{/if}

		<!-- Content -->
		{#if loading}
			<div class="loading-state">
				<IconRefresh size={32} class="spinning" />
				<p>Loading coupons...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<p>{error}</p>
				<button class="btn-secondary" onclick={loadCoupons}>Try Again</button>
			</div>
		{:else if coupons.length === 0}
			<div class="empty-state">
				<div class="empty-icon">
					<IconFilter size={48} />
				</div>
				<h2>No coupons yet</h2>
				<p>Create your first coupon to start offering discounts to your customers.</p>
				<button class="btn-primary" onclick={() => goto('/admin/coupons/create')}>
					<IconPlus size={18} />
					Create First Coupon
				</button>
			</div>
		{:else if filteredCoupons.length === 0}
			<div class="empty-state">
				<p>No coupons match your search criteria.</p>
				<button
					class="btn-secondary"
					onclick={() => {
						searchQuery = '';
						filterStatus = 'all';
					}}
				>
					Clear Filters
				</button>
			</div>
		{:else}
			<div class="coupons-grid">
				{#each filteredCoupons as coupon (coupon.id)}
					<div class="coupon-card" class:expired={isExpired(coupon.valid_until)}>
						<div class="coupon-header">
							<div class="coupon-code">{coupon.code}</div>
							<button
								class="coupon-status"
								class:active={coupon.is_active}
								onclick={() => toggleCouponStatus(coupon)}
								title="Click to toggle status"
							>
								{coupon.is_active ? 'Active' : 'Inactive'}
							</button>
						</div>

						<div class="coupon-details">
							<span class="coupon-type">{coupon.type}</span>
							<span class="coupon-value">{formatDiscountValue(coupon)}</span>
						</div>

						<div class="coupon-meta">
							{#if coupon.usage_limit}
								<span class="meta-item">
									Uses: {coupon.usage_count}/{coupon.usage_limit}
								</span>
							{/if}
							{#if coupon.valid_until}
								<span class="meta-item" class:expired={isExpired(coupon.valid_until)}>
									Expires: {formatDate(coupon.valid_until)}
								</span>
							{/if}
							{#if coupon.minimum_amount}
								<span class="meta-item">
									Min: ${coupon.minimum_amount}
								</span>
							{/if}
						</div>

						<div class="coupon-actions">
							<button
								class="action-btn edit"
								onclick={() => goto(`/admin/coupons/edit/${coupon.id}`)}
							>
								<IconEdit size={16} />
								Edit
							</button>
							<button
								class="action-btn delete"
								onclick={() => deleteCoupon(coupon.id)}
								disabled={deleting === coupon.id}
							>
								{#if deleting === coupon.id}
									<IconRefresh size={16} class="spinning" />
								{:else}
									<IconTrash size={16} />
								{/if}
								Delete
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * COUPONS MANAGEMENT - ICT7 Principal Engineer Grade
	 * RTP Admin Color System
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.admin-coupons {
		background: linear-gradient(
			135deg,
			var(--bg-base) 0%,
			var(--bg-elevated) 50%,
			var(--bg-base) 100%
		);
		position: relative;
		overflow: hidden;
	}

	.admin-page-container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * PAGE HEADER - Centered
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.page-header {
		text-align: center;
		margin-bottom: 1.5rem;
	}

	.page-header h1 {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0 0 0.375rem 0;
	}

	.subtitle {
		color: var(--text-tertiary);
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BUTTON STYLES - Compact, intrinsic width
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
		flex-shrink: 0;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-secondary {
		background: var(--bg-surface);
		color: var(--text-primary);
		border: 1px solid var(--border-default);
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--bg-hover);
		border-color: var(--border-emphasis);
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * STATS BAR - Clean horizontal layout
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.stats-bar {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 3rem;
		padding: 1rem 1.5rem;
		background: var(--bg-elevated);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		margin-bottom: 1.5rem;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.125rem;
	}

	.stat-value {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-primary);
		line-height: 1.2;
	}

	.stat-value.stat-active {
		color: var(--success-emphasis);
	}

	.stat-value.stat-inactive {
		color: var(--text-secondary);
	}

	.stat-label {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * FILTERS BAR
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.filters-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-default);
		border-radius: 8px;
		flex: 1;
		max-width: 320px;
		color: var(--text-tertiary);
	}

	.search-box:focus-within {
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.1);
	}

	.search-box input {
		flex: 1;
		background: transparent;
		border: none;
		color: var(--text-primary);
		font-size: 0.875rem;
		outline: none;
		min-width: 0;
	}

	.search-box input::placeholder {
		color: var(--text-muted);
	}

	.filter-tabs {
		display: flex;
		gap: 0.375rem;
	}

	.filter-tab {
		padding: 0.5rem 0.875rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-default);
		border-radius: 6px;
		color: var(--text-secondary);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.filter-tab:hover {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.filter-tab.active {
		background: rgba(230, 184, 0, 0.12);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * COUPONS GRID
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.coupons-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1rem;
	}

	.coupon-card {
		background: var(--bg-elevated);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		padding: 1.25rem;
		transition: all 0.2s ease;
	}

	.coupon-card:hover {
		border-color: var(--border-default);
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
	}

	.coupon-card.expired {
		opacity: 0.6;
		border-color: var(--error-base);
	}

	.coupon-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.875rem;
	}

	.coupon-code {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--primary-400);
		font-family: 'SF Mono', Monaco, monospace;
		letter-spacing: 0.05em;
	}

	.coupon-details {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.875rem;
	}

	.coupon-type,
	.coupon-value {
		padding: 0.25rem 0.5rem;
		background: var(--bg-surface);
		border-radius: 4px;
		font-size: 0.75rem;
		color: var(--text-secondary);
		text-transform: capitalize;
	}

	.coupon-value {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-400);
		font-weight: 600;
	}

	.coupon-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 0.875rem;
	}

	.meta-item {
		font-size: 0.75rem;
		color: var(--text-tertiary);
	}

	.meta-item.expired {
		color: var(--error-emphasis);
	}

	.coupon-status {
		display: inline-block;
		padding: 0.25rem 0.625rem;
		background: var(--error-soft);
		color: var(--error-emphasis);
		border: 1px solid var(--error-base);
		border-radius: 4px;
		font-size: 0.6875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s ease;
		text-transform: uppercase;
		letter-spacing: 0.03em;
	}

	.coupon-status.active {
		background: var(--success-soft);
		color: var(--success-emphasis);
		border-color: var(--success-base);
	}

	.coupon-status:hover {
		transform: scale(1.02);
	}

	.coupon-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 0.875rem;
		border-top: 1px solid var(--border-muted);
	}

	.action-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-default);
		border-radius: 6px;
		color: var(--text-secondary);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.action-btn:hover:not(:disabled) {
		background: var(--bg-hover);
		color: var(--text-primary);
	}

	.action-btn.edit:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.action-btn.delete:hover:not(:disabled) {
		background: var(--error-soft);
		border-color: var(--error-base);
		color: var(--error-emphasis);
	}

	.action-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * STATE DISPLAYS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: var(--text-secondary);
	}

	.loading-state p,
	.error-state p,
	.empty-state p {
		margin: 0.75rem 0;
		font-size: 0.9375rem;
		max-width: 400px;
	}

	.empty-state h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0.75rem 0 0.25rem;
	}

	.empty-icon {
		color: var(--text-muted);
		margin-bottom: 0.5rem;
	}

	.error-state {
		color: var(--error-emphasis);
	}

	.error-state p {
		margin-bottom: 1.25rem;
	}

	/* Spinning animation */
	:global(.spinning) {
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

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.admin-page-container {
			padding: 1.25rem;
		}

		.page-header h1 {
			font-size: 1.25rem;
		}

		.header-actions {
			gap: 0.5rem;
		}

		.btn-primary,
		.btn-secondary {
			padding: 0.5rem 0.875rem;
			font-size: 0.8125rem;
		}

		.stats-bar {
			gap: 1.5rem;
			padding: 0.875rem 1rem;
		}

		.stat-value {
			font-size: 1.125rem;
		}

		.filters-bar {
			flex-direction: column;
			align-items: stretch;
		}

		.search-box {
			max-width: none;
		}

		.filter-tabs {
			justify-content: center;
		}

		.coupons-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 480px) {
		.admin-page-container {
			padding: 1rem;
		}

		.stats-bar {
			gap: 1rem;
		}

		.filter-tabs {
			flex-wrap: wrap;
		}

		.filter-tab {
			flex: 1;
			min-width: 80px;
			text-align: center;
		}
	}
</style>

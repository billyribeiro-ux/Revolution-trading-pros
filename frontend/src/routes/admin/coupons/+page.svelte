<script lang="ts">
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
					c.code.toLowerCase().includes(query) ||
					(c.type && c.type.toLowerCase().includes(query))
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
			console.error('Failed to load coupons:', err);
		} finally {
			loading = false;
		}
	}

	async function deleteCoupon(id: number) {
		if (!confirm('Are you sure you want to delete this coupon? This action cannot be undone.')) return;

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
			console.error('Delete coupon error:', err);
		} finally {
			deleting = null;
		}
	}

	async function toggleCouponStatus(coupon: Coupon) {
		try {
			await couponsApi.update(coupon.id, { is_active: !coupon.is_active });
			coupons = coupons.map((c) =>
				c.id === coupon.id ? { ...c, is_active: !c.is_active } : c
			);
		} catch (err) {
			console.error('Toggle status error:', err);
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

<div class="page">
	<div class="admin-page-container">
	<!-- Page Header -->
	<div class="page-header">
		<h1>Coupons Management</h1>
		<p class="subtitle">Manage discount codes and promotional offers</p>
	</div>

	<!-- Actions Row -->
	<div class="actions-row">
		<button class="btn-secondary" onclick={loadCoupons} disabled={loading}>
			<IconRefresh size={18} class={loading ? 'spinning' : ''} />
			Refresh
		</button>
		<button class="btn-primary" onclick={() => goto('/admin/coupons/create')}>
			<IconPlus size={18} />
			Create Coupon
		</button>
	</div>

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
			<button class="btn-secondary" onclick={() => { searchQuery = ''; filterStatus = 'all'; }}>
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
	</div><!-- End admin-page-container -->
</div>

<style>
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header Styles - CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	/* Actions Row - CENTERED */
	.actions-row {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		margin-bottom: 2rem;
	}

	/* Button Styles */
	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 6px;
		font-weight: 600;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(230, 184, 0, 0.35);
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #cbd5e1;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(100, 116, 139, 0.3);
	}

	button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Stats Bar */
	.stats-bar {
		display: flex;
		justify-content: center;
		gap: 2rem;
		padding: 1.25rem 1.5rem;
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		margin-bottom: 1.5rem;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-value.stat-active {
		color: #34d399;
	}

	.stat-value.stat-inactive {
		color: #94a3b8;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #94a3b8;
	}

	/* Filters Bar */
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
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		flex: 1;
		max-width: 400px;
		color: #94a3b8;
	}

	.search-box:focus-within {
		border-color: rgba(99, 102, 241, 0.5);
		box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
	}

	.search-box input {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 0.95rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-tabs {
		display: flex;
		gap: 0.5rem;
	}

	.filter-tab {
		padding: 0.625rem 1rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.filter-tab:hover {
		background: rgba(148, 163, 184, 0.15);
	}

	.filter-tab.active {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.4);
		color: #818cf8;
	}

	/* Coupons Grid */
	.coupons-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.coupon-card {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		padding: 1.5rem;
		transition: all 0.2s;
	}

	.coupon-card:hover {
		border-color: rgba(230, 184, 0, 0.4);
		transform: translateY(-2px);
	}

	.coupon-card.expired {
		opacity: 0.7;
		border-color: rgba(239, 68, 68, 0.3);
	}

	.coupon-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.coupon-code {
		font-size: 1.35rem;
		font-weight: 700;
		color: #818cf8;
		font-family: monospace;
		letter-spacing: 0.05em;
	}

	.coupon-details {
		display: flex;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.coupon-type,
	.coupon-value {
		padding: 0.25rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 6px;
		font-size: 0.875rem;
		color: #cbd5e1;
		text-transform: capitalize;
	}

	.coupon-value {
		background: rgba(99, 102, 241, 0.1);
		color: #818cf8;
		font-weight: 600;
	}

	.coupon-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.meta-item {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.meta-item.expired {
		color: #f87171;
	}

	.coupon-status {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.coupon-status.active {
		background: rgba(16, 185, 129, 0.1);
		color: #34d399;
		border-color: rgba(16, 185, 129, 0.3);
	}

	.coupon-status:hover {
		transform: scale(1.05);
	}

	.coupon-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.action-btn {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 6px;
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover:not(:disabled) {
		background: rgba(148, 163, 184, 0.2);
		border-color: rgba(148, 163, 184, 0.3);
	}

	.action-btn.edit:hover:not(:disabled) {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		color: #818cf8;
	}

	.action-btn.delete:hover:not(:disabled) {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.action-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* State displays */
	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #94a3b8;
	}

	.loading-state p,
	.error-state p,
	.empty-state p {
		margin: 1rem 0;
		font-size: 1rem;
	}

	.empty-state h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	.empty-icon {
		color: #64748b;
		margin-bottom: 0.5rem;
	}

	.error-state {
		color: #f87171;
	}

	.error-state p {
		margin-bottom: 1.5rem;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* Responsive */
	@media (max-width: 768px) {
		.page {
			padding: 1rem;
		}

		.actions-row {
			flex-wrap: wrap;
		}

		.stats-bar {
			flex-wrap: wrap;
			justify-content: center;
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
</style>

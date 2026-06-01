<script lang="ts">
	/**
	 * Admin coupons list page.
	 *
	 * R27-C extraction (2026-05-20): broke the 864-LOC page into five
	 * focused presentational components under `_components/`:
	 *   - CouponsPageHeader   — title + refresh/create buttons
	 *   - CouponsStatsBar     — total / active / inactive counters
	 *   - CouponsFiltersBar   — search input + status filter tabs
	 *   - CouponCard          — single card in the coupons-grid
	 *   - CouponsEmptyStates  — loading / error / empty / no-match states
	 *
	 * The page now owns ONLY: data loading, filter state, delete-confirm
	 * orchestration, and the optimistic toggle-status update. Field-alias
	 * fallback (discount_type/type, expires_at/valid_until, etc.) lives in
	 * `CouponCard` so this loop body stays trivial.
	 */
	import { goto } from '$app/navigation';
	import { couponsApi, AdminApiError } from '$lib/api/admin';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import { toastStore } from '$lib/stores/toast.svelte';
	import { logger } from '$lib/utils/logger';

	import CouponCard from './_components/CouponCard.svelte';
	import CouponsEmptyStates from './_components/CouponsEmptyStates.svelte';
	import CouponsFiltersBar from './_components/CouponsFiltersBar.svelte';
	import CouponsPageHeader from './_components/CouponsPageHeader.svelte';
	import CouponsStatsBar from './_components/CouponsStatsBar.svelte';
	import type { Coupon, FilterStatus } from './_components/types';

	// ═══════════════════════════════════════════════════════════════════════════
	// State Management - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let loading = $state(true);
	let deleting = $state<number | null>(null);
	let coupons = $state<Coupon[]>([]);
	let error = $state('');
	let searchQuery = $state('');
	let filterStatus = $state<FilterStatus>('all');
	let showDeleteConfirm = $state(false);
	let pendingDeleteId = $state<number | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// Derived State - Svelte 5 Runes
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredCoupons = $derived.by(() => {
		let result = coupons;

		if (filterStatus === 'active') {
			result = result.filter((c) => c.is_active);
		} else if (filterStatus === 'inactive') {
			result = result.filter((c) => !c.is_active);
		}

		// FIX-2026-04-26 (P2-10): backend returns `discount_type`, not `type`.
		// Read `discount_type` first and fall back to legacy `type`.
		if (searchQuery.trim()) {
			const query = searchQuery.toLowerCase();
			result = result.filter((c) => {
				const dtype = c.discount_type ?? c.type;
				return c.code.toLowerCase().includes(query) || !!dtype?.toLowerCase().includes(query);
			});
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

	function requestDeleteCoupon(id: number) {
		pendingDeleteId = id;
		showDeleteConfirm = true;
	}

	function cancelDeleteCoupon() {
		showDeleteConfirm = false;
		pendingDeleteId = null;
	}

	async function handleConfirmDelete() {
		if (pendingDeleteId === null) return;
		const id = pendingDeleteId;
		showDeleteConfirm = false;
		pendingDeleteId = null;

		deleting = id;
		try {
			await couponsApi.delete(id);
			coupons = coupons.filter((c) => c.id !== id);
			toastStore.success('Coupon deleted successfully');
		} catch (err) {
			if (err instanceof AdminApiError) {
				toastStore.error(`Failed to delete coupon: ${err.message}`);
			} else {
				toastStore.error('Failed to delete coupon. Please try again.');
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
			toastStore.error('Failed to update coupon status');
		}
	}

	function clearFilters() {
		searchQuery = '';
		filterStatus = 'all';
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
		<CouponsPageHeader
			{loading}
			onRefresh={loadCoupons}
			onCreate={() => goto('/admin/coupons/create')}
		/>

		{#if !loading && !error}
			<CouponsStatsBar total={totalCouponsCount} active={activeCouponsCount} />
		{/if}

		{#if !loading && coupons.length > 0}
			<CouponsFiltersBar
				bind:searchQuery
				bind:filterStatus
				total={totalCouponsCount}
				active={activeCouponsCount}
			/>
		{/if}

		{#if loading}
			<CouponsEmptyStates kind="loading" />
		{:else if error}
			<CouponsEmptyStates kind="error" message={error} onRetry={loadCoupons} />
		{:else if coupons.length === 0}
			<CouponsEmptyStates kind="empty" onCreate={() => goto('/admin/coupons/create')} />
		{:else if filteredCoupons.length === 0}
			<CouponsEmptyStates kind="filtered-empty" onClearFilters={clearFilters} />
		{:else}
			<div class="coupons-grid">
				{#each filteredCoupons as coupon (coupon.id)}
					<CouponCard
						{coupon}
						deleting={deleting === coupon.id}
						onToggleStatus={toggleCouponStatus}
						onEdit={(id) => goto(`/admin/coupons/edit/${id}`)}
						onRequestDelete={requestDeleteCoupon}
					/>
				{/each}
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<ConfirmationModal
	isOpen={showDeleteConfirm}
	title="Delete Coupon"
	message="Are you sure you want to delete this coupon? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onConfirm={handleConfirmDelete}
	onCancel={cancelDeleteCoupon}
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * COUPONS MANAGEMENT - page-level chrome only.
	 * Section-specific CSS lives in the extracted _components/* files.
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

	.coupons-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 1rem;
	}

	/* Spinning animation - kept :global so child components inherit */
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

	@media (max-width: 767.98px) {
		.admin-page-container {
			padding: 1.25rem;
		}

		.coupons-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 479.98px) {
		.admin-page-container {
			padding: 1rem;
		}
	}
</style>

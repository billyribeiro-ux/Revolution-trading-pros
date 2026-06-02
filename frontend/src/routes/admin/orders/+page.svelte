<script lang="ts">
	import { onMount } from 'svelte';
	/**
	 * Admin Orders Management Page
	 * ICT 7 Fix: Complete admin orders dashboard
	 * Apple Principal Engineer Grade - February 2026
	 *
	 * R13-C (2026-05-20): Extracted 7 leaf components into _components/
	 * to reduce parent surface from ~1260 LOC to a coordinator file.
	 */

	import type { ComponentProps } from 'svelte';
	import { browser } from '$app/environment';
	import { toastStore } from '$lib/stores/toast.svelte';

	import ErrorBanner from './_components/ErrorBanner.svelte';
	import PageHeader from './_components/PageHeader.svelte';
	import StatsGrid from './_components/StatsGrid.svelte';
	import OrdersToolbar from './_components/OrdersToolbar.svelte';
	import FiltersPanel from './_components/FiltersPanel.svelte';
	import OrdersTable from './_components/OrdersTable.svelte';
	import OrderDetailModal from './_components/OrderDetailModal.svelte';

	// Types
	interface Order {
		id: number;
		order_number: string;
		status: string;
		total: number;
		currency: string;
		user_email: string;
		user_name: string | null;
		payment_provider: string | null;
		item_count: number;
		created_at: string;
		completed_at: string | null;
	}

	interface OrderStats {
		total_orders: number;
		completed_orders: number;
		pending_orders: number;
		refunded_orders: number;
		total_revenue: number;
		revenue_this_month: number;
		average_order_value: number;
	}

	interface Pagination {
		page: number;
		per_page: number;
		total: number;
		total_pages: number;
	}

	// State
	let orders = $state<Order[]>([]);
	let stats = $state<OrderStats | null>(null);
	let pagination = $state<Pagination | null>(null);
	let loading = $state(true);
	let searchQuery = $state('');
	let statusFilter = $state('');
	let showFilters = $state(false);
	let selectedOrder = $state<Order | null>(null);
	let showDetailModal = $state(false);
	let orderDetail = $state<ComponentProps<typeof OrderDetailModal>['orderDetail']>(null);
	let loadingDetail = $state(false);
	let error = $state('');

	// Fetch orders from API
	async function loadOrders(page = 1) {
		loading = true;
		error = '';
		try {
			const params = new URLSearchParams();
			params.set('page', page.toString());
			params.set('per_page', '25');
			if (statusFilter) params.set('status', statusFilter);
			if (searchQuery) params.set('search', searchQuery);

			const response = await fetch(`/api/admin/orders?${params}`);
			if (!response.ok) {
				throw new Error('Failed to load orders');
			}
			const data = await response.json();
			orders = data.data || [];
			stats = data.stats || null;
			pagination = data.pagination || null;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load orders';
			console.error('Error loading orders:', err);
		} finally {
			loading = false;
		}
	}

	// Fetch order details
	async function loadOrderDetail(orderId: number) {
		loadingDetail = true;
		try {
			const response = await fetch(`/api/admin/orders/${orderId}`);
			if (!response.ok) throw new Error('Failed to load order details');
			const data = await response.json();
			orderDetail = data.data;
		} catch (_err) {
			toastStore.error('Failed to load order details');
		} finally {
			loadingDetail = false;
		}
	}

	// Handle search
	function handleSearch() {
		loadOrders(1);
	}

	// Handle filter change
	function handleStatusFilter(status: string) {
		statusFilter = status;
		loadOrders(1);
	}

	// Open order detail
	function openOrderDetail(order: Order) {
		selectedOrder = order;
		showDetailModal = true;
		loadOrderDetail(order.id);
	}

	// Format currency
	function formatCurrency(amount: number, currency = 'USD'): string {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency
		}).format(amount);
	}

	// Format date
	function formatDate(dateString: string): string {
		if (!dateString) return 'N/A';
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	// Get status color
	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed':
				return 'status-completed';
			case 'pending':
				return 'status-pending';
			case 'refunded':
			case 'partial_refund':
				return 'status-refunded';
			case 'failed':
				return 'status-failed';
			default:
				return 'status-default';
		}
	}

	// Export orders
	async function handleExport() {
		try {
			const params = new URLSearchParams();
			if (statusFilter) params.set('status', statusFilter);
			if (searchQuery) params.set('search', searchQuery);
			params.set('format', 'csv');

			const response = await fetch(`/api/admin/orders/export?${params}`);
			if (!response.ok) throw new Error('Export failed');

			const blob = await response.blob();
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `orders-export-${new Date().toISOString().split('T')[0]}.csv`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
			toastStore.success('Orders exported successfully');
		} catch {
			toastStore.error('Failed to export orders');
		}
	}

	function clearAllFilters() {
		statusFilter = '';
		searchQuery = '';
		loadOrders(1);
	}

	// Svelte 5: Initialize on mount
	onMount(() => {
		if (browser) loadOrders();
	});
</script>

<svelte:head>
	<title>Orders Management | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-orders">
	<!-- Background Effects -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
	</div>

	<div class="admin-page-container">
		{#if error}
			<ErrorBanner {error} onretry={() => loadOrders()} />
		{/if}

		<PageHeader onrefresh={() => loadOrders()} onexport={handleExport} />

		{#if stats}
			<StatsGrid {stats} {formatCurrency} />
		{/if}

		<OrdersToolbar
			bind:searchQuery
			{showFilters}
			onsearch={handleSearch}
			ontoggleFilters={() => (showFilters = !showFilters)}
		/>

		{#if showFilters}
			<FiltersPanel bind:statusFilter onchange={handleStatusFilter} onclear={clearAllFilters} />
		{/if}

		<OrdersTable
			{orders}
			{loading}
			{pagination}
			{formatCurrency}
			{formatDate}
			{getStatusColor}
			onpage={(p) => loadOrders(p)}
			onopenDetail={openOrderDetail}
		/>
	</div>
</div>

{#if showDetailModal && selectedOrder}
	<OrderDetailModal
		{selectedOrder}
		{orderDetail}
		{loadingDetail}
		{formatCurrency}
		{formatDate}
		{getStatusColor}
		onclose={() => (showDetailModal = false)}
	/>
{/if}

<style>
	/* Admin Orders Page Styles - ICT 7
	 * Parent retains: page-level background/blobs + container.
	 * All component-specific styles moved with their components.
	 */
	.admin-orders {
		min-height: calc(100vh - 70px - 4rem);
		background: var(--admin-bg, linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%));
		color: white;
		position: relative;
		overflow: hidden;
		margin: -2rem;
		padding: 2rem;
	}

	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		top: -200px;
		right: -200px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		animation: float 20s ease-in-out infinite;
	}

	.bg-blob-2 {
		width: 500px;
		height: 500px;
		bottom: -150px;
		left: -150px;
		background: linear-gradient(135deg, var(--primary-600), #1e293b);
		animation: float 25s ease-in-out infinite reverse;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.05);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.95);
		}
	}

	@media (max-width: 767.98px) {
		.admin-page-container {
			padding: 1rem;
		}
	}
</style>

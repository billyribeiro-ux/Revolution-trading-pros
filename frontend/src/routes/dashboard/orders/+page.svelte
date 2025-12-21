<script lang="ts">
	/**
	 * Dashboard - My Orders Page - Simpler Trading EXACT
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/orders
	 * Shows user's order history with Order, Date, Actions columns
	 *
	 * @version 5.0.0 (Simpler Trading Exact / December 2025)
	 */

	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { authStore, isAuthenticated } from '$lib/stores/auth';
	import { IconDotsVertical, IconEye } from '$lib/icons';
	import Footer from '$lib/components/sections/Footer.svelte';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let openDropdownId = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// EFFECTS
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && !$isAuthenticated && !$authStore.isInitializing) {
			goto('/login?redirect=/dashboard/orders', { replaceState: true });
		}
	});

	// Close dropdown when clicking outside
	$effect(() => {
		if (browser && openDropdownId) {
			const handleClickOutside = (e: MouseEvent) => {
				const target = e.target as HTMLElement;
				if (!target.closest('.dropdown')) {
					openDropdownId = null;
				}
			};
			document.addEventListener('click', handleClickOutside);
			return () => document.removeEventListener('click', handleClickOutside);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA (would come from API)
	// ═══════════════════════════════════════════════════════════════════════════

	interface Order {
		id: string;
		orderNumber: string;
		date: string;
		viewUrl: string;
	}

	// Sample orders data
	const orders: Order[] = [
		{
			id: '2176654',
			orderNumber: '#2176654',
			date: 'December 3, 2025',
			viewUrl: '/dashboard/orders/2176654'
		},
		{
			id: '2173014',
			orderNumber: '#2173014',
			date: 'November 17, 2025',
			viewUrl: '/dashboard/orders/2173014'
		},
		{
			id: '2132732',
			orderNumber: '#2132732',
			date: 'July 18, 2025',
			viewUrl: '/dashboard/orders/2132732'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleDropdown(orderId: string, e: MouseEvent): void {
		e.stopPropagation();
		openDropdownId = openDropdownId === orderId ? null : orderId;
	}
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>My Orders | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<h1 class="dashboard__page-title">My Account</h1>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     MAIN CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Orders Table -->
		<div class="orders-table-wrapper">
			<table class="orders-table">
				<thead>
					<tr>
						<th class="col-order">Order</th>
						<th class="col-date">Date</th>
						<th class="col-actions">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each orders as order (order.id)}
						<tr>
							<td class="col-order">
								<a href={order.viewUrl} class="order-link">
									{order.orderNumber}
								</a>
							</td>
							<td class="col-date">
								<time>{order.date}</time>
							</td>
							<td class="col-actions">
								<div class="dropdown">
									<button
										type="button"
										class="dropdown-toggle"
										onclick={(e) => toggleDropdown(order.id, e)}
										aria-expanded={openDropdownId === order.id}
										aria-haspopup="true"
									>
										<IconDotsVertical size={18} />
									</button>
									{#if openDropdownId === order.id}
										<div class="dropdown-menu">
											<a href={order.viewUrl} class="dropdown-item">
												<IconEye size={14} />
												View
											</a>
										</div>
									{/if}
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Empty State (when no orders) -->
		{#if orders.length === 0}
			<div class="empty-state">
				<p>No orders found.</p>
			</div>
		{/if}
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     FOOTER
     ═══════════════════════════════════════════════════════════════════════════ -->

<Footer />

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - Simpler Trading EXACT
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #e9ebed;
		padding: 20px 30px;
	}

	.dashboard__page-title {
		color: #333;
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 36px;
		font-weight: 700;
		margin: 0;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   CONTENT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: 400px;
	}

	.dashboard__content-main {
		max-width: 100%;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ORDERS TABLE - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.orders-table-wrapper {
		border: 1px solid #e9ebed;
		border-radius: 4px;
		overflow: hidden;
	}

	.orders-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.orders-table thead {
		background: #f8f9fa;
	}

	.orders-table th {
		padding: 12px 16px;
		text-align: left;
		font-weight: 600;
		color: #333;
		border-bottom: 1px solid #e9ebed;
	}

	.orders-table td {
		padding: 16px;
		border-bottom: 1px solid #e9ebed;
		color: #333;
	}

	.orders-table tr:last-child td {
		border-bottom: none;
	}

	/* Column widths */
	.col-order {
		width: 30%;
	}

	.col-date {
		width: 50%;
	}

	.col-actions {
		width: 20%;
		text-align: right !important;
	}

	/* Order link */
	.order-link {
		color: #1e73be;
		text-decoration: none;
		font-weight: 400;
	}

	.order-link:hover {
		text-decoration: underline;
	}

	/* Date */
	.orders-table time {
		color: #333;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DROPDOWN MENU - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-toggle {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: #fff;
		border: 1px solid #ddd;
		border-radius: 4px;
		color: #666;
		cursor: pointer;
		transition: all 0.15s ease;
		margin-left: auto;
	}

	.dropdown-toggle:hover {
		background: #f5f5f5;
		color: #333;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 4px;
		background: #fff;
		border: 1px solid #ddd;
		border-radius: 4px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
		min-width: 120px;
		z-index: 100;
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		color: #333;
		text-decoration: none;
		font-size: 13px;
		transition: background 0.15s ease;
	}

	.dropdown-item:hover {
		background: #f5f5f5;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		padding: 40px;
		text-align: center;
		color: #666;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 768px) {
		.dashboard__header {
			padding: 16px 20px;
		}

		.dashboard__page-title {
			font-size: 28px;
		}

		.dashboard__content {
			padding: 20px;
		}

		.orders-table th,
		.orders-table td {
			padding: 12px;
		}

		.col-order {
			width: 35%;
		}

		.col-date {
			width: 45%;
		}

		.col-actions {
			width: 20%;
		}
	}

	@media screen and (max-width: 480px) {
		.orders-table {
			font-size: 13px;
		}

		.orders-table th,
		.orders-table td {
			padding: 10px 8px;
		}
	}
</style>

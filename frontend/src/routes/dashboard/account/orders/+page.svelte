<script lang="ts">
	interface Order {
		id: number;
		number: string;
		date: string;
		status: string;
		total: string;
	}

	interface PageData {
		orders: Order[];
	}

	let { data }: { data: PageData } = $props();

	const orders = $derived(data.orders || []);

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>My Orders - Revolution Trading Pros</title>
	<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.4/css/all.min.css" />
</svelte:head>

<!-- My Account Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- Order Box - Exact Simpler Trading Structure -->
<div class="fl-module fl-module-rich-text fl-node-59793676759ab dashboard-nav" data-node="59793676759ab">
	<div class="fl-module-content fl-node-content">
		<div class="fl-rich-text">
			<div class="woocommerce">
				<div class="woocommerce-MyAccount-content">
					<div class="woocommerce-notices-wrapper"></div>
					<h2 class="section-title">My Orders</h2>

					{#if orders.length === 0}
						<div class="woocommerce-message woocommerce-info">
							No order has been made yet.
						</div>
					{:else}
						<table class="table">
							<thead>
								<tr>
									<th class="col-xs-2">Order</th><th class="col-xs-3">Date</th><th class="col-xs-2 text-right">Actions</th>
								</tr>
							</thead>
							<tbody class="u--font-size-sm">
								{#each orders as order (order.id)}
									<tr>
										<td class="col-xs-2">
											<a href="/dashboard/account/view-order/{order.id}">
												#{order.number}
											</a>
										</td>
										<td class="col-xs-3">
											<time datetime={order.date}>{formatDate(order.date)}</time>
										</td>
										<td class="col-xs-2 text-right table__actions">
											<div class="dropdown">
												<a href="#" class="btn btn-xs btn-white table__more-actions" id="dLabel-{order.id}" data-bs-toggle="dropdown" aria-expanded="false">
													<span class="st-icon-ellipsis-h icon--md"></span>
												</a>
												<nav class="dropdown-menu" data-append-to-body="1" aria-labelledby="dLabel-{order.id}">
													<div class="dropdown-menu__content">
														<ul class="dropdown-menu__menu dropdown-menu__menu--compact">
															<li>
																<a href="/dashboard/account/view-order/{order.id}">
																	<i class="fa fa-eye icon--sm"></i>View
																</a>
															</li>
														</ul>
													</div>
												</nav>
											</div>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					{/if}
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   PIXEL-PERFECT ORDERS PAGE - MATCHING SIMPLER TRADING REFERENCE EXACTLY
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 30px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.dashboard__header-left {
		flex: 1;
	}

	.dashboard__page-title {
		font-family: 'Open Sans', sans-serif;
		font-size: 28px;
		font-weight: 400;
		font-style: italic;
		color: #333333;
		margin: 0;
		line-height: 1.2;
	}

	/* Beaver Builder Module Structure */
	.fl-module {
		margin-bottom: 0;
	}

	.fl-module-content {
		position: relative;
	}

	.fl-rich-text {
		font-family: 'Open Sans', sans-serif;
		line-height: 1.6;
	}

	/* WooCommerce Container */
	.woocommerce {
		background: #fff;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		padding: 20px;
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
		line-height: 1.6;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.woocommerce-notices-wrapper {
		margin: 0;
	}

	/* Section Title */
	.section-title {
		font-family: 'Open Sans', sans-serif;
		font-size: 18px;
		font-weight: 600;
		color: #333333;
		margin: 0 0 15px 0;
		padding: 0;
		line-height: 1.4;
	}

	/* Orders Table - Pixel Perfect */
	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		font-family: 'Open Sans', sans-serif;
		margin: 0;
		border: none;
	}

	.table thead {
		background: transparent;
		border-bottom: 2px solid #e5e5e5;
	}

	.table thead tr {
		border: none;
	}

	.table thead th {
		padding: 10px 12px;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #666666;
		text-align: left;
		border: none;
		background: transparent;
	}

	.table thead th.text-right {
		text-align: right;
	}

	/* Table Body */
	.table tbody tr {
		border-bottom: 1px solid #e5e5e5;
	}

	.table tbody tr:last-child {
		border-bottom: 1px solid #e5e5e5;
	}

	.table tbody td {
		padding: 15px 12px;
		font-size: 14px;
		color: #333333;
		vertical-align: middle;
		border: none;
	}

	/* Order Number Link */
	.table tbody td a {
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
	}

	.table tbody td a:hover {
		color: #076787;
		text-decoration: underline;
	}

	/* Date */
	.table tbody td time {
		color: #333333;
		font-size: 14px;
		font-weight: 400;
	}

	/* Actions Column */
	.table__actions {
		text-align: right;
	}

	/* Dropdown Button - Ellipsis */
	.btn {
		display: inline-block;
		font-weight: 400;
		text-align: center;
		white-space: nowrap;
		vertical-align: middle;
		user-select: none;
		border: 1px solid transparent;
		padding: 6px 12px;
		font-size: 14px;
		line-height: 1.5;
		border-radius: 4px;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
	}

	.btn-xs {
		padding: 4px 8px;
		font-size: 12px;
		line-height: 1.5;
		border-radius: 3px;
	}

	.btn-white {
		background-color: #ffffff;
		border-color: #d4d4d4;
		color: #666666;
	}

	.btn-white:hover {
		background-color: #f5f5f5;
		border-color: #c4c4c4;
		color: #333333;
	}

	.table__more-actions {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 28px;
		padding: 0 8px;
	}

	/* Ellipsis Icon - Using ::before like reference */
	.st-icon-ellipsis-h {
		display: inline-block;
	}

	.st-icon-ellipsis-h::before {
		content: '\2022\2022\2022'; /* Three dots: ••• */
		font-size: 16px;
		color: #666666;
		letter-spacing: 2px;
	}

	.icon--md {
		font-size: 16px;
	}

	/* Dropdown Container */
	.dropdown {
		position: relative;
		display: inline-block;
	}

	/* Dropdown Menu */
	.dropdown-menu {
		position: absolute;
		right: 0;
		top: 100%;
		margin-top: 2px;
		min-width: 120px;
		background: #fff;
		border: 1px solid #d4d4d4;
		border-radius: 4px;
		box-shadow: 0 6px 12px rgba(0, 0, 0, 0.175);
		opacity: 0;
		visibility: hidden;
		transform: translateY(-5px);
		transition: all 0.15s ease;
		z-index: 1000;
		padding: 0;
	}

	.dropdown:hover .dropdown-menu,
	.dropdown:focus-within .dropdown-menu {
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
	}

	.dropdown-menu__content {
		padding: 5px 0;
	}

	.dropdown-menu__menu {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dropdown-menu__menu--compact {
		padding: 0;
	}

	.dropdown-menu__menu li {
		margin: 0;
	}

	.dropdown-menu__menu li a {
		display: flex;
		align-items: center;
		padding: 8px 15px;
		color: #333333;
		text-decoration: none;
		font-size: 14px;
		font-weight: 400;
		white-space: nowrap;
	}

	.dropdown-menu__menu li a:hover {
		background: #f5f5f5;
		color: #0984ae;
		text-decoration: none;
	}

	.dropdown-menu__menu li a i {
		margin-right: 10px;
		width: 14px;
		text-align: center;
		color: inherit;
		font-size: 14px;
	}

	.icon--sm {
		font-size: 14px;
	}

	/* Info Message - No Orders */
	.woocommerce-message,
	.woocommerce-info {
		padding: 12px 20px;
		background: #f7f6f7;
		border-top: 3px solid #0984ae;
		margin: 0 0 20px 0;
		color: #515151;
		font-size: 14px;
		line-height: 1.6;
		list-style: none;
	}

	.woocommerce-message::before {
		content: none;
	}

	/* Column Widths */
	.col-xs-2 {
		width: 20%;
	}

	.col-xs-3 {
		width: 30%;
	}

	.text-right {
		text-align: right;
	}

	.u--font-size-sm {
		font-size: 14px;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.table {
			display: block;
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		.table thead th,
		.table tbody td {
			padding: 10px 8px;
			font-size: 13px;
		}

		.col-xs-2,
		.col-xs-3 {
			width: auto;
		}
	}
</style>

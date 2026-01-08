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
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">My Account</h1>
	</div>
</header>

<!-- Dashboard Content -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<div class="fl-builder-content fl-builder-content-33 fl-builder-content-primary fl-builder-global-templates-locked" data-post-id="33">
				<div class="fl-row fl-row-fixed-width fl-row-bg-color fl-node-59793676724ad" data-node="59793676724ad">
					<div class="fl-row-content-wrap">
						<div class="fl-row-content fl-row-fixed-width fl-node-content">
							<div class="fl-col-group fl-node-597936767334e" data-node="597936767334e">
								<div class="fl-col fl-node-5979367673419" data-node="5979367673419">
									<div class="fl-col-content fl-node-content">
										<div class="fl-module fl-module-rich-text fl-node-59793676759ab dashboard-nav" data-node="59793676759ab">
											<div class="fl-module-content fl-node-content">
												<div class="fl-rich-text">
													<div class="woocommerce">
														<div class="woocommerce-MyAccount-content">
															<div class="woocommerce-notices-wrapper"></div>
															<h2 class="section-title">My Orders</h2>

															{#if orders.length === 0}
																<div class="woocommerce-message woocommerce-message--info woocommerce-Message woocommerce-Message--info woocommerce-info">
																	<p>No orders have been made yet.</p>
																</div>
															{:else}
																<table class="table">
																	<thead>
																		<tr>
																			<th class="col-xs-2">Order</th>
																			<th class="col-xs-3">Date</th>
																			<th class="col-xs-2 text-right">Actions</th>
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
																					<time datetime={order.date}>
																						{formatDate(order.date)}
																					</time>
																				</td>
																				<td class="col-xs-2 text-right table__actions">
																					<div class="dropdown">
																						<a href="#" class="btn btn-xs btn-white table__more-actions" id="dLabel" data-bs-toggle="dropdown" aria-expanded="false">
																							<span class="st-icon-ellipsis-h icon--md"></span>
																						</a>
																						<nav class="dropdown-menu" data-append-to-body="1" aria-labelledby="dLabel">
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
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
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
		font-family: 'Montserrat', sans-serif;
		font-size: 24px;
		font-weight: 600;
		color: #0a2335;
		margin: 0;
		line-height: 1.2;
	}

	/* Dashboard Content Wrapper */
	.dashboard__content {
		background: #f5f5f5;
		min-height: calc(100vh - 60px);
		padding: 30px;
	}

	.dashboard__content-main {
		max-width: 1200px;
		margin: 0 auto;
	}

	.dashboard__content-section {
		padding: 0;
	}

	/* Beaver Builder Structure */
	.fl-builder-content {
		width: 100%;
		font-family: 'Montserrat', sans-serif;
	}

	.fl-row {
		width: 100%;
		position: relative;
	}

	.fl-row-bg-color {
		background-color: #ffffff;
	}

	.fl-row-content-wrap {
		position: relative;
	}

	.fl-row-content {
		margin-left: auto;
		margin-right: auto;
	}

	.fl-row-fixed-width {
		max-width: 1100px;
	}

	.fl-col-group {
		display: flex;
		flex-wrap: wrap;
	}

	.fl-col {
		flex: 1;
		min-width: 0;
	}

	.fl-col-content {
		padding: 0;
	}

	.fl-node-content {
		position: relative;
	}

	.fl-module {
		margin-bottom: 0;
	}

	.fl-module-content {
		position: relative;
	}

	.fl-rich-text {
		font-family: 'Montserrat', sans-serif;
		line-height: 1.6;
	}

	/* WooCommerce Content */
	.woocommerce {
		background: #fff;
		padding: 0;
		font-family: 'Montserrat', sans-serif;
	}

	.woocommerce-MyAccount-content {
		padding: 0;
	}

	.woocommerce-notices-wrapper {
		margin: 0;
	}

	.section-title {
		font-family: 'Montserrat', sans-serif;
		font-size: 24px;
		font-weight: 700;
		margin-bottom: 20px;
		color: #333;
	}

	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		font-family: 'Montserrat', sans-serif;
	}

	.table thead {
		background: #f8f9fa;
		border-bottom: 1px solid #e9ecef;
	}

	.table thead th {
		padding: 12px 15px;
		font-size: 13px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		color: #495057;
		text-align: left;
	}

	.table thead th.text-right {
		text-align: right;
	}

	.table tbody tr {
		border-bottom: 1px solid #e9ecef;
	}

	.table tbody tr:last-child {
		border-bottom: none;
	}

	.table tbody td {
		padding: 12px 15px;
		font-size: 14px;
		color: #495057;
		vertical-align: middle;
	}

	.table tbody td a {
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
	}

	.table tbody td a:hover {
		color: #076787;
		text-decoration: underline;
	}

	.table tbody td time {
		color: #6c757d;
		font-size: 14px;
	}

	.table__actions {
		text-align: right;
	}

	.table__more-actions {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 6px 10px;
		background: #fff;
		border: 1px solid #dee2e6;
		border-radius: 4px;
		cursor: pointer;
		text-decoration: none;
		color: #495057;
	}

	.table__more-actions:hover {
		background: #f8f9fa;
		border-color: #adb5bd;
		text-decoration: none;
	}

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-menu {
		position: absolute;
		right: 0;
		top: 100%;
		margin-top: 4px;
		min-width: 140px;
		background: #fff;
		border: 1px solid #dee2e6;
		border-radius: 4px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
		opacity: 0;
		visibility: hidden;
		transform: translateY(-5px);
		transition: all 0.15s ease;
		z-index: 1000;
	}

	.dropdown:hover .dropdown-menu,
	.dropdown-menu:hover {
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

	.dropdown-menu__menu li {
		margin: 0;
	}

	.dropdown-menu__menu li a {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 12px;
		color: #495057;
		text-decoration: none;
		font-size: 13px;
		font-weight: 500;
	}

	.dropdown-menu__menu li a:hover {
		background: #f8f9fa;
		color: #0984ae;
		text-decoration: none;
	}

	.dropdown-menu__menu li a i {
		width: 14px;
		text-align: center;
		color: inherit;
	}

	.woocommerce-message {
		padding: 15px 20px;
		background: #e7f3ff;
		border-left: 4px solid #0984ae;
		border-radius: 4px;
		margin-bottom: 20px;
	}

	.woocommerce-message p {
		margin: 0;
		color: #495057;
		font-size: 14px;
		font-family: 'Montserrat', sans-serif;
	}

	.u--font-size-sm {
		font-size: 14px;
	}

	.col-xs-2 {
		width: 20%;
	}

	.col-xs-3 {
		width: 30%;
	}

	.text-right {
		text-align: right;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__header {
			padding: 15px 20px;
		}

		.dashboard__page-title {
			font-size: 20px;
		}

		.dashboard__content {
			padding: 20px 15px;
		}

		.table {
			display: block;
			overflow-x: auto;
		}

		.table thead th,
		.table tbody td {
			padding: 10px 12px;
		}

		.section-title {
			font-size: 20px;
		}
	}
</style>

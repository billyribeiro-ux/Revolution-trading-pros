<script lang="ts">
	import { page } from '$app/stores';
	import type { PageData } from './$types';

	interface Order {
		id: number;
		number: string;
		date: string;
		status: string;
		total: string;
		actions: OrderAction[];
	}

	interface OrderAction {
		name: string;
		url: string;
		icon: string;
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
									<a
										href="#"
										class="btn btn-xs btn-white table__more-actions"
										id="dLabel-{order.id}"
										data-bs-toggle="dropdown"
										aria-expanded="false"
									>
										<span class="st-icon-ellipsis-h icon--md"></span>
									</a>
									<nav
										class="dropdown-menu"
										data-append-to-body="1"
										aria-labelledby="dLabel-{order.id}"
									>
										<div class="dropdown-menu__content">
											<ul class="dropdown-menu__menu dropdown-menu__menu--compact">
												{#each order.actions as action}
													<li>
														<a href={action.url}>
															<i class="fa fa-{action.icon} icon--sm"></i>
															{action.name}
														</a>
													</li>
												{/each}
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

<style>
	.section-title {
		font-size: 24px;
		font-weight: 700;
		margin-bottom: 20px;
		color: #333;
	}

	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
	}

	.table thead {
		background: #f8f9fa;
		border-bottom: 2px solid #e9ecef;
	}

	.table thead th {
		padding: 16px 20px;
		font-size: 14px;
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
		transition: background-color 0.15s ease;
	}

	.table tbody tr:hover {
		background-color: #f8f9fa;
	}

	.table tbody tr:last-child {
		border-bottom: none;
	}

	.table tbody td {
		padding: 16px 20px;
		font-size: 14px;
		color: #495057;
		vertical-align: middle;
	}

	.table tbody td a {
		color: #0984ae;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.table tbody td a:hover {
		color: #076a8a;
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
		width: 32px;
		height: 32px;
		padding: 0;
		background: #fff;
		border: 1px solid #dee2e6;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.table__more-actions:hover {
		background: #f8f9fa;
		border-color: #adb5bd;
	}

	.table__more-actions .st-icon-ellipsis-h {
		color: #495057;
		font-size: 16px;
	}

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-menu {
		position: absolute;
		right: 0;
		top: 100%;
		margin-top: 8px;
		min-width: 160px;
		background: #fff;
		border: 1px solid #dee2e6;
		border-radius: 8px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		opacity: 0;
		visibility: hidden;
		transform: translateY(-10px);
		transition: all 0.2s ease;
		z-index: 1000;
	}

	.dropdown:hover .dropdown-menu,
	.dropdown-menu:hover {
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
	}

	.dropdown-menu__content {
		padding: 8px 0;
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
		gap: 12px;
		padding: 10px 16px;
		color: #495057;
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		transition: all 0.15s ease;
	}

	.dropdown-menu__menu li a:hover {
		background: #f8f9fa;
		color: #0984ae;
		text-decoration: none;
	}

	.dropdown-menu__menu li a i {
		width: 16px;
		text-align: center;
		color: inherit;
	}

	.woocommerce-message {
		padding: 16px 20px;
		background: #e7f3ff;
		border-left: 4px solid #0984ae;
		border-radius: 4px;
		margin-bottom: 20px;
	}

	.woocommerce-message p {
		margin: 0;
		color: #495057;
		font-size: 14px;
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

	@media (max-width: 768px) {
		.table {
			display: block;
			overflow-x: auto;
		}

		.table thead {
			display: none;
		}

		.table tbody tr {
			display: block;
			margin-bottom: 16px;
			border: 1px solid #e9ecef;
			border-radius: 8px;
			padding: 16px;
		}

		.table tbody td {
			display: flex;
			justify-content: space-between;
			align-items: center;
			padding: 8px 0;
			border: none;
		}

		.table tbody td::before {
			content: attr(data-label);
			font-weight: 600;
			text-transform: uppercase;
			font-size: 12px;
			color: #6c757d;
		}

		.table__actions {
			text-align: left;
		}
	}
</style>

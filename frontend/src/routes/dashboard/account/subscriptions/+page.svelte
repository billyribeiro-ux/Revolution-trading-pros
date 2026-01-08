<script lang="ts">
	interface Subscription {
		id: number;
		status: string;
		startDate: string;
		nextPayment: string;
		total: string;
		items: string[];
	}

	interface PageData {
		subscriptions: Subscription[];
	}

	let { data }: { data: PageData } = $props();

	const subscriptions = $derived(data.subscriptions || []);

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function getStatusClass(status: string): string {
		const statusMap: Record<string, string> = {
			active: 'status-active',
			'on-hold': 'status-on-hold',
			cancelled: 'status-cancelled',
			expired: 'status-expired',
			'pending-cancel': 'status-pending-cancel'
		};
		return statusMap[status] || '';
	}
</script>

<svelte:head>
	<title>My Subscriptions - Revolution Trading Pros</title>
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
		<div class="subscriptions-card">
			<div class="woocommerce-notices-wrapper"></div>
			
			<h2 class="section-title">My Subscriptions</h2>

		{#if subscriptions.length === 0}
			<div class="woocommerce-message woocommerce-message--info">
				<p>You have no active subscriptions.</p>
			</div>
		{:else}
			<table class="table shop_table_responsive">
				<thead>
					<tr>
						<th class="subscription-id">Subscription</th>
						<th class="subscription-status">Status</th>
						<th class="subscription-next-payment">Next Payment</th>
						<th class="subscription-total">Total</th>
						<th class="subscription-actions">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each subscriptions as subscription (subscription.id)}
						<tr class="subscription">
							<td class="subscription-id" data-title="Subscription">
								<a href="/dashboard/account/view-subscription/{subscription.id}">
									#{subscription.id}
								</a>
								<br />
								<small>Started: {formatDate(subscription.startDate)}</small>
							</td>
							<td class="subscription-status" data-title="Status">
								<span class="subscription-status-badge {getStatusClass(subscription.status)}">
									{subscription.status}
								</span>
							</td>
							<td class="subscription-next-payment" data-title="Next Payment">
								{subscription.nextPayment ? formatDate(subscription.nextPayment) : 'N/A'}
							</td>
							<td class="subscription-total" data-title="Total">
								{subscription.total}
							</td>
							<td class="subscription-actions" data-title="Actions">
								<a href="/dashboard/account/view-subscription/{subscription.id}" class="btn btn-sm btn-white">
									View
								</a>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
			{/if}
		</div>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
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

	/* Dashboard Content */
	.dashboard__content {
		background: #f5f5f5;
		min-height: calc(100vh - 60px);
		padding: 40px 30px;
	}

	.dashboard__content-main {
		max-width: 1100px;
		margin: 0 auto;
	}

	/* Professional Card Container */
	.subscriptions-card {
		background: #ffffff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		padding: 40px;
		margin-bottom: 30px;
	}

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
		color: #143E59;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.table tbody td a:hover {
		color: #0f2f43;
		text-decoration: underline;
	}

	.table tbody td small {
		color: #6c757d;
		font-size: 12px;
	}

	.subscription-status-badge {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 12px;
		font-size: 12px;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-active {
		background: #d4edda;
		color: #155724;
	}

	.status-on-hold {
		background: #fff3cd;
		color: #856404;
	}

	.status-cancelled,
	.status-expired {
		background: #f8d7da;
		color: #721c24;
	}

	.status-pending-cancel {
		background: #d1ecf1;
		color: #0c5460;
	}

	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.15s ease;
		border: 1px solid #dee2e6;
	}

	.btn-white {
		background: #fff;
		color: #143E59;
		border-color: #143E59;
	}

	.btn-white:hover {
		background: #143E59;
		color: #fff;
	}

	.btn-sm {
		padding: 6px 12px;
		font-size: 13px;
	}

	.woocommerce-message {
		padding: 16px 20px;
		background: #e7f3ff;
		border-left: 4px solid #143E59;
		border-radius: 4px;
		margin-bottom: 20px;
	}

	.woocommerce-message p {
		margin: 0;
		color: #495057;
		font-size: 14px;
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
			content: attr(data-title);
			font-weight: 600;
			text-transform: uppercase;
			font-size: 12px;
			color: #6c757d;
		}
	}
</style>

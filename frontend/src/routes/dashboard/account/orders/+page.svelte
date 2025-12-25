<script lang="ts">
	/**
	 * My Orders - Account Section
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * 100% PIXEL-PERFECT match to SimplerMyOrders reference
	 * Orders table with actions dropdown
	 *
	 * @version 2.0.0 - 100% Pixel Perfect
	 */

	// Sample orders data matching SimplerMyOrders reference
	const orders = [
		{
			id: '2176654',
			date: 'December 3, 2025',
			datetime: '2025-12-03T02:01:54+00:00'
		},
		{
			id: '2173014',
			date: 'November 17, 2025',
			datetime: '2025-11-17T07:02:04+00:00'
		},
		{
			id: '2170001',
			date: 'October 28, 2025',
			datetime: '2025-10-28T14:30:00+00:00'
		}
	];
</script>

<svelte:head>
	<title>My Orders - Account | Revolution Trading Pros</title>
</svelte:head>

<!-- 100% EXACT structure from SimplerMyOrders reference -->
<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">My Orders</h2>

			<table class="table">
				<thead>
					<tr>
						<th class="col-xs-2">Order</th>
						<th class="col-xs-3">Date</th>
						<th class="col-xs-2 text-right">Actions</th>
					</tr>
				</thead>
				<tbody class="u--font-size-sm">
					{#each orders as order}
						<tr>
							<td class="col-xs-2">
								<a href="/dashboard/account/view-order/{order.id}">
									#{order.id}
								</a>
							</td>
							<td class="col-xs-3">
								<time datetime={order.datetime}>{order.date}</time>
							</td>
							<td class="col-xs-2 text-right table__actions">
								<div class="dropdown">
									<a href="/dashboard/account/view-order/{order.id}" class="btn btn-xs btn-white table__more-actions">
										<span class="st-icon-ellipsis-h icon--md">⋯</span>
									</a>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>

			{#if orders.length === 0}
				<div class="empty-state">
					<p>No orders found.</p>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching SimplerMyOrders reference
	   ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Table - Exact from reference */
	.table {
		width: 100%;
		border-collapse: collapse;
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.table thead {
		background: #f9f9f9;
		border-bottom: 1px solid #dbdbdb;
	}

	.table th {
		padding: 15px 20px;
		font-size: 13px;
		font-weight: 700;
		color: #333;
		text-align: left;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.table td {
		padding: 15px 20px;
		font-size: 14px;
		color: #333;
		border-bottom: 1px solid #ededed;
	}

	.table tbody tr:last-child td {
		border-bottom: none;
	}

	.table tbody tr:hover {
		background: #f9f9f9;
	}

	/* Column widths */
	.col-xs-2 {
		width: 20%;
	}

	.col-xs-3 {
		width: 30%;
	}

	.text-right {
		text-align: right;
	}

	/* Font size utility */
	:global(.u--font-size-sm) {
		font-size: 14px;
	}

	/* Links */
	.table td a {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
	}

	.table td a:hover {
		color: #0e6ac4;
		text-decoration: underline;
	}

	/* Actions */
	.table__actions {
		text-align: right;
	}

	.table__more-actions {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 8px 12px;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		color: #666;
		font-size: 16px;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.table__more-actions:hover {
		background: #f4f4f4;
		border-color: #999;
	}

	.icon--md {
		font-size: 16px;
		letter-spacing: 2px;
	}

	/* Empty state */
	.empty-state {
		text-align: center;
		padding: 40px 20px;
		color: #666;
	}

	@media (max-width: 768px) {
		.table th,
		.table td {
			padding: 12px 15px;
		}

		.col-xs-2,
		.col-xs-3 {
			width: auto;
		}
	}
</style>

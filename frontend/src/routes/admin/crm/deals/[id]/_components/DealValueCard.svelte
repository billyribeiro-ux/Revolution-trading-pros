<script lang="ts">
	import type { Deal } from '$lib/crm/types';
	import { formatCurrency, formatDate } from './helpers';

	interface Props {
		deal: Deal;
	}

	let { deal }: Props = $props();
</script>

<div class="value-card">
	<div class="value-main">
		<span class="value-label">Deal Value</span>
		<span class="value-amount">{formatCurrency(deal.amount)}</span>
	</div>
	<div class="value-stats">
		<div class="value-stat">
			<span class="stat-label">Probability</span>
			<div class="probability-bar">
				<div class="bar-fill" style="width: {deal.probability}%"></div>
				<span class="bar-text">{deal.probability}%</span>
			</div>
		</div>
		<div class="value-stat">
			<span class="stat-label">Weighted Value</span>
			<span class="stat-value">{formatCurrency(deal.weighted_value)}</span>
		</div>
		<div class="value-stat">
			<span class="stat-label">Expected Close</span>
			<span class="stat-value">{formatDate(deal.expected_close_date)}</span>
		</div>
		<div class="value-stat">
			<span class="stat-label">Days in Pipeline</span>
			<span class="stat-value">{deal.days_in_pipeline} days</span>
		</div>
	</div>
</div>

<style>
	.value-card {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 16px;
		padding: 24px;
		margin-bottom: 24px;
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 32px;
		align-items: center;
	}

	.value-main {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.value-label {
		font-size: 0.85rem;
		color: #64748b;
	}

	.value-amount {
		font-size: 2.5rem;
		font-weight: 700;
		color: #4ade80;
	}

	.value-stats {
		display: flex;
		gap: 32px;
		flex-wrap: wrap;
	}

	.value-stat {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.value-stat .stat-label {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.value-stat .stat-value {
		font-size: 1.1rem;
		font-weight: 600;
		color: white;
	}

	.probability-bar {
		position: relative;
		width: 100px;
		height: 24px;
		background: #0f172a;
		border-radius: 12px;
		overflow: hidden;
	}

	.bar-fill {
		position: absolute;
		left: 0;
		top: 0;
		height: 100%;
		background: linear-gradient(135deg, #f97316, #ea580c);
		transition: width 0.3s;
	}

	.bar-text {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		font-size: 0.75rem;
		font-weight: 600;
		color: white;
	}

	@media (max-width: 767.98px) {
		.value-card {
			grid-template-columns: 1fr;
		}

		.value-stats {
			justify-content: flex-start;
		}
	}
</style>

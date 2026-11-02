<script lang="ts">
	import {
		IconUsers,
		IconFlame,
		IconTarget,
		IconCheck,
		IconChartBar,
		IconTrendingUp
	} from '#lib/icons/index.js';

	interface LeadStats {
		total: number;
		new: number;
		contacted: number;
		qualified: number;
		proposal: number;
		negotiation: number;
		won: number;
		lost: number;
		hot_leads: number;
		total_value: number;
		conversion_rate: number;
	}

	interface Props {
		stats: LeadStats;
		formatCurrency: (amount: number) => string;
	}

	let { stats, formatCurrency }: Props = $props();
</script>

<section class="stats-grid">
	<div class="stat-card highlight">
		<div class="stat-icon blue">
			<IconUsers size={24} />
		</div>
		<div class="stat-content">
			<span class="stat-value">{stats.total.toLocaleString()}</span>
			<span class="stat-label">Total Leads</span>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon cyan">
			<IconFlame size={24} />
		</div>
		<div class="stat-content">
			<span class="stat-value">{stats.hot_leads}</span>
			<span class="stat-label">Hot Leads</span>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon purple">
			<IconTarget size={24} />
		</div>
		<div class="stat-content">
			<span class="stat-value">{stats.qualified}</span>
			<span class="stat-label">Qualified</span>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon emerald">
			<IconCheck size={24} />
		</div>
		<div class="stat-content">
			<span class="stat-value">{stats.won}</span>
			<span class="stat-label">Won</span>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon amber">
			<IconChartBar size={24} />
		</div>
		<div class="stat-content">
			<span class="stat-value">{formatCurrency(stats.total_value)}</span>
			<span class="stat-label">Pipeline Value</span>
		</div>
	</div>
	<div class="stat-card">
		<div class="stat-icon green">
			<IconTrendingUp size={24} />
		</div>
		<div class="stat-content">
			<span class="stat-value">{stats.conversion_rate.toFixed(1)}%</span>
			<span class="stat-label">Conversion Rate</span>
		</div>
	</div>
</section>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(6, 1fr);
		gap: 16px;
		margin-bottom: 24px;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 16px;
		padding: 20px;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		transition: all 0.2s;
	}

	.stat-card:hover {
		border-color: rgba(230, 184, 0, 0.3);
	}

	.stat-card.highlight {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.stat-icon.blue {
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.cyan {
		background: rgba(88, 166, 255, 0.15);
		color: var(--info-emphasis, #58a6ff);
	}
	.stat-icon.purple {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400, #ffd11a);
	}
	.stat-icon.emerald {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}
	.stat-icon.amber {
		background: rgba(210, 153, 34, 0.15);
		color: var(--warning-emphasis, #d29922);
	}
	.stat-icon.green {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ffffff;
	}

	.stat-label {
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
	}

	@media (max-width: 1400px) {
		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 1023.98px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 767.98px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

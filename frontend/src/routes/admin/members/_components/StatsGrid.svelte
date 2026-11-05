<script lang="ts">
	import {
		IconUsers,
		IconUserCheck,
		IconCreditCard,
		IconTrendingDown,
		IconTrendingUp,
		IconCrown,
		IconCurrencyDollar,
		IconAlertTriangle,
		IconExternalLink
	} from '#lib/icons/index.js';
	import type { MemberStats } from '#lib/api/members.js';

	interface Props {
		stats: MemberStats;
		formatCurrency: (amount: number | null | undefined) => string;
		onRecoverChurned: () => void;
	}

	let { stats, formatCurrency, onRecoverChurned }: Props = $props();

	let sparklineMax = $derived(Math.max(1, ...stats.growth_trend.map((p) => p.new)));
	let activePercent = $derived(
		stats.overview.total_members > 0
			? Math.round((stats.subscriptions.active / stats.overview.total_members) * 100)
			: null
	);
	let activeDashOffset = $derived(
		stats.overview.total_members > 0
			? (stats.subscriptions.active / stats.overview.total_members) * 100
			: 0
	);
</script>

<div class="stats-grid">
	<!-- Total Members -->
	<div class="stat-card gradient-purple">
		<div class="stat-icon">
			<IconUsers size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Total Members</div>
			<div class="stat-value">{stats.overview.total_members.toLocaleString()}</div>
			<div class="stat-change positive">
				<IconTrendingUp size={14} />
				+{stats.overview.new_this_month} this month
			</div>
		</div>
		<div class="stat-sparkline" title={stats.growth_scope}>
			{#each stats.growth_trend.slice(-6) as point (point.month)}
				<div class="sparkline-bar" style:height={`${(point.new / sparklineMax) * 100}%`}></div>
			{/each}
		</div>
	</div>

	<!-- Active Members -->
	<div class="stat-card gradient-emerald">
		<div class="stat-icon">
			<IconUserCheck size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Active Members</div>
			<div class="stat-value">{stats.subscriptions.active.toLocaleString()}</div>
			<div class="stat-change neutral">
				<IconCrown size={14} />
				{stats.subscriptions.trial} in trial
			</div>
		</div>
		<div class="stat-ring">
			<svg aria-hidden="true" viewBox="0 0 36 36">
				<circle
					cx="18"
					cy="18"
					r="16"
					fill="none"
					stroke="currentColor"
					stroke-opacity="0.2"
					stroke-width="3"
				></circle>
				<circle
					cx="18"
					cy="18"
					r="16"
					fill="none"
					stroke="currentColor"
					stroke-width="3"
					stroke-dasharray="{activeDashOffset} 100"
					stroke-linecap="round"
					transform="rotate(-90 18 18)"
				></circle>
			</svg>
			<span>{activePercent === null ? '—' : `${activePercent}%`}</span>
		</div>
	</div>

	<!-- Monthly Recurring Revenue -->
	<div class="stat-card gradient-gold">
		<div class="stat-icon">
			<IconCreditCard size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Recorded net USD order value</div>
			<div class="stat-value">{formatCurrency(stats.revenue.recorded_net_usd_order_value.net_cents / 100)}</div>
			<div class="stat-change neutral">
				<IconCurrencyDollar size={14} />
				{stats.revenue.recorded_net_usd_order_value.order_count} qualifying USD orders
			</div>
		</div>
	</div>

	<!-- Churn Rate -->
	<div class="stat-card gradient-red">
		<div class="stat-icon">
			<IconTrendingDown size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Churn Rate</div>
			<div class="stat-value" title={stats.subscriptions.churn_rate_reason}>Unmeasured</div>
			<div class="stat-change negative">
				<IconAlertTriangle size={14} />
				{stats.subscriptions.churned} current former members
			</div>
		</div>
		<button class="stat-action" onclick={onRecoverChurned}>
			Recover <IconExternalLink size={14} />
		</button>
	</div>
</div>

<details class="measurement-details">
	<summary>Measurement coverage and catalogue estimate</summary>
	<p>{stats.revenue.recorded_net_usd_order_value.limitations}</p>
	<p>{stats.revenue.recorded_net_usd_order_value.excluded_non_usd_orders} non-USD and {stats.revenue.recorded_net_usd_order_value.excluded_unknown_currency_orders} unknown-currency qualifying orders excluded.</p>
	<p>Monthly recurring revenue: unmeasured. {stats.revenue.mrr_reason}</p>
	<p>Average lifetime value: unmeasured. {stats.revenue.avg_ltv_reason}</p>
	<p>Historical churn rate: unmeasured. {stats.subscriptions.churn_rate_reason}</p>
	<p>Monthly catalogue estimate: {stats.revenue.catalog_monthly_estimate.value === null ? 'Unavailable' : stats.revenue.catalog_monthly_estimate.value.toLocaleString('en-US', { maximumFractionDigits: 2 })} catalogue price units; currency unrecorded. {stats.revenue.catalog_monthly_estimate.included_membership_records} active membership records included, {stats.revenue.catalog_monthly_estimate.excluded_membership_records} excluded. {stats.revenue.catalog_monthly_estimate.reason}</p>
	<p>Registration growth: {stats.overview.growth_rate === null ? stats.overview.growth_rate_reason : `${stats.overview.growth_rate}% month to date versus the previous full month`}. {stats.growth_scope}</p>
	<p>Registration trend excludes {stats.growth_timestamp_coverage.unknown_members} members with missing recorded dates and {stats.growth_timestamp_coverage.future_members} with future recorded dates.</p>
	<p>{stats.recorded_timestamp_basis} Evaluation uses {stats.period.timezone}; evaluated {stats.evaluated_at}.</p>
</details>

<style>
	.measurement-details { margin: -1rem 0 2rem; color: var(--text-secondary); font-size: .875rem; }
	.measurement-details p { margin: .6rem 0; }
	.stat-value { overflow-wrap: anywhere; }
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(22, 27, 34, 0.8);
		border-radius: 12px;
		padding: 1.5rem;
		position: relative;
		overflow: hidden;
		border: 1px solid var(--border-muted);
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.stat-card.gradient-purple {
		border-color: rgba(230, 184, 0, 0.3);
	}
	.stat-card.gradient-emerald {
		border-color: rgba(63, 185, 80, 0.3);
	}
	.stat-card.gradient-gold {
		border-color: rgba(210, 153, 34, 0.3);
	}
	.gradient-red .stat-content { padding-bottom: 2.5rem; }

	.stat-card.gradient-red {
		border-color: rgba(248, 81, 73, 0.3);
	}

	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.gradient-purple .stat-icon {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}
	.gradient-emerald .stat-icon {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis);
	}
	.gradient-gold .stat-icon {
		background: rgba(210, 153, 34, 0.15);
		color: var(--warning-emphasis);
	}
	.gradient-red .stat-icon {
		background: rgba(248, 81, 73, 0.15);
		color: var(--error-emphasis);
	}

	.stat-label {
		font-size: 0.8125rem;
		color: var(--admin-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 600;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 800;
		color: var(--admin-text-primary);
	}

	.stat-change {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.8125rem;
		font-weight: 500;
	}

	.stat-change.positive {
		color: var(--admin-success);
	}
	.stat-change.neutral {
		color: var(--admin-text-muted);
	}
	.stat-change.negative {
		color: var(--admin-error);
	}

	.stat-sparkline {
		position: absolute;
		right: 1rem;
		bottom: 1rem;
		display: flex;
		align-items: flex-end;
		gap: 3px;
		height: 40px;
	}

	.sparkline-bar {
		width: 6px;
		background: rgba(230, 184, 0, 0.4);
		border-radius: 2px;
		min-height: 4px;
	}

	.stat-ring {
		position: absolute;
		right: 1rem;
		top: 1rem;
		width: 60px;
		height: 60px;
	}

	.stat-ring svg {
		width: 100%;
		height: 100%;
		color: var(--success-emphasis);
	}

	.stat-ring span {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 700;
		color: var(--success-emphasis);
	}

	.stat-action {
		position: absolute;
		right: 1rem;
		bottom: 1rem;
		padding: 0.375rem 0.75rem;
		background: rgba(248, 81, 73, 0.15);
		border: 1px solid rgba(248, 81, 73, 0.3);
		border-radius: 6px;
		color: var(--error-emphasis);
		font-size: 0.75rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		transition: all 0.2s;
	}

	.stat-action:hover {
		background: rgba(248, 81, 73, 0.25);
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 1023px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 0.75rem;
		}
	}

	@media (max-width: 767px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	@media (max-width: 639px) {
		.stats-grid {
			gap: 0.5rem;
		}

		.stat-card {
			padding: 1rem;
		}

		.stat-value {
			font-size: 1.25rem;
		}
	}

	@media (max-width: 380px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}

		.stat-card {
			padding: 0.75rem;
		}

		.stat-value {
			font-size: 1.125rem;
		}
	}

	@media (hover: none) and (pointer: coarse) {
		.stat-card {
			min-height: 80px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.stat-card {
			transition: none;
		}
	}

	@media (prefers-contrast: high) {
		.stat-card {
			border-width: 2px;
		}

		.stat-value {
			font-weight: 800;
		}
	}

	@media print {
		.stat-card {
			break-inside: avoid;
			box-shadow: none;
			border: 1px solid var(--border-default, #30363d);
	}
	}
</style>

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
	} from '$lib/icons';
	import type { MemberStats } from '$lib/api/members';

	interface Props {
		stats: MemberStats;
		formatCurrency: (amount: number | null | undefined) => string;
		onRecoverChurned: () => void;
	}

	let { stats, formatCurrency, onRecoverChurned }: Props = $props();

	let sparklineMax = $derived(Math.max(...(stats.growth_trend || []).map((p) => p.new)));
	let activePercent = $derived(
		stats.overview.total_members > 0
			? Math.round((stats.subscriptions.active / stats.overview.total_members) * 100)
			: 0
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
		<div class="stat-sparkline">
			{#each (stats.growth_trend || []).slice(-6) as point, i (i)}
				<div class="sparkline-bar" style="height: {(point.new / sparklineMax) * 100}%"></div>
			{/each}
		</div>
	</div>

	<!-- Active Subscribers -->
	<div class="stat-card gradient-emerald">
		<div class="stat-icon">
			<IconUserCheck size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Active Subscribers</div>
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
			<span>{activePercent}%</span>
		</div>
	</div>

	<!-- Monthly Recurring Revenue -->
	<div class="stat-card gradient-gold">
		<div class="stat-icon">
			<IconCreditCard size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Monthly Revenue</div>
			<div class="stat-value">{formatCurrency(stats?.revenue?.mrr ?? 0)}</div>
			<div class="stat-change neutral">
				<IconCurrencyDollar size={14} />
				{formatCurrency(stats?.revenue?.avg_ltv ?? 0)} avg LTV
			</div>
		</div>
		<div class="stat-glow"></div>
	</div>

	<!-- Churn Rate -->
	<div class="stat-card gradient-red">
		<div class="stat-icon">
			<IconTrendingDown size={28} />
		</div>
		<div class="stat-content">
			<div class="stat-label">Churn Rate</div>
			<div class="stat-value">{stats?.subscriptions?.churn_rate ?? 0}%</div>
			<div class="stat-change negative">
				<IconAlertTriangle size={14} />
				{stats?.subscriptions?.churned ?? 0} churned
			</div>
		</div>
		<button class="stat-action" onclick={onRecoverChurned}>
			Recover <IconExternalLink size={14} />
		</button>
	</div>
</div>

<style>
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
		backdrop-filter: blur(10px);
	}

	.stat-card.gradient-purple {
		border-color: rgba(230, 184, 0, 0.3);
	}
	.stat-card.gradient-emerald {
		border-color: rgba(16, 185, 129, 0.3);
	}
	.stat-card.gradient-gold {
		border-color: rgba(251, 191, 36, 0.3);
	}
	.stat-card.gradient-red {
		border-color: rgba(239, 68, 68, 0.3);
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
		background: rgba(16, 185, 129, 0.15);
		color: var(--success-emphasis);
	}
	.gradient-gold .stat-icon {
		background: rgba(251, 191, 36, 0.15);
		color: var(--warning-emphasis);
	}
	.gradient-red .stat-icon {
		background: rgba(239, 68, 68, 0.15);
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

	.stat-glow {
		position: absolute;
		inset: 0;
		background: radial-gradient(circle at 80% 20%, rgba(251, 191, 36, 0.15), transparent 60%);
		pointer-events: none;
	}

	.stat-action {
		position: absolute;
		right: 1rem;
		bottom: 1rem;
		padding: 0.375rem 0.75rem;
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
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
		background: rgba(239, 68, 68, 0.25);
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
			border: 1px solid #ccc;
		}
	}
</style>

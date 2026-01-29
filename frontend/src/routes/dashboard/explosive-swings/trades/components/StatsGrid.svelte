<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * StatsGrid Component - Performance Metrics Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { TradeStats } from '../types';

	interface Props {
		stats: TradeStats;
		isLoading?: boolean;
	}

	const { stats, isLoading = false }: Props = $props();
</script>

<section class="stats-grid" aria-label="Trading performance statistics">
	{#if isLoading}
		{#each Array(8) as _, i}
			<div class="stat-card skeleton" aria-hidden="true">
				<div class="skeleton-value"></div>
				<div class="skeleton-label"></div>
			</div>
		{/each}
	{:else}
		<div class="stat-card">
			<div class="stat-value">{stats.totalTrades}</div>
			<div class="stat-label">Total Trades</div>
		</div>

		<div class="stat-card">
			<div class="stat-value profit">{stats.wins}</div>
			<div class="stat-label">Wins</div>
		</div>

		<div class="stat-card">
			<div class="stat-value loss">{stats.losses}</div>
			<div class="stat-label">Losses</div>
		</div>

		<div class="stat-card">
			<div class="stat-value">{stats.winRate}%</div>
			<div class="stat-label">Win Rate</div>
		</div>

		<div class="stat-card">
			<div
				class="stat-value"
				class:profit={stats.totalProfit > 0}
				class:loss={stats.totalProfit < 0}
			>
				${stats.totalProfit.toLocaleString()}
			</div>
			<div class="stat-label">Total Profit</div>
		</div>

		<div class="stat-card">
			<div class="stat-value">${stats.avgWin}</div>
			<div class="stat-label">Avg Win</div>
		</div>

		<div class="stat-card">
			<div class="stat-value">${stats.avgLoss}</div>
			<div class="stat-label">Avg Loss</div>
		</div>

		<div class="stat-card">
			<div class="stat-value">{stats.profitFactor}</div>
			<div class="stat-label">Profit Factor</div>
		</div>
	{/if}
</section>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: var(--space-4);
		max-width: 1400px;
		margin: 0 auto var(--space-10);
		padding: 0 var(--space-4);
	}

	/* 4 columns at 640px+ */
	@media (min-width: 640px) {
		.stats-grid {
			grid-template-columns: repeat(4, 1fr);
			gap: var(--space-5);
		}
	}

	/* 8 columns ONLY at 1280px+ (not 1024px) */
	@media (min-width: 1280px) {
		.stats-grid {
			grid-template-columns: repeat(8, 1fr);
		}
	}

	.stat-card {
		background: var(--color-bg-card);
		border-radius: var(--radius-lg);
		padding: var(--space-4) var(--space-3);
		text-align: center;
		box-shadow: var(--shadow-sm);
		border: 1px solid var(--color-border-subtle);
		transition: var(--transition-shadow);
		min-width: 0; /* Allow flex shrink */
		overflow: hidden;
	}

	.stat-card:hover {
		box-shadow: var(--shadow-md);
	}

	.stat-value {
		font-size: clamp(1.125rem, 2vw, 1.5rem); /* 18px to 24px, responsive */
		font-weight: var(--font-bold);
		color: var(--color-brand-primary);
		font-family: var(--font-display);
		margin-bottom: var(--space-1);
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.stat-value.profit {
		color: var(--color-profit);
	}

	.stat-value.loss {
		color: var(--color-loss);
	}

	.stat-label {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: var(--tracking-wider);
		font-weight: var(--font-medium);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Skeleton Loading */
	.stat-card.skeleton {
		pointer-events: none;
	}

	.skeleton-value {
		height: 24px;
		width: 60%;
		margin: 0 auto var(--space-2);
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-sm);
	}

	.skeleton-label {
		height: 12px;
		width: 80%;
		margin: 0 auto;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: var(--radius-sm);
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Extra small screens - larger touch targets */
	@media (max-width: 400px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: var(--space-3);
		}

		.stat-card {
			padding: var(--space-3) var(--space-2);
		}

		.stat-value {
			font-size: 1rem;
		}
	}
</style>

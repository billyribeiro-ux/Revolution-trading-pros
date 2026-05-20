<!--
	StatsBar.svelte - Performance Stats Row Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple Principal Engineer ICT 11+ Standards
	Svelte 5 January 2026 Syntax
	
	Single Responsibility: Display win rate, P&L, active/closed counts
	
	Features:
	- Circular progress ring for win rate
	- Sticky positioning at top of page
	- Backdrop blur effect
	- Responsive layout
	
	@version 1.0.0
	@since January 2026
-->
<script lang="ts">
	import type { QuickStats } from '../alerts/types';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() pattern
	// ═══════════════════════════════════════════════════════════════════════════
	interface Props {
		stats: QuickStats;
		isLoading?: boolean;
	}

	let { stats, isLoading = false }: Props = $props();
</script>

<section class="stats-bar" aria-label="Performance statistics">
	{#if isLoading}
		<!-- Skeleton Loading State -->
		<div class="stat stat-skeleton">
			<div class="skeleton-ring"></div>
			<div class="skeleton-label"></div>
		</div>
		<div class="stat stat-skeleton">
			<div class="skeleton-value"></div>
			<div class="skeleton-label"></div>
		</div>
		<div class="stat stat-skeleton">
			<div class="skeleton-value"></div>
			<div class="skeleton-label"></div>
		</div>
		<div class="stat stat-skeleton">
			<div class="skeleton-value"></div>
			<div class="skeleton-label"></div>
		</div>
	{:else}
		<!-- Win Rate with Circular Progress Ring -->
		<div class="stat stat-with-ring">
			<div class="win-rate-ring" role="img" aria-label="Win rate: {stats.winRate}%">
				<svg viewBox="0 0 36 36" class="circular-chart" aria-hidden="true">
					<path
						class="circle-bg"
						d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
					/>
					<path
						class="circle"
						stroke-dasharray="{stats.winRate}, 100"
						d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
					/>
				</svg>
				<span class="stat-value-ring">{stats.winRate}%</span>
			</div>
			<span class="stat-label">Win Rate</span>
		</div>

		<!-- Weekly Profit -->
		<div class="stat">
			<span class="stat-value" class:green={stats.weeklyProfit.startsWith('+')}>
				{stats.weeklyProfit}
			</span>
			<span class="stat-label">This Week</span>
		</div>

		<!-- Active Trades -->
		<div class="stat">
			<span class="stat-value">{stats.activeTrades}</span>
			<span class="stat-label">Active Trades</span>
		</div>

		<!-- Closed This Week -->
		<div class="stat">
			<span class="stat-value">{stats.closedThisWeek}</span>
			<span class="stat-label">Closed This Week</span>
		</div>
	{/if}
</section>

<style>
	.stats-bar {
		display: flex;
		justify-content: center;
		gap: 60px;
		padding: 24px 30px;
		background: rgba(255, 255, 255, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom: 1px solid rgba(229, 231, 235, 0.8);
		flex-wrap: wrap;
		position: sticky;
		top: 0;
		z-index: 50;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
	}

	.stat {
		text-align: center;
	}

	.stat-with-ring {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.win-rate-ring {
		position: relative;
		width: 70px;
		height: 70px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.circular-chart {
		position: absolute;
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.circle-bg {
		fill: none;
		stroke: #e5e7eb;
		stroke-width: 3;
	}

	.circle {
		fill: none;
		stroke: #22c55e;
		stroke-width: 3;
		stroke-linecap: round;
		transition: stroke-dasharray 0.6s ease;
	}

	.stat-value-ring {
		font-size: 18px;
		font-weight: 700;
		color: #143e59;
		font-family: 'Montserrat', sans-serif;
		z-index: 1;
	}

	.stat-value {
		display: block;
		font-size: 32px;
		font-weight: 700;
		color: #143e59;
		font-family: 'Montserrat', sans-serif;
	}

	.stat-value.green {
		color: #22c55e;
	}

	.stat-label {
		font-size: 12px;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-top: 4px;
	}

	/* Skeleton Loading */
	.stat-skeleton {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
	}

	.skeleton-ring {
		width: 70px;
		height: 70px;
		border-radius: 50%;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-value {
		width: 80px;
		height: 32px;
		border-radius: 4px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-label {
		width: 60px;
		height: 14px;
		border-radius: 4px;
		background: linear-gradient(90deg, #e5e7eb 25%, #f3f4f6 50%, #e5e7eb 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Responsive */
	@media (max-width: 640px) {
		.stats-bar {
			gap: 24px;
			padding: 20px;
		}

		.stat-value {
			font-size: 24px;
		}

		.win-rate-ring {
			width: 56px;
			height: 56px;
		}

		.stat-value-ring {
			font-size: 14px;
		}
	}
</style>

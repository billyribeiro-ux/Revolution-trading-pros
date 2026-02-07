<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * SetupBreakdown Component - Trade Setup Performance Breakdown
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 1.0.0 - Phase 4: Analytics Dashboard
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5 January 2026
	 */
	import type { SetupPerformance } from '../analytics.state.svelte';

	interface Props {
		data: SetupPerformance[];
		isLoading?: boolean;
	}

	const { data, isLoading = false }: Props = $props();

	// Sort by total trades
	const sortedData = $derived([...data].sort((a, b) => b.total_trades - a.total_trades));

	// Calculate total trades for pie chart
	const totalTrades = $derived(data.reduce((acc, d) => acc + d.total_trades, 0));

	// Colors for pie chart slices
	const colors = [
		'#3B82F6',
		'#10B981',
		'#F59E0B',
		'#EF4444',
		'#8B5CF6',
		'#EC4899',
		'#06B6D4',
		'#84CC16',
		'#F97316',
		'#6366F1'
	];

	// Calculate pie chart angles
	const pieData = $derived(
		sortedData.map((d, i) => {
			const percentage = totalTrades > 0 ? (d.total_trades / totalTrades) * 100 : 0;
			return {
				...d,
				percentage,
				color: colors[i % colors.length]
			};
		})
	);

	// Generate pie chart path
	function generatePieSlice(startAngle: number, endAngle: number, radius: number): string {
		const x1 = radius + radius * Math.cos(startAngle);
		const y1 = radius + radius * Math.sin(startAngle);
		const x2 = radius + radius * Math.cos(endAngle);
		const y2 = radius + radius * Math.sin(endAngle);
		const largeArcFlag = endAngle - startAngle > Math.PI ? 1 : 0;

		return `M ${radius} ${radius} L ${x1} ${y1} A ${radius} ${radius} 0 ${largeArcFlag} 1 ${x2} ${y2} Z`;
	}

	// Pie slices with angles
	const pieSlices = $derived.by(() => {
		let currentAngle = -Math.PI / 2;
		return pieData.map((d) => {
			const sliceAngle = (d.percentage / 100) * Math.PI * 2;
			const startAngle = currentAngle;
			const endAngle = currentAngle + sliceAngle;
			currentAngle = endAngle;
			return {
				...d,
				startAngle,
				endAngle,
				path: generatePieSlice(startAngle, endAngle, 80)
			};
		});
	});

	// Format helpers
	function _formatPercent(value: number): string {
		return `${value >= 0 ? '+' : ''}${value.toFixed(1)}%`;
	}

	function formatRatio(value: number): string {
		if (!isFinite(value) || value > 100) return '99+';
		return value.toFixed(2);
	}

	// Hover state
	let hoveredSetup: string | null = $state(null);
</script>

<div class="setup-breakdown" role="region" aria-label="Trade setup breakdown">
	<h3 class="section-title">Setup Breakdown</h3>

	{#if isLoading}
		<div class="skeleton-content">
			<div class="skel-pie"></div>
			<div class="skel-legend">
				{#each Array(4) as _}
					<div class="skel-item"></div>
				{/each}
			</div>
		</div>
	{:else if data.length === 0}
		<div class="empty-state">
			<p>No setup data available</p>
		</div>
	{:else}
		<div class="content-wrapper">
			<!-- Pie Chart -->
			<div class="pie-chart-container">
				<svg viewBox="0 0 160 160" class="pie-chart">
					{#each pieSlices as slice}
						<path
							d={slice.path}
							fill={slice.color}
							opacity={hoveredSetup === null || hoveredSetup === slice.setup ? 1 : 0.5}
							class="pie-slice"
							role="img"
							aria-label="{slice.setup}: {slice.percentage.toFixed(0)}% of trades"
							onmouseenter={() => (hoveredSetup = slice.setup)}
							onmouseleave={() => (hoveredSetup = null)}
						/>
					{/each}
					<!-- Center circle for donut effect -->
					<circle cx="80" cy="80" r="50" fill="var(--color-bg-card)" />
					<!-- Center text -->
					<text x="80" y="76" text-anchor="middle" class="center-label">Total</text>
					<text x="80" y="94" text-anchor="middle" class="center-value">{totalTrades}</text>
				</svg>
			</div>

			<!-- Legend & Details -->
			<div class="legend-section">
				{#each pieData as setup}
					<div
						class="legend-item"
						class:hovered={hoveredSetup === setup.setup}
						role="button"
						tabindex="0"
						aria-label="{setup.setup} trading setup details"
						onmouseenter={() => (hoveredSetup = setup.setup)}
						onmouseleave={() => (hoveredSetup = null)}
					>
						<div class="legend-color" style="background-color: {setup.color}"></div>
						<div class="legend-content">
							<div class="legend-header">
								<span class="setup-name">{setup.setup}</span>
								<span class="setup-count"
									>{setup.total_trades} trades ({setup.percentage.toFixed(0)}%)</span
								>
							</div>
							<div class="setup-stats">
								<span
									class="stat"
									class:profit={setup.win_rate >= 50}
									class:loss={setup.win_rate < 50}
								>
									{setup.win_rate.toFixed(0)}% WR
								</span>
								<span
									class="stat"
									class:profit={setup.total_pnl >= 0}
									class:loss={setup.total_pnl < 0}
								>
									P&L: ${setup.total_pnl.toFixed(2)}
								</span>
								<span class="stat">PF: {formatRatio(setup.profit_factor)}</span>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<style>
	.setup-breakdown {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
	}

	.section-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0 0 16px 0;
	}

	.content-wrapper {
		display: flex;
		gap: 24px;
		align-items: flex-start;
	}

	/* Pie Chart */
	.pie-chart-container {
		flex-shrink: 0;
	}

	.pie-chart {
		width: 160px;
		height: 160px;
	}

	.pie-slice {
		cursor: pointer;
		transition: opacity 0.2s;
	}

	.center-label {
		font-size: 12px;
		fill: var(--color-text-tertiary);
	}

	.center-value {
		font-size: 22px;
		font-weight: 700;
		fill: var(--color-text-primary);
	}

	/* Legend */
	.legend-section {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.legend-item {
		display: flex;
		gap: 12px;
		padding: 8px;
		border-radius: 8px;
		transition: background 0.15s;
		cursor: pointer;
	}

	.legend-item:hover,
	.legend-item.hovered {
		background: var(--color-bg-subtle);
	}

	.legend-color {
		width: 12px;
		height: 12px;
		border-radius: 3px;
		margin-top: 4px;
		flex-shrink: 0;
	}

	.legend-content {
		flex: 1;
	}

	.legend-header {
		display: flex;
		justify-content: space-between;
		align-items: baseline;
		margin-bottom: 4px;
	}

	.setup-name {
		font-weight: 600;
		color: var(--color-text-primary);
		font-size: 13px;
	}

	.setup-count {
		font-size: 11px;
		color: var(--color-text-tertiary);
	}

	.setup-stats {
		display: flex;
		gap: 12px;
	}

	.stat {
		font-size: 11px;
		color: var(--color-text-secondary);
		font-variant-numeric: tabular-nums;
	}

	.stat.profit {
		color: var(--color-profit);
	}
	.stat.loss {
		color: var(--color-loss);
	}

	/* Skeleton */
	.skeleton-content {
		display: flex;
		gap: 24px;
		align-items: center;
	}

	.skel-pie {
		width: 160px;
		height: 160px;
		border-radius: 50%;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skel-legend {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.skel-item {
		height: 40px;
		border-radius: 8px;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-bg-muted) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.empty-state {
		height: 160px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-tertiary);
		font-size: 14px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	@media (max-width: 640px) {
		.setup-breakdown {
			padding: 16px;
		}

		.content-wrapper {
			flex-direction: column;
			align-items: center;
		}

		.legend-section {
			width: 100%;
		}
	}
</style>

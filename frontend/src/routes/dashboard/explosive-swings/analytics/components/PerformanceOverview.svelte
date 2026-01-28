<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * PerformanceOverview Component - Key Performance Metrics Grid
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 2.0.0 - Phase 4: Analytics Dashboard Update
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5 January 2026
	 */
	import MetricCard from './MetricCard.svelte';
	import type { AnalyticsSummary, StreakAnalysis, AlertEffectiveness } from '../analytics.state.svelte';

	interface Props {
		summary: AnalyticsSummary;
		streakAnalysis?: StreakAnalysis | null;
		alertEffectiveness?: AlertEffectiveness | null;
		isLoading?: boolean;
	}

	const {
		summary,
		streakAnalysis = null,
		alertEffectiveness = null,
		isLoading = false
	}: Props = $props();

	// Format helpers
	function formatPercent(value: number, decimals: number = 1): string {
		return `${value >= 0 ? '+' : ''}${value.toFixed(decimals)}%`;
	}

	function formatRatio(value: number): string {
		if (!isFinite(value) || value > 100) return '99+';
		return value.toFixed(2);
	}

	// Calculate wins and losses from summary
	const totalTrades = $derived(summary.total_trades);
	const wins = $derived(Math.round(totalTrades * summary.win_rate / 100));
	const losses = $derived(totalTrades - wins);

	// Metric card definitions
	const metrics = $derived([
		{
			label: 'Win Rate',
			value: `${summary.win_rate.toFixed(1)}%`,
			subtext: `${wins}W / ${losses}L`,
			color: summary.win_rate >= 50 ? 'profit' : 'loss',
			icon: 'target'
		},
		{
			label: 'Profit Factor',
			value: formatRatio(summary.profit_factor),
			subtext: summary.profit_factor >= 1.5 ? 'Excellent' : summary.profit_factor >= 1 ? 'Good' : 'Needs Work',
			color: summary.profit_factor >= 1.5 ? 'profit' : summary.profit_factor >= 1 ? 'neutral' : 'loss',
			icon: 'scale'
		},
		{
			label: 'Total P&L',
			value: formatPercent(summary.total_pnl_percent),
			subtext: `$${summary.total_pnl.toFixed(2)}`,
			color: summary.total_pnl_percent >= 0 ? 'profit' : 'loss',
			icon: 'chart'
		},
		{
			label: 'Sharpe Ratio',
			value: summary.sharpe_ratio.toFixed(2),
			subtext: summary.sharpe_ratio >= 2 ? 'Excellent' : summary.sharpe_ratio >= 1 ? 'Good' : 'Below avg',
			color: summary.sharpe_ratio >= 2 ? 'profit' : summary.sharpe_ratio >= 1 ? 'neutral' : 'loss',
			icon: 'chart'
		},
		{
			label: 'Avg Win',
			value: formatPercent(summary.avg_win_percent),
			subtext: `Max: ${formatPercent(summary.largest_win_percent)}`,
			color: 'profit',
			icon: 'trending-up'
		},
		{
			label: 'Avg Loss',
			value: formatPercent(-Math.abs(summary.avg_loss_percent)),
			subtext: `Max: ${formatPercent(-Math.abs(summary.largest_loss_percent))}`,
			color: 'loss',
			icon: 'trending-down'
		},
		{
			label: 'Expectancy',
			value: formatPercent(summary.expectancy),
			subtext: 'Per trade average',
			color: summary.expectancy >= 0 ? 'profit' : 'loss',
			icon: 'calculator'
		},
		{
			label: 'Max Drawdown',
			value: formatPercent(-Math.abs(summary.max_drawdown_percent)),
			subtext: 'Peak to trough',
			color: 'loss',
			icon: 'arrow-down'
		},
		{
			label: 'Risk/Reward',
			value: `${summary.risk_reward_ratio.toFixed(2)}:1`,
			subtext: 'Avg win / Avg loss',
			color: summary.risk_reward_ratio >= 1.5 ? 'profit' : 'neutral',
			icon: 'balance'
		},
		{
			label: 'Avg Hold Time',
			value: `${summary.avg_holding_days.toFixed(1)} days`,
			subtext: 'Per trade',
			color: 'neutral',
			icon: 'clock'
		}
	]);

	// Streak display
	const streakDisplay = $derived(
		streakAnalysis
			? `${streakAnalysis.current_streak} ${streakAnalysis.current_streak_type}`
			: 'N/A'
	);

	const streakColor = $derived(
		streakAnalysis?.current_streak_type === 'WIN' ? 'profit' :
		streakAnalysis?.current_streak_type === 'LOSS' ? 'loss' : 'neutral'
	);
</script>

<div class="performance-overview" role="region" aria-label="Performance metrics overview">
	<div class="header">
		<h2 class="section-title">Key Metrics</h2>
		<span class="trade-count">{totalTrades} trades analyzed</span>
	</div>

	<div class="metrics-grid">
		{#each metrics as metric}
			<div class="metric-card" class:loading={isLoading}>
				{#if isLoading}
					<div class="skeleton">
						<div class="skel-icon"></div>
						<div class="skel-value"></div>
						<div class="skel-label"></div>
					</div>
				{:else}
					<div class="metric-icon {metric.color}">
						{#if metric.icon === 'target'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<circle cx="12" cy="12" r="10"/>
								<circle cx="12" cy="12" r="6"/>
								<circle cx="12" cy="12" r="2"/>
							</svg>
						{:else if metric.icon === 'scale'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M16 16l3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1z"/>
								<path d="M2 16l3-8 3 8c-.87.65-1.92 1-3 1s-2.13-.35-3-1z"/>
								<path d="M7 21h10"/><path d="M12 3v18"/>
								<path d="M3 7h2c2 0 5-1 7-2 2 1 5 2 7 2h2"/>
							</svg>
						{:else if metric.icon === 'chart'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M3 3v18h18"/><path d="M18.7 8l-5.1 5.2-2.8-2.7L7 14.3"/>
							</svg>
						{:else if metric.icon === 'trending-up'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<polyline points="23 6 13.5 15.5 8.5 10.5 1 18"/>
								<polyline points="17 6 23 6 23 12"/>
							</svg>
						{:else if metric.icon === 'trending-down'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<polyline points="23 18 13.5 8.5 8.5 13.5 1 6"/>
								<polyline points="17 18 23 18 23 12"/>
							</svg>
						{:else if metric.icon === 'calculator'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<rect x="4" y="2" width="16" height="20" rx="2"/>
								<line x1="8" y1="6" x2="16" y2="6"/>
								<line x1="8" y1="10" x2="16" y2="10"/>
							</svg>
						{:else if metric.icon === 'arrow-down'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<line x1="12" y1="5" x2="12" y2="19"/>
								<polyline points="19 12 12 19 5 12"/>
							</svg>
						{:else if metric.icon === 'balance'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<path d="M12 3v18"/><rect x="3" y="8" width="6" height="4"/><rect x="15" y="12" width="6" height="4"/>
							</svg>
						{:else if metric.icon === 'clock'}
							<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
							</svg>
						{/if}
					</div>

					<div class="metric-value {metric.color}">{metric.value}</div>
					<div class="metric-label">{metric.label}</div>
					<div class="metric-subtext">{metric.subtext}</div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- Streak and Alert Effectiveness Row -->
	{#if streakAnalysis || alertEffectiveness}
		<div class="additional-metrics">
			{#if streakAnalysis}
				<div class="streak-card {streakColor}">
					<span class="streak-label">Current Streak</span>
					<span class="streak-value">{streakDisplay}</span>
					<span class="streak-subtext">Max: {streakAnalysis.max_win_streak}W / {streakAnalysis.max_loss_streak}L</span>
				</div>
			{/if}

			{#if alertEffectiveness}
				<div class="alert-card">
					<span class="alert-label">Alert Conversion</span>
					<span class="alert-value">{alertEffectiveness.conversion_rate.toFixed(1)}%</span>
					<span class="alert-subtext">{alertEffectiveness.alerts_with_trades} of {alertEffectiveness.total_alerts} alerts</span>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.performance-overview {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		padding: 20px;
		box-shadow: var(--shadow-sm);
	}

	.header {
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		margin-bottom: 16px;
	}

	.section-title {
		font-size: 16px;
		font-weight: 700;
		color: var(--color-text-primary);
		margin: 0;
	}

	.trade-count {
		font-size: 12px;
		color: var(--color-text-tertiary);
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 12px;
	}

	.metric-card {
		background: var(--color-bg-subtle);
		border: 1px solid var(--color-border-subtle);
		border-radius: 10px;
		padding: 16px;
		text-align: center;
		transition: var(--transition-shadow);
	}

	.metric-card:hover {
		box-shadow: var(--shadow-sm);
	}

	.metric-card.loading {
		min-height: 110px;
	}

	.metric-icon {
		width: 36px;
		height: 36px;
		margin: 0 auto 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
		background: var(--color-bg-muted);
	}

	.metric-icon.profit {
		background: var(--color-profit-bg);
		color: var(--color-profit);
	}

	.metric-icon.loss {
		background: var(--color-loss-bg);
		color: var(--color-loss);
	}

	.metric-icon.neutral {
		background: var(--color-bg-muted);
		color: var(--color-text-secondary);
	}

	.metric-value {
		font-size: 22px;
		font-weight: 700;
		font-variant-numeric: tabular-nums;
		margin-bottom: 4px;
		line-height: 1.2;
	}

	.metric-value.profit { color: var(--color-profit); }
	.metric-value.loss { color: var(--color-loss); }
	.metric-value.neutral { color: var(--color-text-primary); }

	.metric-label {
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 2px;
	}

	.metric-subtext {
		font-size: 11px;
		color: var(--color-text-tertiary);
		font-variant-numeric: tabular-nums;
	}

	/* Additional Metrics Row */
	.additional-metrics {
		display: flex;
		gap: 12px;
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid var(--color-border-default);
	}

	.streak-card,
	.alert-card {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 4px;
		padding: 12px;
		border-radius: 8px;
		border: 1px solid var(--color-border-default);
	}

	.streak-card.profit { background: var(--color-profit-bg); border-color: var(--color-profit-border); }
	.streak-card.loss { background: var(--color-loss-bg); border-color: var(--color-loss-border); }
	.streak-card.neutral { background: var(--color-bg-subtle); }

	.streak-label,
	.alert-label {
		font-size: 11px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.streak-value,
	.alert-value {
		font-size: 18px;
		font-weight: 700;
		color: var(--color-text-primary);
	}

	.streak-card.profit .streak-value { color: var(--color-profit); }
	.streak-card.loss .streak-value { color: var(--color-loss); }

	.streak-subtext,
	.alert-subtext {
		font-size: 11px;
		color: var(--color-text-tertiary);
	}

	/* Skeleton Loading */
	.skeleton {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
	}

	.skel-icon {
		width: 36px;
		height: 36px;
		border-radius: 8px;
		background: linear-gradient(90deg, var(--color-bg-muted) 25%, var(--color-bg-subtle) 50%, var(--color-bg-muted) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skel-value {
		width: 60%;
		height: 24px;
		border-radius: 4px;
		background: linear-gradient(90deg, var(--color-bg-muted) 25%, var(--color-bg-subtle) 50%, var(--color-bg-muted) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skel-label {
		width: 80%;
		height: 14px;
		border-radius: 4px;
		background: linear-gradient(90deg, var(--color-bg-muted) 25%, var(--color-bg-subtle) 50%, var(--color-bg-muted) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.metrics-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	@media (max-width: 1024px) {
		.metrics-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: 768px) {
		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.additional-metrics {
			flex-direction: column;
		}
	}

	@media (max-width: 640px) {
		.performance-overview {
			padding: 16px;
		}

		.metrics-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 8px;
		}

		.metric-card {
			padding: 12px;
		}

		.metric-value {
			font-size: 18px;
		}
	}
</style>

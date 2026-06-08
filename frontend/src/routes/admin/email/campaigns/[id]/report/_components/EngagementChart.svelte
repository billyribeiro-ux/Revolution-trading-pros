<!--
	EngagementChart — bar chart of opens/clicks over first 48 hours.
	Extracted from +page.svelte (R22-C) — small local state (view toggle).
-->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconTrendingUp } from '$lib/icons';

	interface EngagementPoint {
		hour: number;
		opens: number;
		clicks: number;
	}

	interface Props {
		engagementOverTime: EngagementPoint[];
		maxEngagement: number;
		formatHour: (hour: number) => string;
	}

	const { engagementOverTime, maxEngagement, formatHour }: Props = $props();

	let activeChartView = $state<'opens' | 'clicks'>('opens');
</script>

<section class="glass-panel engagement-panel" in:fly={{ x: -20, duration: 500, delay: 200 }}>
	<div class="panel-header">
		<div class="panel-title">
			<div class="panel-icon gold">
				<IconTrendingUp size={24} />
			</div>
			<div>
				<h3>Engagement Over Time</h3>
				<span class="panel-subtitle">First 48 hours after send</span>
			</div>
		</div>
		<div class="chart-toggle">
			<button
				class={{ active: activeChartView === 'opens' }}
				onclick={() => (activeChartView = 'opens')}
			>
				Opens
			</button>
			<button
				class={{ active: activeChartView === 'clicks' }}
				onclick={() => (activeChartView = 'clicks')}
			>
				Clicks
			</button>
		</div>
	</div>

	<div class="chart-container">
		<div class="bar-chart">
			{#each engagementOverTime.slice(0, 24) as item, i (item.hour)}
				{const value = activeChartView === 'opens' ? item.opens : item.clicks}
				{const height = (value / maxEngagement) * 100}
				<div class="bar-wrapper" title={`${formatHour(item.hour)}: ${value} ${activeChartView}`}>
					<div class={['bar', activeChartView]} style:height={`${Math.max(height, 2)}%`}></div>
					{#if i % 4 === 0}
						<span class="bar-label">{formatHour(item.hour)}</span>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	.glass-panel {
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: 20px;
		padding: 1.75rem;
		backdrop-filter: blur(20px);
		box-shadow: var(--admin-card-shadow, 0 4px 20px rgba(0, 0, 0, 0.4));
		position: relative;
		overflow: hidden;
	}

	.glass-panel::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 1px;
		background: linear-gradient(
			90deg,
			transparent,
			var(--admin-accent-primary-soft, rgba(230, 184, 0, 0.15)),
			transparent
		);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.panel-title h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.panel-subtitle {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	.panel-icon {
		width: 44px;
		height: 44px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.panel-icon.gold {
		background: var(--admin-widget-gold-bg, rgba(230, 184, 0, 0.15));
		color: var(--admin-widget-gold-icon, var(--primary-400));
	}

	.chart-toggle {
		display: flex;
		background: var(--admin-bg-muted, var(--bg-surface));
		border-radius: 10px;
		padding: 4px;
		border: 1px solid var(--admin-border, var(--border-default));
	}

	.chart-toggle button {
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		color: var(--text-tertiary);
		font-size: 0.8125rem;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.chart-toggle button:hover {
		color: var(--text-primary);
	}

	.chart-toggle button.active {
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.chart-container {
		margin-top: 1rem;
	}

	.bar-chart {
		display: flex;
		align-items: flex-end;
		height: 160px;
		gap: 3px;
		padding: 0 0.5rem;
	}

	.bar-wrapper {
		flex: 1;
		height: 100%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: flex-end;
		position: relative;
	}

	.bar {
		width: 100%;
		border-radius: 3px 3px 0 0;
		transition: height 0.5s ease-out;
		cursor: pointer;
	}

	.bar.opens {
		background: linear-gradient(180deg, var(--primary-500), var(--admin-warning, #bb8009));
	}

	.bar.clicks {
		background: linear-gradient(
			180deg,
			var(--admin-info, #388bfd),
			var(--admin-accent-secondary, var(--secondary-500))
		);
	}

	.bar:hover {
		opacity: 0.8;
	}

	.bar-label {
		font-size: 0.625rem;
		color: var(--text-tertiary);
		margin-top: 0.5rem;
		white-space: nowrap;
	}

	@media (prefers-reduced-motion: reduce) {
		.bar {
			transition: none;
		}
	}
</style>

<!--
	EngagementQuality — click-to-open rate panel with key performance indicators.
	Extracted from +page.svelte (R22-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconChartBar } from '$lib/icons';

	interface Props {
		clickToOpenRate: number;
		opened: number;
		clicked: number;
		formatNumber: (num: number) => string;
	}

	const { clickToOpenRate, opened, clicked, formatNumber }: Props = $props();
</script>

<section class="glass-panel ctor-panel" in:fly={{ x: 20, duration: 500, delay: 200 }}>
	<div class="panel-header">
		<div class="panel-title">
			<div class="panel-icon blue">
				<IconChartBar size={24} />
			</div>
			<div>
				<h3>Engagement Quality</h3>
				<span class="panel-subtitle">Key performance indicators</span>
			</div>
		</div>
	</div>

	<div class="ctor-metrics">
		<div class="ctor-item primary">
			<span class="ctor-label">Click-to-Open Rate</span>
			<span class="ctor-value">{clickToOpenRate.toFixed(1)}%</span>
			<div class="ctor-bar">
				<div class="ctor-bar-fill" style:width={`${Math.min(clickToOpenRate, 100)}%`}></div>
			</div>
			<span class="ctor-desc">Industry avg: 10-15%</span>
		</div>

		<div class="ctor-stats">
			<div class="ctor-stat">
				<span class="stat-value">{formatNumber(opened)}</span>
				<span class="stat-label">Total Opens</span>
			</div>
			<div class="ctor-stat">
				<span class="stat-value">{formatNumber(clicked)}</span>
				<span class="stat-label">Total Clicks</span>
			</div>
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

	.panel-icon.blue {
		background: var(--admin-widget-blue-bg, rgba(56, 139, 253, 0.15));
		color: var(--admin-widget-blue-icon, #58a6ff);
	}

	.ctor-metrics {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.ctor-item.primary {
		text-align: center;
		padding: 1.5rem;
		background: var(--bg-elevated);
		border-radius: 16px;
		border: 1px solid var(--border-muted);
	}

	.ctor-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		display: block;
		margin-bottom: 0.5rem;
	}

	.ctor-value {
		font-size: 2.5rem;
		font-weight: 800;
		color: var(--primary-500);
		display: block;
	}

	.ctor-bar {
		height: 8px;
		background: var(--bg-hover);
		border-radius: 4px;
		margin: 1rem 0 0.5rem;
		overflow: hidden;
	}

	.ctor-bar-fill {
		height: 100%;
		background: linear-gradient(
			90deg,
			var(--primary-500),
			var(--admin-accent-tertiary, var(--primary-400))
		);
		border-radius: 4px;
		transition: width 1s ease-out;
	}

	.ctor-desc {
		font-size: 0.75rem;
		color: var(--text-tertiary);
	}

	.ctor-stats {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.ctor-stat {
		text-align: center;
		padding: 1rem;
		background: var(--bg-elevated);
		border-radius: 12px;
		border: 1px solid var(--border-muted);
	}

	.ctor-stat .stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		display: block;
	}

	.ctor-stat .stat-label {
		font-size: 0.75rem;
		color: var(--text-tertiary);
	}

	@media (max-width: 479.98px) {
		.ctor-value {
			font-size: 2rem;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.ctor-bar-fill {
			transition: none;
		}
	}
</style>

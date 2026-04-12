<!--
	AI Insights Panel Component
	═══════════════════════════════════════════════════════════════════════════
	
	Displays AI-powered insights, anomaly detection, predictions,
	and actionable recommendations based on analytics data.
-->

<script lang="ts">
	import {
		Icon,
		IconAlertTriangle,
		IconBrain,
		IconBulb,
		IconTarget,
		IconTrendingUp
	} from '$lib/icons';

	interface Insight {
		id: string;
		type: 'anomaly' | 'prediction' | 'recommendation' | 'trend';
		severity: 'critical' | 'warning' | 'info';
		title: string;
		description: string;
		metric?: string;
		value?: number;
		change?: number;
		action?: string;
		timestamp: string;
	}

	interface Props {
		insights?: Insight[];
	}

	let { insights = [] }: Props = $props();

	function getSeverityClass(severity: string): string {
		switch (severity) {
			case 'critical':
				return 'severity-critical';
			case 'warning':
				return 'severity-warning';
			case 'info':
				return 'severity-info';
			default:
				return '';
		}
	}

	function getIcon(type: string) {
		switch (type) {
			case 'anomaly':
				return IconAlertTriangle;
			case 'prediction':
				return IconTrendingUp;
			case 'recommendation':
				return IconBulb;
			case 'trend':
				return IconTarget;
			default:
				return IconBrain;
		}
	}
</script>

<div class="insights-panel">
	<div class="panel-header">
		<div class="header-title">
			<Icon icon={IconBrain} size={24} />
			<h3 class="title">AI Insights</h3>
		</div>
		<div class="insights-count">{insights.length} insights</div>
	</div>

	<div class="insights-list">
		{#if insights.length > 0}
			{#each insights as insight (insight.id)}
				<div class="insight-card {getSeverityClass(insight.severity)}">
					<div class="insight-icon">
						<Icon icon={getIcon(insight.type)} size={20} />
					</div>

					<div class="insight-content">
						<div class="insight-header">
							<h4 class="insight-title">{insight.title}</h4>
							<span class="insight-type">{insight.type}</span>
						</div>

						<p class="insight-description">{insight.description}</p>

						{#if insight.metric}
							<div class="insight-metrics">
								<div class="metric">
									<span class="metric-label">{insight.metric}</span>
									{#if insight.value !== undefined}
										<span class="metric-value">{insight.value.toLocaleString()}</span>
									{/if}
									{#if insight.change !== undefined}
										<span
											class="metric-change"
											class:positive={insight.change > 0}
											class:negative={insight.change < 0}
										>
											{insight.change > 0 ? '+' : ''}{insight.change}%
										</span>
									{/if}
								</div>
							</div>
						{/if}

						{#if insight.action}
							<button class="insight-action">
								{insight.action}
							</button>
						{/if}

						<div class="insight-footer">
							<span class="insight-timestamp">{new Date(insight.timestamp).toLocaleString()}</span>
						</div>
					</div>
				</div>
			{/each}
		{:else}
			<div class="empty-state">
				<Icon icon={IconBrain} size={48} />
				<p class="empty-primary">No insights available yet</p>
				<p class="empty-secondary">AI insights will appear as data is analyzed</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.insights-panel {
		background-color: oklch(0.2 0.02 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.35 0.02 250 / 50%);
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		color: oklch(0.7 0.18 300);
	}

	.title {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.insights-count {
		padding-inline: var(--space-3);
		padding-block: var(--space-1);
		background-color: oklch(0.55 0.2 300 / 20%);
		color: oklch(0.7 0.18 300);
		border-radius: 9999px;
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
	}

	.insights-list {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.insight-card {
		display: flex;
		gap: var(--space-4);
		padding: var(--space-4);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		background-color: oklch(0.15 0.01 250 / 50%);
		transition: border-color var(--duration-fast) var(--ease-default);

		&:hover {
			border-color: oklch(0.45 0.01 250);
		}
		&[data-severity='critical'] {
			border-color: oklch(0.55 0.22 25 / 50%);
			background-color: oklch(0.55 0.22 25 / 5%);
		}
		&[data-severity='warning'] {
			border-color: oklch(0.7 0.18 55 / 50%);
			background-color: oklch(0.7 0.18 55 / 5%);
		}
		&[data-severity='info'] {
			border-color: oklch(0.6 0.2 260 / 50%);
			background-color: oklch(0.6 0.2 260 / 5%);
		}
	}

	.insight-icon {
		flex-shrink: 0;
		inline-size: 2.5rem;
		block-size: 2.5rem;
		border-radius: var(--radius-lg);
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: oklch(0.55 0.2 300 / 20%);
		color: oklch(0.7 0.18 300);
	}

	[data-severity='critical'] .insight-icon {
		background-color: oklch(0.55 0.22 25 / 20%);
		color: oklch(0.7 0.2 25);
	}
	[data-severity='warning'] .insight-icon {
		background-color: oklch(0.7 0.18 55 / 20%);
		color: oklch(0.75 0.16 55);
	}
	[data-severity='info'] .insight-icon {
		background-color: oklch(0.6 0.2 260 / 20%);
		color: oklch(0.7 0.18 260);
	}

	.insight-content {
		flex: 1;
	}

	.insight-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-2);
		margin-block-end: var(--space-2);
	}

	.insight-title {
		font-size: var(--text-base);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
	}

	.insight-type {
		padding-inline: var(--space-2);
		padding-block: var(--space-1);
		background-color: oklch(0.38 0.01 250 / 50%);
		color: oklch(0.65 0.01 250);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		text-transform: uppercase;
	}

	.insight-description {
		font-size: var(--text-sm);
		color: oklch(0.75 0.01 250);
		margin-block-end: var(--space-3);
	}

	.insight-metrics {
		margin-block-end: var(--space-3);
	}

	.metric {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		font-size: var(--text-sm);
	}

	.metric-label {
		color: oklch(0.65 0.01 250);
	}
	.metric-value {
		color: oklch(1 0 0);
		font-weight: var(--weight-semibold);
	}

	.metric-change {
		font-weight: var(--weight-semibold);
		&.positive {
			color: oklch(0.7 0.18 160);
		}
		&.negative {
			color: oklch(0.7 0.2 25);
		}
	}

	.insight-action {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.55 0.2 300 / 20%);
		color: oklch(0.7 0.18 300);
		border-radius: var(--radius-lg);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.55 0.2 300 / 30%);
		}
	}

	.insight-footer {
		margin-block-start: var(--space-3);
		padding-block-start: var(--space-3);
		border-block-start: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.insight-timestamp {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: var(--space-12);
		text-align: center;
		color: oklch(0.45 0.01 250);
	}

	.empty-primary {
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-3);
	}

	.empty-secondary {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 250);
		margin-block-start: var(--space-1);
	}
</style>

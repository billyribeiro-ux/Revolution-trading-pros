<!--
	AI Insights Panel Component
	═══════════════════════════════════════════════════════════════════════════
	
	Displays AI-powered insights, anomaly detection, predictions,
	and actionable recommendations based on analytics data.
-->

<script lang="ts">
	import {
		IconBrain,
		IconTrendingUp,
		IconAlertTriangle,
		IconBulb,
		IconTarget
	} from '#lib/icons/index.js';

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

	function formatTimestamp(timestamp: string): string {
		return new Date(timestamp).toLocaleString();
	}
</script>

<div class="insights-panel">
	<div class="panel-header">
		<div class="header-title">
			<IconBrain size={24} class="panel-icon" />
			<h3 class="title">AI Insights</h3>
		</div>
		<div class="insights-count">{insights.length} insights</div>
	</div>

	<div class="insights-list">
		{#if insights.length > 0}
			{#each insights as insight (insight.id)}
				{const Icon = getIcon(insight.type)}
				<div class={['insight-card', getSeverityClass(insight.severity)]}>
					<div class="insight-icon">
						<Icon size={20} />
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
											class={[
												'metric-change',
												{
													positive: insight.change > 0,
													negative: insight.change < 0
												}
											]}
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
							<span class="insight-timestamp">{formatTimestamp(insight.timestamp)}</span>
						</div>
					</div>
				</div>
			{/each}
		{:else}
			<div class="empty-state">
				<IconBrain size={48} class="empty-icon" />
				<p class="empty-title">No insights available yet</p>
				<p class="empty-copy">AI insights will appear as data is analyzed</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.insights-panel {
		background-color: var(--bg-elevated, #161b22);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid var(--border-default, #30363d);
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.header-title :global(.panel-icon) {
		color: #c084fc;
	}

	.title {
		margin: 0;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 700;
		color: #ffffff;
	}

	.insights-count {
		padding: 0.25rem 0.75rem;
		border-radius: 999px;
		background: rgba(230, 184, 0, 0.2);
		color: #c084fc;
		font-size: 0.875rem;
		font-weight: 600;
		white-space: nowrap;
	}

	.insights-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.insight-card {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: var(--bg-base, #0d1117);
		transition: border-color 0.2s ease;
	}

	.insight-card:hover {
		border-color: var(--border-default, #30363d);
	}

	.insight-card.severity-critical {
		border-color: rgba(248, 81, 73, 0.5);
		background: rgba(248, 81, 73, 0.05);
	}

	.insight-card.severity-warning {
		border-color: rgba(210, 153, 34, 0.5);
		background: rgba(210, 153, 34, 0.05);
	}

	.insight-card.severity-info {
		border-color: rgba(88, 166, 255, 0.5);
		background: rgba(88, 166, 255, 0.05);
	}

	.insight-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.5rem;
		background: rgba(230, 184, 0, 0.2);
		color: #c084fc;
	}

	.severity-critical .insight-icon {
		background: rgba(248, 81, 73, 0.2);
		color: var(--error-emphasis, #f85149);
	}

	.severity-warning .insight-icon {
		background: rgba(210, 153, 34, 0.2);
		color: var(--warning-emphasis, #d29922);
	}

	.severity-info .insight-icon {
		background: rgba(88, 166, 255, 0.2);
		color: var(--info-emphasis, #58a6ff);
	}

	.insight-content {
		flex: 1;
		min-width: 0;
	}

	.insight-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.insight-title {
		margin: 0;
		font-size: 1rem;
		line-height: 1.5rem;
		font-weight: 600;
		color: #ffffff;
	}

	.insight-type {
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		background: rgba(55, 65, 81, 0.5);
		color: var(--text-secondary, #8b949e);
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: uppercase;
		white-space: nowrap;
	}

	.insight-description {
		margin: 0 0 0.75rem;
		font-size: 0.875rem;
		color: var(--text-secondary, #8b949e);
	}

	.insight-metrics {
		margin-bottom: 0.75rem;
	}

	.metric {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.875rem;
	}

	.metric-label {
		color: var(--text-secondary, #8b949e);
	}

	.metric-value {
		color: #ffffff;
		font-weight: 600;
	}

	.metric-change {
		font-weight: 600;
	}

	.metric-change.positive {
		color: var(--success-emphasis, #3fb950);
	}

	.metric-change.negative {
		color: var(--error-emphasis, #f85149);
	}

	.insight-action {
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: rgba(230, 184, 0, 0.2);
		color: #c084fc;
		font: inherit;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.insight-action:hover {
		background: rgba(230, 184, 0, 0.3);
	}

	.insight-footer {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(55, 65, 81, 0.5);
	}

	.insight-timestamp {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 3rem;
		text-align: center;
	}

	.empty-state :global(.empty-icon) {
		color: var(--text-tertiary, #6e7681);
	}

	.empty-title {
		margin: 0.75rem 0 0;
		color: var(--text-secondary, #8b949e);
	}

	.empty-copy {
		margin: 0.25rem 0 0;
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
	}

	@media (max-width: 640px) {
		.panel-header,
		.insight-header {
			align-items: flex-start;
			flex-direction: column;
		}
	}
</style>

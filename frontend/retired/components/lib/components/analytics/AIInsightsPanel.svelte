<!--
	AI Insights Panel Component
	═══════════════════════════════════════════════════════════════════════════
	
	Displays AI-powered insights, anomaly detection, predictions,
	and actionable recommendations based on analytics data.
-->

<script lang="ts">
	import { IconBrain, IconTrendingUp, IconAlertTriangle, IconBulb, IconTarget } from '$lib/icons';

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
</script>

<div class="insights-panel">
	<div class="panel-header">
		<div class="header-title">
			<IconBrain size={24} class="text-purple-400" />
			<h3 class="title">AI Insights</h3>
		</div>
		<div class="insights-count">{insights.length} insights</div>
	</div>

	<div class="insights-list">
		{#if insights.length > 0}
			{#each insights as insight}
				{@const Icon = getIcon(insight.type)}
				<div class="insight-card {getSeverityClass(insight.severity)}">
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
				<IconBrain size={48} class="text-gray-600" />
				<p class="text-gray-400 mt-3">No insights available yet</p>
				<p class="text-sm text-gray-500 mt-1">AI insights will appear as data is analyzed</p>
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	@reference "../../../app.css";
	.insights-panel {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.panel-header {
		@apply flex items-center justify-between mb-6;
	}

	.header-title {
		@apply flex items-center gap-3;
	}

	.title {
		@apply text-xl font-bold text-white;
	}

	.insights-count {
		@apply px-3 py-1 bg-purple-500/20 text-purple-400 rounded-full text-sm font-semibold;
	}

	.insights-list {
		@apply space-y-4;
	}

	.insight-card {
		@apply flex gap-4 p-4 rounded-lg border transition-all;
		@apply bg-gray-900/50 border-gray-700/50 hover:border-gray-600;
	}

	.insight-card.severity-critical {
		@apply border-red-500/50 bg-red-500/5;
	}

	.insight-card.severity-warning {
		@apply border-orange-500/50 bg-orange-500/5;
	}

	.insight-card.severity-info {
		@apply border-blue-500/50 bg-blue-500/5;
	}

	.insight-icon {
		@apply flex-shrink-0 w-10 h-10 rounded-lg flex items-center justify-center;
		@apply bg-purple-500/20 text-purple-400;
	}

	.severity-critical .insight-icon {
		@apply bg-red-500/20 text-red-400;
	}

	.severity-warning .insight-icon {
		@apply bg-orange-500/20 text-orange-400;
	}

	.severity-info .insight-icon {
		@apply bg-blue-500/20 text-blue-400;
	}

	.insight-content {
		@apply flex-1;
	}

	.insight-header {
		@apply flex items-start justify-between gap-2 mb-2;
	}

	.insight-title {
		@apply text-base font-semibold text-white;
	}

	.insight-type {
		@apply px-2 py-1 bg-gray-700/50 text-gray-400 rounded text-xs font-medium uppercase;
	}

	.insight-description {
		@apply text-sm text-gray-300 mb-3;
	}

	.insight-metrics {
		@apply mb-3;
	}

	.metric {
		@apply flex items-center gap-3 text-sm;
	}

	.metric-label {
		@apply text-gray-400;
	}

	.metric-value {
		@apply text-white font-semibold;
	}

	.metric-change {
		@apply font-semibold;
	}

	.metric-change.positive {
		@apply text-green-400;
	}

	.metric-change.negative {
		@apply text-red-400;
	}

	.insight-action {
		@apply px-4 py-2 bg-purple-500/20 text-purple-400 rounded-lg text-sm font-medium;
		@apply hover:bg-purple-500/30 transition-colors;
	}

	.insight-footer {
		@apply mt-3 pt-3 border-t border-gray-700/50;
	}

	.insight-timestamp {
		@apply text-xs text-gray-500;
	}

	.empty-state {
		@apply flex flex-col items-center justify-center py-12 text-center;
	}
</style>

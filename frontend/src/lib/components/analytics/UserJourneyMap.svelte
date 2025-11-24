<!--
	User Journey Map Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user paths through the platform with conversion points and drop-offs.
-->

<script lang="ts">
	import { IconArrowRight, IconX, IconCheck } from '@tabler/icons-svelte';

	export let journeyData: Array<{
		step: string;
		users: number;
		conversion_rate: number;
		avg_time: number;
		drop_off: number;
		top_actions: Array<{ action: string; count: number }>;
	}> = [];

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatTime(seconds: number): string {
		if (seconds < 60) return `${Math.round(seconds)}s`;
		if (seconds < 3600) return `${Math.round(seconds / 60)}m`;
		return `${Math.round(seconds / 3600)}h`;
	}
</script>

<div class="journey-map">
	<div class="journey-header">
		<h3 class="journey-title">User Journey Map</h3>
		<div class="journey-stats">
			<span class="stat-item">
				{formatNumber(journeyData[0]?.users || 0)} users started
			</span>
			<span class="stat-item">
				{((journeyData[journeyData.length - 1]?.users / journeyData[0]?.users) * 100).toFixed(1)}%
				completed
			</span>
		</div>
	</div>

	<div class="journey-flow">
		{#each journeyData as step, index}
			<div class="journey-step">
				<div class="step-card">
					<div class="step-header">
						<div class="step-number">{index + 1}</div>
						<div class="step-info">
							<div class="step-name">{step.step}</div>
							<div class="step-users">{formatNumber(step.users)} users</div>
						</div>
						<div class="step-badge">
							{step.conversion_rate.toFixed(1)}%
						</div>
					</div>

					<div class="step-metrics">
						<div class="metric">
							<span class="metric-label">Avg Time</span>
							<span class="metric-value">{formatTime(step.avg_time)}</span>
						</div>
						{#if step.drop_off > 0}
							<div class="metric drop-off">
								<span class="metric-label">Drop-off</span>
								<span class="metric-value">{formatNumber(step.drop_off)}</span>
							</div>
						{/if}
					</div>

					{#if step.top_actions.length > 0}
						<div class="step-actions">
							<div class="actions-label">Top Actions:</div>
							{#each step.top_actions.slice(0, 3) as action}
								<div class="action-item">
									<span class="action-name">{action.action}</span>
									<span class="action-count">{formatNumber(action.count)}</span>
								</div>
							{/each}
						</div>
					{/if}
				</div>

				{#if index < journeyData.length - 1}
					<div class="step-connector">
						<div class="connector-line"></div>
						<div class="connector-stats">
							<IconArrowRight size={20} class="text-gray-400" />
							<span class="conversion-rate">
								{journeyData[index + 1].conversion_rate.toFixed(1)}%
							</span>
						</div>
						{#if step.drop_off > 0}
							<div class="drop-off-indicator">
								<IconX size={16} />
								{formatNumber(step.drop_off)} dropped
							</div>
						{/if}
					</div>
				{:else}
					<div class="completion-badge">
						<IconCheck size={24} class="text-green-400" />
						<span>Journey Complete</span>
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<style lang="postcss">
	@reference "../../../app.css";
	.journey-map {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.journey-header {
		@apply flex items-center justify-between mb-6;
	}

	.journey-title {
		@apply text-xl font-bold text-white;
	}

	.journey-stats {
		@apply flex items-center gap-4;
	}

	.stat-item {
		@apply text-sm text-gray-400;
	}

	.journey-flow {
		@apply space-y-0;
	}

	.journey-step {
		@apply relative;
	}

	.step-card {
		@apply bg-gray-900/50 rounded-lg p-4 border border-gray-700/50;
		@apply hover:border-yellow-500/30 transition-colors;
	}

	.step-header {
		@apply flex items-start gap-3 mb-3;
	}

	.step-number {
		@apply w-8 h-8 rounded-full bg-yellow-500 text-gray-900 font-bold;
		@apply flex items-center justify-center flex-shrink-0;
	}

	.step-info {
		@apply flex-1;
	}

	.step-name {
		@apply text-white font-semibold mb-1;
	}

	.step-users {
		@apply text-sm text-gray-400;
	}

	.step-badge {
		@apply px-3 py-1 bg-green-500/20 text-green-400 rounded-lg text-sm font-semibold;
	}

	.step-metrics {
		@apply flex gap-4 mb-3 pb-3 border-b border-gray-700/50;
	}

	.metric {
		@apply flex flex-col;
	}

	.metric-label {
		@apply text-xs text-gray-500;
	}

	.metric-value {
		@apply text-sm font-semibold text-white;
	}

	.metric.drop-off .metric-value {
		@apply text-red-400;
	}

	.step-actions {
		@apply space-y-2;
	}

	.actions-label {
		@apply text-xs text-gray-500 mb-2;
	}

	.action-item {
		@apply flex items-center justify-between text-sm;
	}

	.action-name {
		@apply text-gray-300;
	}

	.action-count {
		@apply text-yellow-400 font-semibold;
	}

	.step-connector {
		@apply relative py-4 flex flex-col items-center;
	}

	.connector-line {
		@apply w-0.5 h-8 bg-gradient-to-b from-gray-600 to-gray-700;
	}

	.connector-stats {
		@apply flex items-center gap-2 my-2;
	}

	.conversion-rate {
		@apply text-sm font-semibold text-green-400;
	}

	.drop-off-indicator {
		@apply flex items-center gap-1 px-2 py-1 bg-red-500/10 text-red-400 rounded text-xs;
	}

	.completion-badge {
		@apply flex items-center justify-center gap-2 py-4 text-green-400 font-semibold;
	}
</style>

<!--
	User Journey Map Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user paths through the platform with conversion points and drop-offs.
-->

<script lang="ts">
	import { IconArrowRight, IconX, IconCheck } from '$lib/icons';

	interface JourneyStep {
		step: string;
		users: number;
		conversion_rate: number;
		avg_time: number;
		drop_off: number;
		top_actions: Array<{ action: string; count: number }>;
	}

	interface Props {
		journeyData?: JourneyStep[];
	}

	let { journeyData = [] }: Props = $props();

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
				{(
					((journeyData[journeyData.length - 1]?.users ?? 0) / (journeyData[0]?.users ?? 1)) *
					100
				).toFixed(1)}% completed
			</span>
		</div>
	</div>

	<div class="journey-flow">
		{#each journeyData as step, index (step.step)}
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
							{#each step.top_actions.slice(0, 3) as action (action.action)}
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
							<IconArrowRight size={20} class="connector-icon" />
							<span class="conversion-rate">
								{journeyData[index + 1]?.conversion_rate?.toFixed(1) ?? '0.0'}%
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
						<IconCheck size={24} class="completion-icon" />
						<span>Journey Complete</span>
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.journey-map {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.journey-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.journey-title {
		margin: 0;
		font-size: 1.25rem;
		line-height: 1.75rem;
		font-weight: 700;
		color: #ffffff;
	}

	.journey-stats {
		display: flex;
		align-items: center;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.stat-item {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.journey-flow {
		display: flex;
		flex-direction: column;
	}

	.journey-step {
		position: relative;
	}

	.step-card {
		padding: 1rem;
		border: 1px solid rgba(55, 65, 81, 0.5);
		border-radius: 0.5rem;
		background: rgba(17, 24, 39, 0.5);
		transition: border-color 0.2s ease;
	}

	.step-card:hover {
		border-color: rgba(234, 179, 8, 0.3);
	}

	.step-header {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.step-number {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		width: 2rem;
		height: 2rem;
		border-radius: 999px;
		background: #eab308;
		color: #111827;
		font-weight: 700;
	}

	.step-info {
		flex: 1;
		min-width: 0;
	}

	.step-name {
		margin-bottom: 0.25rem;
		color: #ffffff;
		font-weight: 600;
	}

	.step-users {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.step-badge {
		padding: 0.25rem 0.75rem;
		border-radius: 0.5rem;
		background: rgba(34, 197, 94, 0.2);
		color: #4ade80;
		font-size: 0.875rem;
		font-weight: 600;
		white-space: nowrap;
	}

	.step-metrics {
		display: flex;
		gap: 1rem;
		margin-bottom: 0.75rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid rgba(55, 65, 81, 0.5);
	}

	.metric {
		display: flex;
		flex-direction: column;
	}

	.metric-label {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.metric-value {
		font-size: 0.875rem;
		font-weight: 600;
		color: #ffffff;
	}

	.metric.drop-off .metric-value {
		color: #f87171;
	}

	.step-actions {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.actions-label {
		margin-bottom: 0.5rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.action-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		font-size: 0.875rem;
	}

	.action-name {
		color: #d1d5db;
	}

	.action-count {
		color: #facc15;
		font-weight: 600;
	}

	.step-connector {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		padding-block: 1rem;
	}

	.connector-line {
		width: 0.125rem;
		height: 2rem;
		background: linear-gradient(to bottom, #4b5563, #374151);
	}

	.connector-stats {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-block: 0.5rem;
	}

	.connector-stats :global(.connector-icon) {
		color: #9ca3af;
	}

	.conversion-rate {
		color: #4ade80;
		font-size: 0.875rem;
		font-weight: 600;
	}

	.drop-off-indicator {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		font-size: 0.75rem;
	}

	.completion-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding-block: 1rem;
		color: #4ade80;
		font-weight: 600;
	}

	.completion-badge :global(.completion-icon) {
		color: #4ade80;
	}

	@media (max-width: 640px) {
		.journey-header,
		.step-header {
			align-items: flex-start;
			flex-direction: column;
		}
	}
</style>

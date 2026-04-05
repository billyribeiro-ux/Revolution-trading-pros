<!--
	User Journey Map Component
	═══════════════════════════════════════════════════════════════════════════
	
	Visualizes user paths through the platform with conversion points and drop-offs.
-->

<script lang="ts">
	import { Icon, IconArrowRight, IconCheck, IconX } from '$lib/icons';

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
							<Icon icon={IconArrowRight} size={20} />
							<span class="conversion-rate">
								{journeyData[index + 1]?.conversion_rate?.toFixed(1) ?? '0.0'}%
							</span>
						</div>
						{#if step.drop_off > 0}
							<div class="drop-off-indicator">
								<Icon icon={IconX} size={16} />
								{formatNumber(step.drop_off)} dropped
							</div>
						{/if}
					</div>
				{:else}
					<div class="completion-badge">
						<Icon icon={IconCheck} size={24} />
						<span>Journey Complete</span>
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.journey-map {
		background-color: oklch(0.2 0.02 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.35 0.02 250 / 50%);
	}

	.journey-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.journey-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.journey-stats {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.stat-item {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
	}

	.journey-step {
		position: relative;
	}

	.step-card {
		background-color: oklch(0.15 0.01 250 / 50%);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
		border: 1px solid oklch(0.38 0.01 250 / 50%);
		transition: border-color var(--duration-fast) var(--ease-default);
		&:hover {
			border-color: oklch(0.8 0.18 90 / 30%);
		}
	}

	.step-header {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		margin-block-end: var(--space-3);
	}

	.step-number {
		inline-size: 2rem;
		block-size: 2rem;
		border-radius: 9999px;
		background-color: oklch(0.8 0.18 90);
		color: oklch(0.15 0.02 90);
		font-weight: var(--weight-bold);
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.step-info {
		flex: 1;
	}

	.step-name {
		color: oklch(1 0 0);
		font-weight: var(--weight-semibold);
		margin-block-end: var(--space-1);
	}

	.step-users {
		font-size: var(--text-sm);
		color: oklch(0.65 0.01 250);
	}

	.step-badge {
		padding-inline: var(--space-3);
		padding-block: var(--space-1);
		background-color: oklch(0.6 0.18 160 / 20%);
		color: oklch(0.7 0.18 160);
		border-radius: var(--radius-lg);
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
	}

	.step-metrics {
		display: flex;
		gap: var(--space-4);
		margin-block-end: var(--space-3);
		padding-block-end: var(--space-3);
		border-block-end: 1px solid oklch(0.38 0.01 250 / 50%);
	}

	.metric {
		display: flex;
		flex-direction: column;
	}

	.metric-label {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}

	.metric-value {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
	}

	.metric.drop-off .metric-value {
		color: oklch(0.7 0.2 25);
	}

	.step-actions {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.actions-label {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
		margin-block-end: var(--space-2);
	}

	.action-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--text-sm);
	}

	.action-name {
		color: oklch(0.75 0.01 250);
	}

	.action-count {
		color: oklch(0.8 0.18 90);
		font-weight: var(--weight-semibold);
	}

	.step-connector {
		position: relative;
		padding-block: var(--space-4);
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.connector-line {
		inline-size: 0.125rem;
		block-size: 2rem;
		background: linear-gradient(to bottom, oklch(0.45 0.01 250), oklch(0.38 0.01 250));
	}

	.connector-stats {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-block: var(--space-2);
		color: oklch(0.65 0.01 250);
	}

	.conversion-rate {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(0.7 0.18 160);
	}

	.drop-off-indicator {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		padding-inline: var(--space-2);
		padding-block: var(--space-1);
		background-color: oklch(0.55 0.22 25 / 10%);
		color: oklch(0.7 0.2 25);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
	}

	.completion-badge {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		padding-block: var(--space-4);
		color: oklch(0.7 0.18 160);
		font-weight: var(--weight-semibold);
	}
</style>

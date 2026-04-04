<script lang="ts">
	/**
	 * FunnelChart - Enterprise Conversion Funnel Visualization
	 *
	 * Displays a multi-step conversion funnel with drop-off rates
	 * and conversion percentages.
	 */
	import type { FunnelStep } from '$lib/api/analytics';

	interface Props {
		steps?: FunnelStep[];
		title?: string;
		showDropOff?: boolean;
		animated?: boolean;
	}

	let {
		steps = [],
		title = 'Conversion Funnel',
		showDropOff = true,
		animated = true
	}: Props = $props();

	// Calculate width percentages based on conversion
	function getStepWidth(step: FunnelStep, index: number): number {
		if (index === 0) return 100;
		return step.overall_conversion_rate ?? 0;
	}

	// Color step index for data attribute styling
	function getStepColor(index: number): number {
		return index % 6;
	}

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}
</script>

<div class="funnel-container">
	<h3 class="funnel-title">{title}</h3>

	<div class="funnel-steps">
		{#each steps as step, index (step.step_number)}
			{@const width = getStepWidth(step, index)}

			<div class="step-row">
				<!-- Step Bar -->
				<div class="step-bar-wrapper">
					<div
						class="step-bar"
						data-step={getStepColor(index)}
						data-animated={animated || undefined}
						style="width: {width}%"
					>
						<!-- Step Label -->
						<div class="step-label">
							<span class="step-name">
								{step.name}
							</span>
							<span class="step-count">
								{formatNumber(step.count)}
							</span>
						</div>
					</div>

					<!-- Conversion Rate Badge -->
					<div class="rate-badge">
						<span
							class="rate-value"
							data-level={step.conversion_rate >= 80 ? 'good' : step.conversion_rate >= 50 ? 'mid' : 'poor'}
						>
							{step.conversion_rate.toFixed(1)}%
						</span>
					</div>
				</div>

				<!-- Drop-off Indicator -->
				{#if showDropOff && index > 0 && (step.drop_off ?? 0) > 0}
					<div class="dropoff">
						<div class="dropoff-info">
							<span>↓</span>
							<span>{formatNumber(step.drop_off ?? 0)}</span>
							<span class="dropoff-rate">({(step.drop_off_rate ?? 0).toFixed(1)}%)</span>
						</div>
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- Summary -->
	{#if steps.length > 0}
		{@const firstStep = steps[0]}
		{@const lastStep = steps[steps.length - 1]}
		<div class="funnel-summary">
			<div class="summary-row">
				<div>
					<span class="summary-label">Total Entries:</span>
					<span class="summary-value">{formatNumber(firstStep.count)}</span>
				</div>
				<div>
					<span class="summary-label">Conversions:</span>
					<span class="summary-value">{formatNumber(lastStep.count)}</span>
				</div>
				<div>
					<span class="summary-label">Overall Rate:</span>
					<span
						class="summary-value"
						data-level={(lastStep.overall_conversion_rate ?? 0) >= 10 ? 'good' : 'poor'}
					>
						{(lastStep.overall_conversion_rate ?? 0).toFixed(2)}%
					</span>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.funnel-container {
		background-color: oklch(1 0 0);
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
		padding: var(--space-6);
	}

	.funnel-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
		margin-block-end: var(--space-6);
	}

	.funnel-steps {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.step-row { position: relative; }

	.step-bar-wrapper {
		position: relative;
		block-size: 3rem;
		display: flex;
		align-items: center;
	}

	.step-bar {
		block-size: 100%;
		border-radius: var(--radius-lg);
		position: relative;
		overflow: hidden;

		&[data-animated] { transition: all 700ms ease-out; }

		&[data-step='0'] { background-color: oklch(0.6 0.2 260); }
		&[data-step='1'] { background-color: oklch(0.68 0.16 260); }
		&[data-step='2'] { background-color: oklch(0.7 0.15 200); }
		&[data-step='3'] { background-color: oklch(0.68 0.14 175); }
		&[data-step='4'] { background-color: oklch(0.68 0.16 160); }
		&[data-step='5'] { background-color: oklch(0.6 0.18 160); }
	}

	.step-label {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-inline: var(--space-4);
	}

	.step-name {
		color: oklch(1 0 0);
		font-weight: var(--weight-medium);
		font-size: var(--text-sm);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.step-count {
		color: oklch(1 0 0 / 90%);
		font-weight: var(--weight-bold);
	}

	.rate-badge {
		margin-inline-start: var(--space-4);
		flex-shrink: 0;
	}

	.rate-value {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);

		&[data-level='good'] { color: oklch(0.5 0.18 160); }
		&[data-level='mid'] { color: oklch(0.6 0.18 90); }
		&[data-level='poor'] { color: oklch(0.55 0.2 25); }
	}

	.dropoff {
		position: absolute;
		inset-block-start: -0.25rem;
		inset-inline-end: 0;
		transform: translateX(100%);
		margin-inline-start: 4rem;
	}

	.dropoff-info {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		font-size: var(--text-xs);
		color: oklch(0.6 0.2 25);
	}

	.dropoff-rate { color: oklch(0.65 0.01 265); }

	.funnel-summary {
		margin-block-start: var(--space-6);
		padding-block-start: var(--space-4);
		border-block-start: 1px solid oklch(0.95 0.002 265);
	}

	.summary-row {
		display: flex;
		justify-content: space-between;
		font-size: var(--text-sm);
	}

	.summary-label { color: oklch(0.55 0.01 265); }

	.summary-value {
		font-weight: var(--weight-semibold);
		margin-inline-start: var(--space-1);

		&[data-level='good'] { color: oklch(0.5 0.18 160); }
		&[data-level='poor'] { color: oklch(0.65 0.18 55); }
	}
</style>

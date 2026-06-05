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

	const stepTones = ['blue', 'sky', 'cyan', 'teal', 'green', 'emerald'] as const;

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}

	function getConversionTone(rate: number): 'strong' | 'moderate' | 'weak' {
		if (rate >= 80) return 'strong';
		if (rate >= 50) return 'moderate';
		return 'weak';
	}
</script>

<div class="funnel-chart">
	<h3>{title}</h3>

	<div class="funnel-chart__steps">
		{#each steps as step, index (step.step_number)}
			{@const width = getStepWidth(step, index)}
			{@const stepTone = stepTones[index % stepTones.length]}

			<div class="funnel-chart__step">
				<!-- Step Bar -->
				<div class="funnel-chart__bar-row">
					<div
						class={{
							'funnel-chart__bar': true,
							'funnel-chart__bar--animated': animated
						}}
						data-tone={stepTone}
						style:width={`${width}%`}
					>
						<!-- Step Label -->
						<div class="funnel-chart__bar-content">
							<span class="funnel-chart__step-name">
								{step.name}
							</span>
							<span class="funnel-chart__step-count">
								{formatNumber(step.count)}
							</span>
						</div>
					</div>

					<!-- Conversion Rate Badge -->
					<div class="funnel-chart__rate">
						<span data-tone={getConversionTone(step.conversion_rate)}>
							{step.conversion_rate.toFixed(1)}%
						</span>
					</div>
				</div>

				<!-- Drop-off Indicator -->
				{#if showDropOff && index > 0 && (step.drop_off ?? 0) > 0}
					<div class="funnel-chart__dropoff">
						<div class="funnel-chart__dropoff-content">
							<span>↓</span>
							<span>{formatNumber(step.drop_off ?? 0)}</span>
							<span>({(step.drop_off_rate ?? 0).toFixed(1)}%)</span>
						</div>
					</div>
				{/if}
			</div>
		{:else}
			<div class="funnel-chart__empty">No funnel steps available.</div>
		{/each}
	</div>

	<!-- Summary -->
	{#if steps.length > 0}
		{@const firstStep = steps[0]}
		{@const lastStep = steps[steps.length - 1]}
		<div class="funnel-chart__summary">
			<div class="funnel-chart__summary-grid">
				<div>
					<span class="funnel-chart__summary-label">Total Entries:</span>
					<span class="funnel-chart__summary-value">{formatNumber(firstStep.count)}</span>
				</div>
				<div>
					<span class="funnel-chart__summary-label">Conversions:</span>
					<span class="funnel-chart__summary-value">{formatNumber(lastStep.count)}</span>
				</div>
				<div>
					<span class="funnel-chart__summary-label">Overall Rate:</span>
					<span
						class="funnel-chart__summary-value"
						data-tone={(lastStep.overall_conversion_rate ?? 0) >= 10 ? 'strong' : 'low'}
					>
						{(lastStep.overall_conversion_rate ?? 0).toFixed(2)}%
					</span>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.funnel-chart {
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
		padding: 1.5rem;
	}

	.funnel-chart h3 {
		margin: 0 0 1.5rem;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.funnel-chart__steps {
		display: grid;
		gap: 0.75rem;
	}

	.funnel-chart__step {
		position: relative;
	}

	.funnel-chart__bar-row {
		position: relative;
		display: flex;
		height: 3rem;
		align-items: center;
	}

	.funnel-chart__bar {
		position: relative;
		overflow: hidden;
		height: 100%;
		border-radius: 0.5rem;
	}

	.funnel-chart__bar--animated {
		transition: width 700ms ease-out;
	}

	.funnel-chart__bar[data-tone='blue'] {
		background: #3b82f6;
	}

	.funnel-chart__bar[data-tone='sky'] {
		background: #60a5fa;
	}

	.funnel-chart__bar[data-tone='cyan'] {
		background: #22d3ee;
	}

	.funnel-chart__bar[data-tone='teal'] {
		background: #2dd4bf;
	}

	.funnel-chart__bar[data-tone='green'] {
		background: #4ade80;
	}

	.funnel-chart__bar[data-tone='emerald'] {
		background: #10b981;
	}

	.funnel-chart__bar-content {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0 1rem;
	}

	.funnel-chart__step-name {
		overflow: hidden;
		color: #ffffff;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.funnel-chart__step-count {
		color: rgba(255, 255, 255, 0.9);
		font-weight: 700;
		flex: 0 0 auto;
	}

	.funnel-chart__rate {
		margin-left: 1rem;
		flex: 0 0 auto;
	}

	.funnel-chart__rate span,
	.funnel-chart__summary-value {
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	.funnel-chart__rate span[data-tone='strong'],
	.funnel-chart__summary-value[data-tone='strong'] {
		color: #16a34a;
	}

	.funnel-chart__rate span[data-tone='moderate'] {
		color: #ca8a04;
	}

	.funnel-chart__rate span[data-tone='weak'] {
		color: #dc2626;
	}

	.funnel-chart__summary-value[data-tone='low'] {
		color: #ea580c;
	}

	.funnel-chart__dropoff {
		position: absolute;
		top: -0.25rem;
		right: 0;
		margin-left: 4rem;
		transform: translateX(100%);
	}

	.funnel-chart__dropoff-content {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: #ef4444;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.funnel-chart__dropoff-content span:last-child {
		color: #9ca3af;
	}

	.funnel-chart__summary {
		border-top: 1px solid #f3f4f6;
		margin-top: 1.5rem;
		padding-top: 1rem;
	}

	.funnel-chart__summary-grid {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	.funnel-chart__summary-label {
		color: #6b7280;
	}

	.funnel-chart__summary-value {
		margin-left: 0.25rem;
		color: #111827;
	}

	.funnel-chart__empty {
		padding: 2rem 1rem;
		color: #6b7280;
		text-align: center;
	}

	@media (max-width: 768px) {
		.funnel-chart__dropoff {
			position: static;
			margin: 0.375rem 0 0;
			transform: none;
		}

		.funnel-chart__summary-grid {
			flex-direction: column;
			gap: 0.5rem;
		}
	}

	@media (max-width: 520px) {
		.funnel-chart {
			padding: 1rem;
		}

		.funnel-chart__bar-row {
			height: auto;
			align-items: flex-start;
			flex-direction: column;
			gap: 0.5rem;
		}

		.funnel-chart__bar {
			width: 100% !important;
			min-height: 3rem;
		}

		.funnel-chart__rate {
			margin-left: 0;
		}
	}
</style>

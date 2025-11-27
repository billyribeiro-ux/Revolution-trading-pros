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
		return step.overall_conversion_rate;
	}

	// Color gradient for funnel steps
	const stepColors = [
		'bg-blue-500',
		'bg-blue-400',
		'bg-cyan-400',
		'bg-teal-400',
		'bg-green-400',
		'bg-emerald-500'
	];

	function formatNumber(num: number): string {
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toString();
	}
</script>

<div class="bg-white rounded-xl border border-gray-200 p-6">
	<h3 class="text-lg font-semibold text-gray-900 mb-6">{title}</h3>

	<div class="space-y-3">
		{#each steps as step, index (step.step_number)}
			{@const width = getStepWidth(step, index)}
			{@const prevStep = index > 0 ? steps[index - 1] : null}

			<div class="relative">
				<!-- Step Bar -->
				<div class="relative h-12 flex items-center">
					<div
						class="{stepColors[
							index % stepColors.length
						]} h-full rounded-lg relative overflow-hidden
							{animated ? 'transition-all duration-700 ease-out' : ''}"
						style="width: {width}%"
					>
						<!-- Step Label -->
						<div class="absolute inset-0 flex items-center justify-between px-4">
							<span class="text-white font-medium text-sm truncate">
								{step.name}
							</span>
							<span class="text-white/90 font-bold">
								{formatNumber(step.count)}
							</span>
						</div>
					</div>

					<!-- Conversion Rate Badge -->
					<div class="ml-4 flex-shrink-0">
						<span
							class="text-sm font-semibold {step.conversion_rate >= 80
								? 'text-green-600'
								: step.conversion_rate >= 50
									? 'text-yellow-600'
									: 'text-red-600'}"
						>
							{step.conversion_rate.toFixed(1)}%
						</span>
					</div>
				</div>

				<!-- Drop-off Indicator -->
				{#if showDropOff && index > 0 && step.drop_off > 0}
					<div class="absolute -top-1 right-0 transform translate-x-full ml-16">
						<div class="flex items-center gap-1 text-xs text-red-500">
							<span>↓</span>
							<span>{formatNumber(step.drop_off)}</span>
							<span class="text-gray-400">({step.drop_off_rate.toFixed(1)}%)</span>
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
		<div class="mt-6 pt-4 border-t border-gray-100">
			<div class="flex justify-between text-sm">
				<div>
					<span class="text-gray-500">Total Entries:</span>
					<span class="font-semibold ml-1">{formatNumber(firstStep.count)}</span>
				</div>
				<div>
					<span class="text-gray-500">Conversions:</span>
					<span class="font-semibold ml-1">{formatNumber(lastStep.count)}</span>
				</div>
				<div>
					<span class="text-gray-500">Overall Rate:</span>
					<span
						class="font-semibold ml-1 {lastStep.overall_conversion_rate >= 10
							? 'text-green-600'
							: 'text-orange-600'}"
					>
						{lastStep.overall_conversion_rate.toFixed(2)}%
					</span>
				</div>
			</div>
		</div>
	{/if}
</div>

<script lang="ts">
	/**
	 * PeriodSelector - Date Range Selection Component
	 *
	 * Quick period selection and custom date range picker.
	 */

	interface Props {
		value?: string;
		showCustom?: boolean;
		onchange?: (value: string) => void;
	}

	let { value = $bindable('30d'), showCustom = true, onchange }: Props = $props();

	const periods = [
		{ value: '7d', label: '7 Days' },
		{ value: '14d', label: '14 Days' },
		{ value: '30d', label: '30 Days' },
		{ value: '90d', label: '90 Days' },
		{ value: '1y', label: '1 Year' }
	];

	function selectPeriod(period: string) {
		value = period;
		onchange?.(period);
	}
</script>

<div class="flex items-center gap-1 bg-gray-100 rounded-lg p-1">
	{#each periods as period}
		<button
			class="px-3 py-1.5 text-sm font-medium rounded-md transition-all
				{value === period.value ? 'bg-white text-gray-900 shadow-sm' : 'text-gray-600 hover:text-gray-900'}"
			onclick={() => selectPeriod(period.value)}
		>
			{period.label}
		</button>
	{/each}

	{#if showCustom}
		<button
			class="px-3 py-1.5 text-sm font-medium rounded-md transition-all
				{!periods.find((p) => p.value === value)
				? 'bg-white text-gray-900 shadow-sm'
				: 'text-gray-600 hover:text-gray-900'}"
		>
			Custom
		</button>
	{/if}
</div>

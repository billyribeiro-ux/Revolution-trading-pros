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

<div class="period-selector">
	{#each periods as period (period.value)}
		<button
			class="period-btn"
			data-active={value === period.value || undefined}
			onclick={() => selectPeriod(period.value)}
		>
			{period.label}
		</button>
	{/each}

	{#if showCustom}
		<button
			class="period-btn"
			data-active={!periods.find((p) => p.value === value) || undefined}
		>
			Custom
		</button>
	{/if}
</div>

<style>
	.period-selector {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		background-color: oklch(0.95 0.002 265);
		border-radius: var(--radius-lg);
		padding: 0.25rem;
	}

	.period-btn {
		padding-inline: var(--space-3);
		padding-block: 0.375rem;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		border-radius: var(--radius-md);
		transition: all var(--duration-fast) var(--ease-default);
		color: oklch(0.45 0.01 265);
		background: none;
		border: none;
		cursor: pointer;

		&:hover { color: oklch(0.15 0.01 265); }

		&[data-active] {
			background-color: oklch(1 0 0);
			color: oklch(0.15 0.01 265);
			box-shadow: 0 1px 2px oklch(0 0 0 / 5%);
		}
	}
</style>

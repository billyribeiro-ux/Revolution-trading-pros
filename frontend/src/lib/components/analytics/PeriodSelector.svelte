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
			type="button"
			class={{ 'period-selector__button': true, 'is-selected': value === period.value }}
			onclick={() => selectPeriod(period.value)}
		>
			{period.label}
		</button>
	{/each}

	{#if showCustom}
		<button
			type="button"
			class={{
				'period-selector__button': true,
				'is-selected': !periods.some((period) => period.value === value)
			}}
		>
			Custom
		</button>
	{/if}
</div>

<style>
	.period-selector {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		border-radius: 0.5rem;
		background: #f3f4f6;
		padding: 0.25rem;
	}

	.period-selector__button {
		border: 0;
		border-radius: 0.375rem;
		background: transparent;
		color: #4b5563;
		cursor: pointer;
		font: inherit;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1.25rem;
		padding: 0.375rem 0.75rem;
		transition:
			background 150ms ease,
			box-shadow 150ms ease,
			color 150ms ease;
		white-space: nowrap;
	}

	.period-selector__button:hover {
		color: #111827;
	}

	.period-selector__button:focus-visible {
		outline: 2px solid #2563eb;
		outline-offset: 2px;
	}

	.period-selector__button.is-selected {
		background: #ffffff;
		box-shadow: 0 1px 2px rgb(0 0 0 / 0.05);
		color: #111827;
	}

	@media (max-width: 480px) {
		.period-selector {
			display: flex;
			overflow-x: auto;
			width: 100%;
		}

		.period-selector__button {
			flex: 1 0 auto;
		}
	}
</style>

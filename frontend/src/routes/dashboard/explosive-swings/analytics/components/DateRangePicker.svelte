<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * DateRangePicker Component - Period Selection for Analytics
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 1.0.0 - Phase 4: Analytics Dashboard
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA | Svelte 5 January 2026
	 */
	import type { TimePeriod } from '../analytics.state.svelte';

	interface Props {
		selectedPeriod: TimePeriod;
		periodLabel: string;
		onPeriodChange: (period: TimePeriod) => void;
	}

	const { selectedPeriod, periodLabel, onPeriodChange }: Props = $props();

	// Period options
	const periods: { value: TimePeriod; label: string }[] = [
		{ value: '30d', label: '30 Days' },
		{ value: '90d', label: '90 Days' },
		{ value: '180d', label: '6 Months' },
		{ value: '365d', label: '1 Year' },
		{ value: 'ytd', label: 'YTD' },
		{ value: 'all', label: 'All Time' }
	];

	// Handle period selection
	function handlePeriodSelect(period: TimePeriod) {
		onPeriodChange(period);
	}
</script>

<div class="date-range-picker" role="group" aria-label="Select time period">
	<span class="picker-label">Period:</span>

	<div class="period-buttons" role="radiogroup">
		{#each periods as period}
			<button
				type="button"
				class="period-btn"
				class:active={selectedPeriod === period.value}
				onclick={() => handlePeriodSelect(period.value)}
				role="radio"
				aria-checked={selectedPeriod === period.value}
			>
				{period.label}
			</button>
		{/each}
	</div>

	<span class="current-period">{periodLabel}</span>
</div>

<style>
	.date-range-picker {
		display: flex;
		align-items: center;
		gap: 12px;
		flex-wrap: wrap;
	}

	.picker-label {
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text-secondary);
	}

	.period-buttons {
		display: flex;
		gap: 4px;
		background: var(--color-bg-subtle);
		padding: 3px;
		border-radius: 8px;
	}

	.period-btn {
		padding: 6px 12px;
		font-size: 12px;
		font-weight: 600;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		background: transparent;
		color: var(--color-text-secondary);
		transition:
			background 0.15s,
			color 0.15s;
	}

	.period-btn:hover {
		background: var(--color-bg-muted);
		color: var(--color-text-primary);
	}

	.period-btn.active {
		background: var(--color-bg-card);
		color: var(--color-brand-primary);
		box-shadow: var(--shadow-sm);
	}

	.current-period {
		font-size: 12px;
		color: var(--color-text-tertiary);
		margin-left: auto;
	}

	@media (max-width: 768px) {
		.date-range-picker {
			flex-direction: column;
			align-items: flex-start;
		}

		.period-buttons {
			flex-wrap: wrap;
		}

		.current-period {
			margin-left: 0;
		}
	}
</style>

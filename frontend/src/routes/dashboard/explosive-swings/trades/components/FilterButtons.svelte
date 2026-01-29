<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * FilterButtons Component - Trade Result Filters
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */
	import type { FilterStatus } from '../types';

	interface Props {
		selected: FilterStatus;
		onFilterChange: (filter: FilterStatus) => void;
		counts?: {
			all: number;
			active: number;
			win: number;
			loss: number;
		};
	}

	const { selected, onFilterChange, counts }: Props = $props();

	const filters: { value: FilterStatus; label: string }[] = [
		{ value: 'all', label: 'All Trades' },
		{ value: 'active', label: 'Active' },
		{ value: 'win', label: 'Wins' },
		{ value: 'loss', label: 'Losses' }
	];
</script>

<div class="filter-section" role="group" aria-label="Filter trades by result">
	{#each filters as filter}
		<button
			type="button"
			class="filter-btn"
			aria-pressed={selected === filter.value}
			onclick={() => onFilterChange(filter.value)}
		>
			{filter.label}
			{#if counts}
				<span class="filter-count">({counts[filter.value]})</span>
			{/if}
		</button>
	{/each}
</div>

<style>
	.filter-section {
		display: flex;
		gap: var(--space-2);
		justify-content: center;
		flex-wrap: wrap;
		margin-bottom: var(--space-8);
	}

	.filter-btn {
		background: var(--color-bg-card);
		border: 2px solid var(--color-border-default);
		padding: var(--space-2) var(--space-5);
		border-radius: var(--radius-full);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: var(--transition-colors), var(--transition-shadow);
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
	}

	.filter-btn:hover {
		border-color: var(--color-brand-primary);
		color: var(--color-brand-primary);
	}

	.filter-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	.filter-btn[aria-pressed='true'] {
		background: var(--color-brand-primary);
		border-color: var(--color-brand-primary);
		color: white;
	}

	.filter-count {
		font-size: var(--text-xs);
		opacity: 0.8;
	}
</style>

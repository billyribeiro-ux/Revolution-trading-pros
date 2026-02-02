<!--
	AlertFilters.svelte - Filter Pills Component
	═══════════════════════════════════════════════════════════════════════════
	
	Apple Principal Engineer ICT 11+ Standards
	Svelte 5 January 2026 Syntax
	
	Single Responsibility: Display and manage filter state for alerts
	
	@version 1.0.0
	@since January 2026
-->
<script lang="ts">
	import type { AlertFilter, FilterCounts } from './types';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS - Svelte 5 $props() pattern
	// ═══════════════════════════════════════════════════════════════════════════
	interface Props {
		selected: AlertFilter;
		onFilterChange: (filter: AlertFilter) => void;
		counts?: FilterCounts;
		showCounts?: boolean;
	}

	let props: Props = $props();

	// Derived props with defaults
	let selected = $derived(props.selected);
	let onFilterChange = $derived(props.onFilterChange);
	let counts = $derived(props.counts);
	let showCounts = $derived(props.showCounts ?? false);

	// ═══════════════════════════════════════════════════════════════════════════
	// FILTER OPTIONS
	// ═══════════════════════════════════════════════════════════════════════════
	const filters: { value: AlertFilter; label: string }[] = [
		{ value: 'all', label: 'All' },
		{ value: 'entry', label: 'Entries' },
		{ value: 'exit', label: 'Exits' },
		{ value: 'update', label: 'Updates' }
	];

	function handleFilterClick(filter: AlertFilter) {
		onFilterChange(filter);
	}

	function handleKeyDown(event: KeyboardEvent, filter: AlertFilter) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			onFilterChange(filter);
		}
	}
</script>

<div class="filter-pills" role="tablist" aria-label="Filter alerts by type">
	{#each filters as filter}
		<button
			class="pill"
			class:active={selected === filter.value}
			onclick={() => handleFilterClick(filter.value)}
			onkeydown={(e) => handleKeyDown(e, filter.value)}
			role="tab"
			aria-selected={selected === filter.value}
			aria-controls="alerts-list"
		>
			{filter.label}
			{#if showCounts && counts}
				<span class="pill-count">({counts[filter.value]})</span>
			{/if}
		</button>
	{/each}
</div>

<style>
	.filter-pills {
		display: flex;
		gap: 8px;
		flex-wrap: wrap;
	}

	.pill {
		background: #f3f4f6;
		border: none;
		padding: 8px 16px;
		border-radius: 20px;
		font-size: 13px;
		font-weight: 600;
		color: #666;
		cursor: pointer;
		transition: all 0.2s;
		display: inline-flex;
		align-items: center;
		gap: 6px;
	}

	.pill:hover {
		background: #e5e7eb;
	}

	.pill:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.pill.active {
		background: #143e59;
		color: #fff;
	}

	.pill-count {
		font-size: 11px;
		opacity: 0.8;
	}

	.pill.active .pill-count {
		opacity: 1;
	}
</style>

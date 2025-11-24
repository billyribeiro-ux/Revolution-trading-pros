<script lang="ts">
	/**
	 * KpiGrid - Enterprise KPI Dashboard Grid
	 *
	 * Displays a grid of KPI cards with filtering by category
	 * and responsive layout.
	 */
	import KpiCard from './KpiCard.svelte';
	import type { KpiValue } from '$lib/api/analytics';

	export let kpis: KpiValue[] = [];
	export let showPrimaryOnly: boolean = false;
	export let category: string | null = null;
	export let columns: 2 | 3 | 4 | 5 = 4;
	export let onKpiClick: ((kpi: KpiValue) => void) | null = null;

	$: filteredKpis = kpis.filter(kpi => {
		if (showPrimaryOnly && !kpi.is_primary) return false;
		if (category && kpi.category !== category) return false;
		return true;
	});

	$: gridCols = {
		2: 'grid-cols-1 sm:grid-cols-2',
		3: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3',
		4: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-4',
		5: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5'
	}[columns];

	// Group KPIs by category
	$: categories = [...new Set(kpis.map(k => k.category))];
</script>

<div class="space-y-6">
	{#if category === null && !showPrimaryOnly}
		<!-- Show by category -->
		{#each categories as cat}
			{@const categoryKpis = filteredKpis.filter(k => k.category === cat)}
			{#if categoryKpis.length > 0}
				<div>
					<h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-3 capitalize">
						{cat}
					</h3>
					<div class="grid {gridCols} gap-4">
						{#each categoryKpis as kpi (kpi.kpi_key)}
							<div on:click={() => onKpiClick?.(kpi)} on:keypress={() => onKpiClick?.(kpi)}>
								<KpiCard {kpi} clickable={!!onKpiClick} />
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{/each}
	{:else}
		<!-- Flat grid -->
		<div class="grid {gridCols} gap-4">
			{#each filteredKpis as kpi (kpi.kpi_key)}
				<div on:click={() => onKpiClick?.(kpi)} on:keypress={() => onKpiClick?.(kpi)}>
					<KpiCard {kpi} clickable={!!onKpiClick} />
				</div>
			{/each}
		</div>
	{/if}

	{#if filteredKpis.length === 0}
		<div class="text-center py-12 text-gray-500">
			<p>No KPIs available</p>
		</div>
	{/if}
</div>

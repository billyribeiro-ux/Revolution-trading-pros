<script lang="ts">
	/**
	 * KpiGrid - Enterprise KPI Dashboard Grid
	 *
	 * Displays a grid of KPI cards with filtering by category
	 * and responsive layout.
	 */
	import KpiCard from './KpiCard.svelte';
	import type { KpiValue } from '$lib/api/analytics';

	interface Props {
		kpis?: KpiValue[];
		showPrimaryOnly?: boolean;
		category?: string | null;
		columns?: 2 | 3 | 4 | 5;
		onKpiClick?: ((kpi: KpiValue) => void) | null;
	}

	let {
		kpis = [],
		showPrimaryOnly = false,
		category = null,
		columns = 4,
		onKpiClick = null
	}: Props = $props();

	let filteredKpis = $derived(
		kpis.filter((kpi) => {
			if (showPrimaryOnly && !kpi.is_primary) return false;
			if (category && kpi.category !== category) return false;
			return true;
		})
	);

	// Group KPIs by category
	let categories = $derived([...new Set(kpis.map((k) => k.category))]);
</script>

<div class="kpi-grid-wrapper">
	{#if category === null && !showPrimaryOnly}
		<!-- Show by category -->
		{#each categories as cat (cat)}
			{@const categoryKpis = filteredKpis.filter((k) => k.category === cat)}
			{#if categoryKpis.length > 0}
				<div>
					<h3 class="category-title">
						{cat}
					</h3>
					<div class="kpi-grid" style:--columns={columns}>
						{#each categoryKpis as kpi (kpi.kpi_key)}
							<div
								role="button"
								tabindex="0"
								onclick={(e: MouseEvent) => {
									e.preventDefault();
									onKpiClick?.(kpi);
								}}
								onkeypress={(e: KeyboardEvent) => {
									e.preventDefault();
									onKpiClick?.(kpi);
								}}
							>
								<KpiCard {kpi} clickable={!!onKpiClick} />
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{/each}
	{:else}
		<!-- Flat grid -->
		<div class="kpi-grid" style:--columns={columns}>
			{#each filteredKpis as kpi (kpi.kpi_key)}
				<div
					role="button"
					tabindex="0"
					onclick={() => onKpiClick?.(kpi)}
					onkeypress={() => onKpiClick?.(kpi)}
				>
					<KpiCard {kpi} clickable={!!onKpiClick} />
				</div>
			{/each}
		</div>
	{/if}

	{#if filteredKpis.length === 0}
		<div class="empty-state">
			<p>No KPIs available</p>
		</div>
	{/if}
</div>

<style>
	.kpi-grid-wrapper {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.category-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: oklch(0.55 0.01 265);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-block-end: var(--space-3);
		text-transform: capitalize;
	}

	.kpi-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-4);

		@media (min-width: 640px) {
			grid-template-columns: repeat(2, 1fr);
		}
		@media (min-width: 1024px) {
			grid-template-columns: repeat(var(--columns, 4), 1fr);
		}
	}

	.empty-state {
		text-align: center;
		padding-block: var(--space-12);
		color: oklch(0.55 0.01 265);
	}
</style>

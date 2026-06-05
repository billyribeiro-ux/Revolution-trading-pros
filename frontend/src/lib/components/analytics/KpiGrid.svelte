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

	function getCategoryId(categoryName: string) {
		return `kpi-grid-${categoryName.toLowerCase().replace(/[^a-z0-9]+/g, '-')}`;
	}
</script>

<div class="kpi-grid">
	{#if category === null && !showPrimaryOnly}
		<!-- Show by category -->
		{#each categories as cat (cat)}
			{@const categoryKpis = filteredKpis.filter((k) => k.category === cat)}
			{#if categoryKpis.length > 0}
				{@const categoryId = getCategoryId(cat)}
				<section class="kpi-grid__group" aria-labelledby={categoryId}>
					<h3 id={categoryId} class="kpi-grid__heading">{cat}</h3>
					<div class="kpi-grid__cards" data-columns={columns}>
						{#each categoryKpis as kpi (kpi.kpi_key)}
							{#if onKpiClick}
								<button type="button" class="kpi-grid__item" onclick={() => onKpiClick?.(kpi)}>
									<KpiCard {kpi} clickable />
								</button>
							{:else}
								<div class="kpi-grid__item">
									<KpiCard {kpi} />
								</div>
							{/if}
						{/each}
					</div>
				</section>
			{/if}
		{/each}
	{:else}
		<!-- Flat grid -->
		<div class="kpi-grid__cards" data-columns={columns}>
			{#each filteredKpis as kpi (kpi.kpi_key)}
				{#if onKpiClick}
					<button type="button" class="kpi-grid__item" onclick={() => onKpiClick?.(kpi)}>
						<KpiCard {kpi} clickable />
					</button>
				{:else}
					<div class="kpi-grid__item">
						<KpiCard {kpi} />
					</div>
				{/if}
			{/each}
		</div>
	{/if}

	{#if filteredKpis.length === 0}
		<div class="kpi-grid__empty">
			<p>No KPIs available</p>
		</div>
	{/if}
</div>

<style>
	.kpi-grid {
		display: grid;
		gap: 1.5rem;
	}

	.kpi-grid__group {
		display: grid;
		gap: 0.75rem;
	}

	.kpi-grid__heading {
		margin: 0;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 600;
		letter-spacing: 0.05em;
		line-height: 1.25rem;
		text-transform: uppercase;
	}

	.kpi-grid__cards {
		display: grid;
		gap: 1rem;
		grid-template-columns: 1fr;
	}

	.kpi-grid__item {
		border: 0;
		background: transparent;
		color: inherit;
		display: block;
		font: inherit;
		padding: 0;
		text-align: left;
		width: 100%;
	}

	button.kpi-grid__item {
		cursor: pointer;
	}

	button.kpi-grid__item:focus-visible {
		border-radius: 0.75rem;
		outline: 2px solid #2563eb;
		outline-offset: 3px;
	}

	.kpi-grid__empty {
		padding: 3rem 1rem;
		color: #6b7280;
		text-align: center;
	}

	.kpi-grid__empty p {
		margin: 0;
	}

	@media (min-width: 640px) {
		.kpi-grid__cards[data-columns='2'],
		.kpi-grid__cards[data-columns='3'],
		.kpi-grid__cards[data-columns='4'],
		.kpi-grid__cards[data-columns='5'] {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.kpi-grid__cards[data-columns='3'] {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}

		.kpi-grid__cards[data-columns='4'] {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.kpi-grid__cards[data-columns='5'] {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (min-width: 1280px) {
		.kpi-grid__cards[data-columns='5'] {
			grid-template-columns: repeat(5, minmax(0, 1fr));
		}
	}
</style>

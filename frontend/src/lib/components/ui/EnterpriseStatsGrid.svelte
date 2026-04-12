<script lang="ts">
	/**
	 * EnterpriseStatsGrid - Google-style Statistics Dashboard Grid
	 * Features staggered animations and responsive layout
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 * @level L8 Principal Engineer
	 */
	import EnterpriseStatCard from './EnterpriseStatCard.svelte';
	import SkeletonLoader from './SkeletonLoader.svelte';

	interface StatItem {
		id: string;
		title: string;
		value: number;
		format?: 'number' | 'currency' | 'percent' | 'compact';
		decimals?: number;
		prefix?: string;
		suffix?: string;
		trend?: number | null;
		trendLabel?: string;
		icon?: any;
		color?: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'cyan' | 'red';
		sparklineData?: number[];
		target?: number | null;
		targetLabel?: string;
		clickable?: boolean;
	}

	interface Props {
		stats?: StatItem[];
		loading?: boolean;
		columns?: 2 | 3 | 4 | 5;
		staggerDelay?: number;
		onStatClick?: ((stat: StatItem) => void) | null;
	}

	let {
		stats = [],
		loading = false,
		columns = 4,
		staggerDelay = 0.1,
		onStatClick = null
	}: Props = $props();

	let gridCols = $derived(
		{
			2: 'grid-cols-1 sm:grid-cols-2',
			3: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3',
			4: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-4',
			5: 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-5'
		}[columns]
	);
</script>

<div class="stats-grid" style:--columns={columns}>
	{#if loading}
		{#each Array(columns) as _, i (i)}
			<SkeletonLoader variant="stat" />
		{/each}
	{:else}
		{#each stats as stat, index (stat.id)}
			<EnterpriseStatCard
				title={stat.title}
				value={stat.value}
				format={stat.format ?? 'number'}
				decimals={stat.decimals ?? 0}
				prefix={stat.prefix ?? ''}
				suffix={stat.suffix ?? ''}
				trend={stat.trend ?? null}
				trendLabel={stat.trendLabel ?? 'vs last period'}
				icon={stat.icon ?? null}
				color={stat.color ?? 'blue'}
				sparklineData={stat.sparklineData ?? []}
				target={stat.target ?? null}
				targetLabel={stat.targetLabel ?? 'Target'}
				clickable={stat.clickable ?? !!onStatClick}
				delay={index * staggerDelay}
				onclick={() => onStatClick?.(stat)}
			/>
		{/each}
	{/if}
</div>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: var(--space-6);

		@media (min-width: 640px) {
			grid-template-columns: repeat(2, 1fr);
		}

		@media (min-width: 1024px) {
			grid-template-columns: repeat(min(var(--columns, 4), 4), 1fr);
		}

		@media (min-width: 1280px) {
			grid-template-columns: repeat(var(--columns, 4), 1fr);
		}
	}
</style>

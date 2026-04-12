<script lang="ts">
	/**
	 * CohortMatrix - Enterprise Cohort Retention Heatmap
	 *
	 * Displays a retention matrix/heatmap showing user retention
	 * across cohorts and time periods.
	 */
	import type { CohortRow } from '$lib/api/analytics';

	interface Props {
		data?: CohortRow[];
		title?: string;
		metricType?: 'retention' | 'revenue';
		maxPeriods?: number;
	}

	let {
		data = [],
		title = 'Cohort Retention',
		metricType = 'retention',
		maxPeriods = 12
	}: Props = $props();

	// Get retention level for data attribute styling
	function getRetentionLevel(value: number): string {
		if (value >= 80) return 'high';
		if (value >= 60) return 'good';
		if (value >= 40) return 'mid';
		if (value >= 20) return 'low';
		if (value > 0) return 'poor';
		return 'none';
	}

	// Format date for display
	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	// Get available period numbers
	let periodNumbers = $derived(Array.from({ length: maxPeriods }, (_, i) => i));

	// Calculate average retention per period
	let periodAverages = $derived(
		periodNumbers.map((period) => {
			const values = data
				.filter((row) => row.periods && row.periods[period])
				.map((row) => {
					const periodData = row.periods![period];
					return typeof periodData === 'number' ? periodData : periodData.retention_rate;
				});

			if (values.length === 0) return null;
			return values.reduce((a, b) => a + b, 0) / values.length;
		})
	);
</script>

<div class="cohort-container">
	<div class="cohort-header">
		<h3 class="cohort-title">{title}</h3>
		<p class="cohort-subtitle">
			{metricType === 'retention' ? 'User retention by cohort week' : 'Revenue by cohort'}
		</p>
	</div>

	<div class="table-scroll">
		<table class="cohort-table">
			<thead>
				<tr class="bg-gray-50">
					<th class="px-3 py-2 text-left font-medium text-gray-600 sticky left-0 bg-gray-50 z-10">
						Cohort
					</th>
					<th class="px-3 py-2 text-center font-medium text-gray-600"> Size </th>
					{#each periodNumbers as period (period)}
						<th class="px-2 py-2 text-center font-medium text-gray-600 min-w-[60px]">
							{period === 0 ? 'Day 0' : `Week ${period}`}
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each data as row (row.cohort_date)}
					<tr class="data-row">
						<td class="sticky-col row-label">
							{formatDate(row.cohort_date ?? '')}
						</td>
						<td class="center-col size-cell">
							{(row.cohort_size ?? 0).toLocaleString()}
						</td>
						{#each periodNumbers as period (period)}
							{@const periodData = row.periods ? row.periods[period] : null}
							<td class="cell">
								{#if periodData && typeof periodData === 'object'}
									{@const retentionRate = periodData.retention_rate}
									<div
										class="heat-cell"
										data-level={getRetentionLevel(retentionRate)}
										title={metricType === 'retention'
											? `${periodData.active_users} users (${retentionRate.toFixed(1)}%)`
											: `$${periodData.total_revenue.toFixed(0)}`}
									>
										{metricType === 'retention'
											? `${periodData.retention_rate.toFixed(0)}%`
											: `$${periodData.total_revenue.toFixed(0)}`}
									</div>
								{:else}
									<div class="heat-cell" data-level="none"></div>
								{/if}
							</td>
						{/each}
					</tr>
				{/each}

				<!-- Averages Row -->
				<tr class="avg-row">
					<td class="sticky-col avg-label"> Average </td>
					<td class="center-col size-cell">
						{Math.round(
							data.reduce((sum, row) => sum + (row.cohort_size || row.size || 0), 0) / data.length
						).toLocaleString()}
					</td>
					{#each periodAverages as avg (avg)}
						<td class="px-1 py-1">
							{#if avg !== null}
								<div class="heat-cell avg" data-level={getRetentionLevel(avg)}>
									{avg.toFixed(0)}%
								</div>
							{:else}
								<div class="heat-cell" data-level="none"></div>
							{/if}
						</td>
					{/each}
				</tr>
			</tbody>
		</table>
	</div>

	<!-- Legend -->
	<div class="legend">
		<span class="legend-label">Retention:</span>
		<div class="legend-items">
			<div class="legend-item">
				<div class="legend-swatch" data-level="high"></div>
				<span>80%+</span>
			</div>
			<div class="legend-item">
				<div class="legend-swatch" data-level="good"></div>
				<span>60-80%</span>
			</div>
			<div class="legend-item">
				<div class="legend-swatch" data-level="mid"></div>
				<span>40-60%</span>
			</div>
			<div class="legend-item">
				<div class="legend-swatch" data-level="low"></div>
				<span>20-40%</span>
			</div>
			<div class="legend-item">
				<div class="legend-swatch" data-level="poor"></div>
				<span>&lt;20%</span>
			</div>
		</div>
	</div>
</div>

<style>
	.cohort-container {
		background-color: oklch(1 0 0);
		border-radius: var(--radius-xl);
		border: 1px solid oklch(0.9 0.005 265);
		overflow: hidden;
	}

	.cohort-header {
		padding: var(--space-4);
		border-block-end: 1px solid oklch(0.95 0.002 265);
	}

	.cohort-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
	}

	.cohort-subtitle {
		font-size: var(--text-sm);
		color: oklch(0.55 0.01 265);
		margin-block-start: var(--space-1);
	}

	.table-scroll {
		overflow-x: auto;
	}

	.cohort-table {
		inline-size: 100%;
		font-size: var(--text-sm);
		border-collapse: collapse;
	}

	.cohort-table thead tr {
		background-color: oklch(0.97 0.002 265);
	}

	.cohort-table th {
		padding-inline: var(--space-3);
		padding-block: var(--space-2);
		font-weight: var(--weight-medium);
		color: oklch(0.45 0.01 265);
	}

	.sticky-col {
		position: sticky;
		inset-inline-start: 0;
		z-index: 10;
	}

	thead .sticky-col {
		background-color: oklch(0.97 0.002 265);
		text-align: start;
	}

	.center-col {
		text-align: center;
	}

	.period-col {
		text-align: center;
		min-inline-size: 60px;
	}

	.data-row {
		border-block-start: 1px solid oklch(0.95 0.002 265);
		&:hover {
			background-color: oklch(0.97 0.002 265 / 50%);
		}
	}

	.row-label {
		font-weight: var(--weight-medium);
		color: oklch(0.15 0.01 265);
		background-color: oklch(1 0 0);
	}

	.size-cell {
		color: oklch(0.45 0.01 265);
	}

	.cell {
		padding-inline: 0.25rem;
		padding-block: 0.25rem;
	}

	.heat-cell {
		inline-size: 100%;
		block-size: 2rem;
		border-radius: var(--radius-sm);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);

		&.avg {
			font-weight: var(--weight-bold);
		}

		&[data-level='high'] {
			background-color: oklch(0.6 0.18 160);
			color: oklch(1 0 0);
		}
		&[data-level='good'] {
			background-color: oklch(0.68 0.16 160);
			color: oklch(1 0 0);
		}
		&[data-level='mid'] {
			background-color: oklch(0.82 0.16 90);
			color: oklch(1 0 0);
		}
		&[data-level='low'] {
			background-color: oklch(0.75 0.16 55);
			color: oklch(0.25 0.02 55);
		}
		&[data-level='poor'] {
			background-color: oklch(0.65 0.2 25);
			color: oklch(0.25 0.02 25);
		}
		&[data-level='none'] {
			background-color: oklch(0.97 0.002 265);
		}
	}

	.avg-row {
		border-block-start: 2px solid oklch(0.9 0.005 265);
		background-color: oklch(0.97 0.002 265);
	}

	.avg-label {
		font-weight: var(--weight-semibold);
		color: oklch(0.15 0.01 265);
		background-color: oklch(0.97 0.002 265);
	}

	.legend {
		padding: var(--space-4);
		border-block-start: 1px solid oklch(0.95 0.002 265);
		display: flex;
		align-items: center;
		gap: var(--space-4);
		font-size: var(--text-xs);
	}

	.legend-label {
		color: oklch(0.55 0.01 265);
	}

	.legend-items {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.legend-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.legend-swatch {
		inline-size: 1rem;
		block-size: 1rem;
		border-radius: var(--radius-sm);

		&[data-level='high'] {
			background-color: oklch(0.6 0.18 160);
		}
		&[data-level='good'] {
			background-color: oklch(0.68 0.16 160);
		}
		&[data-level='mid'] {
			background-color: oklch(0.82 0.16 90);
		}
		&[data-level='low'] {
			background-color: oklch(0.75 0.16 55);
		}
		&[data-level='poor'] {
			background-color: oklch(0.65 0.2 25);
		}
	}
</style>

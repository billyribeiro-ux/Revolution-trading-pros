<script lang="ts">
	/**
	 * CohortMatrix - Enterprise Cohort Retention Heatmap
	 *
	 * Displays a retention matrix/heatmap showing user retention
	 * across cohorts and time periods.
	 */
	import type { CohortRow } from '$lib/api/analytics';

	type CohortPeriodValue = NonNullable<CohortRow['periods']>[number];

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

	// Format cohort identifiers while preserving API fallback shapes.
	function formatCohortLabel(row: CohortRow): string {
		if (!row.cohort_date) return row.cohort ?? 'Unknown';

		const date = new Date(row.cohort_date);
		if (Number.isNaN(date.getTime())) return row.cohort_date;

		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	function getPeriodData(row: CohortRow, period: number): CohortPeriodValue | null {
		if (!row.periods) return null;
		return row.periods[period] ?? null;
	}

	function getRetentionRate(periodData: CohortPeriodValue | null): number | null {
		if (periodData === null || periodData === undefined) return null;
		return typeof periodData === 'number' ? periodData : periodData.retention_rate;
	}

	function getPeriodTitle(periodData: CohortPeriodValue, retentionRate: number): string {
		if (typeof periodData === 'number') return `${retentionRate.toFixed(1)}%`;

		return metricType === 'retention'
			? `${periodData.active_users} users (${retentionRate.toFixed(1)}%)`
			: `$${periodData.total_revenue.toFixed(0)}`;
	}

	function getPeriodLabel(periodData: CohortPeriodValue, retentionRate: number): string {
		if (typeof periodData === 'number' || metricType === 'retention') {
			return `${retentionRate.toFixed(0)}%`;
		}

		return `$${periodData.total_revenue.toFixed(0)}`;
	}

	// Get available period numbers
	let periodNumbers = $derived(Array.from({ length: maxPeriods }, (_, i) => i));

	// Calculate average retention per period
	let periodAverages = $derived(
		periodNumbers.map((period) => {
			const values = data
				.filter((row) => row.periods && row.periods[period])
				.map((row) => {
					const periodData = (row.periods ?? {})[period];
					if (periodData === undefined) return 0;
					return typeof periodData === 'number' ? periodData : periodData.retention_rate;
				});

			if (values.length === 0) return null;
			return values.reduce((a, b) => a + b, 0) / values.length;
		})
	);

	let averageCohortSize = $derived(
		data.length === 0
			? 0
			: Math.round(
					data.reduce((sum, row) => sum + (row.cohort_size || row.size || 0), 0) / data.length
				)
	);
</script>

<div class="cohort-matrix">
	<div class="cohort-matrix__header">
		<h3>{title}</h3>
		<p>
			{metricType === 'retention' ? 'User retention by cohort week' : 'Revenue by cohort'}
		</p>
	</div>

	<div class="cohort-matrix__table-scroll">
		<table>
			<thead>
				<tr>
					<th class="cohort-matrix__heading cohort-matrix__heading--sticky">Cohort</th>
					<th class="cohort-matrix__heading cohort-matrix__heading--center"> Size </th>
					{#each periodNumbers as period (period)}
						<th class="cohort-matrix__heading cohort-matrix__heading--period">
							{period === 0 ? 'Day 0' : `Week ${period}`}
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each data as row (row.cohort_date ?? row.cohort)}
					<tr class="cohort-matrix__row">
						<td class="cohort-matrix__cohort">
							{formatCohortLabel(row)}
						</td>
						<td class="cohort-matrix__size">
							{(row.cohort_size ?? row.size ?? 0).toLocaleString()}
						</td>
						{#each periodNumbers as period (period)}
							{@const periodData = getPeriodData(row, period)}
							{@const retentionRate = getRetentionRate(periodData)}
							<td class="cohort-matrix__period">
								{#if periodData !== null && retentionRate !== null}
									<div
										class={{
											'retention-cell': true,
											'retention-cell--excellent': retentionRate >= 80,
											'retention-cell--strong': retentionRate >= 60 && retentionRate < 80,
											'retention-cell--moderate': retentionRate >= 40 && retentionRate < 60,
											'retention-cell--low': retentionRate >= 20 && retentionRate < 40,
											'retention-cell--critical': retentionRate > 0 && retentionRate < 20,
											'retention-cell--empty': retentionRate <= 0,
											'retention-cell--inverse': retentionRate >= 40
										}}
										title={getPeriodTitle(periodData, retentionRate)}
									>
										{getPeriodLabel(periodData, retentionRate)}
									</div>
								{:else}
									<div class="retention-cell retention-cell--missing"></div>
								{/if}
							</td>
						{/each}
					</tr>
				{:else}
					<tr>
						<td class="cohort-matrix__empty" colspan={periodNumbers.length + 2}>
							No cohort data available.
						</td>
					</tr>
				{/each}

				<!-- Averages Row -->
				<tr class="cohort-matrix__average">
					<td class="cohort-matrix__average-label">Average</td>
					<td class="cohort-matrix__size">
						{averageCohortSize.toLocaleString()}
					</td>
					{#each periodAverages as avg, i (i)}
						<td class="cohort-matrix__period">
							{#if avg !== null}
								<div
									class={{
										'retention-cell': true,
										'retention-cell--average': true,
										'retention-cell--excellent': avg >= 80,
										'retention-cell--strong': avg >= 60 && avg < 80,
										'retention-cell--moderate': avg >= 40 && avg < 60,
										'retention-cell--low': avg >= 20 && avg < 40,
										'retention-cell--critical': avg > 0 && avg < 20,
										'retention-cell--empty': avg <= 0,
										'retention-cell--inverse': avg >= 40
									}}
								>
									{avg.toFixed(0)}%
								</div>
							{:else}
								<div class="retention-cell retention-cell--empty"></div>
							{/if}
						</td>
					{/each}
				</tr>
			</tbody>
		</table>
	</div>

	<!-- Legend -->
	<div class="cohort-matrix__legend">
		<span class="cohort-matrix__legend-label">Retention:</span>
		<div class="cohort-matrix__legend-items">
			<div class="cohort-matrix__legend-item">
				<div class="cohort-matrix__swatch cohort-matrix__swatch--excellent"></div>
				<span>80%+</span>
			</div>
			<div class="cohort-matrix__legend-item">
				<div class="cohort-matrix__swatch cohort-matrix__swatch--strong"></div>
				<span>60-80%</span>
			</div>
			<div class="cohort-matrix__legend-item">
				<div class="cohort-matrix__swatch cohort-matrix__swatch--moderate"></div>
				<span>40-60%</span>
			</div>
			<div class="cohort-matrix__legend-item">
				<div class="cohort-matrix__swatch cohort-matrix__swatch--low"></div>
				<span>20-40%</span>
			</div>
			<div class="cohort-matrix__legend-item">
				<div class="cohort-matrix__swatch cohort-matrix__swatch--critical"></div>
				<span>&lt;20%</span>
			</div>
		</div>
	</div>
</div>

<style>
	.cohort-matrix {
		overflow: hidden;
		border: 1px solid #e5e7eb;
		border-radius: 0.75rem;
		background: #ffffff;
	}

	.cohort-matrix__header {
		border-bottom: 1px solid #f3f4f6;
		padding: 1rem;
	}

	.cohort-matrix__header h3 {
		margin: 0;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.5;
	}

	.cohort-matrix__header p {
		margin: 0.25rem 0 0;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.35;
	}

	.cohort-matrix__table-scroll {
		overflow-x: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	thead tr {
		background: #f9fafb;
	}

	.cohort-matrix__heading {
		padding: 0.5rem 0.75rem;
		color: #4b5563;
		font-weight: 500;
		text-align: left;
		white-space: nowrap;
	}

	.cohort-matrix__heading--center,
	.cohort-matrix__heading--period {
		text-align: center;
	}

	.cohort-matrix__heading--period {
		min-width: 3.75rem;
		padding-right: 0.5rem;
		padding-left: 0.5rem;
	}

	.cohort-matrix__heading--sticky,
	.cohort-matrix__cohort,
	.cohort-matrix__average-label {
		position: sticky;
		left: 0;
		z-index: 1;
	}

	.cohort-matrix__heading--sticky {
		background: #f9fafb;
	}

	.cohort-matrix__row {
		border-top: 1px solid #f3f4f6;
		transition: background 160ms ease;
	}

	.cohort-matrix__row:hover {
		background: rgba(249, 250, 251, 0.5);
	}

	.cohort-matrix__cohort {
		background: #ffffff;
		color: #111827;
		font-weight: 500;
	}

	.cohort-matrix__cohort,
	.cohort-matrix__size {
		padding: 0.5rem 0.75rem;
	}

	.cohort-matrix__size {
		color: #4b5563;
		text-align: center;
		white-space: nowrap;
	}

	.cohort-matrix__period {
		padding: 0.25rem;
	}

	.retention-cell {
		display: flex;
		width: 100%;
		height: 2rem;
		align-items: center;
		justify-content: center;
		border-radius: 0.25rem;
		color: #374151;
		font-size: 0.75rem;
		font-weight: 500;
		line-height: 1;
	}

	.retention-cell--excellent {
		background: #22c55e;
	}

	.retention-cell--strong {
		background: #4ade80;
	}

	.retention-cell--moderate {
		background: #facc15;
	}

	.retention-cell--low {
		background: #fb923c;
	}

	.retention-cell--critical {
		background: #f87171;
	}

	.retention-cell--empty,
	.retention-cell--missing {
		background: #f3f4f6;
	}

	.retention-cell--missing {
		color: transparent;
	}

	.retention-cell--inverse {
		color: #ffffff;
	}

	.retention-cell--average {
		font-weight: 700;
	}

	.cohort-matrix__average {
		border-top: 2px solid #e5e7eb;
		background: #f9fafb;
	}

	.cohort-matrix__average-label {
		padding: 0.5rem 0.75rem;
		background: #f9fafb;
		color: #111827;
		font-weight: 600;
	}

	.cohort-matrix__empty {
		padding: 2rem 1rem;
		color: #6b7280;
		text-align: center;
	}

	.cohort-matrix__legend {
		display: flex;
		align-items: center;
		gap: 1rem;
		border-top: 1px solid #f3f4f6;
		padding: 1rem;
		font-size: 0.75rem;
		line-height: 1rem;
	}

	.cohort-matrix__legend-label {
		color: #6b7280;
	}

	.cohort-matrix__legend-items {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.cohort-matrix__legend-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: #374151;
	}

	.cohort-matrix__swatch {
		width: 1rem;
		height: 1rem;
		border-radius: 0.25rem;
	}

	.cohort-matrix__swatch--excellent {
		background: #22c55e;
	}

	.cohort-matrix__swatch--strong {
		background: #4ade80;
	}

	.cohort-matrix__swatch--moderate {
		background: #facc15;
	}

	.cohort-matrix__swatch--low {
		background: #fb923c;
	}

	.cohort-matrix__swatch--critical {
		background: #f87171;
	}

	@media (max-width: 640px) {
		.cohort-matrix__legend {
			align-items: flex-start;
			flex-direction: column;
			gap: 0.5rem;
		}
	}
</style>

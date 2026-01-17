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

	// Get retention color based on value
	function getRetentionColor(value: number): string {
		if (value >= 80) return 'bg-green-500';
		if (value >= 60) return 'bg-green-400';
		if (value >= 40) return 'bg-yellow-400';
		if (value >= 20) return 'bg-orange-400';
		if (value > 0) return 'bg-red-400';
		return 'bg-gray-100';
	}

	// Get text color for contrast
	function getTextColor(value: number): string {
		return value >= 40 ? 'text-white' : 'text-gray-700';
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

<div class="bg-white rounded-xl border border-gray-200 overflow-hidden">
	<div class="p-4 border-b border-gray-100">
		<h3 class="text-lg font-semibold text-gray-900">{title}</h3>
		<p class="text-sm text-gray-500 mt-1">
			{metricType === 'retention' ? 'User retention by cohort week' : 'Revenue by cohort'}
		</p>
	</div>

	<div class="overflow-x-auto">
		<table class="w-full text-sm">
			<thead>
				<tr class="bg-gray-50">
					<th class="px-3 py-2 text-left font-medium text-gray-600 sticky left-0 bg-gray-50 z-10">
						Cohort
					</th>
					<th class="px-3 py-2 text-center font-medium text-gray-600"> Size </th>
					{#each periodNumbers as period}
						<th class="px-2 py-2 text-center font-medium text-gray-600 min-w-[60px]">
							{period === 0 ? 'Day 0' : `Week ${period}`}
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each data as row (row.cohort_date)}
					<tr class="border-t border-gray-100 hover:bg-gray-50/50">
						<td class="px-3 py-2 font-medium text-gray-900 sticky left-0 bg-white z-10">
							{formatDate(row.cohort_date ?? '')}
						</td>
						<td class="px-3 py-2 text-center text-gray-600">
							{(row.cohort_size ?? 0).toLocaleString()}
						</td>
						{#each periodNumbers as period}
							{@const periodData = row.periods ? row.periods[period] : null}
							<td class="px-1 py-1">
								{#if periodData && typeof periodData === 'object'}
									{@const retentionRate = periodData.retention_rate}
									<div
										class="w-full h-8 rounded flex items-center justify-center text-xs font-medium
											{getRetentionColor(retentionRate)}
											{getTextColor(retentionRate)}"
										title={metricType === 'retention'
											? `${periodData.active_users} users (${retentionRate.toFixed(1)}%)`
											: `$${periodData.total_revenue.toFixed(0)}`}
									>
										{metricType === 'retention'
											? `${periodData.retention_rate.toFixed(0)}%`
											: `$${periodData.total_revenue.toFixed(0)}`}
									</div>
								{:else}
									<div class="w-full h-8 rounded bg-gray-50"></div>
								{/if}
							</td>
						{/each}
					</tr>
				{/each}

				<!-- Averages Row -->
				<tr class="border-t-2 border-gray-200 bg-gray-50">
					<td class="px-3 py-2 font-semibold text-gray-900 sticky left-0 bg-gray-50 z-10">
						Average
					</td>
					<td class="px-3 py-2 text-center text-gray-600">
						{Math.round(
							data.reduce((sum, row) => sum + (row.cohort_size || row.size || 0), 0) / data.length
						).toLocaleString()}
					</td>
					{#each periodAverages as avg}
						<td class="px-1 py-1">
							{#if avg !== null}
								<div
									class="w-full h-8 rounded flex items-center justify-center text-xs font-bold
										{getRetentionColor(avg)} {getTextColor(avg)}"
								>
									{avg.toFixed(0)}%
								</div>
							{:else}
								<div class="w-full h-8 rounded bg-gray-100"></div>
							{/if}
						</td>
					{/each}
				</tr>
			</tbody>
		</table>
	</div>

	<!-- Legend -->
	<div class="p-4 border-t border-gray-100 flex items-center gap-4 text-xs">
		<span class="text-gray-500">Retention:</span>
		<div class="flex items-center gap-2">
			<div class="flex items-center gap-1">
				<div class="w-4 h-4 rounded bg-green-500"></div>
				<span>80%+</span>
			</div>
			<div class="flex items-center gap-1">
				<div class="w-4 h-4 rounded bg-green-400"></div>
				<span>60-80%</span>
			</div>
			<div class="flex items-center gap-1">
				<div class="w-4 h-4 rounded bg-yellow-400"></div>
				<span>40-60%</span>
			</div>
			<div class="flex items-center gap-1">
				<div class="w-4 h-4 rounded bg-orange-400"></div>
				<span>20-40%</span>
			</div>
			<div class="flex items-center gap-1">
				<div class="w-4 h-4 rounded bg-red-400"></div>
				<span>&lt;20%</span>
			</div>
		</div>
	</div>
</div>

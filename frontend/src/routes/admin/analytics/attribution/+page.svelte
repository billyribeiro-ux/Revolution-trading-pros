<script lang="ts">
	/**
	 * Attribution Report - Multi-Touch Attribution Analysis
	 *
	 * Compare attribution models and analyze marketing
	 * channel performance across the customer journey.
	 */
	import { onMount } from 'svelte';
	import { analyticsApi, type AttributionReport, type ChannelAttribution } from '$lib/api/analytics';
	import AttributionChart from '$lib/components/analytics/AttributionChart.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	import TimeSeriesChart from '$lib/components/analytics/TimeSeriesChart.svelte';

	let report: AttributionReport | null = null;
	let loading = true;
	let error: string | null = null;
	let selectedPeriod = '30d';
	let selectedModel = 'linear';

	const models = [
		{ value: 'first_touch', label: 'First Touch', description: 'Credits the first interaction' },
		{ value: 'last_touch', label: 'Last Touch', description: 'Credits the last interaction' },
		{ value: 'linear', label: 'Linear', description: 'Equal credit to all touchpoints' },
		{ value: 'time_decay', label: 'Time Decay', description: 'More credit to recent touchpoints' },
		{ value: 'position_based', label: 'Position Based', description: '40% first, 40% last, 20% middle' }
	];

	async function loadAttribution() {
		loading = true;
		error = null;
		try {
			report = await analyticsApi.getChannelAttribution(selectedModel, selectedPeriod);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load attribution data';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(event: CustomEvent<string>) {
		selectedPeriod = event.detail;
		loadAttribution();
	}

	function handleModelChange() {
		loadAttribution();
	}

	function formatCurrency(num: number): string {
		if (num >= 1000000) return '$' + (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return '$' + (num / 1000).toFixed(1) + 'K';
		return '$' + num.toFixed(0);
	}

	onMount(() => {
		loadAttribution();
	});

	// Calculate totals
	$: totalRevenue = report?.channels?.reduce((sum, c) => sum + c.attributed_revenue, 0) || 0;
	$: totalConversions = report?.channels?.reduce((sum, c) => sum + c.attributed_conversions, 0) || 0;
	$: totalTouchpoints = report?.channels?.reduce((sum, c) => sum + c.touchpoints, 0) || 0;
</script>

<svelte:head>
	<title>Attribution Report | Analytics</title>
</svelte:head>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<!-- Header -->
	<div class="flex items-center justify-between mb-8">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">Attribution Analysis</h1>
			<p class="text-sm text-gray-500 mt-1">Multi-touch attribution across marketing channels</p>
		</div>
		<div class="flex items-center gap-4">
			<PeriodSelector value={selectedPeriod} on:change={handlePeriodChange} />
			<button class="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 text-sm">
				Export Report
			</button>
		</div>
	</div>

	<!-- Model Selection -->
	<div class="bg-white rounded-xl border border-gray-200 p-6 mb-8">
		<h3 class="text-lg font-semibold text-gray-900 mb-4">Attribution Model</h3>
		<div class="grid grid-cols-1 md:grid-cols-5 gap-4">
			{#each models as model}
				<button
					on:click={() => {
						selectedModel = model.value;
						handleModelChange();
					}}
					class="p-4 rounded-xl border-2 text-left transition-all
						{selectedModel === model.value
						? 'border-blue-600 bg-blue-50'
						: 'border-gray-200 hover:border-gray-300'}"
				>
					<div class="font-medium text-gray-900 mb-1">{model.label}</div>
					<div class="text-xs text-gray-500">{model.description}</div>
				</button>
			{/each}
		</div>
	</div>

	{#if loading}
		<div class="flex items-center justify-center py-20">
			<div class="w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
		</div>
	{:else if error}
		<div class="bg-red-50 border border-red-200 rounded-xl p-6 text-center">
			<p class="text-red-600">{error}</p>
			<button
				on:click={loadAttribution}
				class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
			>
				Retry
			</button>
		</div>
	{:else if report}
		<!-- Summary Stats -->
		<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
			<div class="bg-white rounded-xl border border-gray-200 p-6">
				<div class="text-sm text-gray-500 mb-1">Attributed Revenue</div>
				<div class="text-3xl font-bold text-gray-900">{formatCurrency(totalRevenue)}</div>
			</div>
			<div class="bg-white rounded-xl border border-gray-200 p-6">
				<div class="text-sm text-gray-500 mb-1">Total Conversions</div>
				<div class="text-3xl font-bold text-gray-900">{totalConversions.toLocaleString()}</div>
			</div>
			<div class="bg-white rounded-xl border border-gray-200 p-6">
				<div class="text-sm text-gray-500 mb-1">Total Touchpoints</div>
				<div class="text-3xl font-bold text-gray-900">{totalTouchpoints.toLocaleString()}</div>
			</div>
			<div class="bg-white rounded-xl border border-gray-200 p-6">
				<div class="text-sm text-gray-500 mb-1">Avg Touchpoints/Conv</div>
				<div class="text-3xl font-bold text-gray-900">
					{totalConversions > 0 ? (totalTouchpoints / totalConversions).toFixed(1) : '0'}
				</div>
			</div>
		</div>

		<!-- Attribution Chart -->
		{#if report.channels && report.channels.length > 0}
			<div class="mb-8">
				<AttributionChart channels={report.channels} model={selectedModel} />
			</div>

			<!-- Detailed Table -->
			<div class="bg-white rounded-xl border border-gray-200 overflow-hidden mb-8">
				<div class="p-4 border-b border-gray-100">
					<h3 class="font-semibold text-gray-900">Channel Performance</h3>
				</div>
				<div class="overflow-x-auto">
					<table class="w-full text-sm">
						<thead class="bg-gray-50">
							<tr>
								<th class="text-left py-3 px-4 font-medium text-gray-600">Channel</th>
								<th class="text-right py-3 px-4 font-medium text-gray-600">Touchpoints</th>
								<th class="text-right py-3 px-4 font-medium text-gray-600">Conversions</th>
								<th class="text-right py-3 px-4 font-medium text-gray-600">Revenue</th>
								<th class="text-right py-3 px-4 font-medium text-gray-600">Revenue Share</th>
								<th class="text-right py-3 px-4 font-medium text-gray-600">Avg Order Value</th>
								<th class="text-right py-3 px-4 font-medium text-gray-600">Assisted Conv.</th>
							</tr>
						</thead>
						<tbody class="divide-y divide-gray-100">
							{#each report.channels as channel}
								<tr class="hover:bg-gray-50">
									<td class="py-3 px-4">
										<span class="font-medium text-gray-900 capitalize">{channel.channel}</span>
									</td>
									<td class="py-3 px-4 text-right text-gray-600">
										{channel.touchpoints.toLocaleString()}
									</td>
									<td class="py-3 px-4 text-right text-gray-600">
										{channel.attributed_conversions.toFixed(1)}
									</td>
									<td class="py-3 px-4 text-right font-medium text-gray-900">
										{formatCurrency(channel.attributed_revenue)}
									</td>
									<td class="py-3 px-4 text-right">
										<div class="flex items-center justify-end gap-2">
											<div class="w-16 h-2 bg-gray-100 rounded-full overflow-hidden">
												<div
													class="h-full bg-blue-500 rounded-full"
													style="width: {channel.revenue_share}%"
												></div>
											</div>
											<span class="text-gray-600">{channel.revenue_share.toFixed(1)}%</span>
										</div>
									</td>
									<td class="py-3 px-4 text-right text-gray-600">
										{channel.attributed_conversions > 0
											? formatCurrency(channel.attributed_revenue / channel.attributed_conversions)
											: '-'}
									</td>
									<td class="py-3 px-4 text-right text-gray-600">
										{channel.assisted_conversions.toLocaleString()}
									</td>
								</tr>
							{/each}
						</tbody>
						<tfoot class="bg-gray-50 font-medium">
							<tr>
								<td class="py-3 px-4 text-gray-900">Total</td>
								<td class="py-3 px-4 text-right text-gray-900">{totalTouchpoints.toLocaleString()}</td>
								<td class="py-3 px-4 text-right text-gray-900">{totalConversions.toFixed(1)}</td>
								<td class="py-3 px-4 text-right text-gray-900">{formatCurrency(totalRevenue)}</td>
								<td class="py-3 px-4 text-right text-gray-900">100%</td>
								<td class="py-3 px-4 text-right text-gray-900">
									{totalConversions > 0 ? formatCurrency(totalRevenue / totalConversions) : '-'}
								</td>
								<td class="py-3 px-4 text-right text-gray-900">
									{report.channels.reduce((sum, c) => sum + c.assisted_conversions, 0).toLocaleString()}
								</td>
							</tr>
						</tfoot>
					</table>
				</div>
			</div>

			<!-- Conversion Paths -->
			{#if report.conversion_paths && (report.conversion_paths as any).length > 0}
				<div class="bg-white rounded-xl border border-gray-200 p-6">
					<h3 class="text-lg font-semibold text-gray-900 mb-4">Top Conversion Paths</h3>
					<div class="space-y-3">
						{#each report.conversion_paths.slice(0, 10) as path, i}
							<div class="flex items-center gap-4 p-3 bg-gray-50 rounded-lg">
								<span class="text-sm text-gray-400 w-6">{i + 1}.</span>
								<div class="flex-1 flex items-center gap-2 flex-wrap">
									{#each path.channels as channel, j}
										<span class="px-2 py-1 bg-white border border-gray-200 rounded text-sm capitalize">
											{channel}
										</span>
										{#if j < path.channels.length - 1}
											<span class="text-gray-400">→</span>
										{/if}
									{/each}
								</div>
								<div class="text-right">
									<div class="font-medium text-gray-900">{path.conversions} conv</div>
									<div class="text-sm text-gray-500">{formatCurrency(path.revenue)}</div>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		{:else}
			<div class="bg-white rounded-xl border border-gray-200 p-12 text-center">
				<div class="text-4xl mb-4">📍</div>
				<h3 class="text-lg font-medium text-gray-900 mb-2">No Attribution Data</h3>
				<p class="text-gray-500">Attribution data will appear once conversions are tracked</p>
			</div>
		{/if}
	{/if}
</div>

<script lang="ts">
	/**
	 * Analytics Dashboard - Enterprise Analytics Hub
	 *
	 * Main analytics dashboard integrating KPIs, funnels, cohorts,
	 * attribution, and real-time metrics.
	 */
	import { onMount } from 'svelte';
	import { analyticsApi, type DashboardData } from '$lib/api/analytics';
	import KpiGrid from '$lib/components/analytics/KpiGrid.svelte';
	import FunnelChart from '$lib/components/analytics/FunnelChart.svelte';
	import CohortMatrix from '$lib/components/analytics/CohortMatrix.svelte';
	import TimeSeriesChart from '$lib/components/analytics/TimeSeriesChart.svelte';
	import RealTimeWidget from '$lib/components/analytics/RealTimeWidget.svelte';
	import AttributionChart from '$lib/components/analytics/AttributionChart.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';

	let dashboardData: DashboardData | null = null;
	let loading = true;
	let error: string | null = null;
	let selectedPeriod = '30d';
	let attributionModel = 'linear';

	// Navigation tabs
	const tabs = [
		{ id: 'overview', label: 'Overview', icon: '📊' },
		{ id: 'funnels', label: 'Funnels', icon: '🔻' },
		{ id: 'cohorts', label: 'Cohorts', icon: '👥' },
		{ id: 'attribution', label: 'Attribution', icon: '🎯' }
	];
	let activeTab = 'overview';

	async function loadDashboard() {
		loading = true;
		error = null;
		try {
			dashboardData = await analyticsApi.getDashboard(selectedPeriod);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load dashboard';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(event: CustomEvent<string>) {
		selectedPeriod = event.detail;
		loadDashboard();
	}

	onMount(() => {
		loadDashboard();
	});

	// Format revenue time series for chart
	$: revenueTimeSeries =
		dashboardData?.time_series?.revenue?.map((item) => ({
			date: item.date,
			value: item.value,
			label: '$' + item.value.toLocaleString()
		})) || [];

	// Format users time series for chart
	$: usersTimeSeries =
		dashboardData?.time_series?.users?.map((item) => ({
			date: item.date,
			value: item.value
		})) || [];
</script>

<svelte:head>
	<title>Analytics Dashboard | Revolution Trading</title>
</svelte:head>

<div class="min-h-screen bg-gray-50">
	<!-- Header -->
	<div class="bg-white border-b border-gray-200">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-2xl font-bold text-gray-900">Analytics Dashboard</h1>
					<p class="text-sm text-gray-500 mt-1">
						Enterprise insights and performance metrics
					</p>
				</div>
				<div class="flex items-center gap-4">
					<PeriodSelector value={selectedPeriod} on:change={handlePeriodChange} />
					<a
						href="/admin/analytics/events"
						class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors text-sm font-medium"
					>
						Event Explorer
					</a>
				</div>
			</div>

			<!-- Tab Navigation -->
			<div class="flex items-center gap-1 mt-6 border-b border-gray-200 -mb-px">
				{#each tabs as tab}
					<button
						class="px-4 py-3 text-sm font-medium border-b-2 transition-colors
							{activeTab === tab.id
							? 'border-blue-600 text-blue-600'
							: 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
						on:click={() => (activeTab = tab.id)}
					>
						<span class="mr-2">{tab.icon}</span>
						{tab.label}
					</button>
				{/each}
			</div>
		</div>
	</div>

	<!-- Content -->
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		{#if loading}
			<div class="flex items-center justify-center py-20">
				<div class="w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
			</div>
		{:else if error}
			<div class="bg-red-50 border border-red-200 rounded-xl p-6 text-center">
				<p class="text-red-600">{error}</p>
				<button
					class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
					on:click={loadDashboard}
				>
					Retry
				</button>
			</div>
		{:else if dashboardData}
			<!-- Overview Tab -->
			{#if activeTab === 'overview'}
				<div class="space-y-8">
					<!-- Real-time Widget -->
					<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
						<div class="lg:col-span-2">
							<RealTimeWidget />
						</div>
						<div class="bg-white rounded-xl border border-gray-200 p-6">
							<h3 class="text-lg font-semibold text-gray-900 mb-4">Quick Actions</h3>
							<div class="space-y-3">
								<a
									href="/admin/analytics/segments"
									class="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors"
								>
									<span class="text-2xl">👥</span>
									<div>
										<div class="font-medium text-gray-900">Segments</div>
										<div class="text-sm text-gray-500">Manage user segments</div>
									</div>
								</a>
								<a
									href="/admin/analytics/goals"
									class="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors"
								>
									<span class="text-2xl">🎯</span>
									<div>
										<div class="font-medium text-gray-900">Goals</div>
										<div class="text-sm text-gray-500">Track conversion goals</div>
									</div>
								</a>
								<a
									href="/admin/analytics/reports"
									class="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors"
								>
									<span class="text-2xl">📈</span>
									<div>
										<div class="font-medium text-gray-900">Reports</div>
										<div class="text-sm text-gray-500">Custom report builder</div>
									</div>
								</a>
							</div>
						</div>
					</div>

					<!-- KPI Grid -->
					{#if dashboardData.kpis}
						<KpiGrid kpis={dashboardData.kpis} />
					{/if}

					<!-- Time Series Charts -->
					<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
						{#if revenueTimeSeries.length > 0}
							<TimeSeriesChart
								data={revenueTimeSeries}
								title="Revenue Trend"
								color="#10B981"
								formatValue={(v) => '$' + v.toLocaleString()}
							/>
						{/if}
						{#if usersTimeSeries.length > 0}
							<TimeSeriesChart
								data={usersTimeSeries}
								title="Active Users"
								color="#3B82F6"
								formatValue={(v) => v.toLocaleString()}
							/>
						{/if}
					</div>

					<!-- Top Performing -->
					<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
						<!-- Top Pages -->
						<div class="bg-white rounded-xl border border-gray-200 p-6">
							<h3 class="text-lg font-semibold text-gray-900 mb-4">Top Pages</h3>
							{#if dashboardData.top_pages && dashboardData.top_pages.length > 0}
								<div class="space-y-3">
									{#each dashboardData.top_pages.slice(0, 10) as page, i}
										<div class="flex items-center justify-between">
											<div class="flex items-center gap-3">
												<span class="text-sm text-gray-400 w-6">{i + 1}.</span>
												<span class="text-sm text-gray-900 truncate max-w-[300px]">
													{page.page_path}
												</span>
											</div>
											<span class="text-sm font-medium text-gray-600">
												{page.views.toLocaleString()}
											</span>
										</div>
									{/each}
								</div>
							{:else}
								<p class="text-gray-500 text-sm">No page data available</p>
							{/if}
						</div>

						<!-- Top Events -->
						<div class="bg-white rounded-xl border border-gray-200 p-6">
							<h3 class="text-lg font-semibold text-gray-900 mb-4">Top Events</h3>
							{#if dashboardData.top_events && dashboardData.top_events.length > 0}
								<div class="space-y-3">
									{#each dashboardData.top_events.slice(0, 10) as event, i}
										<div class="flex items-center justify-between">
											<div class="flex items-center gap-3">
												<span class="text-sm text-gray-400 w-6">{i + 1}.</span>
												<span class="text-sm text-gray-900">{event.event_name}</span>
											</div>
											<span class="text-sm font-medium text-gray-600">
												{event.count.toLocaleString()}
											</span>
										</div>
									{/each}
								</div>
							{:else}
								<p class="text-gray-500 text-sm">No event data available</p>
							{/if}
						</div>
					</div>
				</div>
			{/if}

			<!-- Funnels Tab -->
			{#if activeTab === 'funnels'}
				<div class="space-y-8">
					<div class="flex items-center justify-between">
						<h2 class="text-xl font-semibold text-gray-900">Conversion Funnels</h2>
						<a
							href="/admin/analytics/funnels/create"
							class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm font-medium"
						>
							Create Funnel
						</a>
					</div>

					{#if dashboardData.funnels && dashboardData.funnels.length > 0}
						<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
							{#each dashboardData.funnels as funnel}
								<FunnelChart
									steps={funnel.steps}
									title={funnel.name}
									showDropoff={true}
								/>
							{/each}
						</div>
					{:else}
						<div class="bg-white rounded-xl border border-gray-200 p-12 text-center">
							<div class="text-4xl mb-4">🔻</div>
							<h3 class="text-lg font-medium text-gray-900 mb-2">No Funnels Yet</h3>
							<p class="text-gray-500 mb-6">Create your first funnel to track user journeys</p>
							<a
								href="/admin/analytics/funnels/create"
								class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 inline-block"
							>
								Create First Funnel
							</a>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Cohorts Tab -->
			{#if activeTab === 'cohorts'}
				<div class="space-y-8">
					<div class="flex items-center justify-between">
						<h2 class="text-xl font-semibold text-gray-900">Cohort Analysis</h2>
						<div class="flex items-center gap-3">
							<select
								class="px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
							>
								<option value="weekly">Weekly Cohorts</option>
								<option value="monthly">Monthly Cohorts</option>
								<option value="daily">Daily Cohorts</option>
							</select>
							<a
								href="/admin/analytics/cohorts/create"
								class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm font-medium"
							>
								Create Cohort
							</a>
						</div>
					</div>

					{#if dashboardData.cohorts && dashboardData.cohorts.length > 0}
						{#each dashboardData.cohorts as cohort}
							<CohortMatrix
								data={cohort.retention_matrix}
								title={cohort.name}
							/>
						{/each}
					{:else}
						<div class="bg-white rounded-xl border border-gray-200 p-12 text-center">
							<div class="text-4xl mb-4">👥</div>
							<h3 class="text-lg font-medium text-gray-900 mb-2">No Cohort Data</h3>
							<p class="text-gray-500 mb-6">Start tracking user retention with cohort analysis</p>
							<a
								href="/admin/analytics/cohorts/create"
								class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 inline-block"
							>
								Create First Cohort
							</a>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Attribution Tab -->
			{#if activeTab === 'attribution'}
				<div class="space-y-8">
					<div class="flex items-center justify-between">
						<h2 class="text-xl font-semibold text-gray-900">Channel Attribution</h2>
						<div class="flex items-center gap-3">
							<select
								bind:value={attributionModel}
								class="px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
							>
								<option value="first_touch">First Touch</option>
								<option value="last_touch">Last Touch</option>
								<option value="linear">Linear</option>
								<option value="time_decay">Time Decay</option>
								<option value="position_based">Position Based</option>
							</select>
						</div>
					</div>

					{#if dashboardData.attribution && dashboardData.attribution.channels}
						<AttributionChart
							channels={dashboardData.attribution.channels}
							model={attributionModel}
						/>

						<!-- Attribution Comparison -->
						<div class="bg-white rounded-xl border border-gray-200 p-6">
							<h3 class="text-lg font-semibold text-gray-900 mb-4">Model Comparison</h3>
							<div class="overflow-x-auto">
								<table class="w-full text-sm">
									<thead>
										<tr class="border-b border-gray-200">
											<th class="text-left py-3 px-4 font-medium text-gray-600">Channel</th>
											<th class="text-right py-3 px-4 font-medium text-gray-600">First Touch</th>
											<th class="text-right py-3 px-4 font-medium text-gray-600">Last Touch</th>
											<th class="text-right py-3 px-4 font-medium text-gray-600">Linear</th>
											<th class="text-right py-3 px-4 font-medium text-gray-600">Time Decay</th>
											<th class="text-right py-3 px-4 font-medium text-gray-600">Position Based</th>
										</tr>
									</thead>
									<tbody>
										{#each dashboardData.attribution.channels as channel}
											<tr class="border-b border-gray-100 hover:bg-gray-50">
												<td class="py-3 px-4 font-medium text-gray-900 capitalize">{channel.channel}</td>
												<td class="py-3 px-4 text-right text-gray-600">{channel.first_touch_share?.toFixed(1) || '-'}%</td>
												<td class="py-3 px-4 text-right text-gray-600">{channel.last_touch_share?.toFixed(1) || '-'}%</td>
												<td class="py-3 px-4 text-right text-gray-600">{channel.linear_share?.toFixed(1) || '-'}%</td>
												<td class="py-3 px-4 text-right text-gray-600">{channel.time_decay_share?.toFixed(1) || '-'}%</td>
												<td class="py-3 px-4 text-right text-gray-600">{channel.position_based_share?.toFixed(1) || '-'}%</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						</div>
					{:else}
						<div class="bg-white rounded-xl border border-gray-200 p-12 text-center">
							<div class="text-4xl mb-4">🎯</div>
							<h3 class="text-lg font-medium text-gray-900 mb-2">No Attribution Data</h3>
							<p class="text-gray-500">Attribution data will appear once conversions are tracked</p>
						</div>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>

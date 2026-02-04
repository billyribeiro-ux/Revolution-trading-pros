<script lang="ts">
	/**
	 * Reports - Custom Report Builder
	 *
	 * Create, schedule, and manage custom analytics reports
	 * with multiple visualization options.
	 */
	import { browser } from '$app/environment';
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';

	interface Report {
		id: string;
		name: string;
		description?: string;
		type: 'dashboard' | 'table' | 'chart' | 'mixed';
		schedule?: {
			frequency: 'daily' | 'weekly' | 'monthly';
			recipients: string[];
		};
		created_at: string;
		last_run?: string;
		status: 'active' | 'paused' | 'draft';
	}

	let reports = $state<Report[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let showCreateModal = $state(false);
	let activeFilter = $state<'all' | 'active' | 'scheduled' | 'draft'>('all');

	// New report form
	let newReport = $state({
		name: '',
		description: '',
		type: 'dashboard' as 'dashboard' | 'table' | 'chart' | 'mixed',
		metrics: [] as string[],
		dimensions: [] as string[],
		schedule: false,
		frequency: 'weekly' as 'daily' | 'weekly' | 'monthly',
		recipients: ''
	});

	// Available metrics
	const availableMetrics = [
		{ value: 'revenue', label: 'Revenue' },
		{ value: 'users', label: 'Users' },
		{ value: 'sessions', label: 'Sessions' },
		{ value: 'page_views', label: 'Page Views' },
		{ value: 'conversions', label: 'Conversions' },
		{ value: 'conversion_rate', label: 'Conversion Rate' },
		{ value: 'arpu', label: 'ARPU' },
		{ value: 'ltv', label: 'LTV' },
		{ value: 'churn_rate', label: 'Churn Rate' },
		{ value: 'retention_rate', label: 'Retention Rate' }
	];

	// Available dimensions
	const availableDimensions = [
		{ value: 'date', label: 'Date' },
		{ value: 'channel', label: 'Channel' },
		{ value: 'country', label: 'Country' },
		{ value: 'device', label: 'Device' },
		{ value: 'browser', label: 'Browser' },
		{ value: 'page', label: 'Page' },
		{ value: 'campaign', label: 'Campaign' },
		{ value: 'segment', label: 'Segment' }
	];

	async function loadReports() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getReports();
			reports = response.reports || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load reports';
		} finally {
			loading = false;
		}
	}

	async function createReport() {
		try {
			await analyticsApi.createReport({
				name: newReport.name,
				description: newReport.description,
				type: newReport.type,
				config: {
					metrics: newReport.metrics,
					dimensions: newReport.dimensions
				},
				...(newReport.schedule && {
					schedule: {
						frequency: newReport.frequency,
						recipients: newReport.recipients.split(',').map((e) => e.trim())
					}
				})
			});
			showCreateModal = false;
			newReport = {
				name: '',
				description: '',
				type: 'dashboard',
				metrics: [],
				dimensions: [],
				schedule: false,
				frequency: 'weekly',
				recipients: ''
			};
			loadReports();
		} catch (e) {
			alert(e instanceof Error ? e.message : 'Failed to create report');
		}
	}

	function toggleMetric(metric: string) {
		if (newReport.metrics.includes(metric)) {
			newReport.metrics = newReport.metrics.filter((m) => m !== metric);
		} else {
			newReport.metrics = [...newReport.metrics, metric];
		}
	}

	function toggleDimension(dimension: string) {
		if (newReport.dimensions.includes(dimension)) {
			newReport.dimensions = newReport.dimensions.filter((d) => d !== dimension);
		} else {
			newReport.dimensions = [...newReport.dimensions, dimension];
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (!browser) return;

		const init = async () => {
			// Load connection status first
			await connections.load();
			connectionLoading = false;

			// Only load data if analytics is connected
			if (getIsAnalyticsConnected()) {
				await loadReports();
			} else {
				loading = false;
			}
		};
		init();
	});

	const filteredReports = $derived(
		reports.filter((report) => {
			if (activeFilter === 'all') return true;
			if (activeFilter === 'active') return report.status === 'active';
			if (activeFilter === 'scheduled') return report.schedule;
			if (activeFilter === 'draft') return report.status === 'draft';
			return true;
		})
	);

	// Report type icons
	const typeIcons: Record<string, string> = {
		dashboard: '📊',
		table: '📋',
		chart: '📈',
		mixed: '🔀'
	};
</script>

<svelte:head>
	<title>Reports | Analytics</title>
</svelte:head>

<div class="bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Header -->
		<div class="flex items-center justify-between mb-8">
			<div>
				<h1 class="text-2xl font-bold text-white">Reports</h1>
				<p class="text-sm text-slate-400 mt-1">Create and manage custom analytics reports</p>
			</div>
			{#if getIsAnalyticsConnected()}
				<button
					onclick={() => (showCreateModal = true)}
					class="px-4 py-2 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-500 hover:to-blue-500 text-white rounded-lg text-sm font-medium transition-all shadow-lg"
				>
					Create Report
				</button>
			{/if}
		</div>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-purple-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-12 h-12 border-4 border-purple-500 rounded-full animate-spin border-t-transparent"
					></div>
				</div>
			</div>
		{:else if !getIsAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Filters -->
			<div class="flex items-center gap-2 mb-6">
				{#each [{ value: 'all', label: 'All Reports' }, { value: 'active', label: 'Active' }, { value: 'scheduled', label: 'Scheduled' }, { value: 'draft', label: 'Drafts' }] as filter}
					<button
						onclick={() => (activeFilter = filter.value as typeof activeFilter)}
						class="px-4 py-2 rounded-lg text-sm font-medium transition-all
							{activeFilter === filter.value
							? 'bg-white text-slate-900'
							: 'bg-white/5 border border-white/10 text-slate-400 hover:bg-white/10 hover:text-white'}"
					>
						{filter.label}
					</button>
				{/each}
			</div>

			{#if loading}
				<div class="flex items-center justify-center py-20">
					<div
						class="w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"
					></div>
				</div>
			{:else if error}
				<div class="bg-red-50 border border-red-200 rounded-xl p-6 text-center">
					<p class="text-red-600">{error}</p>
					<button
						onclick={loadReports}
						class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
					>
						Retry
					</button>
				</div>
			{:else if filteredReports.length === 0}
				<div class="bg-white rounded-xl border border-gray-200 p-12 text-center">
					<div class="text-4xl mb-4">📊</div>
					<h3 class="text-lg font-medium text-gray-900 mb-2">No Reports Yet</h3>
					<p class="text-gray-500 mb-6">Create your first custom report</p>
					<button
						onclick={() => (showCreateModal = true)}
						class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 inline-block"
					>
						Create Your First Report
					</button>
				</div>
			{:else}
				<!-- Reports Grid -->
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each filteredReports as report}
						<div
							class="bg-white rounded-xl border border-gray-200 p-6 hover:shadow-lg transition-shadow"
						>
							<div class="flex items-start justify-between mb-4">
								<div class="flex items-center gap-3">
									<span class="text-2xl">{typeIcons[report.type] || '📊'}</span>
									<div>
										<h3 class="font-semibold text-gray-900">{report.name}</h3>
										<span
											class="text-xs px-2 py-0.5 rounded capitalize
									{report.status === 'active'
												? 'bg-green-100 text-green-700'
												: report.status === 'draft'
													? 'bg-gray-100 text-gray-700'
													: 'bg-yellow-100 text-yellow-700'}"
										>
											{report.status}
										</span>
									</div>
								</div>
								<button class="text-gray-400 hover:text-gray-600">⋮</button>
							</div>

							{#if report.description}
								<p class="text-sm text-gray-500 mb-4 line-clamp-2">{report.description}</p>
							{/if}

							<div class="space-y-2 text-sm text-gray-500">
								<div class="flex items-center justify-between">
									<span>Created</span>
									<span>{formatDate(report.created_at)}</span>
								</div>
								{#if report.last_run}
									<div class="flex items-center justify-between">
										<span>Last Run</span>
										<span>{formatDate(report.last_run)}</span>
									</div>
								{/if}
								{#if report.schedule}
									<div class="flex items-center justify-between">
										<span>Schedule</span>
										<span class="capitalize">{report.schedule.frequency}</span>
									</div>
								{/if}
							</div>

							<div class="mt-4 pt-4 border-t border-gray-100 flex gap-2">
								<a
									href="/admin/analytics/reports/{report.id}"
									class="flex-1 px-3 py-2 text-sm text-center bg-blue-600 text-white rounded-lg hover:bg-blue-700"
								>
									View Report
								</a>
								<button
									class="px-3 py-2 text-sm border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50"
								>
									Edit
								</button>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Report Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-gray-100">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-gray-900">Create Report</h2>
					<button
						onclick={() => (showCreateModal = false)}
						class="text-gray-400 hover:text-gray-600 text-2xl"
					>
						✕
					</button>
				</div>
			</div>

			<div class="p-6 space-y-6">
				<!-- Basic Info -->
				<div class="space-y-4">
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-1" for="report-name"
							>Name</label
						>
						<input
							type="text"
							id="report-name" name="report-name"
							bind:value={newReport.name}
							placeholder="e.g., Weekly Revenue Report"
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
						/>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-1" for="report-description"
							>Description</label
						>
						<textarea
							id="report-description"
							bind:value={newReport.description}
							placeholder="Describe this report..."
							rows={2}
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
						></textarea>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-1" for="report-type"
							>Type</label
						>
						<select
							id="report-type"
							bind:value={newReport.type}
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
						>
							<option value="dashboard">Dashboard</option>
							<option value="table">Table</option>
							<option value="chart">Chart</option>
							<option value="mixed">Mixed</option>
						</select>
					</div>
				</div>

				<!-- Metrics -->
				<div>
					<span class="block text-sm font-medium text-gray-700 mb-2">Metrics</span>
					<div class="flex flex-wrap gap-2">
						{#each availableMetrics as metric}
							<button
								onclick={() => toggleMetric(metric.value)}
								class="px-3 py-1.5 rounded-lg text-sm transition-all
									{newReport.metrics.includes(metric.value)
									? 'bg-blue-600 text-white'
									: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
							>
								{metric.label}
							</button>
						{/each}
					</div>
				</div>

				<!-- Dimensions -->
				<div>
					<span class="block text-sm font-medium text-gray-700 mb-2">Dimensions</span>
					<div class="flex flex-wrap gap-2">
						{#each availableDimensions as dimension}
							<button
								onclick={() => toggleDimension(dimension.value)}
								class="px-3 py-1.5 rounded-lg text-sm transition-all
									{newReport.dimensions.includes(dimension.value)
									? 'bg-purple-600 text-white'
									: 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
							>
								{dimension.label}
							</button>
						{/each}
					</div>
				</div>

				<!-- Schedule -->
				<div class="border-t border-gray-100 pt-6">
					<label class="flex items-center gap-3 cursor-pointer">
						<input
							id="page-newreport-schedule" name="page-newreport-schedule" type="checkbox"
							bind:checked={newReport.schedule}
							class="w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
						/>
						<span class="font-medium text-gray-900">Schedule this report</span>
					</label>

					{#if newReport.schedule}
						<div class="mt-4 space-y-4 pl-7">
							<div>
								<label for="report-frequency" class="block text-sm font-medium text-gray-700 mb-1"
									>Frequency</label
								>
								<select
									bind:value={newReport.frequency}
									class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
								>
									<option value="daily">Daily</option>
									<option value="weekly">Weekly</option>
									<option value="monthly">Monthly</option>
								</select>
							</div>
							<div>
								<label for="report-recipients" class="block text-sm font-medium text-gray-700 mb-1"
									>Recipients (comma-separated)</label
								>
								<input
									id="report-recipients" name="report-recipients"
									type="text"
									bind:value={newReport.recipients}
									placeholder="email@example.com, another@example.com"
									class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
								/>
							</div>
						</div>
					{/if}
				</div>
			</div>

			<div class="p-6 border-t border-gray-100 flex justify-end gap-3">
				<button
					onclick={() => (showCreateModal = false)}
					class="px-4 py-2 text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50"
				>
					Cancel
				</button>
				<button
					onclick={createReport}
					disabled={!newReport.name || newReport.metrics.length === 0}
					class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
				>
					Create Report
				</button>
			</div>
		</div>
	</div>
{/if}

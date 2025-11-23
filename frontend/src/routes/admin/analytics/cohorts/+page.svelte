<script lang="ts">
	/**
	 * Cohort Analysis - User Retention Analysis
	 *
	 * Analyze user behavior patterns over time with
	 * cohort-based retention matrices.
	 */
	import { onMount } from 'svelte';
	import { analyticsApi } from '$lib/api/analytics';
	import CohortMatrix from '$lib/components/analytics/CohortMatrix.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';

	interface Cohort {
		key: string;
		name: string;
		description?: string;
		type: 'signup' | 'first_purchase' | 'custom';
		granularity: 'daily' | 'weekly' | 'monthly';
		retention_matrix: Array<{
			cohort: string;
			size: number;
			periods: number[];
		}>;
		summary?: {
			avg_retention: number;
			best_cohort: string;
			worst_cohort: string;
		};
	}

	let cohorts: Cohort[] = [];
	let loading = true;
	let error: string | null = null;
	let selectedPeriod = '90d';
	let selectedGranularity = 'weekly';
	let selectedCohort: Cohort | null = null;
	let showCreateModal = false;

	// New cohort form
	let newCohort = {
		name: '',
		description: '',
		type: 'signup' as 'signup' | 'first_purchase' | 'custom',
		granularity: 'weekly' as 'daily' | 'weekly' | 'monthly',
		start_event: '',
		return_event: ''
	};

	async function loadCohorts() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getCohorts({
				period: selectedPeriod,
				granularity: selectedGranularity
			});
			cohorts = response.cohorts || [];
			if (cohorts.length > 0 && !selectedCohort) {
				selectedCohort = cohorts[0];
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load cohorts';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(event: CustomEvent<string>) {
		selectedPeriod = event.detail;
		loadCohorts();
	}

	function handleGranularityChange() {
		loadCohorts();
	}

	async function createCohort() {
		try {
			await analyticsApi.createCohort({
				name: newCohort.name,
				description: newCohort.description,
				type: newCohort.type,
				granularity: newCohort.granularity,
				start_event: newCohort.start_event || undefined,
				return_event: newCohort.return_event || undefined
			});
			showCreateModal = false;
			newCohort = {
				name: '',
				description: '',
				type: 'signup',
				granularity: 'weekly',
				start_event: '',
				return_event: ''
			};
			loadCohorts();
		} catch (e) {
			alert(e instanceof Error ? e.message : 'Failed to create cohort');
		}
	}

	onMount(() => {
		loadCohorts();
	});

	// Calculate overall retention metrics
	$: overallMetrics = selectedCohort
		? {
				avgWeek1: calculateAvgRetention(selectedCohort.retention_matrix, 0),
				avgWeek4: calculateAvgRetention(selectedCohort.retention_matrix, 3),
				avgWeek8: calculateAvgRetention(selectedCohort.retention_matrix, 7)
			}
		: null;

	function calculateAvgRetention(
		matrix: Array<{ cohort: string; size: number; periods: number[] }>,
		periodIndex: number
	): number {
		const values = matrix.filter((r) => r.periods[periodIndex] !== undefined).map((r) => r.periods[periodIndex]);
		if (values.length === 0) return 0;
		return values.reduce((sum, v) => sum + v, 0) / values.length;
	}
</script>

<svelte:head>
	<title>Cohort Analysis | Analytics</title>
</svelte:head>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<!-- Header -->
	<div class="flex items-center justify-between mb-8">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">Cohort Analysis</h1>
			<p class="text-sm text-gray-500 mt-1">Analyze user retention patterns over time</p>
		</div>
		<div class="flex items-center gap-4">
			<PeriodSelector value={selectedPeriod} on:change={handlePeriodChange} />
			<select
				bind:value={selectedGranularity}
				on:change={handleGranularityChange}
				class="px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500"
			>
				<option value="daily">Daily</option>
				<option value="weekly">Weekly</option>
				<option value="monthly">Monthly</option>
			</select>
			<button
				on:click={() => (showCreateModal = true)}
				class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm font-medium"
			>
				Create Cohort
			</button>
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
				on:click={loadCohorts}
				class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
			>
				Retry
			</button>
		</div>
	{:else if cohorts.length === 0}
		<div class="bg-white rounded-xl border border-gray-200 p-12 text-center">
			<div class="text-4xl mb-4">👥</div>
			<h3 class="text-lg font-medium text-gray-900 mb-2">No Cohorts Yet</h3>
			<p class="text-gray-500 mb-6">Create your first cohort to analyze user retention</p>
			<button
				on:click={() => (showCreateModal = true)}
				class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 inline-block"
			>
				Create Your First Cohort
			</button>
		</div>
	{:else}
		<!-- Cohort Selector -->
		<div class="flex gap-2 mb-8 overflow-x-auto pb-2">
			{#each cohorts as cohort}
				<button
					on:click={() => (selectedCohort = cohort)}
					class="px-4 py-2 rounded-lg text-sm font-medium whitespace-nowrap transition-all
						{selectedCohort?.key === cohort.key
						? 'bg-blue-600 text-white'
						: 'bg-white border border-gray-200 text-gray-700 hover:bg-gray-50'}"
				>
					{cohort.name}
				</button>
			{/each}
		</div>

		{#if selectedCohort}
			<!-- Retention Summary -->
			{#if overallMetrics}
				<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
					<div class="bg-white rounded-xl border border-gray-200 p-6">
						<div class="text-sm text-gray-500 mb-1">
							{selectedGranularity === 'weekly' ? 'Week 1' : selectedGranularity === 'monthly' ? 'Month 1' : 'Day 1'} Retention
						</div>
						<div class="text-3xl font-bold text-gray-900">
							{overallMetrics.avgWeek1.toFixed(1)}%
						</div>
					</div>
					<div class="bg-white rounded-xl border border-gray-200 p-6">
						<div class="text-sm text-gray-500 mb-1">
							{selectedGranularity === 'weekly' ? 'Week 4' : selectedGranularity === 'monthly' ? 'Month 4' : 'Day 4'} Retention
						</div>
						<div class="text-3xl font-bold text-gray-900">
							{overallMetrics.avgWeek4.toFixed(1)}%
						</div>
					</div>
					<div class="bg-white rounded-xl border border-gray-200 p-6">
						<div class="text-sm text-gray-500 mb-1">
							{selectedGranularity === 'weekly' ? 'Week 8' : selectedGranularity === 'monthly' ? 'Month 8' : 'Day 8'} Retention
						</div>
						<div class="text-3xl font-bold text-gray-900">
							{overallMetrics.avgWeek8.toFixed(1)}%
						</div>
					</div>
				</div>
			{/if}

			<!-- Cohort Matrix -->
			<CohortMatrix
				data={selectedCohort.retention_matrix}
				title={selectedCohort.name}
				periodLabel={selectedGranularity === 'weekly' ? 'Week' : selectedGranularity === 'monthly' ? 'Month' : 'Day'}
			/>

			<!-- Cohort Info -->
			<div class="mt-8 bg-white rounded-xl border border-gray-200 p-6">
				<h3 class="text-lg font-semibold text-gray-900 mb-4">Cohort Details</h3>
				<div class="grid grid-cols-2 md:grid-cols-4 gap-6">
					<div>
						<div class="text-xs font-medium text-gray-400 uppercase mb-1">Type</div>
						<div class="text-sm text-gray-900 capitalize">{selectedCohort.type.replace('_', ' ')}</div>
					</div>
					<div>
						<div class="text-xs font-medium text-gray-400 uppercase mb-1">Granularity</div>
						<div class="text-sm text-gray-900 capitalize">{selectedCohort.granularity}</div>
					</div>
					<div>
						<div class="text-xs font-medium text-gray-400 uppercase mb-1">Total Cohorts</div>
						<div class="text-sm text-gray-900">{selectedCohort.retention_matrix.length}</div>
					</div>
					<div>
						<div class="text-xs font-medium text-gray-400 uppercase mb-1">Total Users</div>
						<div class="text-sm text-gray-900">
							{selectedCohort.retention_matrix.reduce((sum, r) => sum + r.size, 0).toLocaleString()}
						</div>
					</div>
				</div>
				{#if selectedCohort.description}
					<div class="mt-4 pt-4 border-t border-gray-100">
						<div class="text-xs font-medium text-gray-400 uppercase mb-1">Description</div>
						<p class="text-sm text-gray-600">{selectedCohort.description}</p>
					</div>
				{/if}
			</div>
		{/if}
	{/if}
</div>

<!-- Create Cohort Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white rounded-2xl w-full max-w-lg">
			<div class="p-6 border-b border-gray-100">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-gray-900">Create Cohort</h2>
					<button
						on:click={() => (showCreateModal = false)}
						class="text-gray-400 hover:text-gray-600 text-2xl"
					>
						✕
					</button>
				</div>
			</div>

			<div class="p-6 space-y-4">
				<div>
					<label class="block text-sm font-medium text-gray-700 mb-1">Name</label>
					<input
						type="text"
						bind:value={newCohort.name}
						placeholder="e.g., Weekly Signup Retention"
						class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
					/>
				</div>

				<div>
					<label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
					<textarea
						bind:value={newCohort.description}
						placeholder="Describe this cohort..."
						rows={2}
						class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
					></textarea>
				</div>

				<div class="grid grid-cols-2 gap-4">
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-1">Type</label>
						<select
							bind:value={newCohort.type}
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
						>
							<option value="signup">Signup Date</option>
							<option value="first_purchase">First Purchase</option>
							<option value="custom">Custom Event</option>
						</select>
					</div>
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-1">Granularity</label>
						<select
							bind:value={newCohort.granularity}
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
						>
							<option value="daily">Daily</option>
							<option value="weekly">Weekly</option>
							<option value="monthly">Monthly</option>
						</select>
					</div>
				</div>

				{#if newCohort.type === 'custom'}
					<div class="grid grid-cols-2 gap-4">
						<div>
							<label class="block text-sm font-medium text-gray-700 mb-1">Start Event</label>
							<input
								type="text"
								bind:value={newCohort.start_event}
								placeholder="e.g., signup"
								class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
							/>
						</div>
						<div>
							<label class="block text-sm font-medium text-gray-700 mb-1">Return Event</label>
							<input
								type="text"
								bind:value={newCohort.return_event}
								placeholder="e.g., login"
								class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
							/>
						</div>
					</div>
				{/if}
			</div>

			<div class="p-6 border-t border-gray-100 flex justify-end gap-3">
				<button
					on:click={() => (showCreateModal = false)}
					class="px-4 py-2 text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50"
				>
					Cancel
				</button>
				<button
					on:click={createCohort}
					disabled={!newCohort.name}
					class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50"
				>
					Create Cohort
				</button>
			</div>
		</div>
	</div>
{/if}

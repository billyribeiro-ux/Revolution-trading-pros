<script lang="ts">
	/**
	 * Cohort Analysis - User Retention Analysis
	 * Apple ICT7 Grade Implementation
	 *
	 * Analyze user behavior patterns over time with
	 * cohort-based retention matrices.
	 */
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, isAnalyticsConnected } from '$lib/stores/connections';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
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

	// Svelte 5 Runes - State
	let cohorts = $state<Cohort[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('90d');
	let selectedGranularity = $state('weekly');
	let selectedCohort = $state<Cohort | null>(null);
	let showCreateModal = $state(false);

	// New cohort form
	let newCohort = $state({
		name: '',
		description: '',
		type: 'signup' as 'signup' | 'first_purchase' | 'custom',
		granularity: 'weekly' as 'daily' | 'weekly' | 'monthly',
		start_event: '',
		return_event: ''
	});

	async function loadCohorts() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getCohorts({
				period: selectedPeriod,
				granularity: selectedGranularity
			});
			cohorts = (response.cohorts || []) as any;
			if (cohorts.length > 0 && !selectedCohort) {
				selectedCohort = cohorts[0];
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load cohorts';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
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

	// Svelte 5 - $effect replaces onMount
	$effect(() => {
		async function init() {
			await connections.load();
			connectionLoading = false;

			if ($isAnalyticsConnected) {
				await loadCohorts();
			} else {
				loading = false;
			}
		}
		init();
	});

	// Calculate overall retention metrics
	const overallMetrics = $derived(
		selectedCohort
			? {
					avgWeek1: calculateAvgRetention(selectedCohort.retention_matrix, 0),
					avgWeek4: calculateAvgRetention(selectedCohort.retention_matrix, 3),
					avgWeek8: calculateAvgRetention(selectedCohort.retention_matrix, 7)
				}
			: null
	);

	function calculateAvgRetention(
		matrix: Array<{ cohort: string; size: number; periods: number[] }>,
		periodIndex: number
	): number {
		const values = matrix
			.filter((r) => r.periods[periodIndex] !== undefined)
			.map((r) => r.periods[periodIndex]);
		if (values.length === 0) return 0;
		return values.reduce((sum, v) => sum + v, 0) / values.length;
	}
</script>

<svelte:head>
	<title>Cohort Analysis | Analytics</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Apple ICT7 Grade Header -->
		<header class="flex items-center justify-between mb-8">
			<div class="flex items-center gap-4">
				<div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-purple-500 to-pink-600 flex items-center justify-center text-2xl shadow-lg shadow-purple-500/20">
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
					</svg>
				</div>
				<div>
					<h1 class="text-2xl font-bold text-white tracking-tight">Cohort Analysis</h1>
					<p class="text-sm text-slate-400">Analyze user retention patterns over time</p>
				</div>
			</div>
			{#if $isAnalyticsConnected}
				<div class="flex items-center gap-4">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<select
						bind:value={selectedGranularity}
						onchange={handleGranularityChange}
						class="px-4 py-2.5 bg-slate-800/50 border border-white/10 rounded-xl text-sm text-white focus:ring-2 focus:ring-purple-500/50 outline-none"
					>
						<option value="daily">Daily</option>
						<option value="weekly">Weekly</option>
						<option value="monthly">Monthly</option>
					</select>
					<button
						onclick={() => (showCreateModal = true)}
						class="px-5 py-2.5 bg-gradient-to-r from-purple-500 to-pink-600 text-white rounded-xl hover:from-purple-400 hover:to-pink-500 text-sm font-semibold transition-all shadow-lg shadow-purple-500/25 hover:shadow-purple-500/40"
					>
						Create Cohort
					</button>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-purple-500/20 rounded-full"></div>
					<div class="absolute top-0 left-0 w-12 h-12 border-4 border-purple-500 rounded-full animate-spin border-t-transparent"></div>
				</div>
			</div>
		{:else if !$isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else if loading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-10 h-10 border-4 border-purple-500/20 rounded-full"></div>
					<div class="absolute top-0 left-0 w-10 h-10 border-4 border-purple-500 rounded-full animate-spin border-t-transparent"></div>
				</div>
			</div>
		{:else if error}
			<div class="bg-red-500/10 backdrop-blur-xl border border-red-500/20 rounded-2xl p-8 text-center">
				<div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-red-500/10 flex items-center justify-center">
					<svg class="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
				</div>
				<p class="text-red-400 mb-4">{error}</p>
				<button
					onclick={loadCohorts}
					class="px-5 py-2.5 bg-red-500/20 text-red-400 rounded-xl hover:bg-red-500/30 border border-red-500/30 transition-all"
				>
					Retry
				</button>
			</div>
		{:else if cohorts.length === 0}
			<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-12 text-center">
				<div class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-purple-500/10 flex items-center justify-center">
					<svg class="w-8 h-8 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
					</svg>
				</div>
				<h3 class="text-lg font-medium text-white mb-2">No Cohorts Yet</h3>
				<p class="text-slate-400 mb-6">Create your first cohort to analyze user retention</p>
				<button
					onclick={() => (showCreateModal = true)}
					class="px-6 py-3 bg-gradient-to-r from-purple-500 to-pink-600 text-white rounded-xl hover:from-purple-400 hover:to-pink-500 font-semibold shadow-lg shadow-purple-500/25 transition-all"
				>
					Create Your First Cohort
				</button>
			</div>
		{:else}
			<!-- Cohort Selector -->
			<div class="flex gap-2 mb-8 overflow-x-auto pb-2">
				{#each cohorts as cohort}
					<button
						onclick={() => (selectedCohort = cohort)}
						class="px-4 py-2.5 rounded-xl text-sm font-medium whitespace-nowrap transition-all
							{selectedCohort?.key === cohort.key
							? 'bg-gradient-to-r from-purple-500 to-pink-600 text-white shadow-lg shadow-purple-500/25'
							: 'bg-white/5 border border-white/10 text-slate-400 hover:bg-white/10 hover:text-white'}"
					>
						{cohort.name}
					</button>
				{/each}
			</div>

			{#if selectedCohort}
				<!-- Retention Summary -->
				{#if overallMetrics}
					<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
						<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6">
							<div class="text-sm text-slate-400 mb-2">
								{selectedGranularity === 'weekly'
									? 'Week 1'
									: selectedGranularity === 'monthly'
										? 'Month 1'
										: 'Day 1'} Retention
							</div>
							<div class="text-3xl font-bold text-white">
								{overallMetrics.avgWeek1.toFixed(1)}%
							</div>
						</div>
						<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6">
							<div class="text-sm text-slate-400 mb-2">
								{selectedGranularity === 'weekly'
									? 'Week 4'
									: selectedGranularity === 'monthly'
										? 'Month 4'
										: 'Day 4'} Retention
							</div>
							<div class="text-3xl font-bold text-purple-400">
								{overallMetrics.avgWeek4.toFixed(1)}%
							</div>
						</div>
						<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6">
							<div class="text-sm text-slate-400 mb-2">
								{selectedGranularity === 'weekly'
									? 'Week 8'
									: selectedGranularity === 'monthly'
										? 'Month 8'
										: 'Day 8'} Retention
							</div>
							<div class="text-3xl font-bold text-pink-400">
								{overallMetrics.avgWeek8.toFixed(1)}%
							</div>
						</div>
					</div>
				{/if}

				<!-- Cohort Matrix -->
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 overflow-hidden mb-8">
					<CohortMatrix data={selectedCohort.retention_matrix} title={selectedCohort.name} />
				</div>

				<!-- Cohort Info -->
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6">
					<h3 class="text-lg font-semibold text-white mb-4">Cohort Details</h3>
					<div class="grid grid-cols-2 md:grid-cols-4 gap-6">
						<div>
							<div class="text-xs font-medium text-slate-500 uppercase mb-1">Type</div>
							<div class="text-sm text-white capitalize">
								{selectedCohort.type.replace('_', ' ')}
							</div>
						</div>
						<div>
							<div class="text-xs font-medium text-slate-500 uppercase mb-1">Granularity</div>
							<div class="text-sm text-white capitalize">{selectedCohort.granularity}</div>
						</div>
						<div>
							<div class="text-xs font-medium text-slate-500 uppercase mb-1">Total Cohorts</div>
							<div class="text-sm text-white">{selectedCohort.retention_matrix.length}</div>
						</div>
						<div>
							<div class="text-xs font-medium text-slate-500 uppercase mb-1">Total Users</div>
							<div class="text-sm text-white">
								{selectedCohort.retention_matrix.reduce((sum, r) => sum + r.size, 0).toLocaleString()}
							</div>
						</div>
					</div>
					{#if selectedCohort.description}
						<div class="mt-4 pt-4 border-t border-white/10">
							<div class="text-xs font-medium text-slate-500 uppercase mb-1">Description</div>
							<p class="text-sm text-slate-300">{selectedCohort.description}</p>
						</div>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Cohort Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
		<div class="bg-slate-900 rounded-2xl w-full max-w-lg border border-white/10 shadow-2xl">
			<div class="p-6 border-b border-white/10">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-white">Create Cohort</h2>
					<button
						onclick={() => (showCreateModal = false)}
						class="w-8 h-8 rounded-lg bg-white/5 hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition-colors"
						aria-label="Close modal"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
						</svg>
					</button>
				</div>
			</div>

			<div class="p-6 space-y-4">
				<div>
					<label for="cohort-name" class="block text-sm font-medium text-slate-300 mb-2">Name</label>
					<input
						id="cohort-name"
						type="text"
						bind:value={newCohort.name}
						placeholder="e.g., Weekly Signup Retention"
						class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-purple-500/50 outline-none"
					/>
				</div>

				<div>
					<label for="cohort-description" class="block text-sm font-medium text-slate-300 mb-2">Description</label>
					<textarea
						id="cohort-description"
						bind:value={newCohort.description}
						placeholder="Describe this cohort..."
						rows={2}
						class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-purple-500/50 outline-none resize-none"
					></textarea>
				</div>

				<div class="grid grid-cols-2 gap-4">
					<div>
						<label for="cohort-type" class="block text-sm font-medium text-slate-300 mb-2">Type</label>
						<select
							id="cohort-type"
							bind:value={newCohort.type}
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white focus:ring-2 focus:ring-purple-500/50 outline-none"
						>
							<option value="signup">Signup Date</option>
							<option value="first_purchase">First Purchase</option>
							<option value="custom">Custom Event</option>
						</select>
					</div>
					<div>
						<label for="cohort-granularity" class="block text-sm font-medium text-slate-300 mb-2">Granularity</label>
						<select
							id="cohort-granularity"
							bind:value={newCohort.granularity}
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white focus:ring-2 focus:ring-purple-500/50 outline-none"
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
							<label for="cohort-start-event" class="block text-sm font-medium text-slate-300 mb-2">Start Event</label>
							<input
								id="cohort-start-event"
								type="text"
								bind:value={newCohort.start_event}
								placeholder="e.g., signup"
								class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-purple-500/50 outline-none"
							/>
						</div>
						<div>
							<label for="cohort-return-event" class="block text-sm font-medium text-slate-300 mb-2">Return Event</label>
							<input
								id="cohort-return-event"
								type="text"
								bind:value={newCohort.return_event}
								placeholder="e.g., login"
								class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-purple-500/50 outline-none"
							/>
						</div>
					</div>
				{/if}
			</div>

			<div class="p-6 border-t border-white/10 flex justify-end gap-3">
				<button
					onclick={() => (showCreateModal = false)}
					class="px-5 py-2.5 text-slate-300 border border-white/10 rounded-xl hover:bg-white/5 transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={createCohort}
					disabled={!newCohort.name}
					class="px-5 py-2.5 bg-gradient-to-r from-purple-500 to-pink-600 text-white rounded-xl hover:from-purple-400 hover:to-pink-500 font-semibold disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-purple-500/25"
				>
					Create Cohort
				</button>
			</div>
		</div>
	</div>
{/if}

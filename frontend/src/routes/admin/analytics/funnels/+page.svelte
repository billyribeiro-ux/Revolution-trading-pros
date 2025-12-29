<script lang="ts">
	/**
	 * Funnels Dashboard - Conversion Funnel Analysis
	 *
	 * Create, manage, and analyze multi-step conversion funnels
	 * with detailed drop-off analysis.
	 */
	import { onMount } from 'svelte';
	import { analyticsApi } from '$lib/api/analytics';
	import FunnelChart from '$lib/components/analytics/FunnelChart.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';

	interface Funnel {
		key: string;
		name: string;
		description?: string;
		steps: Array<{
			name: string;
			count: number;
			conversion_rate: number;
			drop_off_rate: number;
		}>;
		overall_conversion: number;
		avg_completion_time?: number;
	}

	let funnels: Funnel[] = [];
	let loading = true;
	let error: string | null = null;
	let selectedPeriod = '30d';
	let selectedFunnel: Funnel | null = null;
	let showCreateModal = false;

	// New funnel form
	let newFunnel = {
		name: '',
		description: '',
		steps: [{ name: '', event_name: '' }]
	};

	async function loadFunnels() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getFunnels(selectedPeriod);
			funnels = response.funnels || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load funnels';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		loadFunnels();
	}

	function addStep() {
		newFunnel.steps = [...newFunnel.steps, { name: '', event_name: '' }];
	}

	function removeStep(index: number) {
		if (newFunnel.steps.length > 1) {
			newFunnel.steps = newFunnel.steps.filter((_, i) => i !== index);
		}
	}

	async function createFunnel() {
		try {
			await analyticsApi.createFunnel({
				name: newFunnel.name,
				description: newFunnel.description,
				steps: newFunnel.steps.map((s, i) => ({
					step_number: i + 1,
					name: s.name,
					event_name: s.event_name
				}))
			});
			showCreateModal = false;
			newFunnel = { name: '', description: '', steps: [{ name: '', event_name: '' }] };
			loadFunnels();
		} catch (e) {
			alert(e instanceof Error ? e.message : 'Failed to create funnel');
		}
	}

	function formatTime(seconds?: number): string {
		if (!seconds) return '-';
		if (seconds < 60) return `${seconds.toFixed(0)}s`;
		if (seconds < 3600) return `${(seconds / 60).toFixed(0)}m`;
		return `${(seconds / 3600).toFixed(1)}h`;
	}

	onMount(() => {
		loadFunnels();
	});
</script>

<svelte:head>
	<title>Conversion Funnels | Analytics</title>
</svelte:head>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<!-- Header -->
	<div class="flex items-center justify-between mb-8">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">Conversion Funnels</h1>
			<p class="text-sm text-gray-500 mt-1">Track and optimize user conversion journeys</p>
		</div>
		<div class="flex items-center gap-4">
			<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
			<button
				onclick={() => (showCreateModal = true)}
				class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm font-medium"
			>
				Create Funnel
			</button>
		</div>
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
				onclick={loadFunnels}
				class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
			>
				Retry
			</button>
		</div>
	{:else if funnels.length === 0}
		<div class="bg-white rounded-xl border border-gray-200 p-12 text-center">
			<div class="text-4xl mb-4">🔻</div>
			<h3 class="text-lg font-medium text-gray-900 mb-2">No Funnels Yet</h3>
			<p class="text-gray-500 mb-6">Create your first funnel to track user conversion journeys</p>
			<button
				onclick={() => (showCreateModal = true)}
				class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 inline-block"
			>
				Create Your First Funnel
			</button>
		</div>
	{:else}
		<!-- Funnel Cards -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
			{#each funnels as funnel}
				<button
					onclick={() => (selectedFunnel = funnel)}
					class="text-left bg-white rounded-xl border border-gray-200 p-6 hover:shadow-lg transition-shadow
						{selectedFunnel?.key === funnel.key ? 'ring-2 ring-blue-500' : ''}"
				>
					<div class="flex items-center justify-between mb-4">
						<h3 class="font-semibold text-gray-900">{funnel.name}</h3>
						<span class="text-2xl font-bold text-blue-600">
							{funnel.overall_conversion.toFixed(1)}%
						</span>
					</div>
					{#if funnel.description}
						<p class="text-sm text-gray-500 mb-4">{funnel.description}</p>
					{/if}
					<div class="flex items-center gap-2 text-sm text-gray-500">
						<span>{funnel.steps.length} steps</span>
						{#if funnel.avg_completion_time}
							<span>·</span>
							<span>Avg time: {formatTime(funnel.avg_completion_time)}</span>
						{/if}
					</div>
				</button>
			{/each}
		</div>

		<!-- Selected Funnel Detail -->
		{#if selectedFunnel}
			<div class="space-y-6">
				<FunnelChart steps={selectedFunnel.steps} title={selectedFunnel.name} showDropOff={true} />

				<!-- Step Details Table -->
				<div class="bg-white rounded-xl border border-gray-200 overflow-hidden">
					<div class="p-4 border-b border-gray-100">
						<h3 class="font-semibold text-gray-900">Step Analysis</h3>
					</div>
					<div class="overflow-x-auto">
						<table class="w-full text-sm">
							<thead class="bg-gray-50">
								<tr>
									<th class="text-left py-3 px-4 font-medium text-gray-600">Step</th>
									<th class="text-right py-3 px-4 font-medium text-gray-600">Users</th>
									<th class="text-right py-3 px-4 font-medium text-gray-600">Conversion Rate</th>
									<th class="text-right py-3 px-4 font-medium text-gray-600">Drop-off</th>
									<th class="text-right py-3 px-4 font-medium text-gray-600">From Start</th>
								</tr>
							</thead>
							<tbody class="divide-y divide-gray-100">
								{#each selectedFunnel.steps as step, i}
									{@const firstStep = selectedFunnel.steps[0]}
									{@const fromStart =
										firstStep && firstStep.count > 0
											? (step.count / firstStep.count) * 100
											: 0}
									<tr class="hover:bg-gray-50">
										<td class="py-3 px-4">
											<div class="flex items-center gap-3">
												<span
													class="w-6 h-6 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center text-xs font-medium"
												>
													{i + 1}
												</span>
												<span class="font-medium text-gray-900">{step.name}</span>
											</div>
										</td>
										<td class="py-3 px-4 text-right text-gray-900 font-medium">
											{step.count.toLocaleString()}
										</td>
										<td class="py-3 px-4 text-right">
											<span class="text-green-600 font-medium">
												{step.conversion_rate.toFixed(1)}%
											</span>
										</td>
										<td class="py-3 px-4 text-right">
											{#if i > 0}
												<span class="text-red-500">
													-{step.drop_off_rate.toFixed(1)}%
												</span>
											{:else}
												<span class="text-gray-400">-</span>
											{/if}
										</td>
										<td class="py-3 px-4 text-right">
											<div class="flex items-center justify-end gap-2">
												<div class="w-20 h-2 bg-gray-100 rounded-full overflow-hidden">
													<div
														class="h-full bg-blue-500 rounded-full"
														style="width: {fromStart}%"
													></div>
												</div>
												<span class="text-gray-600 w-12 text-right">{fromStart.toFixed(1)}%</span>
											</div>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		{/if}
	{/if}
</div>

<!-- Create Funnel Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-gray-100">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-gray-900">Create Funnel</h2>
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
						<label for="funnel-name" class="block text-sm font-medium text-gray-700 mb-1"
							>Name</label
						>
						<input
							id="funnel-name"
							type="text"
							bind:value={newFunnel.name}
							placeholder="e.g., Purchase Funnel"
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
						/>
					</div>
					<div>
						<label for="funnel-description" class="block text-sm font-medium text-gray-700 mb-1"
							>Description</label
						>
						<textarea
							id="funnel-description"
							bind:value={newFunnel.description}
							placeholder="Describe this funnel..."
							rows={2}
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
						></textarea>
					</div>
				</div>

				<!-- Steps -->
				<div>
					<div class="flex items-center justify-between mb-3">
						<span class="text-sm font-medium text-gray-700">Funnel Steps</span>
						<button
							onclick={addStep}
							class="text-sm text-blue-600 hover:text-blue-700 font-medium"
						>
							+ Add Step
						</button>
					</div>
					<div class="space-y-3">
						{#each newFunnel.steps as step, index}
							<div class="flex items-center gap-2 p-3 bg-gray-50 rounded-lg">
								<span
									class="w-6 h-6 bg-blue-100 text-blue-600 rounded-full flex items-center justify-center text-xs font-medium flex-shrink-0"
								>
									{index + 1}
								</span>
								<input
									type="text"
									bind:value={step.name}
									placeholder="Step name"
									class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm"
								/>
								<input
									type="text"
									bind:value={step.event_name}
									placeholder="Event name"
									class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm"
								/>
								{#if newFunnel.steps.length > 1}
									<button
										onclick={() => removeStep(index)}
										class="p-2 text-red-500 hover:text-red-700 hover:bg-red-50 rounded-lg"
									>
										🗑️
									</button>
								{/if}
							</div>
						{/each}
					</div>
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
					onclick={createFunnel}
					disabled={!newFunnel.name || newFunnel.steps.some((s) => !s.name || !s.event_name)}
					class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					Create Funnel
				</button>
			</div>
		</div>
	</div>
{/if}

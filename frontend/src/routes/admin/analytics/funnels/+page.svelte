<script lang="ts">
	/**
	 * Funnels Dashboard - Conversion Funnel Analysis
	 * Apple ICT7 Grade Implementation
	 *
	 * Create, manage, and analyze multi-step conversion funnels
	 * with detailed drop-off analysis.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { page } from '$app/state';
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import FunnelChart from '$lib/components/analytics/FunnelChart.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import { toastStore } from '$lib/stores/toast.svelte';

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

	// Svelte 5 Runes - State
	let funnels = $state<Funnel[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('30d');
	let selectedFunnel = $state<Funnel | null>(null);
	let showCreateModal = $state(false);

	// New funnel form
	let newFunnel = $state({
		name: '',
		description: '',
		steps: [{ name: '', event_name: '' }]
	});

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
		// FIX-2026-04-26 (audit 08-analytics §P2-6): reset selection on period
		// change. Previously `selectedFunnel` could point at a stale funnel
		// object that wasn't present in the new period, breaking the
		// `selectedFunnel?.key === funnel.key` test.
		selectedFunnel = null;
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
			toastStore.error(e instanceof Error ? e.message : 'Failed to create funnel');
		}
	}

	function formatTime(seconds?: number): string {
		if (!seconds) return '-';
		if (seconds < 60) return `${seconds.toFixed(0)}s`;
		if (seconds < 3600) return `${(seconds / 60).toFixed(0)}m`;
		return `${(seconds / 3600).toFixed(1)}h`;
	}

	// FIX-2026-04-26 (P1-3): $derived restores reactivity past helper's `untrack`.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): onMount replaces the $effect cascade pattern.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Funnels] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadFunnels();
			} else {
				loading = false;
			}

			// FIX-2026-04-26 (audit 08-analytics §P3-6): the stub create page at
			// `/admin/analytics/funnels/create` redirects here with `?create=1`
			// so the modal opens itself. Same hook covers any other deep link.
			if (page.url.searchParams.get('create') === '1') {
				showCreateModal = true;
			}
		})();
	});
</script>

<svelte:head>
	<title>Conversion Funnels | Analytics</title>
</svelte:head>

<div class="bg-linear-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Apple ICT7 Grade Header -->
		<header class="flex items-center justify-between mb-8">
			<div class="flex items-center gap-4">
				<div
					class="w-12 h-12 rounded-2xl bg-linear-to-br from-violet-500 to-purple-600 flex items-center justify-center text-2xl shadow-lg shadow-violet-500/20"
				>
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chevron-down -->
					<IconChevronDown size={24} aria-hidden="true" />
				</div>
				<div>
					<h1 class="text-2xl font-bold text-white tracking-tight">Conversion Funnels</h1>
					<p class="text-sm text-slate-400">Track and optimize user conversion journeys</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<div class="flex items-center gap-4">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<button
						onclick={() => (showCreateModal = true)}
						class="px-5 py-2.5 bg-linear-to-r from-violet-500 to-purple-600 text-white rounded-xl hover:from-violet-400 hover:to-purple-500 text-sm font-semibold transition-all shadow-lg shadow-violet-500/25 hover:shadow-violet-500/40"
					>
						Create Funnel
					</button>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-violet-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-12 h-12 border-4 border-violet-500 rounded-full animate-spin border-t-transparent"
					></div>
				</div>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else if loading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-10 h-10 border-4 border-violet-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-10 h-10 border-4 border-violet-500 rounded-full animate-spin border-t-transparent"
					></div>
				</div>
			</div>
		{:else if error}
			<div
				class="bg-red-500/10 backdrop-blur-xl border border-red-500/20 rounded-2xl p-8 text-center"
			>
				<div
					class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-red-500/10 flex items-center justify-center"
				>
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: alert-circle error -->
					<IconAlertCircle size={32} aria-hidden="true" />
				</div>
				<p class="text-red-400 mb-4">{error}</p>
				<button
					onclick={loadFunnels}
					class="px-5 py-2.5 bg-red-500/20 text-red-400 rounded-xl hover:bg-red-500/30 border border-red-500/30 transition-all"
				>
					Retry
				</button>
			</div>
		{:else if funnels.length === 0}
			<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-12 text-center">
				<div
					class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-violet-500/10 flex items-center justify-center"
				>
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chevron-down (no funnels empty state) -->
					<IconChevronDown size={32} aria-hidden="true" />
				</div>
				<h3 class="text-lg font-medium text-white mb-2">No Funnels Yet</h3>
				<p class="text-slate-400 mb-6">
					Create your first funnel to track user conversion journeys
				</p>
				<button
					onclick={() => (showCreateModal = true)}
					class="px-6 py-3 bg-linear-to-r from-violet-500 to-purple-600 text-white rounded-xl hover:from-violet-400 hover:to-purple-500 font-semibold shadow-lg shadow-violet-500/25 transition-all"
				>
					Create Your First Funnel
				</button>
			</div>
		{:else}
			<!-- Funnel Cards -->
			<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
				{#each funnels as funnel (funnel.key)}
					<button
						onclick={() => (selectedFunnel = funnel)}
						class="text-left bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6 hover:bg-white/10 hover:border-violet-500/30 transition-all
							{selectedFunnel?.key === funnel.key ? 'ring-2 ring-violet-500 border-violet-500/50' : ''}"
					>
						<div class="flex items-center justify-between mb-4">
							<h3 class="font-semibold text-white">{funnel.name}</h3>
							<span
								class="text-2xl font-bold bg-linear-to-r from-violet-400 to-purple-400 bg-clip-text text-transparent"
							>
								{funnel.overall_conversion.toFixed(1)}%
							</span>
						</div>
						{#if funnel.description}
							<p class="text-sm text-slate-400 mb-4">{funnel.description}</p>
						{/if}
						<div class="flex items-center gap-4 text-sm text-slate-500">
							<span class="flex items-center gap-1.5">
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: list -->
								<IconList size={16} aria-hidden="true" />
								{funnel.steps.length} steps
							</span>
							{#if funnel.avg_completion_time}
								<span class="flex items-center gap-1.5">
									<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: clock -->
									<IconClock size={16} aria-hidden="true" />
									Avg: {formatTime(funnel.avg_completion_time)}
								</span>
							{/if}
						</div>
					</button>
				{/each}
			</div>

			<!-- Selected Funnel Detail -->
			{#if selectedFunnel}
				<div class="space-y-6">
					<FunnelChart
						steps={selectedFunnel.steps}
						title={selectedFunnel.name}
						showDropOff={true}
					/>

					<!-- Step Details Table -->
					<div
						class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 overflow-hidden"
					>
						<div class="p-5 border-b border-white/10">
							<h3 class="font-semibold text-white">Step Analysis</h3>
						</div>
						<div class="overflow-x-auto">
							<table class="w-full text-sm">
								<thead class="bg-slate-800/50">
									<tr>
										<th
											class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
											>Step</th
										>
										<th
											class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
											>Users</th
										>
										<th
											class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
											>Conversion</th
										>
										<th
											class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
											>Drop-off</th
										>
										<th
											class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
											>From Start</th
										>
									</tr>
								</thead>
								<tbody class="divide-y divide-white/5">
									{#each selectedFunnel.steps as step, i (i)}
										{@const firstStep = selectedFunnel.steps[0]}
										{@const fromStart =
											firstStep && firstStep.count > 0 ? (step.count / firstStep.count) * 100 : 0}
										<tr class="hover:bg-white/5 transition-colors">
											<td class="py-4 px-5">
												<div class="flex items-center gap-3">
													<span
														class="w-7 h-7 bg-violet-500/20 text-violet-400 rounded-lg flex items-center justify-center text-xs font-semibold"
													>
														{i + 1}
													</span>
													<span class="font-medium text-white">{step.name}</span>
												</div>
											</td>
											<td class="py-4 px-5 text-right text-white font-medium">
												{step.count.toLocaleString()}
											</td>
											<td class="py-4 px-5 text-right">
												<span class="text-emerald-400 font-medium">
													{step.conversion_rate.toFixed(1)}%
												</span>
											</td>
											<td class="py-4 px-5 text-right">
												{#if i > 0}
													<span class="text-red-400">
														-{step.drop_off_rate.toFixed(1)}%
													</span>
												{:else}
													<span class="text-slate-500">-</span>
												{/if}
											</td>
											<td class="py-4 px-5 text-right">
												<div class="flex items-center justify-end gap-3">
													<div class="w-20 h-2 bg-slate-700 rounded-full overflow-hidden">
														<div
															class="h-full bg-linear-to-r from-violet-500 to-purple-500 rounded-full"
															style="width: {fromStart}%"
														></div>
													</div>
													<span class="text-slate-400 w-14 text-right">{fromStart.toFixed(1)}%</span
													>
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
</div>

<!-- Create Funnel Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
		<div
			class="bg-slate-900 rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto border border-white/10 shadow-2xl"
		>
			<div class="p-6 border-b border-white/10">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-white">Create Funnel</h2>
					<button
						onclick={() => (showCreateModal = false)}
						class="w-8 h-8 rounded-lg bg-white/5 hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition-colors"
						aria-label="Close modal"
					>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (close) -->
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
			</div>

			<div class="p-6 space-y-6">
				<!-- Basic Info -->
				<div class="space-y-4">
					<div>
						<label for="funnel-name" class="block text-sm font-medium text-slate-300 mb-2"
							>Name</label
						>
						<input
							id="funnel-name"
							name="funnel-name"
							type="text"
							bind:value={newFunnel.name}
							placeholder="e.g., Purchase Funnel"
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500/50 outline-none transition-all"
						/>
					</div>
					<div>
						<label for="funnel-description" class="block text-sm font-medium text-slate-300 mb-2"
							>Description</label
						>
						<textarea
							id="funnel-description"
							bind:value={newFunnel.description}
							placeholder="Describe this funnel..."
							rows={2}
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500/50 outline-none transition-all resize-none"
						></textarea>
					</div>
				</div>

				<!-- Steps -->
				<div>
					<div class="flex items-center justify-between mb-4">
						<span class="text-sm font-medium text-slate-300">Funnel Steps</span>
						<button
							onclick={addStep}
							class="text-sm text-violet-400 hover:text-violet-300 font-medium transition-colors"
						>
							+ Add Step
						</button>
					</div>
					<div class="space-y-3">
						{#each newFunnel.steps as step, index (index)}
							<div
								class="flex items-center gap-3 p-4 bg-slate-800/30 rounded-xl border border-white/5"
							>
								<span
									class="w-7 h-7 bg-violet-500/20 text-violet-400 rounded-lg flex items-center justify-center text-xs font-semibold flex-shrink-0"
								>
									{index + 1}
								</span>
								<input
									id="page-step-name"
									name="page-step-name"
									type="text"
									bind:value={step.name}
									placeholder="Step name"
									class="flex-1 px-3 py-2 bg-slate-800/50 border border-white/10 rounded-lg text-sm text-white placeholder-slate-500 focus:ring-2 focus:ring-violet-500/50 outline-none"
								/>
								<input
									id="page-step-event-name"
									name="page-step-event-name"
									type="text"
									bind:value={step.event_name}
									placeholder="Event name"
									class="flex-1 px-3 py-2 bg-slate-800/50 border border-white/10 rounded-lg text-sm text-white placeholder-slate-500 focus:ring-2 focus:ring-violet-500/50 outline-none"
								/>
								{#if newFunnel.steps.length > 1}
									<button
										onclick={() => removeStep(index)}
										class="p-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition-colors"
										aria-label="Remove step"
									>
										<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: trash -->
										<IconTrash size={16} aria-hidden="true" />
									</button>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			</div>

			<div class="p-6 border-t border-white/10 flex justify-end gap-3">
				<button
					onclick={() => (showCreateModal = false)}
					class="px-5 py-2.5 text-slate-300 border border-white/10 rounded-xl hover:bg-white/5 transition-colors"
				>
					Cancel
				</button>
				<button
					onclick={createFunnel}
					disabled={!newFunnel.name || newFunnel.steps.some((s) => !s.name || !s.event_name)}
					class="px-5 py-2.5 bg-linear-to-r from-violet-500 to-purple-600 text-white rounded-xl hover:from-violet-400 hover:to-purple-500 font-semibold disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-violet-500/25"
				>
					Create Funnel
				</button>
			</div>
		</div>
	</div>
{/if}

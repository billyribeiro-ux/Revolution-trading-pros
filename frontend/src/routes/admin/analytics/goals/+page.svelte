<script lang="ts">
	/**
	 * Conversion Goals - Goal Tracking Management
	 * Apple ICT7 Grade Implementation
	 *
	 * Create, manage, and track conversion goals
	 * with real-time progress monitoring.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconDots from '@tabler/icons-svelte-runes/icons/dots-vertical';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { toastStore } from '$lib/stores/toast.svelte';

	interface Goal {
		id: string;
		name: string;
		description?: string;
		type: 'event' | 'pageview' | 'revenue' | 'duration';
		target_event?: string;
		target_url?: string;
		target_value?: number;
		current_value: number;
		completions: number;
		conversion_rate: number;
		status: 'active' | 'paused' | 'completed';
		created_at: string;
	}

	// Svelte 5 Runes - State
	let goals = $state<Goal[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('30d');
	let showCreateModal = $state(false);
	let activeFilter = $state<'all' | 'active' | 'paused' | 'completed'>('all');

	// New goal form
	let newGoal = $state({
		name: '',
		description: '',
		type: 'event' as 'event' | 'pageview' | 'revenue' | 'duration',
		target_event: '',
		target_url: '',
		target_value: 0
	});

	// Goal type options
	const goalTypes = [
		{
			value: 'event',
			label: 'Event Goal',
			icon: 'M13 10V3L4 14h7v7l9-11h-7z',
			description: 'Track specific events'
		},
		{
			value: 'pageview',
			label: 'Pageview Goal',
			icon: 'M15 12a3 3 0 11-6 0 3 3 0 016 0zM2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z',
			description: 'Track page visits'
		},
		{
			value: 'revenue',
			label: 'Revenue Goal',
			icon: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
			description: 'Track revenue targets'
		},
		{
			value: 'duration',
			label: 'Duration Goal',
			icon: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z',
			description: 'Track time on site'
		}
	];

	async function loadGoals() {
		loading = true;
		error = null;
		try {
			const params = new URLSearchParams({ period: selectedPeriod });
			const response = await fetch(`/api/admin/analytics/goals?${params.toString()}`);
			if (!response.ok) {
				// FIX-2026-04-26 (audit 08-analytics §P1-4): surface real upstream
				// status instead of silently zeroing the list.
				throw new Error(`Failed to load goals (HTTP ${response.status})`);
			}
			const data = await response.json();
			goals = data.goals || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load goals';
			goals = [];
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		loadGoals();
	}

	async function createGoal() {
		try {
			// Prepared for API integration
			const response = await fetch('/api/admin/analytics/goals', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(newGoal)
			});
			if (response.ok) {
				showCreateModal = false;
				newGoal = {
					name: '',
					description: '',
					type: 'event',
					target_event: '',
					target_url: '',
					target_value: 0
				};
				loadGoals();
			}
		} catch (e) {
			toastStore.error(e instanceof Error ? e.message : 'Failed to create goal');
		}
	}

	function getProgressColor(rate: number): string {
		if (rate >= 100) return 'from-emerald-500 to-green-500';
		if (rate >= 75) return 'from-blue-500 to-cyan-500';
		if (rate >= 50) return 'from-amber-500 to-yellow-500';
		return 'from-orange-500 to-red-500';
	}

	// FIX-2026-04-26 (P1-3): see analytics/+page.svelte — $derived restores
	// reactivity that the helper's `untrack` would otherwise discard.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): onMount replaces the $effect cascade pattern.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Goals] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadGoals();
			} else {
				loading = false;
			}
		})();
	});

	// Derived filtered goals
	const filteredGoals = $derived(
		goals.filter((goal) => {
			if (activeFilter === 'all') return true;
			return goal.status === activeFilter;
		})
	);

	// Derived stats
	const stats = $derived({
		total: goals.length,
		active: goals.filter((g) => g.status === 'active').length,
		completed: goals.filter((g) => g.status === 'completed').length,
		totalCompletions: goals.reduce((sum, g) => sum + g.completions, 0)
	});
</script>

<svelte:head>
	<title>Conversion Goals | Analytics</title>
</svelte:head>

<div class="bg-linear-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Apple ICT7 Grade Header -->
		<header class="flex items-center justify-between mb-8">
			<div class="flex items-center gap-4">
				<div
					class="w-12 h-12 rounded-2xl bg-linear-to-br from-emerald-500 to-teal-600 flex items-center justify-center text-2xl shadow-lg shadow-emerald-500/20"
				>
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check (goal) -->
					<IconCircleCheck size={24} aria-hidden="true" />
				</div>
				<div>
					<h1 class="text-2xl font-bold text-white tracking-tight">Conversion Goals</h1>
					<p class="text-sm text-slate-400">Track and measure your conversion objectives</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<div class="flex items-center gap-4">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
					<button
						onclick={() => (showCreateModal = true)}
						class="px-5 py-2.5 bg-linear-to-r from-emerald-500 to-teal-600 text-white rounded-xl hover:from-emerald-400 hover:to-teal-500 text-sm font-semibold transition-all shadow-lg shadow-emerald-500/25 hover:shadow-emerald-500/40"
					>
						Create Goal
					</button>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-emerald-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-12 h-12 border-4 border-emerald-500 rounded-full animate-spin border-t-transparent"
					></div>
				</div>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Stats Grid -->
			<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-white mb-1">{stats.total}</div>
					<div class="text-sm text-slate-400">Total Goals</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-emerald-400 mb-1">{stats.active}</div>
					<div class="text-sm text-slate-400">Active Goals</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-blue-400 mb-1">{stats.completed}</div>
					<div class="text-sm text-slate-400">Completed</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-amber-400 mb-1">
						{stats.totalCompletions.toLocaleString()}
					</div>
					<div class="text-sm text-slate-400">Total Completions</div>
				</div>
			</div>

			<!-- Filters -->
			<div class="flex items-center gap-2 mb-6">
				{#each [{ value: 'all', label: 'All Goals' }, { value: 'active', label: 'Active' }, { value: 'paused', label: 'Paused' }, { value: 'completed', label: 'Completed' }] as filter (filter.value)}
					<button
						onclick={() => (activeFilter = filter.value as typeof activeFilter)}
						class="px-4 py-2 rounded-xl text-sm font-medium transition-all
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
					<div class="relative">
						<div class="w-10 h-10 border-4 border-emerald-500/20 rounded-full"></div>
						<div
							class="absolute top-0 left-0 w-10 h-10 border-4 border-emerald-500 rounded-full animate-spin border-t-transparent"
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
						onclick={loadGoals}
						class="px-5 py-2.5 bg-red-500/20 text-red-400 rounded-xl hover:bg-red-500/30 border border-red-500/30 transition-all"
					>
						Retry
					</button>
				</div>
			{:else if filteredGoals.length === 0}
				<div
					class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-12 text-center"
				>
					<div
						class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-emerald-500/10 flex items-center justify-center"
					>
						<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: circle-check (no goals empty state) -->
						<IconCircleCheck size={32} aria-hidden="true" />
					</div>
					<h3 class="text-lg font-medium text-white mb-2">No Goals Configured</h3>
					<p class="text-slate-400 mb-6">Set up conversion goals to track important user actions</p>
					<button
						onclick={() => (showCreateModal = true)}
						class="px-6 py-3 bg-linear-to-r from-emerald-500 to-teal-600 text-white rounded-xl hover:from-emerald-400 hover:to-teal-500 font-semibold shadow-lg shadow-emerald-500/25 transition-all"
					>
						Create Your First Goal
					</button>
				</div>
			{:else}
				<!-- Goals Grid -->
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each filteredGoals as goal (goal.id)}
						<div
							class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6 hover:bg-white/10 transition-all"
						>
							<div class="flex items-start justify-between mb-4">
								<div class="flex items-center gap-3">
									<div
										class="w-10 h-10 rounded-xl bg-emerald-500/20 flex items-center justify-center"
									>
										<svg
											class="w-5 h-5 text-emerald-400"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d={goalTypes.find((t) => t.value === goal.type)?.icon ||
													'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'}
											/>
										</svg>
									</div>
									<div>
										<h3 class="font-semibold text-white">{goal.name}</h3>
										<span
											class="text-xs px-2 py-0.5 rounded-full capitalize
											{goal.status === 'active'
												? 'bg-emerald-500/20 text-emerald-400'
												: goal.status === 'completed'
													? 'bg-blue-500/20 text-blue-400'
													: 'bg-slate-500/20 text-slate-400'}"
										>
											{goal.status}
										</span>
									</div>
								</div>
								<button class="text-slate-400 hover:text-white" aria-label="More options">
									<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: dots-vertical (menu) -->
									<IconDots size={20} aria-hidden="true" />
								</button>
							</div>

							{#if goal.description}
								<p class="text-sm text-slate-400 mb-4 line-clamp-2">{goal.description}</p>
							{/if}

							<!-- Progress Bar -->
							<div class="mb-4">
								<div class="flex items-center justify-between text-sm mb-2">
									<span class="text-slate-400">Progress</span>
									<span class="text-white font-medium">{goal.conversion_rate.toFixed(1)}%</span>
								</div>
								<div class="h-2 bg-slate-700 rounded-full overflow-hidden">
									<div
										class="h-full bg-linear-to-r {getProgressColor(
											goal.conversion_rate
										)} rounded-full transition-all duration-500"
										style="width: {Math.min(100, goal.conversion_rate)}%"
									></div>
								</div>
							</div>

							<div class="flex items-center justify-between text-sm text-slate-400">
								<span>{goal.completions.toLocaleString()} completions</span>
								<span class="capitalize">{goal.type}</span>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Goal Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
		<div
			class="bg-slate-900 rounded-2xl w-full max-w-xl max-h-[90vh] overflow-y-auto border border-white/10 shadow-2xl"
		>
			<div class="p-6 border-b border-white/10">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-white">Create Goal</h2>
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
						<label for="goal-name" class="block text-sm font-medium text-slate-300 mb-2">Name</label
						>
						<input
							id="goal-name"
							name="goal-name"
							type="text"
							bind:value={newGoal.name}
							placeholder="e.g., Newsletter Signup"
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500/50 outline-none transition-all"
						/>
					</div>
					<div>
						<label for="goal-description" class="block text-sm font-medium text-slate-300 mb-2"
							>Description</label
						>
						<textarea
							id="goal-description"
							bind:value={newGoal.description}
							placeholder="Describe this goal..."
							rows={2}
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500/50 outline-none transition-all resize-none"
						></textarea>
					</div>
				</div>

				<!-- Goal Type Selection -->
				<div>
					<span class="block text-sm font-medium text-slate-300 mb-3">Goal Type</span>
					<div class="grid grid-cols-2 gap-3">
						{#each goalTypes as type (type.value)}
							<button
								onclick={() => (newGoal.type = type.value as typeof newGoal.type)}
								class="p-4 rounded-xl border-2 text-left transition-all
									{newGoal.type === type.value
									? 'border-emerald-500 bg-emerald-500/10'
									: 'border-white/10 hover:border-white/20 bg-slate-800/30'}"
							>
								<svg
									class="w-5 h-5 mb-2 {newGoal.type === type.value
										? 'text-emerald-400'
										: 'text-slate-400'}"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d={type.icon}
									/>
								</svg>
								<div class="font-medium text-white text-sm">{type.label}</div>
								<div class="text-xs text-slate-400">{type.description}</div>
							</button>
						{/each}
					</div>
				</div>

				<!-- Conditional Fields -->
				{#if newGoal.type === 'event'}
					<div>
						<label for="target-event" class="block text-sm font-medium text-slate-300 mb-2"
							>Target Event</label
						>
						<input
							id="target-event"
							name="target-event"
							type="text"
							bind:value={newGoal.target_event}
							placeholder="e.g., signup_completed"
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-emerald-500/50 outline-none transition-all"
						/>
					</div>
				{:else if newGoal.type === 'pageview'}
					<div>
						<label for="target-url" class="block text-sm font-medium text-slate-300 mb-2"
							>Target URL</label
						>
						<input
							id="target-url"
							name="target-url"
							type="text"
							bind:value={newGoal.target_url}
							placeholder="e.g., /thank-you"
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-emerald-500/50 outline-none transition-all"
						/>
					</div>
				{:else if newGoal.type === 'revenue' || newGoal.type === 'duration'}
					<div>
						<label for="target-value" class="block text-sm font-medium text-slate-300 mb-2">
							{newGoal.type === 'revenue' ? 'Target Revenue ($)' : 'Target Duration (seconds)'}
						</label>
						<input
							id="target-value"
							name="target-value"
							type="number"
							bind:value={newGoal.target_value}
							placeholder={newGoal.type === 'revenue' ? 'e.g., 10000' : 'e.g., 300'}
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-emerald-500/50 outline-none transition-all"
						/>
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
					onclick={createGoal}
					disabled={!newGoal.name}
					class="px-5 py-2.5 bg-linear-to-r from-emerald-500 to-teal-600 text-white rounded-xl hover:from-emerald-400 hover:to-teal-500 font-semibold disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-emerald-500/25"
				>
					Create Goal
				</button>
			</div>
		</div>
	</div>
{/if}

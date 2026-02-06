<script lang="ts">
	/**
	 * Event Explorer - Advanced Event Analysis Tool
	 * Apple ICT7 Grade Implementation
	 *
	 * Browse, search, and analyze tracked events with
	 * filtering, grouping, and visualization capabilities.
	 */
	import { analyticsApi, type AnalyticsEvent } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';

	// Svelte 5 Runes - State
	let events = $state<AnalyticsEvent[]>([]);
	let eventTypes = $state<{ name: string; count: number }[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('7d');
	let selectedEventType = $state('');
	let searchQuery = $state('');
	let page = $state(1);
	let totalPages = $state(1);
	let totalEvents = $state(0);

	// Event type colors - Apple-inspired palette
	const eventColors: Record<string, string> = {
		page_view: 'bg-blue-500/20 text-blue-400 border-blue-500/30',
		click: 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30',
		form_submit: 'bg-purple-500/20 text-purple-400 border-purple-500/30',
		purchase: 'bg-amber-500/20 text-amber-400 border-amber-500/30',
		signup: 'bg-pink-500/20 text-pink-400 border-pink-500/30',
		login: 'bg-cyan-500/20 text-cyan-400 border-cyan-500/30',
		error: 'bg-red-500/20 text-red-400 border-red-500/30',
		custom: 'bg-slate-500/20 text-slate-400 border-slate-500/30'
	};

	async function loadEvents() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getEvents({
				period: selectedPeriod,
				...(selectedEventType && { event_type: selectedEventType }),
				...(searchQuery && { search: searchQuery }),
				page
			});
			events = response.events;
			eventTypes = response.event_types || [];
			totalPages = response.pagination?.total_pages || 1;
			totalEvents = response.pagination?.total || 0;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load events';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		page = 1;
		loadEvents();
	}

	function handleEventTypeChange() {
		page = 1;
		loadEvents();
	}

	function handleSearch() {
		page = 1;
		loadEvents();
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatProperties(props: Record<string, unknown>): string {
		if (!props || Object.keys(props).length === 0) return '-';
		return JSON.stringify(props).slice(0, 100) + (JSON.stringify(props).length > 100 ? '...' : '');
	}

	// Svelte 5 - $effect replaces onMount for reactive initialization
	$effect(() => {
		async function init() {
			await connections.load();
			connectionLoading = false;

			if (getIsAnalyticsConnected()) {
				await loadEvents();
			} else {
				loading = false;
			}
		}
		init();
	});
</script>

<svelte:head>
	<title>Event Explorer | Analytics</title>
</svelte:head>

<div class="bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Apple ICT7 Grade Header -->
		<header class="mb-8">
			<div class="flex items-center gap-4 mb-2">
				<div
					class="w-12 h-12 rounded-2xl bg-gradient-to-br from-amber-500 to-orange-600 flex items-center justify-center text-2xl shadow-lg shadow-amber-500/20"
				>
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 10V3L4 14h7v7l9-11h-7z"
						/>
					</svg>
				</div>
				<div>
					<h1 class="text-2xl font-bold text-white tracking-tight">Event Explorer</h1>
					<p class="text-sm text-slate-400">Browse and analyze tracked events in real-time</p>
				</div>
			</div>
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-amber-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-12 h-12 border-4 border-amber-500 rounded-full animate-spin border-t-transparent"
					></div>
				</div>
			</div>
		{:else if !getIsAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Filters - Glass morphism card -->
			<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5 mb-6">
				<div class="flex flex-wrap items-center gap-4">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />

					<select
						bind:value={selectedEventType}
						onchange={handleEventTypeChange}
						class="px-4 py-2.5 bg-slate-800/50 border border-white/10 rounded-xl text-sm text-white focus:ring-2 focus:ring-amber-500/50 focus:border-amber-500/50 outline-none transition-all"
					>
						<option value="">All Event Types</option>
						{#each eventTypes as type}
							<option value={type.name}>{type.name} ({type.count})</option>
						{/each}
					</select>

					<div class="flex-1 min-w-[200px]">
						<div class="relative">
							<input
								id="page-searchquery"
								name="page-searchquery"
								type="text"
								bind:value={searchQuery}
								onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
								placeholder="Search events..."
								class="w-full px-4 py-2.5 pl-11 bg-slate-800/50 border border-white/10 rounded-xl text-sm text-white placeholder-slate-500 focus:ring-2 focus:ring-amber-500/50 focus:border-amber-500/50 outline-none transition-all"
							/>
							<svg
								class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-500"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
								/>
							</svg>
						</div>
					</div>

					<button
						onclick={handleSearch}
						class="px-5 py-2.5 bg-gradient-to-r from-amber-500 to-orange-600 text-white rounded-xl hover:from-amber-400 hover:to-orange-500 text-sm font-semibold transition-all shadow-lg shadow-amber-500/25 hover:shadow-amber-500/40"
					>
						Search
					</button>
				</div>
			</div>

			<!-- Event Type Distribution -->
			{#if eventTypes.length > 0}
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6 mb-6">
					<h3 class="text-lg font-semibold text-white mb-4">Event Distribution</h3>
					<div class="flex flex-wrap gap-2">
						{#each eventTypes.slice(0, 12) as type}
							<button
								onclick={() => {
									selectedEventType = type.name;
									handleEventTypeChange();
								}}
								class="px-4 py-2 rounded-xl text-sm font-medium transition-all border
									{selectedEventType === type.name
									? 'bg-gradient-to-r from-amber-500 to-orange-600 text-white border-transparent shadow-lg shadow-amber-500/25'
									: eventColors[type.name] ||
										'bg-slate-700/50 text-slate-300 border-slate-600/50 hover:bg-slate-700'}"
							>
								{type.name}
								<span class="ml-2 opacity-75">{type.count.toLocaleString()}</span>
							</button>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Events Table -->
			<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 overflow-hidden">
				<div class="p-5 border-b border-white/10 flex items-center justify-between">
					<h3 class="font-semibold text-white">
						Events
						<span class="text-sm font-normal text-slate-400 ml-2">
							({totalEvents.toLocaleString()} total)
						</span>
					</h3>
					<button
						class="px-4 py-2 text-sm text-slate-300 hover:text-white border border-white/10 rounded-xl hover:bg-white/5 transition-all"
					>
						Export CSV
					</button>
				</div>

				{#if loading}
					<div class="flex items-center justify-center py-20">
						<div class="relative">
							<div class="w-10 h-10 border-4 border-amber-500/20 rounded-full"></div>
							<div
								class="absolute top-0 left-0 w-10 h-10 border-4 border-amber-500 rounded-full animate-spin border-t-transparent"
							></div>
						</div>
					</div>
				{:else if error}
					<div class="p-8 text-center">
						<div
							class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-red-500/10 flex items-center justify-center"
						>
							<svg
								class="w-8 h-8 text-red-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
								/>
							</svg>
						</div>
						<p class="text-red-400 mb-4">{error}</p>
						<button
							onclick={loadEvents}
							class="px-5 py-2.5 bg-red-500/20 text-red-400 rounded-xl hover:bg-red-500/30 border border-red-500/30 transition-all"
						>
							Retry
						</button>
					</div>
				{:else if events.length === 0}
					<div class="p-12 text-center">
						<div
							class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-amber-500/10 flex items-center justify-center"
						>
							<svg
								class="w-8 h-8 text-amber-400"
								fill="none"
								stroke="currentColor"
								viewBox="0 0 24 24"
							>
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M13 10V3L4 14h7v7l9-11h-7z"
								/>
							</svg>
						</div>
						<h3 class="text-lg font-medium text-white mb-2">No Events Found</h3>
						<p class="text-slate-400">Try adjusting your filters or time range</p>
					</div>
				{:else}
					<div class="overflow-x-auto">
						<table class="w-full text-sm">
							<thead class="bg-slate-800/50">
								<tr>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Event</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>User</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Page</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Properties</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Source</th
									>
									<th
										class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Time</th
									>
								</tr>
							</thead>
							<tbody class="divide-y divide-white/5">
								{#each events as event}
									<tr class="hover:bg-white/5 cursor-pointer transition-colors">
										<td class="py-4 px-5">
											<span
												class="px-3 py-1.5 rounded-lg text-xs font-medium border {eventColors[
													event.event_type
												] || 'bg-slate-700/50 text-slate-300 border-slate-600/50'}"
											>
												{event.event_name}
											</span>
										</td>
										<td class="py-4 px-5">
											{#if event.user_id}
												<span class="text-white">{event.user_id}</span>
											{:else}
												<span class="text-slate-500 italic">Anonymous</span>
											{/if}
										</td>
										<td class="py-4 px-5 max-w-[200px] truncate text-slate-400">
											{event.page_path || '-'}
										</td>
										<td class="py-4 px-5 max-w-[200px] truncate text-slate-500 font-mono text-xs">
											{formatProperties(event.properties || {})}
										</td>
										<td class="py-4 px-5">
											<span class="text-xs text-slate-500 capitalize">
												{event.channel || '-'}
											</span>
										</td>
										<td class="py-4 px-5 text-right text-slate-400 whitespace-nowrap">
											{formatDate(event.created_at)}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>

					<!-- Pagination -->
					{#if totalPages > 1}
						<div class="p-5 border-t border-white/10 flex items-center justify-between">
							<p class="text-sm text-slate-400">
								Page {page} of {totalPages}
							</p>
							<div class="flex items-center gap-2">
								<button
									onclick={() => {
										page = Math.max(1, page - 1);
										loadEvents();
									}}
									disabled={page === 1}
									class="px-4 py-2 text-sm border border-white/10 rounded-xl hover:bg-white/5 text-slate-300 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
								>
									Previous
								</button>
								<button
									onclick={() => {
										page = Math.min(totalPages, page + 1);
										loadEvents();
									}}
									disabled={page === totalPages}
									class="px-4 py-2 text-sm border border-white/10 rounded-xl hover:bg-white/5 text-slate-300 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
								>
									Next
								</button>
							</div>
						</div>
					{/if}
				{/if}
			</div>
		{/if}
	</div>
</div>

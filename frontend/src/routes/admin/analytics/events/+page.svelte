<script lang="ts">
	/**
	 * Event Explorer - Advanced Event Analysis Tool
	 *
	 * Browse, search, and analyze tracked events with
	 * filtering, grouping, and visualization capabilities.
	 */
	import { onMount } from 'svelte';
	import { analyticsApi, type AnalyticsEvent } from '$lib/api/analytics';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	import TimeSeriesChart from '$lib/components/analytics/TimeSeriesChart.svelte';

	let events: AnalyticsEvent[] = [];
	let eventTypes: { name: string; count: number }[] = [];
	let loading = true;
	let error: string | null = null;
	let selectedPeriod = '7d';
	let selectedEventType = '';
	let searchQuery = '';
	let page = 1;
	let totalPages = 1;
	let totalEvents = 0;

	// Event type colors
	const eventColors: Record<string, string> = {
		page_view: 'bg-blue-100 text-blue-700',
		click: 'bg-green-100 text-green-700',
		form_submit: 'bg-purple-100 text-purple-700',
		purchase: 'bg-yellow-100 text-yellow-700',
		signup: 'bg-pink-100 text-pink-700',
		login: 'bg-cyan-100 text-cyan-700',
		error: 'bg-red-100 text-red-700',
		custom: 'bg-gray-100 text-gray-700'
	};

	async function loadEvents() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getEvents({
				period: selectedPeriod,
				event_type: selectedEventType || undefined,
				search: searchQuery || undefined,
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

	onMount(() => {
		loadEvents();
	});
</script>

<svelte:head>
	<title>Event Explorer | Analytics</title>
</svelte:head>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<!-- Header -->
	<div class="mb-8">
		<h1 class="text-2xl font-bold text-gray-900">Event Explorer</h1>
		<p class="text-sm text-gray-500 mt-1">Browse and analyze tracked events</p>
	</div>

	<!-- Filters -->
	<div class="bg-white rounded-xl border border-gray-200 p-4 mb-6">
		<div class="flex flex-wrap items-center gap-4">
			<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />

			<select
				bind:value={selectedEventType}
				onchange={handleEventTypeChange}
				class="px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
			>
				<option value="">All Event Types</option>
				{#each eventTypes as type}
					<option value={type.name}>{type.name} ({type.count})</option>
				{/each}
			</select>

			<div class="flex-1 min-w-[200px]">
				<div class="relative">
					<input
						type="text"
						bind:value={searchQuery}
						onkeydown={(e) => e.key === 'Enter' && handleSearch()}
						placeholder="Search events..."
						class="w-full px-4 py-2 pl-10 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
					/>
					<span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">🔍</span>
				</div>
			</div>

			<button
				onclick={handleSearch}
				class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm font-medium"
			>
				Search
			</button>
		</div>
	</div>

	<!-- Event Type Distribution -->
	{#if eventTypes.length > 0}
		<div class="bg-white rounded-xl border border-gray-200 p-6 mb-6">
			<h3 class="text-lg font-semibold text-gray-900 mb-4">Event Distribution</h3>
			<div class="flex flex-wrap gap-2">
				{#each eventTypes.slice(0, 12) as type}
					<button
						onclick={() => {
							selectedEventType = type.name;
							handleEventTypeChange();
						}}
						class="px-3 py-2 rounded-lg text-sm font-medium transition-all
							{selectedEventType === type.name
							? 'bg-blue-600 text-white'
							: eventColors[type.name] || 'bg-gray-100 text-gray-700 hover:bg-gray-200'}"
					>
						{type.name}
						<span class="ml-2 opacity-75">{type.count.toLocaleString()}</span>
					</button>
				{/each}
			</div>
		</div>
	{/if}

	<!-- Events Table -->
	<div class="bg-white rounded-xl border border-gray-200 overflow-hidden">
		<div class="p-4 border-b border-gray-100 flex items-center justify-between">
			<h3 class="font-semibold text-gray-900">
				Events
				<span class="text-sm font-normal text-gray-500">
					({totalEvents.toLocaleString()} total)
				</span>
			</h3>
			<button
				class="px-3 py-1.5 text-sm text-gray-600 hover:text-gray-900 border border-gray-300 rounded-lg hover:bg-gray-50"
			>
				Export CSV
			</button>
		</div>

		{#if loading}
			<div class="flex items-center justify-center py-20">
				<div
					class="w-8 h-8 border-4 border-blue-600 border-t-transparent rounded-full animate-spin"
				></div>
			</div>
		{:else if error}
			<div class="p-6 text-center text-red-600">
				<p>{error}</p>
				<button
					onclick={loadEvents}
					class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
				>
					Retry
				</button>
			</div>
		{:else if events.length === 0}
			<div class="p-12 text-center">
				<div class="text-4xl mb-4">⚡</div>
				<h3 class="text-lg font-medium text-gray-900 mb-2">No Events Found</h3>
				<p class="text-gray-500">Try adjusting your filters or time range</p>
			</div>
		{:else}
			<div class="overflow-x-auto">
				<table class="w-full text-sm">
					<thead class="bg-gray-50">
						<tr>
							<th class="text-left py-3 px-4 font-medium text-gray-600">Event</th>
							<th class="text-left py-3 px-4 font-medium text-gray-600">User</th>
							<th class="text-left py-3 px-4 font-medium text-gray-600">Page</th>
							<th class="text-left py-3 px-4 font-medium text-gray-600">Properties</th>
							<th class="text-left py-3 px-4 font-medium text-gray-600">Source</th>
							<th class="text-right py-3 px-4 font-medium text-gray-600">Time</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-100">
						{#each events as event}
							<tr class="hover:bg-gray-50 cursor-pointer">
								<td class="py-3 px-4">
									<span
										class="px-2 py-1 rounded text-xs font-medium {eventColors[event.event_type] ||
											'bg-gray-100 text-gray-700'}"
									>
										{event.event_name}
									</span>
								</td>
								<td class="py-3 px-4">
									{#if event.user_id}
										<span class="text-gray-900">{event.user_id}</span>
									{:else}
										<span class="text-gray-400 italic">Anonymous</span>
									{/if}
								</td>
								<td class="py-3 px-4 max-w-[200px] truncate text-gray-600">
									{event.page_path || '-'}
								</td>
								<td class="py-3 px-4 max-w-[200px] truncate text-gray-500 font-mono text-xs">
									{formatProperties(event.properties || {})}
								</td>
								<td class="py-3 px-4">
									<span class="text-xs text-gray-500 capitalize">
										{event.channel || '-'}
									</span>
								</td>
								<td class="py-3 px-4 text-right text-gray-500 whitespace-nowrap">
									{formatDate(event.created_at)}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>

			<!-- Pagination -->
			{#if totalPages > 1}
				<div class="p-4 border-t border-gray-100 flex items-center justify-between">
					<p class="text-sm text-gray-500">
						Page {page} of {totalPages}
					</p>
					<div class="flex items-center gap-2">
						<button
							onclick={() => {
								page = Math.max(1, page - 1);
								loadEvents();
							}}
							disabled={page === 1}
							class="px-3 py-1.5 text-sm border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
						>
							Previous
						</button>
						<button
							onclick={() => {
								page = Math.min(totalPages, page + 1);
								loadEvents();
							}}
							disabled={page === totalPages}
							class="px-3 py-1.5 text-sm border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
						>
							Next
						</button>
					</div>
				</div>
			{/if}
		{/if}
	</div>
</div>

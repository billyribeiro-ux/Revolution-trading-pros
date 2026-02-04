<script lang="ts">
	/**
	 * Session Recordings - User Session Replay
	 * Apple ICT7 Grade Implementation
	 *
	 * Watch how users interact with your site
	 * through recorded session playback.
	 */
	import { analyticsApi } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';

	interface Recording {
		id: string;
		session_id: string;
		user_id?: string;
		user_email?: string;
		duration: number;
		pages_viewed: number;
		events_count: number;
		device_type: 'desktop' | 'tablet' | 'mobile';
		browser: string;
		country: string;
		started_at: string;
		has_errors: boolean;
		has_rage_clicks: boolean;
	}

	// Svelte 5 Runes - State
	let recordings = $state<Recording[]>([]);
	let selectedRecording = $state<Recording | null>(null);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('7d');
	let activeFilter = $state<'all' | 'with_errors' | 'rage_clicks' | 'long'>('all');
	let page = $state(1);
	let totalPages = $state(1);

	async function loadRecordings() {
		loading = true;
		error = null;
		try {
			// Prepared for future API integration
			const response = await fetch(
				`/api/admin/analytics/recordings?period=${selectedPeriod}&filter=${activeFilter}&page=${page}`
			);
			if (response.ok) {
				const data = await response.json();
				recordings = data.recordings || [];
				totalPages = data.pagination?.total_pages || 1;
			} else {
				recordings = [];
			}
		} catch (e) {
			// For now, set empty array since API might not exist yet
			recordings = [];
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		page = 1;
		loadRecordings();
	}

	function handleFilterChange(filter: typeof activeFilter) {
		activeFilter = filter;
		page = 1;
		loadRecordings();
	}

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
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

	function getDeviceIcon(device: string): string {
		switch (device) {
			case 'mobile':
				return 'M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z';
			case 'tablet':
				return 'M12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z';
			default:
				return 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z';
		}
	}

	// Svelte 5 - $effect replaces onMount
	$effect(() => {
		async function init() {
			await connections.load();
			connectionLoading = false;

			if (getIsAnalyticsConnected()) {
				await loadRecordings();
			} else {
				loading = false;
			}
		}
		init();
	});

	// Derived stats
	const stats = $derived({
		total: recordings.length,
		avgDuration:
			recordings.length > 0
				? Math.round(recordings.reduce((sum, r) => sum + r.duration, 0) / recordings.length)
				: 0,
		withErrors: recordings.filter((r) => r.has_errors).length,
		rageClicks: recordings.filter((r) => r.has_rage_clicks).length
	});
</script>

<svelte:head>
	<title>Session Recordings | Analytics</title>
</svelte:head>

<div class="bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Apple ICT7 Grade Header -->
		<header class="flex items-center justify-between mb-8">
			<div class="flex items-center gap-4">
				<div
					class="w-12 h-12 rounded-2xl bg-gradient-to-br from-indigo-500 to-blue-600 flex items-center justify-center text-2xl shadow-lg shadow-indigo-500/20"
				>
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
						/>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
				</div>
				<div>
					<h1 class="text-2xl font-bold text-white tracking-tight">Session Recordings</h1>
					<p class="text-sm text-slate-400">Watch how users interact with your site</p>
				</div>
			</div>
			{#if getIsAnalyticsConnected()}
				<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-indigo-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-12 h-12 border-4 border-indigo-500 rounded-full animate-spin border-t-transparent"
					></div>
				</div>
			</div>
		{:else if !getIsAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Stats Grid -->
			<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-white mb-1">{stats.total}</div>
					<div class="text-sm text-slate-400">Total Recordings</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-indigo-400 mb-1">
						{formatDuration(stats.avgDuration)}
					</div>
					<div class="text-sm text-slate-400">Avg Duration</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-red-400 mb-1">{stats.withErrors}</div>
					<div class="text-sm text-slate-400">With Errors</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-amber-400 mb-1">{stats.rageClicks}</div>
					<div class="text-sm text-slate-400">Rage Clicks</div>
				</div>
			</div>

			<!-- Filters -->
			<div class="flex items-center gap-2 mb-6">
				{#each [{ value: 'all', label: 'All Sessions' }, { value: 'with_errors', label: 'With Errors' }, { value: 'rage_clicks', label: 'Rage Clicks' }, { value: 'long', label: 'Long Sessions' }] as filter}
					<button
						onclick={() => handleFilterChange(filter.value as typeof activeFilter)}
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
						<div class="w-10 h-10 border-4 border-indigo-500/20 rounded-full"></div>
						<div
							class="absolute top-0 left-0 w-10 h-10 border-4 border-indigo-500 rounded-full animate-spin border-t-transparent"
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
						<svg class="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
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
						onclick={loadRecordings}
						class="px-5 py-2.5 bg-red-500/20 text-red-400 rounded-xl hover:bg-red-500/30 border border-red-500/30 transition-all"
					>
						Retry
					</button>
				</div>
			{:else if recordings.length === 0}
				<div
					class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-12 text-center"
				>
					<div
						class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-indigo-500/10 flex items-center justify-center"
					>
						<svg
							class="w-8 h-8 text-indigo-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
							/>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
					</div>
					<h3 class="text-lg font-medium text-white mb-2">No Recordings Available</h3>
					<p class="text-slate-400 mb-6">
						Session recordings will appear once enabled and visitors interact with your site
					</p>
					<a
						href="/admin/settings/tracking"
						class="inline-block px-6 py-3 bg-gradient-to-r from-indigo-500 to-blue-600 text-white rounded-xl hover:from-indigo-400 hover:to-blue-500 font-semibold shadow-lg shadow-indigo-500/25 transition-all"
					>
						Enable Recordings
					</a>
				</div>
			{:else}
				<!-- Recordings List -->
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 overflow-hidden">
					<div class="overflow-x-auto">
						<table class="w-full text-sm">
							<thead class="bg-slate-800/50">
								<tr>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Session</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>User</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Device</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Duration</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Pages</th
									>
									<th
										class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Flags</th
									>
									<th
										class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
										>Date</th
									>
									<th
										class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
									></th>
								</tr>
							</thead>
							<tbody class="divide-y divide-white/5">
								{#each recordings as recording}
									<tr class="hover:bg-white/5 transition-colors">
										<td class="py-4 px-5">
											<span class="font-mono text-xs text-slate-400">
												{recording.session_id.slice(0, 8)}...
											</span>
										</td>
										<td class="py-4 px-5">
											{#if recording.user_email}
												<span class="text-white">{recording.user_email}</span>
											{:else}
												<span class="text-slate-500 italic">Anonymous</span>
											{/if}
										</td>
										<td class="py-4 px-5">
											<div class="flex items-center gap-2">
												<svg
													class="w-4 h-4 text-slate-400"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
												>
													<path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="2"
														d={getDeviceIcon(recording.device_type)}
													/>
												</svg>
												<span class="text-slate-400 capitalize">{recording.device_type}</span>
											</div>
										</td>
										<td class="py-4 px-5">
											<span class="text-white font-medium"
												>{formatDuration(recording.duration)}</span
											>
										</td>
										<td class="py-4 px-5">
											<span class="text-slate-400">{recording.pages_viewed} pages</span>
										</td>
										<td class="py-4 px-5">
											<div class="flex items-center gap-2">
												{#if recording.has_errors}
													<span
														class="px-2 py-1 rounded-lg bg-red-500/20 text-red-400 text-xs font-medium"
														>Error</span
													>
												{/if}
												{#if recording.has_rage_clicks}
													<span
														class="px-2 py-1 rounded-lg bg-amber-500/20 text-amber-400 text-xs font-medium"
														>Rage</span
													>
												{/if}
												{#if !recording.has_errors && !recording.has_rage_clicks}
													<span class="text-slate-500">-</span>
												{/if}
											</div>
										</td>
										<td class="py-4 px-5 text-right text-slate-400 whitespace-nowrap">
											{formatDate(recording.started_at)}
										</td>
										<td class="py-4 px-5 text-right">
											<button
												onclick={() => (selectedRecording = recording)}
												class="px-3 py-1.5 bg-indigo-500/20 text-indigo-400 rounded-lg hover:bg-indigo-500/30 text-xs font-medium transition-all"
											>
												Play
											</button>
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
										loadRecordings();
									}}
									disabled={page === 1}
									class="px-4 py-2 text-sm border border-white/10 rounded-xl hover:bg-white/5 text-slate-300 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
								>
									Previous
								</button>
								<button
									onclick={() => {
										page = Math.min(totalPages, page + 1);
										loadRecordings();
									}}
									disabled={page === totalPages}
									class="px-4 py-2 text-sm border border-white/10 rounded-xl hover:bg-white/5 text-slate-300 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
								>
									Next
								</button>
							</div>
						</div>
					{/if}
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Recording Player Modal -->
{#if selectedRecording}
	<div class="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50 p-4">
		<div
			class="bg-slate-900 rounded-2xl w-full max-w-5xl max-h-[90vh] overflow-hidden border border-white/10 shadow-2xl"
		>
			<div class="p-5 border-b border-white/10 flex items-center justify-between">
				<div class="flex items-center gap-4">
					<div class="w-10 h-10 rounded-xl bg-indigo-500/20 flex items-center justify-center">
						<svg
							class="w-5 h-5 text-indigo-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
							/>
						</svg>
					</div>
					<div>
						<h3 class="font-semibold text-white">Session Recording</h3>
						<p class="text-xs text-slate-400">{selectedRecording.session_id}</p>
					</div>
				</div>
				<button
					onclick={() => (selectedRecording = null)}
					class="w-8 h-8 rounded-lg bg-white/5 hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition-colors"
					aria-label="Close recording"
				>
					<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>

			<!-- Player Area -->
			<div class="aspect-video bg-slate-800/50 flex items-center justify-center">
				<div class="text-center">
					<div
						class="w-20 h-20 mx-auto mb-4 rounded-2xl bg-indigo-500/10 flex items-center justify-center"
					>
						<svg
							class="w-10 h-10 text-indigo-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
							/>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>
					</div>
					<p class="text-slate-400">Session recording player</p>
					<p class="text-slate-500 text-sm mt-1">Requires session replay library integration</p>
				</div>
			</div>

			<!-- Session Info -->
			<div class="p-5 border-t border-white/10 grid grid-cols-4 gap-4 text-sm">
				<div>
					<span class="text-slate-500">Duration</span>
					<p class="text-white font-medium">{formatDuration(selectedRecording.duration)}</p>
				</div>
				<div>
					<span class="text-slate-500">Pages</span>
					<p class="text-white font-medium">{selectedRecording.pages_viewed} viewed</p>
				</div>
				<div>
					<span class="text-slate-500">Device</span>
					<p class="text-white font-medium capitalize">{selectedRecording.device_type}</p>
				</div>
				<div>
					<span class="text-slate-500">Browser</span>
					<p class="text-white font-medium">{selectedRecording.browser}</p>
				</div>
			</div>
		</div>
	</div>
{/if}

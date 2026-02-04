<script lang="ts">
	/**
	 * User Segments - Dynamic Audience Segmentation
	 * Apple ICT7 Grade Implementation
	 *
	 * Create, manage, and analyze user segments
	 * based on behavior, attributes, and rules.
	 */
	import { analyticsApi, type Segment } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// Svelte 5 Runes - State
	let segments = $state<Segment[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedSegment = $state<Segment | null>(null);
	let showCreateModal = $state(false);
	let viewMode = $state<'grid' | 'list'>('grid');

	// Delete confirmation modal state
	let showDeleteModal = $state(false);
	let pendingDeleteKey = $state<string | null>(null);

	// New segment form
	let newSegment = $state({
		name: '',
		description: '',
		type: 'dynamic' as 'static' | 'dynamic' | 'computed',
		rules: [{ field: '', operator: '>', value: '' }]
	});

	// Segment type colors
	const segmentColors: Record<string, string> = {
		dynamic: 'from-blue-500 to-cyan-500',
		static: 'from-purple-500 to-violet-500',
		computed: 'from-amber-500 to-orange-500'
	};

	// Available rule fields
	const ruleFields = [
		{ value: 'total_revenue', label: 'Total Revenue' },
		{ value: 'order_count', label: 'Order Count' },
		{ value: 'last_active_at', label: 'Last Active' },
		{ value: 'days_since_signup', label: 'Days Since Signup' },
		{ value: 'page_views', label: 'Page Views' },
		{ value: 'session_count', label: 'Session Count' },
		{ value: 'country', label: 'Country' },
		{ value: 'device_type', label: 'Device Type' },
		{ value: 'channel', label: 'Acquisition Channel' }
	];

	// Available operators
	const operators = [
		{ value: '=', label: 'equals' },
		{ value: '!=', label: 'not equals' },
		{ value: '>', label: 'greater than' },
		{ value: '>=', label: 'greater or equal' },
		{ value: '<', label: 'less than' },
		{ value: '<=', label: 'less or equal' },
		{ value: 'contains', label: 'contains' },
		{ value: 'in', label: 'is one of' }
	];

	async function loadSegments() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getSegments();
			segments = response.segments || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load segments';
		} finally {
			loading = false;
		}
	}

	function addRule() {
		newSegment.rules = [...newSegment.rules, { field: 'total_revenue', operator: '>', value: '' }];
	}

	function removeRule(index: number) {
		if (newSegment.rules.length > 1) {
			newSegment.rules = newSegment.rules.filter((_, i) => i !== index);
		}
	}

	async function createSegment() {
		try {
			await analyticsApi.createSegment({
				name: newSegment.name,
				description: newSegment.description,
				type: newSegment.type,
				rules: { conditions: newSegment.rules }
			});
			showCreateModal = false;
			newSegment = {
				name: '',
				description: '',
				type: 'dynamic',
				rules: [{ field: '', operator: '>', value: '' }]
			};
			loadSegments();
		} catch (e) {
			alert(e instanceof Error ? e.message : 'Failed to create segment');
		}
	}

	function deleteSegment(key: string) {
		pendingDeleteKey = key;
		showDeleteModal = true;
	}

	async function confirmDeleteSegment() {
		if (!pendingDeleteKey) return;
		showDeleteModal = false;
		const key = pendingDeleteKey;
		pendingDeleteKey = null;

		try {
			await analyticsApi.deleteSegment(key);
			loadSegments();
			if (selectedSegment?.key === key) {
				selectedSegment = null;
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to delete segment';
		}
	}

	// Svelte 5 - $effect replaces onMount
	$effect(() => {
		async function init() {
			await connections.load();
			connectionLoading = false;

			if (getIsAnalyticsConnected()) {
				await loadSegments();
			} else {
				loading = false;
			}
		}
		init();
	});

	// Derived stats
	const stats = $derived({
		total: segments.length,
		dynamic: segments.filter((s) => s.type === 'dynamic').length,
		static: segments.filter((s) => s.type === 'static').length,
		totalUsers: segments.reduce((sum, s) => sum + s.user_count, 0)
	});
</script>

<svelte:head>
	<title>User Segments | Analytics</title>
</svelte:head>

<div class="bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Apple ICT7 Grade Header -->
		<header class="flex items-center justify-between mb-8">
			<div class="flex items-center gap-4">
				<div
					class="w-12 h-12 rounded-2xl bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center text-2xl shadow-lg shadow-cyan-500/20"
				>
					<svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
						/>
					</svg>
				</div>
				<div>
					<h1 class="text-2xl font-bold text-white tracking-tight">User Segments</h1>
					<p class="text-sm text-slate-400">Create and manage audience segments</p>
				</div>
			</div>
			{#if getIsAnalyticsConnected()}
				<div class="flex items-center gap-3">
					<div class="flex items-center bg-slate-800/50 rounded-xl border border-white/10 p-1">
						<button
							onclick={() => (viewMode = 'grid')}
							aria-label="Grid view"
							class="p-2 rounded-lg transition-all {viewMode === 'grid'
								? 'bg-white/10 text-white'
								: 'text-slate-400 hover:text-white'}"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
								/>
							</svg>
						</button>
						<button
							onclick={() => (viewMode = 'list')}
							aria-label="List view"
							class="p-2 rounded-lg transition-all {viewMode === 'list'
								? 'bg-white/10 text-white'
								: 'text-slate-400 hover:text-white'}"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M4 6h16M4 10h16M4 14h16M4 18h16"
								/>
							</svg>
						</button>
					</div>
					<button
						onclick={() => (showCreateModal = true)}
						class="px-5 py-2.5 bg-gradient-to-r from-cyan-500 to-blue-600 text-white rounded-xl hover:from-cyan-400 hover:to-blue-500 text-sm font-semibold transition-all shadow-lg shadow-cyan-500/25 hover:shadow-cyan-500/40"
					>
						Create Segment
					</button>
				</div>
			{/if}
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="flex items-center justify-center py-20">
				<div class="relative">
					<div class="w-12 h-12 border-4 border-cyan-500/20 rounded-full"></div>
					<div
						class="absolute top-0 left-0 w-12 h-12 border-4 border-cyan-500 rounded-full animate-spin border-t-transparent"
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
					<div class="text-sm text-slate-400">Total Segments</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-cyan-400 mb-1">{stats.dynamic}</div>
					<div class="text-sm text-slate-400">Dynamic Segments</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-purple-400 mb-1">{stats.static}</div>
					<div class="text-sm text-slate-400">Static Segments</div>
				</div>
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-5">
					<div class="text-3xl font-bold text-emerald-400 mb-1">
						{stats.totalUsers.toLocaleString()}
					</div>
					<div class="text-sm text-slate-400">Total Users</div>
				</div>
			</div>

			{#if loading}
				<div class="flex items-center justify-center py-20">
					<div class="relative">
						<div class="w-10 h-10 border-4 border-cyan-500/20 rounded-full"></div>
						<div
							class="absolute top-0 left-0 w-10 h-10 border-4 border-cyan-500 rounded-full animate-spin border-t-transparent"
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
						onclick={loadSegments}
						class="px-5 py-2.5 bg-red-500/20 text-red-400 rounded-xl hover:bg-red-500/30 border border-red-500/30 transition-all"
					>
						Retry
					</button>
				</div>
			{:else if segments.length === 0}
				<div
					class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-12 text-center"
				>
					<div
						class="w-16 h-16 mx-auto mb-4 rounded-2xl bg-cyan-500/10 flex items-center justify-center"
					>
						<svg
							class="w-8 h-8 text-cyan-400"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"
							/>
						</svg>
					</div>
					<h3 class="text-lg font-medium text-white mb-2">No Segments Yet</h3>
					<p class="text-slate-400 mb-6">Create your first segment to organize your users</p>
					<button
						onclick={() => (showCreateModal = true)}
						class="px-6 py-3 bg-gradient-to-r from-cyan-500 to-blue-600 text-white rounded-xl hover:from-cyan-400 hover:to-blue-500 font-semibold shadow-lg shadow-cyan-500/25 transition-all"
					>
						Create Your First Segment
					</button>
				</div>
			{:else if viewMode === 'grid'}
				<!-- Grid View -->
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
					{#each segments as segment}
						<div
							role="button"
							tabindex="0"
							onclick={() => (selectedSegment = segment)}
							onkeydown={(e) => e.key === 'Enter' && (selectedSegment = segment)}
							class="text-left bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 p-6 hover:bg-white/10 hover:border-cyan-500/30 transition-all cursor-pointer
								{selectedSegment?.key === segment.key ? 'ring-2 ring-cyan-500 border-cyan-500/50' : ''}"
						>
							<div class="flex items-start justify-between mb-4">
								<div
									class="w-10 h-10 rounded-xl bg-gradient-to-br {segmentColors[segment.type] ||
										'from-gray-500 to-gray-600'} flex items-center justify-center"
								>
									<svg
										class="w-5 h-5 text-white"
										fill="none"
										stroke="currentColor"
										viewBox="0 0 24 24"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"
										/>
									</svg>
								</div>
								{#if segment.is_system}
									<span
										class="px-2 py-1 rounded-lg bg-slate-500/20 text-slate-400 text-xs font-medium"
										>System</span
									>
								{:else}
									<button
										onclick={(e) => {
											e.stopPropagation();
											deleteSegment(segment.key);
										}}
										aria-label="Delete segment"
										class="p-1.5 text-slate-500 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors"
									>
										<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
											/>
										</svg>
									</button>
								{/if}
							</div>
							<h3 class="font-semibold text-white mb-1">{segment.name}</h3>
							{#if segment.description}
								<p class="text-sm text-slate-400 mb-4 line-clamp-2">{segment.description}</p>
							{/if}
							<div class="flex items-center justify-between text-sm">
								<span class="text-cyan-400 font-medium"
									>{segment.user_count.toLocaleString()} users</span
								>
								<span class="text-slate-500 capitalize">{segment.type}</span>
							</div>
							{#if segment.percentage}
								<div class="mt-3 h-1.5 bg-slate-700 rounded-full overflow-hidden">
									<div
										class="h-full bg-gradient-to-r {segmentColors[segment.type] ||
											'from-gray-500 to-gray-600'} rounded-full"
										style="width: {Math.min(100, segment.percentage)}%"
									></div>
								</div>
								<p class="text-xs text-slate-500 mt-1">
									{segment.percentage.toFixed(1)}% of total users
								</p>
							{/if}
						</div>
					{/each}
				</div>
			{:else}
				<!-- List View -->
				<div class="bg-white/5 backdrop-blur-xl rounded-2xl border border-white/10 overflow-hidden">
					<table class="w-full text-sm">
						<thead class="bg-slate-800/50">
							<tr>
								<th
									class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
									>Segment</th
								>
								<th
									class="text-left py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
									>Type</th
								>
								<th
									class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
									>Users</th
								>
								<th
									class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
									>% of Total</th
								>
								<th
									class="text-right py-4 px-5 font-medium text-slate-400 uppercase text-xs tracking-wider"
								></th>
							</tr>
						</thead>
						<tbody class="divide-y divide-white/5">
							{#each segments as segment}
								<tr
									class="hover:bg-white/5 cursor-pointer transition-colors"
									onclick={() => (selectedSegment = segment)}
								>
									<td class="py-4 px-5">
										<div class="flex items-center gap-3">
											<div
												class="w-8 h-8 rounded-lg bg-gradient-to-br {segmentColors[segment.type] ||
													'from-gray-500 to-gray-600'} flex items-center justify-center"
											>
												<svg
													class="w-4 h-4 text-white"
													fill="none"
													stroke="currentColor"
													viewBox="0 0 24 24"
												>
													<path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="2"
														d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7"
													/>
												</svg>
											</div>
											<div>
												<span class="font-medium text-white">{segment.name}</span>
												{#if segment.description}
													<p class="text-xs text-slate-500 truncate max-w-[200px]">
														{segment.description}
													</p>
												{/if}
											</div>
										</div>
									</td>
									<td class="py-4 px-5">
										<span
											class="px-2.5 py-1 rounded-lg text-xs font-medium capitalize
											{segment.type === 'dynamic'
												? 'bg-blue-500/20 text-blue-400'
												: segment.type === 'static'
													? 'bg-purple-500/20 text-purple-400'
													: 'bg-amber-500/20 text-amber-400'}"
										>
											{segment.type}
										</span>
									</td>
									<td class="py-4 px-5 text-right text-white font-medium">
										{segment.user_count.toLocaleString()}
									</td>
									<td class="py-4 px-5 text-right text-slate-400">
										{segment.percentage?.toFixed(1) || '-'}%
									</td>
									<td class="py-4 px-5 text-right">
										{#if !segment.is_system}
											<button
												onclick={(e) => {
													e.stopPropagation();
													deleteSegment(segment.key);
												}}
												aria-label="Delete segment"
												class="p-2 text-slate-500 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors"
											>
												<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path
														stroke-linecap="round"
														stroke-linejoin="round"
														stroke-width="2"
														d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
													/>
												</svg>
											</button>
										{/if}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{/if}
		{/if}
	</div>
</div>

<!-- Create Segment Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
		<div
			class="bg-slate-900 rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto border border-white/10 shadow-2xl"
		>
			<div class="p-6 border-b border-white/10">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-white">Create Segment</h2>
					<button
						onclick={() => (showCreateModal = false)}
						aria-label="Close modal"
						class="w-8 h-8 rounded-lg bg-white/5 hover:bg-white/10 flex items-center justify-center text-slate-400 hover:text-white transition-colors"
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
			</div>

			<div class="p-6 space-y-6">
				<!-- Basic Info -->
				<div class="space-y-4">
					<div>
						<label for="segment-name" class="block text-sm font-medium text-slate-300 mb-2"
							>Name</label
						>
						<input
							id="segment-name" name="segment-name"
							type="text"
							bind:value={newSegment.name}
							placeholder="e.g., High-Value Customers"
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-cyan-500/50 focus:border-cyan-500/50 outline-none transition-all"
						/>
					</div>
					<div>
						<label for="segment-description" class="block text-sm font-medium text-slate-300 mb-2"
							>Description</label
						>
						<textarea
							id="segment-description"
							bind:value={newSegment.description}
							placeholder="Describe this segment..."
							rows={2}
							class="w-full px-4 py-3 bg-slate-800/50 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:ring-2 focus:ring-cyan-500/50 focus:border-cyan-500/50 outline-none transition-all resize-none"
						></textarea>
					</div>
				</div>

				<!-- Segment Type -->
				<div>
					<span class="block text-sm font-medium text-slate-300 mb-3">Segment Type</span>
					<div class="grid grid-cols-3 gap-3">
						{#each [{ value: 'dynamic', label: 'Dynamic', desc: 'Auto-updates based on rules' }, { value: 'static', label: 'Static', desc: 'Manual user list' }, { value: 'computed', label: 'Computed', desc: 'Based on calculations' }] as type}
							<button
								onclick={() => (newSegment.type = type.value as typeof newSegment.type)}
								class="p-4 rounded-xl border-2 text-left transition-all
									{newSegment.type === type.value
									? 'border-cyan-500 bg-cyan-500/10'
									: 'border-white/10 hover:border-white/20 bg-slate-800/30'}"
							>
								<div class="font-medium text-white text-sm mb-1">{type.label}</div>
								<div class="text-xs text-slate-400">{type.desc}</div>
							</button>
						{/each}
					</div>
				</div>

				<!-- Rules -->
				<div>
					<div class="flex items-center justify-between mb-3">
						<span class="text-sm font-medium text-slate-300">Rules</span>
						<button
							onclick={addRule}
							class="text-sm text-cyan-400 hover:text-cyan-300 font-medium transition-colors"
						>
							+ Add Rule
						</button>
					</div>

					<div class="space-y-3">
						{#each newSegment.rules as rule, index}
							<div
								class="flex items-center gap-2 p-4 bg-slate-800/30 rounded-xl border border-white/5"
							>
								<select
									bind:value={rule.field}
									class="flex-1 px-3 py-2.5 bg-slate-800/50 border border-white/10 rounded-lg text-sm text-white focus:ring-2 focus:ring-cyan-500/50 outline-none"
								>
									<option value="">Select field...</option>
									{#each ruleFields as field}
										<option value={field.value}>{field.label}</option>
									{/each}
								</select>

								<select
									bind:value={rule.operator}
									class="px-3 py-2.5 bg-slate-800/50 border border-white/10 rounded-lg text-sm text-white focus:ring-2 focus:ring-cyan-500/50 outline-none"
								>
									{#each operators as op}
										<option value={op.value}>{op.label}</option>
									{/each}
								</select>

								<input
									id="page-rule-value" name="page-rule-value" type="text"
									bind:value={rule.value}
									placeholder="Value"
									class="flex-1 px-3 py-2.5 bg-slate-800/50 border border-white/10 rounded-lg text-sm text-white placeholder-slate-500 focus:ring-2 focus:ring-cyan-500/50 outline-none"
								/>

								{#if newSegment.rules.length > 1}
									<button
										onclick={() => removeRule(index)}
										aria-label="Remove rule"
										class="p-2 text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded-lg transition-colors"
									>
										<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M6 18L18 6M6 6l12 12"
											/>
										</svg>
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
					onclick={createSegment}
					disabled={!newSegment.name}
					class="px-5 py-2.5 bg-gradient-to-r from-cyan-500 to-blue-600 text-white rounded-xl hover:from-cyan-400 hover:to-blue-500 font-semibold disabled:opacity-50 disabled:cursor-not-allowed transition-all shadow-lg shadow-cyan-500/25"
				>
					Create Segment
				</button>
			</div>
		</div>
	</div>
{/if}

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Segment"
	message="Are you sure you want to delete this segment?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteSegment}
	onCancel={() => { showDeleteModal = false; pendingDeleteKey = null; }}
/>

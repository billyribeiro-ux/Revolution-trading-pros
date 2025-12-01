<script lang="ts">
	/**
	 * Segment Builder - User Segmentation Management
	 *
	 * Create, edit, and manage user segments with
	 * rule-based filtering and dynamic updates.
	 */
	import { onMount } from 'svelte';
	import { analyticsApi, type Segment } from '$lib/api/analytics';
	import SegmentList from '$lib/components/analytics/SegmentList.svelte';

	let segments: Segment[] = [];
	let loading = true;
	let error: string | null = null;
	let showCreateModal = false;
	let selectedSegment: Segment | null = null;

	// New segment form
	let newSegment = {
		name: '',
		description: '',
		type: 'dynamic' as 'static' | 'dynamic' | 'computed',
		rules: [] as Array<{
			field: string;
			operator: string;
			value: string;
		}>
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
		{ value: 'not_contains', label: 'does not contain' },
		{ value: 'in', label: 'is one of' },
		{ value: 'not_in', label: 'is not one of' }
	];

	async function loadSegments() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getSegments();
			segments = response.segments;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load segments';
		} finally {
			loading = false;
		}
	}

	function handleSelectSegment(segment: Segment) {
		selectedSegment = segment;
	}

	function addRule() {
		newSegment.rules = [...newSegment.rules, { field: 'total_revenue', operator: '>', value: '' }];
	}

	function removeRule(index: number) {
		newSegment.rules = newSegment.rules.filter((_, i) => i !== index);
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
				rules: []
			};
			loadSegments();
		} catch (e) {
			alert(e instanceof Error ? e.message : 'Failed to create segment');
		}
	}

	function closeModal() {
		showCreateModal = false;
		selectedSegment = null;
	}

	onMount(() => {
		loadSegments();
	});
</script>

<svelte:head>
	<title>User Segments | Analytics</title>
</svelte:head>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
	<!-- Header -->
	<div class="flex items-center justify-between mb-8">
		<div>
			<h1 class="text-2xl font-bold text-gray-900">User Segments</h1>
			<p class="text-sm text-gray-500 mt-1">
				Create and manage user segments for targeted analysis
			</p>
		</div>
		<button
			onclick={() => (showCreateModal = true)}
			class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm font-medium"
		>
			Create Segment
		</button>
	</div>

	<!-- Stats -->
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
		<div class="bg-white rounded-xl border border-gray-200 p-4">
			<div class="text-2xl font-bold text-gray-900">{segments.length}</div>
			<div class="text-sm text-gray-500">Total Segments</div>
		</div>
		<div class="bg-white rounded-xl border border-gray-200 p-4">
			<div class="text-2xl font-bold text-blue-600">
				{segments.filter((s) => s.type === 'dynamic').length}
			</div>
			<div class="text-sm text-gray-500">Dynamic Segments</div>
		</div>
		<div class="bg-white rounded-xl border border-gray-200 p-4">
			<div class="text-2xl font-bold text-purple-600">
				{segments.filter((s) => s.is_system).length}
			</div>
			<div class="text-sm text-gray-500">System Segments</div>
		</div>
		<div class="bg-white rounded-xl border border-gray-200 p-4">
			<div class="text-2xl font-bold text-green-600">
				{segments.reduce((sum, s) => sum + s.user_count, 0).toLocaleString()}
			</div>
			<div class="text-sm text-gray-500">Total Users Segmented</div>
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
				onclick={loadSegments}
				class="mt-4 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
			>
				Retry
			</button>
		</div>
	{:else}
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<!-- Segment List -->
			<div class="lg:col-span-2">
				<SegmentList {segments} onSelect={handleSelectSegment} />
			</div>

			<!-- Segment Details -->
			<div>
				{#if selectedSegment}
					<div class="bg-white rounded-xl border border-gray-200 p-6 sticky top-24">
						<div class="flex items-center justify-between mb-4">
							<h3 class="text-lg font-semibold text-gray-900">{selectedSegment.name}</h3>
							<button
								onclick={() => (selectedSegment = null)}
								class="text-gray-400 hover:text-gray-600"
							>
								✕
							</button>
						</div>

						{#if selectedSegment.description}
							<p class="text-sm text-gray-500 mb-4">{selectedSegment.description}</p>
						{/if}

						<div class="space-y-4">
							<div>
								<div class="text-xs font-medium text-gray-400 uppercase mb-1">Type</div>
								<div class="text-sm text-gray-900 capitalize">{selectedSegment.type}</div>
							</div>

							<div>
								<div class="text-xs font-medium text-gray-400 uppercase mb-1">Users</div>
								<div class="text-2xl font-bold text-gray-900">
									{selectedSegment.user_count.toLocaleString()}
								</div>
								<div class="text-sm text-gray-500">
									{selectedSegment.percentage.toFixed(1)}% of total
								</div>
							</div>

							{#if selectedSegment.rules}
								<div>
									<div class="text-xs font-medium text-gray-400 uppercase mb-2">Rules</div>
									<div class="bg-gray-50 rounded-lg p-3 text-xs font-mono text-gray-600">
										<pre>{JSON.stringify(selectedSegment.rules, null, 2)}</pre>
									</div>
								</div>
							{/if}

							<div class="pt-4 border-t border-gray-100 flex gap-2">
								<button
									class="flex-1 px-3 py-2 text-sm bg-blue-600 text-white rounded-lg hover:bg-blue-700"
								>
									View Users
								</button>
								{#if !selectedSegment.is_system}
									<button
										class="px-3 py-2 text-sm border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50"
									>
										Edit
									</button>
								{/if}
							</div>
						</div>
					</div>
				{:else}
					<div class="bg-gray-50 rounded-xl border border-gray-200 p-6 text-center">
						<div class="text-4xl mb-4">👆</div>
						<p class="text-gray-500">Select a segment to view details</p>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<!-- Create Segment Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
		<div class="bg-white rounded-2xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-gray-100">
				<div class="flex items-center justify-between">
					<h2 class="text-xl font-bold text-gray-900">Create Segment</h2>
					<button onclick={closeModal} class="text-gray-400 hover:text-gray-600 text-2xl">
						✕
					</button>
				</div>
			</div>

			<div class="p-6 space-y-6">
				<!-- Basic Info -->
				<div class="space-y-4">
					<div>
						<label class="block text-sm font-medium text-gray-700 mb-1" for="segment-name"
							>Name</label
						>
						<input
							type="text"
							bind:value={newSegment.name}
							placeholder="e.g., High Value Customers"
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
						/>
					</div>

					<div>
						<label class="block text-sm font-medium text-gray-700 mb-1" for="segment-description"
							>Description</label
						>
						<textarea
							bind:value={newSegment.description}
							placeholder="Describe this segment..."
							rows={2}
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
						></textarea>
					</div>

					<div>
						<label for="segment-type" class="block text-sm font-medium text-gray-700 mb-1"
							>Type</label
						>
						<select
							id="segment-type"
							bind:value={newSegment.type}
							class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
						>
							<option value="dynamic">Dynamic (auto-updates)</option>
							<option value="static">Static (manual)</option>
							<option value="computed">Computed (SQL-based)</option>
						</select>
					</div>
				</div>

				<!-- Rules -->
				<div>
					<div class="flex items-center justify-between mb-3">
						<span class="text-sm font-medium text-gray-700">Rules</span>
						<button
							onclick={addRule}
							class="text-sm text-blue-600 hover:text-blue-700 font-medium"
						>
							+ Add Rule
						</button>
					</div>

					{#if newSegment.rules.length === 0}
						<div class="bg-gray-50 rounded-lg p-6 text-center">
							<p class="text-gray-500 text-sm">No rules defined. Add rules to filter users.</p>
						</div>
					{:else}
						<div class="space-y-3">
							{#each newSegment.rules as rule, index}
								<div class="flex items-center gap-2 p-3 bg-gray-50 rounded-lg">
									<select
										bind:value={rule.field}
										class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm"
									>
										{#each ruleFields as field}
											<option value={field.value}>{field.label}</option>
										{/each}
									</select>

									<select
										bind:value={rule.operator}
										class="px-3 py-2 border border-gray-300 rounded-lg text-sm"
									>
										{#each operators as op}
											<option value={op.value}>{op.label}</option>
										{/each}
									</select>

									<input
										type="text"
										bind:value={rule.value}
										placeholder="Value"
										class="flex-1 px-3 py-2 border border-gray-300 rounded-lg text-sm"
									/>

									<button
										onclick={() => removeRule(index)}
										class="p-2 text-red-500 hover:text-red-700 hover:bg-red-50 rounded-lg"
									>
										🗑️
									</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</div>

			<div class="p-6 border-t border-gray-100 flex justify-end gap-3">
				<button
					onclick={closeModal}
					class="px-4 py-2 text-gray-700 border border-gray-300 rounded-lg hover:bg-gray-50"
				>
					Cancel
				</button>
				<button
					onclick={createSegment}
					disabled={!newSegment.name}
					class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					Create Segment
				</button>
			</div>
		</div>
	</div>
{/if}

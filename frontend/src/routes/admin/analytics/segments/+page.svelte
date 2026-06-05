<script lang="ts">
	/**
	 * User Segments - Dynamic Audience Segmentation
	 * Apple ICT7 Grade Implementation
	 *
	 * Create, manage, and analyze user segments
	 * based on behavior, attributes, and rules.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { analyticsApi, type Segment } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconLayoutGrid from '@tabler/icons-svelte-runes/icons/layout-grid';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import { toastStore } from '$lib/stores/toast.svelte';

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

	const segmentTypeClasses: Record<string, string> = {
		dynamic: 'segment-type--dynamic',
		static: 'segment-type--static',
		computed: 'segment-type--computed'
	};

	function getSegmentTypeClass(type: string) {
		return segmentTypeClasses[type] ?? 'segment-type--fallback';
	}

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
			toastStore.error(e instanceof Error ? e.message : 'Failed to create segment');
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
					console.error('[Segments] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadSegments();
			} else {
				loading = false;
			}
		})();
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

<div class="segments-page">
	<div class="segments-container">
		<header class="segments-header">
			<div class="title-row">
				<div class="title-icon" aria-hidden="true">
					<IconUsers size={24} />
				</div>
				<div>
					<h1>User Segments</h1>
					<p>Create and manage audience segments</p>
				</div>
			</div>
			{#if isAnalyticsConnected}
				<div class="header-actions">
					<div class="view-toggle">
						<button
							type="button"
							onclick={() => (viewMode = 'grid')}
							aria-label="Grid view"
							class={{ 'view-toggle-button': true, active: viewMode === 'grid' }}
						>
							<IconLayoutGrid size={16} aria-hidden="true" />
						</button>
						<button
							type="button"
							onclick={() => (viewMode = 'list')}
							aria-label="List view"
							class={{ 'view-toggle-button': true, active: viewMode === 'list' }}
						>
							<IconList size={16} aria-hidden="true" />
						</button>
					</div>
					<button type="button" onclick={() => (showCreateModal = true)} class="primary-button">
						Create Segment
					</button>
				</div>
			{/if}
		</header>

		{#if connectionLoading}
			<div class="loading-state" aria-label="Loading connection status" aria-live="polite">
				<span class="spinner spinner--large"></span>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<div class="stats-grid">
				<div class="stat-card">
					<div class="stat-value">{stats.total}</div>
					<div class="stat-label">Total Segments</div>
				</div>
				<div class="stat-card">
					<div class="stat-value stat-value--cyan">{stats.dynamic}</div>
					<div class="stat-label">Dynamic Segments</div>
				</div>
				<div class="stat-card">
					<div class="stat-value stat-value--purple">{stats.static}</div>
					<div class="stat-label">Static Segments</div>
				</div>
				<div class="stat-card">
					<div class="stat-value stat-value--emerald">
						{stats.totalUsers.toLocaleString()}
					</div>
					<div class="stat-label">Total Users</div>
				</div>
			</div>

			{#if loading}
				<div class="loading-state" aria-label="Loading segments" aria-live="polite">
					<span class="spinner"></span>
				</div>
			{:else if error}
				<div class="feedback-card feedback-card--error">
					<div class="feedback-icon" aria-hidden="true">
						<IconAlertCircle size={32} />
					</div>
					<p>{error}</p>
					<button type="button" onclick={loadSegments} class="danger-button">Retry</button>
				</div>
			{:else if segments.length === 0}
				<div class="empty-card">
					<div class="empty-icon" aria-hidden="true">
						<IconUsers size={32} />
					</div>
					<h3>No Segments Yet</h3>
					<p>Create your first segment to organize your users</p>
					<button
						type="button"
						onclick={() => (showCreateModal = true)}
						class="primary-button primary-button--large"
					>
						Create Your First Segment
					</button>
				</div>
			{:else if viewMode === 'grid'}
				<div class="segment-grid">
					{#each segments as segment (segment.key)}
						<div
							role="button"
							tabindex="0"
							onclick={() => (selectedSegment = segment)}
							onkeydown={(e) => e.key === 'Enter' && (selectedSegment = segment)}
							class={{
								'segment-card': true,
								selected: selectedSegment?.key === segment.key
							}}
						>
							<div class="segment-card-header">
								<div class={['segment-icon', getSegmentTypeClass(segment.type)]}>
									<IconUsers size={20} aria-hidden="true" />
								</div>
								{#if segment.is_system}
									<span class="system-badge">System</span>
								{:else}
									<button
										type="button"
										aria-label="Delete segment"
										class="icon-action"
										onclick={(e) => {
											e.stopPropagation();
											deleteSegment(segment.key);
										}}
									>
										<IconTrash size={16} aria-hidden="true" />
									</button>
								{/if}
							</div>
							<h3>{segment.name}</h3>
							{#if segment.description}
								<p class="segment-description">{segment.description}</p>
							{/if}
							<div class="segment-meta">
								<span>{segment.user_count.toLocaleString()} users</span>
								<span>{segment.type}</span>
							</div>
							{#if segment.percentage}
								<div class="progress-track">
									<div
										class={['progress-fill', getSegmentTypeClass(segment.type)]}
										style:width={`${Math.min(100, segment.percentage)}%`}
									></div>
								</div>
								<p class="segment-percentage">{segment.percentage.toFixed(1)}% of total users</p>
							{/if}
						</div>
					{/each}
				</div>
			{:else}
				<div class="segments-table-card">
					<table>
						<thead>
							<tr>
								<th>Segment</th>
								<th>Type</th>
								<th class="numeric-cell">Users</th>
								<th class="numeric-cell">% of Total</th>
								<th class="action-cell"><span class="sr-only">Actions</span></th>
							</tr>
						</thead>
						<tbody>
							{#each segments as segment (segment.key)}
								<tr onclick={() => (selectedSegment = segment)}>
									<td>
										<div class="table-segment">
											<div class={['table-segment-icon', getSegmentTypeClass(segment.type)]}>
												<IconUsers size={16} aria-hidden="true" />
											</div>
											<div class="table-segment-copy">
												<span>{segment.name}</span>
												{#if segment.description}
													<p>{segment.description}</p>
												{/if}
											</div>
										</div>
									</td>
									<td>
										<span class={['type-badge', getSegmentTypeClass(segment.type)]}>
											{segment.type}
										</span>
									</td>
									<td class="numeric-cell strong-cell">
										{segment.user_count.toLocaleString()}
									</td>
									<td class="numeric-cell muted-cell">
										{segment.percentage?.toFixed(1) || '-'}%
									</td>
									<td class="action-cell">
										{#if !segment.is_system}
											<button
												type="button"
												onclick={(e) => {
													e.stopPropagation();
													deleteSegment(segment.key);
												}}
												aria-label="Delete segment"
												class="icon-action"
											>
												<IconTrash size={16} aria-hidden="true" />
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
	<div class="modal-shell">
		<div class="modal-panel" role="dialog" aria-modal="true" aria-labelledby="create-segment-title">
			<header class="modal-header">
				<div class="modal-title-row">
					<h2 id="create-segment-title">Create Segment</h2>
					<button
						type="button"
						onclick={() => (showCreateModal = false)}
						aria-label="Close modal"
						class="modal-close-button"
					>
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
			</header>

			<div class="modal-body">
				<div class="form-stack">
					<div class="field-group">
						<label for="segment-name">Name</label>
						<input
							id="segment-name"
							name="segment-name"
							type="text"
							bind:value={newSegment.name}
							placeholder="e.g., High-Value Customers"
							class="form-control"
						/>
					</div>
					<div class="field-group">
						<label for="segment-description">Description</label>
						<textarea
							id="segment-description"
							bind:value={newSegment.description}
							placeholder="Describe this segment..."
							rows={2}
							class="form-control textarea-control"
						></textarea>
					</div>
				</div>

				<div>
					<span class="field-label">Segment Type</span>
					<div class="type-option-grid">
						{#each [{ value: 'dynamic', label: 'Dynamic', desc: 'Auto-updates based on rules' }, { value: 'static', label: 'Static', desc: 'Manual user list' }, { value: 'computed', label: 'Computed', desc: 'Based on calculations' }] as type (type.value)}
							<button
								type="button"
								onclick={() => (newSegment.type = type.value as typeof newSegment.type)}
								class={{ 'type-option': true, selected: newSegment.type === type.value }}
							>
								<span>{type.label}</span>
								<small>{type.desc}</small>
							</button>
						{/each}
					</div>
				</div>

				<div>
					<div class="rules-header">
						<span>Rules</span>
						<button type="button" onclick={addRule} class="link-button"> + Add Rule </button>
					</div>

					<div class="rule-list">
						{#each newSegment.rules as rule, index (index)}
							<div class="rule-row">
								<select
									bind:value={rule.field}
									class="form-control form-control--compact"
									aria-label={`Rule ${index + 1} field`}
								>
									<option value="">Select field...</option>
									{#each ruleFields as field (field.value)}
										<option value={field.value}>{field.label}</option>
									{/each}
								</select>

								<select
									bind:value={rule.operator}
									class="form-control form-control--compact operator-control"
									aria-label={`Rule ${index + 1} operator`}
								>
									{#each operators as op (op.value)}
										<option value={op.value}>{op.label}</option>
									{/each}
								</select>

								<input
									type="text"
									bind:value={rule.value}
									placeholder="Value"
									class="form-control form-control--compact"
									aria-label={`Rule ${index + 1} value`}
								/>

								{#if newSegment.rules.length > 1}
									<button
										type="button"
										onclick={() => removeRule(index)}
										aria-label="Remove rule"
										class="icon-action icon-action--danger"
									>
										<IconX size={16} aria-hidden="true" />
									</button>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			</div>

			<footer class="modal-footer">
				<button type="button" onclick={() => (showCreateModal = false)} class="secondary-button">
					Cancel
				</button>
				<button
					type="button"
					onclick={createSegment}
					disabled={!newSegment.name}
					class="primary-button"
				>
					Create Segment
				</button>
			</footer>
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
	onCancel={() => {
		showDeleteModal = false;
		pendingDeleteKey = null;
	}}
/>

<style>
	.segments-page {
		min-height: 100%;
		background:
			radial-gradient(circle at 12% 0%, rgb(6 182 212 / 12%), transparent 28rem),
			radial-gradient(circle at 82% 8%, rgb(124 58 237 / 10%), transparent 30rem),
			linear-gradient(135deg, #07080d 0%, #101118 52%, #08090f 100%);
		color: #f8fafc;
	}

	.segments-container {
		width: min(100%, 80rem);
		margin-inline: auto;
		padding: 2rem 1rem;
	}

	.segments-header,
	.title-row,
	.header-actions,
	.view-toggle,
	.segment-card-header,
	.segment-meta,
	.table-segment,
	.rules-header,
	.modal-title-row,
	.modal-footer {
		display: flex;
		align-items: center;
	}

	.segments-header {
		justify-content: space-between;
		gap: 1.5rem;
		margin-bottom: 2rem;
	}

	.title-row {
		gap: 1rem;
		min-width: 0;
	}

	.title-icon,
	.feedback-icon,
	.empty-icon,
	.segment-icon,
	.table-segment-icon,
	.modal-close-button,
	.icon-action {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		flex: 0 0 auto;
	}

	.title-icon {
		width: 3rem;
		height: 3rem;
		border-radius: 1rem;
		background: linear-gradient(135deg, #06b6d4, #2563eb);
		box-shadow: 0 18px 40px rgb(6 182 212 / 20%);
	}

	h1,
	h2,
	h3,
	p {
		margin: 0;
	}

	h1 {
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0;
		line-height: 1.2;
	}

	.title-row p,
	.stat-label,
	.empty-card p,
	.segment-description,
	.segment-percentage,
	.table-segment-copy p {
		color: #94a3b8;
	}

	.title-row p,
	.stat-label {
		font-size: 0.875rem;
	}

	.header-actions {
		gap: 0.75rem;
	}

	.view-toggle {
		border: 1px solid rgb(255 255 255 / 10%);
		border-radius: 0.75rem;
		background: rgb(30 32 40 / 70%);
		padding: 0.25rem;
	}

	.view-toggle-button,
	.modal-close-button,
	.icon-action {
		border: 0;
		background: transparent;
		color: #94a3b8;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			border-color 160ms ease,
			color 160ms ease,
			box-shadow 160ms ease;
	}

	.view-toggle-button {
		width: 2rem;
		height: 2rem;
		border-radius: 0.5rem;
	}

	.view-toggle-button:hover,
	.view-toggle-button.active {
		background: rgb(255 255 255 / 10%);
		color: #ffffff;
	}

	.primary-button,
	.secondary-button,
	.danger-button,
	.link-button,
	.type-option {
		font: inherit;
		cursor: pointer;
	}

	.primary-button,
	.secondary-button,
	.danger-button {
		border-radius: 0.75rem;
		padding: 0.625rem 1.25rem;
		font-size: 0.875rem;
		font-weight: 650;
		transition:
			background 160ms ease,
			border-color 160ms ease,
			box-shadow 160ms ease,
			opacity 160ms ease;
	}

	.primary-button {
		border: 0;
		background: linear-gradient(90deg, #06b6d4, #2563eb);
		color: #ffffff;
		box-shadow: 0 18px 42px rgb(6 182 212 / 22%);
	}

	.primary-button:hover:not(:disabled) {
		background: linear-gradient(90deg, #22d3ee, #3b82f6);
		box-shadow: 0 18px 42px rgb(6 182 212 / 35%);
	}

	.primary-button--large {
		padding: 0.75rem 1.5rem;
	}

	.secondary-button {
		border: 1px solid rgb(255 255 255 / 10%);
		background: transparent;
		color: #cbd5e1;
	}

	.secondary-button:hover {
		background: rgb(255 255 255 / 5%);
	}

	.danger-button {
		border: 1px solid rgb(239 68 68 / 30%);
		background: rgb(239 68 68 / 18%);
		color: #f87171;
	}

	.danger-button:hover {
		background: rgb(239 68 68 / 28%);
	}

	.primary-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.loading-state {
		display: flex;
		justify-content: center;
		padding-block: 5rem;
	}

	.spinner {
		width: 2.5rem;
		height: 2.5rem;
		border: 4px solid rgb(6 182 212 / 20%);
		border-top-color: #06b6d4;
		border-radius: 999px;
		animation: spin 800ms linear infinite;
	}

	.spinner--large {
		width: 3rem;
		height: 3rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, minmax(0, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card,
	.feedback-card,
	.empty-card,
	.segment-card,
	.segments-table-card,
	.modal-panel {
		border: 1px solid rgb(255 255 255 / 10%);
		background: rgb(255 255 255 / 5%);
		backdrop-filter: blur(18px);
	}

	.stat-card {
		border-radius: 1rem;
		padding: 1.25rem;
	}

	.stat-value {
		margin-bottom: 0.25rem;
		color: #ffffff;
		font-size: 1.875rem;
		font-weight: 750;
		line-height: 1.1;
	}

	.stat-value--cyan {
		color: #22d3ee;
	}

	.stat-value--purple {
		color: #c084fc;
	}

	.stat-value--emerald {
		color: #34d399;
	}

	.feedback-card,
	.empty-card {
		border-radius: 1rem;
		padding: 3rem 2rem;
		text-align: center;
	}

	.feedback-card--error {
		border-color: rgb(239 68 68 / 20%);
		background: rgb(239 68 68 / 10%);
	}

	.feedback-icon,
	.empty-icon {
		width: 4rem;
		height: 4rem;
		margin: 0 auto 1rem;
		border-radius: 1rem;
	}

	.feedback-icon {
		background: rgb(239 68 68 / 12%);
		color: #f87171;
	}

	.empty-icon {
		background: rgb(6 182 212 / 10%);
		color: #22d3ee;
	}

	.feedback-card p {
		margin-bottom: 1rem;
		color: #f87171;
	}

	.empty-card h3 {
		margin-bottom: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.empty-card p {
		margin-bottom: 1.5rem;
	}

	.segment-grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 1.5rem;
	}

	.segment-card {
		border-radius: 1rem;
		padding: 1.5rem;
		text-align: left;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			border-color 160ms ease,
			box-shadow 160ms ease;
	}

	.segment-card:hover {
		border-color: rgb(6 182 212 / 35%);
		background: rgb(255 255 255 / 10%);
	}

	.segment-card.selected {
		border-color: rgb(6 182 212 / 55%);
		box-shadow: 0 0 0 2px rgb(6 182 212 / 80%);
	}

	.segment-card-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.segment-icon {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.75rem;
	}

	.segment-type--dynamic {
		background: linear-gradient(135deg, #3b82f6, #06b6d4);
		color: #ffffff;
	}

	.segment-type--static {
		background: linear-gradient(135deg, #a855f7, #7c3aed);
		color: #ffffff;
	}

	.segment-type--computed {
		background: linear-gradient(135deg, #f59e0b, #f97316);
		color: #ffffff;
	}

	.segment-type--fallback {
		background: linear-gradient(135deg, #6b7280, #4b5563);
		color: #ffffff;
	}

	.system-badge,
	.type-badge {
		border-radius: 0.5rem;
		font-size: 0.75rem;
		font-weight: 650;
	}

	.system-badge {
		background: rgb(100 116 139 / 20%);
		color: #94a3b8;
		padding: 0.25rem 0.5rem;
	}

	.icon-action {
		width: 2rem;
		height: 2rem;
		border-radius: 0.5rem;
	}

	.icon-action:hover,
	.icon-action--danger:hover {
		background: rgb(239 68 68 / 10%);
		color: #f87171;
	}

	.segment-card h3 {
		margin-bottom: 0.25rem;
		color: #ffffff;
		font-weight: 650;
	}

	.segment-description {
		display: -webkit-box;
		overflow: hidden;
		margin-bottom: 1rem;
		font-size: 0.875rem;
		line-height: 1.5;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
	}

	.segment-meta {
		justify-content: space-between;
		gap: 1rem;
		font-size: 0.875rem;
	}

	.segment-meta span:first-child {
		color: #22d3ee;
		font-weight: 650;
	}

	.segment-meta span:last-child,
	.segment-percentage {
		color: #64748b;
		text-transform: capitalize;
	}

	.progress-track {
		height: 0.375rem;
		overflow: hidden;
		margin-top: 0.75rem;
		border-radius: 999px;
		background: #334155;
	}

	.progress-fill {
		height: 100%;
		border-radius: inherit;
	}

	.segment-percentage {
		margin-top: 0.25rem;
		font-size: 0.75rem;
	}

	.segments-table-card {
		overflow-x: auto;
		border-radius: 1rem;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	thead {
		background: rgb(30 41 59 / 55%);
	}

	th,
	td {
		padding: 1rem 1.25rem;
	}

	th {
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 650;
		letter-spacing: 0.04em;
		text-align: left;
		text-transform: uppercase;
	}

	tbody tr {
		border-top: 1px solid rgb(255 255 255 / 5%);
		cursor: pointer;
		transition: background-color 160ms ease;
	}

	tbody tr:hover {
		background: rgb(255 255 255 / 5%);
	}

	.numeric-cell,
	.action-cell {
		text-align: right;
	}

	.strong-cell {
		color: #ffffff;
		font-weight: 650;
	}

	.muted-cell {
		color: #94a3b8;
	}

	.table-segment {
		gap: 0.75rem;
		min-width: 16rem;
	}

	.table-segment-icon {
		width: 2rem;
		height: 2rem;
		border-radius: 0.5rem;
	}

	.table-segment-copy {
		min-width: 0;
	}

	.table-segment-copy span {
		color: #ffffff;
		font-weight: 650;
	}

	.table-segment-copy p {
		max-width: 12.5rem;
		overflow: hidden;
		margin-top: 0.125rem;
		font-size: 0.75rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.type-badge {
		display: inline-flex;
		padding: 0.25rem 0.625rem;
		text-transform: capitalize;
	}

	.type-badge.segment-type--dynamic {
		background: rgb(59 130 246 / 20%);
		color: #60a5fa;
	}

	.type-badge.segment-type--static {
		background: rgb(168 85 247 / 20%);
		color: #c084fc;
	}

	.type-badge.segment-type--computed {
		background: rgb(245 158 11 / 20%);
		color: #fbbf24;
	}

	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	.modal-shell {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgb(0 0 0 / 60%);
		backdrop-filter: blur(4px);
		padding: 1rem;
	}

	.modal-panel {
		width: min(100%, 42rem);
		max-height: 90vh;
		overflow-y: auto;
		border-radius: 1rem;
		background: #111827;
		box-shadow: 0 28px 80px rgb(0 0 0 / 50%);
	}

	.modal-header,
	.modal-body,
	.modal-footer {
		padding: 1.5rem;
	}

	.modal-header,
	.modal-footer {
		border-color: rgb(255 255 255 / 10%);
	}

	.modal-header {
		border-bottom: 1px solid rgb(255 255 255 / 10%);
	}

	.modal-title-row {
		justify-content: space-between;
		gap: 1rem;
	}

	.modal-title-row h2 {
		font-size: 1.25rem;
		font-weight: 750;
	}

	.modal-close-button {
		width: 2rem;
		height: 2rem;
		border-radius: 0.5rem;
		background: rgb(255 255 255 / 5%);
	}

	.modal-close-button:hover {
		background: rgb(255 255 255 / 10%);
		color: #ffffff;
	}

	.modal-body {
		display: grid;
		gap: 1.5rem;
	}

	.form-stack {
		display: grid;
		gap: 1rem;
	}

	.field-group {
		display: grid;
		gap: 0.5rem;
	}

	label,
	.field-label,
	.rules-header span {
		color: #cbd5e1;
		font-size: 0.875rem;
		font-weight: 650;
	}

	.field-label {
		display: block;
		margin-bottom: 0.75rem;
	}

	.form-control {
		width: 100%;
		border: 1px solid rgb(255 255 255 / 10%);
		border-radius: 0.75rem;
		background: rgb(30 41 59 / 55%);
		padding: 0.75rem 1rem;
		color: #ffffff;
		font: inherit;
		transition:
			border-color 160ms ease,
			box-shadow 160ms ease;
	}

	.form-control::placeholder {
		color: #64748b;
	}

	.form-control:focus {
		outline: none;
		border-color: rgb(6 182 212 / 55%);
		box-shadow: 0 0 0 3px rgb(6 182 212 / 18%);
	}

	.form-control--compact {
		min-height: 2.625rem;
		border-radius: 0.5rem;
		padding: 0.625rem 0.75rem;
		font-size: 0.875rem;
	}

	.textarea-control {
		resize: none;
	}

	.type-option-grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: 0.75rem;
	}

	.type-option {
		border: 2px solid rgb(255 255 255 / 10%);
		border-radius: 0.75rem;
		background: rgb(30 41 59 / 32%);
		padding: 1rem;
		color: #ffffff;
		text-align: left;
		transition:
			background-color 160ms ease,
			border-color 160ms ease;
	}

	.type-option:hover {
		border-color: rgb(255 255 255 / 20%);
	}

	.type-option.selected {
		border-color: #06b6d4;
		background: rgb(6 182 212 / 10%);
	}

	.type-option span,
	.type-option small {
		display: block;
	}

	.type-option span {
		margin-bottom: 0.25rem;
		font-size: 0.875rem;
		font-weight: 650;
	}

	.type-option small {
		color: #94a3b8;
		font-size: 0.75rem;
		line-height: 1.4;
	}

	.rules-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.75rem;
	}

	.link-button {
		border: 0;
		background: transparent;
		color: #22d3ee;
		font-size: 0.875rem;
		font-weight: 650;
	}

	.link-button:hover {
		color: #67e8f9;
	}

	.rule-list {
		display: grid;
		gap: 0.75rem;
	}

	.rule-row {
		display: grid;
		grid-template-columns: minmax(0, 1fr) minmax(8rem, auto) minmax(0, 1fr) auto;
		gap: 0.5rem;
		align-items: center;
		border: 1px solid rgb(255 255 255 / 5%);
		border-radius: 0.75rem;
		background: rgb(30 41 59 / 32%);
		padding: 1rem;
	}

	.operator-control {
		width: auto;
	}

	.modal-footer {
		justify-content: flex-end;
		gap: 0.75rem;
		border-top: 1px solid rgb(255 255 255 / 10%);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 900px) {
		.stats-grid,
		.segment-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}

		.type-option-grid {
			grid-template-columns: 1fr;
		}

		.rule-row {
			grid-template-columns: 1fr;
		}

		.operator-control {
			width: 100%;
		}
	}

	@media (max-width: 680px) {
		.segments-container {
			padding: 1.25rem 0.875rem;
		}

		.segments-header,
		.header-actions {
			align-items: flex-start;
			flex-direction: column;
		}

		.header-actions,
		.primary-button {
			width: 100%;
		}

		.stats-grid,
		.segment-grid {
			grid-template-columns: 1fr;
		}

		.modal-footer {
			align-items: stretch;
			flex-direction: column-reverse;
		}

		.secondary-button,
		.modal-footer .primary-button {
			width: 100%;
		}
	}
</style>

<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { toastStore } from '$lib/stores/toast';
	import {
		IconFilter,
		IconPlus,
		IconUsers,
		IconTag,
		IconEdit,
		IconTrash,
		IconCopy,
		IconX,
		IconCheck,
		IconSearch,
		IconRefresh,
		IconArrowLeft,
		IconMail
	} from '$lib/icons';

	// State
	let loading = true;
	let activeTab: 'segments' | 'tags' | 'saved' = 'segments';

	// Segments
	let segments: Segment[] = [];
	let tags: Tag[] = [];
	let savedFilters: SavedFilter[] = [];

	// Create modal
	let showCreateSegmentModal = false;
	let showCreateTagModal = false;

	// Form state
	let newSegment = {
		name: '',
		description: '',
		conditions: [] as Condition[]
	};
	let newTag = { name: '', color: '#6366f1' };

	interface Segment {
		id: number;
		name: string;
		description: string;
		type: 'smart' | 'static';
		conditions: Condition[];
		memberCount: number;
		lastUpdated: string;
		isSystem: boolean;
	}

	interface Condition {
		field: string;
		operator: string;
		value: string | number;
	}

	interface Tag {
		id: number;
		name: string;
		color: string;
		memberCount: number;
	}

	interface SavedFilter {
		id: number;
		name: string;
		filters: Record<string, string | number>;
		createdAt: string;
		usageCount: number;
	}

	// Condition field options
	const conditionFields = [
		{ value: 'status', label: 'Member Status' },
		{ value: 'subscription_status', label: 'Subscription Status' },
		{ value: 'total_spent', label: 'Total Spent' },
		{ value: 'days_since_signup', label: 'Days Since Signup' },
		{ value: 'days_since_last_order', label: 'Days Since Last Order' },
		{ value: 'email_engagement', label: 'Email Engagement' },
		{ value: 'product', label: 'Has Product' },
		{ value: 'tag', label: 'Has Tag' }
	];

	const conditionOperators: Record<string, { value: string; label: string }[]> = {
		status: [
			{ value: 'eq', label: 'Is' },
			{ value: 'neq', label: 'Is not' }
		],
		subscription_status: [
			{ value: 'eq', label: 'Is' },
			{ value: 'neq', label: 'Is not' }
		],
		total_spent: [
			{ value: 'gt', label: 'Greater than' },
			{ value: 'lt', label: 'Less than' },
			{ value: 'eq', label: 'Equals' },
			{ value: 'gte', label: 'At least' },
			{ value: 'lte', label: 'At most' }
		],
		days_since_signup: [
			{ value: 'gt', label: 'Greater than' },
			{ value: 'lt', label: 'Less than' },
			{ value: 'eq', label: 'Equals' }
		],
		days_since_last_order: [
			{ value: 'gt', label: 'Greater than' },
			{ value: 'lt', label: 'Less than' }
		],
		email_engagement: [
			{ value: 'eq', label: 'Is' }
		],
		product: [
			{ value: 'has', label: 'Has' },
			{ value: 'not_has', label: 'Does not have' }
		],
		tag: [
			{ value: 'has', label: 'Has' },
			{ value: 'not_has', label: 'Does not have' }
		]
	};

	const tagColors = [
		'#6366f1', '#8b5cf6', '#ec4899', '#ef4444', '#f97316',
		'#eab308', '#22c55e', '#14b8a6', '#06b6d4', '#3b82f6'
	];

	onMount(async () => {
		await loadData();
	});

	async function loadData() {
		loading = true;
		await new Promise((r) => setTimeout(r, 500));

		// Mock segments
		segments = [
			{
				id: 1,
				name: 'High Value Customers',
				description: 'Customers with LTV > $500',
				type: 'smart',
				conditions: [{ field: 'total_spent', operator: 'gt', value: 500 }],
				memberCount: 847,
				lastUpdated: new Date(Date.now() - 3600000).toISOString(),
				isSystem: false
			},
			{
				id: 2,
				name: 'At Risk of Churn',
				description: 'No activity in 30+ days',
				type: 'smart',
				conditions: [
					{ field: 'days_since_last_order', operator: 'gt', value: 30 },
					{ field: 'subscription_status', operator: 'eq', value: 'active' }
				],
				memberCount: 620,
				lastUpdated: new Date(Date.now() - 7200000).toISOString(),
				isSystem: false
			},
			{
				id: 3,
				name: 'Active Subscribers',
				description: 'All active paid subscribers',
				type: 'smart',
				conditions: [{ field: 'subscription_status', operator: 'eq', value: 'active' }],
				memberCount: 8420,
				lastUpdated: new Date(Date.now() - 1800000).toISOString(),
				isSystem: true
			},
			{
				id: 4,
				name: 'Trial Users',
				description: 'Members on free trial',
				type: 'smart',
				conditions: [{ field: 'status', operator: 'eq', value: 'trial' }],
				memberCount: 2690,
				lastUpdated: new Date(Date.now() - 3600000).toISOString(),
				isSystem: true
			},
			{
				id: 5,
				name: 'Engaged Email Subscribers',
				description: 'Opened email in last 7 days',
				type: 'smart',
				conditions: [{ field: 'email_engagement', operator: 'eq', value: 'engaged' }],
				memberCount: 3250,
				lastUpdated: new Date(Date.now() - 86400000).toISOString(),
				isSystem: false
			}
		];

		// Mock tags
		tags = [
			{ id: 1, name: 'VIP', color: '#6366f1', memberCount: 156 },
			{ id: 2, name: 'High Value', color: '#22c55e', memberCount: 423 },
			{ id: 3, name: 'At Risk', color: '#ef4444', memberCount: 89 },
			{ id: 4, name: 'New', color: '#3b82f6', memberCount: 312 },
			{ id: 5, name: 'Engaged', color: '#14b8a6', memberCount: 567 },
			{ id: 6, name: 'Support Priority', color: '#f97316', memberCount: 34 },
			{ id: 7, name: 'Enterprise', color: '#8b5cf6', memberCount: 78 },
			{ id: 8, name: 'Referrer', color: '#ec4899', memberCount: 145 }
		];

		// Mock saved filters
		savedFilters = [
			{
				id: 1,
				name: 'High spenders this month',
				filters: { spending_tier: 'whale', date_from: '2024-12-01' },
				createdAt: new Date(Date.now() - 86400000 * 5).toISOString(),
				usageCount: 12
			},
			{
				id: 2,
				name: 'New signups last week',
				filters: { status: 'active', date_from: '2024-12-18' },
				createdAt: new Date(Date.now() - 86400000 * 2).toISOString(),
				usageCount: 8
			},
			{
				id: 3,
				name: 'Churned with high LTV',
				filters: { status: 'churned', spending_tier: 'high' },
				createdAt: new Date(Date.now() - 86400000 * 10).toISOString(),
				usageCount: 23
			}
		];

		loading = false;
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function addCondition() {
		newSegment.conditions = [
			...newSegment.conditions,
			{ field: 'status', operator: 'eq', value: '' }
		];
	}

	function removeCondition(index: number) {
		newSegment.conditions = newSegment.conditions.filter((_, i) => i !== index);
	}

	function handleCreateSegment() {
		if (!newSegment.name) {
			toastStore.error('Please enter a segment name');
			return;
		}
		if (newSegment.conditions.length === 0) {
			toastStore.error('Please add at least one condition');
			return;
		}

		const segment: Segment = {
			id: segments.length + 1,
			name: newSegment.name,
			description: newSegment.description,
			type: 'smart',
			conditions: newSegment.conditions,
			memberCount: Math.floor(Math.random() * 1000) + 100,
			lastUpdated: new Date().toISOString(),
			isSystem: false
		};

		segments = [...segments, segment];
		showCreateSegmentModal = false;
		newSegment = { name: '', description: '', conditions: [] };
		toastStore.success('Segment created successfully');
	}

	function handleCreateTag() {
		if (!newTag.name) {
			toastStore.error('Please enter a tag name');
			return;
		}

		const tag: Tag = {
			id: tags.length + 1,
			name: newTag.name,
			color: newTag.color,
			memberCount: 0
		};

		tags = [...tags, tag];
		showCreateTagModal = false;
		newTag = { name: '', color: '#6366f1' };
		toastStore.success('Tag created successfully');
	}

	function deleteSegment(id: number) {
		const segment = segments.find(s => s.id === id);
		if (segment?.isSystem) {
			toastStore.error('Cannot delete system segments');
			return;
		}
		if (!confirm('Delete this segment?')) return;
		segments = segments.filter((s) => s.id !== id);
		toastStore.success('Segment deleted');
	}

	function deleteTag(id: number) {
		if (!confirm('Delete this tag? It will be removed from all members.')) return;
		tags = tags.filter((t) => t.id !== id);
		toastStore.success('Tag deleted');
	}

	function deleteSavedFilter(id: number) {
		if (!confirm('Delete this saved filter?')) return;
		savedFilters = savedFilters.filter((f) => f.id !== id);
		toastStore.success('Filter deleted');
	}

	function applySavedFilter(filter: SavedFilter) {
		// Navigate to members page with filter applied
		const params = new URLSearchParams();
		Object.entries(filter.filters).forEach(([key, value]) => {
			params.set(key, String(value));
		});
		goto(`/admin/members?${params.toString()}`);
	}

	function viewSegmentMembers(segment: Segment) {
		// Navigate to members page filtered by segment
		goto(`/admin/members?segment=${segment.id}`);
	}
</script>

<svelte:head>
	<title>Member Segments | Revolution Trading Pros</title>
</svelte:head>

<div class="segments-page">
	<!-- Header -->
	<div class="page-header">
		<button class="back-btn" onclick={() => goto('/admin/members')}>
			<IconArrowLeft size={20} />
			Back to Members
		</button>

		<div class="header-content">
			<div class="header-title">
				<div class="title-icon">
					<IconFilter size={28} />
				</div>
				<div>
					<h1>Member Segments</h1>
					<p class="subtitle">Organize members with segments, tags, and filters</p>
				</div>
			</div>

			<div class="header-actions">
				<button class="btn-secondary" onclick={loadData}>
					<IconRefresh size={18} />
					Refresh
				</button>
			</div>
		</div>
	</div>

	<!-- Tabs -->
	<div class="tabs-container">
		<div class="tabs">
			<button class:active={activeTab === 'segments'} onclick={() => (activeTab = 'segments')}>
				<IconFilter size={18} />
				Smart Segments ({segments.length})
			</button>
			<button class:active={activeTab === 'tags'} onclick={() => (activeTab = 'tags')}>
				<IconTag size={18} />
				Tags ({tags.length})
			</button>
			<button class:active={activeTab === 'saved'} onclick={() => (activeTab = 'saved')}>
				<IconSearch size={18} />
				Saved Filters ({savedFilters.length})
			</button>
		</div>

		<div class="tab-actions">
			{#if activeTab === 'segments'}
				<button class="btn-primary" onclick={() => (showCreateSegmentModal = true)}>
					<IconPlus size={18} />
					New Segment
				</button>
			{:else if activeTab === 'tags'}
				<button class="btn-primary" onclick={() => (showCreateTagModal = true)}>
					<IconPlus size={18} />
					New Tag
				</button>
			{/if}
		</div>
	</div>

	{#if loading}
		<div class="loading-grid">
			{#each [1, 2, 3, 4] as _}
				<div class="skeleton skeleton-card"></div>
			{/each}
		</div>
	{:else}
		<!-- Segments Tab -->
		{#if activeTab === 'segments'}
			<div class="segments-grid">
				{#each segments as segment}
					<div class="segment-card" class:system={segment.isSystem}>
						<div class="segment-header">
							<div class="segment-info">
								<h3>{segment.name}</h3>
								{#if segment.isSystem}
									<span class="system-badge">System</span>
								{/if}
							</div>
							<div class="segment-count">
								<IconUsers size={16} />
								{formatNumber(segment.memberCount)}
							</div>
						</div>

						<p class="segment-description">{segment.description}</p>

						<div class="segment-conditions">
							{#each segment.conditions as condition}
								<span class="condition-tag">
									{conditionFields.find((f) => f.value === condition.field)?.label || condition.field}
									{conditionOperators[condition.field]?.find((o) => o.value === condition.operator)?.label || condition.operator}
									{condition.value}
								</span>
							{/each}
						</div>

						<div class="segment-footer">
							<span class="updated">Updated {formatDate(segment.lastUpdated)}</span>
							<div class="segment-actions">
								<button class="btn-icon" onclick={() => viewSegmentMembers(segment)} title="View members">
									<IconUsers size={16} />
								</button>
								<button class="btn-icon" title="Send campaign">
									<IconMail size={16} />
								</button>
								{#if !segment.isSystem}
									<button class="btn-icon" title="Edit">
										<IconEdit size={16} />
									</button>
									<button class="btn-icon danger" onclick={() => deleteSegment(segment.id)} title="Delete">
										<IconTrash size={16} />
									</button>
								{/if}
							</div>
						</div>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Tags Tab -->
		{#if activeTab === 'tags'}
			<div class="tags-grid">
				{#each tags as tag}
					<div class="tag-card">
						<div class="tag-header">
							<div class="tag-name" style="--tag-color: {tag.color}">
								<span class="tag-dot"></span>
								{tag.name}
							</div>
							<div class="tag-count">
								{formatNumber(tag.memberCount)} members
							</div>
						</div>

						<div class="tag-actions">
							<button class="btn-secondary small" onclick={() => goto(`/admin/members?tag=${tag.id}`)}>
								<IconUsers size={14} />
								View Members
							</button>
							<button class="btn-icon" title="Edit">
								<IconEdit size={16} />
							</button>
							<button class="btn-icon danger" onclick={() => deleteTag(tag.id)} title="Delete">
								<IconTrash size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Saved Filters Tab -->
		{#if activeTab === 'saved'}
			<div class="filters-list">
				{#each savedFilters as filter}
					<div class="filter-card">
						<div class="filter-info">
							<h3>{filter.name}</h3>
							<div class="filter-meta">
								<span>Created {formatDate(filter.createdAt)}</span>
								<span>Used {filter.usageCount} times</span>
							</div>
						</div>

						<div class="filter-pills">
							{#each Object.entries(filter.filters) as [key, value]}
								<span class="filter-pill">
									{key}: {value}
								</span>
							{/each}
						</div>

						<div class="filter-actions">
							<button class="btn-primary small" onclick={() => applySavedFilter(filter)}>
								Apply Filter
							</button>
							<button class="btn-icon" title="Duplicate">
								<IconCopy size={16} />
							</button>
							<button class="btn-icon danger" onclick={() => deleteSavedFilter(filter.id)} title="Delete">
								<IconTrash size={16} />
							</button>
						</div>
					</div>
				{/each}

				{#if savedFilters.length === 0}
					<div class="empty-state">
						<IconSearch size={48} stroke={1} />
						<h3>No saved filters</h3>
						<p>Save filters from the Members page to quickly apply them later</p>
					</div>
				{/if}
			</div>
		{/if}
	{/if}
</div>

<!-- Create Segment Modal -->
{#if showCreateSegmentModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={() => (showCreateSegmentModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateSegmentModal = false)}
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal-content large"
			role="document"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Create Smart Segment</h2>
				<button class="close-btn" onclick={() => (showCreateSegmentModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="form-group">
					<label for="segment-name">Segment Name</label>
					<input
						id="segment-name"
						type="text"
						bind:value={newSegment.name}
						placeholder="e.g., High Value Customers"
					/>
				</div>

				<div class="form-group">
					<label for="segment-description">Description</label>
					<input
						id="segment-description"
						type="text"
						bind:value={newSegment.description}
						placeholder="Brief description of this segment"
					/>
				</div>

				<div class="conditions-section">
					<div class="conditions-header">
						<span class="conditions-label">Conditions</span>
						<span class="match-label">Members must match ALL conditions</span>
					</div>

					{#each newSegment.conditions as condition, index}
						<div class="condition-row">
							<select bind:value={condition.field}>
								{#each conditionFields as field}
									<option value={field.value}>{field.label}</option>
								{/each}
							</select>

							<select bind:value={condition.operator}>
								{#each conditionOperators[condition.field] || [] as op}
									<option value={op.value}>{op.label}</option>
								{/each}
							</select>

							<input
								type="text"
								bind:value={condition.value}
								placeholder="Value"
							/>

							<button class="btn-icon danger" onclick={() => removeCondition(index)}>
								<IconX size={16} />
							</button>
						</div>
					{/each}

					<button class="btn-secondary" onclick={addCondition}>
						<IconPlus size={16} />
						Add Condition
					</button>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showCreateSegmentModal = false)}>
					Cancel
				</button>
				<button class="btn-primary" onclick={handleCreateSegment}>
					<IconCheck size={18} />
					Create Segment
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Create Tag Modal -->
{#if showCreateTagModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={() => (showCreateTagModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateTagModal = false)}
	>
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="modal-content"
			role="document"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
		>
			<div class="modal-header">
				<h2>Create New Tag</h2>
				<button class="close-btn" onclick={() => (showCreateTagModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<div class="form-group">
					<label for="tag-name">Tag Name</label>
					<input
						id="tag-name"
						type="text"
						bind:value={newTag.name}
						placeholder="e.g., VIP"
					/>
				</div>

				<div class="form-group">
					<span class="form-label">Tag Color</span>
					<div class="color-picker">
						{#each tagColors as color}
							<button
								class="color-option"
								class:selected={newTag.color === color}
								style="background-color: {color}"
								onclick={() => (newTag.color = color)}
								aria-label="Select color {color}"
							></button>
						{/each}
					</div>
				</div>

				<div class="tag-preview">
					<span class="preview-label">Preview:</span>
					<span class="tag-badge" style="background-color: {newTag.color}20; color: {newTag.color}; border-color: {newTag.color}40">
						{newTag.name || 'Tag Name'}
					</span>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showCreateTagModal = false)}>
					Cancel
				</button>
				<button class="btn-primary" onclick={handleCreateTag}>
					<IconCheck size={18} />
					Create Tag
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.segments-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Header */
	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: #94a3b8;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: #a5b4fc;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.title-icon {
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.subtitle {
		color: #64748b;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	/* Tabs */
	.tabs-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
	}

	.tabs button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: none;
		border: none;
		color: #64748b;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border-radius: 10px;
		transition: all 0.2s;
	}

	.tabs button:hover {
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
	}

	.tabs button.active {
		background: rgba(99, 102, 241, 0.15);
		color: #a5b4fc;
	}

	/* Segments Grid */
	.segments-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
		gap: 1rem;
	}

	.segment-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.segment-card.system {
		border-color: rgba(99, 102, 241, 0.2);
	}

	.segment-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 0.75rem;
	}

	.segment-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.segment-info h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.system-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.2);
		color: #a5b4fc;
		font-size: 0.6875rem;
		font-weight: 600;
		border-radius: 6px;
		text-transform: uppercase;
	}

	.segment-count {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #a5b4fc;
	}

	.segment-description {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0 0 1rem;
	}

	.segment-conditions {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.condition-tag {
		padding: 0.375rem 0.75rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 8px;
		font-size: 0.75rem;
		color: #cbd5e1;
	}

	.segment-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.updated {
		font-size: 0.75rem;
		color: #64748b;
	}

	.segment-actions {
		display: flex;
		gap: 0.375rem;
	}

	/* Tags Grid */
	.tags-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.tag-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.25rem;
	}

	.tag-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.tag-name {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.tag-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background-color: var(--tag-color);
	}

	.tag-count {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.tag-actions {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	/* Filters List */
	.filters-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.filter-card {
		display: flex;
		align-items: center;
		gap: 1.5rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
		padding: 1.25rem 1.5rem;
	}

	.filter-info {
		flex: 1;
	}

	.filter-info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem;
	}

	.filter-meta {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.filter-pills {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.filter-pill {
		padding: 0.375rem 0.75rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 8px;
		font-size: 0.75rem;
		color: #a5b4fc;
	}

	.filter-actions {
		display: flex;
		gap: 0.5rem;
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.btn-secondary {
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-primary.small,
	.btn-secondary.small {
		padding: 0.5rem 0.875rem;
		font-size: 0.8125rem;
	}

	.btn-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(99, 102, 241, 0.15);
		border-color: rgba(99, 102, 241, 0.3);
		color: #a5b4fc;
	}

	.btn-icon.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		border-color: rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: #64748b;
		text-align: center;
		background: rgba(30, 41, 59, 0.3);
		border-radius: 16px;
		border: 2px dashed rgba(148, 163, 184, 0.2);
	}

	.empty-state h3 {
		color: #f1f5f9;
		margin: 1rem 0 0.5rem;
	}

	/* Loading */
	.loading-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
		gap: 1rem;
	}

	.skeleton {
		background: linear-gradient(90deg, rgba(148, 163, 184, 0.1) 25%, rgba(148, 163, 184, 0.2) 50%, rgba(148, 163, 184, 0.1) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 16px;
	}

	.skeleton-card {
		height: 200px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: #1e293b;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 20px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-content.large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(148, 163, 184, 0.1);
		border: none;
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	/* Form */
	.form-group {
		margin-bottom: 1rem;
	}

	.form-group label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.form-group input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	/* Conditions */
	.conditions-section {
		margin-top: 1.5rem;
	}

	.conditions-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.conditions-header .conditions-label {
		font-size: 0.8125rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.form-label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: #94a3b8;
		margin-bottom: 0.5rem;
	}

	.match-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	.condition-row {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.condition-row select,
	.condition-row input {
		flex: 1;
		padding: 0.625rem 0.875rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.condition-row select:focus,
	.condition-row input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	/* Color Picker */
	.color-picker {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.color-option {
		width: 32px;
		height: 32px;
		border-radius: 8px;
		border: 2px solid transparent;
		cursor: pointer;
		transition: all 0.2s;
	}

	.color-option:hover {
		transform: scale(1.1);
	}

	.color-option.selected {
		border-color: white;
		box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.5);
	}

	/* Tag Preview */
	.tag-preview {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-top: 1rem;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 10px;
	}

	.preview-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	.tag-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.8125rem;
		font-weight: 500;
		border: 1px solid;
	}

	@media (max-width: 768px) {
		.tabs-container {
			flex-direction: column;
			gap: 1rem;
			align-items: flex-start;
		}

		.segments-grid,
		.tags-grid,
		.loading-grid {
			grid-template-columns: 1fr;
		}

		.filter-card {
			flex-direction: column;
			align-items: flex-start;
		}

		.filter-actions {
			width: 100%;
			justify-content: flex-end;
		}
	}
</style>

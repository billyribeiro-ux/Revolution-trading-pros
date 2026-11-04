<script lang="ts">
	/**
	 * Member Segments - Apple ICT7 Principal Engineer Grade
	 *
	 * Svelte 5 runes implementation with:
	 * - $state for reactive state management
	 * - $effect for side effects and data loading
	 * - $derived for computed values
	 * - Full TypeScript type safety
	 *
	 */

	import { onMount, onDestroy } from 'svelte';
	import {
		listMemberSegments,
		saveMemberSegment,
		deleteMemberSegment,
		exportMemberSegment
	} from '#lib/api/member-segments.js';
	import { goto } from '$app/navigation';
	import { toastStore } from '#lib/stores/toast.svelte.js';
	import { adminFetch } from '#lib/utils/adminFetch.js';
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
		IconDownload,
		IconArrowLeft,
		IconChartBar,
		IconMail,
		IconCurrencyDollar,
		IconCalendar,
		IconTrendingUp
	} from '#lib/icons/index.js';
	import SegmentDetailDrawer from '#lib/components/admin/SegmentDetailDrawer.svelte';
	import ConfirmationModal from '#lib/components/admin/ConfirmationModal.svelte';

	// State - Svelte 5 Runes

	let loading = $state(true);
	let loadGeneration = 0;
	let loadErrors = $state({ segments: '', tags: '', saved: '' });
	let editSegmentId = $state<number | undefined>(undefined);
	let editSegmentType = $state<'smart' | 'static'>('smart');
	let savingSegment = $state(false);
	let segmentError = $state('');
	let actionError = $state('');
	let exportError = $state('');
	let isExporting = $state(false);
	let savingAction = $state(false);
	let editTagId = $state<number | undefined>(undefined);
	let segmentPage = $state(1);
	let segmentTotal = $state(0);
	let segmentSearch = $state('');
	onDestroy(() => {
		loadGeneration++;
	});
	let activeTab = $state<'segments' | 'tags' | 'saved'>('segments');

	// Segments
	let segments = $state<Segment[]>([]);
	let tags = $state<Tag[]>([]);
	let savedFilters = $state<SavedFilter[]>([]);

	// Create modal
	let showCreateSegmentModal = $state(false);
	let showCreateTagModal = $state(false);
	let showSaveFilterModal = $state(false);

	// Enterprise components state
	let showSegmentDrawer = $state(false);
	let selectedSegment = $state<Segment | null>(null);
	let showDeleteModal = $state(false);
	let deleteTarget = $state<{
		type: 'segment' | 'tag' | 'filter';
		item: Segment | Tag | SavedFilter;
	} | null>(null);
	let isDeleting = $state(false);

	// Form state
	let newSegment = $state({
		name: '',
		description: '',
		conditions: [] as Condition[]
	});
	let newTag = $state({ name: '', color: '#6366f1' });
	let newFilter = $state({ name: '', filters: {} as Record<string, string | number> });

	// FIX-2026-04-26 (audit 02 §P1-4 / §P3-2): `memberCount` is now nullable
	// because the backend doesn't always return it (and the page used to fake
	// it with `Math.random()`). UI renders "—" when null.
	interface Segment {
		id: number;
		name: string;
		description: string;
		type: 'smart' | 'static';
		conditions: Condition[];
		memberCount: number | null;
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
		memberCount: number | null;
	}

	interface SavedFilter {
		id: number;
		name: string;
		filters: Record<string, string | number>;
		createdAt: string;
		usageCount: number | null;
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
		email_engagement: [{ value: 'eq', label: 'Is' }],
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
		'#E6B800',
		'#B38F00',
		'#ec4899',
		'#ef4444',
		'#f97316',
		'#eab308',
		'#22c55e',
		'#14b8a6',
		'#06b6d4',
		'#3b82f6'
	];

	// Lifecycle - Svelte 5 $effect

	// FIX-2026-04-26 (audit 02 §P2-9 / CLAUDE.md commit 34a0bd070):
	// initialization should run once on mount, not in a `$effect`.
	onMount(() => {
		loadData();
	});

	// Data Loading - Real API Integration

	function object(value: unknown): Record<string, unknown> {
		if (!value || typeof value !== 'object' || Array.isArray(value))
			throw new Error('Invalid member configuration response');
		return value as Record<string, unknown>;
	}
	function integer(value: unknown): number {
		if (typeof value !== 'number' || !Number.isSafeInteger(value) || value < 0)
			throw new Error('Invalid member configuration count');
		return value;
	}
	function text(value: unknown): string {
		if (typeof value !== 'string') throw new Error('Invalid member configuration text');
		return value;
	}
	function tag(value: unknown): Tag {
		const row = object(value);
		return {
			id: integer(row.id),
			name: text(row.name),
			color: row.color === null ? '#64748b' : text(row.color),
			memberCount: integer(row.member_count)
		};
	}
	function savedFilter(value: unknown): SavedFilter {
		const row = object(value),
			filters = object(row.filters);
		const fields: Record<string, string | number> = {};
		for (const [key, value] of Object.entries(filters)) {
			if (typeof value !== 'string' && (typeof value !== 'number' || !Number.isFinite(value)))
				throw new Error('Invalid saved member filter');
			fields[key] = value;
		}
		return {
			id: integer(row.id),
			name: text(row.name),
			filters: fields,
			createdAt: text(row.created_at),
			usageCount: row.usage_count === undefined ? null : integer(row.usage_count)
		};
	}
	async function loadData(page = segmentPage) {
		const generation = ++loadGeneration;
		loading = true;
		loadErrors = { segments: '', tags: '', saved: '' };
		const results = await Promise.allSettled([
			listMemberSegments({ page, per_page: 20, search: segmentSearch.trim() || undefined }),
			adminFetch<unknown>('/api/admin/members/tags?per_page=100'),
			adminFetch<unknown>('/api/admin/members/filters?per_page=100')
		]);
		if (generation !== loadGeneration) return;
		const [segmentResult, tagResult, filterResult] = results;
		try {
			if (segmentResult.status === 'rejected') throw segmentResult.reason;
			segments = segmentResult.value.segments;
			segmentTotal = segmentResult.value.total;
			segmentPage = segmentResult.value.page;
		} catch (error) {
			loadErrors.segments =
				error instanceof Error ? error.message : 'Unable to load member segments';
		}
		try {
			if (tagResult.status === 'rejected') throw tagResult.reason;
			const value = object(tagResult.value);
			if (!Array.isArray(value.tags)) throw new Error('Invalid member tag list');
			tags = value.tags.map(tag);
		} catch (error) {
			loadErrors.tags = error instanceof Error ? error.message : 'Unable to load member tags';
		}
		try {
			if (filterResult.status === 'rejected') throw filterResult.reason;
			const value = object(filterResult.value);
			if (!Array.isArray(value.filters)) throw new Error('Invalid saved member filter list');
			savedFilters = value.filters.map(savedFilter);
		} catch (error) {
			loadErrors.saved =
				error instanceof Error ? error.message : 'Unable to load saved member filters';
		}
		loading = false;
	}

	function formatNumber(num: number | null | undefined): string {
		// FIX-2026-04-26 (audit 02 §P1-4): the page used to forge member counts
		// with `Math.random()`. Now we accept `null` from the backend and render
		// an em-dash so admins can tell "we don't know yet" from "zero".
		if (num === null || num === undefined || Number.isNaN(num)) return '—';
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

	function beginCreateSegment() {
		editSegmentId = undefined;
		editSegmentType = 'smart';
		newSegment = { name: '', description: '', conditions: [] };
		segmentError = '';
		showCreateSegmentModal = true;
	}
	async function handleCreateSegment() {
		if (savingSegment) return;
		segmentError = '';
		if (!newSegment.name.trim() || (!newSegment.conditions.length && editSegmentType === 'smart')) {
			segmentError = 'Enter a name and at least one condition';
			return;
		}
		savingSegment = true;
		try {
			const saved = await saveMemberSegment(
				{ ...newSegment, type: editSegmentType },
				editSegmentId
			);
			segments =
				editSegmentId === undefined
					? [saved, ...segments]
					: segments.map((row) => (row.id === saved.id ? saved : row));
			showCreateSegmentModal = false;
			toastStore.success(editSegmentId === undefined ? 'Segment created' : 'Segment updated');
			await loadData();
		} catch (error) {
			segmentError = error instanceof Error ? error.message : 'Unable to save member segment';
		} finally {
			savingSegment = false;
		}
	}

	function beginTag(tag?: Tag) {
		editTagId = tag?.id;
		newTag = tag ? { name: tag.name, color: tag.color } : { name: '', color: '#6366f1' };
		actionError = '';
		showCreateTagModal = true;
	}
	async function handleCreateTag() {
		if (savingAction) return;
		actionError = '';
		if (!newTag.name.trim()) {
			actionError = 'Enter a tag name';
			return;
		}
		savingAction = true;
		try {
			const saved = tag(
				await adminFetch<unknown>(
					`/api/admin/members/tags${editTagId === undefined ? '' : `/${editTagId}`}`,
					{ method: editTagId === undefined ? 'POST' : 'PUT', body: JSON.stringify(newTag) }
				)
			);
			tags =
				editTagId === undefined
					? [saved, ...tags]
					: tags.map((row) => (row.id === saved.id ? saved : row));
			showCreateTagModal = false;
			toastStore.success(editTagId === undefined ? 'Tag created' : 'Tag updated');
		} catch (error) {
			actionError = error instanceof Error ? error.message : 'Unable to save tag';
		} finally {
			savingAction = false;
		}
	}

	// Enterprise drawer handlers
	function openSegmentDrawer(segment: Segment) {
		selectedSegment = segment;
		showSegmentDrawer = true;
	}

	function closeSegmentDrawer() {
		showSegmentDrawer = false;
		selectedSegment = null;
	}

	function handleEditSegment(segment: Segment) {
		closeSegmentDrawer();
		editSegmentId = segment.id;
		editSegmentType = segment.type;
		newSegment = {
			name: segment.name,
			description: segment.description,
			conditions: segment.conditions.map((rule) => ({ ...rule }))
		};
		segmentError = '';
		showCreateSegmentModal = true;
	}

	function handleExportSegment(segment: Segment) {
		exportSegmentData(segment);
	}

	// Delete confirmation handlers
	function confirmDeleteSegment(segment: Segment) {
		if (segment.isSystem) {
			toastStore.error('Cannot delete system segments');
			return;
		}
		deleteTarget = { type: 'segment', item: segment };
		showDeleteModal = true;
	}

	function confirmDeleteTag(tag: Tag) {
		deleteTarget = { type: 'tag', item: tag };
		showDeleteModal = true;
	}

	function confirmDeleteFilter(filter: SavedFilter) {
		deleteTarget = { type: 'filter', item: filter };
		showDeleteModal = true;
	}

	async function handleConfirmDelete() {
		if (!deleteTarget) return;

		const targetType = deleteTarget.type;
		const targetItem = deleteTarget.item;

		isDeleting = true;
		try {
			// FIX-2026-04-26 (audit 02 §P2-4): the previous version swallowed
			// the API error and applied the local mutation regardless. With the
			// proxies now wired up, an HTTP 5xx would have left the UI showing
			// the row gone while the DB still had it. Now we re-throw on
			// failure, leave UI in place, and call `loadData()` to re-sync.
			if (targetType === 'segment') {
				const segment = targetItem as Segment;
				await deleteMemberSegment(segment.id);
				segments = segments.filter((s) => s.id !== segment.id);
				toastStore.success('Segment deleted');

				if (selectedSegment && selectedSegment.id === segment.id) {
					closeSegmentDrawer();
				}
			} else if (targetType === 'tag') {
				const tag = targetItem as Tag;
				await adminFetch(`/api/admin/members/tags/${tag.id}`, { method: 'DELETE' });
				tags = tags.filter((t) => t.id !== tag.id);
				toastStore.success('Tag deleted');
			} else if (targetType === 'filter') {
				const filter = targetItem as SavedFilter;
				await adminFetch(`/api/admin/members/filters/${filter.id}`, { method: 'DELETE' });
				savedFilters = savedFilters.filter((f) => f.id !== filter.id);
				toastStore.success('Filter deleted');
			}
		} catch (err) {
			console.error('Delete failed:', err);
			toastStore.error(err instanceof Error ? err.message : 'Delete failed — re-syncing');
			// Pull canonical state back so the UI never disagrees with the DB.
			await loadData();
		} finally {
			isDeleting = false;
			showDeleteModal = false;
			deleteTarget = null;
		}
	}

	function cancelDelete() {
		showDeleteModal = false;
		deleteTarget = null;
	}

	// Get delete modal config based on target
	let deleteModalConfig = $derived.by(() => {
		if (!deleteTarget) return { title: '', message: '', confirmText: 'Delete' };
		switch (deleteTarget.type) {
			case 'segment':
				return {
					title: 'Delete Segment',
					message: `Are you sure you want to delete "${(deleteTarget.item as Segment).name}"? This action cannot be undone.`,
					confirmText: 'Delete Segment'
				};
			case 'tag':
				return {
					title: 'Delete Tag',
					message: `Are you sure you want to delete "${(deleteTarget.item as Tag).name}"? It will be removed from all members.`,
					confirmText: 'Delete Tag'
				};
			case 'filter':
				return {
					title: 'Delete Saved Filter',
					message: `Are you sure you want to delete "${(deleteTarget.item as SavedFilter).name}"?`,
					confirmText: 'Delete Filter'
				};
			default:
				return { title: '', message: '', confirmText: 'Delete' };
		}
	});

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

	async function exportSegmentData(segment: Segment) {
		if (isExporting) return;
		isExporting = true;
		exportError = '';
		try {
			const blob = await exportMemberSegment(segment.id);
			const url = URL.createObjectURL(blob),
				link = document.createElement('a');
			link.href = url;
			link.download = `member-segment-${segment.id}.csv`;
			document.body.append(link);
			link.click();
			link.remove();
			setTimeout(() => URL.revokeObjectURL(url), 1000);
		} catch (error) {
			exportError = error instanceof Error ? error.message : 'Unable to export member segment';
		} finally {
			isExporting = false;
		}
	}

	async function handleSaveFilter() {
		if (savingAction) return;
		actionError = '';
		if (!newFilter.name.trim()) {
			actionError = 'Enter a filter name';
			return;
		}
		savingAction = true;
		try {
			const saved = savedFilter(
				await adminFetch<unknown>('/api/admin/members/filters', {
					method: 'POST',
					body: JSON.stringify(newFilter)
				})
			);
			savedFilters = [saved, ...savedFilters];
			showSaveFilterModal = false;
			newFilter = { name: '', filters: {} };
			toastStore.success('Filter saved');
		} catch (error) {
			actionError = error instanceof Error ? error.message : 'Unable to save filter';
		} finally {
			savingAction = false;
		}
	}
	async function duplicateFilter(filter: SavedFilter) {
		if (savingAction) return;
		savingAction = true;
		try {
			const saved = savedFilter(
				await adminFetch<unknown>('/api/admin/members/filters', {
					method: 'POST',
					body: JSON.stringify({
						name: `${filter.name.slice(0, 248)} (Copy)`,
						filters: filter.filters
					})
				})
			);
			savedFilters = [saved, ...savedFilters];
			toastStore.success('Filter duplicated');
		} catch (error) {
			toastStore.error(error instanceof Error ? error.message : 'Unable to duplicate filter');
		} finally {
			savingAction = false;
		}
	}
</script>

<div class="admin-segments">
	<div class="admin-page-container">
		{#if exportError}<p role="alert">{exportError}</p>{/if}
		<!-- Header -->
		<header class="page-header">
			<button class="back-btn" onclick={() => goto('/admin/members')}>
				<IconArrowLeft size={20} />
				Back to Members
			</button>

			<h1>Member Segments</h1>
			<p class="subtitle">Organize members with segments, tags, and filters</p>

			<div class="header-actions">
				<button class="btn-secondary" onclick={() => loadData()}>
					<IconRefresh size={18} />
					Refresh
				</button>
				<button class="btn-secondary" onclick={() => (showSaveFilterModal = true)}>
					<IconDownload size={18} />
					Save Filter
				</button>
			</div>
		</header>

		<!-- Tabs -->
		<div class="tabs-container">
			<div class="tabs">
				<button
					class={{ active: activeTab === 'segments' }}
					onclick={() => (activeTab = 'segments')}
				>
					<IconFilter size={18} />
					Smart Segments ({segments.length})
				</button>
				<button class={{ active: activeTab === 'tags' }} onclick={() => (activeTab = 'tags')}>
					<IconTag size={18} />
					Tags ({tags.length})
				</button>
				<button class={{ active: activeTab === 'saved' }} onclick={() => (activeTab = 'saved')}>
					<IconSearch size={18} />
					Saved Filters ({savedFilters.length})
				</button>
			</div>

			<div class="tab-actions">
				{#if activeTab === 'segments'}
					<button class="btn-primary" onclick={beginCreateSegment}>
						<IconPlus size={18} />
						New Segment
					</button>
				{:else if activeTab === 'tags'}
					<button class="btn-primary" onclick={() => beginTag()}>
						<IconPlus size={18} />
						New Tag
					</button>
				{/if}
			</div>
		</div>

		{#if loading}
			<div class="loading-grid">
				{#each [1, 2, 3, 4] as _, i (i)}
					<div class="skeleton skeleton-card"></div>
				{/each}
			</div>
		{:else if loadErrors[activeTab]}
			<div role="alert" class="empty-state">
				<p>{loadErrors[activeTab]}</p>
				<button class="btn-secondary" onclick={() => loadData()}>Retry</button>
			</div>
		{:else}
			<!-- Segments Tab -->
			{#if activeTab === 'segments'}
				<form
					onsubmit={(event) => {
						event.preventDefault();
						void loadData(1);
					}}
					class="segment-search"
				>
					<label>Search segments<input type="search" bind:value={segmentSearch} /></label>
					<button class="btn-secondary" type="submit">Search</button>
				</form>
				{#if !segments.length}<p class="empty-state">No matching member segments.</p>{/if}
				<div class="segments-grid">
					{#each segments as segment (segment.id)}
						<div
							class={['segment-card', { system: segment.isSystem }]}
							role="button"
							tabindex="0"
							onclick={() => openSegmentDrawer(segment)}
							onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && openSegmentDrawer(segment)}
						>
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
								{#each segment.conditions as condition, ci (ci)}
									<span class="condition-tag">
										{conditionFields.find((f) => f.value === condition.field)?.label ||
											condition.field}
										{conditionOperators[condition.field]?.find(
											(o) => o.value === condition.operator
										)?.label || condition.operator}
										{condition.value}
									</span>
								{/each}
							</div>

							<div class="segment-footer">
								<div class="footer-left">
									<span class="updated">
										<IconCalendar size={12} />
										Updated {formatDate(segment.lastUpdated)}
									</span>
									{#if (segment.memberCount ?? 0) > 1000}
										<span class="growth-badge">
											<IconTrendingUp size={12} />
											High Volume
										</span>
									{/if}
								</div>
								<!-- FIX-2026-04-26 (audit 02 §P2-3): action buttons used to bubble
								     into the card's openSegmentDrawer handler. Each onclick now
								     stopPropagation()s before invoking its action. -->
								<div class="segment-actions">
									<button
										class="btn-icon"
										type="button"
										onclick={(e) => {
											e.stopPropagation();
											viewSegmentMembers(segment);
										}}
										title="View members"
									>
										<IconUsers size={16} />
									</button>
									<button
										class="btn-icon"
										type="button"
										onclick={(e) => {
											e.stopPropagation();
											openSegmentDrawer(segment);
										}}
										title="View analytics"
									>
										<IconChartBar size={16} />
									</button>
									<button
										class="btn-icon"
										type="button"
										onclick={(e) => {
											e.stopPropagation();
											exportSegmentData(segment);
										}}
										title="Export data"
									>
										<IconDownload size={16} />
									</button>
									<button
										class="btn-icon"
										type="button"
										title="Send campaign"
										onclick={(e) => e.stopPropagation()}
									>
										<IconMail size={16} />
									</button>
									{#if !segment.isSystem}
										<button
											class="btn-icon"
											type="button"
											title="Edit"
											onclick={(e) => {
												e.stopPropagation();
												handleEditSegment(segment);
											}}
										>
											<IconEdit size={16} />
										</button>
										<button
											class="btn-icon danger"
											type="button"
											onclick={(e) => {
												e.stopPropagation();
												confirmDeleteSegment(segment);
											}}
											title="Delete"
										>
											<IconTrash size={16} />
										</button>
									{/if}
								</div>
							</div>
						</div>
					{/each}
				</div>
				<nav class="segment-search" aria-label="Member segment pages">
					<button
						class="btn-secondary"
						disabled={segmentPage <= 1}
						onclick={() => loadData(segmentPage - 1)}>Previous segments</button
					>
					<span>Page {segmentPage} · {segmentTotal} segments</span>
					<button
						class="btn-secondary"
						disabled={segmentPage * 20 >= segmentTotal}
						onclick={() => loadData(segmentPage + 1)}>Next segments</button
					>
				</nav>
			{/if}

			<!-- Tags Tab -->
			{#if activeTab === 'tags'}
				<div class="tags-grid">
					{#each tags as tag (tag.id)}
						<div class="tag-card">
							<div class="tag-header">
								<div class="tag-name" style:--tag-color={tag.color}>
									<span class="tag-dot"></span>
									{tag.name}
								</div>
								<div class="tag-count">
									{formatNumber(tag.memberCount)} members
								</div>
							</div>

							<div class="tag-actions">
								<button
									class="btn-secondary small"
									type="button"
									onclick={() => goto(`/admin/members?tag=${tag.id}`)}
								>
									<IconUsers size={14} />
									View Members
								</button>
								<button class="btn-icon" type="button" title="Edit" onclick={() => beginTag(tag)}>
									<IconEdit size={16} />
								</button>
								<button
									class="btn-icon danger"
									type="button"
									onclick={() => confirmDeleteTag(tag)}
									title="Delete"
								>
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
					{#each savedFilters as filter (filter.id)}
						<div class="filter-card">
							<div class="filter-info">
								<h3>{filter.name}</h3>
								<div class="filter-meta">
									<span>Created {formatDate(filter.createdAt)}</span>
									<span
										>{filter.usageCount === null
											? 'Usage history unavailable'
											: `Recorded uses: ${filter.usageCount}`}</span
									>
								</div>
							</div>

							<div class="filter-pills">
								{#each Object.entries(filter.filters) as [key, value] (key)}
									<span class="filter-pill">
										{key}: {value}
									</span>
								{/each}
							</div>

							<div class="filter-actions">
								<button
									class="btn-primary small"
									type="button"
									onclick={() => applySavedFilter(filter)}
								>
									Apply Filter
								</button>
								<button
									class="btn-icon"
									type="button"
									title="Duplicate"
									disabled={savingAction}
									onclick={() => duplicateFilter(filter)}
								>
									<IconCopy size={16} />
								</button>
								<button
									class="btn-icon danger"
									type="button"
									onclick={() => confirmDeleteFilter(filter)}
									title="Delete"
								>
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
</div>

<!-- Create Segment Modal -->
{#if showCreateSegmentModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={(e: MouseEvent) => {
			if (e.target === e.currentTarget) showCreateSegmentModal = false;
		}}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateSegmentModal = false)}
	>
		<div class="modal-content large" role="document">
			<div class="modal-header">
				<h2>{editSegmentId === undefined ? 'Create Smart Segment' : 'Edit Segment'}</h2>
				<button class="close-btn" onclick={() => (showCreateSegmentModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if segmentError}<p role="alert">{segmentError}</p>{/if}
				<div class="form-group">
					<label for="segment-name">Segment Name</label>
					<input
						autocomplete="off"
						id="segment-name"
						name="segment-name"
						type="text"
						bind:value={newSegment.name}
						placeholder="e.g., High Value Customers"
					/>
				</div>

				<div class="form-group">
					<label for="segment-description">Description</label>
					<input
						autocomplete="off"
						id="segment-description"
						name="segment-description"
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

					{#each newSegment.conditions as condition, index (index)}
						<div class="condition-row">
							<select
								id="segments-condition-field"
								name="segments_condition_field"
								bind:value={condition.field}
							>
								{#each conditionFields as field (field.value)}
									<option value={field.value}>
										{#if field.value === 'total_spent'}
											<IconCurrencyDollar size={14} />
										{/if}
										{field.label}
									</option>
								{/each}
							</select>

							<select
								id="segments-condition-operator"
								name="segments_condition_operator"
								bind:value={condition.operator}
							>
								{#each conditionOperators[condition.field] || [] as op (op.value)}
									<option value={op.value}>{op.label}</option>
								{/each}
							</select>

							<input
								autocomplete="off"
								id="page-condition-value"
								name="page-condition-value"
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
				<button class="btn-primary" disabled={savingSegment} onclick={handleCreateSegment}>
					<IconCheck size={18} />
					{savingSegment
						? 'Saving...'
						: editSegmentId === undefined
							? 'Create Segment'
							: 'Save Segment'}
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
		onclick={(e: MouseEvent) => {
			if (e.target === e.currentTarget) showCreateTagModal = false;
		}}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateTagModal = false)}
	>
		<div class="modal-content" role="document">
			<div class="modal-header">
				<h2>{editTagId === undefined ? 'Create New Tag' : 'Edit Tag'}</h2>
				<button class="close-btn" onclick={() => (showCreateTagModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if actionError}<p role="alert">{actionError}</p>{/if}
				<div class="form-group">
					<label for="tag-name">Tag Name</label>
					<input
						autocomplete="off"
						id="tag-name"
						name="tag-name"
						type="text"
						bind:value={newTag.name}
						placeholder="e.g., VIP"
					/>
				</div>

				<div class="form-group">
					<span class="form-label">Tag Color</span>
					<div class="color-picker">
						{#each tagColors as color (color)}
							<button
								class={['color-option', { selected: newTag.color === color }]}
								style:background-color={color}
								onclick={() => (newTag.color = color)}
								aria-label="Select color {color}"
							></button>
						{/each}
					</div>
				</div>

				<div class="tag-preview">
					<span class="preview-label">Preview:</span>
					<span
						class="tag-badge"
						style:background-color={`${newTag.color}20`}
						style:color={newTag.color}
						style:border-color={`${newTag.color}40`}
					>
						{newTag.name || 'Tag Name'}
					</span>
				</div>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showCreateTagModal = false)}> Cancel </button>
				<button class="btn-primary" disabled={savingAction} onclick={handleCreateTag}>
					<IconCheck size={18} />
					{savingAction ? 'Saving...' : editTagId === undefined ? 'Create Tag' : 'Save Tag'}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Save Filter Modal -->
{#if showSaveFilterModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={(e: MouseEvent) => {
			if (e.target === e.currentTarget) showSaveFilterModal = false;
		}}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showSaveFilterModal = false)}
	>
		<div class="modal-content" role="document">
			<div class="modal-header">
				<h2>Save Current Filter</h2>
				<button class="close-btn" onclick={() => (showSaveFilterModal = false)}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				{#if actionError}<p role="alert">{actionError}</p>{/if}
				<div class="form-group">
					<label for="filter-name">Filter Name</label>
					<input
						autocomplete="off"
						id="filter-name"
						name="filter-name"
						type="text"
						bind:value={newFilter.name}
						placeholder="e.g., High Value Active Members"
					/>
				</div>
				<p class="help-text">
					<IconCalendar size={14} />
					This will save your current filter settings for quick access later.
				</p>
			</div>

			<div class="modal-footer">
				<button class="btn-secondary" onclick={() => (showSaveFilterModal = false)}>
					Cancel
				</button>
				<button class="btn-primary" disabled={savingAction} onclick={handleSaveFilter}>
					<IconCheck size={18} />
					Save Filter
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Enterprise Segment Detail Drawer -->
<SegmentDetailDrawer
	isOpen={showSegmentDrawer}
	segment={selectedSegment}
	onClose={closeSegmentDrawer}
	onEdit={handleEditSegment}
	onDelete={confirmDeleteSegment}
	onExport={handleExportSegment}
/>

<!-- Delete Confirmation Modal -->
<ConfirmationModal
	isOpen={showDeleteModal}
	title={deleteModalConfig.title}
	message={deleteModalConfig.message}
	confirmText={deleteModalConfig.confirmText}
	variant="danger"
	isLoading={isDeleting}
	onConfirm={handleConfirmDelete}
	onCancel={cancelDelete}
/>

<style>
	.segment-search {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 1rem;
		margin-block: 1rem;
	}
	.segment-search label {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}
	.segment-search input {
		background: var(--bg-tertiary);
		color: var(--text-primary);
		border: 1px solid var(--border-color);
		padding: 0.5rem;
		border-radius: 0.4rem;
	}

	/* Root Container */
	.admin-segments {
		position: relative;
		overflow-x: hidden;
		background: var(--bg-base, #0d1117);
	}

	/* Animated Background Blobs */

	.admin-page-container {
		position: relative;
		z-index: 1;
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header - Centered Style */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.back-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0;
		color: var(--text-secondary, #8b949e);
		background: none;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-btn:hover {
		color: var(--primary-400);
	}

	/* Unused - keeping for future use
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
		background: #E6B800;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--bg-base, #0d1117);
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}*/

	.subtitle {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		margin: 0.25rem 0 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		margin-top: 1rem;
	}

	/* Tabs */
	.tabs-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
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
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		border-radius: 10px;
		transition: all 0.2s;
	}

	.tabs button:hover {
		background: rgba(230, 184, 0, 0.1);
		color: var(--primary-400);
	}

	.tabs button.active {
		background: rgba(230, 184, 0, 0.15);
		color: var(--primary-400);
	}

	/* Segments Grid */
	.segments-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
		gap: 1rem;
	}

	.segment-card {
		display: flex;
		flex-direction: column;
		text-align: left;
		width: 100%;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
		padding: 1.5rem;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: inherit;
	}

	.segment-card:hover {
		background: var(--bg-elevated, #161b22);
		border-color: rgba(230, 184, 0, 0.3);
		box-shadow: 0 8px 25px -8px rgba(0, 0, 0, 0.4);
	}

	.segment-card:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px rgba(230, 184, 0, 0.5);
	}

	.segment-card.system {
		border-color: rgba(230, 184, 0, 0.2);
	}

	.segment-card.system:hover {
		border-color: rgba(230, 184, 0, 0.4);
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
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.system-badge {
		padding: 0.25rem 0.5rem;
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-400);
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
		color: var(--primary-400);
	}

	.segment-description {
		color: var(--text-secondary, #8b949e);
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
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
		font-size: 0.75rem;
		color: var(--text-secondary, #8b949e);
	}

	.segment-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding-top: 1rem;
		border-top: 1px solid var(--border-muted, #21262d);
	}

	.footer-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.updated {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.growth-badge {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: rgba(63, 185, 80, 0.1);
		border: 1px solid rgba(63, 185, 80, 0.2);
		border-radius: 6px;
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--success-emphasis, #3fb950);
	}

	.segment-actions {
		display: flex;
		gap: 0.375rem;
	}

	.help-text {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		font-size: 0.8125rem;
		color: var(--primary-400);
	}

	/* Tags Grid */
	.tags-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.tag-card {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
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
		color: var(--text-primary, #f0f6fc);
	}

	.tag-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		background-color: var(--tag-color);
	}

	.tag-count {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
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
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
		padding: 1.25rem 1.5rem;
	}

	.filter-info {
		flex: 1;
	}

	.filter-info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem;
	}

	.filter-meta {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.filter-pills {
		display: flex;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.filter-pill {
		padding: 0.375rem 0.75rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		font-size: 0.75rem;
		color: var(--primary-400);
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
		background: var(--primary-500);
		color: var(--bg-base);
	}

	.btn-primary:hover {
		background: var(--primary-500, #e6b800);
	}

	.btn-secondary {
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
	}

	.btn-secondary:hover {
		background: var(--bg-hover, #252b33);
		border-color: var(--border-default, #30363d);
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
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: var(--primary-400);
	}

	.btn-icon.danger:hover {
		background: rgba(248, 81, 73, 0.1);
		border-color: rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
		background: rgba(30, 41, 59, 0.3);
		border-radius: 16px;
		border: 2px dashed var(--border-default, #30363d);
	}

	.empty-state h3 {
		color: var(--text-primary, #f0f6fc);
		margin: 1rem 0 0.5rem;
	}

	/* Loading */
	.loading-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
		gap: 1rem;
	}

	.skeleton {
		background: linear-gradient(
			90deg,
			var(--bg-surface, #1c2128) 25%,
			var(--bg-hover, #252b33) 50%,
			var(--bg-surface, #1c2128) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 16px;
	}

	.skeleton-card {
		height: 200px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: var(--bg-elevated, #161b22);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 500px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 60px -15px rgba(0, 0, 0, 0.7);
	}

	.modal-content.large {
		max-width: 700px;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
		background: var(--bg-base, #0d1117);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-hover, #252b33);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 10px;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-muted, #21262d);
		background: rgba(15, 23, 42, 0.3);
	}

	/* Form */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
	}

	.form-group input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 10px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	.form-group input:focus {
		outline: none;
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.form-group input::placeholder {
		color: var(--text-tertiary, #6e7681);
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
		color: var(--text-secondary, #8b949e);
	}

	.form-label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.5rem;
	}

	.match-label {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
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
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.875rem;
	}

	.condition-row select:focus,
	.condition-row input:focus {
		outline: none;
		border-color: rgba(230, 184, 0, 0.5);
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
		box-shadow: 0 0 0 2px rgba(230, 184, 0, 0.5);
	}

	/* Tag Preview */
	.tag-preview {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-top: 1rem;
		padding: 1rem;
		background: var(--bg-base, #0d1117);
		border-radius: 10px;
	}

	.preview-label {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.tag-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.8125rem;
		font-weight: 500;
		border: 1px solid;
	}

	@media (max-width: 767.98px) {
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

<script lang="ts">
	/**
	 * Dynamic Segments - FluentCRM Pro Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Create dynamic contact segments based on conditions.
	 * Contacts are automatically added/removed based on matching criteria.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { onMount } from 'svelte';
	import {
		IconFilter,
		IconPlus,
		IconSearch,
		IconEdit,
		IconTrash,
		IconEye,
		IconRefresh,
		IconUsers,
		IconChartBar,
		IconCopy,
		IconPlayerPlay,
		IconReload
	} from '@tabler/icons-svelte';
	import { api } from '$lib/api/config';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface DynamicSegment {
		id: string;
		title: string;
		description?: string;
		conditions: SegmentCondition[];
		match_type: 'all' | 'any';
		contacts_count: number;
		is_active: boolean;
		last_synced_at?: string;
		created_at: string;
		updated_at: string;
	}

	interface SegmentCondition {
		field: string;
		operator: string;
		value: any;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let segments = $state<DynamicSegment[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let filterActive = $state<boolean | 'all'>('all');

	let stats = $state({
		total: 0,
		active: 0,
		totalContacts: 0
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadSegments() {
		isLoading = true;
		error = '';

		try {
			const response = await api.get('/api/admin/crm/segments');
			segments = response?.data || response || [];

			stats = {
				total: segments.length,
				active: segments.filter(s => s.is_active).length,
				totalContacts: segments.reduce((sum, s) => sum + (s.contacts_count || 0), 0)
			};
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load segments';
		} finally {
			isLoading = false;
		}
	}

	async function duplicateSegment(id: string) {
		try {
			await api.post(`/api/admin/crm/segments/${id}/duplicate`);
			await loadSegments();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to duplicate segment';
		}
	}

	async function deleteSegment(id: string) {
		if (!confirm('Are you sure you want to delete this segment?')) return;

		try {
			await api.delete(`/api/admin/crm/segments/${id}`);
			await loadSegments();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete segment';
		}
	}

	async function syncSegment(id: string) {
		try {
			await api.post(`/api/admin/crm/segments/${id}/sync`);
			await loadSegments();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to sync segment';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatDate(dateString: string | undefined): string {
		if (!dateString) return 'Never';
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getConditionSummary(conditions: SegmentCondition[], matchType: string): string {
		if (!conditions || conditions.length === 0) return 'No conditions';
		const count = conditions.length;
		return `${count} condition${count > 1 ? 's' : ''} (${matchType === 'all' ? 'AND' : 'OR'})`;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredSegments = $derived(
		segments.filter(segment => {
			const matchesSearch = !searchQuery ||
				segment.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				segment.description?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchesActive = filterActive === 'all' || segment.is_active === filterActive;
			return matchesSearch && matchesActive;
		})
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		loadSegments();
	});
</script>

<svelte:head>
	<title>Dynamic Segments - FluentCRM Pro</title>
</svelte:head>

<div class="segments-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>
				<IconFilter size={28} class="header-icon" />
				Dynamic Segments
			</h1>
			<p class="page-description">Create smart segments that auto-update based on contact conditions</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadSegments()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<a href="/admin/crm/segments/new" class="btn-primary">
				<IconPlus size={18} />
				New Segment
			</a>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconFilter size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.total)}</span>
				<span class="stat-label">Total Segments</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon green">
				<IconPlayerPlay size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.active)}</span>
				<span class="stat-label">Active Segments</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconUsers size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{formatNumber(stats.totalContacts)}</span>
				<span class="stat-label">Total Contacts</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon amber">
				<IconChartBar size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">
					{stats.total > 0 ? Math.round(stats.totalContacts / stats.total) : 0}
				</span>
				<span class="stat-label">Avg per Segment</span>
			</div>
		</div>
	</div>

	<!-- Search & Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input type="text" placeholder="Search segments..." bind:value={searchQuery} />
		</div>
		<select class="filter-select" bind:value={filterActive}>
			<option value="all">All Segments</option>
			<option value={true}>Active</option>
			<option value={false}>Inactive</option>
		</select>
	</div>

	<!-- Segments Grid/Table -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading segments...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadSegments()}>Try Again</button>
		</div>
	{:else if filteredSegments.length === 0}
		<div class="empty-state">
			<IconFilter size={48} />
			<h3>No segments found</h3>
			<p>Create dynamic segments to automatically group contacts based on conditions</p>
			<a href="/admin/crm/segments/new" class="btn-primary">
				<IconPlus size={18} />
				Create Segment
			</a>
		</div>
	{:else}
		<div class="segments-grid">
			{#each filteredSegments as segment}
				<div class="segment-card">
					<div class="segment-header">
						<div class="segment-icon">
							<IconFilter size={24} />
						</div>
						<div class="segment-title-section">
							<h3>{segment.title}</h3>
							<span class="segment-status" class:active={segment.is_active}>
								{segment.is_active ? 'Active' : 'Inactive'}
							</span>
						</div>
					</div>

					{#if segment.description}
						<p class="segment-description">{segment.description}</p>
					{/if}

					<div class="segment-stats">
						<div class="stat">
							<IconUsers size={16} />
							<span>{formatNumber(segment.contacts_count)} contacts</span>
						</div>
						<div class="stat">
							<IconFilter size={16} />
							<span>{getConditionSummary(segment.conditions, segment.match_type)}</span>
						</div>
					</div>

					<div class="segment-meta">
						<span>Last synced: {formatDate(segment.last_synced_at)}</span>
					</div>

					<div class="segment-actions">
						<a href="/admin/crm/segments/{segment.id}" class="btn-action" title="View Contacts">
							<IconEye size={16} />
						</a>
						<a href="/admin/crm/segments/{segment.id}/edit" class="btn-action" title="Edit">
							<IconEdit size={16} />
						</a>
						<button class="btn-action" title="Sync Now" onclick={() => syncSegment(segment.id)}>
							<IconReload size={16} />
						</button>
						<button class="btn-action" title="Duplicate" onclick={() => duplicateSegment(segment.id)}>
							<IconCopy size={16} />
						</button>
						<button class="btn-action danger" title="Delete" onclick={() => deleteSegment(segment.id)}>
							<IconTrash size={16} />
						</button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.segments-page {
		max-width: 1600px;
		padding: 24px;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 12px;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-header h1 :global(.header-icon) {
		color: #f97316;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(249, 115, 22, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(249, 115, 22, 0.2);
		color: #f97316;
	}

	.btn-refresh :global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(249, 115, 22, 0.4);
	}

	/* Stats Grid */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(249, 115, 22, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue { background: rgba(59, 130, 246, 0.15); color: #60a5fa; }
	.stat-icon.green { background: rgba(34, 197, 94, 0.15); color: #4ade80; }
	.stat-icon.purple { background: rgba(139, 92, 246, 0.15); color: #a78bfa; }
	.stat-icon.amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; }

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Filters */
	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(249, 115, 22, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(249, 115, 22, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	/* Segments Grid */
	.segments-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.segment-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(249, 115, 22, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
		transition: all 0.2s;
	}

	.segment-card:hover {
		border-color: rgba(249, 115, 22, 0.3);
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(0, 0, 0, 0.2);
	}

	.segment-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.segment-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		flex-shrink: 0;
	}

	.segment-title-section {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.segment-title-section h3 {
		margin: 0;
		font-size: 1.1rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.segment-status {
		display: inline-flex;
		padding: 2px 8px;
		border-radius: 10px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		width: fit-content;
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}

	.segment-status.active {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}

	.segment-description {
		margin: 0 0 1rem;
		font-size: 0.85rem;
		color: #94a3b8;
		line-height: 1.5;
	}

	.segment-stats {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-bottom: 1rem;
		padding: 12px;
		background: rgba(15, 23, 42, 0.5);
		border-radius: 10px;
	}

	.segment-stats .stat {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 0.85rem;
		color: #e2e8f0;
	}

	.segment-stats .stat :global(svg) {
		color: #64748b;
	}

	.segment-meta {
		font-size: 0.75rem;
		color: #64748b;
		margin-bottom: 1rem;
	}

	.segment-actions {
		display: flex;
		gap: 8px;
		padding-top: 1rem;
		border-top: 1px solid rgba(249, 115, 22, 0.1);
	}

	.btn-action {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid rgba(249, 115, 22, 0.2);
		border-radius: 8px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
	}

	.btn-action:hover {
		background: rgba(249, 115, 22, 0.1);
		color: #f97316;
		border-color: rgba(249, 115, 22, 0.3);
	}

	.btn-action.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
		border-color: rgba(239, 68, 68, 0.3);
	}

	/* States */
	.loading-state, .error-state, .empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0 0 1.5rem 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(249, 115, 22, 0.2);
		border-top-color: #f97316;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* Responsive */
	@media (max-width: 1200px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.segments-page {
			padding: 16px;
		}

		.stats-grid {
			grid-template-columns: 1fr;
		}

		.segments-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

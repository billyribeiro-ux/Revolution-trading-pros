<!--
	/admin/watchlist - Weekly Watchlist Management
	Apple Principal Engineer ICT 7 Grade - January 2026

	Features:
	- Watchlist listing with search and filtering
	- Status management (draft/published/archived)
	- Room targeting support for all 6 services
	- Quick actions (edit, view, publish, archive, delete)
	- Real-time stats dashboard
	- Full Svelte 5 $state/$derived/$effect reactivity
-->

<script lang="ts">
	/**
	 * Admin Weekly Watchlist - Content Management
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Admin interface for managing weekly watchlist items.
	 * Allows creating, editing, and managing watchlist videos.
	 *
	 * @version 2.0.0 (January 2026) - Added ICT7 header
	 */

	import { browser } from '$app/environment';
	import { watchlistApi, type WatchlistItem } from '$lib/api/watchlist';
	import { ROOMS, ALL_ROOM_IDS, isAllRooms, getRoomsByIds } from '$lib/config/rooms';
	import RoomSelector from '$lib/components/admin/RoomSelector.svelte';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconDots from '@tabler/icons-svelte/icons/dots';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let selectedStatus = $state<string>('');
	let showDeleteModal = $state(false);
	let itemToDelete = $state<WatchlistItem | null>(null);
	let activeDropdown = $state<string | null>(null);
	let showCreateModal = $state(false);

	// API state
	let items = $state<WatchlistItem[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let isSaving = $state(false);

	// Create form state
	let newItem = $state({
		title: '',
		trader: '',
		traderImage: '',
		weekOf: '',
		description: '',
		videoSrc: '',
		videoPoster: '',
		spreadsheetSrc: '',
		status: 'draft' as 'published' | 'draft' | 'archived',
		rooms: [...ALL_ROOM_IDS] as string[]
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// DATA FETCHING
	// ═══════════════════════════════════════════════════════════════════════════

	// Initial data load using $effect
	$effect(() => {
		if (!browser) return;

		// Fetch watchlist items on mount
		loadWatchlistItems();
	});

	async function loadWatchlistItems() {
		isLoading = true;
		error = null;

		try {
			const response = await watchlistApi.getAll({ per_page: 50 });
			items = response.data || [];
		} catch (err) {
			console.error('Failed to fetch watchlist items:', err);
			error = err instanceof Error ? err.message : 'Failed to load watchlist items';
		} finally {
			isLoading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED DATA
	// ═══════════════════════════════════════════════════════════════════════════

	// Filter items
	let filteredItems = $derived.by(() => {
		return items.filter((item) => {
			// Search filter
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				if (
					!item.title.toLowerCase().includes(query) &&
					!item.trader.toLowerCase().includes(query)
				) {
					return false;
				}
			}

			// Status filter
			if (selectedStatus && item.status !== selectedStatus) {
				return false;
			}

			return true;
		});
	});

	// Stats
	let stats = $derived.by(() => {
		return {
			total: items.length,
			published: items.filter((i) => i.status === 'published').length,
			draft: items.filter((i) => i.status === 'draft').length,
			archived: items.filter((i) => i.status === 'archived').length
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleDeleteClick(item: WatchlistItem) {
		itemToDelete = item;
		showDeleteModal = true;
		activeDropdown = null;
	}

	async function handleConfirmDelete() {
		if (!itemToDelete) return;

		try {
			await watchlistApi.delete(itemToDelete.slug);
			items = items.filter((i) => i.slug !== itemToDelete!.slug);
		} catch (err) {
			console.error('Failed to delete item:', err);
		}

		showDeleteModal = false;
		itemToDelete = null;
	}

	async function handlePublish(item: WatchlistItem) {
		try {
			const response = await watchlistApi.publish(item.slug);
			items = items.map((i) => (i.slug === item.slug ? response.data : i));
		} catch (err) {
			console.error('Failed to publish item:', err);
		}
		activeDropdown = null;
	}

	async function handleArchive(item: WatchlistItem) {
		try {
			const response = await watchlistApi.archive(item.slug);
			items = items.map((i) => (i.slug === item.slug ? response.data : i));
		} catch (err) {
			console.error('Failed to archive item:', err);
		}
		activeDropdown = null;
	}

	function toggleDropdown(slug: string) {
		activeDropdown = activeDropdown === slug ? null : slug;
	}

	function clearFilters() {
		searchQuery = '';
		selectedStatus = '';
	}

	function resetCreateForm() {
		newItem = {
			title: '',
			trader: '',
			traderImage: '',
			weekOf: '',
			description: '',
			videoSrc: '',
			videoPoster: '',
			spreadsheetSrc: '',
			status: 'draft',
			rooms: [...ALL_ROOM_IDS]
		};
	}

	async function handleCreate() {
		if (!newItem.title || !newItem.trader || !newItem.weekOf) {
			return;
		}

		if (newItem.rooms.length === 0) {
			alert('Please select at least one room');
			return;
		}

		isSaving = true;
		try {
			const response = await watchlistApi.create({
				title: newItem.title,
				trader: newItem.trader,
				traderImage: newItem.traderImage || undefined,
				weekOf: newItem.weekOf,
				description: newItem.description || undefined,
				videoSrc: newItem.videoSrc || undefined,
				videoPoster: newItem.videoPoster || undefined,
				spreadsheetSrc: newItem.spreadsheetSrc || undefined,
				status: newItem.status,
				rooms: newItem.rooms
			});
			items = [response.data, ...items];
			showCreateModal = false;
			resetCreateForm();
		} catch (err) {
			console.error('Failed to create item:', err);
		}
		isSaving = false;
	}

	// Format date
	function formatDate(date: string): string {
		return new Date(date).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}
</script>

<svelte:head>
	<title>Weekly Watchlist | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-watchlist">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<!-- Header - Centered Style -->
		<header class="page-header">
			<h1>Weekly Watchlist</h1>
			<p class="subtitle">Manage weekly watchlist videos and content</p>
			<div class="header-actions">
				<button type="button" class="btn-primary" onclick={() => (showCreateModal = true)}>
					<IconPlus size={18} />
					Add Watchlist
				</button>
			</div>
		</header>

		<!-- Stats -->
		<div class="stats-grid">
			<div class="stat-card">
				<span class="stat-value">{stats.total}</span>
				<span class="stat-label">Total Items</span>
			</div>
			<div class="stat-card">
				<span class="stat-value">{stats.published}</span>
				<span class="stat-label">Published</span>
			</div>
			<div class="stat-card">
				<span class="stat-value">{stats.draft}</span>
				<span class="stat-label">Drafts</span>
			</div>
			<div class="stat-card">
				<span class="stat-value">{stats.archived}</span>
				<span class="stat-label">Archived</span>
			</div>
		</div>

		<!-- Filters -->
		<div class="filters-bar">
			<div class="search-box">
				<IconSearch size={18} />
				<input
					type="text"
					id="search-watchlist"
					name="search-watchlist"
					placeholder="Search watchlist items..."
					bind:value={searchQuery}
				/>
			</div>

			<div class="filter-group">
				<select bind:value={selectedStatus}>
					<option value="">All Status</option>
					<option value="published">Published</option>
					<option value="draft">Draft</option>
					<option value="archived">Archived</option>
				</select>
			</div>

			{#if searchQuery || selectedStatus}
				<button type="button" class="clear-btn" onclick={clearFilters}> Clear filters </button>
			{/if}
		</div>

		<!-- Content -->
		{#if isLoading}
			<div class="loading-state">
				<div class="loading-spinner"></div>
				<p>Loading watchlist items...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<p>{error}</p>
				<button type="button" onclick={() => location.reload()}>Try Again</button>
			</div>
		{:else}
			<!-- Items Table -->
			<div class="table-container">
				<table class="data-table">
					<thead>
						<tr>
							<th class="col-title">Watchlist</th>
							<th class="col-trader">Trader</th>
							<th class="col-week">Week Of</th>
							<th class="col-rooms">Rooms</th>
							<th class="col-status">Status</th>
							<th class="col-actions">Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each filteredItems as item (item.slug)}
							<tr>
								<td class="col-title">
									<div class="item-cell">
										<div class="item-thumb">
											{#if item.video.poster}
												<img src={item.video.poster} alt="" />
											{:else}
												<div class="thumb-placeholder">
													<IconVideo size={20} />
												</div>
											{/if}
										</div>
										<div class="item-info">
											<span class="item-title">{item.title}</span>
											<span class="item-meta">{item.subtitle}</span>
										</div>
									</div>
								</td>
								<td class="col-trader">
									<div class="trader-cell">
										{#if item.traderImage}
											<img src={item.traderImage} alt="" class="trader-avatar" />
										{/if}
										<span>{item.trader}</span>
									</div>
								</td>
								<td class="col-week">
									<div class="week-cell">
										<IconCalendar size={14} />
										{formatDate(item.weekOf)}
									</div>
								</td>
								<td class="col-rooms">
									<div class="rooms-cell">
										{#if !item.rooms || item.rooms.length === 0}
											<span class="rooms-badge rooms-none">None</span>
										{:else if isAllRooms(item.rooms)}
											<span class="rooms-badge rooms-all">All Rooms</span>
										{:else}
											<div class="rooms-tags">
												{#each getRoomsByIds(item.rooms).slice(0, 2) as room}
													<span
														class="room-tag"
														style="background-color: {room.color}20; color: {room.color}"
														>{room.shortName}</span
													>
												{/each}
												{#if item.rooms.length > 2}
													<span class="rooms-more">+{item.rooms.length - 2}</span>
												{/if}
											</div>
										{/if}
									</div>
								</td>
								<td class="col-status">
									<span class="status-badge status-{item.status}">
										{item.status}
									</span>
								</td>
								<td class="col-actions">
									<div class="actions-cell">
										<a href="/admin/watchlist/{item.slug}/edit" class="action-btn" title="Edit">
											<IconEdit size={16} />
										</a>
										<a
											href="/watchlist/{item.slug}"
											class="action-btn"
											title="View"
											target="_blank"
										>
											<IconEye size={16} />
										</a>
										<div class="dropdown">
											<button
												type="button"
												class="action-btn"
												onclick={() => toggleDropdown(item.slug)}
											>
												<IconDots size={16} />
											</button>
											{#if activeDropdown === item.slug}
												<div class="dropdown-menu">
													{#if item.status === 'draft'}
														<button onclick={() => handlePublish(item)}>
															<IconCheck size={14} />
															Publish
														</button>
													{/if}
													{#if item.status === 'published'}
														<button onclick={() => handleArchive(item)}>
															<IconX size={14} />
															Archive
														</button>
													{/if}
													<button class="danger" onclick={() => handleDeleteClick(item)}>
														<IconTrash size={14} />
														Delete
													</button>
												</div>
											{/if}
										</div>
									</div>
								</td>
							</tr>
						{:else}
							<tr>
								<td colspan="7" class="empty-state">
									<p>No watchlist items found</p>
									{#if searchQuery || selectedStatus}
										<button type="button" onclick={clearFilters}>Clear filters</button>
									{:else}
										<button type="button" onclick={() => (showCreateModal = true)}
											>Create your first watchlist</button
										>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<!-- Create Modal -->
{#if showCreateModal}
	<div
		class="modal-overlay"
		onclick={() => (showCreateModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateModal = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div
			class="modal modal-large"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="0"
		>
			<h3>Create Weekly Watchlist</h3>
			<form
				onsubmit={(e: Event) => {
					e.preventDefault();
					handleCreate();
				}}
			>
				<div class="form-grid">
					<div class="form-group">
						<label for="title">Title *</label>
						<input
							id="title" name="title"
							type="text"
							bind:value={newItem.title}
							placeholder="Weekly Watchlist with TG Watkins"
							required
						/>
					</div>

					<div class="form-group">
						<label for="trader">Trader *</label>
						<input
							id="trader" name="trader"
							type="text"
							bind:value={newItem.trader}
							placeholder="TG Watkins"
							required
						/>
					</div>

					<div class="form-group">
						<label for="weekOf">Week Of *</label>
						<input id="weekOf" name="weekOf" type="date" bind:value={newItem.weekOf} required />
					</div>

					<div class="form-group">
						<label for="status">Status</label>
						<select id="status" bind:value={newItem.status}>
							<option value="draft">Draft</option>
							<option value="published">Published</option>
						</select>
					</div>

					<div class="form-group full-width">
						<label for="description">Description</label>
						<textarea
							id="description"
							bind:value={newItem.description}
							placeholder="Week of December 22, 2025."
							rows="2"
						></textarea>
					</div>

					<div class="form-group full-width">
						<label for="videoSrc">Video URL</label>
						<input
							id="videoSrc" name="videoSrc"
							type="url"
							bind:value={newItem.videoSrc}
							placeholder="https://..."
						/>
					</div>

					<div class="form-group">
						<label for="videoPoster">Video Poster URL</label>
						<input
							id="videoPoster" name="videoPoster"
							type="url"
							bind:value={newItem.videoPoster}
							placeholder="https://..."
						/>
					</div>

					<div class="form-group">
						<label for="spreadsheetSrc">Spreadsheet URL</label>
						<input
							id="spreadsheetSrc" name="spreadsheetSrc"
							type="url"
							bind:value={newItem.spreadsheetSrc}
							placeholder="https://docs.google.com/..."
						/>
					</div>

					<div class="form-group full-width">
						<RoomSelector bind:selectedRooms={newItem.rooms} />
					</div>
				</div>

				<div class="modal-actions">
					<button type="button" class="btn-cancel" onclick={() => (showCreateModal = false)}>
						Cancel
					</button>
					<button type="submit" class="btn-primary" disabled={isSaving}>
						{isSaving ? 'Creating...' : 'Create Watchlist'}
					</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Delete Modal -->
{#if showDeleteModal}
	<div
		class="modal-overlay"
		onclick={() => (showDeleteModal = false)}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showDeleteModal = false)}
		role="button"
		tabindex="-1"
		aria-label="Close modal"
	>
		<div
			class="modal"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="0"
		>
			<h3>Delete Watchlist?</h3>
			<p>Are you sure you want to delete "{itemToDelete?.title}"? This action cannot be undone.</p>
			<div class="modal-actions">
				<button type="button" class="btn-cancel" onclick={() => (showDeleteModal = false)}>
					Cancel
				</button>
				<button type="button" class="btn-delete" onclick={handleConfirmDelete}> Delete </button>
			</div>
		</div>
	</div>
{/if}

<style>
	.admin-page {
		padding: 32px;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 32px;
	}

	.page-header h1 {
		margin: 0;
		font-size: 1.75rem;
		font-weight: 700;
		color: white;
	}

	.subtitle {
		margin: 4px 0 0;
		font-size: 0.9rem;
		color: #94a3b8;
	}

	.header-actions {
		display: flex;
		gap: 12px;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		transition: all 0.2s ease;
		border: none;
		cursor: pointer;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	/* Stats */
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 16px;
		margin-bottom: 24px;
	}

	.stat-card {
		background: #1e293b;
		border-radius: 12px;
		padding: 20px;
		border: 1px solid #334155;
	}

	.stat-value {
		display: block;
		font-size: 1.75rem;
		font-weight: 700;
		color: white;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	/* Filters */
	.filters-bar {
		display: flex;
		flex-wrap: wrap;
		gap: 12px;
		margin-bottom: 24px;
		align-items: center;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		flex: 1;
		max-width: 300px;
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		background: none;
		border: none;
		outline: none;
		color: white;
		font-size: 0.875rem;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-group {
		display: flex;
		gap: 8px;
	}

	.filter-group select {
		padding: 10px 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.clear-btn {
		padding: 10px 16px;
		background: none;
		border: 1px solid #475569;
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.8rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.clear-btn:hover {
		border-color: #f97316;
		color: #f97316;
	}

	/* Loading/Error States */
	.loading-state,
	.error-state {
		text-align: center;
		padding: 60px 20px;
		color: #64748b;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #334155;
		border-top-color: #f97316;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		margin: 0 auto 16px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.error-state button {
		margin-top: 16px;
		padding: 10px 20px;
		background: #f97316;
		color: white;
		border: none;
		border-radius: 8px;
		cursor: pointer;
	}

	/* Table */
	.table-container {
		background: #1e293b;
		border-radius: 12px;
		border: 1px solid #334155;
		overflow: hidden;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
	}

	.data-table th,
	.data-table td {
		padding: 14px 16px;
		text-align: left;
	}

	.data-table th {
		background: #0f172a;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.data-table td {
		border-top: 1px solid #334155;
		font-size: 0.875rem;
		color: #cbd5e1;
	}

	.data-table tbody tr:hover {
		background: rgba(255, 255, 255, 0.02);
	}

	/* Column widths */
	.col-title {
		width: 30%;
	}
	.col-trader {
		width: 12%;
	}
	.col-week {
		width: 12%;
	}
	.col-rooms {
		width: 18%;
	}
	.col-status {
		width: 10%;
	}
	.col-actions {
		width: 10%;
	}

	/* Item cell */
	.item-cell {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.item-thumb {
		width: 80px;
		height: 45px;
		border-radius: 4px;
		overflow: hidden;
		background: #334155;
		flex-shrink: 0;
	}

	.item-thumb img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumb-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.item-info {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.item-title {
		font-weight: 500;
		color: white;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.item-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Trader cell */
	.trader-cell {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.trader-avatar {
		width: 28px;
		height: 28px;
		border-radius: 50%;
		object-fit: cover;
	}

	/* Week cell */
	.week-cell {
		display: flex;
		align-items: center;
		gap: 6px;
		color: #94a3b8;
	}

	/* Rooms cell */
	.rooms-cell {
		display: flex;
		align-items: center;
	}

	.rooms-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 9999px;
		font-size: 0.7rem;
		font-weight: 600;
	}

	.rooms-all {
		background: linear-gradient(135deg, rgba(16, 185, 129, 0.15), rgba(9, 132, 174, 0.15));
		color: #10b981;
	}

	.rooms-none {
		background: rgba(100, 116, 139, 0.1);
		color: #64748b;
	}

	.rooms-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
		align-items: center;
	}

	.room-tag {
		display: inline-block;
		padding: 3px 8px;
		border-radius: 4px;
		font-size: 0.65rem;
		font-weight: 700;
	}

	.rooms-more {
		font-size: 0.7rem;
		color: #64748b;
		margin-left: 4px;
	}

	/* Status badge */
	.status-badge {
		display: inline-block;
		padding: 4px 10px;
		border-radius: 9999px;
		font-size: 0.7rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-published {
		background: rgba(34, 197, 94, 0.1);
		color: #22c55e;
	}

	.status-draft {
		background: rgba(251, 191, 36, 0.1);
		color: #fbbf24;
	}

	.status-archived {
		background: rgba(100, 116, 139, 0.1);
		color: #64748b;
	}

	/* Actions */
	.actions-cell {
		display: flex;
		gap: 4px;
	}

	.action-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: none;
		border: none;
		border-radius: 6px;
		color: #64748b;
		cursor: pointer;
		text-decoration: none;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #334155;
		color: white;
	}

	/* Dropdown */
	.dropdown {
		position: relative;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		z-index: 10;
		min-width: 140px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
		padding: 4px;
	}

	.dropdown-menu button {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 8px 12px;
		background: none;
		border: none;
		border-radius: 4px;
		color: #cbd5e1;
		font-size: 0.8rem;
		cursor: pointer;
		text-align: left;
	}

	.dropdown-menu button:hover {
		background: #334155;
	}

	.dropdown-menu button.danger {
		color: #ef4444;
	}

	.dropdown-menu button.danger:hover {
		background: rgba(239, 68, 68, 0.1);
	}

	/* Empty state */
	.empty-state {
		text-align: center;
		padding: 60px 20px !important;
		color: #64748b;
	}

	.empty-state p {
		margin: 0 0 16px;
	}

	.empty-state button {
		color: #f97316;
		background: none;
		border: none;
		cursor: pointer;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: #1e293b;
		border-radius: 12px;
		padding: 24px;
		max-width: 400px;
		width: 90%;
	}

	.modal-large {
		max-width: 600px;
	}

	.modal h3 {
		margin: 0 0 20px;
		font-size: 1.25rem;
		color: white;
	}

	.modal p {
		margin: 0 0 24px;
		font-size: 0.9rem;
		color: #94a3b8;
	}

	/* Form */
	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
		margin-bottom: 24px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group.full-width {
		grid-column: 1 / -1;
	}

	.form-group label {
		font-size: 0.8rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 10px 14px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #f97316;
	}

	.form-group textarea {
		resize: vertical;
	}

	.modal-actions {
		display: flex;
		gap: 12px;
		justify-content: flex-end;
	}

	.btn-cancel,
	.btn-delete {
		padding: 10px 20px;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
	}

	.btn-cancel {
		background: #334155;
		color: white;
	}

	.btn-delete {
		background: #ef4444;
		color: white;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}

		.col-week,
		.col-rooms {
			display: none;
		}
	}

	@media (max-width: 768px) {
		.admin-page {
			padding: 16px;
		}

		.page-header {
			flex-direction: column;
			gap: 16px;
		}

		.stats-grid {
			grid-template-columns: 1fr 1fr;
		}

		.filters-bar {
			flex-direction: column;
		}

		.search-box {
			width: 100%;
			max-width: none;
		}

		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

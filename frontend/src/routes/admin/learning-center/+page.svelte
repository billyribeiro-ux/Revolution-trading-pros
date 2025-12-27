<script lang="ts">
	/**
	 * Admin Learning Center - Content Management
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Admin interface for managing learning center content across all trading rooms.
	 * Allows creating, editing, and organizing lessons, modules, and categories.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { learningCenterStore } from '$lib/stores/learningCenter';
	import type { LessonWithRelations } from '$lib/types/learning-center';
	import { get } from 'svelte/store';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconCopy from '@tabler/icons-svelte/icons/copy';
	import IconDots from '@tabler/icons-svelte/icons/dots';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconFileText from '@tabler/icons-svelte/icons/file-text';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let selectedRoom = $state<string>('');
	let selectedTrainer = $state<string>('');
	let selectedStatus = $state<string>('');
	let showDeleteModal = $state(false);
	let lessonToDelete = $state<string | null>(null);
	let activeDropdown = $state<string | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// STORE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	let storeData = $derived(get(learningCenterStore));

	// Get trading rooms with learning centers
	let tradingRooms = $derived(
		storeData.tradingRooms.filter(r => r.hasLearningCenter && r.isActive)
	);

	// Get all trainers
	let trainers = $derived(storeData.trainers);

	// Get all lessons with relations
	let allLessons = $derived.by((): LessonWithRelations[] => {
		// Get lessons from store
		const lessons = storeData.lessons;

		// Enrich with relations
		return lessons.map(lesson => {
			const trainer = storeData.trainers.find(t => t.id === lesson.trainerId);
			const category = storeData.categories.find(c => c.id === lesson.categoryId);
			return {
				...lesson,
				...(trainer && { trainer }),
				...(category && { category }),
				tradingRooms: storeData.tradingRooms.filter(r => lesson.tradingRoomIds?.includes(r.id))
			} as LessonWithRelations;
		});
	});

	// Filter lessons
	let filteredLessons = $derived.by(() => {
		return allLessons.filter(lesson => {
			// Search filter
			if (searchQuery) {
				const query = searchQuery.toLowerCase();
				if (!lesson.title.toLowerCase().includes(query) &&
					!lesson.description.toLowerCase().includes(query)) {
					return false;
				}
			}

			// Room filter
			if (selectedRoom && !lesson.tradingRoomIds?.includes(selectedRoom)) {
				return false;
			}

			// Trainer filter
			if (selectedTrainer && lesson.trainerId !== selectedTrainer) {
				return false;
			}

			// Status filter
			if (selectedStatus && lesson.status !== selectedStatus) {
				return false;
			}

			return true;
		});
	});

	// Stats
	let stats = $derived.by(() => {
		return {
			total: allLessons.length,
			published: allLessons.filter(l => l.status === 'published').length,
			draft: allLessons.filter(l => l.status === 'draft').length,
			videos: allLessons.filter(l => l.type === 'video').length
		};
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function handleDeleteClick(lessonId: string) {
		lessonToDelete = lessonId;
		showDeleteModal = true;
		activeDropdown = null;
	}

	async function handleConfirmDelete() {
		if (!lessonToDelete) return;

		try {
			// TODO: Implement delete functionality via API
			console.log('Delete lesson:', lessonToDelete);
		} catch (err) {
			console.error('Failed to delete lesson:', err);
		}

		showDeleteModal = false;
		lessonToDelete = null;
	}

	function handleDuplicate(lesson: LessonWithRelations) {
		// Navigate to create page with lesson data pre-filled
		const data = encodeURIComponent(JSON.stringify({
			...lesson,
			title: `${lesson.title} (Copy)`,
			slug: `${lesson.slug}-copy`,
			id: undefined
		}));
		window.location.href = `/admin/learning-center/create?duplicate=${data}`;
	}

	function toggleDropdown(lessonId: string) {
		activeDropdown = activeDropdown === lessonId ? null : lessonId;
	}

	function clearFilters() {
		searchQuery = '';
		selectedRoom = '';
		selectedTrainer = '';
		selectedStatus = '';
	}

	// Get type icon
	function getTypeIcon(type: string) {
		switch (type) {
			case 'video':
				return IconVideo;
			case 'article':
				return IconFileText;
			default:
				return IconFileText;
		}
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
	<title>Learning Center | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-left">
			<h1>Learning Center</h1>
			<p class="subtitle">Manage educational content for all trading rooms</p>
		</div>
		<div class="header-actions">
			<a href="/admin/learning-center/modules" class="btn-secondary">
				Manage Modules
			</a>
			<a href="/admin/learning-center/create" class="btn-primary">
				<IconPlus size={18} />
				Add Lesson
			</a>
		</div>
	</header>

	<!-- Stats -->
	<div class="stats-grid">
		<div class="stat-card">
			<span class="stat-value">{stats.total}</span>
			<span class="stat-label">Total Lessons</span>
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
			<span class="stat-value">{stats.videos}</span>
			<span class="stat-label">Videos</span>
		</div>
	</div>

	<!-- Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search lessons..."
				bind:value={searchQuery}
			/>
		</div>

		<div class="filter-group">
			<select bind:value={selectedRoom}>
				<option value="">All Rooms</option>
				{#each tradingRooms as room}
					<option value={room.id}>{room.shortName || room.name}</option>
				{/each}
			</select>

			<select bind:value={selectedTrainer}>
				<option value="">All Trainers</option>
				{#each trainers as trainer}
					<option value={trainer.id}>{trainer.name}</option>
				{/each}
			</select>

			<select bind:value={selectedStatus}>
				<option value="">All Status</option>
				<option value="published">Published</option>
				<option value="draft">Draft</option>
				<option value="archived">Archived</option>
			</select>
		</div>

		{#if searchQuery || selectedRoom || selectedTrainer || selectedStatus}
			<button type="button" class="clear-btn" onclick={clearFilters}>
				Clear filters
			</button>
		{/if}
	</div>

	<!-- Lessons Table -->
	<div class="table-container">
		<table class="data-table">
			<thead>
				<tr>
					<th class="col-title">Lesson</th>
					<th class="col-rooms">Trading Rooms</th>
					<th class="col-trainer">Trainer</th>
					<th class="col-status">Status</th>
					<th class="col-date">Published</th>
					<th class="col-actions">Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each filteredLessons as lesson (lesson.id)}
					{@const Icon = getTypeIcon(lesson.type)}
					<tr>
						<td class="col-title">
							<div class="lesson-cell">
								<div class="lesson-thumb">
									{#if lesson.thumbnailUrl}
										<img src={lesson.thumbnailUrl} alt="" />
									{:else}
										<div class="thumb-placeholder">
											<Icon size={20} />
										</div>
									{/if}
								</div>
								<div class="lesson-info">
									<span class="lesson-title">{lesson.title}</span>
									<span class="lesson-meta">
										{lesson.type} {lesson.duration ? `• ${lesson.duration}` : ''}
									</span>
								</div>
							</div>
						</td>
						<td class="col-rooms">
							<div class="room-tags">
								{#each lesson.tradingRooms?.slice(0, 2) || [] as room}
									<span class="room-tag">{room.shortName || room.name}</span>
								{/each}
								{#if (lesson.tradingRooms?.length || 0) > 2}
									<span class="room-more">+{(lesson.tradingRooms?.length || 0) - 2}</span>
								{/if}
							</div>
						</td>
						<td class="col-trainer">
							{#if lesson.trainer}
								<div class="trainer-cell">
									{#if lesson.trainer.thumbnailUrl}
										<img src={lesson.trainer.thumbnailUrl} alt="" class="trainer-avatar" />
									{/if}
									<span>{lesson.trainer.name}</span>
								</div>
							{:else}
								<span class="no-data">-</span>
							{/if}
						</td>
						<td class="col-status">
							<span class="status-badge status-{lesson.status}">
								{lesson.status}
							</span>
						</td>
						<td class="col-date">
							{lesson.publishedAt ? formatDate(lesson.publishedAt) : '-'}
						</td>
						<td class="col-actions">
							<div class="actions-cell">
								<a
									href="/admin/learning-center/{lesson.id}/edit"
									class="action-btn"
									title="Edit"
								>
									<IconEdit size={16} />
								</a>
								<a
									href="/dashboard/{lesson.tradingRooms?.[0]?.slug || 'day-trading-room'}/learning-center/{lesson.slug}"
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
										onclick={() => toggleDropdown(lesson.id)}
									>
										<IconDots size={16} />
									</button>
									{#if activeDropdown === lesson.id}
										<div class="dropdown-menu">
											<button onclick={() => handleDuplicate(lesson)}>
												<IconCopy size={14} />
												Duplicate
											</button>
											<button
												class="danger"
												onclick={() => handleDeleteClick(lesson.id)}
											>
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
						<td colspan="6" class="empty-state">
							<p>No lessons found</p>
							{#if searchQuery || selectedRoom || selectedTrainer || selectedStatus}
								<button type="button" onclick={clearFilters}>Clear filters</button>
							{:else}
								<a href="/admin/learning-center/create">Create your first lesson</a>
							{/if}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>

<!-- Delete Modal -->
{#if showDeleteModal}
	<div 
		class="modal-overlay" 
		onclick={() => showDeleteModal = false}
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
			<h3>Delete Lesson?</h3>
			<p>This action cannot be undone. The lesson will be permanently deleted.</p>
			<div class="modal-actions">
				<button type="button" class="btn-cancel" onclick={() => showDeleteModal = false}>
					Cancel
				</button>
				<button type="button" class="btn-delete" onclick={handleConfirmDelete}>
					Delete
				</button>
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

	.btn-primary,
	.btn-secondary {
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
	}

	.btn-primary {
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	.btn-secondary {
		background: #334155;
		color: white;
	}

	.btn-secondary:hover {
		background: #475569;
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

	.col-title {
		width: 35%;
	}

	.col-rooms {
		width: 20%;
	}

	.col-trainer {
		width: 15%;
	}

	.col-status {
		width: 10%;
	}

	.col-date {
		width: 10%;
	}

	.col-actions {
		width: 10%;
	}

	/* Lesson cell */
	.lesson-cell {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.lesson-thumb {
		width: 56px;
		height: 32px;
		border-radius: 4px;
		overflow: hidden;
		background: #334155;
		flex-shrink: 0;
	}

	.lesson-thumb img {
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

	.lesson-info {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.lesson-title {
		font-weight: 500;
		color: white;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.lesson-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Room tags */
	.room-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
	}

	.room-tag {
		padding: 2px 8px;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #f97316;
	}

	.room-more {
		padding: 2px 6px;
		background: #334155;
		border-radius: 4px;
		font-size: 0.7rem;
		color: #64748b;
	}

	/* Trainer cell */
	.trainer-cell {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.trainer-avatar {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		object-fit: cover;
	}

	.no-data {
		color: #475569;
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

	.empty-state button,
	.empty-state a {
		color: #f97316;
		background: none;
		border: none;
		cursor: pointer;
		text-decoration: none;
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

	.modal h3 {
		margin: 0 0 12px;
		font-size: 1.125rem;
		color: white;
	}

	.modal p {
		margin: 0 0 24px;
		font-size: 0.9rem;
		color: #94a3b8;
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

		.col-rooms,
		.col-date {
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

		.search-box,
		.filter-group {
			width: 100%;
			max-width: none;
		}

		.filter-group {
			flex-wrap: wrap;
		}

		.filter-group select {
			flex: 1;
			min-width: 120px;
		}
	}
</style>

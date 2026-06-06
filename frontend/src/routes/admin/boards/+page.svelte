<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { boardsAPI } from '$lib/api/boards';
	import { logger } from '$lib/utils/logger';
	import type { Board, Folder, Activity, Task } from '$lib/boards/types';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import {
		IconLayoutKanban,
		IconPlus,
		IconFolder,
		IconStar,
		IconStarFilled,
		IconArchive,
		IconSearch,
		IconClock,
		IconChecks,
		IconAlertTriangle,
		IconCalendar,
		IconChartBar,
		IconSettings,
		IconUpload,
		IconTemplate,
		IconUsers,
		IconList,
		IconLayoutGrid,
		IconRefresh,
		IconTrash,
		IconCopy,
		IconEye
	} from '$lib/icons';

	// State
	let boards = $state<Board[]>([]);
	let folders = $state<Folder[]>([]);
	let recentActivity = $state<Activity[]>([]);
	let myTasks = $state<Task[]>([]);
	let overdueTasks = $state<Task[]>([]);
	let tasksDueToday = $state<Task[]>([]);
	let loading = $state(true);
	let showArchiveModal = $state(false);
	let showDeleteModal = $state(false);
	let pendingBoard = $state<Board | null>(null);
	let searchQuery = $state('');
	let viewMode = $state<'grid' | 'list'>('grid');
	let showArchived = $state(false);
	let selectedFolder = $state<string | null>(null);
	let showCreateModal = $state(false);
	let showFolderModal = $state(false);

	// Stats
	let stats = $state({
		total_boards: 0,
		total_tasks: 0,
		completed_tasks: 0,
		overdue_tasks: 0,
		my_tasks: 0,
		tasks_due_today: 0,
		tasks_due_this_week: 0
	});

	// New board form
	let newBoard = $state({
		title: '',
		description: '',
		type: 'kanban' as const,
		folder_id: undefined as string | undefined
	});

	// New folder form
	let newFolder = $state({
		title: '',
		color: '#3b82f6'
	});

	// Filtered boards
	let filteredBoards = $derived.by(() => {
		let result = boards;

		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(b) => b.title.toLowerCase().includes(query) || b.description?.toLowerCase().includes(query)
			);
		}

		if (!showArchived) {
			result = result.filter((b) => !b.is_archived);
		}

		if (selectedFolder) {
			result = result.filter((b) => b.folder_id === selectedFolder);
		}

		return result;
	});

	// Favorite boards
	let favoriteBoards = $derived.by(() => boards.filter((b) => b.is_favorite && !b.is_archived));

	// FIX-2026-04-26 (P3-7): defense-in-depth — `background_color` is user-controlled
	// and rendered into dynamic style values. If the backend
	// ever skips validation, an admin could inject CSS via something like
	// `red; }/* … */`. Validate as a strict 6-digit hex on read; fall back to brand
	// color otherwise.
	const HEX_RE = /^#[0-9a-fA-F]{6}$/;
	function safeColor(value: string | null | undefined, fallback = '#E6B800'): string {
		return typeof value === 'string' && HEX_RE.test(value) ? value : fallback;
	}

	// FIX-2026-04-26 (P2-1): Was `$effect(() => { if (browser) loadData(); })` — the
	// exact pattern that produced write-while-reading cascades on CRM/campaigns and
	// was systematically replaced with onMount in commit 34a0bd070. Same fix here.
	onMount(() => {
		void loadData();
	});

	async function loadData() {
		loading = true;
		try {
			const [boardsRes, foldersRes, statsRes, myTasksRes, overdueRes, todayRes] = await Promise.all(
				[
					boardsAPI.getBoards(),
					boardsAPI.getFolders(),
					boardsAPI.getDashboardStats(),
					boardsAPI.getMyTasks({ per_page: 5 }),
					boardsAPI.getOverdueTasks(),
					boardsAPI.getTasksDueToday()
				]
			);

			boards = boardsRes.data;
			folders = foldersRes;
			stats = statsRes;
			recentActivity = statsRes.recent_activity || [];
			myTasks = myTasksRes.data;
			overdueTasks = overdueRes;
			tasksDueToday = todayRes;
		} catch (error) {
			logger.error('[admin/boards] Failed to load boards data', { error });
		} finally {
			loading = false;
		}
	}

	async function createBoard() {
		if (!newBoard.title.trim()) return;

		try {
			const board = await boardsAPI.createBoard(newBoard);
			boards = [board, ...boards];
			showCreateModal = false;
			newBoard = { title: '', description: '', type: 'kanban', folder_id: undefined };
			goto(`/admin/boards/${board.id}`);
		} catch (error) {
			logger.error('[admin/boards] Failed to create board', { error });
		}
	}

	async function createFolder() {
		if (!newFolder.title.trim()) return;

		try {
			const folder = await boardsAPI.createFolder(newFolder);
			folders = [...folders, folder];
			showFolderModal = false;
			newFolder = { title: '', color: '#3b82f6' };
		} catch (error) {
			logger.error('[admin/boards] Failed to create folder', { error });
		}
	}

	async function toggleFavorite(board: Board) {
		try {
			const updated = await boardsAPI.toggleFavorite(board.id);
			boards = boards.map((b) => (b.id === board.id ? updated : b));
		} catch (error) {
			logger.error('[admin/boards] Failed to toggle favorite', { error });
		}
	}

	function archiveBoard(board: Board) {
		pendingBoard = board;
		showArchiveModal = true;
	}

	async function confirmArchive() {
		if (!pendingBoard) return;
		showArchiveModal = false;
		const board = pendingBoard;
		pendingBoard = null;
		try {
			const updated = await boardsAPI.archiveBoard(board.id);
			boards = boards.map((b) => (b.id === board.id ? updated : b));
		} catch (error) {
			logger.error('[admin/boards] Failed to archive board', { error });
		}
	}

	function deleteBoard(board: Board) {
		pendingBoard = board;
		showDeleteModal = true;
	}

	async function confirmDelete() {
		if (!pendingBoard) return;
		showDeleteModal = false;
		const board = pendingBoard;
		pendingBoard = null;
		try {
			await boardsAPI.deleteBoard(board.id);
			boards = boards.filter((b) => b.id !== board.id);
		} catch (error) {
			logger.error('[admin/boards] Failed to delete board', { error });
		}
	}

	async function duplicateBoard(board: Board) {
		try {
			const duplicate = await boardsAPI.duplicateBoard(board.id);
			boards = [duplicate, ...boards];
		} catch (error) {
			logger.error('[admin/boards] Failed to duplicate board', { error });
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

<svelte:head>
	<title>Project Boards | Admin</title>
</svelte:head>

<div class="admin-boards">
	<!-- Animated Background -->
	<div class="background-effects">
		<div class="background-blob background-blob-1"></div>
		<div class="background-blob background-blob-2"></div>
		<div class="background-blob background-blob-3"></div>
	</div>

	<!-- Header -->
	<div class="boards-header">
		<div class="boards-container boards-header-inner">
			<div class="boards-header-row">
				<div class="boards-title-group">
					<div class="boards-title-icon">
						<IconLayoutKanban class="title-icon" />
					</div>
					<div>
						<h1 class="boards-title">Project Boards</h1>
						<p class="boards-subtitle">Manage your projects and tasks</p>
					</div>
				</div>

				<div class="boards-actions">
					<a href="/admin/boards/time-tracking" class="boards-nav-link">
						<IconClock class="small-icon" />
						Time Tracking
					</a>
					<a href="/admin/boards/reports" class="boards-nav-link">
						<IconChartBar class="small-icon" />
						Reports
					</a>
					<a href="/admin/boards/settings" class="boards-nav-link">
						<IconSettings class="small-icon" />
						Settings
					</a>
					<button onclick={() => (showCreateModal = true)} class="primary-button">
						<IconPlus class="small-icon" />
						New Board
					</button>
				</div>
			</div>
		</div>
	</div>

	<div class="boards-container boards-content">
		<!-- Stats Cards -->
		<div class="stats-grid">
			<div class="stat-card">
				<div class="stat-value">{stats.total_boards}</div>
				<div class="stat-label">Total Boards</div>
			</div>
			<div class="stat-card">
				<div class="stat-value">{stats.total_tasks}</div>
				<div class="stat-label">Total Tasks</div>
			</div>
			<div class="stat-card">
				<div class="stat-value stat-value--green">{stats.completed_tasks}</div>
				<div class="stat-label">Completed</div>
			</div>
			<div class="stat-card">
				<div class="stat-value stat-value--red">{stats.overdue_tasks}</div>
				<div class="stat-label">Overdue</div>
			</div>
			<div class="stat-card">
				<div class="stat-value stat-value--blue">{stats.my_tasks}</div>
				<div class="stat-label">My Tasks</div>
			</div>
			<div class="stat-card">
				<div class="stat-value stat-value--orange">{stats.tasks_due_today}</div>
				<div class="stat-label">Due Today</div>
			</div>
			<div class="stat-card">
				<div class="stat-value stat-value--brand">{stats.tasks_due_this_week}</div>
				<div class="stat-label">Due This Week</div>
			</div>
		</div>

		<div class="boards-layout">
			<!-- Sidebar -->
			<div class="boards-sidebar">
				<!-- Quick Actions -->
				<div class="sidebar-card">
					<h3 class="sidebar-title">Quick Actions</h3>
					<div class="sidebar-list">
						<button onclick={() => (showCreateModal = true)} class="sidebar-action">
							<IconPlus class="small-icon" />
							Create Board
						</button>
						<button onclick={() => (showFolderModal = true)} class="sidebar-action">
							<IconFolder class="small-icon" />
							Create Folder
						</button>
						<a href="/admin/boards/import" class="sidebar-action">
							<IconUpload class="small-icon" />
							Import from Trello/Asana
						</a>
						<a href="/admin/boards/templates" class="sidebar-action">
							<IconTemplate class="small-icon" />
							Browse Templates
						</a>
					</div>
				</div>

				<!-- Folders -->
				<div class="sidebar-card">
					<h3 class="sidebar-title">Folders</h3>
					<div class="folder-filter-list">
						<button
							onclick={() => (selectedFolder = null)}
							class={['folder-filter', { active: selectedFolder === null }]}
						>
							<IconLayoutKanban class="small-icon" />
							All Boards
							<span class="count-badge">{boards.filter((b) => !b.is_archived).length}</span>
						</button>
						{#each folders as folder (folder.id)}
							<button
								onclick={() => (selectedFolder = folder.id)}
								class={['folder-filter', { active: selectedFolder === folder.id }]}
							>
								<span class="folder-color-icon" style:--folder-color={safeColor(folder.color)}>
									<IconFolder class="small-icon" />
								</span>
								{folder.title}
								<span class="count-badge">{folder.board_count || 0}</span>
							</button>
						{/each}
					</div>
				</div>

				<!-- My Tasks Due Today -->
				{#if tasksDueToday.length > 0}
					<div class="sidebar-card">
						<h3 class="sidebar-title sidebar-title--icon">
							<IconCalendar class="small-icon status-orange" />
							Due Today
						</h3>
						<div class="sidebar-list">
							{#each tasksDueToday.slice(0, 5) as task (task.id)}
								<a href="/admin/boards/{task.board_id}?task={task.id}" class="task-link">
									<div class="task-title">{task.title}</div>
									<div class="task-meta">
										{task.due_date ? formatDate(task.due_date) : ''}
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Overdue Tasks -->
				{#if overdueTasks.length > 0}
					<div class="sidebar-card sidebar-card--danger">
						<h3 class="sidebar-title sidebar-title--danger sidebar-title--icon">
							<IconAlertTriangle class="small-icon" />
							Overdue Tasks
						</h3>
						<div class="sidebar-list">
							{#each overdueTasks.slice(0, 5) as task (task.id)}
								<a
									href="/admin/boards/{task.board_id}?task={task.id}"
									class="task-link task-link--danger"
								>
									<div class="task-title">{task.title}</div>
									<div class="task-meta task-meta--danger">
										{task.due_date ? formatDate(task.due_date) : ''}
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- My Tasks -->
				{#if myTasks.length > 0}
					<div class="sidebar-card">
						<h3 class="sidebar-title sidebar-title--icon">
							<IconChecks class="small-icon status-blue" />
							My Tasks
						</h3>
						<div class="sidebar-list">
							{#each myTasks.slice(0, 5) as task (task.id)}
								<a href="/admin/boards/{task.board_id}?task={task.id}" class="task-link">
									<div class="task-title">{task.title}</div>
									<div class="task-meta">
										{task.due_date ? formatDate(task.due_date) : 'No due date'}
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Recent Activity -->
				{#if recentActivity.length > 0}
					<div class="sidebar-card">
						<h3 class="sidebar-title sidebar-title--icon">
							<IconClock class="small-icon muted-icon" />
							Recent Activity
						</h3>
						<div class="sidebar-list">
							{#each recentActivity.slice(0, 5) as activity (activity.id)}
								<div class="activity-item">
									<span class="activity-user">{activity.user?.name || 'Someone'}</span>
									{activity.description}
									<span class="activity-time">
										{activity.created_at ? formatDate(activity.created_at) : ''}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>

			<!-- Main Content -->
			<div class="boards-main">
				<!-- Search and Filters -->
				<div class="boards-toolbar">
					<div class="search-field">
						<IconSearch class="search-icon" />
						<input
							id="boards-searchquery"
							name="boards-searchquery"
							type="text"
							placeholder="Search boards..."
							bind:value={searchQuery}
							class="search-input"
						/>
					</div>
					<div class="view-toggle">
						<button
							onclick={() => (viewMode = 'grid')}
							class={['view-button', { active: viewMode === 'grid' }]}
							aria-label="Grid view"
						>
							<IconLayoutGrid class="toolbar-icon" />
						</button>
						<button
							onclick={() => (viewMode = 'list')}
							class={['view-button', { active: viewMode === 'list' }]}
							aria-label="List view"
						>
							<IconList class="toolbar-icon" />
						</button>
					</div>
					<label class="archive-toggle">
						<input
							id="boards-showarchived"
							name="boards-showarchived"
							type="checkbox"
							bind:checked={showArchived}
							class="archive-checkbox"
						/>
						Show archived
					</label>
					<button onclick={loadData} class="refresh-button" aria-label="Refresh boards">
						<IconRefresh class="toolbar-icon" />
					</button>
				</div>

				<!-- Favorite Boards -->
				{#if favoriteBoards.length > 0}
					<div class="board-section">
						<h3 class="section-heading section-heading--icon">
							<IconStarFilled class="small-icon status-brand" />
							Favorites
						</h3>
						<div class="board-grid">
							{#each favoriteBoards as board (board.id)}
								<a
									href="/admin/boards/{board.id}"
									class="board-card board-card--favorite"
									style:border-left={`4px solid ${safeColor(board.background_color)}`}
								>
									<div class="board-card-header">
										<h4 class="board-card-title">{board.title}</h4>
										<button
											onclick={(e: MouseEvent) => {
												e.preventDefault();
												e.stopPropagation();
												toggleFavorite(board);
											}}
											class="favorite-button active"
											aria-label="Remove favorite"
										>
											<IconStarFilled class="toolbar-icon" />
										</button>
									</div>
									{#if board.description}
										<p class="board-description">
											{board.description}
										</p>
									{/if}
									<div class="board-meta">
										<span>{board.task_count || 0} tasks</span>
										<span>{board.member_count || 1} members</span>
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- All Boards -->
				<div>
					<h3 class="section-heading">
						{selectedFolder ? folders.find((f) => f.id === selectedFolder)?.title : 'All Boards'}
					</h3>

					{#if loading}
						<div class="loading-state">
							<div class="spinner"></div>
						</div>
					{:else if filteredBoards.length === 0}
						<div class="empty-state">
							<IconLayoutKanban class="empty-icon" />
							<h3 class="empty-title">No boards yet</h3>
							<p class="empty-copy">Create your first board to start managing projects</p>
							<button onclick={() => (showCreateModal = true)} class="primary-button">
								Create Board
							</button>
						</div>
					{:else if viewMode === 'grid'}
						<div class="board-grid">
							{#each filteredBoards as board (board.id)}
								<div class="board-card">
									<a
										href="/admin/boards/{board.id}"
										class="board-card-link"
										style:border-left={`4px solid ${safeColor(board.background_color)}`}
									>
										<div class="board-card-header">
											<h4 class="board-card-title">
												{board.title}
												{#if board.is_archived}
													<span class="archive-badge">Archived</span>
												{/if}
											</h4>
											<button
												onclick={(e: MouseEvent) => {
													e.preventDefault();
													e.stopPropagation();
													toggleFavorite(board);
												}}
												class={['favorite-button', { active: board.is_favorite }]}
												aria-label={board.is_favorite ? 'Remove favorite' : 'Add favorite'}
											>
												{#if board.is_favorite}
													<IconStarFilled class="toolbar-icon" />
												{:else}
													<IconStar class="toolbar-icon" />
												{/if}
											</button>
										</div>
										{#if board.description}
											<p class="board-description">
												{board.description}
											</p>
										{/if}
										<div class="board-meta">
											<span class="board-meta-item">
												<IconChecks class="meta-icon" />
												{board.completed_task_count || 0}/{board.task_count || 0}
											</span>
											<span class="board-meta-item">
												<IconUsers class="meta-icon" />
												{board.member_count || 1}
											</span>
										</div>
									</a>
									<div class="board-card-actions">
										<button
											onclick={() => goto(`/admin/boards/${board.id}`)}
											class="icon-button"
											title="Open"
										>
											<IconEye class="small-icon" />
										</button>
										<button
											onclick={() => duplicateBoard(board)}
											class="icon-button"
											title="Duplicate"
										>
											<IconCopy class="small-icon" />
										</button>
										<button onclick={() => archiveBoard(board)} class="icon-button" title="Archive">
											<IconArchive class="small-icon" />
										</button>
										<button
											onclick={() => deleteBoard(board)}
											class="icon-button icon-button--danger"
											title="Delete"
										>
											<IconTrash class="small-icon" />
										</button>
									</div>
								</div>
							{/each}
						</div>
					{:else}
						<div class="board-table-shell">
							<table class="board-table">
								<thead>
									<tr>
										<th>Board</th>
										<th>Tasks</th>
										<th>Members</th>
										<th>Updated</th>
										<th class="align-right">Actions</th>
									</tr>
								</thead>
								<tbody>
									{#each filteredBoards as board (board.id)}
										<tr>
											<td>
												<a href="/admin/boards/{board.id}" class="table-board-link">
													<div
														class="table-color-strip"
														style:background-color={safeColor(board.background_color)}
													></div>
													<div>
														<div class="table-board-title">
															{board.title}
															{#if board.is_favorite}
																<IconStarFilled class="small-icon status-brand" />
															{/if}
															{#if board.is_archived}
																<span class="archive-badge">Archived</span>
															{/if}
														</div>
														{#if board.description}
															<div class="table-description">
																{board.description}
															</div>
														{/if}
													</div>
												</a>
											</td>
											<td>
												{board.completed_task_count || 0}/{board.task_count || 0}
											</td>
											<td>
												{board.member_count || 1}
											</td>
											<td class="muted-cell">
												{formatDate(board.updated_at)}
											</td>
											<td>
												<div class="table-actions">
													<button
														onclick={() => toggleFavorite(board)}
														class="icon-button"
														aria-label={board.is_favorite ? 'Remove favorite' : 'Add favorite'}
													>
														{#if board.is_favorite}
															<IconStarFilled class="small-icon status-brand" />
														{:else}
															<IconStar class="small-icon" />
														{/if}
													</button>
													<button
														onclick={() => duplicateBoard(board)}
														class="icon-button"
														aria-label="Duplicate board"
													>
														<IconCopy class="small-icon" />
													</button>
													<button
														onclick={() => deleteBoard(board)}
														class="icon-button icon-button--danger"
														aria-label="Delete board"
													>
														<IconTrash class="small-icon" />
													</button>
												</div>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					{/if}
				</div>
			</div>
		</div>
	</div>

	<!-- Create Board Modal -->
	{#if showCreateModal}
		<div class="modal-backdrop">
			<div class="modal-panel">
				<div class="modal-header">
					<h2 class="modal-title">Create New Board</h2>
				</div>
				<div class="modal-body">
					<div class="form-field">
						<label class="form-label" for="board-title">Board Title</label>
						<input
							type="text"
							id="board-title"
							name="board-title"
							bind:value={newBoard.title}
							placeholder="Enter board title..."
							class="form-control"
						/>
					</div>
					<div class="form-field">
						<label class="form-label" for="board-description">Description (optional)</label>
						<textarea
							id="board-description"
							bind:value={newBoard.description}
							placeholder="Enter description..."
							rows="3"
							class="form-control"
						></textarea>
					</div>
					<div class="form-field">
						<label class="form-label" for="board-type">Board Type</label>
						<select id="board-type" bind:value={newBoard.type} class="form-control">
							<option value="kanban">Kanban Board</option>
							<option value="list">List View</option>
							<option value="calendar">Calendar View</option>
							<option value="table">Table View</option>
						</select>
					</div>
					{#if folders.length > 0}
						<div class="form-field">
							<label class="form-label" for="board-folder">Folder (optional)</label>
							<select id="board-folder" bind:value={newBoard.folder_id} class="form-control">
								<!-- FIX-2026-04-26 (P3-3): <option value={null}> silently coerces to "" — use explicit empty string. -->
								<option value="">No folder</option>
								{#each folders as folder (folder.id)}
									<option value={folder.id}>{folder.title}</option>
								{/each}
							</select>
						</div>
					{/if}
				</div>
				<div class="modal-footer">
					<button onclick={() => (showCreateModal = false)} class="secondary-button">
						Cancel
					</button>
					<button onclick={createBoard} disabled={!newBoard.title.trim()} class="primary-button">
						Create Board
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Create Folder Modal -->
	{#if showFolderModal}
		<div class="modal-backdrop">
			<div class="modal-panel">
				<div class="modal-header">
					<h2 class="modal-title">Create New Folder</h2>
				</div>
				<div class="modal-body">
					<div class="form-field">
						<label class="form-label" for="folder-name">Folder Name</label>
						<input
							type="text"
							id="folder-name"
							name="folder-name"
							bind:value={newFolder.title}
							placeholder="Enter folder name..."
							class="form-control"
						/>
					</div>
					<div class="form-field">
						<label class="form-label" for="folder-color">Color</label>
						<input
							type="color"
							id="folder-color"
							name="folder-color"
							bind:value={newFolder.color}
							class="color-input"
						/>
					</div>
				</div>
				<div class="modal-footer">
					<button onclick={() => (showFolderModal = false)} class="secondary-button">
						Cancel
					</button>
					<button onclick={createFolder} disabled={!newFolder.title.trim()} class="primary-button">
						Create Folder
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showArchiveModal}
	title="Archive Board"
	message={pendingBoard ? `Archive "${pendingBoard.title}"?` : ''}
	confirmText="Archive"
	variant="warning"
	onConfirm={confirmArchive}
	onCancel={() => {
		showArchiveModal = false;
		pendingBoard = null;
	}}
/>

<ConfirmationModal
	isOpen={showDeleteModal}
	title="Delete Board"
	message={pendingBoard ? `Delete "${pendingBoard.title}"? This cannot be undone.` : ''}
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDelete}
	onCancel={() => {
		showDeleteModal = false;
		pendingBoard = null;
	}}
/>

<style>
	.admin-boards {
		position: relative;
		min-height: 100%;
		background: #f9fafb;
		color: #111827;
	}

	:global(.dark) .admin-boards {
		background: #111827;
		color: #f9fafb;
	}

	.background-effects {
		pointer-events: none;
		position: fixed;
		inset: 0;
		overflow: hidden;
		z-index: 0;
	}

	.background-blob {
		position: absolute;
		width: 18rem;
		height: 18rem;
		border-radius: 999px;
		filter: blur(80px);
		opacity: 0.18;
	}

	.background-blob-1 {
		top: 5rem;
		left: 8%;
		background: #e6b800;
	}

	.background-blob-2 {
		top: 18rem;
		right: 10%;
		background: #3b82f6;
	}

	.background-blob-3 {
		bottom: 4rem;
		left: 38%;
		background: #10b981;
	}

	.boards-header,
	.boards-content {
		position: relative;
		z-index: 1;
	}

	.boards-header {
		border-bottom: 1px solid #e5e7eb;
		background: #ffffff;
	}

	:global(.dark) .boards-header {
		border-color: #374151;
		background: #1f2937;
	}

	.boards-container {
		width: min(100% - 2rem, 80rem);
		margin-inline: auto;
	}

	.boards-header-inner {
		padding-block: 1.5rem;
	}

	.boards-header-row,
	.boards-title-group,
	.boards-actions,
	.boards-nav-link,
	.primary-button,
	.secondary-button,
	.sidebar-action,
	.folder-filter,
	.sidebar-title--icon,
	.section-heading--icon,
	.boards-toolbar,
	.view-toggle,
	.archive-toggle,
	.board-card-header,
	.board-meta,
	.board-meta-item,
	.board-card-actions,
	.table-board-link,
	.table-board-title,
	.table-actions,
	.modal-footer {
		display: flex;
		align-items: center;
	}

	.boards-header-row {
		justify-content: space-between;
		gap: 1rem;
	}

	.boards-title-group {
		gap: 0.75rem;
	}

	.boards-title-icon {
		padding: 0.5rem;
		border-radius: 0.5rem;
		background: rgba(230, 184, 0, 0.12);
		color: #e6b800;
	}

	:global(.dark) .boards-title-icon {
		background: rgba(230, 184, 0, 0.2);
		color: #ffd11a;
	}

	.boards-title {
		margin: 0;
		font-size: 1.5rem;
		line-height: 2rem;
		font-weight: 700;
	}

	.boards-subtitle {
		margin: 0;
		color: #6b7280;
		font-size: 0.875rem;
	}

	:global(.dark) .boards-subtitle {
		color: #9ca3af;
	}

	.boards-actions {
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.boards-nav-link,
	.sidebar-action,
	.folder-filter,
	.secondary-button {
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #374151;
		font: inherit;
		text-decoration: none;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
	}

	.boards-nav-link {
		gap: 0.5rem;
		padding: 0.5rem 1rem;
	}

	.boards-nav-link:hover,
	.sidebar-action:hover,
	.folder-filter:hover,
	.secondary-button:hover,
	.task-link:hover {
		background: #f3f4f6;
	}

	:global(.dark) .boards-nav-link,
	:global(.dark) .sidebar-action,
	:global(.dark) .folder-filter,
	:global(.dark) .secondary-button {
		color: #d1d5db;
	}

	:global(.dark) .boards-nav-link:hover,
	:global(.dark) .sidebar-action:hover,
	:global(.dark) .folder-filter:hover,
	:global(.dark) .secondary-button:hover,
	:global(.dark) .task-link:hover {
		background: #374151;
	}

	.primary-button {
		justify-content: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: #e6b800;
		color: #0d1117;
		font: inherit;
		font-weight: 600;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			opacity 0.2s ease;
	}

	.primary-button:hover:not(:disabled) {
		background: #b38f00;
	}

	.primary-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.secondary-button {
		padding: 0.5rem 1rem;
	}

	.boards-content {
		padding-block: 2rem;
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card,
	.sidebar-card,
	.board-card,
	.board-table-shell,
	.empty-state,
	.modal-panel {
		border: 1px solid #e5e7eb;
		background: #ffffff;
	}

	:global(.dark) .stat-card,
	:global(.dark) .sidebar-card,
	:global(.dark) .board-card,
	:global(.dark) .board-table-shell,
	:global(.dark) .empty-state,
	:global(.dark) .modal-panel {
		border-color: #374151;
		background: #1f2937;
	}

	.stat-card,
	.sidebar-card,
	.board-card,
	.board-table-shell,
	.empty-state {
		border-radius: 0.75rem;
	}

	.stat-card,
	.sidebar-card {
		padding: 1rem;
	}

	.stat-value {
		font-size: 1.5rem;
		line-height: 2rem;
		font-weight: 700;
		color: #111827;
	}

	:global(.dark) .stat-value {
		color: #ffffff;
	}

	.stat-value--green {
		color: #16a34a;
	}

	.stat-value--red {
		color: #dc2626;
	}

	.stat-value--blue {
		color: #2563eb;
	}

	.stat-value--orange {
		color: #ea580c;
	}

	.stat-value--brand {
		color: #e6b800;
	}

	.stat-label,
	.task-meta,
	.activity-time,
	.board-description,
	.board-meta,
	.table-description,
	.empty-copy,
	.muted-cell {
		color: #6b7280;
	}

	:global(.dark) .stat-label,
	:global(.dark) .task-meta,
	:global(.dark) .board-description,
	:global(.dark) .board-meta,
	:global(.dark) .table-description,
	:global(.dark) .empty-copy,
	:global(.dark) .muted-cell {
		color: #9ca3af;
	}

	.stat-label,
	.board-description,
	.task-title,
	.task-meta,
	.table-description {
		font-size: 0.875rem;
	}

	.boards-layout {
		display: grid;
		grid-template-columns: 1fr;
		gap: 2rem;
	}

	.boards-sidebar {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.sidebar-card--danger {
		border-color: #fecaca;
	}

	:global(.dark) .sidebar-card--danger {
		border-color: rgba(127, 29, 29, 0.5);
	}

	.sidebar-title {
		margin: 0 0 1rem;
		font-size: 1rem;
		font-weight: 600;
		color: #111827;
	}

	:global(.dark) .sidebar-title {
		color: #ffffff;
	}

	.sidebar-title--icon,
	.section-heading--icon {
		gap: 0.5rem;
	}

	.sidebar-title--danger {
		color: #dc2626;
	}

	:global(.dark) .sidebar-title--danger {
		color: #f87171;
	}

	.sidebar-list,
	.folder-filter-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.folder-filter-list {
		gap: 0.25rem;
	}

	.sidebar-action,
	.folder-filter {
		width: 100%;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		text-align: left;
	}

	.folder-color-icon {
		display: inline-flex;
		color: var(--folder-color);
	}

	.sidebar-action {
		padding-inline: 1rem;
	}

	.folder-filter.active {
		background: rgba(230, 184, 0, 0.12);
		color: #e6b800;
	}

	:global(.dark) .folder-filter.active {
		background: rgba(230, 184, 0, 0.2);
		color: #ffd11a;
	}

	.count-badge {
		margin-left: auto;
		padding: 0.125rem 0.5rem;
		border-radius: 999px;
		background: #e5e7eb;
		font-size: 0.75rem;
	}

	:global(.dark) .count-badge {
		background: #374151;
	}

	.task-link {
		display: block;
		padding: 0.5rem;
		border-radius: 0.5rem;
		text-decoration: none;
		transition: background-color 0.2s ease;
	}

	.task-link--danger:hover {
		background: #fef2f2;
	}

	:global(.dark) .task-link--danger:hover {
		background: rgba(127, 29, 29, 0.2);
	}

	.task-title {
		overflow: hidden;
		color: #111827;
		font-weight: 500;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	:global(.dark) .task-title {
		color: #ffffff;
	}

	.task-meta,
	.activity-item,
	.activity-time {
		font-size: 0.75rem;
	}

	.task-meta--danger {
		color: #ef4444;
	}

	.activity-item {
		color: #4b5563;
	}

	:global(.dark) .activity-item {
		color: #9ca3af;
	}

	.activity-user {
		font-weight: 500;
	}

	.boards-toolbar {
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.search-field {
		position: relative;
		flex: 1;
		min-width: 0;
	}

	.search-input,
	.form-control {
		width: 100%;
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		background: #ffffff;
		color: #111827;
		font: inherit;
	}

	.search-input {
		padding: 0.5rem 1rem 0.5rem 2.5rem;
	}

	.form-control {
		padding: 0.5rem 0.75rem;
	}

	:global(.dark) .search-input,
	:global(.dark) .form-control {
		border-color: #4b5563;
		background: #374151;
		color: #ffffff;
	}

	.search-input:focus,
	.form-control:focus {
		outline: 2px solid #e6b800;
		outline-offset: 2px;
	}

	.search-field :global(.search-icon) {
		position: absolute;
		top: 50%;
		left: 0.75rem;
		width: 1.25rem;
		height: 1.25rem;
		color: #9ca3af;
		transform: translateY(-50%);
	}

	.view-toggle {
		gap: 0.5rem;
	}

	.view-button,
	.refresh-button,
	.favorite-button,
	.icon-button {
		border: 0;
		background: transparent;
		color: #6b7280;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease,
			opacity 0.2s ease;
	}

	.view-button,
	.refresh-button {
		padding: 0.5rem;
		border-radius: 0.5rem;
	}

	.view-button:hover,
	.refresh-button:hover,
	.icon-button:hover {
		background: #f3f4f6;
	}

	:global(.dark) .view-button:hover,
	:global(.dark) .refresh-button:hover,
	:global(.dark) .icon-button:hover {
		background: #374151;
	}

	.view-button.active,
	.favorite-button.active {
		color: #e6b800;
	}

	.view-button.active {
		background: rgba(230, 184, 0, 0.12);
	}

	.archive-toggle {
		gap: 0.5rem;
		color: #4b5563;
		font-size: 0.875rem;
	}

	:global(.dark) .archive-toggle {
		color: #9ca3af;
	}

	.archive-checkbox {
		width: 1rem;
		height: 1rem;
		accent-color: #e6b800;
	}

	.section-heading {
		margin: 0 0 1rem;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 600;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	:global(.dark) .section-heading {
		color: #9ca3af;
	}

	.board-section {
		margin-bottom: 2rem;
	}

	.board-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1rem;
	}

	.board-card {
		overflow: hidden;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.board-card--favorite {
		display: block;
		padding: 1rem;
		text-decoration: none;
	}

	.board-card:hover {
		border-color: #e6b800;
		box-shadow: 0 10px 24px rgba(17, 24, 39, 0.08);
	}

	:global(.dark) .board-card:hover {
		border-color: #ffd11a;
	}

	.board-card-link {
		display: block;
		padding: 1rem;
		color: inherit;
		text-decoration: none;
	}

	.board-card-header {
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.75rem;
	}

	.board-card-title,
	.table-board-title {
		color: #111827;
		font-weight: 600;
	}

	:global(.dark) .board-card-title,
	:global(.dark) .table-board-title {
		color: #ffffff;
	}

	.board-card:hover .board-card-title {
		color: #e6b800;
	}

	.board-description {
		display: -webkit-box;
		overflow: hidden;
		margin: 0 0 0.75rem;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
	}

	.board-meta {
		gap: 1rem;
		font-size: 0.75rem;
	}

	.board-meta-item {
		gap: 0.25rem;
	}

	.favorite-button {
		opacity: 0;
	}

	.board-card:hover .favorite-button,
	.favorite-button.active,
	.board-card--favorite .favorite-button {
		opacity: 1;
	}

	.board-card-actions {
		justify-content: flex-end;
		gap: 0.25rem;
		padding: 0.5rem 1rem;
		border-top: 1px solid #e5e7eb;
		background: #f9fafb;
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	:global(.dark) .board-card-actions {
		border-color: #374151;
		background: rgba(55, 65, 81, 0.5);
	}

	.board-card:hover .board-card-actions {
		opacity: 1;
	}

	.icon-button {
		padding: 0.375rem;
		border-radius: 0.25rem;
	}

	.icon-button:hover {
		color: #374151;
	}

	:global(.dark) .icon-button:hover {
		color: #e5e7eb;
	}

	.icon-button--danger {
		color: #ef4444;
	}

	.icon-button--danger:hover {
		background: #fee2e2;
		color: #b91c1c;
	}

	:global(.dark) .icon-button--danger:hover {
		background: rgba(127, 29, 29, 0.3);
		color: #f87171;
	}

	.archive-badge {
		margin-left: 0.5rem;
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
		background: #e5e7eb;
		color: #4b5563;
		font-size: 0.75rem;
		font-weight: 500;
	}

	:global(.dark) .archive-badge {
		background: #374151;
		color: #9ca3af;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		padding-block: 3rem;
	}

	.spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid rgba(230, 184, 0, 0.25);
		border-bottom-color: #e6b800;
		border-radius: 999px;
		animation: boards-spin 0.8s linear infinite;
	}

	.empty-state {
		padding: 3rem 1rem;
		text-align: center;
	}

	.empty-state :global(.empty-icon) {
		width: 3rem;
		height: 3rem;
		margin-inline: auto;
		margin-bottom: 1rem;
		color: #9ca3af;
	}

	.empty-title {
		margin: 0 0 0.5rem;
		font-size: 1.125rem;
		font-weight: 500;
	}

	.empty-copy {
		margin: 0 0 1rem;
	}

	.board-table-shell {
		overflow-x: auto;
	}

	.board-table {
		width: 100%;
		border-collapse: collapse;
	}

	.board-table thead {
		background: #f9fafb;
	}

	:global(.dark) .board-table thead {
		background: #374151;
	}

	.board-table th,
	.board-table td {
		padding: 0.75rem 1rem;
		text-align: left;
	}

	.board-table th {
		color: #6b7280;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	:global(.dark) .board-table th {
		color: #9ca3af;
	}

	.board-table td {
		color: #4b5563;
		font-size: 0.875rem;
	}

	:global(.dark) .board-table td {
		color: #d1d5db;
	}

	.board-table tbody tr {
		border-top: 1px solid #e5e7eb;
	}

	:global(.dark) .board-table tbody tr {
		border-color: #374151;
	}

	.board-table tbody tr:hover {
		background: #f9fafb;
	}

	:global(.dark) .board-table tbody tr:hover {
		background: rgba(55, 65, 81, 0.5);
	}

	.align-right {
		text-align: right;
	}

	.table-board-link {
		gap: 0.75rem;
		color: inherit;
		text-decoration: none;
	}

	.table-color-strip {
		width: 0.5rem;
		height: 2rem;
		border-radius: 0.25rem;
		flex-shrink: 0;
	}

	.table-board-title {
		gap: 0.5rem;
	}

	.table-description {
		max-width: 20rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.table-actions {
		justify-content: flex-end;
		gap: 0.25rem;
	}

	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.5);
	}

	.modal-panel {
		width: min(100%, 28rem);
		border-radius: 0.75rem;
		box-shadow: 0 20px 40px rgba(15, 23, 42, 0.25);
	}

	.modal-header,
	.modal-body,
	.modal-footer {
		padding: 1.5rem;
	}

	.modal-header {
		border-bottom: 1px solid #e5e7eb;
	}

	.modal-footer {
		justify-content: flex-end;
		gap: 0.75rem;
		border-top: 1px solid #e5e7eb;
	}

	:global(.dark) .modal-header,
	:global(.dark) .modal-footer {
		border-color: #374151;
	}

	.modal-title {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
	}

	.modal-body {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-field {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.form-label {
		color: #374151;
		font-size: 0.875rem;
		font-weight: 500;
	}

	:global(.dark) .form-label {
		color: #d1d5db;
	}

	.color-input {
		width: 100%;
		height: 2.5rem;
		border: 0;
		border-radius: 0.5rem;
		cursor: pointer;
	}

	.admin-boards :global(.status-brand) {
		color: #e6b800;
	}

	.admin-boards :global(.status-orange) {
		color: #f97316;
	}

	.admin-boards :global(.status-blue) {
		color: #3b82f6;
	}

	.admin-boards :global(.muted-icon) {
		color: #6b7280;
	}

	.admin-boards :global(.title-icon) {
		width: 1.5rem;
		height: 1.5rem;
	}

	.admin-boards :global(.small-icon) {
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
	}

	.admin-boards :global(.toolbar-icon) {
		width: 1.25rem;
		height: 1.25rem;
	}

	.admin-boards :global(.meta-icon) {
		width: 0.875rem;
		height: 0.875rem;
	}

	@media (min-width: 768px) {
		.stats-grid {
			grid-template-columns: repeat(4, minmax(0, 1fr));
		}

		.board-grid {
			grid-template-columns: repeat(2, minmax(0, 1fr));
		}
	}

	@media (min-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(7, minmax(0, 1fr));
		}

		.boards-layout {
			grid-template-columns: minmax(0, 1fr) minmax(0, 3fr);
		}
	}

	@media (min-width: 1280px) {
		.board-grid {
			grid-template-columns: repeat(3, minmax(0, 1fr));
		}
	}

	@media (max-width: 767px) {
		.boards-header-row,
		.boards-toolbar,
		.modal-footer {
			align-items: stretch;
			flex-direction: column;
		}

		.boards-actions,
		.view-toggle {
			width: 100%;
		}

		.boards-nav-link,
		.primary-button,
		.secondary-button,
		.refresh-button {
			width: 100%;
		}

		.favorite-button,
		.board-card-actions {
			opacity: 1;
		}
	}

	@keyframes boards-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

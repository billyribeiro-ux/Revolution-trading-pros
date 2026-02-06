<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { boardsAPI } from '$lib/api/boards';
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

	// Svelte 5: Initialize on mount
	$effect(() => {
		if (browser) loadData();
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
			console.error('Failed to load boards data:', error);
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
			console.error('Failed to create board:', error);
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
			console.error('Failed to create folder:', error);
		}
	}

	async function toggleFavorite(board: Board) {
		try {
			const updated = await boardsAPI.toggleFavorite(board.id);
			boards = boards.map((b) => (b.id === board.id ? updated : b));
		} catch (error) {
			console.error('Failed to toggle favorite:', error);
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
			console.error('Failed to archive board:', error);
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
			console.error('Failed to delete board:', error);
		}
	}

	async function duplicateBoard(board: Board) {
		try {
			const duplicate = await boardsAPI.duplicateBoard(board.id);
			boards = [duplicate, ...boards];
		} catch (error) {
			console.error('Failed to duplicate board:', error);
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

<div class="admin-boards bg-gray-50 dark:bg-gray-900">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-3">
					<div class="p-2 bg-[#E6B800]/10 dark:bg-[#E6B800]/20 rounded-lg">
						<IconLayoutKanban class="w-6 h-6 text-[#E6B800] dark:text-[#FFD11A]" />
					</div>
					<div>
						<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Project Boards</h1>
						<p class="text-sm text-gray-500 dark:text-gray-400">Manage your projects and tasks</p>
					</div>
				</div>

				<div class="flex items-center gap-3">
					<a
						href="/admin/boards/time-tracking"
						class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
					>
						<IconClock class="w-4 h-4" />
						Time Tracking
					</a>
					<a
						href="/admin/boards/reports"
						class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
					>
						<IconChartBar class="w-4 h-4" />
						Reports
					</a>
					<a
						href="/admin/boards/settings"
						class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
					>
						<IconSettings class="w-4 h-4" />
						Settings
					</a>
					<button
						onclick={() => (showCreateModal = true)}
						class="px-4 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg flex items-center gap-2"
					>
						<IconPlus class="w-4 h-4" />
						New Board
					</button>
				</div>
			</div>
		</div>
	</div>

	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<!-- Stats Cards -->
		<div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-7 gap-4 mb-8">
			<div
				class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
			>
				<div class="text-2xl font-bold text-gray-900 dark:text-white">{stats.total_boards}</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">Total Boards</div>
			</div>
			<div
				class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
			>
				<div class="text-2xl font-bold text-gray-900 dark:text-white">{stats.total_tasks}</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">Total Tasks</div>
			</div>
			<div
				class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
			>
				<div class="text-2xl font-bold text-green-600">{stats.completed_tasks}</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">Completed</div>
			</div>
			<div
				class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
			>
				<div class="text-2xl font-bold text-red-600">{stats.overdue_tasks}</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">Overdue</div>
			</div>
			<div
				class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
			>
				<div class="text-2xl font-bold text-blue-600">{stats.my_tasks}</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">My Tasks</div>
			</div>
			<div
				class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
			>
				<div class="text-2xl font-bold text-orange-600">{stats.tasks_due_today}</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">Due Today</div>
			</div>
			<div
				class="bg-white dark:bg-gray-800 rounded-xl p-4 border border-gray-200 dark:border-gray-700"
			>
				<div class="text-2xl font-bold text-[#E6B800]">{stats.tasks_due_this_week}</div>
				<div class="text-sm text-gray-500 dark:text-gray-400">Due This Week</div>
			</div>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
			<!-- Sidebar -->
			<div class="lg:col-span-1 space-y-6">
				<!-- Quick Actions -->
				<div
					class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4"
				>
					<h3 class="font-semibold text-gray-900 dark:text-white mb-4">Quick Actions</h3>
					<div class="space-y-2">
						<button
							onclick={() => (showCreateModal = true)}
							class="w-full px-4 py-2 text-left text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
						>
							<IconPlus class="w-4 h-4" />
							Create Board
						</button>
						<button
							onclick={() => (showFolderModal = true)}
							class="w-full px-4 py-2 text-left text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
						>
							<IconFolder class="w-4 h-4" />
							Create Folder
						</button>
						<a
							href="/admin/boards/import"
							class="w-full px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
						>
							<IconUpload class="w-4 h-4" />
							Import from Trello/Asana
						</a>
						<a
							href="/admin/boards/templates"
							class="w-full px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2"
						>
							<IconTemplate class="w-4 h-4" />
							Browse Templates
						</a>
					</div>
				</div>

				<!-- Folders -->
				<div
					class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4"
				>
					<h3 class="font-semibold text-gray-900 dark:text-white mb-4">Folders</h3>
					<div class="space-y-1">
						<button
							onclick={() => (selectedFolder = null)}
							class="w-full px-3 py-2 text-left rounded-lg flex items-center gap-2 {selectedFolder ===
							null
								? 'bg-[#E6B800]/10 dark:bg-[#E6B800]/20 text-[#E6B800] dark:text-[#FFD11A]'
								: 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconLayoutKanban class="w-4 h-4" />
							All Boards
							<span class="ml-auto text-xs bg-gray-200 dark:bg-gray-700 px-2 py-0.5 rounded-full"
								>{boards.filter((b) => !b.is_archived).length}</span
							>
						</button>
						{#each folders as folder}
							<button
								onclick={() => (selectedFolder = folder.id)}
								class="w-full px-3 py-2 text-left rounded-lg flex items-center gap-2 {selectedFolder ===
								folder.id
									? 'bg-[#E6B800]/10 dark:bg-[#E6B800]/20 text-[#E6B800] dark:text-[#FFD11A]'
									: 'text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'}"
							>
								<IconFolder class="w-4 h-4" style="color: {folder.color}" />
								{folder.title}
								<span class="ml-auto text-xs bg-gray-200 dark:bg-gray-700 px-2 py-0.5 rounded-full"
									>{folder.board_count || 0}</span
								>
							</button>
						{/each}
					</div>
				</div>

				<!-- My Tasks Due Today -->
				{#if tasksDueToday.length > 0}
					<div
						class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4"
					>
						<h3 class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
							<IconCalendar class="w-4 h-4 text-orange-500" />
							Due Today
						</h3>
						<div class="space-y-2">
							{#each tasksDueToday.slice(0, 5) as task}
								<a
									href="/admin/boards/{task.board_id}?task={task.id}"
									class="block p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700"
								>
									<div class="text-sm font-medium text-gray-900 dark:text-white truncate">
										{task.title}
									</div>
									<div class="text-xs text-gray-500 dark:text-gray-400">
										{task.due_date ? formatDate(task.due_date) : ''}
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Overdue Tasks -->
				{#if overdueTasks.length > 0}
					<div
						class="bg-white dark:bg-gray-800 rounded-xl border border-red-200 dark:border-red-900/50 p-4"
					>
						<h3 class="font-semibold text-red-600 dark:text-red-400 mb-4 flex items-center gap-2">
							<IconAlertTriangle class="w-4 h-4" />
							Overdue Tasks
						</h3>
						<div class="space-y-2">
							{#each overdueTasks.slice(0, 5) as task}
								<a
									href="/admin/boards/{task.board_id}?task={task.id}"
									class="block p-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20"
								>
									<div class="text-sm font-medium text-gray-900 dark:text-white truncate">
										{task.title}
									</div>
									<div class="text-xs text-red-500">
										{task.due_date ? formatDate(task.due_date) : ''}
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- My Tasks -->
				{#if myTasks.length > 0}
					<div
						class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4"
					>
						<h3 class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
							<IconChecks class="w-4 h-4 text-blue-500" />
							My Tasks
						</h3>
						<div class="space-y-2">
							{#each myTasks.slice(0, 5) as task}
								<a
									href="/admin/boards/{task.board_id}?task={task.id}"
									class="block p-2 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700"
								>
									<div class="text-sm font-medium text-gray-900 dark:text-white truncate">
										{task.title}
									</div>
									<div class="text-xs text-gray-500 dark:text-gray-400">
										{task.due_date ? formatDate(task.due_date) : 'No due date'}
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Recent Activity -->
				{#if recentActivity.length > 0}
					<div
						class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4"
					>
						<h3 class="font-semibold text-gray-900 dark:text-white mb-4 flex items-center gap-2">
							<IconClock class="w-4 h-4 text-gray-500" />
							Recent Activity
						</h3>
						<div class="space-y-2">
							{#each recentActivity.slice(0, 5) as activity}
								<div class="text-xs text-gray-600 dark:text-gray-400">
									<span class="font-medium">{activity.user?.name || 'Someone'}</span>
									{activity.description}
									<span class="text-gray-500"
										>{activity.created_at ? formatDate(activity.created_at) : ''}</span
									>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</div>

			<!-- Main Content -->
			<div class="lg:col-span-3">
				<!-- Search and Filters -->
				<div class="flex items-center gap-4 mb-6">
					<div class="flex-1 relative">
						<IconSearch class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
						<input
							id="boards-searchquery"
							name="boards-searchquery"
							type="text"
							placeholder="Search boards..."
							bind:value={searchQuery}
							class="w-full pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#E6B800]"
						/>
					</div>
					<div class="flex items-center gap-2">
						<button
							onclick={() => (viewMode = 'grid')}
							class="p-2 rounded-lg {viewMode === 'grid'
								? 'bg-[#E6B800]/10 dark:bg-[#E6B800]/20 text-[#E6B800]'
								: 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconLayoutGrid class="w-5 h-5" />
						</button>
						<button
							onclick={() => (viewMode = 'list')}
							class="p-2 rounded-lg {viewMode === 'list'
								? 'bg-[#E6B800]/10 dark:bg-[#E6B800]/20 text-[#E6B800]'
								: 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700'}"
						>
							<IconList class="w-5 h-5" />
						</button>
					</div>
					<label class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
						<input
							id="boards-showarchived"
							name="boards-showarchived"
							type="checkbox"
							bind:checked={showArchived}
							class="rounded border-gray-300 dark:border-gray-600"
						/>
						Show archived
					</label>
					<button
						onclick={loadData}
						class="p-2 text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						<IconRefresh class="w-5 h-5" />
					</button>
				</div>

				<!-- Favorite Boards -->
				{#if favoriteBoards.length > 0}
					<div class="mb-8">
						<h3
							class="text-sm font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-4 flex items-center gap-2"
						>
							<IconStarFilled class="w-4 h-4 text-yellow-500" />
							Favorites
						</h3>
						<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
							{#each favoriteBoards as board}
								<a
									href="/admin/boards/{board.id}"
									class="group bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-4 hover:shadow-lg hover:border-[#E6B800] dark:hover:border-[#FFD11A] transition-all"
									style="border-left: 4px solid {board.background_color || '#E6B800'}"
								>
									<div class="flex items-start justify-between mb-3">
										<h4
											class="font-semibold text-gray-900 dark:text-white group-hover:text-[#E6B800] dark:group-hover:text-[#FFD11A]"
										>
											{board.title}
										</h4>
										<button
											onclick={(e: MouseEvent) => {
												e.preventDefault();
												e.stopPropagation();
												toggleFavorite(board);
											}}
											class="text-yellow-500 hover:text-yellow-600"
										>
											<IconStarFilled class="w-5 h-5" />
										</button>
									</div>
									{#if board.description}
										<p class="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mb-3">
											{board.description}
										</p>
									{/if}
									<div class="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
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
					<h3
						class="text-sm font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-4"
					>
						{selectedFolder ? folders.find((f) => f.id === selectedFolder)?.title : 'All Boards'}
					</h3>

					{#if loading}
						<div class="flex items-center justify-center py-12">
							<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#E6B800]"></div>
						</div>
					{:else if filteredBoards.length === 0}
						<div
							class="text-center py-12 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700"
						>
							<IconLayoutKanban class="w-12 h-12 text-gray-400 mx-auto mb-4" />
							<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">No boards yet</h3>
							<p class="text-gray-500 dark:text-gray-400 mb-4">
								Create your first board to start managing projects
							</p>
							<button
								onclick={() => (showCreateModal = true)}
								class="px-4 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg"
							>
								Create Board
							</button>
						</div>
					{:else if viewMode === 'grid'}
						<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
							{#each filteredBoards as board}
								<div
									class="group bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 hover:shadow-lg hover:border-[#E6B800] dark:hover:border-[#FFD11A] transition-all overflow-hidden"
								>
									<a
										href="/admin/boards/{board.id}"
										class="block p-4"
										style="border-left: 4px solid {board.background_color || '#E6B800'}"
									>
										<div class="flex items-start justify-between mb-3">
											<h4
												class="font-semibold text-gray-900 dark:text-white group-hover:text-[#E6B800] dark:group-hover:text-[#FFD11A]"
											>
												{board.title}
												{#if board.is_archived}
													<span
														class="ml-2 text-xs bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400 px-2 py-0.5 rounded"
														>Archived</span
													>
												{/if}
											</h4>
											<button
												onclick={(e: MouseEvent) => {
													e.preventDefault();
													e.stopPropagation();
													toggleFavorite(board);
												}}
												class="{board.is_favorite
													? 'text-yellow-500'
													: 'text-gray-400 opacity-0 group-hover:opacity-100'} hover:text-yellow-500 transition-all"
											>
												{#if board.is_favorite}
													<IconStarFilled class="w-5 h-5" />
												{:else}
													<IconStar class="w-5 h-5" />
												{/if}
											</button>
										</div>
										{#if board.description}
											<p class="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mb-3">
												{board.description}
											</p>
										{/if}
										<div class="flex items-center gap-4 text-xs text-gray-500 dark:text-gray-400">
											<span class="flex items-center gap-1">
												<IconChecks class="w-3.5 h-3.5" />
												{board.completed_task_count || 0}/{board.task_count || 0}
											</span>
											<span class="flex items-center gap-1">
												<IconUsers class="w-3.5 h-3.5" />
												{board.member_count || 1}
											</span>
										</div>
									</a>
									<div
										class="px-4 py-2 bg-gray-50 dark:bg-gray-700/50 border-t border-gray-200 dark:border-gray-700 flex items-center justify-end gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
									>
										<button
											onclick={() => goto(`/admin/boards/${board.id}`)}
											class="p-1.5 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 rounded"
											title="Open"
										>
											<IconEye class="w-4 h-4" />
										</button>
										<button
											onclick={() => duplicateBoard(board)}
											class="p-1.5 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 rounded"
											title="Duplicate"
										>
											<IconCopy class="w-4 h-4" />
										</button>
										<button
											onclick={() => archiveBoard(board)}
											class="p-1.5 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 rounded"
											title="Archive"
										>
											<IconArchive class="w-4 h-4" />
										</button>
										<button
											onclick={() => deleteBoard(board)}
											class="p-1.5 text-red-500 hover:text-red-700 hover:bg-red-100 dark:hover:bg-red-900/30 rounded"
											title="Delete"
										>
											<IconTrash class="w-4 h-4" />
										</button>
									</div>
								</div>
							{/each}
						</div>
					{:else}
						<div
							class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden"
						>
							<table class="w-full">
								<thead class="bg-gray-50 dark:bg-gray-700">
									<tr>
										<th
											class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
											>Board</th
										>
										<th
											class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
											>Tasks</th
										>
										<th
											class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
											>Members</th
										>
										<th
											class="px-4 py-3 text-left text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
											>Updated</th
										>
										<th
											class="px-4 py-3 text-right text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase"
											>Actions</th
										>
									</tr>
								</thead>
								<tbody class="divide-y divide-gray-200 dark:divide-gray-700">
									{#each filteredBoards as board}
										<tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
											<td class="px-4 py-3">
												<a href="/admin/boards/{board.id}" class="flex items-center gap-3">
													<div
														class="w-2 h-8 rounded"
														style="background-color: {board.background_color || '#E6B800'}"
													></div>
													<div>
														<div
															class="font-medium text-gray-900 dark:text-white flex items-center gap-2"
														>
															{board.title}
															{#if board.is_favorite}
																<IconStarFilled class="w-4 h-4 text-yellow-500" />
															{/if}
															{#if board.is_archived}
																<span
																	class="text-xs bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-400 px-2 py-0.5 rounded"
																	>Archived</span
																>
															{/if}
														</div>
														{#if board.description}
															<div
																class="text-sm text-gray-500 dark:text-gray-400 truncate max-w-xs"
															>
																{board.description}
															</div>
														{/if}
													</div>
												</a>
											</td>
											<td class="px-4 py-3 text-sm text-gray-600 dark:text-gray-300">
												{board.completed_task_count || 0}/{board.task_count || 0}
											</td>
											<td class="px-4 py-3 text-sm text-gray-600 dark:text-gray-300">
												{board.member_count || 1}
											</td>
											<td class="px-4 py-3 text-sm text-gray-500 dark:text-gray-400">
												{formatDate(board.updated_at)}
											</td>
											<td class="px-4 py-3 text-right">
												<div class="flex items-center justify-end gap-1">
													<button
														onclick={() => toggleFavorite(board)}
														class="p-1.5 text-gray-500 hover:text-yellow-500 rounded"
													>
														{#if board.is_favorite}
															<IconStarFilled class="w-4 h-4 text-yellow-500" />
														{:else}
															<IconStar class="w-4 h-4" />
														{/if}
													</button>
													<button
														onclick={() => duplicateBoard(board)}
														class="p-1.5 text-gray-500 hover:text-gray-700 dark:hover:text-gray-200 rounded"
													>
														<IconCopy class="w-4 h-4" />
													</button>
													<button
														onclick={() => deleteBoard(board)}
														class="p-1.5 text-red-500 hover:text-red-700 rounded"
													>
														<IconTrash class="w-4 h-4" />
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
		<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
			<div class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-md mx-4">
				<div class="p-6 border-b border-gray-200 dark:border-gray-700">
					<h2 class="text-xl font-semibold text-gray-900 dark:text-white">Create New Board</h2>
				</div>
				<div class="p-6 space-y-4">
					<div>
						<label
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
							for="board-title">Board Title</label
						>
						<input
							type="text"
							id="board-title"
							name="board-title"
							bind:value={newBoard.title}
							placeholder="Enter board title..."
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#E6B800]"
						/>
					</div>
					<div>
						<label
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
							for="board-description">Description (optional)</label
						>
						<textarea
							id="board-description"
							bind:value={newBoard.description}
							placeholder="Enter description..."
							rows="3"
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#E6B800]"
						></textarea>
					</div>
					<div>
						<label
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
							for="board-type">Board Type</label
						>
						<select
							id="board-type"
							bind:value={newBoard.type}
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#E6B800]"
						>
							<option value="kanban">Kanban Board</option>
							<option value="list">List View</option>
							<option value="calendar">Calendar View</option>
							<option value="table">Table View</option>
						</select>
					</div>
					{#if folders.length > 0}
						<div>
							<label
								class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
								for="board-folder">Folder (optional)</label
							>
							<select
								id="board-folder"
								bind:value={newBoard.folder_id}
								class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#E6B800]"
							>
								<option value={null}>No folder</option>
								{#each folders as folder}
									<option value={folder.id}>{folder.title}</option>
								{/each}
							</select>
						</div>
					{/if}
				</div>
				<div class="p-6 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
					<button
						onclick={() => (showCreateModal = false)}
						class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						Cancel
					</button>
					<button
						onclick={createBoard}
						disabled={!newBoard.title.trim()}
						class="px-4 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
					>
						Create Board
					</button>
				</div>
			</div>
		</div>
	{/if}

	<!-- Create Folder Modal -->
	{#if showFolderModal}
		<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
			<div class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-md mx-4">
				<div class="p-6 border-b border-gray-200 dark:border-gray-700">
					<h2 class="text-xl font-semibold text-gray-900 dark:text-white">Create New Folder</h2>
				</div>
				<div class="p-6 space-y-4">
					<div>
						<label
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
							for="folder-name">Folder Name</label
						>
						<input
							type="text"
							id="folder-name"
							name="folder-name"
							bind:value={newFolder.title}
							placeholder="Enter folder name..."
							class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-[#E6B800]"
						/>
					</div>
					<div>
						<label
							class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
							for="folder-color">Color</label
						>
						<input
							type="color"
							id="folder-color"
							name="folder-color"
							bind:value={newFolder.color}
							class="w-full h-10 rounded-lg cursor-pointer"
						/>
					</div>
				</div>
				<div class="p-6 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-3">
					<button
						onclick={() => (showFolderModal = false)}
						class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						Cancel
					</button>
					<button
						onclick={createFolder}
						disabled={!newFolder.title.trim()}
						class="px-4 py-2 bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg disabled:opacity-50 disabled:cursor-not-allowed"
					>
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

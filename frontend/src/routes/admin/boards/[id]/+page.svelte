<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { boardsAPI } from '$lib/api/boards';
	import type {
		Board,
		Stage,
		Task,
		Label,
		BoardMember,
		Comment,
		Attachment,
		Subtask,
		CustomFieldDefinition
	} from '$lib/boards/types';
	import {
		IconArrowLeft,
		IconPlus,
		IconDots,
		IconStar,
		IconStarFilled,
		IconUsers,
		IconSettings,
		IconFilter,
		IconSearch,
		IconCalendar,
		IconPaperclip,
		IconMessage,
		IconChecklist,
		IconX,
		IconTrash,
		IconCheck,
		IconUser,
		IconPlayerPlay,
		IconPlayerStop,
		IconSubtask
	} from '$lib/icons';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	// Get board ID from URL
	const boardId = $derived(page.params['id'] ?? '');

	// State
	let board = $state<Board | null>(null);
	let stages = $state<Stage[]>([]);
	let tasks = $state<Task[]>([]);
	let labels = $state<Label[]>([]);
	let members = $state<BoardMember[]>([]);
			// @ts-expect-error write-only state
	let customFields = $state<CustomFieldDefinition[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let filterAssignee = $state<string | null>(null);
	let filterLabel = $state<string | null>(null);
	let filterPriority = $state<string | null>(null);
	let showFilters = $state(false);

	// Task modal state
	let selectedTask = $state<Task | null>(null);
	let showTaskModal = $state(false);
	let taskComments = $state<Comment[]>([]);
	let taskAttachments = $state<Attachment[]>([]);
	let taskSubtasks = $state<Subtask[]>([]);
	let newComment = $state('');
	let newSubtaskTitle = $state('');
	let editingTaskTitle = $state(false);
	let editingTaskDescription = $state(false);

	// New task state
	let showNewTaskInput = $state<string | null>(null);
	let newTaskTitle = $state('');

	// New stage state
	let showNewStageInput = $state(false);
	let newStageTitle = $state('');

	// Drag and drop state
	let draggedTask = $state<Task | null>(null);
	let dragOverStage = $state<string | null>(null);
	let dragOverPosition = $state<number | null>(null);

	// Timer state
	let activeTimer = $state<{ taskId: string; startedAt: Date } | null>(null);
	let timerDisplay = $state('00:00:00');

	// Delete confirmation modal state
	let showDeleteTaskModal = $state(false);

	// Filtered tasks by stage
	let tasksByStage = $derived.by(() => {
		const result: Record<string, Task[]> = {};
		stages.forEach((stage) => {
			result[stage.id] = tasks
				.filter((t) => {
					if (t.stage_id !== stage.id) return false;
					if (searchQuery) {
						const query = searchQuery.toLowerCase();
						if (
							!t.title.toLowerCase().includes(query) &&
							!t.description?.toLowerCase().includes(query)
						) {
							return false;
						}
					}
					if (filterAssignee && !t.assignees.includes(filterAssignee)) return false;
					if (filterLabel && !t.labels?.some((l) => l.id === filterLabel)) return false;
					if (filterPriority && t.priority !== filterPriority) return false;
					return true;
				})
				.sort((a, b) => a.position - b.position);
		});
		return result;
	});

	onMount(() => {
		loadBoard();

		// Check for task in URL query
		const taskId = page.url.searchParams.get('task');
		if (taskId) {
			const task = tasks.find((t) => t.id === taskId);
			if (task) {
				openTaskModal(task);
			}
		}

		// Start timer interval
		const timerInterval = setInterval(() => {
			if (activeTimer) {
				const elapsed = Math.floor((Date.now() - activeTimer.startedAt.getTime()) / 1000);
				const hours = Math.floor(elapsed / 3600);
				const minutes = Math.floor((elapsed % 3600) / 60);
				const seconds = elapsed % 60;
				timerDisplay = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
			}
		}, 1000);

		return () => clearInterval(timerInterval);
	});

	async function loadBoard() {
		loading = true;
		try {
			const [boardRes, stagesRes, tasksRes, labelsRes, fieldsRes] = await Promise.all([
				boardsAPI.getBoard(boardId),
				boardsAPI.getStages(boardId),
				boardsAPI.getTasks(boardId),
				boardsAPI.getLabels(boardId),
				boardsAPI.getCustomFields(boardId)
			]);

			board = boardRes.board;
			members = boardRes.members || [];
			stages = stagesRes;
			tasks = tasksRes.data;
			labels = labelsRes;
			customFields = fieldsRes;
			// Suppress unused warning - customFields loaded for future use
		} catch (error) {
			console.error('Failed to load board:', error);
		} finally {
			loading = false;
		}
	}

	async function createTask(stageId: string) {
		if (!newTaskTitle.trim()) return;

		try {
			const task = await boardsAPI.createTask(boardId, {
				title: newTaskTitle,
				stage_id: stageId,
				position: (tasksByStage[stageId]?.length || 0) + 1
			});
			tasks = [...tasks, task];
			newTaskTitle = '';
			showNewTaskInput = null;
		} catch (error) {
			console.error('Failed to create task:', error);
		}
	}

	async function createStage() {
		if (!newStageTitle.trim()) return;

		try {
			const stage = await boardsAPI.createStage(boardId, {
				title: newStageTitle,
				color: '#6b7280',
				position: stages.length + 1
			});
			stages = [...stages, stage];
			newStageTitle = '';
			showNewStageInput = false;
		} catch (error) {
			console.error('Failed to create stage:', error);
		}
	}

	async function openTaskModal(task: Task) {
		selectedTask = task;
		showTaskModal = true;

		// Load task details
		try {
			const [fullTask, comments, attachments, subtasks] = await Promise.all([
				boardsAPI.getTask(boardId, task.id),
				boardsAPI.getComments(boardId, task.id),
				boardsAPI.getAttachments(boardId, task.id),
				boardsAPI.getSubtasks(boardId, task.id)
			]);
			selectedTask = fullTask;
			taskComments = comments;
			taskAttachments = attachments;
			taskSubtasks = subtasks;
		} catch (error) {
			console.error('Failed to load task details:', error);
		}
	}

	function closeTaskModal() {
		showTaskModal = false;
		selectedTask = null;
		taskComments = [];
		taskAttachments = [];
		taskSubtasks = [];
		editingTaskTitle = false;
		editingTaskDescription = false;
	}

	async function updateTask(data: Partial<Task>) {
		if (!selectedTask) return;

		try {
			const updated = await boardsAPI.updateTask(boardId, selectedTask.id, data);
			tasks = tasks.map((t) => (t.id === selectedTask!.id ? updated : t));
			selectedTask = updated;
		} catch (error) {
			console.error('Failed to update task:', error);
		}
	}

	function deleteTask() {
		if (!selectedTask) return;
		showDeleteTaskModal = true;
	}

	async function confirmDeleteTask() {
		if (!selectedTask) return;
		showDeleteTaskModal = false;

		try {
			await boardsAPI.deleteTask(boardId, selectedTask.id);
			tasks = tasks.filter((t) => t.id !== selectedTask!.id);
			closeTaskModal();
		} catch (error) {
			console.error('Failed to delete task:', error);
		}
	}

	async function completeTask() {
		if (!selectedTask) return;

		try {
			const updated = await boardsAPI.completeTask(boardId, selectedTask.id);
			tasks = tasks.map((t) => (t.id === selectedTask!.id ? updated : t));
			selectedTask = updated;
		} catch (error) {
			console.error('Failed to complete task:', error);
		}
	}

	async function addComment() {
		if (!selectedTask || !newComment.trim()) return;

		try {
			const comment = await boardsAPI.createComment(boardId, selectedTask.id, newComment);
			taskComments = [...taskComments, comment];
			newComment = '';
		} catch (error) {
			console.error('Failed to add comment:', error);
		}
	}

	async function addSubtask() {
		if (!selectedTask || !newSubtaskTitle.trim()) return;

		try {
			const subtask = await boardsAPI.createSubtask(boardId, selectedTask.id, {
				title: newSubtaskTitle
			});
			taskSubtasks = [...taskSubtasks, subtask];
			newSubtaskTitle = '';
		} catch (error) {
			console.error('Failed to add subtask:', error);
		}
	}

	async function toggleSubtask(subtask: Subtask) {
		if (!selectedTask) return;

		try {
			const updated = await boardsAPI.toggleSubtaskComplete(boardId, selectedTask.id, subtask.id);
			taskSubtasks = taskSubtasks.map((s) => (s.id === subtask.id ? updated : s));
		} catch (error) {
			console.error('Failed to toggle subtask:', error);
		}
	}

	async function startTimer() {
		if (!selectedTask) return;

		try {
			await boardsAPI.startTimer(boardId, selectedTask.id);
			activeTimer = { taskId: selectedTask.id, startedAt: new Date() };
		} catch (error) {
			console.error('Failed to start timer:', error);
		}
	}

	async function stopTimer() {
		if (!selectedTask || !activeTimer) return;

		try {
			await boardsAPI.stopTimer(boardId, selectedTask.id);
			activeTimer = null;
			timerDisplay = '00:00:00';
		} catch (error) {
			console.error('Failed to stop timer:', error);
		}
	}

	// Drag and drop handlers
	function handleDragStart(event: DragEvent, task: Task) {
		draggedTask = task;
		if (event.dataTransfer) {
			event.dataTransfer.effectAllowed = 'move';
		}
	}

	function handleDragOver(event: DragEvent, stageId: string, position: number) {
		event.preventDefault();
		dragOverStage = stageId;
		dragOverPosition = position;
	}

	function handleDragLeave() {
		dragOverStage = null;
		dragOverPosition = null;
	}

	async function handleDrop(event: DragEvent, stageId: string, position: number) {
		event.preventDefault();
		if (!draggedTask) return;

		try {
			const updated = await boardsAPI.moveTask(boardId, draggedTask.id, stageId, position);
			tasks = tasks.map((t) => (t.id === draggedTask!.id ? updated : t));

			// Reorder other tasks in the stage
			const stageTasks = tasks.filter((t) => t.stage_id === stageId && t.id !== draggedTask!.id);
			stageTasks.forEach((t, i) => {
				if (i >= position) {
					t.position = i + 1;
				}
			});
		} catch (error) {
			console.error('Failed to move task:', error);
		} finally {
			draggedTask = null;
			dragOverStage = null;
			dragOverPosition = null;
		}
	}

	function handleDragEnd() {
		draggedTask = null;
		dragOverStage = null;
		dragOverPosition = null;
	}

	function getPriorityColor(priority: string): string {
		const colors: Record<string, string> = {
			urgent: 'bg-red-500',
			high: 'bg-orange-500',
			medium: 'bg-yellow-500',
			low: 'bg-blue-500',
			none: 'bg-gray-400'
		};
		return colors[priority] || 'bg-gray-400';
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const diffDays = Math.ceil((date.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));

		if (diffDays < 0) return `${Math.abs(diffDays)}d overdue`;
		if (diffDays === 0) return 'Today';
		if (diffDays === 1) return 'Tomorrow';
		if (diffDays <= 7) return `${diffDays}d`;
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	function isOverdue(dateStr: string): boolean {
		return new Date(dateStr) < new Date();
	}
</script>

<svelte:head>
	<title>{board?.title || 'Board'} | Project Boards</title>
</svelte:head>

<div class="bg-gray-100 dark:bg-gray-900 flex flex-col">
	<!-- Header -->
	<div
		class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex-shrink-0"
	>
		<div class="px-4 py-3">
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-4">
					<a
						href="/admin/boards"
						class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						<IconArrowLeft class="w-5 h-5" />
					</a>
					{#if board}
						<div class="flex items-center gap-3">
							<div
								class="w-3 h-8 rounded"
								style="background-color: {board.background_color || '#E6B800'}"
							></div>
							<div>
								<h1
									class="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2"
								>
									{board.title}
									<button
										onclick={async () => {
											const updated = await boardsAPI.toggleFavorite(boardId);
											board = updated;
										}}
										class={board.is_favorite
											? 'text-yellow-500'
											: 'text-gray-400 hover:text-yellow-500'}
									>
										{#if board.is_favorite}
											<IconStarFilled class="w-5 h-5" />
										{:else}
											<IconStar class="w-5 h-5" />
										{/if}
									</button>
								</h1>
								{#if board.description}
									<p class="text-sm text-gray-500 dark:text-gray-400">{board.description}</p>
								{/if}
							</div>
						</div>
					{/if}
				</div>

				<div class="flex items-center gap-3">
					<!-- Search -->
					<div class="relative">
						<IconSearch class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
						<input
							id="page-searchquery"
							name="page-searchquery"
							type="text"
							placeholder="Search tasks..."
							bind:value={searchQuery}
							class="pl-9 pr-4 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white w-48 focus:w-64 transition-all focus:ring-2 focus:ring-[#E6B800]"
						/>
					</div>

					<!-- Filters -->
					<button
						onclick={() => (showFilters = !showFilters)}
						class="px-3 py-1.5 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg flex items-center gap-2 {showFilters
							? 'bg-gray-100 dark:bg-gray-700'
							: ''}"
					>
						<IconFilter class="w-4 h-4" />
						Filters
					</button>

					<!-- Members -->
					<div class="flex items-center -space-x-2">
						{#each members.slice(0, 4) as member}
							<div
								class="w-8 h-8 rounded-full bg-[#E6B800] flex items-center justify-center text-[#0D1117] text-sm font-medium border-2 border-white dark:border-gray-800"
								title={member.name}
							>
								{member.name.charAt(0).toUpperCase()}
							</div>
						{/each}
						{#if members.length > 4}
							<div
								class="w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center text-gray-600 dark:text-gray-300 text-xs font-medium border-2 border-white dark:border-gray-800"
							>
								+{members.length - 4}
							</div>
						{/if}
						<button
							class="w-8 h-8 rounded-full bg-gray-100 dark:bg-gray-700 flex items-center justify-center text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 border-2 border-white dark:border-gray-800 ml-1"
						>
							<IconUsers class="w-4 h-4" />
						</button>
					</div>

					<!-- Settings -->
					<a
						href="/admin/boards/{boardId}/settings"
						class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
					>
						<IconSettings class="w-5 h-5" />
					</a>
				</div>
			</div>

			<!-- Filter Bar -->
			{#if showFilters}
				<div
					class="mt-3 pt-3 border-t border-gray-200 dark:border-gray-700 flex items-center gap-4"
				>
					<select
						bind:value={filterAssignee}
						class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
					>
						<option value={null}>All assignees</option>
						{#each members as member}
							<option value={member.user_id}>{member.name}</option>
						{/each}
					</select>
					<select
						bind:value={filterLabel}
						class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
					>
						<option value={null}>All labels</option>
						{#each labels as label}
							<option value={label.id}>{label.title}</option>
						{/each}
					</select>
					<select
						bind:value={filterPriority}
						class="px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
					>
						<option value={null}>All priorities</option>
						<option value="urgent">Urgent</option>
						<option value="high">High</option>
						<option value="medium">Medium</option>
						<option value="low">Low</option>
						<option value="none">None</option>
					</select>
					{#if filterAssignee || filterLabel || filterPriority}
						<button
							onclick={() => {
								filterAssignee = null;
								filterLabel = null;
								filterPriority = null;
							}}
							class="px-3 py-1.5 text-sm text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg"
						>
							Clear filters
						</button>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	<!-- Kanban Board -->
	<div class="flex-1 overflow-x-auto p-4">
		{#if loading}
			<div class="flex items-center justify-center h-64">
				<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#E6B800]"></div>
			</div>
		{:else}
			<div class="flex gap-4 h-full min-h-[calc(100vh-180px)]">
				{#each stages as stage}
					<div
						class="flex-shrink-0 w-72 bg-gray-200/50 dark:bg-gray-800/50 rounded-xl flex flex-col max-h-full"
					>
						<!-- Stage Header -->
						<div class="p-3 flex items-center justify-between flex-shrink-0">
							<div class="flex items-center gap-2">
								<div class="w-3 h-3 rounded-full" style="background-color: {stage.color}"></div>
								<h3 class="font-medium text-gray-900 dark:text-white">{stage.title}</h3>
								<span
									class="text-xs bg-gray-300 dark:bg-gray-700 text-gray-600 dark:text-gray-400 px-2 py-0.5 rounded-full"
								>
									{tasksByStage[stage.id]?.length || 0}
								</span>
							</div>
							<button
								class="p-1 text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-700 rounded"
							>
								<IconDots class="w-4 h-4" />
							</button>
						</div>

						<!-- Tasks -->
						<div
							class="flex-1 overflow-y-auto p-2 space-y-2"
							ondragover={(e: DragEvent) =>
								handleDragOver(e, stage.id, tasksByStage[stage.id]?.length || 0)}
							ondragleave={handleDragLeave}
							ondrop={(e: DragEvent) =>
								handleDrop(e, stage.id, tasksByStage[stage.id]?.length || 0)}
							role="region"
							aria-label="Task drop zone"
						>
							{#each tasksByStage[stage.id] || [] as task, index}
								<div
									draggable="true"
									ondragstart={(e: DragEvent) => handleDragStart(e, task)}
									ondragend={handleDragEnd}
									ondragover={(e: DragEvent) => handleDragOver(e, stage.id, index)}
									role="button"
									tabindex="0"
									class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-3 cursor-pointer hover:shadow-md transition-shadow {draggedTask?.id ===
									task.id
										? 'opacity-50'
										: ''} {dragOverStage === stage.id && dragOverPosition === index
										? 'border-t-2 border-[#E6B800]'
										: ''}"
									onclick={() => openTaskModal(task)}
									onkeydown={(e: KeyboardEvent) => {
										if (e.key === 'Enter' || e.key === ' ') {
											e.preventDefault();
											openTaskModal(task);
										}
									}}
								>
									<!-- Labels -->
									{#if task.labels && task.labels.length > 0}
										<div class="flex flex-wrap gap-1 mb-2">
											{#each task.labels as label}
												<span
													class="px-2 py-0.5 text-xs rounded text-white"
													style="background-color: {label.color}"
												>
													{label.title}
												</span>
											{/each}
										</div>
									{/if}

									<!-- Title -->
									<h4 class="text-sm font-medium text-gray-900 dark:text-white mb-2">
										{task.title}
									</h4>

									<!-- Meta -->
									<div
										class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400"
									>
										<div class="flex items-center gap-3">
											{#if task.due_date}
												<span
													class="flex items-center gap-1 {isOverdue(task.due_date) &&
													task.status !== 'completed'
														? 'text-red-500'
														: ''}"
												>
													<IconCalendar class="w-3.5 h-3.5" />
													{formatDate(task.due_date)}
												</span>
											{/if}
											{#if task.subtask_count}
												<span class="flex items-center gap-1">
													<IconChecklist class="w-3.5 h-3.5" />
													{task.completed_subtask_count || 0}/{task.subtask_count}
												</span>
											{/if}
											{#if task.comment_count}
												<span class="flex items-center gap-1">
													<IconMessage class="w-3.5 h-3.5" />
													{task.comment_count}
												</span>
											{/if}
											{#if task.attachment_count}
												<span class="flex items-center gap-1">
													<IconPaperclip class="w-3.5 h-3.5" />
													{task.attachment_count}
												</span>
											{/if}
										</div>
										<div class="flex items-center gap-1">
											{#if task.priority && task.priority !== 'none'}
												<div class="w-2 h-2 rounded-full {getPriorityColor(task.priority)}"></div>
											{/if}
											{#if task.assignees && task.assignees.length > 0}
												<div class="flex -space-x-1">
													{#each task.assignees.slice(0, 2) as assigneeId}
														{@const assignee = members.find((m) => m.user_id === assigneeId)}
														{#if assignee}
															<div
																class="w-6 h-6 rounded-full bg-[#E6B800] flex items-center justify-center text-[#0D1117] text-xs border border-white dark:border-gray-800"
																title={assignee.name}
															>
																{assignee.name.charAt(0).toUpperCase()}
															</div>
														{/if}
													{/each}
												</div>
											{/if}
										</div>
									</div>
								</div>
							{/each}

							<!-- Drop zone indicator -->
							{#if dragOverStage === stage.id && dragOverPosition === (tasksByStage[stage.id]?.length || 0)}
								<div class="h-1 bg-[#E6B800] rounded"></div>
							{/if}

							<!-- Add Task Input -->
							{#if showNewTaskInput === stage.id}
								<div
									class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-3"
								>
									<textarea
										bind:value={newTaskTitle}
										placeholder="Enter task title..."
										rows="2"
										class="w-full text-sm border-0 p-0 resize-none focus:ring-0 bg-transparent text-gray-900 dark:text-white placeholder-gray-400"
										onkeydown={(e: KeyboardEvent) => {
											if (e.key === 'Enter' && !e.shiftKey) {
												e.preventDefault();
												createTask(stage.id);
											}
											if (e.key === 'Escape') {
												showNewTaskInput = null;
												newTaskTitle = '';
											}
										}}
									></textarea>
									<div class="flex items-center justify-between mt-2">
										<div class="flex items-center gap-2">
											<button
												onclick={() => createTask(stage.id)}
												disabled={!newTaskTitle.trim()}
												class="px-3 py-1 text-xs bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded disabled:opacity-50 disabled:cursor-not-allowed"
											>
												Add
											</button>
											<button
												onclick={() => {
													showNewTaskInput = null;
													newTaskTitle = '';
												}}
												class="px-3 py-1 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
											>
												Cancel
											</button>
										</div>
									</div>
								</div>
							{/if}
						</div>

						<!-- Add Task Button -->
						{#if showNewTaskInput !== stage.id}
							<div class="p-2 flex-shrink-0">
								<button
									onclick={() => (showNewTaskInput = stage.id)}
									class="w-full px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-300/50 dark:hover:bg-gray-700/50 rounded-lg flex items-center gap-2"
								>
									<IconPlus class="w-4 h-4" />
									Add task
								</button>
							</div>
						{/if}
					</div>
				{/each}

				<!-- Add Stage -->
				<div class="flex-shrink-0 w-72">
					{#if showNewStageInput}
						<div class="bg-gray-200/50 dark:bg-gray-800/50 rounded-xl p-3">
							<input
								id="page-newstagetitle"
								name="page-newstagetitle"
								type="text"
								bind:value={newStageTitle}
								placeholder="Stage title..."
								class="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white mb-2"
								onkeydown={(e: KeyboardEvent) => {
									if (e.key === 'Enter') createStage();
									if (e.key === 'Escape') {
										showNewStageInput = false;
										newStageTitle = '';
									}
								}}
							/>
							<div class="flex items-center gap-2">
								<button
									onclick={createStage}
									disabled={!newStageTitle.trim()}
									class="px-3 py-1.5 text-sm bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg disabled:opacity-50"
								>
									Add Stage
								</button>
								<button
									onclick={() => {
										showNewStageInput = false;
										newStageTitle = '';
									}}
									class="px-3 py-1.5 text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-300 dark:hover:bg-gray-700 rounded-lg"
								>
									Cancel
								</button>
							</div>
						</div>
					{:else}
						<button
							onclick={() => (showNewStageInput = true)}
							class="w-full px-4 py-3 text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-200/50 dark:hover:bg-gray-800/50 rounded-xl border-2 border-dashed border-gray-300 dark:border-gray-700 flex items-center justify-center gap-2"
						>
							<IconPlus class="w-4 h-4" />
							Add Stage
						</button>
					{/if}
				</div>
			</div>
		{/if}
	</div>

	<!-- Task Detail Modal -->
	{#if showTaskModal && selectedTask}
		<div
			class="fixed inset-0 bg-black/50 flex items-start justify-center z-50 overflow-y-auto py-8"
		>
			<div class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-3xl mx-4">
				<!-- Modal Header -->
				<div
					class="p-4 border-b border-gray-200 dark:border-gray-700 flex items-start justify-between"
				>
					<div class="flex-1">
						{#if editingTaskTitle}
							<input
								id="page-text"
								name="page-text"
								type="text"
								value={selectedTask.title}
								onblur={(e: FocusEvent) => {
									updateTask({ title: (e.currentTarget as HTMLInputElement).value });
									editingTaskTitle = false;
								}}
								onkeydown={(e: KeyboardEvent) => {
									if (e.key === 'Enter') {
										updateTask({ title: (e.currentTarget as HTMLInputElement).value });
										editingTaskTitle = false;
									}
								}}
								class="w-full text-xl font-semibold bg-transparent border-b border-[#E6B800] focus:outline-none text-gray-900 dark:text-white"
							/>
						{:else}
							<button
								type="button"
								class="text-xl font-semibold text-gray-900 dark:text-white cursor-pointer hover:text-[#E6B800] dark:hover:text-[#FFD11A] text-left w-full bg-transparent border-0 p-0"
								onclick={() => (editingTaskTitle = true)}
							>
								{selectedTask?.title}
							</button>
						{/if}
						<p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
							in {stages.find((s) => s.id === selectedTask?.stage_id)?.title || 'Unknown'}
						</p>
					</div>
					<div class="flex items-center gap-2">
						{#if selectedTask?.status === 'completed'}
							<span
								class="px-2 py-1 text-xs bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400 rounded flex items-center gap-1"
							>
								<IconCheck class="w-3.5 h-3.5" />
								Completed
							</span>
						{:else}
							<button
								onclick={completeTask}
								class="px-3 py-1.5 text-sm bg-green-600 hover:bg-green-700 text-white rounded-lg flex items-center gap-1"
							>
								<IconCheck class="w-4 h-4" />
								Complete
							</button>
						{/if}
						<button
							onclick={closeTaskModal}
							class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
						>
							<IconX class="w-5 h-5" />
						</button>
					</div>
				</div>

				<div class="flex">
					<!-- Main Content -->
					<div class="flex-1 p-6 border-r border-gray-200 dark:border-gray-700">
						<!-- Description -->
						<div class="mb-6">
							<h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Description</h3>
							{#if editingTaskDescription}
								<textarea
									value={selectedTask.description || ''}
									onblur={(e: FocusEvent) => {
										updateTask({ description: (e.currentTarget as HTMLTextAreaElement).value });
										editingTaskDescription = false;
									}}
									rows="4"
									class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
									placeholder="Add a description..."
								></textarea>
							{:else}
								<div
									onclick={() => (editingTaskDescription = true)}
									onkeydown={(e: KeyboardEvent) => {
										if (e.key === 'Enter' || e.key === ' ') {
											e.preventDefault();
											editingTaskDescription = true;
										}
									}}
									role="button"
									tabindex="0"
									class="text-sm text-gray-600 dark:text-gray-400 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-700 rounded-lg p-2 -m-2 min-h-[60px]"
								>
									{selectedTask.description || 'Click to add a description...'}
								</div>
							{/if}
						</div>

						<!-- Subtasks -->
						<div class="mb-6">
							<h3
								class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 flex items-center gap-2"
							>
								<IconSubtask class="w-4 h-4" />
								Subtasks
								{#if taskSubtasks.length > 0}
									<span class="text-xs text-gray-500">
										{taskSubtasks.filter((s) => s.is_completed).length}/{taskSubtasks.length}
									</span>
								{/if}
							</h3>
							<div class="space-y-2">
								{#each taskSubtasks as subtask}
									<div class="flex items-center gap-2 group">
										<button
											onclick={() => toggleSubtask(subtask)}
											class="w-5 h-5 rounded border {subtask.is_completed
												? 'bg-green-500 border-green-500 text-white'
												: 'border-gray-300 dark:border-gray-600'} flex items-center justify-center"
										>
											{#if subtask.is_completed}
												<IconCheck class="w-3 h-3" />
											{/if}
										</button>
										<span
											class="text-sm {subtask.is_completed
												? 'line-through text-gray-400'
												: 'text-gray-700 dark:text-gray-300'}"
										>
											{subtask.title}
										</span>
									</div>
								{/each}
								<div class="flex items-center gap-2">
									<input
										id="page-newsubtasktitle"
										name="page-newsubtasktitle"
										type="text"
										bind:value={newSubtaskTitle}
										placeholder="Add subtask..."
										class="flex-1 px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										onkeydown={(e: KeyboardEvent) => {
											if (e.key === 'Enter') addSubtask();
										}}
									/>
									<button
										onclick={addSubtask}
										disabled={!newSubtaskTitle.trim()}
										class="px-3 py-1.5 text-sm bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg disabled:opacity-50"
									>
										Add
									</button>
								</div>
							</div>
						</div>

						<!-- Comments -->
						<div>
							<h3
								class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2 flex items-center gap-2"
							>
								<IconMessage class="w-4 h-4" />
								Comments ({taskComments.length})
							</h3>
							<div class="space-y-3">
								{#each taskComments as comment}
									<div class="flex gap-3">
										<div
											class="w-8 h-8 rounded-full bg-[#E6B800] flex items-center justify-center text-[#0D1117] text-sm flex-shrink-0"
										>
											{comment.author?.name?.charAt(0).toUpperCase() || 'U'}
										</div>
										<div class="flex-1">
											<div class="flex items-center gap-2">
												<span class="text-sm font-medium text-gray-900 dark:text-white"
													>{comment.author?.name || 'Unknown'}</span
												>
												<span class="text-xs text-gray-500"
													>{new Date(comment.created_at).toLocaleString()}</span
												>
											</div>
											<p class="text-sm text-gray-600 dark:text-gray-400 mt-1">{comment.content}</p>
										</div>
									</div>
								{/each}
								<div class="flex gap-3">
									<div
										class="w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center text-gray-600 dark:text-gray-400 text-sm flex-shrink-0"
									>
										<IconUser class="w-4 h-4" />
									</div>
									<div class="flex-1">
										<textarea
											bind:value={newComment}
											placeholder="Write a comment..."
											rows="2"
											class="w-full px-3 py-2 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
										></textarea>
										<button
											onclick={addComment}
											disabled={!newComment.trim()}
											class="mt-2 px-3 py-1.5 text-sm bg-[#E6B800] hover:bg-[#B38F00] text-[#0D1117] rounded-lg disabled:opacity-50"
										>
											Comment
										</button>
									</div>
								</div>
							</div>
						</div>
					</div>

					<!-- Sidebar -->
					<div class="w-64 p-4 space-y-4">
						<!-- Timer -->
						<div>
							<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase mb-2">
								Time Tracking
							</h4>
							<div class="flex items-center gap-2">
								{#if activeTimer?.taskId === selectedTask.id}
									<span class="text-lg font-mono text-[#E6B800]">{timerDisplay}</span>
									<button
										onclick={stopTimer}
										class="p-1.5 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded"
									>
										<IconPlayerStop class="w-5 h-5" />
									</button>
								{:else}
									<span class="text-sm text-gray-600 dark:text-gray-400"
										>{selectedTask.logged_minutes || 0}m logged</span
									>
									<button
										onclick={startTimer}
										class="p-1.5 text-green-500 hover:bg-green-50 dark:hover:bg-green-900/20 rounded"
									>
										<IconPlayerPlay class="w-5 h-5" />
									</button>
								{/if}
							</div>
						</div>

						<!-- Assignees -->
						<div>
							<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase mb-2">
								Assignees
							</h4>
							<div class="flex flex-wrap gap-2">
								{#each selectedTask.assignees || [] as assigneeId}
									{@const assignee = members.find((m) => m.user_id === assigneeId)}
									{#if assignee}
										<div
											class="flex items-center gap-2 px-2 py-1 bg-gray-100 dark:bg-gray-700 rounded-full"
										>
											<div
												class="w-5 h-5 rounded-full bg-[#E6B800] flex items-center justify-center text-[#0D1117] text-xs"
											>
												{assignee.name.charAt(0).toUpperCase()}
											</div>
											<span class="text-xs text-gray-700 dark:text-gray-300">{assignee.name}</span>
										</div>
									{/if}
								{/each}
								<button
									class="px-2 py-1 text-xs text-[#E6B800] hover:bg-[#E6B800]/5 dark:hover:bg-[#B38F00]/20 rounded-full"
								>
									+ Add
								</button>
							</div>
						</div>

						<!-- Due Date -->
						<div>
							<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase mb-2">
								Due Date
							</h4>
							<input
								id="page-date"
								name="page-date"
								type="date"
								value={selectedTask.due_date?.split('T')[0] || ''}
								onchange={(e: Event) =>
									updateTask({ due_date: (e.currentTarget as HTMLInputElement).value })}
								class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
							/>
						</div>

						<!-- Priority -->
						<div>
							<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase mb-2">
								Priority
							</h4>
							<select
								value={selectedTask.priority}
								onchange={(e: Event) =>
									updateTask({ priority: (e.currentTarget as HTMLSelectElement).value as any })}
								class="w-full px-3 py-1.5 text-sm border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
							>
								<option value="none">None</option>
								<option value="low">Low</option>
								<option value="medium">Medium</option>
								<option value="high">High</option>
								<option value="urgent">Urgent</option>
							</select>
						</div>

						<!-- Labels -->
						<div>
							<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase mb-2">
								Labels
							</h4>
							<div class="flex flex-wrap gap-1">
								{#each selectedTask.labels || [] as label}
									<span
										class="px-2 py-0.5 text-xs rounded text-white"
										style="background-color: {label.color}"
									>
										{label.title}
									</span>
								{/each}
								<button
									class="px-2 py-0.5 text-xs text-[#E6B800] hover:bg-[#E6B800]/5 dark:hover:bg-[#B38F00]/20 rounded"
								>
									+ Add
								</button>
							</div>
						</div>

						<!-- Attachments -->
						<div>
							<h4 class="text-xs font-medium text-gray-500 dark:text-gray-400 uppercase mb-2">
								Attachments ({taskAttachments.length})
							</h4>
							<div class="space-y-1">
								{#each taskAttachments as attachment}
									<a
										href={attachment.url}
										target="_blank"
										class="flex items-center gap-2 px-2 py-1.5 text-xs text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded"
									>
										<IconPaperclip class="w-3.5 h-3.5" />
										<span class="truncate">{attachment.original_filename}</span>
									</a>
								{/each}
								<button
									class="w-full px-2 py-1.5 text-xs text-[#E6B800] hover:bg-[#E6B800]/5 dark:hover:bg-[#B38F00]/20 rounded flex items-center gap-2"
								>
									<IconPlus class="w-3.5 h-3.5" />
									Add attachment
								</button>
							</div>
						</div>

						<!-- Actions -->
						<div class="pt-4 border-t border-gray-200 dark:border-gray-700 space-y-1">
							<button
								onclick={deleteTask}
								class="w-full px-3 py-1.5 text-sm text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg flex items-center gap-2"
							>
								<IconTrash class="w-4 h-4" />
								Delete task
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showDeleteTaskModal}
	title="Delete Task"
	message="Delete this task?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteTask}
	onCancel={() => {
		showDeleteTaskModal = false;
	}}
/>

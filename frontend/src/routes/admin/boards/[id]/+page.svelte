<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { boardsAPI } from '$lib/api/boards';
	import type {
		Board,
		Stage,
		Task,
		TaskPriority,
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
	// FIX-2026-04-26 (P0-4): toast for drop-failure surfacing.
	import { toastStore } from '$lib/stores/toast.svelte';
	import { logger } from '$lib/utils/logger';

	// Get board ID from URL
	const boardId = $derived(page.params['id'] ?? '');

	// State
	let board = $state<Board | null>(null);
	let stages = $state<Stage[]>([]);
	let tasks = $state<Task[]>([]);
	let labels = $state<Label[]>([]);
	let members = $state<BoardMember[]>([]);
	let _customFields = $state<CustomFieldDefinition[]>([]);
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
	// FIX-2026-04-26 (P0-4): per-board reorder lock to prevent racing concurrent drops.
	let dropInFlight = $state(false);

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
		// FIX-2026-04-26 (P1-6): await loadBoard() before processing the ?task= deep-link
		// so the lookup runs against populated `tasks`, not the empty initial array.
		void (async () => {
			await loadBoard();
			const taskId = page.url.searchParams.get('task');
			if (taskId) {
				const task = tasks.find((t) => t.id === taskId);
				if (task) {
					openTaskModal(task);
				}
			}
		})();

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
			_customFields = fieldsRes;
			// Suppress unused warning - _customFields loaded for future use
		} catch (error) {
			logger.error('[BoardPage] Failed to load board', { error });
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
			logger.error('[BoardPage] Failed to create task', { error });
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
			logger.error('[BoardPage] Failed to create stage', { error });
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
			logger.error('[BoardPage] Failed to load task details', { error });
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
		const target = selectedTask;

		try {
			const updated = await boardsAPI.updateTask(boardId, target.id, data);
			tasks = tasks.map((t) => (t.id === target.id ? updated : t));
			selectedTask = updated;
		} catch (error) {
			logger.error('[BoardPage] Failed to update task', { error });
		}
	}

	function deleteTask() {
		if (!selectedTask) return;
		showDeleteTaskModal = true;
	}

	async function confirmDeleteTask() {
		if (!selectedTask) return;
		const target = selectedTask;
		showDeleteTaskModal = false;

		try {
			await boardsAPI.deleteTask(boardId, target.id);
			tasks = tasks.filter((t) => t.id !== target.id);
			closeTaskModal();
		} catch (error) {
			logger.error('[BoardPage] Failed to delete task', { error });
		}
	}

	async function completeTask() {
		if (!selectedTask) return;
		const target = selectedTask;

		try {
			const updated = await boardsAPI.completeTask(boardId, target.id);
			tasks = tasks.map((t) => (t.id === target.id ? updated : t));
			selectedTask = updated;
		} catch (error) {
			logger.error('[BoardPage] Failed to complete task', { error });
		}
	}

	async function addComment() {
		if (!selectedTask || !newComment.trim()) return;

		try {
			const comment = await boardsAPI.createComment(boardId, selectedTask.id, newComment);
			taskComments = [...taskComments, comment];
			newComment = '';
		} catch (error) {
			logger.error('[BoardPage] Failed to add comment', { error });
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
			logger.error('[BoardPage] Failed to add subtask', { error });
		}
	}

	async function toggleSubtask(subtask: Subtask) {
		if (!selectedTask) return;

		try {
			const updated = await boardsAPI.toggleSubtaskComplete(boardId, selectedTask.id, subtask.id);
			taskSubtasks = taskSubtasks.map((s) => (s.id === subtask.id ? updated : s));
		} catch (error) {
			logger.error('[BoardPage] Failed to toggle subtask', { error });
		}
	}

	async function startTimer() {
		if (!selectedTask) return;

		try {
			await boardsAPI.startTimer(boardId, selectedTask.id);
			activeTimer = { taskId: selectedTask.id, startedAt: new Date() };
		} catch (error) {
			logger.error('[BoardPage] Failed to start timer', { error });
		}
	}

	async function stopTimer() {
		if (!selectedTask || !activeTimer) return;

		try {
			await boardsAPI.stopTimer(boardId, selectedTask.id);
			activeTimer = null;
			timerDisplay = '00:00:00';
		} catch (error) {
			logger.error('[BoardPage] Failed to stop timer', { error });
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

		// FIX-2026-04-26 (P0-4):
		//  - Capture draggedTask into a local const before any await so concurrent
		//    drops can't trample the module-level reference (also kills the `!`
		//    non-null-assertion crashes).
		//  - Add a per-board lock so a fast double-drop doesn't fire two requests.
		//  - Optimistically reorder, then on failure refetch + show error toast.
		//  - Always reassign `tasks = [...tasks]` so Svelte 5's deep proxy notices.
		if (dropInFlight) return;
		const moving = draggedTask;
		dropInFlight = true;

		// Snapshot for rollback.
		const snapshot = tasks.map((t) => ({ ...t }));

		try {
			// Optimistic: shift sibling positions in the source array (not a filtered copy).
			const next = tasks.map((t) => {
				if (t.id === moving.id) {
					return { ...t, stage_id: stageId, position };
				}
				if (t.stage_id === stageId && t.position >= position) {
					return { ...t, position: t.position + 1 };
				}
				return t;
			});
			tasks = next;

			const updated = await boardsAPI.moveTask(boardId, moving.id, stageId, position);
			// Reconcile with backend's authoritative view of the moved task.
			tasks = tasks.map((t) => (t.id === moving.id ? updated : t));
		} catch (error) {
			logger.error('[BoardPage] Failed to move task', { error });
			// Roll back to pre-drop state, then refetch to be sure we're in sync.
			tasks = snapshot;
			toastStore.error('Failed to move task. Reloading board.');
			try {
				await loadBoard();
			} catch (reloadErr) {
				logger.error('[BoardPage] Failed to reload board after drop failure', {
					error: reloadErr
				});
			}
		} finally {
			draggedTask = null;
			dragOverStage = null;
			dragOverPosition = null;
			dropInFlight = false;
		}
	}

	function handleDragEnd() {
		draggedTask = null;
		dragOverStage = null;
		dragOverPosition = null;
	}

	function getPriorityColor(priority: string): string {
		const colors: Record<string, string> = {
			urgent: 'priority-dot--urgent',
			high: 'priority-dot--high',
			medium: 'priority-dot--medium',
			low: 'priority-dot--low',
			none: 'priority-dot--none'
		};
		return colors[priority] || 'priority-dot--none';
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

<div class="board-detail">
	<!-- Header -->
	<div class="board-header">
		<div class="board-header-inner">
			<div class="board-header-row">
				<div class="board-title-area">
					<a href="/admin/boards" class="icon-button" aria-label="Back to boards">
						<IconArrowLeft class="toolbar-icon" />
					</a>
					{#if board}
						<div class="board-title-group">
							<div
								class="board-color-strip"
								style="background-color: {board.background_color || '#E6B800'}"
							></div>
							<div>
								<h1 class="board-title">
									{board.title}
									<button
										onclick={async () => {
											const updated = await boardsAPI.toggleFavorite(boardId);
											board = updated;
										}}
										class="favorite-button"
										class:active={board.is_favorite}
										aria-label={board.is_favorite ? 'Remove favorite' : 'Add favorite'}
									>
										{#if board.is_favorite}
											<IconStarFilled class="toolbar-icon" />
										{:else}
											<IconStar class="toolbar-icon" />
										{/if}
									</button>
								</h1>
								{#if board.description}
									<p class="board-subtitle">{board.description}</p>
								{/if}
							</div>
						</div>
					{/if}
				</div>

				<div class="board-toolbar">
					<!-- Search -->
					<div class="search-field">
						<IconSearch class="search-icon" />
						<input
							id="page-searchquery"
							name="page-searchquery"
							type="text"
							placeholder="Search tasks..."
							bind:value={searchQuery}
							class="search-input"
						/>
					</div>

					<!-- Filters -->
					<button
						onclick={() => (showFilters = !showFilters)}
						class="toolbar-button"
						class:active={showFilters}
					>
						<IconFilter class="small-icon" />
						Filters
					</button>

					<!-- Members -->
					<div class="member-stack">
						{#each members.slice(0, 4) as member, i (i)}
							<div class="member-avatar" title={member.name}>
								{member.name.charAt(0).toUpperCase()}
							</div>
						{/each}
						{#if members.length > 4}
							<div class="member-avatar member-avatar--more">
								+{members.length - 4}
							</div>
						{/if}
						<button class="member-add-button" aria-label="Manage members">
							<IconUsers class="small-icon" />
						</button>
					</div>

					<!-- Settings -->
					<a
						href="/admin/boards/{boardId}/settings"
						class="icon-button"
						aria-label="Board settings"
					>
						<IconSettings class="toolbar-icon" />
					</a>
				</div>
			</div>

			<!-- Filter Bar -->
			{#if showFilters}
				<div class="filter-bar">
					<select bind:value={filterAssignee} class="filter-select">
						<option value={null}>All assignees</option>
						{#each members as member (member.user_id)}
							<option value={member.user_id}>{member.name}</option>
						{/each}
					</select>
					<select bind:value={filterLabel} class="filter-select">
						<option value={null}>All labels</option>
						{#each labels as label (label.id)}
							<option value={label.id}>{label.title}</option>
						{/each}
					</select>
					<select bind:value={filterPriority} class="filter-select">
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
							class="clear-filters-button"
						>
							Clear filters
						</button>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	<!-- Kanban Board -->
	<div class="kanban-scroll">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
			</div>
		{:else}
			<div class="kanban-board">
				{#each stages as stage (stage.id)}
					<div class="stage-column">
						<!-- Stage Header -->
						<div class="stage-header">
							<div class="stage-title-group">
								<div class="stage-color-dot" style="background-color: {stage.color}"></div>
								<h3 class="stage-title">{stage.title}</h3>
								<span class="stage-count">
									{tasksByStage[stage.id]?.length || 0}
								</span>
							</div>
							<button class="stage-menu-button" aria-label="Stage actions">
								<IconDots class="small-icon" />
							</button>
						</div>

						<!-- Tasks -->
						<div
							class="task-list"
							ondragover={(e: DragEvent) =>
								handleDragOver(e, stage.id, tasksByStage[stage.id]?.length || 0)}
							ondragleave={handleDragLeave}
							ondrop={(e: DragEvent) =>
								handleDrop(e, stage.id, tasksByStage[stage.id]?.length || 0)}
							role="region"
							aria-label="Task drop zone"
						>
							{#each tasksByStage[stage.id] || [] as task, index (task.id)}
								<div
									draggable="true"
									ondragstart={(e: DragEvent) => handleDragStart(e, task)}
									ondragend={handleDragEnd}
									ondragover={(e: DragEvent) => handleDragOver(e, stage.id, index)}
									role="button"
									tabindex="0"
									class="task-card"
									class:dragging={draggedTask?.id === task.id}
									class:drop-target={dragOverStage === stage.id && dragOverPosition === index}
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
										<div class="task-labels">
											{#each task.labels as label (label.id)}
												<span class="task-label" style="background-color: {label.color}">
													{label.title}
												</span>
											{/each}
										</div>
									{/if}

									<!-- Title -->
									<h4 class="task-title">
										{task.title}
									</h4>

									<!-- Meta -->
									<div class="task-meta-row">
										<div class="task-meta-list">
											{#if task.due_date}
												<span
													class="task-meta-item"
													class:overdue={isOverdue(task.due_date) && task.status !== 'completed'}
												>
													<IconCalendar class="meta-icon" />
													{formatDate(task.due_date)}
												</span>
											{/if}
											{#if task.subtask_count}
												<span class="task-meta-item">
													<IconChecklist class="meta-icon" />
													{task.completed_subtask_count || 0}/{task.subtask_count}
												</span>
											{/if}
											{#if task.comment_count}
												<span class="task-meta-item">
													<IconMessage class="meta-icon" />
													{task.comment_count}
												</span>
											{/if}
											{#if task.attachment_count}
												<span class="task-meta-item">
													<IconPaperclip class="meta-icon" />
													{task.attachment_count}
												</span>
											{/if}
										</div>
										<div class="task-end-meta">
											{#if task.priority && task.priority !== 'none'}
												<div class="priority-dot {getPriorityColor(task.priority)}"></div>
											{/if}
											{#if task.assignees && task.assignees.length > 0}
												<div class="mini-avatar-stack">
													{#each task.assignees.slice(0, 2) as assigneeId (assigneeId)}
														{@const assignee = members.find((m) => m.user_id === assigneeId)}
														{#if assignee}
															<div class="mini-avatar" title={assignee.name}>
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
								<div class="drop-indicator"></div>
							{/if}

							<!-- Add Task Input -->
							{#if showNewTaskInput === stage.id}
								<div class="task-composer">
									<textarea
										bind:value={newTaskTitle}
										placeholder="Enter task title..."
										rows="2"
										class="task-composer-input"
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
									<div class="composer-actions">
										<div class="button-row">
											<button
												onclick={() => createTask(stage.id)}
												disabled={!newTaskTitle.trim()}
												class="primary-button primary-button--small"
											>
												Add
											</button>
											<button
												onclick={() => {
													showNewTaskInput = null;
													newTaskTitle = '';
												}}
												class="ghost-button ghost-button--small"
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
							<div class="add-task-wrap">
								<button onclick={() => (showNewTaskInput = stage.id)} class="add-task-button">
									<IconPlus class="small-icon" />
									Add task
								</button>
							</div>
						{/if}
					</div>
				{/each}

				<!-- Add Stage -->
				<div class="add-stage-column">
					{#if showNewStageInput}
						<div class="stage-composer">
							<input
								id="page-newstagetitle"
								name="page-newstagetitle"
								type="text"
								bind:value={newStageTitle}
								placeholder="Stage title..."
								class="form-control"
								onkeydown={(e: KeyboardEvent) => {
									if (e.key === 'Enter') createStage();
									if (e.key === 'Escape') {
										showNewStageInput = false;
										newStageTitle = '';
									}
								}}
							/>
							<div class="button-row">
								<button
									onclick={createStage}
									disabled={!newStageTitle.trim()}
									class="primary-button primary-button--small"
								>
									Add Stage
								</button>
								<button
									onclick={() => {
										showNewStageInput = false;
										newStageTitle = '';
									}}
									class="ghost-button ghost-button--small"
								>
									Cancel
								</button>
							</div>
						</div>
					{:else}
						<button onclick={() => (showNewStageInput = true)} class="add-stage-button">
							<IconPlus class="small-icon" />
							Add Stage
						</button>
					{/if}
				</div>
			</div>
		{/if}
	</div>

	<!-- Task Detail Modal -->
	{#if showTaskModal && selectedTask}
		<div class="modal-backdrop modal-backdrop--top">
			<div class="task-modal">
				<!-- Modal Header -->
				<div class="task-modal-header">
					<div class="task-modal-title-area">
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
								class="task-title-input"
							/>
						{:else}
							<button
								type="button"
								class="task-title-button"
								onclick={() => (editingTaskTitle = true)}
							>
								{selectedTask?.title}
							</button>
						{/if}
						<p class="task-location">
							in {stages.find((s) => s.id === selectedTask?.stage_id)?.title || 'Unknown'}
						</p>
					</div>
					<div class="modal-header-actions">
						{#if selectedTask?.status === 'completed'}
							<span class="status-pill status-pill--complete">
								<IconCheck class="meta-icon" />
								Completed
							</span>
						{:else}
							<button onclick={completeTask} class="complete-button">
								<IconCheck class="small-icon" />
								Complete
							</button>
						{/if}
						<button onclick={closeTaskModal} class="icon-button" aria-label="Close task modal">
							<IconX class="toolbar-icon" />
						</button>
					</div>
				</div>

				<div class="task-modal-layout">
					<!-- Main Content -->
					<div class="task-modal-main">
						<!-- Description -->
						<div class="modal-section">
							<h3 class="modal-section-title">Description</h3>
							{#if editingTaskDescription}
								<textarea
									value={selectedTask.description || ''}
									onblur={(e: FocusEvent) => {
										updateTask({ description: (e.currentTarget as HTMLTextAreaElement).value });
										editingTaskDescription = false;
									}}
									rows="4"
									class="form-control"
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
									class="description-preview"
								>
									{selectedTask.description || 'Click to add a description...'}
								</div>
							{/if}
						</div>

						<!-- Subtasks -->
						<div class="modal-section">
							<h3 class="modal-section-title modal-section-title--icon">
								<IconSubtask class="small-icon" />
								Subtasks
								{#if taskSubtasks.length > 0}
									<span class="modal-count">
										{taskSubtasks.filter((s) => s.is_completed).length}/{taskSubtasks.length}
									</span>
								{/if}
							</h3>
							<div class="subtask-list">
								{#each taskSubtasks as subtask (subtask.id)}
									<div class="subtask-item">
										<button
											onclick={() => toggleSubtask(subtask)}
											class="subtask-check"
											class:checked={subtask.is_completed}
											aria-label={subtask.is_completed
												? 'Mark subtask incomplete'
												: 'Mark subtask complete'}
										>
											{#if subtask.is_completed}
												<IconCheck class="tiny-icon" />
											{/if}
										</button>
										<span class="subtask-title" class:complete={subtask.is_completed}>
											{subtask.title}
										</span>
									</div>
								{/each}
								<div class="inline-form">
									<input
										id="page-newsubtasktitle"
										name="page-newsubtasktitle"
										type="text"
										bind:value={newSubtaskTitle}
										placeholder="Add subtask..."
										class="form-control"
										onkeydown={(e: KeyboardEvent) => {
											if (e.key === 'Enter') addSubtask();
										}}
									/>
									<button
										onclick={addSubtask}
										disabled={!newSubtaskTitle.trim()}
										class="primary-button primary-button--small"
									>
										Add
									</button>
								</div>
							</div>
						</div>

						<!-- Comments -->
						<div>
							<h3 class="modal-section-title modal-section-title--icon">
								<IconMessage class="small-icon" />
								Comments ({taskComments.length})
							</h3>
							<div class="comment-list">
								{#each taskComments as comment (comment.id)}
									<div class="comment-item">
										<div class="member-avatar comment-avatar">
											{comment.author?.name?.charAt(0).toUpperCase() || 'U'}
										</div>
										<div class="comment-body">
											<div class="comment-header">
												<span class="comment-author">{comment.author?.name || 'Unknown'}</span>
												<span class="comment-time"
													>{new Date(comment.created_at).toLocaleString()}</span
												>
											</div>
											<p class="comment-content">{comment.content}</p>
										</div>
									</div>
								{/each}
								<div class="comment-item">
									<div class="member-avatar member-avatar--muted comment-avatar">
										<IconUser class="small-icon" />
									</div>
									<div class="comment-body">
										<textarea
											bind:value={newComment}
											placeholder="Write a comment..."
											rows="2"
											class="form-control"
										></textarea>
										<button
											onclick={addComment}
											disabled={!newComment.trim()}
											class="primary-button primary-button--small comment-button"
										>
											Comment
										</button>
									</div>
								</div>
							</div>
						</div>
					</div>

					<!-- Sidebar -->
					<div class="task-modal-sidebar">
						<!-- Timer -->
						<div class="sidebar-section">
							<h4 class="sidebar-label">Time Tracking</h4>
							<div class="sidebar-inline">
								{#if activeTimer?.taskId === selectedTask.id}
									<span class="timer-display">{timerDisplay}</span>
									<button
										onclick={stopTimer}
										class="icon-button icon-button--danger"
										aria-label="Stop timer"
									>
										<IconPlayerStop class="toolbar-icon" />
									</button>
								{:else}
									<span class="sidebar-text">{selectedTask.logged_minutes || 0}m logged</span>
									<button
										onclick={startTimer}
										class="icon-button icon-button--success"
										aria-label="Start timer"
									>
										<IconPlayerPlay class="toolbar-icon" />
									</button>
								{/if}
							</div>
						</div>

						<!-- Assignees -->
						<div class="sidebar-section">
							<h4 class="sidebar-label">Assignees</h4>
							<div class="chip-list">
								{#each selectedTask.assignees || [] as assigneeId (assigneeId)}
									{@const assignee = members.find((m) => m.user_id === assigneeId)}
									{#if assignee}
										<div class="assignee-chip">
											<div class="mini-avatar">
												{assignee.name.charAt(0).toUpperCase()}
											</div>
											<span>{assignee.name}</span>
										</div>
									{/if}
								{/each}
								<button class="chip-add-button"> + Add </button>
							</div>
						</div>

						<!-- Due Date -->
						<div class="sidebar-section">
							<h4 class="sidebar-label">Due Date</h4>
							<input
								id="page-date"
								name="page-date"
								type="date"
								value={selectedTask.due_date?.split('T')[0] || ''}
								onchange={(e: Event) =>
									updateTask({ due_date: (e.currentTarget as HTMLInputElement).value })}
								class="form-control"
							/>
						</div>

						<!-- Priority -->
						<div class="sidebar-section">
							<h4 class="sidebar-label">Priority</h4>
							<select
								value={selectedTask.priority}
								onchange={(e: Event) =>
									updateTask({
										priority: (e.currentTarget as HTMLSelectElement).value as TaskPriority
									})}
								class="form-control"
							>
								<option value="none">None</option>
								<option value="low">Low</option>
								<option value="medium">Medium</option>
								<option value="high">High</option>
								<option value="urgent">Urgent</option>
							</select>
						</div>

						<!-- Labels -->
						<div class="sidebar-section">
							<h4 class="sidebar-label">Labels</h4>
							<div class="label-chip-list">
								{#each selectedTask.labels || [] as label (label.id)}
									<span class="task-label" style="background-color: {label.color}">
										{label.title}
									</span>
								{/each}
								<button class="label-add-button"> + Add </button>
							</div>
						</div>

						<!-- Attachments -->
						<div class="sidebar-section">
							<h4 class="sidebar-label">
								Attachments ({taskAttachments.length})
							</h4>
							<div class="attachment-list">
								{#each taskAttachments as attachment (attachment.id)}
									<a href={attachment.url} target="_blank" class="attachment-link">
										<IconPaperclip class="meta-icon" />
										<span>{attachment.original_filename}</span>
									</a>
								{/each}
								<button class="attachment-add-button">
									<IconPlus class="meta-icon" />
									Add attachment
								</button>
							</div>
						</div>

						<!-- Actions -->
						<div class="sidebar-section sidebar-section--bordered">
							<button onclick={deleteTask} class="danger-action-button">
								<IconTrash class="small-icon" />
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

<style>
	.board-detail {
		display: flex;
		flex-direction: column;
		min-height: 100%;
		background: #f3f4f6;
		color: #111827;
	}

	:global(.dark) .board-detail {
		background: #111827;
		color: #f9fafb;
	}

	.board-header {
		flex-shrink: 0;
		border-bottom: 1px solid #e5e7eb;
		background: #ffffff;
	}

	:global(.dark) .board-header {
		border-color: #374151;
		background: #1f2937;
	}

	.board-header-inner {
		padding: 0.75rem 1rem;
	}

	.board-header-row,
	.board-title-area,
	.board-title-group,
	.board-title,
	.board-toolbar,
	.toolbar-button,
	.member-stack,
	.member-avatar,
	.member-add-button,
	.filter-bar,
	.stage-header,
	.stage-title-group,
	.task-meta-row,
	.task-meta-list,
	.task-meta-item,
	.task-end-meta,
	.composer-actions,
	.button-row,
	.add-task-button,
	.add-stage-button,
	.task-modal-header,
	.modal-header-actions,
	.status-pill,
	.complete-button,
	.task-modal-layout,
	.modal-section-title--icon,
	.subtask-item,
	.subtask-check,
	.inline-form,
	.comment-item,
	.comment-header,
	.sidebar-inline,
	.chip-list,
	.assignee-chip,
	.label-chip-list,
	.attachment-link,
	.attachment-add-button,
	.danger-action-button {
		display: flex;
		align-items: center;
	}

	.board-header-row {
		justify-content: space-between;
		gap: 1rem;
	}

	.board-title-area {
		gap: 1rem;
		min-width: 0;
	}

	.board-title-group {
		gap: 0.75rem;
		min-width: 0;
	}

	.board-color-strip {
		width: 0.75rem;
		height: 2rem;
		border-radius: 0.25rem;
		flex-shrink: 0;
	}

	.board-title {
		gap: 0.5rem;
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: #111827;
	}

	:global(.dark) .board-title {
		color: #ffffff;
	}

	.board-subtitle,
	.task-location,
	.description-preview,
	.sidebar-text,
	.comment-content {
		color: #6b7280;
	}

	:global(.dark) .board-subtitle,
	:global(.dark) .task-location,
	:global(.dark) .description-preview,
	:global(.dark) .sidebar-text,
	:global(.dark) .comment-content {
		color: #9ca3af;
	}

	.board-subtitle,
	.task-location,
	.description-preview,
	.comment-content {
		margin: 0;
		font-size: 0.875rem;
	}

	.board-toolbar {
		gap: 0.75rem;
		flex-wrap: wrap;
		justify-content: flex-end;
	}

	.search-field {
		position: relative;
	}

	.search-input,
	.filter-select,
	.form-control {
		border: 1px solid #d1d5db;
		border-radius: 0.5rem;
		background: #ffffff;
		color: #111827;
		font: inherit;
		font-size: 0.875rem;
	}

	.search-input {
		width: 12rem;
		padding: 0.375rem 1rem 0.375rem 2.25rem;
		transition: width 0.2s ease;
	}

	.search-input:focus {
		width: 16rem;
	}

	.filter-select,
	.form-control {
		padding: 0.5rem 0.75rem;
	}

	.form-control {
		width: 100%;
	}

	:global(.dark) .search-input,
	:global(.dark) .filter-select,
	:global(.dark) .form-control {
		border-color: #4b5563;
		background: #374151;
		color: #ffffff;
	}

	.search-input:focus,
	.filter-select:focus,
	.form-control:focus {
		outline: 2px solid #e6b800;
		outline-offset: 2px;
	}

	.search-field :global(.search-icon) {
		position: absolute;
		top: 50%;
		left: 0.75rem;
		width: 1rem;
		height: 1rem;
		color: #9ca3af;
		transform: translateY(-50%);
	}

	.icon-button,
	.favorite-button,
	.toolbar-button,
	.stage-menu-button,
	.ghost-button,
	.chip-add-button,
	.label-add-button,
	.attachment-add-button {
		border: 0;
		background: transparent;
		color: #6b7280;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			color 0.2s ease,
			opacity 0.2s ease;
	}

	.icon-button,
	.toolbar-button,
	.stage-menu-button {
		border-radius: 0.5rem;
	}

	.icon-button {
		padding: 0.5rem;
	}

	.toolbar-button {
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		color: #374151;
		font-size: 0.875rem;
	}

	:global(.dark) .toolbar-button {
		color: #d1d5db;
	}

	.icon-button:hover,
	.toolbar-button:hover,
	.stage-menu-button:hover,
	.ghost-button:hover {
		background: #f3f4f6;
		color: #374151;
	}

	:global(.dark) .icon-button:hover,
	:global(.dark) .toolbar-button:hover,
	:global(.dark) .stage-menu-button:hover,
	:global(.dark) .ghost-button:hover {
		background: #374151;
		color: #e5e7eb;
	}

	.toolbar-button.active {
		background: #f3f4f6;
	}

	:global(.dark) .toolbar-button.active {
		background: #374151;
	}

	.favorite-button {
		color: #9ca3af;
	}

	.favorite-button:hover,
	.favorite-button.active {
		color: #eab308;
	}

	.member-stack {
		margin-left: 0.5rem;
	}

	.member-avatar,
	.member-add-button,
	.mini-avatar {
		justify-content: center;
		border-radius: 999px;
	}

	.member-avatar,
	.member-add-button {
		width: 2rem;
		height: 2rem;
		border: 2px solid #ffffff;
		margin-left: -0.5rem;
	}

	:global(.dark) .member-avatar,
	:global(.dark) .member-add-button {
		border-color: #1f2937;
	}

	.member-avatar {
		background: #e6b800;
		color: #0d1117;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.member-avatar--more,
	.member-avatar--muted,
	.member-add-button {
		background: #d1d5db;
		color: #4b5563;
	}

	:global(.dark) .member-avatar--more,
	:global(.dark) .member-avatar--muted,
	:global(.dark) .member-add-button {
		background: #4b5563;
		color: #d1d5db;
	}

	.member-add-button {
		margin-left: 0.25rem;
	}

	.filter-bar {
		gap: 1rem;
		flex-wrap: wrap;
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid #e5e7eb;
	}

	:global(.dark) .filter-bar {
		border-color: #374151;
	}

	.clear-filters-button,
	.danger-action-button {
		border: 0;
		border-radius: 0.5rem;
		background: transparent;
		color: #dc2626;
		font: inherit;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.clear-filters-button {
		padding: 0.375rem 0.75rem;
	}

	.clear-filters-button:hover,
	.danger-action-button:hover,
	.icon-button--danger:hover {
		background: #fef2f2;
	}

	:global(.dark) .clear-filters-button:hover,
	:global(.dark) .danger-action-button:hover,
	:global(.dark) .icon-button--danger:hover {
		background: rgba(127, 29, 29, 0.2);
	}

	.kanban-scroll {
		flex: 1;
		overflow-x: auto;
		padding: 1rem;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 16rem;
	}

	.spinner {
		width: 2rem;
		height: 2rem;
		border: 2px solid rgba(230, 184, 0, 0.25);
		border-bottom-color: #e6b800;
		border-radius: 999px;
		animation: board-spin 0.8s linear infinite;
	}

	.kanban-board {
		display: flex;
		gap: 1rem;
		min-height: calc(100vh - 180px);
	}

	.stage-column,
	.add-stage-column {
		width: 18rem;
		flex-shrink: 0;
	}

	.stage-column {
		display: flex;
		flex-direction: column;
		max-height: 100%;
		border-radius: 0.75rem;
		background: rgba(229, 231, 235, 0.5);
	}

	:global(.dark) .stage-column {
		background: rgba(31, 41, 55, 0.5);
	}

	.stage-header {
		justify-content: space-between;
		flex-shrink: 0;
		padding: 0.75rem;
	}

	.stage-title-group {
		gap: 0.5rem;
	}

	.stage-color-dot,
	.priority-dot {
		border-radius: 999px;
		flex-shrink: 0;
	}

	.stage-color-dot {
		width: 0.75rem;
		height: 0.75rem;
	}

	.stage-title {
		margin: 0;
		color: #111827;
		font-size: 1rem;
		font-weight: 500;
	}

	:global(.dark) .stage-title {
		color: #ffffff;
	}

	.stage-count {
		padding: 0.125rem 0.5rem;
		border-radius: 999px;
		background: #d1d5db;
		color: #4b5563;
		font-size: 0.75rem;
	}

	:global(.dark) .stage-count {
		background: #374151;
		color: #9ca3af;
	}

	.stage-menu-button {
		padding: 0.25rem;
	}

	.task-list {
		display: flex;
		flex: 1;
		flex-direction: column;
		gap: 0.5rem;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.task-card,
	.task-composer,
	.task-modal {
		border: 1px solid #e5e7eb;
		background: #ffffff;
	}

	:global(.dark) .task-card,
	:global(.dark) .task-composer,
	:global(.dark) .task-modal {
		border-color: #374151;
		background: #1f2937;
	}

	.task-card,
	.task-composer {
		padding: 0.75rem;
		border-radius: 0.5rem;
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.08);
	}

	.task-card {
		cursor: pointer;
		transition:
			box-shadow 0.2s ease,
			opacity 0.2s ease,
			border-color 0.2s ease;
	}

	.task-card:hover {
		box-shadow: 0 6px 14px rgba(15, 23, 42, 0.12);
	}

	.task-card.dragging {
		opacity: 0.5;
	}

	.task-card.drop-target {
		border-top: 2px solid #e6b800;
	}

	.task-labels,
	.label-chip-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.25rem;
	}

	.task-labels {
		margin-bottom: 0.5rem;
	}

	.task-label {
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
		color: #ffffff;
		font-size: 0.75rem;
	}

	.task-title {
		margin: 0 0 0.5rem;
		color: #111827;
		font-size: 0.875rem;
		font-weight: 500;
	}

	:global(.dark) .task-title {
		color: #ffffff;
	}

	.task-meta-row {
		justify-content: space-between;
		gap: 0.5rem;
		color: #6b7280;
		font-size: 0.75rem;
	}

	:global(.dark) .task-meta-row {
		color: #9ca3af;
	}

	.task-meta-list {
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.task-meta-item {
		gap: 0.25rem;
	}

	.task-meta-item.overdue {
		color: #ef4444;
	}

	.task-end-meta {
		gap: 0.25rem;
	}

	.priority-dot {
		width: 0.5rem;
		height: 0.5rem;
	}

	.priority-dot--urgent {
		background: #ef4444;
	}

	.priority-dot--high {
		background: #f97316;
	}

	.priority-dot--medium {
		background: #eab308;
	}

	.priority-dot--low {
		background: #3b82f6;
	}

	.priority-dot--none {
		background: #9ca3af;
	}

	.mini-avatar-stack {
		display: flex;
		margin-left: 0.25rem;
	}

	.mini-avatar {
		width: 1.5rem;
		height: 1.5rem;
		margin-left: -0.25rem;
		border: 1px solid #ffffff;
		background: #e6b800;
		color: #0d1117;
		font-size: 0.75rem;
	}

	:global(.dark) .mini-avatar {
		border-color: #1f2937;
	}

	.drop-indicator {
		height: 0.25rem;
		border-radius: 0.25rem;
		background: #e6b800;
	}

	.task-composer-input {
		width: 100%;
		padding: 0;
		border: 0;
		background: transparent;
		color: #111827;
		font: inherit;
		font-size: 0.875rem;
		resize: none;
	}

	:global(.dark) .task-composer-input {
		color: #ffffff;
	}

	.task-composer-input:focus {
		outline: 0;
	}

	.composer-actions {
		justify-content: space-between;
		margin-top: 0.5rem;
	}

	.button-row {
		gap: 0.5rem;
	}

	.primary-button {
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

	.primary-button--small,
	.ghost-button--small {
		padding: 0.375rem 0.75rem;
		font-size: 0.75rem;
	}

	.ghost-button {
		border-radius: 0.375rem;
	}

	.add-task-wrap {
		flex-shrink: 0;
		padding: 0.5rem;
	}

	.add-task-button,
	.add-stage-button {
		width: 100%;
		gap: 0.5rem;
		border: 0;
		border-radius: 0.75rem;
		background: transparent;
		color: #4b5563;
		font: inherit;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.add-task-button {
		justify-content: flex-start;
		padding: 0.5rem 0.75rem;
	}

	.add-stage-button {
		justify-content: center;
		padding: 0.75rem 1rem;
		border: 2px dashed #d1d5db;
	}

	.add-task-button:hover,
	.add-stage-button:hover {
		background: rgba(209, 213, 219, 0.5);
	}

	:global(.dark) .add-task-button,
	:global(.dark) .add-stage-button {
		color: #9ca3af;
	}

	:global(.dark) .add-task-button:hover,
	:global(.dark) .add-stage-button:hover,
	:global(.dark) .stage-composer {
		background: rgba(31, 41, 55, 0.5);
	}

	:global(.dark) .add-stage-button {
		border-color: #374151;
	}

	.stage-composer {
		padding: 0.75rem;
		border-radius: 0.75rem;
		background: rgba(229, 231, 235, 0.5);
	}

	.stage-composer .form-control {
		margin-bottom: 0.5rem;
	}

	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		justify-content: center;
		background: rgba(0, 0, 0, 0.5);
	}

	.modal-backdrop--top {
		align-items: flex-start;
		overflow-y: auto;
		padding-block: 2rem;
	}

	.task-modal {
		width: min(100% - 2rem, 48rem);
		border-radius: 0.75rem;
		box-shadow: 0 20px 40px rgba(15, 23, 42, 0.25);
	}

	.task-modal-header {
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
	}

	:global(.dark) .task-modal-header {
		border-color: #374151;
	}

	.task-modal-title-area,
	.task-modal-main,
	.comment-body {
		flex: 1;
		min-width: 0;
	}

	.task-title-input,
	.task-title-button {
		width: 100%;
		color: #111827;
		font: inherit;
		font-size: 1.25rem;
		font-weight: 600;
	}

	:global(.dark) .task-title-input,
	:global(.dark) .task-title-button {
		color: #ffffff;
	}

	.task-title-input {
		border: 0;
		border-bottom: 1px solid #e6b800;
		background: transparent;
	}

	.task-title-input:focus {
		outline: 0;
	}

	.task-title-button {
		padding: 0;
		border: 0;
		background: transparent;
		text-align: left;
		cursor: pointer;
	}

	.task-title-button:hover {
		color: #e6b800;
	}

	.task-location {
		margin-top: 0.25rem;
	}

	.modal-header-actions {
		align-items: flex-start;
		gap: 0.5rem;
	}

	.status-pill {
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		font-size: 0.75rem;
	}

	.status-pill--complete {
		background: #dcfce7;
		color: #16a34a;
	}

	:global(.dark) .status-pill--complete {
		background: rgba(20, 83, 45, 0.3);
		color: #4ade80;
	}

	.complete-button {
		gap: 0.25rem;
		padding: 0.375rem 0.75rem;
		border: 0;
		border-radius: 0.5rem;
		background: #16a34a;
		color: #ffffff;
		font: inherit;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.complete-button:hover {
		background: #15803d;
	}

	.task-modal-layout {
		align-items: stretch;
	}

	.task-modal-main {
		padding: 1.5rem;
		border-right: 1px solid #e5e7eb;
	}

	:global(.dark) .task-modal-main {
		border-color: #374151;
	}

	.modal-section {
		margin-bottom: 1.5rem;
	}

	.modal-section-title {
		margin: 0 0 0.5rem;
		color: #374151;
		font-size: 0.875rem;
		font-weight: 500;
	}

	:global(.dark) .modal-section-title {
		color: #d1d5db;
	}

	.modal-section-title--icon {
		gap: 0.5rem;
	}

	.modal-count,
	.comment-time,
	.sidebar-label {
		color: #6b7280;
		font-size: 0.75rem;
	}

	.description-preview {
		min-height: 60px;
		margin: -0.5rem;
		padding: 0.5rem;
		border-radius: 0.5rem;
		cursor: pointer;
	}

	.description-preview:hover {
		background: #f9fafb;
	}

	:global(.dark) .description-preview:hover {
		background: #374151;
	}

	.subtask-list,
	.comment-list,
	.attachment-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.comment-list {
		gap: 0.75rem;
	}

	.subtask-item,
	.inline-form {
		gap: 0.5rem;
	}

	.subtask-check {
		justify-content: center;
		width: 1.25rem;
		height: 1.25rem;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		background: transparent;
		color: #ffffff;
		cursor: pointer;
	}

	:global(.dark) .subtask-check {
		border-color: #4b5563;
	}

	.subtask-check.checked {
		border-color: #22c55e;
		background: #22c55e;
	}

	.subtask-title {
		color: #374151;
		font-size: 0.875rem;
	}

	:global(.dark) .subtask-title {
		color: #d1d5db;
	}

	.subtask-title.complete {
		color: #9ca3af;
		text-decoration: line-through;
	}

	.inline-form .form-control {
		flex: 1;
	}

	.comment-item {
		align-items: flex-start;
		gap: 0.75rem;
	}

	.comment-avatar {
		flex-shrink: 0;
		margin-left: 0;
		border: 0;
	}

	.comment-header {
		gap: 0.5rem;
	}

	.comment-author {
		color: #111827;
		font-size: 0.875rem;
		font-weight: 500;
	}

	:global(.dark) .comment-author {
		color: #ffffff;
	}

	.comment-content {
		margin-top: 0.25rem;
	}

	.comment-button {
		margin-top: 0.5rem;
	}

	.task-modal-sidebar {
		width: 16rem;
		padding: 1rem;
	}

	.sidebar-section {
		margin-bottom: 1rem;
	}

	.sidebar-section--bordered {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
	}

	:global(.dark) .sidebar-section--bordered {
		border-color: #374151;
	}

	.sidebar-label {
		margin: 0 0 0.5rem;
		font-weight: 500;
		letter-spacing: 0.08em;
		text-transform: uppercase;
	}

	.sidebar-inline {
		gap: 0.5rem;
	}

	.timer-display {
		color: #e6b800;
		font-family: var(--font-mono);
		font-size: 1.125rem;
	}

	.icon-button--success {
		color: #22c55e;
	}

	.icon-button--success:hover {
		background: #f0fdf4;
	}

	:global(.dark) .icon-button--success:hover {
		background: rgba(20, 83, 45, 0.2);
	}

	.chip-list {
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.assignee-chip {
		gap: 0.5rem;
		padding: 0.25rem 0.5rem;
		border-radius: 999px;
		background: #f3f4f6;
		color: #374151;
		font-size: 0.75rem;
	}

	:global(.dark) .assignee-chip {
		background: #374151;
		color: #d1d5db;
	}

	.chip-add-button,
	.label-add-button,
	.attachment-add-button {
		color: #e6b800;
		font-size: 0.75rem;
	}

	.chip-add-button {
		padding: 0.25rem 0.5rem;
		border-radius: 999px;
	}

	.label-add-button {
		padding: 0.125rem 0.5rem;
		border-radius: 0.25rem;
	}

	.chip-add-button:hover,
	.label-add-button:hover,
	.attachment-add-button:hover {
		background: rgba(230, 184, 0, 0.08);
	}

	.attachment-list {
		gap: 0.25rem;
	}

	.attachment-link,
	.attachment-add-button,
	.danger-action-button {
		gap: 0.5rem;
		padding: 0.375rem 0.5rem;
		border-radius: 0.375rem;
		text-decoration: none;
	}

	.attachment-link {
		color: #4b5563;
		font-size: 0.75rem;
	}

	:global(.dark) .attachment-link {
		color: #9ca3af;
	}

	.attachment-link:hover {
		background: #f3f4f6;
	}

	:global(.dark) .attachment-link:hover {
		background: #374151;
	}

	.attachment-link span {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.attachment-add-button {
		width: 100%;
	}

	.danger-action-button {
		width: 100%;
	}

	.board-detail :global(.small-icon) {
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
	}

	.board-detail :global(.toolbar-icon) {
		width: 1.25rem;
		height: 1.25rem;
		flex-shrink: 0;
	}

	.board-detail :global(.meta-icon) {
		width: 0.875rem;
		height: 0.875rem;
		flex-shrink: 0;
	}

	.board-detail :global(.tiny-icon) {
		width: 0.75rem;
		height: 0.75rem;
	}

	@media (max-width: 900px) {
		.board-header-row,
		.board-toolbar,
		.filter-bar,
		.task-modal-layout {
			align-items: stretch;
			flex-direction: column;
		}

		.search-input,
		.search-input:focus {
			width: 100%;
		}

		.search-field,
		.task-modal-sidebar {
			width: 100%;
		}

		.task-modal-main {
			border-right: 0;
			border-bottom: 1px solid #e5e7eb;
		}

		:global(.dark) .task-modal-main {
			border-color: #374151;
		}
	}

	@keyframes board-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

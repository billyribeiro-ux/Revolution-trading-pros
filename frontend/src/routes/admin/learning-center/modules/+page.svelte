<script lang="ts">
	/**
	 * Admin Learning Center - Module Management
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Admin interface for managing learning center modules.
	 * Allows creating, editing, and organizing lesson modules by trading room.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { learningCenterStore, MODULES, TRADING_ROOMS } from '$lib/stores/learningCenter';
	import type { LessonModule, TradingRoom } from '$lib/types/learning-center';
	import { get } from 'svelte/store';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconChevronDown from '@tabler/icons-svelte/icons/chevron-down';

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let selectedRoom = $state<string>('');
	let showCreateModal = $state(false);
	let showEditModal = $state(false);
	let showDeleteModal = $state(false);
	let moduleToEdit = $state<LessonModule | null>(null);
	let moduleToDelete = $state<string | null>(null);

	// Form state
	let formName = $state('');
	let formSlug = $state('');
	let formDescription = $state('');
	let formRoomId = $state('');

	// ═══════════════════════════════════════════════════════════════════════════
	// STORE DATA
	// ═══════════════════════════════════════════════════════════════════════════

	let storeData = $derived(get(learningCenterStore));

	// Get trading rooms with learning centers
	let tradingRooms = $derived(
		storeData.tradingRooms.filter(r => r.hasLearningCenter && r.isActive)
	);

	// Get modules filtered by selected room
	let modules = $derived.by(() => {
		let filtered = storeData.modules;
		if (selectedRoom) {
			filtered = filtered.filter(m => m.tradingRoomId === selectedRoom);
		}
		return filtered.sort((a, b) => (a.sortOrder ?? 0) - (b.sortOrder ?? 0));
	});

	// Get lesson count per module
	function getLessonCount(moduleId: string): number {
		return storeData.lessons.filter(l => l.moduleId === moduleId).length;
	}

	// Get room name
	function getRoomName(roomId: string): string {
		const room = storeData.tradingRooms.find(r => r.id === roomId);
		return room?.shortName || room?.name || 'Unknown';
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function openCreateModal() {
		formName = '';
		formSlug = '';
		formDescription = '';
		formRoomId = selectedRoom || '';
		showCreateModal = true;
	}

	function openEditModal(module: LessonModule) {
		moduleToEdit = module;
		formName = module.name;
		formSlug = module.slug || '';
		formDescription = module.description || '';
		formRoomId = module.tradingRoomId || '';
		showEditModal = true;
	}

	function handleDeleteClick(moduleId: string) {
		moduleToDelete = moduleId;
		showDeleteModal = true;
	}

	async function handleCreateModule() {
		if (!formName || !formRoomId) return;

		try {
			// TODO: Implement create via API
			console.log('Create module:', { name: formName, slug: formSlug, description: formDescription, tradingRoomId: formRoomId });
		} catch (err) {
			console.error('Failed to create module:', err);
		}

		showCreateModal = false;
	}

	async function handleUpdateModule() {
		if (!moduleToEdit || !formName) return;

		try {
			// TODO: Implement update via API
			console.log('Update module:', { id: moduleToEdit.id, name: formName, slug: formSlug, description: formDescription });
		} catch (err) {
			console.error('Failed to update module:', err);
		}

		showEditModal = false;
		moduleToEdit = null;
	}

	async function handleConfirmDelete() {
		if (!moduleToDelete) return;

		try {
			// TODO: Implement delete via API
			console.log('Delete module:', moduleToDelete);
		} catch (err) {
			console.error('Failed to delete module:', err);
		}

		showDeleteModal = false;
		moduleToDelete = null;
	}

	function generateSlug(name: string): string {
		return name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '');
	}

	// Auto-generate slug from name
	$effect(() => {
		if (formName && !formSlug) {
			formSlug = generateSlug(formName);
		}
	});
</script>

<svelte:head>
	<title>Manage Modules | Learning Center | Admin</title>
</svelte:head>

<div class="admin-page">
	<!-- Header -->
	<header class="page-header">
		<div class="header-left">
			<a href="/admin/learning-center" class="back-link">
				<IconArrowLeft size={18} />
				Back to Learning Center
			</a>
			<h1>Manage Modules</h1>
			<p class="subtitle">Organize lessons into modules for each trading room</p>
		</div>
		<div class="header-actions">
			<button type="button" class="btn-primary" onclick={openCreateModal}>
				<IconPlus size={18} />
				Add Module
			</button>
		</div>
	</header>

	<!-- Filters -->
	<div class="filters-bar">
		<select bind:value={selectedRoom}>
			<option value="">All Trading Rooms</option>
			{#each tradingRooms as room}
				<option value={room.id}>{room.shortName || room.name}</option>
			{/each}
		</select>
	</div>

	<!-- Modules Grid -->
	<div class="modules-grid">
		{#each modules as module (module.id)}
			<div class="module-card">
				<div class="module-header">
					<div class="drag-handle">
						<IconGripVertical size={16} />
					</div>
					<div class="module-info">
						<h3>{module.name}</h3>
						<span class="room-badge">{getRoomName(module.tradingRoomId || '')}</span>
					</div>
					<div class="module-actions">
						<button type="button" class="action-btn" title="Edit" onclick={() => openEditModal(module)}>
							<IconEdit size={16} />
						</button>
						<button type="button" class="action-btn danger" title="Delete" onclick={() => handleDeleteClick(module.id)}>
							<IconTrash size={16} />
						</button>
					</div>
				</div>
				{#if module.description}
					<p class="module-description">{module.description}</p>
				{/if}
				<div class="module-meta">
					<span class="lesson-count">{getLessonCount(module.id || '')} lessons</span>
					<span class="sort-order">Order: {module.sortOrder}</span>
				</div>
			</div>
		{:else}
			<div class="empty-state">
				<p>No modules found</p>
				<button type="button" onclick={openCreateModal}>Create your first module</button>
			</div>
		{/each}
	</div>
</div>

<!-- Create Modal -->
{#if showCreateModal}
	<div class="modal-overlay" onclick={() => showCreateModal = false} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showCreateModal = false)} role="button" tabindex="-1" aria-label="Close modal">
		<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="0">
			<h3>Create Module</h3>
			<form onsubmit={(e: Event) => { e.preventDefault(); handleCreateModule(); }}>
				<div class="form-group">
					<label for="room">Trading Room</label>
					<select id="room" bind:value={formRoomId} required>
						<option value="">Select a room</option>
						{#each tradingRooms as room}
							<option value={room.id}>{room.shortName || room.name}</option>
						{/each}
					</select>
				</div>
				<div class="form-group">
					<label for="name">Module Name</label>
					<input type="text" id="name" bind:value={formName} placeholder="e.g., Getting Started" required />
				</div>
				<div class="form-group">
					<label for="slug">Slug</label>
					<input type="text" id="slug" bind:value={formSlug} placeholder="getting-started" />
				</div>
				<div class="form-group">
					<label for="description">Description</label>
					<textarea id="description" bind:value={formDescription} placeholder="Brief description of this module..." rows="3"></textarea>
				</div>
				<div class="modal-actions">
					<button type="button" class="btn-cancel" onclick={() => showCreateModal = false}>Cancel</button>
					<button type="submit" class="btn-primary">Create Module</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Edit Modal -->
{#if showEditModal}
	<div class="modal-overlay" onclick={() => showEditModal = false} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showEditModal = false)} role="button" tabindex="-1" aria-label="Close modal">
		<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="0">
			<h3>Edit Module</h3>
			<form onsubmit={(e: Event) => { e.preventDefault(); handleUpdateModule(); }}>
				<div class="form-group">
					<label for="edit-name">Module Name</label>
					<input type="text" id="edit-name" bind:value={formName} required />
				</div>
				<div class="form-group">
					<label for="edit-slug">Slug</label>
					<input type="text" id="edit-slug" bind:value={formSlug} />
				</div>
				<div class="form-group">
					<label for="edit-description">Description</label>
					<textarea id="edit-description" bind:value={formDescription} rows="3"></textarea>
				</div>
				<div class="modal-actions">
					<button type="button" class="btn-cancel" onclick={() => showEditModal = false}>Cancel</button>
					<button type="submit" class="btn-primary">Save Changes</button>
				</div>
			</form>
		</div>
	</div>
{/if}

<!-- Delete Modal -->
{#if showDeleteModal}
	<div class="modal-overlay" onclick={() => showDeleteModal = false} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showDeleteModal = false)} role="button" tabindex="-1" aria-label="Close modal">
		<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="0">
			<h3>Delete Module?</h3>
			<p>This will remove the module. Any lessons in this module will become unassigned.</p>
			<div class="modal-actions">
				<button type="button" class="btn-cancel" onclick={() => showDeleteModal = false}>Cancel</button>
				<button type="button" class="btn-delete" onclick={handleConfirmDelete}>Delete</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.admin-page {
		padding: 32px;
		max-width: 1200px;
		margin: 0 auto;
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 32px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.875rem;
		margin-bottom: 8px;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #f97316;
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

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	/* Filters */
	.filters-bar {
		margin-bottom: 24px;
	}

	.filters-bar select {
		padding: 10px 16px;
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 8px;
		color: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	/* Modules Grid */
	.modules-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 16px;
	}

	.module-card {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
		padding: 20px;
		transition: all 0.2s;
	}

	.module-card:hover {
		border-color: #475569;
	}

	.module-header {
		display: flex;
		align-items: flex-start;
		gap: 12px;
	}

	.drag-handle {
		color: #475569;
		cursor: grab;
		padding: 4px;
		margin-top: 2px;
	}

	.module-info {
		flex: 1;
		min-width: 0;
	}

	.module-info h3 {
		margin: 0 0 4px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.room-badge {
		display: inline-block;
		padding: 2px 8px;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #f97316;
	}

	.module-actions {
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
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #334155;
		color: white;
	}

	.action-btn.danger:hover {
		background: rgba(239, 68, 68, 0.1);
		color: #ef4444;
	}

	.module-description {
		margin: 12px 0 0;
		font-size: 0.8rem;
		color: #94a3b8;
		line-height: 1.5;
	}

	.module-meta {
		display: flex;
		gap: 16px;
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid #334155;
	}

	.lesson-count, .sort-order {
		font-size: 0.75rem;
		color: #64748b;
	}

	/* Empty State */
	.empty-state {
		grid-column: 1 / -1;
		text-align: center;
		padding: 60px 20px;
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
		max-width: 480px;
		width: 90%;
	}

	.modal h3 {
		margin: 0 0 20px;
		font-size: 1.125rem;
		color: white;
	}

	.form-group {
		margin-bottom: 16px;
	}

	.form-group label {
		display: block;
		margin-bottom: 6px;
		font-size: 0.8rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 10px 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 6px;
		color: white;
		font-size: 0.875rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #f97316;
	}

	.modal-actions {
		display: flex;
		gap: 12px;
		justify-content: flex-end;
		margin-top: 24px;
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
	@media (max-width: 768px) {
		.admin-page {
			padding: 16px;
		}

		.page-header {
			flex-direction: column;
			gap: 16px;
		}

		.modules-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

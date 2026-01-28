<script lang="ts">
	/**
	 * Bulk Edit Modal - Revolution Trading Pros
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { videoOpsApi } from '$lib/api/video-advanced';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconMinus from '@tabler/icons-svelte/icons/minus';

	interface Props {
		selectedIds: number[];
		contentTypes?: { value: string; label: string }[];
		traders?: { id: number; name: string }[];
		rooms?: { id: number; name: string }[];
		tags?: { slug: string; name: string; color: string }[];
		onSaved?: () => void;
		onClose?: () => void;
	}

	let {
		selectedIds,
		contentTypes = [],
		traders = [],
		rooms = [],
		tags = [],
		onSaved,
		onClose
	}: Props = $props();

	let isSaving = $state(false);
	let error = $state('');

	// What to update flags
	let updateContentType = $state(false);
	let updateTrader = $state(false);
	let updatePublished = $state(false);
	let updateFeatured = $state(false);
	let updateDifficulty = $state(false);

	// Values
	let contentType = $state('');
	let traderId = $state<number | null>(null);
	let isPublished = $state(true);
	let isFeatured = $state(false);
	let difficultyLevel = $state('');

	// Tags
	let addTags = $state<string[]>([]);
	let removeTags = $state<string[]>([]);

	// Rooms
	let addRoomIds = $state<number[]>([]);
	let removeRoomIds = $state<number[]>([]);

	const difficulties = [
		{ value: 'beginner', label: 'Beginner' },
		{ value: 'intermediate', label: 'Intermediate' },
		{ value: 'advanced', label: 'Advanced' }
	];

	async function saveBulkEdit() {
		const updates: any = {};

		if (updateContentType && contentType) updates.content_type = contentType;
		if (updateTrader) updates.trader_id = traderId;
		if (updatePublished) updates.is_published = isPublished;
		if (updateFeatured) updates.is_featured = isFeatured;
		if (updateDifficulty && difficultyLevel) updates.difficulty_level = difficultyLevel;
		if (addTags.length > 0) updates.add_tags = addTags;
		if (removeTags.length > 0) updates.remove_tags = removeTags;
		if (addRoomIds.length > 0) updates.add_room_ids = addRoomIds;
		if (removeRoomIds.length > 0) updates.remove_room_ids = removeRoomIds;

		if (Object.keys(updates).length === 0) {
			error = 'No changes selected';
			return;
		}

		isSaving = true;
		error = '';

		const result = await videoOpsApi.bulkEdit({
			video_ids: selectedIds,
			updates
		});

		if (result.success) {
			if (onSaved) onSaved();
			if (onClose) onClose();
		} else {
			error = result.error || 'Failed to update videos';
		}

		isSaving = false;
	}

	function toggleAddTag(slug: string) {
		if (addTags.includes(slug)) {
			addTags = addTags.filter((t) => t !== slug);
		} else {
			addTags = [...addTags, slug];
			// Remove from removeTags if present
			removeTags = removeTags.filter((t) => t !== slug);
		}
	}

	function toggleRemoveTag(slug: string) {
		if (removeTags.includes(slug)) {
			removeTags = removeTags.filter((t) => t !== slug);
		} else {
			removeTags = [...removeTags, slug];
			// Remove from addTags if present
			addTags = addTags.filter((t) => t !== slug);
		}
	}

	function toggleAddRoom(id: number) {
		if (addRoomIds.includes(id)) {
			addRoomIds = addRoomIds.filter((r) => r !== id);
		} else {
			addRoomIds = [...addRoomIds, id];
			removeRoomIds = removeRoomIds.filter((r) => r !== id);
		}
	}

	function toggleRemoveRoom(id: number) {
		if (removeRoomIds.includes(id)) {
			removeRoomIds = removeRoomIds.filter((r) => r !== id);
		} else {
			removeRoomIds = [...removeRoomIds, id];
			addRoomIds = addRoomIds.filter((r) => r !== id);
		}
	}
</script>

<div class="bulk-edit-modal">
	<div class="modal-header">
		<div class="header-left">
			<IconEdit size={24} />
			<h3>Bulk Edit {selectedIds.length} Videos</h3>
		</div>
		{#if onClose}
			<button type="button" class="btn-close" onclick={onClose}>
				<IconX size={20} />
			</button>
		{/if}
	</div>

	{#if error}
		<div class="error-message">{error}</div>
	{/if}

	<div class="edit-sections">
		<!-- Content Type -->
		<div class="edit-section">
			<label class="section-toggle">
				<input type="checkbox" bind:checked={updateContentType} />
				<span>Change Content Type</span>
			</label>
			{#if updateContentType}
				<select bind:value={contentType}>
					<option value="">Select content type...</option>
					{#each contentTypes as ct (ct.value)}
						<option value={ct.value}>{ct.label}</option>
					{/each}
				</select>
			{/if}
		</div>

		<!-- Trader -->
		<div class="edit-section">
			<label class="section-toggle">
				<input type="checkbox" bind:checked={updateTrader} />
				<span>Change Trader</span>
			</label>
			{#if updateTrader}
				<select bind:value={traderId}>
					<option value={null}>No trader</option>
					{#each traders as trader (trader.id)}
						<option value={trader.id}>{trader.name}</option>
					{/each}
				</select>
			{/if}
		</div>

		<!-- Published -->
		<div class="edit-section">
			<label class="section-toggle">
				<input type="checkbox" bind:checked={updatePublished} />
				<span>Change Published Status</span>
			</label>
			{#if updatePublished}
				<div class="radio-group">
					<label>
						<input type="radio" bind:group={isPublished} value={true} />
						Published
					</label>
					<label>
						<input type="radio" bind:group={isPublished} value={false} />
						Unpublished
					</label>
				</div>
			{/if}
		</div>

		<!-- Featured -->
		<div class="edit-section">
			<label class="section-toggle">
				<input type="checkbox" bind:checked={updateFeatured} />
				<span>Change Featured Status</span>
			</label>
			{#if updateFeatured}
				<div class="radio-group">
					<label>
						<input type="radio" bind:group={isFeatured} value={true} />
						Featured
					</label>
					<label>
						<input type="radio" bind:group={isFeatured} value={false} />
						Not Featured
					</label>
				</div>
			{/if}
		</div>

		<!-- Difficulty -->
		<div class="edit-section">
			<label class="section-toggle">
				<input type="checkbox" bind:checked={updateDifficulty} />
				<span>Change Difficulty Level</span>
			</label>
			{#if updateDifficulty}
				<select bind:value={difficultyLevel}>
					<option value="">Select difficulty...</option>
					{#each difficulties as d (d.value)}
						<option value={d.value}>{d.label}</option>
					{/each}
				</select>
			{/if}
		</div>

		<!-- Tags -->
		{#if tags.length > 0}
			<div class="edit-section tags-section">
				<h4>Modify Tags</h4>
				<div class="tags-grid">
					{#each tags as tag (tag.slug)}
						<div class="tag-item">
							<span class="tag-name" style="--tag-color: {tag.color}">{tag.name}</span>
							<div class="tag-actions">
								<button
									type="button"
									class="btn-tag-action add"
									class:active={addTags.includes(tag.slug)}
									onclick={() => toggleAddTag(tag.slug)}
									title="Add tag"
								>
									<IconPlus size={14} />
								</button>
								<button
									type="button"
									class="btn-tag-action remove"
									class:active={removeTags.includes(tag.slug)}
									onclick={() => toggleRemoveTag(tag.slug)}
									title="Remove tag"
								>
									<IconMinus size={14} />
								</button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Rooms -->
		{#if rooms.length > 0}
			<div class="edit-section rooms-section">
				<h4>Modify Room Assignments</h4>
				<div class="rooms-grid">
					{#each rooms as room (room.id)}
						<div class="room-item">
							<span class="room-name">{room.name}</span>
							<div class="room-actions">
								<button
									type="button"
									class="btn-room-action add"
									class:active={addRoomIds.includes(room.id)}
									onclick={() => toggleAddRoom(room.id)}
									title="Add to room"
								>
									<IconPlus size={14} />
								</button>
								<button
									type="button"
									class="btn-room-action remove"
									class:active={removeRoomIds.includes(room.id)}
									onclick={() => toggleRemoveRoom(room.id)}
									title="Remove from room"
								>
									<IconMinus size={14} />
								</button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>

	<div class="modal-actions">
		<button type="button" class="btn-cancel" onclick={onClose}>Cancel</button>
		<button type="button" class="btn-save" onclick={saveBulkEdit} disabled={isSaving}>
			<IconCheck size={16} />
			{isSaving ? 'Saving...' : `Update ${selectedIds.length} Videos`}
		</button>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   BULK EDIT MODAL - ICT 7 Principal Engineer Grade
	   ═══════════════════════════════════════════════════════════════════════════ */

	.bulk-edit-modal {
		background: var(--admin-surface-primary);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		max-width: 500px;
		width: 100%;
		max-height: 80vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: var(--space-6);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		color: var(--admin-text-primary);
	}

	.header-left h3 {
		margin: 0;
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--admin-text-primary);
	}

	.btn-close {
		background: transparent;
		border: none;
		color: var(--admin-text-muted);
		cursor: pointer;
		padding: var(--space-2);
		border-radius: var(--radius-sm);
		transition: var(--transition-all);
		min-width: 44px;
		min-height: 44px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.btn-close:hover {
		background: var(--admin-surface-hover);
		color: var(--admin-text-primary);
	}

	.error-message {
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss);
		color: var(--color-loss);
		padding: var(--space-3);
		border-radius: var(--radius-md);
		margin-bottom: var(--space-4);
		font-size: var(--text-sm);
	}

	.edit-sections {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.edit-section {
		background: var(--admin-surface-sunken);
		padding: var(--space-4);
		border-radius: var(--radius-md);
	}

	.section-toggle {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
		cursor: pointer;
		color: var(--admin-text-primary);
		min-height: 44px;
	}

	.section-toggle input {
		accent-color: var(--admin-accent-primary);
		width: 18px;
		height: 18px;
	}

	.edit-section select {
		width: 100%;
		margin-top: var(--space-3);
		padding: var(--space-3);
		background: var(--admin-surface-primary);
		border: 1px solid var(--admin-border-subtle);
		border-radius: var(--radius-md);
		color: var(--admin-text-primary);
		font-size: var(--text-sm);
		transition: var(--transition-colors);
		min-height: 44px;
	}

	.edit-section select:focus {
		outline: none;
		border-color: var(--admin-accent-primary);
		box-shadow: var(--admin-focus-ring);
	}

	.radio-group {
		display: flex;
		gap: var(--space-4);
		margin-top: var(--space-3);
	}

	.radio-group label {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
		cursor: pointer;
		color: var(--admin-text-primary);
		min-height: 44px;
	}

	.radio-group input {
		accent-color: var(--admin-accent-primary);
		width: 18px;
		height: 18px;
	}

	.tags-section h4,
	.rooms-section h4 {
		margin: 0 0 var(--space-3);
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		color: var(--admin-text-muted);
	}

	.tags-grid,
	.rooms-grid {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}

	.tag-item,
	.room-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-2);
		background: var(--admin-surface-primary);
		border-radius: var(--radius-md);
		font-size: var(--text-xs);
		color: var(--admin-text-primary);
	}

	.tag-name {
		padding: var(--space-1) var(--space-2);
		background: var(--tag-color, var(--admin-accent-primary));
		border-radius: var(--radius-sm);
		color: var(--color-text-primary);
	}

	.tag-actions,
	.room-actions {
		display: flex;
		gap: var(--space-1);
	}

	.btn-tag-action,
	.btn-room-action {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		background: var(--admin-surface-sunken);
		border: none;
		border-radius: var(--radius-sm);
		color: var(--admin-text-muted);
		cursor: pointer;
		transition: var(--transition-all);
	}

	.btn-tag-action:hover,
	.btn-room-action:hover {
		background: var(--admin-surface-hover);
	}

	.btn-tag-action.add.active,
	.btn-room-action.add.active {
		background: var(--color-profit);
		color: var(--color-bg-card);
	}

	.btn-tag-action.remove.active,
	.btn-room-action.remove.active {
		background: var(--color-loss);
		color: var(--color-bg-card);
	}

	.modal-actions {
		display: flex;
		justify-content: flex-end;
		gap: var(--space-3);
		margin-top: var(--space-6);
		padding-top: var(--space-4);
		border-top: 1px solid var(--admin-border-subtle);
	}

	.btn-cancel {
		padding: var(--space-3) var(--space-4);
		background: transparent;
		border: 1px solid var(--admin-border-subtle);
		color: var(--admin-text-secondary);
		border-radius: var(--radius-md);
		cursor: pointer;
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		transition: var(--transition-all);
		min-height: 44px;
	}

	.btn-cancel:hover {
		background: var(--admin-surface-hover);
		border-color: var(--admin-border-light);
	}

	.btn-save {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		background: var(--admin-accent-primary);
		color: var(--color-text-primary);
		border: none;
		border-radius: var(--radius-md);
		cursor: pointer;
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		transition: var(--transition-all);
		min-height: 44px;
	}

	.btn-save:hover:not(:disabled) {
		background: var(--admin-accent-primary-hover);
	}

	.btn-save:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - Mobile (< sm: 640px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.bulk-edit-modal {
			padding: var(--space-4);
			max-height: 90vh;
		}

		.modal-actions {
			flex-direction: column;
		}

		.btn-cancel,
		.btn-save {
			width: 100%;
			justify-content: center;
		}

		.radio-group {
			flex-direction: column;
			gap: var(--space-2);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.btn-close,
		.btn-cancel,
		.btn-save,
		.btn-tag-action,
		.btn-room-action,
		.edit-section select {
			transition: none;
		}
	}
</style>

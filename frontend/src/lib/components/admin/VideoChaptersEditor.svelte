<script lang="ts">
	/**
	 * Video Chapters Editor - Revolution Trading Pros
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { chaptersApi, parseTimeToSeconds, type VideoChapter } from '$lib/api/video-advanced';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconEdit from '@tabler/icons-svelte-runes/icons/edit';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconGripVertical from '@tabler/icons-svelte-runes/icons/grip-vertical';

	interface Props {
		videoId: number;
		videoDuration?: number;
		onClose?: () => void;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const videoId = $derived(props.videoId);
	const _videoDuration = $derived(props.videoDuration ?? 0);
	const onClose = $derived(props.onClose);

	let chapters = $state<VideoChapter[]>([]);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let editingId = $state<number | null>(null);

	// New chapter form
	let newChapter = $state({
		title: '',
		description: '',
		startTime: '',
		endTime: ''
	});

	// Edit form
	let editForm = $state({
		title: '',
		description: '',
		startTime: '',
		endTime: ''
	});

	$effect(() => {
		loadChapters();
	});

	async function loadChapters() {
		isLoading = true;
		error = '';

		const result = await chaptersApi.list(videoId);

		if (result.success && result.data) {
			chapters = result.data;
		} else {
			error = result.error || 'Failed to load chapters';
		}

		isLoading = false;
	}

	async function addChapter() {
		if (!newChapter.title || !newChapter.startTime) {
			error = 'Title and start time are required';
			return;
		}

		const startSeconds = parseTimeToSeconds(newChapter.startTime);
		if (startSeconds === null) {
			error = 'Invalid start time format. Use MM:SS or HH:MM:SS';
			return;
		}

		const endSeconds = newChapter.endTime
			? (parseTimeToSeconds(newChapter.endTime) ?? undefined)
			: undefined;

		isSaving = true;
		error = '';

		const result = await chaptersApi.create(videoId, {
			title: newChapter.title,
			description: newChapter.description || undefined,
			start_time_seconds: startSeconds,
			end_time_seconds: endSeconds
		});

		if (result.success) {
			newChapter = { title: '', description: '', startTime: '', endTime: '' };
			await loadChapters();
		} else {
			error = result.error || 'Failed to add chapter';
		}

		isSaving = false;
	}

	function startEditing(chapter: VideoChapter) {
		editingId = chapter.id;
		editForm = {
			title: chapter.title,
			description: chapter.description || '',
			startTime: chapter.formatted_start_time,
			endTime: chapter.formatted_end_time || ''
		};
	}

	function cancelEditing() {
		editingId = null;
		editForm = { title: '', description: '', startTime: '', endTime: '' };
	}

	async function saveEdit(chapterId: number) {
		const startSeconds = parseTimeToSeconds(editForm.startTime);
		if (startSeconds === null) {
			error = 'Invalid start time format';
			return;
		}

		const endSeconds = editForm.endTime
			? (parseTimeToSeconds(editForm.endTime) ?? undefined)
			: undefined;

		isSaving = true;
		error = '';

		const result = await chaptersApi.update(videoId, chapterId, {
			title: editForm.title,
			description: editForm.description || undefined,
			start_time_seconds: startSeconds,
			end_time_seconds: endSeconds
		});

		if (result.success) {
			editingId = null;
			await loadChapters();
		} else {
			error = result.error || 'Failed to update chapter';
		}

		isSaving = false;
	}

	async function deleteChapter(chapterId: number) {
		if (!confirm('Are you sure you want to delete this chapter?')) return;

		const result = await chaptersApi.delete(videoId, chapterId);

		if (result.success) {
			await loadChapters();
		} else {
			error = result.error || 'Failed to delete chapter';
		}
	}

	async function parseFromDescription(text: string) {
		// Parse chapters from YouTube-style timestamp format
		// e.g., "0:00 Introduction\n1:30 Main Topic\n5:45 Conclusion"
		const lines = text.split('\n');
		const parsedChapters: { title: string; start_time_seconds: number }[] = [];

		for (const line of lines) {
			const match = line.match(/^(\d{1,2}:\d{2}(?::\d{2})?)\s+(.+)$/);
			if (match) {
				const seconds = parseTimeToSeconds(match[1]);
				if (seconds !== null) {
					parsedChapters.push({
						title: match[2].trim(),
						start_time_seconds: seconds
					});
				}
			}
		}

		if (parsedChapters.length > 0) {
			isSaving = true;
			const result = await chaptersApi.bulkCreate(videoId, parsedChapters);

			if (result.success) {
				await loadChapters();
			} else {
				error = result.error || 'Failed to import chapters';
			}
			isSaving = false;
		} else {
			error = 'No valid chapters found in text';
		}
	}
</script>

<div class="chapters-editor">
	<div class="editor-header">
		<h3>Video Chapters</h3>
		{#if onClose}
			<button type="button" class="btn-close" onclick={onClose}>
				<IconX size={20} />
			</button>
		{/if}
	</div>

	{#if error}
		<div class="error-message">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading chapters...</div>
	{:else}
		<!-- Existing Chapters -->
		<div class="chapters-list">
			{#each chapters as chapter (chapter.id)}
				<div class="chapter-item" class:editing={editingId === chapter.id}>
					{#if editingId === chapter.id}
						<div class="chapter-edit-form">
							<input
								type="text"
								bind:value={editForm.title}
								placeholder="Chapter title"
								class="input-title"
							/>
							<div class="time-inputs">
								<input
									type="text"
									bind:value={editForm.startTime}
									placeholder="Start (MM:SS)"
									class="input-time"
								/>
								<span>-</span>
								<input
									type="text"
									bind:value={editForm.endTime}
									placeholder="End (optional)"
									class="input-time"
								/>
							</div>
							<textarea
								bind:value={editForm.description}
								placeholder="Description (optional)"
								rows="2"
							></textarea>
							<div class="edit-actions">
								<button
									type="button"
									class="btn-save"
									onclick={() => saveEdit(chapter.id)}
									disabled={isSaving}
								>
									<IconCheck size={16} /> Save
								</button>
								<button type="button" class="btn-cancel" onclick={cancelEditing}>
									<IconX size={16} /> Cancel
								</button>
							</div>
						</div>
					{:else}
						<div class="chapter-grip">
							<IconGripVertical size={16} />
						</div>
						<div class="chapter-number">{chapter.chapter_number}</div>
						<div class="chapter-time">{chapter.formatted_start_time}</div>
						<div class="chapter-info">
							<div class="chapter-title">{chapter.title}</div>
							{#if chapter.description}
								<div class="chapter-desc">{chapter.description}</div>
							{/if}
						</div>
						<div class="chapter-actions">
							<button
								type="button"
								class="btn-icon"
								onclick={() => startEditing(chapter)}
								title="Edit"
							>
								<IconEdit size={16} />
							</button>
							<button
								type="button"
								class="btn-icon danger"
								onclick={() => deleteChapter(chapter.id)}
								title="Delete"
							>
								<IconTrash size={16} />
							</button>
						</div>
					{/if}
				</div>
			{/each}
		</div>

		<!-- Add New Chapter -->
		<div class="add-chapter-form">
			<h4>Add New Chapter</h4>
			<div class="form-row">
				<input
					type="text"
					bind:value={newChapter.title}
					placeholder="Chapter title"
					class="input-title"
				/>
			</div>
			<div class="form-row time-row">
				<input
					type="text"
					bind:value={newChapter.startTime}
					placeholder="Start time (MM:SS)"
					class="input-time"
				/>
				<input
					type="text"
					bind:value={newChapter.endTime}
					placeholder="End time (optional)"
					class="input-time"
				/>
			</div>
			<div class="form-row">
				<textarea bind:value={newChapter.description} placeholder="Description (optional)" rows="2"
				></textarea>
			</div>
			<button
				type="button"
				class="btn-add"
				onclick={addChapter}
				disabled={isSaving || !newChapter.title || !newChapter.startTime}
			>
				<IconPlus size={16} /> Add Chapter
			</button>
		</div>

		<!-- Bulk Import -->
		<div class="bulk-import">
			<h4>Import from Text</h4>
			<p class="import-hint">Paste timestamps in YouTube format: "0:00 Introduction"</p>
			<textarea
				id="bulk-import-text"
				placeholder="0:00 Introduction&#10;1:30 Main Topic&#10;5:45 Conclusion"
				rows="4"
			></textarea>
			<button
				type="button"
				class="btn-import"
				onclick={() => {
					const textarea = document.getElementById('bulk-import-text') as HTMLTextAreaElement;
					parseFromDescription(textarea.value);
				}}
				disabled={isSaving}
			>
				Import Chapters
			</button>
		</div>
	{/if}
</div>

<style>
	.chapters-editor {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 600px;
	}

	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.editor-header h3 {
		margin: 0;
		font-size: 1.25rem;
	}

	.btn-close {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.25rem;
	}

	.error-message {
		background: #ef44441a;
		border: 1px solid #ef4444;
		color: #ef4444;
		padding: 0.75rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.loading {
		text-align: center;
		padding: 2rem;
		color: var(--text-secondary);
	}

	.chapters-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
	}

	.chapter-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.chapter-item.editing {
		flex-direction: column;
		align-items: stretch;
	}

	.chapter-grip {
		color: var(--text-secondary);
		cursor: grab;
	}

	.chapter-number {
		min-width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--primary, #e6b800);
		color: #0d1117;
		border-radius: 50%;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.chapter-time {
		font-family: monospace;
		font-size: 0.875rem;
		color: var(--text-secondary);
		min-width: 60px;
	}

	.chapter-info {
		flex: 1;
		min-width: 0;
	}

	.chapter-title {
		font-weight: 500;
	}

	.chapter-desc {
		font-size: 0.875rem;
		color: var(--text-secondary);
		margin-top: 0.25rem;
	}

	.chapter-actions {
		display: flex;
		gap: 0.25rem;
	}

	.btn-icon {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.375rem;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: var(--bg-hover, #ffffff1a);
		color: var(--text-primary, white);
	}

	.btn-icon.danger:hover {
		color: #ef4444;
	}

	.chapter-edit-form {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		width: 100%;
	}

	.time-inputs {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.input-title,
	.input-time,
	textarea {
		background: var(--bg-primary, #0f0f1a);
		border: 1px solid var(--border-color, #333);
		color: var(--text-primary, white);
		padding: 0.5rem 0.75rem;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.input-title {
		width: 100%;
	}

	.input-time {
		width: 100px;
	}

	textarea {
		width: 100%;
		resize: vertical;
	}

	.edit-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-save,
	.btn-cancel {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.5rem 0.75rem;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-save {
		background: var(--primary, #e6b800);
		color: #0d1117;
	}

	.btn-cancel {
		background: var(--bg-tertiary, #252542);
		color: var(--text-secondary);
	}

	.add-chapter-form,
	.bulk-import {
		background: var(--bg-tertiary, #252542);
		padding: 1rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.add-chapter-form h4,
	.bulk-import h4 {
		margin: 0 0 0.75rem;
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.form-row {
		margin-bottom: 0.75rem;
	}

	.time-row {
		display: flex;
		gap: 0.5rem;
	}

	.btn-add,
	.btn-import {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		width: 100%;
		padding: 0.625rem;
		background: var(--primary, #e6b800);
		color: #0d1117;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.875rem;
		transition: opacity 0.2s;
	}

	.btn-add:disabled,
	.btn-import:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.import-hint {
		font-size: 0.75rem;
		color: var(--text-secondary);
		margin-bottom: 0.5rem;
	}
</style>

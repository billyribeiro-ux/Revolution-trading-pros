<script lang="ts">
	/**
	 * Admin Learning Center - Edit Lesson
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Admin interface for editing existing learning center lessons.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { learningCenterStore } from '$lib/stores/learningCenter';
	import type { LessonWithRelations, TradingRoom, Trainer, LessonCategory, LessonModule } from '$lib/types/learning-center';
	import { get } from 'svelte/store';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import IconDeviceFloppy from '@tabler/icons-svelte/icons/device-floppy';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconPhoto from '@tabler/icons-svelte/icons/photo';
	import IconVideo from '@tabler/icons-svelte/icons/video';

	// ═══════════════════════════════════════════════════════════════════════════
	// PROPS & STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const lessonId = $derived($page.params.id!);

	// Store data
	let storeData = $derived(get(learningCenterStore));

	// Get lesson from store
	let lesson = $derived.by((): LessonWithRelations | undefined => {
		const found = storeData.lessons.find(l => l.id === lessonId);
		if (!found) return undefined;
		return {
			...found,
			trainer: storeData.trainers.find(t => t.id === found.trainerId),
			category: storeData.categories.find(c => c.id === found.categoryId),
			tradingRooms: storeData.tradingRooms.filter(r => found.tradingRoomIds?.includes(r.id)),
			module: storeData.modules.find(m => m.id === found.moduleId)
		};
	});

	// Form state
	let formTitle = $state('');
	let formSlug = $state('');
	let formDescription = $state('');
	let formFullDescription = $state('');
	let formType = $state<'video' | 'article'>('video');
	let formVideoUrl = $state('');
	let formThumbnailUrl = $state('');
	let formDuration = $state('');
	let formTrainerId = $state('');
	let formCategoryId = $state('');
	let formModuleId = $state('');
	let formTradingRoomIds = $state<string[]>([]);
	let formStatus = $state<'draft' | 'published' | 'archived'>('draft');
	let formIsFeatured = $state(false);
	let formIsPinned = $state(false);
	let formTags = $state('');

	let isLoading = $state(false);
	let isSaving = $state(false);
	let showDeleteModal = $state(false);

	// Reference data
	let tradingRooms = $derived(storeData.tradingRooms.filter(r => r.hasLearningCenter && r.isActive));
	let trainers = $derived(storeData.trainers.filter(t => t.isActive));
	let categories = $derived(storeData.categories.filter(c => c.isActive));
	let modules = $derived(storeData.modules);

	// Initialize form when lesson loads
	$effect(() => {
		if (lesson) {
			formTitle = lesson.title;
			formSlug = lesson.slug;
			formDescription = lesson.description;
			formFullDescription = lesson.fullDescription || '';
			formType = lesson.type as 'video' | 'article';
			formVideoUrl = lesson.videoUrl || '';
			formThumbnailUrl = lesson.thumbnailUrl || '';
			formDuration = lesson.duration || '';
			formTrainerId = lesson.trainerId || '';
			formCategoryId = lesson.categoryId || '';
			formModuleId = lesson.moduleId || '';
			formTradingRoomIds = lesson.tradingRoomIds || [];
			formStatus = lesson.status as 'draft' | 'published' | 'archived';
			formIsFeatured = lesson.isFeatured || false;
			formIsPinned = lesson.isPinned || false;
			formTags = lesson.tags?.join(', ') || '';
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// EVENT HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════

	function generateSlug(title: string): string {
		return title.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/(^-|-$)/g, '');
	}

	function handleTitleChange() {
		if (formTitle && !formSlug) {
			formSlug = generateSlug(formTitle);
		}
	}

	function toggleRoom(roomId: string) {
		if (formTradingRoomIds.includes(roomId)) {
			formTradingRoomIds = formTradingRoomIds.filter(id => id !== roomId);
		} else {
			formTradingRoomIds = [...formTradingRoomIds, roomId];
		}
	}

	async function handleSave() {
		if (!formTitle || formTradingRoomIds.length === 0) {
			alert('Please fill in required fields');
			return;
		}

		isSaving = true;

		try {
			// TODO: Implement save via API
			const updateData = {
				id: lessonId,
				title: formTitle,
				slug: formSlug,
				description: formDescription,
				fullDescription: formFullDescription,
				type: formType,
				videoUrl: formVideoUrl,
				thumbnailUrl: formThumbnailUrl,
				duration: formDuration,
				trainerId: formTrainerId,
				categoryId: formCategoryId,
				moduleId: formModuleId,
				tradingRoomIds: formTradingRoomIds,
				status: formStatus,
				isFeatured: formIsFeatured,
				isPinned: formIsPinned,
				tags: formTags.split(',').map(t => t.trim()).filter(Boolean)
			};

			console.log('Saving lesson:', updateData);

			// Simulate API delay
			await new Promise(resolve => setTimeout(resolve, 500));

			goto('/admin/learning-center');
		} catch (err) {
			console.error('Failed to save lesson:', err);
			alert('Failed to save lesson');
		} finally {
			isSaving = false;
		}
	}

	async function handleDelete() {
		try {
			// TODO: Implement delete via API
			console.log('Delete lesson:', lessonId);
			goto('/admin/learning-center');
		} catch (err) {
			console.error('Failed to delete lesson:', err);
		}
		showDeleteModal = false;
	}
</script>

<svelte:head>
	<title>Edit Lesson | Learning Center | Admin</title>
</svelte:head>

<div class="admin-page">
	{#if !lesson}
		<div class="not-found">
			<h2>Lesson Not Found</h2>
			<p>The lesson you're looking for doesn't exist.</p>
			<a href="/admin/learning-center">Back to Learning Center</a>
		</div>
	{:else}
		<!-- Header -->
		<header class="page-header">
			<div class="header-left">
				<a href="/admin/learning-center" class="back-link">
					<IconArrowLeft size={18} />
					Back to Learning Center
				</a>
				<h1>Edit Lesson</h1>
			</div>
			<div class="header-actions">
				<a
					href="/dashboard/{lesson.tradingRooms?.[0]?.slug || 'day-trading-room'}/learning-center/{lesson.slug}"
					class="btn-secondary"
					target="_blank"
				>
					<IconEye size={18} />
					Preview
				</a>
				<button type="button" class="btn-danger" onclick={() => showDeleteModal = true}>
					<IconTrash size={18} />
					Delete
				</button>
				<button type="button" class="btn-primary" onclick={handleSave} disabled={isSaving}>
					<IconDeviceFloppy size={18} />
					{isSaving ? 'Saving...' : 'Save Changes'}
				</button>
			</div>
		</header>

		<!-- Form -->
		<div class="form-layout">
			<!-- Main Content -->
			<div class="form-main">
				<!-- Basic Info -->
				<section class="form-section">
					<h2>Basic Information</h2>

					<div class="form-group">
						<label for="title">Title <span class="required">*</span></label>
						<input
							type="text"
							id="title"
							bind:value={formTitle}
							oninput={handleTitleChange}
							placeholder="Lesson title"
							required
						/>
					</div>

					<div class="form-group">
						<label for="slug">Slug</label>
						<input
							type="text"
							id="slug"
							bind:value={formSlug}
							placeholder="lesson-slug"
						/>
					</div>

					<div class="form-group">
						<label for="description">Short Description</label>
						<textarea
							id="description"
							bind:value={formDescription}
							placeholder="Brief description for listings..."
							rows="2"
						></textarea>
					</div>

					<div class="form-group">
						<label for="fullDescription">Full Description</label>
						<textarea
							id="fullDescription"
							bind:value={formFullDescription}
							placeholder="Detailed lesson description..."
							rows="5"
						></textarea>
					</div>
				</section>

				<!-- Media -->
				<section class="form-section">
					<h2>Media</h2>

					<div class="form-row">
						<div class="form-group">
							<label for="type">Content Type</label>
							<select id="type" bind:value={formType}>
								<option value="video">Video</option>
								<option value="article">Article</option>
							</select>
						</div>

						<div class="form-group">
							<label for="duration">Duration</label>
							<input
								type="text"
								id="duration"
								bind:value={formDuration}
								placeholder="e.g., 45:00"
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="videoUrl">
							{formType === 'video' ? 'Video URL' : 'Content URL'}
						</label>
						<div class="input-with-icon">
							<IconVideo size={18} />
							<input
								type="url"
								id="videoUrl"
								bind:value={formVideoUrl}
								placeholder="https://..."
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="thumbnailUrl">Thumbnail URL</label>
						<div class="input-with-icon">
							<IconPhoto size={18} />
							<input
								type="url"
								id="thumbnailUrl"
								bind:value={formThumbnailUrl}
								placeholder="https://..."
							/>
						</div>
						{#if formThumbnailUrl}
							<div class="thumbnail-preview">
								<img src={formThumbnailUrl} alt="Thumbnail preview" />
							</div>
						{/if}
					</div>
				</section>

				<!-- Tags -->
				<section class="form-section">
					<h2>Tags</h2>
					<div class="form-group">
						<label for="tags">Tags (comma-separated)</label>
						<input
							type="text"
							id="tags"
							bind:value={formTags}
							placeholder="e.g., options, beginner, strategy"
						/>
					</div>
				</section>
			</div>

			<!-- Sidebar -->
			<div class="form-sidebar">
				<!-- Status -->
				<section class="sidebar-section">
					<h3>Status</h3>
					<div class="form-group">
						<select bind:value={formStatus}>
							<option value="draft">Draft</option>
							<option value="published">Published</option>
							<option value="archived">Archived</option>
						</select>
					</div>

					<div class="checkbox-group">
						<label>
							<input type="checkbox" bind:checked={formIsFeatured} />
							Featured
						</label>
						<label>
							<input type="checkbox" bind:checked={formIsPinned} />
							Pinned
						</label>
					</div>
				</section>

				<!-- Trading Rooms -->
				<section class="sidebar-section">
					<h3>Trading Rooms <span class="required">*</span></h3>
					<div class="room-checkboxes">
						{#each tradingRooms as room}
							<label class="room-checkbox" class:selected={formTradingRoomIds.includes(room.id)}>
								<input
									type="checkbox"
									checked={formTradingRoomIds.includes(room.id)}
									onchange={() => toggleRoom(room.id)}
								/>
								<span>{room.shortName || room.name}</span>
							</label>
						{/each}
					</div>
				</section>

				<!-- Trainer -->
				<section class="sidebar-section">
					<h3>Trainer</h3>
					<select bind:value={formTrainerId}>
						<option value="">Select trainer</option>
						{#each trainers as trainer}
							<option value={trainer.id}>{trainer.name}</option>
						{/each}
					</select>
				</section>

				<!-- Category -->
				<section class="sidebar-section">
					<h3>Category</h3>
					<select bind:value={formCategoryId}>
						<option value="">Select category</option>
						{#each categories as category}
							<option value={category.id}>{category.name}</option>
						{/each}
					</select>
				</section>

				<!-- Module -->
				<section class="sidebar-section">
					<h3>Module</h3>
					<select bind:value={formModuleId}>
						<option value="">No module</option>
						{#each modules as module}
							<option value={module.id}>{module.name}</option>
						{/each}
					</select>
				</section>
			</div>
		</div>
	{/if}
</div>

<!-- Delete Modal -->
{#if showDeleteModal}
	<div class="modal-overlay" onclick={() => showDeleteModal = false} onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && (showDeleteModal = false)} role="button" tabindex="-1" aria-label="Close modal">
		<div class="modal" onclick={(e: MouseEvent) => e.stopPropagation()} onkeydown={(e: KeyboardEvent) => e.stopPropagation()} role="dialog" aria-modal="true" tabindex="0">
			<h3>Delete Lesson?</h3>
			<p>This action cannot be undone. The lesson will be permanently deleted.</p>
			<div class="modal-actions">
				<button type="button" class="btn-cancel" onclick={() => showDeleteModal = false}>Cancel</button>
				<button type="button" class="btn-delete" onclick={handleDelete}>Delete</button>
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

	/* Not Found */
	.not-found {
		text-align: center;
		padding: 80px 20px;
	}

	.not-found h2 {
		color: white;
		margin: 0 0 8px;
	}

	.not-found p {
		color: #94a3b8;
		margin: 0 0 24px;
	}

	.not-found a {
		color: #f97316;
		text-decoration: none;
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

	.header-actions {
		display: flex;
		gap: 12px;
	}

	.btn-primary,
	.btn-secondary,
	.btn-danger {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 10px 20px;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		text-decoration: none;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-primary {
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: #334155;
		color: white;
	}

	.btn-secondary:hover {
		background: #475569;
	}

	.btn-danger {
		background: transparent;
		color: #ef4444;
		border: 1px solid #ef4444;
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.1);
	}

	/* Form Layout */
	.form-layout {
		display: grid;
		grid-template-columns: 1fr 320px;
		gap: 24px;
	}

	.form-section,
	.sidebar-section {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 24px;
	}

	.form-section h2 {
		margin: 0 0 20px;
		font-size: 1rem;
		font-weight: 600;
		color: white;
	}

	.sidebar-section h3 {
		margin: 0 0 12px;
		font-size: 0.875rem;
		font-weight: 600;
		color: white;
	}

	.sidebar-section:last-child {
		margin-bottom: 0;
	}

	/* Form Groups */
	.form-group {
		margin-bottom: 16px;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-group label {
		display: block;
		margin-bottom: 6px;
		font-size: 0.8rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.required {
		color: #ef4444;
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

	.sidebar-section select {
		width: 100%;
		padding: 10px 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 6px;
		color: white;
		font-size: 0.875rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.input-with-icon {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 6px;
		color: #64748b;
	}

	.input-with-icon:focus-within {
		border-color: #f97316;
	}

	.input-with-icon input {
		flex: 1;
		background: none;
		border: none;
		padding: 0;
	}

	.input-with-icon input:focus {
		outline: none;
	}

	/* Thumbnail Preview */
	.thumbnail-preview {
		margin-top: 12px;
		border-radius: 8px;
		overflow: hidden;
		background: #0f172a;
	}

	.thumbnail-preview img {
		width: 100%;
		max-height: 200px;
		object-fit: cover;
	}

	/* Checkboxes */
	.checkbox-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.checkbox-group label {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 0.875rem;
		color: #cbd5e1;
		cursor: pointer;
	}

	.room-checkboxes {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.room-checkbox {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 12px;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 6px;
		font-size: 0.8rem;
		color: #cbd5e1;
		cursor: pointer;
		transition: all 0.2s;
	}

	.room-checkbox.selected {
		border-color: #f97316;
		background: rgba(249, 115, 22, 0.1);
	}

	.room-checkbox input {
		display: none;
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
		.form-layout {
			grid-template-columns: 1fr;
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

		.header-actions {
			flex-wrap: wrap;
		}

		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

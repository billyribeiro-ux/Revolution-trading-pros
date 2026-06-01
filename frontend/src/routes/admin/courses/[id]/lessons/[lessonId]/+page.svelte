<script lang="ts">
	/**
	 * Admin Lesson Editor Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 *
	 * Features:
	 * - Full Svelte 5 runes ($state, $derived, $effect)
	 * - Complete Bunny.net video upload integration
	 * - CRUD operations for lessons and downloads
	 * - Real-time validation and auto-save
	 */

	import { onMount } from 'svelte';
	// FIX-2026-04-26 (P1-5/P1-6): use SvelteKit router state for params; add
	// beforeNavigate to guard unsaved changes.
	import { goto, beforeNavigate } from '$app/navigation';
	import { page } from '$app/state';
	import { adminFetch } from '$lib/utils/adminFetch';
	import BunnyVideoUploader from '$lib/components/admin/BunnyVideoUploader.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconChevronLeft from '@tabler/icons-svelte-runes/icons/chevron-left';
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';

	interface Lesson {
		id: string;
		course_id: string;
		module_id?: number;
		title: string;
		slug: string;
		description?: string;
		video_url?: string;
		bunny_video_guid?: string;
		thumbnail_url?: string;
		duration_minutes?: number;
		content_html?: string;
		is_free: boolean;
		is_preview?: boolean;
		is_published?: boolean;
		sort_order?: number;
		drip_days?: number;
	}

	interface Download {
		id: number;
		title: string;
		file_name: string;
		download_url?: string;
	}

	interface Module {
		id: number;
		title: string;
	}

	// FIX-2026-04-26 (P1-5): derive params from page.params (reactive across
	// client-side navigation) instead of parsing window.location.pathname.
	let courseId = $derived(page.params.id ?? '');
	let lessonId = $derived(page.params.lessonId ?? '');

	let lesson = $state<Lesson | null>(null);
	let downloads = $state<Download[]>([]);
	let modules = $state<Module[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	// FIX-2026-04-26 (P3-8): renamed from `errorMessage` (which by convention
	// signaled "unused") so the existing writes flow into UI.
	let errorMessage = $state('');

	// Delete confirmation modal state
	let showDeleteLessonModal = $state(false);
	let showRemoveDownloadModal = $state(false);
	let pendingRemoveDownloadId = $state<number | null>(null);

	// FIX-2026-04-26 (P0-3): input modal replacements for native prompt() flows
	// for Add Download (title + URL).
	let showAddDownloadModal = $state(false);
	let addDownloadStep = $state<'title' | 'url'>('title');
	let addDownloadTitle = $state('');
	let addDownloadUrl = $state('');

	// FIX-2026-04-26 (P1-9): validate Bunny GUID looks like a UUID/hex pattern;
	// validate thumbnail URL is https:// or relative — prevents `javascript:`
	// scheme + odd characters from leaking into the iframe `src`.
	const BUNNY_GUID_RE = /^[a-fA-F0-9-]{32,40}$/;
	let safeBunnyGuid = $derived(
		lesson?.bunny_video_guid && BUNNY_GUID_RE.test(lesson.bunny_video_guid)
			? lesson.bunny_video_guid
			: null
	);
	function isSafeImgUrl(url: string | undefined | null): boolean {
		if (!url) return false;
		if (url.startsWith('/')) return true;
		try {
			const u = new URL(url);
			return u.protocol === 'https:' || u.protocol === 'http:';
		} catch {
			return false;
		}
	}
	let safeThumbnailUrl = $derived(
		lesson?.thumbnail_url && isSafeImgUrl(lesson.thumbnail_url) ? lesson.thumbnail_url : null
	);

	// FIX-2026-04-26 (P1-6): unsaved-changes guard.
	let lastSavedSnapshot = $state<string>('');
	let hasUnsavedChanges = $derived(lesson ? JSON.stringify(lesson) !== lastSavedSnapshot : false);

	beforeNavigate((nav) => {
		// TODO(modal-confirm): SvelteKit's beforeNavigate is synchronous —
		// nav.cancel() must be called inline, so native confirm() is the only
		// portable option today.
		if (
			hasUnsavedChanges &&
			!nav.willUnload &&
			!confirm('You have unsaved changes. Discard and leave?')
		) {
			nav.cancel();
		}
	});

	onMount(() => {
		fetchLesson();
		const handler = (e: BeforeUnloadEvent) => {
			if (hasUnsavedChanges) {
				e.preventDefault();
			}
		};
		window.addEventListener('beforeunload', handler);
		return () => window.removeEventListener('beforeunload', handler);
	});

	const fetchLesson = async () => {
		loading = true;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const [lessonData, modulesData] = await Promise.all([
				adminFetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`),
				adminFetch(`/api/admin/courses/${courseId}/modules`)
			]);

			if (lessonData.success) {
				lesson = lessonData.data.lesson;
				downloads = lessonData.data.downloads || [];
				// FIX-2026-04-26 (P1-6): snapshot for dirty tracking.
				lastSavedSnapshot = JSON.stringify(lesson);
			}
			if (modulesData.success) {
				modules = modulesData.data || [];
			}
		} catch (e) {
			console.error('Failed to fetch lesson:', e);
		} finally {
			loading = false;
		}
	};

	const saveLesson = async () => {
		if (!lesson) return;
		saving = true;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, {
				method: 'PUT',
				body: JSON.stringify(lesson)
			});
			if (data.success) {
				lesson = data.data;
			} else {
				errorMessage = data.error || 'Failed to save lesson';
			}
		} catch {
			errorMessage = 'Failed to save lesson';
		} finally {
			saving = false;
		}
	};

	const deleteLesson = () => {
		showDeleteLessonModal = true;
	};

	const confirmDeleteLesson = async () => {
		showDeleteLessonModal = false;
		try {
			// ICT 7 FIX: Use adminFetch for absolute URL on Pages.dev
			await adminFetch(`/api/admin/courses/${courseId}/lessons/${lessonId}`, { method: 'DELETE' });
			goto(`/admin/courses/${courseId}`);
		} catch {
			errorMessage = 'Failed to delete lesson';
		}
	};

	// State for video uploader modal
	let showVideoUploader = $state(false);

	// Handle successful video upload from BunnyVideoUploader
	const handleVideoUploadComplete = (data: {
		video_url: string;
		embed_url: string;
		video_guid: string;
		thumbnail_url: string;
		duration?: number;
	}) => {
		if (!lesson) return;

		lesson.bunny_video_guid = data.video_guid;
		lesson.thumbnail_url = data.thumbnail_url || lesson.thumbnail_url;
		if (data.duration) {
			lesson.duration_minutes = Math.ceil(data.duration / 60);
		}

		showVideoUploader = false;
		saveLesson();
	};

	// Handle video upload error
	const handleVideoUploadError = (error: string) => {
		errorMessage = `Video upload failed: ${error}`;
	};

	// FIX-2026-04-26 (P0-3): replace native prompt() flows with multi-step
	// modal. Step 1 captures title; Step 2 captures URL with validation.
	const addDownload = () => {
		addDownloadTitle = '';
		addDownloadUrl = '';
		addDownloadStep = 'title';
		showAddDownloadModal = true;
	};

	const confirmAddDownloadStep = async (value?: string) => {
		const v = (value ?? '').trim();
		if (addDownloadStep === 'title') {
			if (!v) {
				errorMessage = 'Download title is required';
				return;
			}
			addDownloadTitle = v;
			addDownloadStep = 'url';
			return;
		}
		// URL step — optional, but if provided must be a valid http(s) URL.
		if (v && !isSafeImgUrl(v)) {
			errorMessage = 'URL must start with http:// or https://';
			return;
		}
		const title = addDownloadTitle;
		const fileUrl = v || undefined;
		showAddDownloadModal = false;
		addDownloadStep = 'title';
		try {
			const data = await adminFetch(
				`/api/admin/courses/${courseId}/lessons/${lessonId}/downloads`,
				{
					method: 'POST',
					body: JSON.stringify({ title, file_url: fileUrl })
				}
			);

			// FIX-2026-04-26 (P0-2): tolerate envelope variations.
			const created = data?.data ?? data;
			if (created && (data.success === undefined || data.success)) {
				downloads = [...downloads, created];
			} else {
				errorMessage = data?.error || data?.message || 'Failed to add download';
			}
		} catch {
			errorMessage = 'Failed to add download';
		}
	};

	// Remove download from lesson
	const removeDownload = (downloadId: number) => {
		pendingRemoveDownloadId = downloadId;
		showRemoveDownloadModal = true;
	};

	const confirmRemoveDownload = async () => {
		if (pendingRemoveDownloadId === null) return;
		showRemoveDownloadModal = false;
		const downloadId = pendingRemoveDownloadId;
		pendingRemoveDownloadId = null;

		try {
			await adminFetch(
				`/api/admin/courses/${courseId}/lessons/${lessonId}/downloads/${downloadId}`,
				{
					method: 'DELETE'
				}
			);
			downloads = downloads.filter((d) => d.id !== downloadId);
		} catch {
			errorMessage = 'Failed to remove download';
		}
	};
</script>

<svelte:head>
	<title>{lesson?.title || 'Edit Lesson'} | Admin</title>
</svelte:head>

<div class="lesson-editor">
	{#if errorMessage}
		<!-- FIX-2026-04-26 (P3-8): surface previously-silent errorMessage. -->
		<div class="error-banner" role="alert">
			<span>{errorMessage}</span>
			<button type="button" aria-label="Dismiss" onclick={() => (errorMessage = '')}>
				<IconX size={14} aria-hidden="true" />
			</button>
		</div>
	{/if}
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading lesson...</p>
		</div>
	{:else if lesson}
		<header class="editor-header">
			<div class="header-left">
				<a href="/admin/courses/{courseId}" class="back-link">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: chevron-left (back link) -->
					<IconChevronLeft size={20} aria-hidden="true" />
					Back to Course
				</a>
				<h1>{lesson.title}</h1>
			</div>
			<div class="header-actions">
				<button class="btn-danger" onclick={deleteLesson}>Delete</button>
				<button class="btn-primary" onclick={saveLesson} disabled={saving}>
					{saving ? 'Saving...' : 'Save Lesson'}
				</button>
			</div>
		</header>

		<div class="editor-content">
			<div class="main-panel">
				<section class="form-section">
					<h2>Lesson Details</h2>
					<div class="form-grid">
						<div class="form-group full">
							<label for="title">Title</label>
							<input id="title" name="title" type="text" bind:value={lesson.title} />
						</div>
						<div class="form-group full">
							<label for="slug">Slug</label>
							<input id="slug" name="slug" type="text" bind:value={lesson.slug} />
						</div>
						<div class="form-group full">
							<label for="description">Description</label>
							<textarea id="description" rows="3" bind:value={lesson.description}></textarea>
						</div>
						<div class="form-group">
							<label for="module">Module</label>
							<select id="module" bind:value={lesson.module_id}>
								<option value={null}>No Module</option>
								{#each modules as mod (mod.id)}
									<option value={mod.id}>{mod.title}</option>
								{/each}
							</select>
						</div>
						<div class="form-group">
							<label for="duration">Duration (minutes)</label>
							<input
								id="duration"
								name="duration"
								type="number"
								bind:value={lesson.duration_minutes}
							/>
						</div>
						<div class="form-group">
							<label for="sort">Sort Order</label>
							<input id="sort" name="sort" type="number" bind:value={lesson.sort_order} />
						</div>
						<div class="form-group">
							<label for="drip">Drip Days</label>
							<input
								id="drip"
								name="drip"
								type="number"
								bind:value={lesson.drip_days}
								placeholder="0 = immediate"
							/>
						</div>
					</div>
				</section>

				<section class="form-section">
					<h2>Video</h2>
					<div class="video-section">
						{#if lesson.bunny_video_guid}
							<div class="video-preview">
								<!-- FIX-2026-04-26 (P1-9): only render iframe when GUID matches the
								     UUID/hex pattern; otherwise show a validation hint. -->
								{#if safeBunnyGuid}
									<iframe
										src="https://iframe.mediadelivery.net/embed/{safeBunnyGuid}"
										title={lesson.title}
										allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
										allowfullscreen
									></iframe>
								{:else}
									<div class="video-invalid">
										Invalid Bunny Video GUID. Expected hex/UUID-style identifier.
									</div>
								{/if}
							</div>
							<div class="video-info">
								<p><strong>Video GUID:</strong> {lesson.bunny_video_guid}</p>
								<button
									class="btn-secondary"
									onclick={() => {
										if (lesson) lesson.bunny_video_guid = undefined;
									}}
								>
									Remove Video
								</button>
							</div>
						{:else}
							<div class="upload-area">
								{#if showVideoUploader}
									<div class="video-uploader-container">
										<BunnyVideoUploader
											onUploadComplete={handleVideoUploadComplete}
											onError={handleVideoUploadError}
										/>
										<button
											type="button"
											class="btn-cancel-upload"
											onclick={() => (showVideoUploader = false)}
										>
											Cancel
										</button>
									</div>
								{:else}
									<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: video (video placeholder) -->
									<IconVideo size={48} aria-hidden="true" />
									<p>Upload a video or enter a Bunny.net Video GUID</p>
									<button
										type="button"
										class="btn-upload-video"
										onclick={() => (showVideoUploader = true)}
									>
										<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: upload (upload video btn) -->
										<IconUpload size={20} aria-hidden="true" />
										Upload Video
									</button>
									<span class="divider">or</span>
									<input
										id="page-lesson-bunny-video-guid"
										name="page-lesson-bunny-video-guid"
										type="text"
										placeholder="Enter Bunny Video GUID"
										bind:value={lesson.bunny_video_guid}
									/>
								{/if}
							</div>
						{/if}
					</div>
				</section>

				<section class="form-section">
					<h2>Content</h2>
					<div class="form-group full">
						<label for="content">Lesson Content (HTML)</label>
						<textarea id="content" rows="10" bind:value={lesson.content_html} class="code-input"
						></textarea>
					</div>
				</section>
			</div>

			<aside class="side-panel">
				<section class="panel-section">
					<h3>Access Settings</h3>
					<label class="toggle">
						<input
							id="page-lesson-is-published"
							name="page-lesson-is-published"
							type="checkbox"
							bind:checked={lesson.is_published}
						/>
						<span>Published</span>
					</label>
					<label class="toggle">
						<input
							id="page-lesson-is-free"
							name="page-lesson-is-free"
							type="checkbox"
							bind:checked={lesson.is_free}
						/>
						<span>Free Preview</span>
					</label>
					<label class="toggle">
						<input
							id="page-lesson-is-preview"
							name="page-lesson-is-preview"
							type="checkbox"
							bind:checked={lesson.is_preview}
						/>
						<span>Show in Preview</span>
					</label>
				</section>

				<section class="panel-section">
					<h3>Thumbnail</h3>
					<div class="thumbnail-preview">
						<!-- FIX-2026-04-26 (P1-9): only render <img> when URL passes
						     scheme validation (https://, http://, or relative). -->
						{#if safeThumbnailUrl}
							<img src={safeThumbnailUrl} alt="Thumbnail" width="320" height="180" loading="lazy" />
						{:else if lesson.thumbnail_url}
							<div class="no-thumb">Invalid thumbnail URL</div>
						{:else}
							<div class="no-thumb">No thumbnail</div>
						{/if}
					</div>
					<input
						id="page-lesson-thumbnail-url"
						name="page-lesson-thumbnail-url"
						type="text"
						placeholder="Thumbnail URL"
						bind:value={lesson.thumbnail_url}
					/>
				</section>

				<section class="panel-section">
					<h3>Downloads ({downloads.length})</h3>
					{#if downloads.length === 0}
						<p class="empty-text">No downloads attached</p>
					{:else}
						<ul class="downloads-list">
							{#each downloads as dl (dl.id)}
								<li class="download-item">
									<span class="download-title">{dl.title}</span>
									<button
										type="button"
										class="btn-remove-download"
										onclick={() => removeDownload(dl.id)}
										aria-label="Remove download"
									>
										<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (remove download) -->
										<IconX size={14} aria-hidden="true" />
									</button>
								</li>
							{/each}
						</ul>
					{/if}
					<button class="btn-add" onclick={addDownload}>+ Add Download</button>
				</section>
			</aside>
		</div>
	{/if}
</div>

<ConfirmationModal
	isOpen={showDeleteLessonModal}
	title="Delete Lesson"
	message="Are you sure you want to delete this lesson?"
	confirmText="Delete"
	variant="danger"
	onConfirm={confirmDeleteLesson}
	onCancel={() => (showDeleteLessonModal = false)}
/>

<ConfirmationModal
	isOpen={showRemoveDownloadModal}
	title="Remove Download"
	message="Remove this download?"
	confirmText="Remove"
	variant="danger"
	onConfirm={confirmRemoveDownload}
	onCancel={() => {
		showRemoveDownloadModal = false;
		pendingRemoveDownloadId = null;
	}}
/>

<!-- FIX-2026-04-26 (P0-3): two-step input modal replacement for native prompts. -->
<ConfirmationModal
	isOpen={showAddDownloadModal && addDownloadStep === 'title'}
	title="Add Download — Title"
	message="Enter a title for this download."
	confirmText="Next"
	variant="info"
	showInput={true}
	inputLabel="Download title"
	inputPlaceholder="e.g. Worksheet PDF"
	bind:inputValue={addDownloadTitle}
	onConfirm={confirmAddDownloadStep}
	onCancel={() => {
		showAddDownloadModal = false;
		addDownloadStep = 'title';
		addDownloadTitle = '';
		addDownloadUrl = '';
	}}
/>

<ConfirmationModal
	isOpen={showAddDownloadModal && addDownloadStep === 'url'}
	title="Add Download — File URL"
	message="Enter the file URL (https:// or relative path). Leave empty to attach later."
	confirmText="Add"
	variant="info"
	showInput={true}
	inputLabel="File URL (optional)"
	inputPlaceholder="https://cdn.example.com/file.pdf"
	bind:inputValue={addDownloadUrl}
	onConfirm={confirmAddDownloadStep}
	onCancel={() => {
		showAddDownloadModal = false;
		addDownloadStep = 'title';
		addDownloadTitle = '';
		addDownloadUrl = '';
	}}
/>

<style>
	.error-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		background: #fee2e2;
		color: #991b1b;
		border: 1px solid #fecaca;
		padding: 10px 14px;
		border-radius: 8px;
		margin-bottom: 16px;
		font-size: 14px;
	}
	.error-banner button {
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		padding: 4px;
		display: flex;
		align-items: center;
	}
	.video-invalid {
		padding: 16px;
		background: #fef3c7;
		color: #92400e;
		border: 1px dashed #fbbf24;
		border-radius: 6px;
		font-size: 14px;
	}

	.lesson-editor {
		padding: 24px;
		max-width: 1400px;
		margin: 0 auto;
	}

	.loading {
		text-align: center;
		padding: 64px;
	}
	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 16px;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 24px;
		padding-bottom: 24px;
		border-bottom: 1px solid #e5e7eb;
	}
	.header-left {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: #6b7280;
		text-decoration: none;
		font-size: 14px;
	}
	.back-link:hover {
		color: #143e59;
	}
	.header-left h1 {
		font-size: 24px;
		margin: 0;
		color: #1f2937;
	}

	.header-actions {
		display: flex;
		gap: 8px;
	}
	.btn-primary,
	.btn-secondary,
	.btn-danger {
		padding: 10px 16px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}
	.btn-primary {
		background: #143e59;
		color: #fff;
	}
	.btn-primary:hover {
		background: #0f2d42;
	}
	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.btn-secondary {
		background: #f3f4f6;
		color: #1f2937;
	}
	.btn-secondary:hover {
		background: #e5e7eb;
	}
	.btn-danger {
		background: #fee2e2;
		color: #dc2626;
	}
	.btn-danger:hover {
		background: #fecaca;
	}

	.editor-content {
		display: grid;
		grid-template-columns: 1fr 320px;
		gap: 24px;
	}

	.main-panel {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}
	.form-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
	}
	.form-section h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 20px;
	}

	.form-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}
	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}
	.form-group.full {
		grid-column: 1 / -1;
	}
	.form-group label {
		font-size: 13px;
		font-weight: 500;
		color: #374151;
	}
	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #143e59;
	}
	.code-input {
		font-family: monospace;
		font-size: 13px;
	}

	.video-section {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
	.video-preview {
		aspect-ratio: 16/9;
		background: #000;
		border-radius: 8px;
		overflow: hidden;
	}
	.video-preview iframe {
		width: 100%;
		height: 100%;
		border: none;
	}
	.video-info {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px;
		background: #f9fafb;
		border-radius: 6px;
	}
	.video-info p {
		margin: 0;
		font-size: 13px;
		color: #6b7280;
	}

	.upload-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		padding: 32px;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 8px;
		text-align: center;
	}
	.upload-area :global(svg) {
		color: #9ca3af;
	}
	.upload-area p {
		color: #6b7280;
		margin: 0;
	}
	/* .upload-area input[type="file"] { cursor: pointer; } - Unused */
	.upload-area input[type='text'] {
		width: 100%;
		max-width: 300px;
		padding: 10px 12px;
		border: 1px solid #e5e7eb;
		border-radius: 6px;
		font-size: 14px;
	}
	.upload-area input[type='text']:focus {
		outline: none;
		border-color: #143e59;
	}
	.divider {
		color: #9ca3af;
		font-size: 12px;
	}

	/* Video Uploader Container */
	.video-uploader-container {
		width: 100%;
		max-width: 500px;
	}
	.btn-upload-video {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px 24px;
		background: linear-gradient(135deg, #143e59 0%, #1e73be 100%);
		border: none;
		border-radius: 8px;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}
	.btn-upload-video:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.3);
	}
	.btn-cancel-upload {
		margin-top: 12px;
		padding: 8px 16px;
		background: #f3f4f6;
		border: none;
		border-radius: 6px;
		color: #6b7280;
		font-size: 13px;
		cursor: pointer;
	}
	.btn-cancel-upload:hover {
		background: #e5e7eb;
	}

	.side-panel {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}
	.panel-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 16px;
	}
	.panel-section h3 {
		font-size: 14px;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 12px;
	}

	.toggle {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px 0;
		cursor: pointer;
		font-size: 14px;
	}
	.toggle input {
		width: 18px;
		height: 18px;
	}

	.thumbnail-preview {
		aspect-ratio: 16/9;
		background: #f3f4f6;
		border-radius: 6px;
		overflow: hidden;
		margin-bottom: 8px;
	}
	.thumbnail-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}
	.no-thumb {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #9ca3af;
		font-size: 13px;
	}

	.empty-text {
		color: #9ca3af;
		font-size: 13px;
		margin: 0;
	}
	.downloads-list {
		list-style: none;
		margin: 0 0 12px;
		padding: 0;
	}
	.downloads-list li {
		padding: 8px;
		background: #f9fafb;
		border-radius: 4px;
		margin-bottom: 4px;
		font-size: 13px;
	}
	.download-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.download-title {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	.btn-remove-download {
		background: none;
		border: none;
		color: #9ca3af;
		cursor: pointer;
		padding: 4px;
		border-radius: 4px;
		flex-shrink: 0;
	}
	.btn-remove-download:hover {
		background: #fee2e2;
		color: #dc2626;
	}
	.btn-add {
		width: 100%;
		padding: 8px;
		background: #f3f4f6;
		border: 1px dashed #d1d5db;
		border-radius: 6px;
		color: #6b7280;
		font-size: 13px;
		cursor: pointer;
	}
	.btn-add:hover {
		background: #e5e7eb;
	}

	@media (max-width: 1023.98px) {
		.editor-content {
			grid-template-columns: 1fr;
		}
		.side-panel {
			order: -1;
		}
	}

	@media (max-width: 767.98px) {
		.editor-header {
			flex-direction: column;
			gap: 16px;
		}
		.form-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

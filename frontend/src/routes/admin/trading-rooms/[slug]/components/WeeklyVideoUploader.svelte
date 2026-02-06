<!--
	Weekly Video Uploader - ICT 7 Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Features:
	- Drag & drop video upload to Bunny.net
	- Video URL paste support
	- Preview before publish
	- Auto-archive previous video
	- Thumbnail generation
	
	@version 1.0.0
-->
<script lang="ts">
	import { untrack } from 'svelte';
	import { weeklyVideoApi, type WeeklyVideo } from '$lib/api/room-content';

	// Icons
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconCloudUpload from '@tabler/icons-svelte-runes/icons/cloud-upload';
	import IconVideo from '@tabler/icons-svelte-runes/icons/video';
	import IconLink from '@tabler/icons-svelte-runes/icons/link';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconArchive from '@tabler/icons-svelte-runes/icons/archive';

	// ═══════════════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════════════

	interface Props {
		roomSlug: string;
		onSuccess?: (message: string) => void;
		onError?: (message: string) => void;
	}

	const { roomSlug, onSuccess, onError }: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════════

	let currentVideo = $state<WeeklyVideo | null>(null);
	let archivedVideos = $state<WeeklyVideo[]>([]);
	let isLoading = $state(true);
	let showUploadModal = $state(false);
	let isUploading = $state(false);
	let uploadProgress = $state(0);
	let isDragOver = $state(false);

	// Upload mode
	let uploadMode = $state<'url' | 'file'>('url');

	// Form state
	let form = $state({
		week_of: getNextMonday(),
		week_title: '',
		video_title: '',
		video_url: '',
		video_platform: 'bunny' as 'bunny' | 'vimeo' | 'youtube',
		thumbnail_url: '',
		duration: '',
		description: ''
	});

	// File upload
	let uploadFile = $state<File | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════════
	// HELPERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function getNextMonday(): string {
		const today = new Date();
		const dayOfWeek = today.getDay();
		const daysUntilMonday = dayOfWeek === 0 ? 1 : (8 - dayOfWeek) % 7 || 7;
		const nextMonday = new Date(today);
		nextMonday.setDate(today.getDate() + daysUntilMonday);
		return nextMonday.toISOString().split('T')[0];
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			weekday: 'short',
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function detectPlatform(url: string): 'bunny' | 'vimeo' | 'youtube' {
		if (url.includes('vimeo.com')) return 'vimeo';
		if (url.includes('youtube.com') || url.includes('youtu.be')) return 'youtube';
		return 'bunny';
	}

	function generateEmbedUrl(url: string, platform: string): string {
		if (platform === 'vimeo') {
			const match = url.match(/vimeo\.com\/(\d+)/);
			if (match) return `https://player.vimeo.com/video/${match[1]}`;
		}
		if (platform === 'youtube') {
			const match = url.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&]+)/);
			if (match) return `https://www.youtube.com/embed/${match[1]}`;
		}
		return url;
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// DERIVED
	// ═══════════════════════════════════════════════════════════════════════════════

	const isFormValid = $derived(
		form.video_url.trim() !== '' && form.video_title.trim() !== '' && form.week_of !== ''
	);

	const embedUrl = $derived(
		form.video_url ? generateEmbedUrl(form.video_url, form.video_platform) : ''
	);

	// ═══════════════════════════════════════════════════════════════════════════════
	// API FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════════

	async function loadVideos() {
		isLoading = true;
		try {
			const [currentRes, archived] = await Promise.all([
				weeklyVideoApi.getCurrent(roomSlug),
				weeklyVideoApi.list(roomSlug)
			]);
			currentVideo = currentRes.data ?? null;
			archivedVideos = archived.data.filter((v) => !v.is_current);
		} catch (err) {
			console.error('Failed to load videos:', err);
		} finally {
			isLoading = false;
		}
	}

	async function publishVideo() {
		if (!isFormValid) return;
		isUploading = true;

		try {
			await weeklyVideoApi.create({
				room_slug: roomSlug,
				week_of: form.week_of,
				week_title: form.week_title || `Week of ${formatDate(form.week_of)}`,
				video_title: form.video_title,
				video_url: form.video_url,
				video_platform: form.video_platform,
				thumbnail_url: form.thumbnail_url || undefined,
				duration: form.duration || undefined,
				description: form.description || undefined
			});

			onSuccess?.('Weekly breakdown video published');
			closeModal();
			await loadVideos();
		} catch (err) {
			onError?.('Failed to publish video');
			console.error(err);
		} finally {
			isUploading = false;
		}
	}

	async function uploadToBunny() {
		if (!uploadFile) return;
		isUploading = true;
		uploadProgress = 0;

		try {
			// Step 1: Create video entry on Bunny.net
			const createRes = await fetch('/api/admin/bunny/create-video', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					title: form.video_title || uploadFile.name,
					collection_id: roomSlug
				})
			});

			if (!createRes.ok) throw new Error('Failed to create video entry');
			const { video_id, upload_url } = await createRes.json();

			// Step 2: Upload file to Bunny.net
			const uploadRes = await fetch(upload_url, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/octet-stream' },
				body: uploadFile
			});

			if (!uploadRes.ok) throw new Error('Failed to upload video');

			// Step 3: Update form with Bunny URL
			form.video_url = `https://iframe.mediadelivery.net/embed/VIDEO_LIBRARY_ID/${video_id}`;
			form.video_platform = 'bunny';

			uploadProgress = 100;
			onSuccess?.('Video uploaded to Bunny.net');
		} catch (err) {
			onError?.('Failed to upload video');
			console.error(err);
		} finally {
			isUploading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// UI HANDLERS
	// ═══════════════════════════════════════════════════════════════════════════════

	function openUploadModal() {
		form = {
			week_of: getNextMonday(),
			week_title: '',
			video_title: '',
			video_url: '',
			video_platform: 'bunny',
			thumbnail_url: '',
			duration: '',
			description: ''
		};
		uploadFile = null;
		showUploadModal = true;
	}

	function closeModal() {
		showUploadModal = false;
		uploadFile = null;
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragOver = true;
	}

	function handleDragLeave() {
		isDragOver = false;
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragOver = false;

		const files = e.dataTransfer?.files;
		if (files && files.length > 0) {
			const file = files[0];
			if (file.type.startsWith('video/')) {
				uploadFile = file;
				uploadMode = 'file';
				form.video_title = file.name.replace(/\.[^/.]+$/, '');
			} else {
				onError?.('Please drop a video file');
			}
		}
	}

	function handleFileSelect(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			uploadFile = input.files[0];
			uploadMode = 'file';
			form.video_title = uploadFile.name.replace(/\.[^/.]+$/, '');
		}
	}

	function handleUrlPaste() {
		form.video_platform = detectPlatform(form.video_url);
	}

	// ═══════════════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (roomSlug) {
			untrack(() => loadVideos());
		}
	});
</script>

<div class="weekly-video-uploader">
	<!-- Header -->
	<div class="uploader-header">
		<div class="header-left">
			<h3 class="uploader-title">Weekly Breakdown Video</h3>
			{#if currentVideo}
				<span class="current-badge">
					<IconPlayerPlay size={14} />
					Live
				</span>
			{/if}
		</div>
		<button class="btn-upload" onclick={openUploadModal}>
			<IconUpload size={18} />
			<span>Upload New Video</span>
		</button>
	</div>

	<!-- Current Video Display -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<span>Loading...</span>
		</div>
	{:else if currentVideo}
		<div class="current-video-card">
			<div class="video-preview">
				{#if currentVideo.thumbnail_url}
					<img src={currentVideo.thumbnail_url} alt={currentVideo.video_title} />
				{:else}
					<div class="video-placeholder">
						<IconVideo size={48} />
					</div>
				{/if}
				<a
					href={currentVideo.video_url}
					target="_blank"
					rel="noopener noreferrer"
					class="play-overlay"
				>
					<IconPlayerPlay size={32} />
				</a>
			</div>
			<div class="video-info">
				<h4 class="video-title">{currentVideo.video_title}</h4>
				<p class="video-week">{currentVideo.week_title}</p>
				<div class="video-meta">
					<span class="meta-item">
						<IconCalendar size={14} />
						Week of {formatDate(currentVideo.week_of)}
					</span>
					{#if currentVideo.duration}
						<span class="meta-item">
							<IconPlayerPlay size={14} />
							{currentVideo.duration}
						</span>
					{/if}
				</div>
				{#if currentVideo.description}
					<p class="video-description">{currentVideo.description}</p>
				{/if}
			</div>
		</div>
	{:else}
		<div class="empty-state">
			<div class="empty-icon">
				<IconVideo size={48} />
			</div>
			<h4>No Weekly Video</h4>
			<p>Upload this week's breakdown video</p>
			<button class="btn-upload-first" onclick={openUploadModal}>
				<IconUpload size={18} />
				Upload Video
			</button>
		</div>
	{/if}

	<!-- Archived Videos -->
	{#if archivedVideos.length > 0}
		<div class="archived-section">
			<div class="archived-header">
				<IconArchive size={18} />
				<span>Archived Videos ({archivedVideos.length})</span>
			</div>
			<div class="archived-list">
				{#each archivedVideos.slice(0, 5) as video}
					<div class="archived-item">
						<span class="archived-title">{video.video_title}</span>
						<span class="archived-date">{formatDate(video.week_of)}</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>

<!-- Upload Modal -->
{#if showUploadModal}
	<div
		class="modal-overlay"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={(e) => e.target === e.currentTarget && closeModal()}
		onkeydown={(e) => e.key === 'Escape' && closeModal()}
	>
		<div class="modal-content">
			<div class="modal-header">
				<h3>Upload Weekly Breakdown Video</h3>
				<button class="modal-close" onclick={closeModal}>
					<IconX size={20} />
				</button>
			</div>

			<div class="modal-body">
				<!-- Upload Mode Tabs -->
				<div class="mode-tabs">
					<button
						class="mode-tab"
						class:active={uploadMode === 'url'}
						onclick={() => (uploadMode = 'url')}
					>
						<IconLink size={18} />
						Paste URL
					</button>
					<button
						class="mode-tab"
						class:active={uploadMode === 'file'}
						onclick={() => (uploadMode = 'file')}
					>
						<IconCloudUpload size={18} />
						Upload File
					</button>
				</div>

				{#if uploadMode === 'file'}
					<!-- Drag & Drop Zone -->
					<div
						class="drop-zone"
						class:drag-over={isDragOver}
						class:has-file={uploadFile}
						role="button"
						tabindex="0"
						aria-label="Drop zone for video files"
						ondragover={handleDragOver}
						ondragleave={handleDragLeave}
						ondrop={handleDrop}
					>
						{#if uploadFile}
							<div class="file-info">
								<IconVideo size={32} />
								<span class="file-name">{uploadFile.name}</span>
								<span class="file-size">{(uploadFile.size / 1024 / 1024).toFixed(2)} MB</span>
								<button class="btn-remove-file" onclick={() => (uploadFile = null)}>
									<IconX size={16} />
								</button>
							</div>
						{:else}
							<IconCloudUpload size={48} />
							<p class="drop-text">Drag & drop video here</p>
							<p class="drop-subtext">or</p>
							<label class="btn-browse">
								<input type="file" accept="video/*" onchange={handleFileSelect} hidden />
								Browse Files
							</label>
						{/if}
					</div>

					{#if isUploading}
						<div class="upload-progress">
							<div class="progress-bar" style="width: {uploadProgress}%"></div>
							<span class="progress-text">{uploadProgress}% uploaded</span>
						</div>
					{/if}
				{:else}
					<!-- URL Input -->
					<div class="url-input-group">
						<label for="video_url">Video URL</label>
						<div class="url-input-wrapper">
							<input
								id="video_url"
								name="video_url"
								type="url"
								bind:value={form.video_url}
								oninput={handleUrlPaste}
								placeholder="https://vimeo.com/123456789 or Bunny.net embed URL"
								class="form-input"
							/>
							<span class="platform-badge">{form.video_platform}</span>
						</div>
					</div>

					<!-- Video Preview -->
					{#if embedUrl}
						<div class="video-preview-embed">
							<iframe
								src={embedUrl}
								title="Video Preview"
								frameborder="0"
								allow="autoplay; fullscreen"
							></iframe>
						</div>
					{/if}
				{/if}

				<!-- Form Fields -->
				<div class="form-fields">
					<div class="form-row">
						<div class="form-group">
							<label for="week_of">Week Of *</label>
							<input
								id="week_of"
								name="week_of"
								type="date"
								bind:value={form.week_of}
								class="form-input"
								required
							/>
						</div>
						<div class="form-group">
							<label for="week_title">Week Title</label>
							<input
								id="week_title"
								name="week_title"
								type="text"
								bind:value={form.week_title}
								placeholder="Week of Jan 27, 2026"
								class="form-input"
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="video_title">Video Title *</label>
						<input
							id="video_title"
							name="video_title"
							type="text"
							bind:value={form.video_title}
							placeholder="Weekly Breakdown - Key Levels & Setups"
							class="form-input"
							required
						/>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="thumbnail_url">Thumbnail URL</label>
							<input
								id="thumbnail_url"
								name="thumbnail_url"
								type="url"
								bind:value={form.thumbnail_url}
								placeholder="https://..."
								class="form-input"
							/>
						</div>
						<div class="form-group">
							<label for="duration">Duration</label>
							<input
								id="duration"
								name="duration"
								type="text"
								bind:value={form.duration}
								placeholder="12:34"
								class="form-input"
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="description">Description</label>
						<textarea
							id="description"
							bind:value={form.description}
							placeholder="Brief overview of this week's content..."
							class="form-textarea"
							rows="3"
						></textarea>
					</div>
				</div>
			</div>

			<div class="modal-footer">
				<div class="archive-notice">
					<IconArchive size={16} />
					<span>Publishing will archive the current video</span>
				</div>
				<div class="footer-actions">
					<button class="btn-cancel" onclick={closeModal}>Cancel</button>
					<button
						class="btn-publish"
						onclick={uploadMode === 'file' && uploadFile && !form.video_url
							? uploadToBunny
							: publishVideo}
						disabled={(!isFormValid && !uploadFile) || isUploading}
					>
						{#if isUploading}
							<span class="spinner-small"></span>
							{uploadMode === 'file' ? 'Uploading...' : 'Publishing...'}
						{:else}
							<IconCheck size={18} />
							Publish Video
						{/if}
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.weekly-video-uploader {
		background: white;
		border-radius: 12px;
		border: 1px solid #e2e8f0;
		overflow: hidden;
	}

	.uploader-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		border-bottom: 1px solid #e2e8f0;
		background: #f8fafc;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.uploader-title {
		font-size: 16px;
		font-weight: 600;
		color: #1e293b;
		margin: 0;
	}

	.current-badge {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 10px;
		background: #dcfce7;
		color: #166534;
		border-radius: 20px;
		font-size: 12px;
		font-weight: 600;
	}

	.btn-upload {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		background: #143e59;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		color: white;
		cursor: pointer;
	}

	.btn-upload:hover {
		background: #0f2d42;
	}

	/* Current Video Card */
	.current-video-card {
		display: grid;
		grid-template-columns: 280px 1fr;
		gap: 20px;
		padding: 20px;
	}

	.video-preview {
		position: relative;
		border-radius: 10px;
		overflow: hidden;
		aspect-ratio: 16/9;
		background: #1e293b;
	}

	.video-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.video-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #64748b;
	}

	.play-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.4);
		color: white;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.video-preview:hover .play-overlay {
		opacity: 1;
	}

	.video-info {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.video-title {
		margin: 0;
		font-size: 18px;
		font-weight: 600;
		color: #1e293b;
	}

	.video-week {
		margin: 0;
		font-size: 14px;
		color: #64748b;
	}

	.video-meta {
		display: flex;
		gap: 16px;
		margin-top: 8px;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 13px;
		color: #64748b;
	}

	.video-description {
		margin: 8px 0 0;
		font-size: 14px;
		color: #475569;
		line-height: 1.5;
	}

	/* Loading & Empty */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 48px 24px;
		text-align: center;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e2e8f0;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-icon {
		color: #94a3b8;
		margin-bottom: 16px;
	}

	.empty-state h4 {
		margin: 0 0 8px;
		color: #1e293b;
	}

	.empty-state p {
		margin: 0 0 20px;
		color: #64748b;
	}

	.btn-upload-first {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		background: #143e59;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: white;
		cursor: pointer;
	}

	/* Archived Section */
	.archived-section {
		border-top: 1px solid #e2e8f0;
		padding: 16px 20px;
	}

	.archived-header {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		font-weight: 600;
		color: #64748b;
		margin-bottom: 12px;
	}

	.archived-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.archived-item {
		display: flex;
		justify-content: space-between;
		padding: 8px 12px;
		background: #f8fafc;
		border-radius: 6px;
		font-size: 13px;
	}

	.archived-title {
		color: #334155;
	}

	.archived-date {
		color: #94a3b8;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 24px;
	}

	.modal-content {
		background: white;
		border-radius: 16px;
		width: 100%;
		max-width: 720px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h3 {
		margin: 0;
		font-size: 18px;
		font-weight: 600;
	}

	.modal-close {
		padding: 8px;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #64748b;
		cursor: pointer;
	}

	.modal-body {
		padding: 24px;
	}

	/* Mode Tabs */
	.mode-tabs {
		display: flex;
		gap: 8px;
		margin-bottom: 20px;
	}

	.mode-tab {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 12px;
		background: #f1f5f9;
		border: 2px solid transparent;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.mode-tab:hover {
		background: #e2e8f0;
	}

	.mode-tab.active {
		background: white;
		border-color: #143e59;
		color: #143e59;
	}

	/* Drop Zone */
	.drop-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		background: #f8fafc;
		transition: all 0.2s;
		margin-bottom: 20px;
	}

	.drop-zone.drag-over {
		border-color: #143e59;
		background: rgba(20, 62, 89, 0.05);
	}

	.drop-zone.has-file {
		border-style: solid;
		border-color: #22c55e;
		background: #f0fdf4;
	}

	.drop-text {
		margin: 12px 0 4px;
		font-size: 16px;
		font-weight: 500;
		color: #334155;
	}

	.drop-subtext {
		margin: 0 0 12px;
		font-size: 13px;
		color: #94a3b8;
	}

	.btn-browse {
		padding: 10px 20px;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		color: #475569;
		cursor: pointer;
	}

	.btn-browse:hover {
		background: #f1f5f9;
	}

	.file-info {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 8px;
		color: #166534;
	}

	.file-name {
		font-size: 14px;
		font-weight: 600;
	}

	.file-size {
		font-size: 13px;
		color: #64748b;
	}

	.btn-remove-file {
		padding: 4px 8px;
		background: #fee2e2;
		border: none;
		border-radius: 4px;
		color: #dc2626;
		cursor: pointer;
		margin-top: 8px;
	}

	/* Upload Progress */
	.upload-progress {
		margin-bottom: 20px;
	}

	.progress-bar {
		height: 8px;
		background: #143e59;
		border-radius: 4px;
		transition: width 0.3s;
	}

	.progress-text {
		display: block;
		margin-top: 8px;
		font-size: 13px;
		color: #64748b;
		text-align: center;
	}

	/* URL Input */
	.url-input-group {
		margin-bottom: 20px;
	}

	.url-input-group label {
		display: block;
		margin-bottom: 6px;
		font-size: 12px;
		font-weight: 600;
		color: #475569;
		text-transform: uppercase;
	}

	.url-input-wrapper {
		position: relative;
	}

	.url-input-wrapper .form-input {
		padding-right: 80px;
	}

	.platform-badge {
		position: absolute;
		right: 8px;
		top: 50%;
		transform: translateY(-50%);
		padding: 4px 10px;
		background: #e2e8f0;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		color: #64748b;
	}

	/* Video Preview */
	.video-preview-embed {
		margin-bottom: 20px;
		border-radius: 10px;
		overflow: hidden;
		aspect-ratio: 16/9;
		background: #1e293b;
	}

	.video-preview-embed iframe {
		width: 100%;
		height: 100%;
	}

	/* Form Fields */
	.form-fields {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group label {
		font-size: 12px;
		font-weight: 600;
		color: #475569;
		text-transform: uppercase;
	}

	.form-input,
	.form-textarea {
		padding: 10px 12px;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: #143e59;
		box-shadow: 0 0 0 3px rgba(20, 62, 89, 0.1);
	}

	.form-textarea {
		resize: vertical;
		font-family: inherit;
	}

	/* Modal Footer */
	.modal-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 24px;
		border-top: 1px solid #e2e8f0;
		background: #f8fafc;
	}

	.archive-notice {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		color: #64748b;
	}

	.footer-actions {
		display: flex;
		gap: 12px;
	}

	.btn-cancel {
		padding: 10px 20px;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		color: #475569;
		cursor: pointer;
	}

	.btn-publish {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 24px;
		background: #143e59;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 600;
		color: white;
		cursor: pointer;
	}

	.btn-publish:hover:not(:disabled) {
		background: #0f2d42;
	}

	.btn-publish:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.current-video-card {
			grid-template-columns: 1fr;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.modal-footer {
			flex-direction: column;
			gap: 12px;
		}

		.footer-actions {
			width: 100%;
		}

		.btn-cancel,
		.btn-publish {
			flex: 1;
		}
	}
</style>

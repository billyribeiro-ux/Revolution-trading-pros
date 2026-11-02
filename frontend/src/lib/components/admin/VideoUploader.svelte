<script lang="ts">
	/**
	 * VideoUploader - Real Video Upload Component
	 *
	 * Handles video uploads with:
	 * - Drag and drop support
	 * - Progress tracking
	 * - Thumbnail generation/upload
	 * - Metadata entry
	 * - Category selection
	 *
	 */

	import { onMount } from 'svelte';
	import Icon from '#lib/components/Icon.svelte';

	// Props
	interface Props {
		membershipId?: string;
		onUploadComplete?: (video: UploadedVideo) => void;
		onCancel?: () => void;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const membershipId = $derived(props.membershipId ?? '');
	const onUploadComplete = $derived(props.onUploadComplete);
	const onCancel = $derived(props.onCancel);

	// Types
	interface UploadedVideo {
		id: string;
		title: string;
		slug: string;
		description: string;
		video_url: string;
		thumbnail_url: string;
		duration: number;
		categories: string[];
		instructor_id: string;
	}

	interface Category {
		id: string;
		name: string;
		slug: string;
	}

	interface Instructor {
		id: string;
		name: string;
	}

	// State
	let step = $state<'upload' | 'metadata' | 'complete'>('upload');
	let isDragging = $state(false);
	let videoFile = $state<File | null>(null);
	let thumbnailFile = $state<File | null>(null);
	let uploadProgress = $state(0);
	let _thumbnailProgress = $state(0);
	let isUploading = $state(false);
	let error = $state<string | null>(null);
	let uploadSessionId = $state<string | null>(null);

	// Video metadata
	let title = $state('');
	let description = $state('');
	let selectedCategories = $state<string[]>([]);
	let selectedInstructor = $state('');
	let isPublished = $state(false);
	let isPremium = $state(true);

	// Data from API
	let categories = $state<Category[]>([]);
	let instructors = $state<Instructor[]>([]);

	// Uploaded URLs
	let videoUrl = $state('');
	let thumbnailUrl = $state('');
	let videoDuration = $state(0);

	// File input refs are imperative DOM handles; they do not affect rendering.
	let videoInput: HTMLInputElement | null = null;
	let thumbnailInput: HTMLInputElement | null = null;

	const videoDropClass = $derived([
		'drop-zone',
		isDragging && 'drop-zone--dragging',
		videoFile !== null && 'drop-zone--has-file'
	]);
	const thumbnailDropClass = $derived([
		'thumbnail-drop',
		thumbnailFile !== null && 'thumbnail-drop--has-file'
	]);
	const uploadProgressWidth = $derived(`${uploadProgress}%`);

	// Allowed file types
	const allowedVideoTypes = ['video/mp4', 'video/webm', 'video/quicktime'];
	const allowedImageTypes = ['image/jpeg', 'image/png', 'image/webp'];
	const maxVideoSize = 500 * 1024 * 1024; // 500MB
	const maxThumbnailSize = 5 * 1024 * 1024; // 5MB

	// Load categories and instructors
	async function loadFormData() {
		try {
			const response = await fetch('/api/learning-center');
			if (response.ok) {
				const data = await response.json();
				categories = data.data.categories || [];
				instructors = data.data.instructors || [];
			}
		} catch (err) {
			console.error('Failed to load form data:', err);
		}
	}

	onMount(() => {
		loadFormData();
	});

	// Drag and drop handlers
	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		isDragging = true;
	}

	function handleDragLeave(event: DragEvent) {
		event.preventDefault();
		isDragging = false;
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		isDragging = false;

		const files = event.dataTransfer?.files;
		if (files && files.length > 0) {
			handleVideoSelect(files[0]);
		}
	}

	function handleVideoSelect(file: File) {
		error = null;

		// Validate type
		if (!allowedVideoTypes.includes(file.type)) {
			error = 'Invalid file type. Please upload MP4, WebM, or QuickTime video.';
			return;
		}

		// Validate size
		if (file.size > maxVideoSize) {
			error = `File too large. Maximum size is ${maxVideoSize / 1024 / 1024}MB.`;
			return;
		}

		videoFile = file;
		title = file.name.replace(/\.[^/.]+$/, '').replace(/[-_]/g, ' ');

		// Get video duration
		const video = document.createElement('video');
		video.preload = 'metadata';
		video.onloadedmetadata = () => {
			videoDuration = Math.round(video.duration);
			URL.revokeObjectURL(video.src);
		};
		video.src = URL.createObjectURL(file);
	}

	function handleThumbnailSelect(file: File) {
		error = null;

		if (!allowedImageTypes.includes(file.type)) {
			error = 'Invalid thumbnail type. Please upload JPEG, PNG, or WebP image.';
			return;
		}

		if (file.size > maxThumbnailSize) {
			error = `Thumbnail too large. Maximum size is ${maxThumbnailSize / 1024 / 1024}MB.`;
			return;
		}

		thumbnailFile = file;
	}

	function handleFileInputChange(event: Event, type: 'video' | 'thumbnail') {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) {
			if (type === 'video') {
				handleVideoSelect(file);
			} else {
				handleThumbnailSelect(file);
			}
		}
	}

	function attachVideoInput(element: HTMLInputElement): () => void {
		videoInput = element;
		return () => {
			if (videoInput === element) {
				videoInput = null;
			}
		};
	}

	function attachThumbnailInput(element: HTMLInputElement): () => void {
		thumbnailInput = element;
		return () => {
			if (thumbnailInput === element) {
				thumbnailInput = null;
			}
		};
	}

	// Upload functions
	async function uploadFile(file: File, type: 'video' | 'thumbnail'): Promise<string> {
		const formData = new FormData();
		formData.append('file', file);
		formData.append('type', type);
		if (uploadSessionId) {
			formData.append('session_id', uploadSessionId);
		}

		const xhr = new XMLHttpRequest();

		return new Promise((resolve, reject) => {
			xhr.upload.addEventListener('progress', (event) => {
				if (event.lengthComputable) {
					const progress = Math.round((event.loaded / event.total) * 100);
					if (type === 'video') {
						uploadProgress = progress;
					} else {
						_thumbnailProgress = progress;
					}
				}
			});

			xhr.addEventListener('load', () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					const response = JSON.parse(xhr.responseText);
					resolve(response.data.url);
				} else {
					reject(new Error('Upload failed'));
				}
			});

			xhr.addEventListener('error', () => {
				reject(new Error('Upload failed'));
			});

			xhr.open('POST', '/api/videos/upload');
			xhr.send(formData);
		});
	}

	async function startUpload() {
		if (!videoFile) return;

		error = null;
		isUploading = true;

		try {
			// Initialize upload session
			const initResponse = await fetch('/api/videos/upload?action=init', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					filename: videoFile.name,
					size: videoFile.size,
					content_type: videoFile.type
				})
			});

			if (!initResponse.ok) {
				throw new Error('Failed to initialize upload');
			}

			const initData = await initResponse.json();
			uploadSessionId = initData.data.upload_id;

			// Upload video
			videoUrl = await uploadFile(videoFile, 'video');

			// Upload thumbnail if provided
			if (thumbnailFile) {
				thumbnailUrl = await uploadFile(thumbnailFile, 'thumbnail');
			}

			// Move to metadata step
			step = 'metadata';
		} catch (err) {
			error = err instanceof Error ? err.message : 'Upload failed';
		} finally {
			isUploading = false;
		}
	}

	async function saveVideo() {
		error = null;
		isUploading = true;

		try {
			const response = await fetch('/api/videos', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					title,
					description,
					video_url: videoUrl,
					thumbnail_url: thumbnailUrl,
					duration: videoDuration,
					categories: selectedCategories,
					instructor: instructors.find((i) => i.id === selectedInstructor) || { id: '', name: '' },
					membership_id: membershipId,
					is_premium: isPremium,
					is_published: isPublished
				})
			});

			if (!response.ok) {
				throw new Error('Failed to save video');
			}

			const data = await response.json();
			step = 'complete';

			onUploadComplete?.(data.data);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save video';
		} finally {
			isUploading = false;
		}
	}

	function reset() {
		step = 'upload';
		videoFile = null;
		thumbnailFile = null;
		uploadProgress = 0;
		_thumbnailProgress = 0;
		error = null;
		title = '';
		description = '';
		selectedCategories = [];
		selectedInstructor = '';
		videoUrl = '';
		thumbnailUrl = '';
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
	}

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = seconds % 60;
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}
</script>

<div class="video-uploader">
	{#if step === 'upload'}
		<div class="upload-step">
			<h2>Upload Video</h2>

			<!-- Video Drop Zone -->
			<div
				class={videoDropClass}
				role="button"
				tabindex="0"
				ondragover={handleDragOver}
				ondragleave={handleDragLeave}
				ondrop={handleDrop}
				onclick={() => videoInput?.click()}
				onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && videoInput?.click()}
			>
				{#if videoFile}
					<div class="file-preview">
						<Icon name="IconVideo" size={40} class="file-icon" />
						<div class="file-info">
							<span class="file-name">{videoFile.name}</span>
							<span class="file-size">{formatFileSize(videoFile.size)}</span>
							{#if videoDuration > 0}
								<span class="file-duration">{formatDuration(videoDuration)}</span>
							{/if}
						</div>
						<button
							type="button"
							class="remove-file"
							aria-label="Remove video file"
							onclick={() => (videoFile = null)}
						>
							<Icon name="IconX" size={16} />
						</button>
					</div>
				{:else}
					<Icon name="IconUpload" size={48} class="upload-icon" />
					<p>Drag and drop video here or click to browse</p>
					<span class="supported-formats">MP4, WebM, QuickTime (max 500MB)</span>
				{/if}
			</div>
			<input
				id="video-file-upload"
				name="video_file"
				type="file"
				accept="video/mp4,video/webm,video/quicktime"
				hidden
				onchange={(e: Event) => handleFileInputChange(e, 'video')}
				{@attach attachVideoInput}
			/>

			<!-- Thumbnail Upload -->
			<div class="thumbnail-section">
				<h3>Thumbnail (Optional)</h3>
				<div
					class={thumbnailDropClass}
					role="button"
					tabindex="0"
					onclick={() => thumbnailInput?.click()}
					onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && thumbnailInput?.click()}
				>
					{#if thumbnailFile}
						<img
							src={URL.createObjectURL(thumbnailFile)}
							alt="Thumbnail preview"
							width="200"
							height="112"
						/>
						<button
							type="button"
							class="remove-thumbnail"
							aria-label="Remove thumbnail"
							onclick={() => (thumbnailFile = null)}
						>
							<Icon name="IconX" size={16} />
						</button>
					{:else}
						<Icon name="IconPhoto" size={24} />
						<span>Add thumbnail</span>
					{/if}
				</div>
				<input
					id="thumbnail-file-upload"
					name="thumbnail_file"
					type="file"
					accept="image/jpeg,image/png,image/webp"
					hidden
					onchange={(e: Event) => handleFileInputChange(e, 'thumbnail')}
					{@attach attachThumbnailInput}
				/>
			</div>

			{#if error}
				<div class="error-message">{error}</div>
			{/if}

			{#if isUploading}
				<div class="upload-progress">
					<div class="progress-bar">
						<div class="progress-fill" style:width={uploadProgressWidth}></div>
					</div>
					<span class="progress-text">Uploading... {uploadProgress}%</span>
				</div>
			{/if}

			<div class="actions">
				{#if onCancel}
					<button type="button" class="btn btn-secondary" onclick={onCancel}>Cancel</button>
				{/if}
				<button
					type="button"
					class="btn btn-primary"
					disabled={!videoFile || isUploading}
					onclick={startUpload}
				>
					{isUploading ? 'Uploading...' : 'Continue'}
				</button>
			</div>
		</div>
	{/if}

	{#if step === 'metadata'}
		<div class="metadata-step">
			<h2>Video Details</h2>

			<div class="form-group">
				<label for="title">Title *</label>
				<input id="title" type="text" bind:value={title} placeholder="Enter video title" required />
			</div>

			<div class="form-group">
				<label for="description">Description</label>
				<textarea
					id="description"
					bind:value={description}
					placeholder="Enter video description"
					rows="4"></textarea>
			</div>

			<div class="form-row">
				<div class="form-group">
					<label for="instructor">Instructor</label>
					<select id="instructor" bind:value={selectedInstructor}>
						<option value="">Select instructor</option>
						{#each instructors as instructor (instructor.id)}
							<option value={instructor.id}>{instructor.name}</option>
						{/each}
					</select>
				</div>

				<fieldset class="form-group category-fieldset">
					<legend>Categories</legend>
					<div class="category-checkboxes">
						{#each categories as category (category.id)}
							<label class="checkbox-label" for="category-{category.id}">
								<input
									id="category-{category.id}"
									name="categories"
									type="checkbox"
									value={category.id}
									checked={selectedCategories.includes(category.id)}
									onchange={(e: Event) => {
										if ((e.target as HTMLInputElement).checked) {
											selectedCategories = [...selectedCategories, category.id];
										} else {
											selectedCategories = selectedCategories.filter((c) => c !== category.id);
										}
									}}
								/>
								{category.name}
							</label>
						{/each}
					</div>
				</fieldset>
			</div>

			<div class="form-row">
				<label class="toggle-label" for="video-is-premium">
					<input type="checkbox" id="video-is-premium" name="is_premium" bind:checked={isPremium} />
					<span>Premium Content</span>
				</label>
				<label class="toggle-label" for="video-is-published">
					<input type="checkbox" id="video-is-published" name="is_published" bind:checked={isPublished} />
					<span>Publish Immediately</span>
				</label>
			</div>

			{#if error}
				<div class="error-message">{error}</div>
			{/if}

			<div class="actions">
				<button type="button" class="btn btn-secondary" onclick={() => (step = 'upload')}
					>Back</button
				>
				<button
					type="button"
					class="btn btn-primary"
					disabled={!title || isUploading}
					onclick={saveVideo}
				>
					{isUploading ? 'Saving...' : 'Save Video'}
				</button>
			</div>
		</div>
	{/if}

	{#if step === 'complete'}
		<div class="complete-step">
			<div class="success-icon">
				<Icon name="IconCircleCheck" size={40} />
			</div>
			<h2>Video Uploaded Successfully!</h2>
			<p>Your video "{title}" has been uploaded and saved.</p>
			<div class="actions">
				<button type="button" class="btn btn-secondary" onclick={reset}>Upload Another</button>
				<button type="button" class="btn btn-primary" onclick={onCancel}>Done</button>
			</div>
		</div>
	{/if}
</div>

<style>
	.video-uploader {
		max-width: 700px;
		margin: 0 auto;
		padding: 2rem;
	}

	h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 1.5rem;
	}

	h3 {
		font-size: 1rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
		margin: 0 0 0.75rem;
	}

	.drop-zone {
		border: 2px dashed rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		padding: 3rem 2rem;
		text-align: center;
		cursor: pointer;
		transition: all 0.2s;
		background: rgba(255, 255, 255, 0.02);
	}

	.drop-zone:hover,
	.drop-zone--dragging {
		border-color: var(--primary-500, #e6b800);
		background: rgba(88, 166, 255, 0.1);
	}

	.drop-zone--has-file {
		border-style: solid;
		border-color: var(--success-base, #2ea043);
		background: rgba(63, 185, 80, 0.1);
		padding: 1.5rem;
	}

	:global(.upload-icon) {
		width: 48px;
		height: 48px;
		color: var(--text-tertiary, #6e7681);
		margin-bottom: 1rem;
	}

	.drop-zone p {
		color: var(--text-secondary, #8b949e);
		margin: 0 0 0.5rem;
	}

	.supported-formats {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
	}

	.file-preview {
		display: flex;
		align-items: center;
		gap: 1rem;
		text-align: left;
	}

	:global(.file-icon) {
		width: 40px;
		height: 40px;
		color: var(--success-emphasis, #3fb950);
		flex-shrink: 0;
	}

	.file-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.file-name {
		color: var(--text-primary, #f0f6fc);
		font-weight: 500;
	}

	.file-size,
	.file-duration {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
	}

	.remove-file,
	.remove-thumbnail {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: rgba(248, 81, 73, 0.2);
		border: none;
		color: var(--error-emphasis, #f85149);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.remove-file:hover,
	.remove-thumbnail:hover {
		background: rgba(248, 81, 73, 0.3);
	}

	.remove-file :global(svg),
	.remove-thumbnail :global(svg) {
		width: 16px;
		height: 16px;
	}

	.thumbnail-section {
		margin-top: 1.5rem;
	}

	.thumbnail-drop {
		width: 200px;
		height: 112px;
		border: 2px dashed rgba(255, 255, 255, 0.2);
		border-radius: 8px;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
		overflow: hidden;
	}

	.thumbnail-drop:hover {
		border-color: var(--primary-500, #e6b800);
	}

	.thumbnail-drop--has-file {
		border-style: solid;
		border-color: var(--success-base, #2ea043);
	}

	.thumbnail-drop :global(svg) {
		width: 24px;
		height: 24px;
		color: var(--text-tertiary, #6e7681);
	}

	.thumbnail-drop span {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
	}

	.thumbnail-drop img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-drop .remove-thumbnail {
		position: absolute;
		top: 4px;
		right: 4px;
		width: 24px;
		height: 24px;
	}

	.error-message {
		background: rgba(248, 81, 73, 0.1);
		border: 1px solid rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
		padding: 0.75rem 1rem;
		border-radius: 8px;
		margin-top: 1rem;
		font-size: 0.875rem;
	}

	.upload-progress {
		margin-top: 1.5rem;
	}

	.progress-bar {
		height: 8px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--primary-500, #e6b800);
		transition: width 0.3s;
	}

	.progress-text {
		display: block;
		text-align: center;
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.btn {
		padding: 0.75rem 1.5rem;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
	}

	.btn-primary {
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}


	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: var(--text-primary, #f0f6fc);
	}

	.btn-secondary:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	/* Metadata Step */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.category-fieldset {
		border: none;
		padding: 0;
		margin: 0;
	}

	.category-fieldset legend {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.5rem;
		padding: 0;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.5rem;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.15);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: var(--primary-500, #e6b800);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5rem;
		margin-bottom: 1.25rem;
	}

	.category-checkboxes {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		border-radius: 6px;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		cursor: pointer;
	}

	.checkbox-label:has(:global(input:checked)) {
		background: rgba(88, 166, 255, 0.2);
		color: var(--info-emphasis, #58a6ff);
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		color: var(--text-secondary, #8b949e);
	}

	/* Complete Step */
	.complete-step {
		text-align: center;
		padding: 2rem;
	}

	.success-icon {
		width: 80px;
		height: 80px;
		margin: 0 auto 1.5rem;
		background: rgba(63, 185, 80, 0.1);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.success-icon :global(svg) {
		width: 40px;
		height: 40px;
		color: var(--success-emphasis, #3fb950);
	}

	.complete-step p {
		color: var(--text-secondary, #8b949e);
		margin-bottom: 1.5rem;
	}

	@media (max-width: 639.98px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

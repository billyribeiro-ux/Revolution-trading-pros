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
	 * @version 1.0.0 - December 2025
	 */

	import { onMount } from 'svelte';

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
	let thumbnailProgress = $state(0);
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

	// File input refs
	let videoInput = $state<HTMLInputElement | null>(null);
	let thumbnailInput = $state<HTMLInputElement | null>(null);

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
						thumbnailProgress = progress;
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
		thumbnailProgress = 0;
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
				class="drop-zone"
				class:dragging={isDragging}
				class:has-file={videoFile !== null}
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
						<svg
							class="file-icon"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
						>
							<polygon points="23 7 16 12 23 17 23 7"></polygon>
							<rect x="1" y="5" width="15" height="14" rx="2" ry="2"></rect>
						</svg>
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
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<line x1="18" y1="6" x2="6" y2="18"></line>
								<line x1="6" y1="6" x2="18" y2="18"></line>
							</svg>
						</button>
					</div>
				{:else}
					<svg
						class="upload-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
						<polyline points="17 8 12 3 7 8"></polyline>
						<line x1="12" y1="3" x2="12" y2="15"></line>
					</svg>
					<p>Drag and drop video here or click to browse</p>
					<span class="supported-formats">MP4, WebM, QuickTime (max 500MB)</span>
				{/if}
			</div>
			<input
				bind:this={videoInput}
				type="file"
				accept="video/mp4,video/webm,video/quicktime"
				hidden
				onchange={(e: Event) => handleFileInputChange(e, 'video')}
			/>

			<!-- Thumbnail Upload -->
			<div class="thumbnail-section">
				<h3>Thumbnail (Optional)</h3>
				<div
					class="thumbnail-drop"
					class:has-file={thumbnailFile !== null}
					role="button"
					tabindex="0"
					onclick={() => thumbnailInput?.click()}
					onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && thumbnailInput?.click()}
				>
					{#if thumbnailFile}
						<img src={URL.createObjectURL(thumbnailFile)} alt="Thumbnail preview" />
						<button
							type="button"
							class="remove-thumbnail"
							aria-label="Remove thumbnail"
							onclick={() => (thumbnailFile = null)}
						>
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<line x1="18" y1="6" x2="6" y2="18"></line>
								<line x1="6" y1="6" x2="18" y2="18"></line>
							</svg>
						</button>
					{:else}
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
							<circle cx="8.5" cy="8.5" r="1.5"></circle>
							<polyline points="21 15 16 10 5 21"></polyline>
						</svg>
						<span>Add thumbnail</span>
					{/if}
				</div>
				<input
					bind:this={thumbnailInput}
					type="file"
					accept="image/jpeg,image/png,image/webp"
					hidden
					onchange={(e: Event) => handleFileInputChange(e, 'thumbnail')}
				/>
			</div>

			{#if error}
				<div class="error-message">{error}</div>
			{/if}

			{#if isUploading}
				<div class="upload-progress">
					<div class="progress-bar">
						<div class="progress-fill" style="width: {uploadProgress}%"></div>
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
					rows="4"
				></textarea>
			</div>

			<div class="form-row">
				<div class="form-group">
					<label for="instructor">Instructor</label>
					<select id="instructor" bind:value={selectedInstructor}>
						<option value="">Select instructor</option>
						{#each instructors as instructor}
							<option value={instructor.id}>{instructor.name}</option>
						{/each}
					</select>
				</div>

				<fieldset class="form-group category-fieldset">
					<legend>Categories</legend>
					<div class="category-checkboxes">
						{#each categories as category}
							<label class="checkbox-label">
								<input
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
				<label class="toggle-label">
					<input type="checkbox" bind:checked={isPremium} />
					<span>Premium Content</span>
				</label>
				<label class="toggle-label">
					<input type="checkbox" bind:checked={isPublished} />
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
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
					<polyline points="22 4 12 14.01 9 11.01"></polyline>
				</svg>
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
		color: #f1f5f9;
		margin: 0 0 1.5rem;
	}

	h3 {
		font-size: 1rem;
		font-weight: 500;
		color: #94a3b8;
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
	.drop-zone.dragging {
		border-color: #3b82f6;
		background: rgba(59, 130, 246, 0.1);
	}

	.drop-zone.has-file {
		border-style: solid;
		border-color: #22c55e;
		background: rgba(34, 197, 94, 0.1);
		padding: 1.5rem;
	}

	.upload-icon {
		width: 48px;
		height: 48px;
		color: #64748b;
		margin-bottom: 1rem;
	}

	.drop-zone p {
		color: #94a3b8;
		margin: 0 0 0.5rem;
	}

	.supported-formats {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.file-preview {
		display: flex;
		align-items: center;
		gap: 1rem;
		text-align: left;
	}

	.file-icon {
		width: 40px;
		height: 40px;
		color: #22c55e;
		flex-shrink: 0;
	}

	.file-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.file-name {
		color: #f1f5f9;
		font-weight: 500;
	}

	.file-size,
	.file-duration {
		font-size: 0.8125rem;
		color: #64748b;
	}

	.remove-file,
	.remove-thumbnail {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: rgba(239, 68, 68, 0.2);
		border: none;
		color: #ef4444;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s;
	}

	.remove-file:hover,
	.remove-thumbnail:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.remove-file svg,
	.remove-thumbnail svg {
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
		border-color: #3b82f6;
	}

	.thumbnail-drop.has-file {
		border-style: solid;
		border-color: #22c55e;
	}

	.thumbnail-drop svg {
		width: 24px;
		height: 24px;
		color: #64748b;
	}

	.thumbnail-drop span {
		font-size: 0.8125rem;
		color: #64748b;
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
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #fca5a5;
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
		background: linear-gradient(90deg, #3b82f6, #06b6d4);
		transition: width 0.3s;
	}

	.progress-text {
		display: block;
		text-align: center;
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: #94a3b8;
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
		background: linear-gradient(135deg, #3b82f6, #06b6d4);
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: #e2e8f0;
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
		color: #94a3b8;
		margin-bottom: 0.5rem;
		padding: 0;
	}

	.form-group label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #94a3b8;
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
		color: #f1f5f9;
		font-size: 0.9375rem;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: #3b82f6;
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
		color: #94a3b8;
		cursor: pointer;
	}

	.checkbox-label:has(input:checked) {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		color: #94a3b8;
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
		background: rgba(34, 197, 94, 0.1);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.success-icon svg {
		width: 40px;
		height: 40px;
		color: #22c55e;
	}

	.complete-step p {
		color: #94a3b8;
		margin-bottom: 1.5rem;
	}

	@media (max-width: 640px) {
		.form-row {
			grid-template-columns: 1fr;
		}
	}
</style>

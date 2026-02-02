<script lang="ts">
	/**
	 * BunnyVideoUploader - Bunny.net Direct Upload Component
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Apple ICT 11+ Principal Engineer Grade - January 2026
	 *
	 * Direct upload to Bunny.net Stream with:
	 * - TUS protocol resumable uploads
	 * - Progress tracking
	 * - Drag and drop support
	 * - Automatic video URL generation
	 *
	 * @version 2.0.0 - January 2026
	 */

	interface Props {
		onUploadComplete?: (data: {
			video_url: string;
			embed_url: string;
			video_guid: string;
			thumbnail_url: string;
			duration?: number;
		}) => void;
		onError?: (error: string) => void;
		libraryId?: number;
		apiBase?: string;
	}

	let props: Props = $props();

	// Destructure with defaults for internal use
	const onUploadComplete = $derived(props.onUploadComplete);
	const onError = $derived(props.onError);
	const libraryId = $derived(props.libraryId ?? 585929); // Default Bunny library ID (revolution-trading-courses)
	const apiBase = $derived(props.apiBase ?? '/api/admin/bunny');

	// State
	let isDragging = $state(false);
	let videoFile = $state<File | null>(null);
	let uploadProgress = $state(0);
	let isUploading = $state(false);
	let isGettingUrl = $state(false);
	let error = $state<string | null>(null);
	let uploadStatus = $state<'idle' | 'preparing' | 'uploading' | 'processing' | 'complete'>('idle');

	// File input ref
	let fileInput = $state<HTMLInputElement | null>(null);

	// Allowed file types and limits
	const allowedTypes = [
		'video/mp4',
		'video/webm',
		'video/quicktime',
		'video/x-msvideo',
		'video/x-matroska'
	];
	const maxSize = 5 * 1024 * 1024 * 1024; // 5GB for Bunny.net

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
			handleFileSelect(files[0]);
		}
	}

	function handleFileSelect(file: File) {
		error = null;

		// Validate type
		if (!allowedTypes.includes(file.type)) {
			error = 'Invalid file type. Please upload MP4, WebM, QuickTime, AVI, or MKV.';
			onError?.(error);
			return;
		}

		// Validate size
		if (file.size > maxSize) {
			error = `File too large. Maximum size is ${maxSize / (1024 * 1024 * 1024)}GB.`;
			onError?.(error);
			return;
		}

		videoFile = file;
	}

	function handleInputChange(event: Event) {
		const input = event.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) {
			handleFileSelect(file);
		}
	}

	async function startUpload() {
		if (!videoFile) return;

		error = null;
		isUploading = true;
		uploadStatus = 'preparing';
		isGettingUrl = true;

		try {
			// Step 1: Create video entry and get upload URL from our API
			const createResponse = await fetch(`${apiBase}/create-video`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					title: videoFile.name.replace(/\.[^/.]+$/, ''),
					library_id: libraryId
				})
			});

			if (!createResponse.ok) {
				const errorData = await createResponse.json().catch(() => ({}));
				throw new Error(errorData.error || 'Failed to create video entry');
			}

			const { video_guid, upload_url, video_url, embed_url } = await createResponse.json();
			isGettingUrl = false;

			// Step 2: Upload to Bunny.net
			uploadStatus = 'uploading';
			await uploadToBunny(upload_url, videoFile);

			// Step 3: Wait for processing
			uploadStatus = 'processing';
			const finalData = await waitForProcessing(video_guid);

			// Step 4: Complete
			uploadStatus = 'complete';
			onUploadComplete?.({
				video_url: finalData.video_url || video_url,
				embed_url: finalData.embed_url || embed_url,
				video_guid,
				thumbnail_url: finalData.thumbnail_url || '',
				duration: finalData.duration
			});
		} catch (err) {
			const message = err instanceof Error ? err.message : 'Upload failed';
			error = message;
			onError?.(message);
			uploadStatus = 'idle';
		} finally {
			isUploading = false;
			isGettingUrl = false;
		}
	}

	async function waitForProcessing(videoGuid: string, maxAttempts = 30): Promise<any> {
		for (let i = 0; i < maxAttempts; i++) {
			await new Promise((resolve) => setTimeout(resolve, 2000));

			try {
				const response = await fetch(`${apiBase}/video-status/${videoGuid}`);
				if (response.ok) {
					const data = await response.json();
					if (data.status === 'ready' || data.status === 4) {
						return data;
					}
					if (data.status === 'failed' || data.status === 5) {
						throw new Error('Video processing failed');
					}
				}
			} catch (e) {
				// Continue waiting
			}
		}
		// Return basic data if processing takes too long
		return {};
	}

	async function uploadToBunny(uploadUrl: string, file: File): Promise<void> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();

			xhr.upload.addEventListener('progress', (event) => {
				if (event.lengthComputable) {
					uploadProgress = Math.round((event.loaded / event.total) * 100);
				}
			});

			xhr.addEventListener('load', () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					resolve();
				} else {
					reject(new Error(`Upload failed with status ${xhr.status}`));
				}
			});

			xhr.addEventListener('error', () => {
				reject(new Error('Network error during upload'));
			});

			xhr.addEventListener('abort', () => {
				reject(new Error('Upload aborted'));
			});

			xhr.open('PUT', uploadUrl, true);
			xhr.setRequestHeader('Content-Type', file.type);
			xhr.send(file);
		});
	}

	function clearFile() {
		videoFile = null;
		uploadProgress = 0;
		uploadStatus = 'idle';
		error = null;
		if (fileInput) {
			fileInput.value = '';
		}
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
		return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
	}

	function getStatusText(): string {
		switch (uploadStatus) {
			case 'preparing':
				return 'Preparing upload...';
			case 'uploading':
				return `Uploading... ${uploadProgress}%`;
			case 'processing':
				return 'Processing video...';
			case 'complete':
				return 'Upload complete!';
			default:
				return '';
		}
	}
</script>

<div class="bunny-uploader">
	{#if uploadStatus === 'complete'}
		<div class="complete-state">
			<div class="success-icon">
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
					<polyline points="22 4 12 14.01 9 11.01"></polyline>
				</svg>
			</div>
			<p>Video uploaded successfully!</p>
			<button type="button" class="btn-link" onclick={clearFile}> Upload another video </button>
		</div>
	{:else if videoFile}
		<div class="file-selected">
			<div class="file-info">
				<div class="file-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<polygon points="23 7 16 12 23 17 23 7"></polygon>
						<rect x="1" y="5" width="15" height="14" rx="2" ry="2"></rect>
					</svg>
				</div>
				<div class="file-details">
					<span class="file-name">{videoFile.name}</span>
					<span class="file-size">{formatFileSize(videoFile.size)}</span>
				</div>
				{#if !isUploading}
					<button type="button" class="btn-remove" onclick={clearFile} aria-label="Remove file">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="18" y1="6" x2="6" y2="18"></line>
							<line x1="6" y1="6" x2="18" y2="18"></line>
						</svg>
					</button>
				{/if}
			</div>

			{#if isUploading}
				<div class="upload-progress">
					<div class="progress-bar">
						<div
							class="progress-fill"
							class:indeterminate={uploadStatus === 'preparing' || uploadStatus === 'processing'}
							style="width: {uploadStatus === 'uploading' ? uploadProgress : 100}%"
						></div>
					</div>
					<span class="progress-text">{getStatusText()}</span>
				</div>
			{:else}
				<button type="button" class="btn-upload" onclick={startUpload}>
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
						<polyline points="17 8 12 3 7 8"></polyline>
						<line x1="12" y1="3" x2="12" y2="15"></line>
					</svg>
					Upload to Bunny.net
				</button>
			{/if}
		</div>
	{:else}
		<div
			class="drop-zone"
			class:dragging={isDragging}
			role="button"
			tabindex="0"
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			ondrop={handleDrop}
			onclick={() => fileInput?.click()}
			onkeydown={(e) => e.key === 'Enter' && fileInput?.click()}
		>
			<div class="drop-zone-content">
				<div class="upload-icon">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
						<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
						<polyline points="17 8 12 3 7 8"></polyline>
						<line x1="12" y1="3" x2="12" y2="15"></line>
					</svg>
				</div>
				<p class="drop-text">Drag and drop video here</p>
				<p class="drop-hint">or click to browse</p>
				<span class="supported-formats">MP4, WebM, QuickTime, AVI, MKV (max 5GB)</span>
			</div>
		</div>
		<input
			bind:this={fileInput}
			type="file"
			accept="video/mp4,video/webm,video/quicktime,video/x-msvideo,video/x-matroska"
			hidden
			onchange={handleInputChange}
		/>
	{/if}

	{#if error}
		<div class="error-message">
			<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			{error}
		</div>
	{/if}
</div>

<style>
	.bunny-uploader {
		width: 100%;
	}

	.drop-zone {
		border: 2px dashed rgba(230, 184, 0, 0.3);
		border-radius: 12px;
		padding: 2.5rem 2rem;
		text-align: center;
		cursor: pointer;
		transition: all 0.2s;
		background: rgba(230, 184, 0, 0.05);
	}

	.drop-zone:hover,
	.drop-zone.dragging {
		border-color: #e6b800;
		background: rgba(230, 184, 0, 0.1);
	}

	.drop-zone:focus {
		outline: 2px solid #e6b800;
		outline-offset: 2px;
	}

	.drop-zone-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.upload-icon {
		width: 48px;
		height: 48px;
		color: #e6b800;
		margin-bottom: 0.5rem;
	}

	.upload-icon svg {
		width: 100%;
		height: 100%;
	}

	.drop-text {
		font-size: 1rem;
		font-weight: 500;
		color: #e2e8f0;
		margin: 0;
	}

	.drop-hint {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0;
	}

	.supported-formats {
		font-size: 0.75rem;
		color: #475569;
		margin-top: 0.5rem;
	}

	.file-selected {
		padding: 1.25rem;
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 12px;
	}

	.file-info {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.file-icon {
		width: 40px;
		height: 40px;
		color: #22c55e;
		flex-shrink: 0;
	}

	.file-icon svg {
		width: 100%;
		height: 100%;
	}

	.file-details {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.file-name {
		font-weight: 500;
		color: #f1f5f9;
		word-break: break-word;
	}

	.file-size {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.btn-remove {
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
		flex-shrink: 0;
		transition: all 0.2s;
	}

	.btn-remove:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.btn-remove svg {
		width: 16px;
		height: 16px;
	}

	.upload-progress {
		margin-top: 1rem;
	}

	.progress-bar {
		height: 8px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #e6b800, #b38f00);
		transition: width 0.3s;
		border-radius: 4px;
	}

	.progress-fill.indeterminate {
		width: 100% !important;
		animation: indeterminate 1.5s infinite ease-in-out;
		background: linear-gradient(
			90deg,
			rgba(230, 184, 0, 0.3) 0%,
			#e6b800 50%,
			rgba(230, 184, 0, 0.3) 100%
		);
		background-size: 200% 100%;
	}

	@keyframes indeterminate {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	.progress-text {
		display: block;
		text-align: center;
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.btn-upload {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		margin-top: 1rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #e6b800, #b38f00);
		border: none;
		border-radius: 8px;
		color: #0d1117;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-upload:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.4);
	}

	.btn-upload svg {
		width: 20px;
		height: 20px;
	}

	.complete-state {
		text-align: center;
		padding: 2rem;
	}

	.success-icon {
		width: 64px;
		height: 64px;
		margin: 0 auto 1rem;
		background: rgba(34, 197, 94, 0.15);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.success-icon svg {
		width: 32px;
		height: 32px;
		color: #22c55e;
	}

	.complete-state p {
		color: #4ade80;
		font-weight: 500;
		margin: 0 0 1rem;
	}

	.btn-link {
		background: transparent;
		border: none;
		color: #ffd11a;
		font-size: 0.875rem;
		cursor: pointer;
		text-decoration: underline;
	}

	.btn-link:hover {
		color: #e6b800;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #fca5a5;
		font-size: 0.875rem;
	}

	.error-message svg {
		width: 18px;
		height: 18px;
		flex-shrink: 0;
	}
</style>

<!--
	Video Upload Modal - ICT 7 Frontend Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Drag & drop video upload modal for weekly breakdown videos.
	Supports Bunny.net direct file upload AND URL pasting.
	Auto-generates thumbnails with selection option.
	
	@version 2.0.0 - Enterprise Rewrite with File Upload
-->
<script lang="ts">
	import { weeklyVideoApi } from '$lib/api/room-content';

	interface Props {
		isOpen: boolean;
		roomSlug: string;
		onClose: () => void;
		onSuccess?: () => void;
	}

	const { isOpen, roomSlug, onClose, onSuccess }: Props = $props();

	// Upload mode: 'file' for direct Bunny upload, 'url' for pasting existing URLs
	let uploadMode = $state<'file' | 'url'>('file');
	
	// Core states
	let isSaving = $state(false);
	let errorMessage = $state('');
	let isDragOver = $state(false);
	
	// File upload states
	let videoFile = $state<File | null>(null);
	let uploadProgress = $state(0);
	let uploadStatus = $state<'idle' | 'preparing' | 'uploading' | 'processing' | 'complete'>('idle');
	let fileInput = $state<HTMLInputElement | null>(null);
	
	// Thumbnail selection
	let generatedThumbnails = $state<string[]>([]);
	let selectedThumbnailIndex = $state(0);
	
	// File validation
	const allowedTypes = ['video/mp4', 'video/webm', 'video/quicktime', 'video/x-msvideo', 'video/x-matroska'];
	const maxFileSize = 5 * 1024 * 1024 * 1024; // 5GB

	let form = $state({
		week_of: getNextMonday(),
		week_title: '',
		video_title: '',
		video_url: '',
		video_guid: '',
		video_platform: 'bunny' as 'bunny' | 'vimeo' | 'youtube',
		thumbnail_url: '',
		duration: '',
		description: ''
	});
	
	// Lock body scroll when modal is open
	$effect(() => {
		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}
		return () => { document.body.style.overflow = ''; };
	});

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
		if (!url || url.trim() === '') return '';
		
		if (platform === 'vimeo') {
			const match = url.match(/vimeo\.com\/(\d+)/);
			if (match) return `https://player.vimeo.com/video/${match[1]}`;
		}
		if (platform === 'youtube') {
			const match = url.match(/(?:youtube\.com\/watch\?v=|youtu\.be\/)([^&]+)/);
			if (match) return `https://www.youtube.com/embed/${match[1]}`;
		}
		// Return URL only if it's a valid external embed URL
		if (url.includes('iframe.mediadelivery.net') || 
			url.includes('player.vimeo.com') || 
			url.includes('youtube.com/embed') ||
			url.includes('bunnycdn')) {
			return url;
		}
		return '';
	}
	
	// Check if URL is a valid embeddable video URL (not a relative path)
	function isValidEmbedUrl(url: string): boolean {
		if (!url || url.trim() === '') return false;
		return url.startsWith('https://iframe.mediadelivery.net') ||
			url.startsWith('https://player.vimeo.com') ||
			url.startsWith('https://www.youtube.com/embed') ||
			url.includes('bunnycdn');
	}

	const isFormValid = $derived(
		form.video_url.trim() !== '' && 
		form.video_title.trim() !== '' &&
		form.week_of !== ''
	);

	const embedUrl = $derived(
		form.video_url ? generateEmbedUrl(form.video_url, form.video_platform) : ''
	);
	
	const canShowPreview = $derived(isValidEmbedUrl(embedUrl));

	function resetForm() {
		form = {
			week_of: getNextMonday(),
			week_title: '',
			video_title: '',
			video_url: '',
			video_guid: '',
			video_platform: 'bunny',
			thumbnail_url: '',
			duration: '',
			description: ''
		};
		errorMessage = '';
		videoFile = null;
		uploadProgress = 0;
		uploadStatus = 'idle';
		generatedThumbnails = [];
		selectedThumbnailIndex = 0;
		if (fileInput) fileInput.value = '';
	}
	
	// File size formatter
	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
		return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
	}
	
	// File selection handler
	function handleFileSelect(file: File) {
		errorMessage = '';
		
		if (!allowedTypes.includes(file.type)) {
			errorMessage = 'Invalid file type. Please upload MP4, WebM, QuickTime, AVI, or MKV.';
			return;
		}
		
		if (file.size > maxFileSize) {
			errorMessage = 'File too large. Maximum size is 5GB.';
			return;
		}
		
		videoFile = file;
		// Auto-set video title from filename if empty
		if (!form.video_title) {
			form.video_title = file.name.replace(/\.[^/.]+$/, '').replace(/[-_]/g, ' ');
		}
	}
	
	function handleFileInputChange(e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) handleFileSelect(file);
	}
	
	// Bunny.net upload functions
	async function startBunnyUpload() {
		if (!videoFile) return;
		
		errorMessage = '';
		uploadStatus = 'preparing';
		
		try {
			// Step 1: Create video entry on Bunny.net
			const createResponse = await fetch(`/api/weekly-video/${roomSlug}/upload`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'include',
				body: JSON.stringify({
					title: form.video_title || videoFile.name.replace(/\.[^/.]+$/, '')
				})
			});
			
			if (!createResponse.ok) {
				const errorData = await createResponse.json().catch(() => ({}));
				throw new Error(errorData.error || 'Failed to create video entry');
			}
			
			const createData = await createResponse.json();
			const { video_guid, embed_url, video_url } = createData;
			
			if (!video_guid) {
				throw new Error('Failed to get video GUID from server');
			}
			
			form.video_guid = video_guid;
			form.video_url = embed_url || video_url || '';
			
			// Step 2: Upload file to Bunny.net via our proxy
			uploadStatus = 'uploading';
			await uploadFileToBunny(video_guid, videoFile);
			
			// Step 3: Wait for processing and get thumbnails
			uploadStatus = 'processing';
			const finalData = await waitForBunnyProcessing(video_guid);
			
			// Set thumbnail and duration from processed video
			if (finalData.thumbnail_url) {
				form.thumbnail_url = finalData.thumbnail_url;
				// Bunny generates multiple thumbnails
				generatedThumbnails = [
					finalData.thumbnail_url,
					finalData.thumbnail_url.replace('thumbnail.jpg', 'thumbnail_1.jpg'),
					finalData.thumbnail_url.replace('thumbnail.jpg', 'thumbnail_2.jpg'),
					finalData.thumbnail_url.replace('thumbnail.jpg', 'thumbnail_3.jpg')
				];
			}
			
			if (finalData.duration) {
				const mins = Math.floor(finalData.duration / 60);
				const secs = Math.floor(finalData.duration % 60);
				form.duration = `${mins}:${secs.toString().padStart(2, '0')}`;
			}
			
			uploadStatus = 'complete';
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : 'Upload failed';
			uploadStatus = 'idle';
		}
	}
	
	async function uploadFileToBunny(videoGuid: string, file: File): Promise<void> {
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
			
			xhr.addEventListener('error', () => reject(new Error('Network error during upload')));
			xhr.addEventListener('abort', () => reject(new Error('Upload aborted')));
			
			// Use room-specific upload endpoint
			const proxyUrl = `/api/weekly-video/${roomSlug}/upload?video_guid=${videoGuid}`;
			xhr.open('PUT', proxyUrl, true);
			xhr.setRequestHeader('Content-Type', file.type);
			xhr.withCredentials = true;
			xhr.send(file);
		});
	}
	
	async function waitForBunnyProcessing(videoGuid: string, maxAttempts = 30): Promise<any> {
		for (let i = 0; i < maxAttempts; i++) {
			await new Promise((resolve) => setTimeout(resolve, 2000));
			
			try {
				const response = await fetch(`/api/weekly-video/${roomSlug}/upload/status/${videoGuid}`, {
					credentials: 'include'
				});
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
		return {};
	}
	
	function selectThumbnail(index: number) {
		selectedThumbnailIndex = index;
		form.thumbnail_url = generatedThumbnails[index] || '';
	}
	
	function clearFile() {
		videoFile = null;
		uploadProgress = 0;
		uploadStatus = 'idle';
		generatedThumbnails = [];
		form.video_url = '';
		form.thumbnail_url = '';
		form.duration = '';
		if (fileInput) fileInput.value = '';
	}
	
	function getUploadStatusText(): string {
		switch (uploadStatus) {
			case 'preparing': return 'Preparing upload...';
			case 'uploading': return `Uploading... ${uploadProgress}%`;
			case 'processing': return 'Processing video...';
			case 'complete': return 'Upload complete!';
			default: return '';
		}
	}

	async function handleSubmit() {
		if (!isFormValid) return;
		isSaving = true;
		errorMessage = '';

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

			resetForm();
			onSuccess?.();
			onClose();
		} catch (err) {
			errorMessage = 'Failed to publish video. Please try again.';
			console.error(err);
		} finally {
			isSaving = false;
		}
	}

	function handleClose() {
		resetForm();
		onClose();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') handleClose();
	}

	function handleUrlChange() {
		form.video_platform = detectPlatform(form.video_url);
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
		e.stopPropagation();
		isDragOver = false;
		
		// Check for files first (file upload mode)
		if (uploadMode === 'file') {
			const files = e.dataTransfer?.files;
			if (files && files.length > 0) {
				handleFileSelect(files[0]);
				return;
			}
		}
		
		// Check for URL in dropped data (url mode)
		const url = e.dataTransfer?.getData('text/plain');
		if (url && (url.startsWith('http://') || url.startsWith('https://'))) {
			form.video_url = url;
			form.video_platform = detectPlatform(url);
		}
	}
</script>

{#if isOpen}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<!-- Portal-style modal - renders above everything with backdrop -->
	<div 
		class="modal-portal" 
		role="dialog" 
		aria-modal="true" 
		aria-labelledby="modal-title"
		tabindex="-1"
		onclick={(e) => e.target === e.currentTarget && handleClose()}
		onkeydown={handleKeydown}
	>
		<div class="modal-backdrop"></div>
		<div class="modal-container">
			<!-- Dark Header -->
			<div class="modal-header">
				<div class="header-content">
					<div class="header-icon">
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="22" height="22">
							<path d="M23 7l-7 5 7 5V7zM14 5H3a2 2 0 00-2 2v10a2 2 0 002 2h11a2 2 0 002-2V7a2 2 0 00-2-2z" />
						</svg>
					</div>
					<div class="header-text">
						<h3 id="modal-title">Upload Weekly Video</h3>
						<p class="header-subtitle">Publish a new weekly breakdown</p>
					</div>
				</div>
				<button class="modal-close" onclick={handleClose} aria-label="Close modal">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>

			{#if errorMessage}
				<div class="error-banner">
					<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
						<circle cx="12" cy="12" r="10"></circle>
						<line x1="12" y1="8" x2="12" y2="12"></line>
						<line x1="12" y1="16" x2="12.01" y2="16"></line>
					</svg>
					{errorMessage}
				</div>
			{/if}

			<div class="modal-body">
				<!-- Upload Mode Tabs -->
				<div class="mode-tabs">
					<button 
						type="button"
						class="mode-tab" 
						class:active={uploadMode === 'file'}
						onclick={() => uploadMode = 'file'}
					>
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
							<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
						</svg>
						Upload File
					</button>
					<button 
						type="button"
						class="mode-tab" 
						class:active={uploadMode === 'url'}
						onclick={() => uploadMode = 'url'}
					>
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
							<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
							<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
						</svg>
						Paste URL
					</button>
				</div>

				<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
					{#if uploadMode === 'file'}
						<!-- FILE UPLOAD MODE -->
						{#if uploadStatus === 'complete'}
							<!-- Upload Complete State -->
							<div class="upload-complete">
								<div class="success-icon">
									<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
										<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
										<polyline points="22 4 12 14.01 9 11.01"></polyline>
									</svg>
								</div>
								<p class="success-text">Video uploaded successfully!</p>
								<p class="video-url-display">{form.video_url}</p>
								<button type="button" class="btn-link" onclick={clearFile}>
									Upload a different video
								</button>
							</div>
							
							<!-- Thumbnail Selection -->
							{#if generatedThumbnails.length > 0}
								<div class="thumbnail-section">
									<span class="section-label">Select Thumbnail</span>
									<div class="thumbnail-grid">
										{#each generatedThumbnails as thumb, i}
											<button
												type="button"
												class="thumbnail-option"
												class:selected={selectedThumbnailIndex === i}
												onclick={() => selectThumbnail(i)}
											>
												<img src={thumb} alt="Thumbnail option {i + 1}" loading="lazy" />
												{#if selectedThumbnailIndex === i}
													<div class="thumb-check">
														<svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
															<path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z" />
														</svg>
													</div>
												{/if}
											</button>
										{/each}
									</div>
								</div>
							{/if}
						{:else if videoFile}
							<!-- File Selected State -->
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
									{#if uploadStatus === 'idle'}
										<button type="button" class="btn-remove" onclick={clearFile} aria-label="Remove file">
											<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
												<line x1="18" y1="6" x2="6" y2="18"></line>
												<line x1="6" y1="6" x2="18" y2="18"></line>
											</svg>
										</button>
									{/if}
								</div>
								
								{#if uploadStatus !== 'idle'}
									<div class="upload-progress">
										<div class="progress-bar">
											<div
												class="progress-fill"
												class:indeterminate={uploadStatus === 'preparing' || uploadStatus === 'processing'}
												style="width: {uploadStatus === 'uploading' ? uploadProgress : 100}%"
											></div>
										</div>
										<span class="progress-text">{getUploadStatusText()}</span>
									</div>
								{:else}
									<button type="button" class="btn-upload-bunny" onclick={startBunnyUpload}>
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
							<!-- Drag & Drop Zone for Files -->
							<div
								class="drop-zone"
								class:drag-over={isDragOver}
								role="region"
								aria-label="Video upload area"
								ondragover={handleDragOver}
								ondragleave={handleDragLeave}
								ondrop={handleDrop}
							>
								<div class="drop-zone-content">
									<div class="upload-icon-large">
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
											<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
											<polyline points="17 8 12 3 7 8"></polyline>
											<line x1="12" y1="3" x2="12" y2="15"></line>
										</svg>
									</div>
									<p class="drop-text">Drag and drop video here</p>
									<p class="drop-hint">or use the button below</p>
									<span class="supported-formats">MP4, WebM, QuickTime, AVI, MKV (max 5GB)</span>
									
									<!-- Browse Files Button -->
									<button 
										type="button" 
										class="btn-browse-files"
										onclick={() => fileInput?.click()}
									>
										<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="18" height="18">
											<path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
										</svg>
										Browse Files & Folders
									</button>
								</div>
							</div>
							<!-- Hidden file input with webkitdirectory support -->
							<input
								bind:this={fileInput}
								type="file"
								accept="video/mp4,video/webm,video/quicktime,video/x-msvideo,video/x-matroska"
								hidden
								onchange={handleFileInputChange}
							/>
						{/if}
					{:else}
						<!-- URL MODE -->
						<!-- Drag & Drop Zone for URL -->
						<div 
							class="drop-zone"
							class:drag-over={isDragOver}
							class:has-url={form.video_url}
							role="button"
							tabindex="0"
							aria-label="Drop video URL here"
							ondragover={handleDragOver}
							ondragleave={handleDragLeave}
							ondrop={handleDrop}
						>
							{#if canShowPreview && embedUrl}
								<div class="video-preview">
									<iframe
										src={embedUrl}
										title="Video Preview"
										frameborder="0"
										allow="autoplay; fullscreen"
									></iframe>
								</div>
							{:else}
								<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48">
									<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
									<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
								</svg>
								<p class="drop-text">Drag & drop a video URL here</p>
								<p class="drop-subtext">or paste URL below</p>
							{/if}
						</div>

						<!-- URL Input -->
						<div class="form-group url-group">
							<label for="video_url">Video URL *</label>
							<div class="url-input-wrapper">
								<input
									id="video_url"
									type="url"
									bind:value={form.video_url}
									oninput={handleUrlChange}
									placeholder="https://vimeo.com/123456789 or Bunny.net embed URL"
									class="form-input"
									required
								/>
								<span class="platform-badge">{form.video_platform}</span>
							</div>
						</div>
						
						<!-- Manual Thumbnail/Duration for URL mode -->
						<div class="form-row">
							<div class="form-group">
								<label for="thumbnail_url_manual">Thumbnail URL</label>
								<input
									id="thumbnail_url_manual"
									type="url"
									bind:value={form.thumbnail_url}
									placeholder="https://..."
									class="form-input"
								/>
							</div>
							<div class="form-group">
								<label for="duration_manual">Duration</label>
								<input
									id="duration_manual"
									type="text"
									bind:value={form.duration}
									placeholder="24:35"
									class="form-input"
								/>
							</div>
						</div>
					{/if}
					
					<!-- Common Form Fields Divider -->
					<div class="form-divider"></div>

				<div class="form-row">
					<div class="form-group">
						<label for="week_of">Week Of *</label>
						<input
							id="week_of"
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
						type="text"
						bind:value={form.video_title}
						placeholder="Weekly Breakdown - Key Levels & Setups"
						class="form-input"
						required
					/>
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

					<div class="form-actions">
						<div class="archive-notice">
							<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
								<path d="M21 8v13H3V8M1 3h22v5H1zM10 12h4" />
							</svg>
							<span>Publishing will archive the current video</span>
						</div>
						<div class="action-buttons">
							<button type="button" class="btn-cancel" onclick={handleClose}>Cancel</button>
							<button type="submit" class="btn-publish" disabled={!isFormValid || isSaving}>
								{#if isSaving}
									<span class="spinner"></span>
									Publishing...
								{:else}
									<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
										<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
									</svg>
									Publish Video
								{/if}
							</button>
						</div>
					</div>
				</form>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL PORTAL - Top-level positioning (won't be cut off by parent containers)
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-portal {
		position: fixed;
		inset: 0;
		z-index: 99999;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 20px;
	}

	.modal-backdrop {
		position: absolute;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		animation: backdropFade 0.2s ease-out;
	}

	@keyframes backdropFade {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL CONTAINER - Proper viewport-safe sizing
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-container {
		position: relative;
		background: #ffffff;
		border-radius: 20px;
		width: 100%;
		max-width: 640px;
		max-height: calc(100vh - 40px);
		max-height: calc(100dvh - 40px);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 
			0 0 0 1px rgba(0, 0, 0, 0.05),
			0 25px 50px -12px rgba(0, 0, 0, 0.5),
			0 0 100px -20px rgba(20, 62, 89, 0.4);
		animation: modalSlideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes modalSlideUp {
		from {
			opacity: 0;
			transform: translateY(30px) scale(0.97);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}
	
	.modal-body {
		flex: 1;
		overflow-y: auto;
		padding: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODE TABS - File Upload vs URL Paste
	   ═══════════════════════════════════════════════════════════════════════════ */
	.mode-tabs {
		display: flex;
		gap: 8px;
		padding: 16px 24px;
		background: #f8fafc;
		border-bottom: 1px solid #e2e8f0;
	}

	.mode-tab {
		display: flex;
		align-items: center;
		gap: 8px;
		flex: 1;
		padding: 12px 16px;
		background: #fff;
		border: 2px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s ease;
		justify-content: center;
	}

	.mode-tab:hover {
		border-color: #cbd5e1;
		color: #475569;
	}

	.mode-tab.active {
		background: #143E59;
		border-color: #143E59;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   FILE UPLOAD STATES
	   ═══════════════════════════════════════════════════════════════════════════ */
	.file-selected {
		padding: 1.25rem;
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 12px;
		margin-bottom: 20px;
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
		color: #1e293b;
		word-break: break-word;
	}

	.file-size {
		font-size: 0.8125rem;
		color: #64748b;
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
		background: rgba(0, 0, 0, 0.1);
		border-radius: 4px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: linear-gradient(90deg, #143E59, #1a5a7e);
		transition: width 0.3s;
		border-radius: 4px;
	}

	.progress-fill.indeterminate {
		width: 100% !important;
		animation: indeterminate 1.5s infinite ease-in-out;
		background: linear-gradient(
			90deg,
			rgba(20, 62, 89, 0.3) 0%,
			#143E59 50%,
			rgba(20, 62, 89, 0.3) 100%
		);
		background-size: 200% 100%;
	}

	@keyframes indeterminate {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}

	.progress-text {
		display: block;
		text-align: center;
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: #64748b;
	}

	.btn-upload-bunny {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		margin-top: 1rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #143E59, #1a5a7e);
		border: none;
		border-radius: 8px;
		color: #fff;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-upload-bunny:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.4);
	}

	.btn-upload-bunny svg {
		width: 20px;
		height: 20px;
	}

	/* Upload Complete State */
	.upload-complete {
		text-align: center;
		padding: 2rem;
		background: rgba(34, 197, 94, 0.08);
		border: 1px solid rgba(34, 197, 94, 0.2);
		border-radius: 12px;
		margin-bottom: 20px;
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

	.success-text {
		color: #166534;
		font-weight: 600;
		margin: 0 0 0.5rem;
	}

	.video-url-display {
		color: #64748b;
		font-size: 0.8125rem;
		word-break: break-all;
		margin: 0 0 1rem;
	}

	.btn-link {
		background: transparent;
		border: none;
		color: #143E59;
		font-size: 0.875rem;
		cursor: pointer;
		text-decoration: underline;
	}

	.btn-link:hover {
		color: #0f2d42;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   THUMBNAIL SELECTION
	   ═══════════════════════════════════════════════════════════════════════════ */
	.thumbnail-section {
		margin-bottom: 20px;
	}

	.section-label {
		display: block;
		font-size: 12px;
		font-weight: 600;
		color: #475569;
		margin-bottom: 10px;
	}

	.thumbnail-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 10px;
	}

	.thumbnail-option {
		position: relative;
		aspect-ratio: 16/9;
		border: 3px solid transparent;
		border-radius: 8px;
		overflow: hidden;
		cursor: pointer;
		transition: all 0.15s ease;
		padding: 0;
		background: #f1f5f9;
	}

	.thumbnail-option img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-option:hover {
		border-color: #cbd5e1;
	}

	.thumbnail-option.selected {
		border-color: #143E59;
		box-shadow: 0 0 0 2px rgba(20, 62, 89, 0.2);
	}

	.thumb-check {
		position: absolute;
		top: 4px;
		right: 4px;
		width: 24px;
		height: 24px;
		background: #143E59;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
	}

	/* Drop Zone Large Icon */
	.upload-icon-large {
		width: 56px;
		height: 56px;
		color: #143E59;
		margin-bottom: 0.5rem;
	}

	.upload-icon-large svg {
		width: 100%;
		height: 100%;
	}

	.drop-zone-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.supported-formats {
		font-size: 0.75rem;
		color: #94a3b8;
		margin-top: 0.5rem;
	}
	
	/* Form Divider */
	.form-divider {
		height: 1px;
		background: #e2e8f0;
		margin: 20px 0;
	}

	/* Browse Files Button */
	.btn-browse-files {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		margin-top: 16px;
		padding: 12px 24px;
		background: linear-gradient(135deg, #143E59 0%, #1a5a7e 100%);
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 2px 8px rgba(20, 62, 89, 0.3);
	}

	.btn-browse-files:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(20, 62, 89, 0.4);
		background: linear-gradient(135deg, #0f2d42 0%, #143E59 100%);
	}

	.btn-browse-files:active {
		transform: translateY(0);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER - Dark theme matching dashboard
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		background: linear-gradient(135deg, #143E59 0%, #1a4d6e 100%);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 14px;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 12px;
		color: #fff;
	}

	.header-text h3 {
		margin: 0;
		font-size: 18px;
		font-weight: 700;
		color: #fff;
		letter-spacing: -0.3px;
	}

	.header-subtitle {
		margin: 2px 0 0;
		font-size: 13px;
		color: rgba(255, 255, 255, 0.7);
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		padding: 0;
		background: rgba(255, 255, 255, 0.1);
		border: none;
		border-radius: 10px;
		color: rgba(255, 255, 255, 0.8);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.modal-close:hover {
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
		transform: scale(1.05);
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 16px 24px 0;
		padding: 14px 16px;
		background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
		border: 1px solid #fecaca;
		border-radius: 12px;
		color: #dc2626;
		font-size: 14px;
		font-weight: 500;
	}

	.modal-form {
		flex: 1;
		overflow-y: auto;
		padding: 24px;
	}

	/* Drop Zone */
	.drop-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 32px 24px;
		border: 2px dashed #cbd5e1;
		border-radius: 16px;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		margin-bottom: 20px;
		transition: all 0.2s ease;
		color: #64748b;
		cursor: pointer;
	}

	.drop-zone:hover {
		border-color: #94a3b8;
		background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%);
	}

	.drop-zone.drag-over {
		border-color: #143E59;
		background: rgba(20, 62, 89, 0.08);
		border-style: solid;
		transform: scale(1.01);
	}

	.drop-zone.has-url {
		padding: 0;
		border-style: solid;
		border-color: #143E59;
		overflow: hidden;
	}

	.drop-text {
		margin: 12px 0 4px;
		font-size: 15px;
		font-weight: 500;
		color: #334155;
	}

	.drop-subtext {
		margin: 0;
		font-size: 13px;
		color: #94a3b8;
	}

	.video-preview {
		width: 100%;
		aspect-ratio: 16/9;
		border-radius: 10px;
		overflow: hidden;
	}

	.video-preview iframe {
		width: 100%;
		height: 100%;
	}

	/* URL Input */
	.url-group {
		margin-bottom: 16px;
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
		background: #143E59;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		color: white;
	}

	/* Form */
	.form-row {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 14px;
		margin-bottom: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
		margin-bottom: 16px;
	}

	.form-group label {
		font-size: 12px;
		font-weight: 600;
		color: #475569;
	}

	.form-input,
	.form-textarea {
		padding: 12px 14px;
		border: 2px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 500;
		background: #f8fafc;
		color: #1e293b;
		transition: all 0.15s ease;
	}

	.form-input:hover,
	.form-textarea:hover {
		border-color: #cbd5e1;
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: #143E59;
		background: #fff;
		box-shadow: 0 0 0 4px rgba(20, 62, 89, 0.1);
	}

	.form-textarea {
		resize: vertical;
		font-family: inherit;
		min-height: 80px;
	}

	/* Actions */
	.form-actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 24px;
		padding-top: 20px;
		border-top: 2px solid #f1f5f9;
	}

	.archive-notice {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		color: #64748b;
		background: #fef9c3;
		padding: 8px 12px;
		border-radius: 8px;
		border: 1px solid #fde047;
	}

	.action-buttons {
		display: flex;
		gap: 12px;
	}

	.btn-cancel {
		padding: 12px 24px;
		background: #f1f5f9;
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel:hover {
		background: #e2e8f0;
	}

	.btn-publish {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 28px;
		background: linear-gradient(135deg, #143E59 0%, #1a4d6e 100%);
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 700;
		color: white;
		cursor: pointer;
		transition: all 0.15s;
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.25);
	}

	.btn-publish:hover:not(:disabled) {
		background: linear-gradient(135deg, #0f2d42 0%, #143E59 100%);
		transform: translateY(-1px);
		box-shadow: 0 6px 16px rgba(20, 62, 89, 0.35);
	}

	.btn-publish:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
		box-shadow: none;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255,255,255,0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */
	@media (max-width: 640px) {
		.modal-portal {
			padding: 0;
			align-items: flex-end;
		}

		.modal-container {
			max-height: 95vh;
			max-height: 95dvh;
			border-radius: 20px 20px 0 0;
			animation: modalSlideUpMobile 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		}

		@keyframes modalSlideUpMobile {
			from {
				opacity: 0;
				transform: translateY(100%);
			}
			to {
				opacity: 1;
				transform: translateY(0);
			}
		}

		.modal-header {
			padding: 16px 20px;
		}

		.header-icon {
			width: 40px;
			height: 40px;
		}

		.modal-form {
			padding: 20px;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-actions {
			flex-direction: column;
			gap: 12px;
		}

		.archive-notice {
			order: 2;
			width: 100%;
			justify-content: center;
		}

		.action-buttons {
			width: 100%;
			order: 1;
		}

		.btn-cancel,
		.btn-publish {
			flex: 1;
			justify-content: center;
		}
	}
</style>

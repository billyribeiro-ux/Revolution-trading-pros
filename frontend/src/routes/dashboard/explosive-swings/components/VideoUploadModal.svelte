<!--
	═══════════════════════════════════════════════════════════════════════════════════
	VideoUploadModal Component
	═══════════════════════════════════════════════════════════════════════════════════
	
	@description Professional video upload modal for weekly watchlist videos with
	             Bunny.net integration, drag-drop upload, and thumbnail selection.
	@version 5.0.0 - ICT 7+ Grade: Fixed z-index stacking, upload timeout, responsive layout
	@requires Svelte 5.0+ (January 2026 syntax)
	@standards Apple Principal Engineer ICT 7+ Standards
	
	@changelog 5.0.0
	  - Fixed z-index to use isolation: isolate for guaranteed stacking above navbars
	  - Increased upload timeout from 60s to 5 minutes for large videos
	  - Auto-upload starts immediately on valid file drop
	  - Enhanced mobile responsive layout for all screen sizes
	
	@example
	<VideoUploadModal
	  isOpen={showModal}
	  roomSlug="explosive-swings"
	  onClose={() => showModal = false}
	  onSuccess={() => refreshVideos()}
	/>
-->
<script lang="ts">
	import { weeklyVideoApi } from '$lib/api/room-content';
	import DatePicker from '$lib/components/ui/DatePicker.svelte';
	import FileDropZone from '$lib/components/ui/FileDropZone.svelte';
	import UploadProgress from '$lib/components/ui/UploadProgress.svelte';
	import ThumbnailSelector from '$lib/components/ui/ThumbnailSelector.svelte';

	// Type definitions
	interface BunnyProcessingResult {
		status: 'ready' | 'failed' | number;
		thumbnail_url?: string;
		duration?: number;
	}

	interface BunnyCreateResponse {
		video_guid: string;
		embed_url?: string;
		video_url?: string;
	}

	// Constants
	const MAX_FILE_SIZE_GB = 5;
	const MAX_FILE_SIZE_BYTES = MAX_FILE_SIZE_GB * 1024 * 1024 * 1024;
	const ALLOWED_VIDEO_TYPES = [
		'video/mp4',
		'video/webm',
		'video/quicktime',
		'video/x-msvideo',
		'video/x-matroska'
	] as const;
	const PROCESSING_POLL_INTERVAL_MS = 3000;
	const PROCESSING_MAX_ATTEMPTS = 100; // 5 minutes total (100 * 3s = 300s)

	interface Props {
		isOpen: boolean;
		roomSlug: string;
		onClose: () => void;
		onSuccess?: () => void;
	}

	const { isOpen, roomSlug, onClose, onSuccess }: Props = $props();

	// Upload mode
	let uploadMode = $state<'file' | 'url'>('file');

	// Core states
	let isSaving = $state(false);
	let errorMessage = $state('');

	// File upload states
	let videoFile = $state<File | null>(null);
	let uploadProgress = $state(0);
	let uploadStatus = $state<'idle' | 'preparing' | 'uploading' | 'processing' | 'complete'>('idle');

	// Thumbnail selection
	let generatedThumbnails = $state<string[]>([]);
	let selectedThumbnailIndex = $state(0);

	// Abort control state
	let activeXhr = $state<XMLHttpRequest | null>(null);
	let processingAborted = $state(false);

	// A11y status announcements
	let statusAnnouncement = $state('');

	let form = $state({
		week_of: getNextMonday(),
		week_title: '',
		video_title: '',
		video_url: '',
		video_guid: '',
		video_platform: 'bunny' as 'bunny',
		thumbnail_url: '',
		duration: '',
		description: ''
	});

	// Lock body scroll when modal is open
	$effect(() => {
		if (!isOpen) return;

		const previousOverflow = document.body.style.overflow;
		document.body.style.overflow = 'hidden';

		return () => {
			document.body.style.overflow = previousOverflow;
		};
	});

	// A11y: Announce upload status changes
	$effect(() => {
		if (uploadStatus === 'uploading') {
			statusAnnouncement = `Uploading: ${uploadProgress}%`;
		} else if (uploadStatus === 'processing') {
			statusAnnouncement = 'Processing video on server...';
		} else if (uploadStatus === 'complete') {
			statusAnnouncement = 'Upload complete!';
		} else {
			statusAnnouncement = '';
		}
	});

	function getNextMonday(): string {
		const today = new Date();
		const dayOfWeek = today.getDay();
		const daysUntilMonday = dayOfWeek === 0 ? 1 : (8 - dayOfWeek) % 7 || 7;
		const nextMonday = new Date(today);
		nextMonday.setDate(today.getDate() + daysUntilMonday);
		return nextMonday.toISOString().split('T')[0];
	}

	function formatDuration(seconds: number): string {
		const mins = Math.floor(seconds / 60);
		const secs = Math.floor(seconds % 60);
		return `${mins}:${secs.toString().padStart(2, '0')}`;
	}

	// Bunny.net only - strict URL validation
	function isValidBunnyUrl(url: string): boolean {
		if (!url || typeof url !== 'string') return false;
		try {
			const parsed = new URL(url.trim());
			return parsed.protocol === 'https:' && parsed.hostname === 'iframe.mediadelivery.net';
		} catch {
			return false;
		}
	}

	const isFormValid = $derived(
		isValidBunnyUrl(form.video_url) && form.video_title.trim().length >= 3 && form.week_of !== ''
	);

	const canShowPreview = $derived(isValidBunnyUrl(form.video_url));

	function resetForm() {
		// Abort any active upload immediately
		if (activeXhr) {
			activeXhr.abort();
			activeXhr = null;
		}
		processingAborted = true;

		// Reset form state
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

		// Reset abort flag for next upload cycle
		processingAborted = false;
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
		return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
	}

	function handleFileSelect(file: File) {
		errorMessage = '';
		videoFile = file;
		if (!form.video_title) {
			form.video_title = file.name
				.replace(/\.[^/.]+$/, '')
				.replace(/[-_]/g, ' ')
				.trim();
		}
		startBunnyUpload();
	}

	function handleFileError(message: string) {
		errorMessage = message;
	}

	// Bunny.net upload functions
	async function startBunnyUpload() {
		if (!videoFile) return;

		errorMessage = '';
		uploadStatus = 'preparing';
		processingAborted = false;

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
				throw new Error(errorData.error || `Server error: ${createResponse.status}`);
			}

			const createData: BunnyCreateResponse = await createResponse.json();

			if (!createData.video_guid) {
				throw new Error('Server did not return video GUID');
			}

			form.video_guid = createData.video_guid;
			form.video_url = createData.embed_url || createData.video_url || '';

			// Check if aborted during API call
			if (processingAborted) {
				throw new Error('Upload cancelled');
			}

			// Step 2: Upload file to Bunny.net via proxy
			uploadStatus = 'uploading';
			await uploadFileToBunny(createData.video_guid, videoFile);

			// Check if aborted during upload
			if (processingAborted) {
				throw new Error('Upload cancelled');
			}

			// Step 3: Wait for processing and get thumbnails
			uploadStatus = 'processing';
			const finalData = await waitForBunnyProcessing(createData.video_guid);

			// Set thumbnail from processed video
			if (finalData.thumbnail_url) {
				form.thumbnail_url = finalData.thumbnail_url;

				// Generate thumbnail variants if URL follows expected pattern
				if (finalData.thumbnail_url.includes('thumbnail.jpg')) {
					generatedThumbnails = [
						finalData.thumbnail_url,
						finalData.thumbnail_url.replace('thumbnail.jpg', 'thumbnail_1.jpg'),
						finalData.thumbnail_url.replace('thumbnail.jpg', 'thumbnail_2.jpg'),
						finalData.thumbnail_url.replace('thumbnail.jpg', 'thumbnail_3.jpg')
					];
				} else {
					generatedThumbnails = [finalData.thumbnail_url];
				}
			}

			// Set duration from processed video
			if (finalData.duration && finalData.duration > 0) {
				form.duration = formatDuration(finalData.duration);
			}

			uploadStatus = 'complete';
		} catch (err) {
			if (err instanceof Error && err.message.includes('cancelled')) {
				// Silent abort — user closed modal
				uploadStatus = 'idle';
			} else {
				errorMessage = err instanceof Error ? err.message : 'Upload failed. Please try again.';
				uploadStatus = 'idle';
			}
		}
	}

	async function uploadFileToBunny(videoGuid: string, file: File): Promise<void> {
		return new Promise((resolve, reject) => {
			const xhr = new XMLHttpRequest();
			activeXhr = xhr;

			xhr.upload.addEventListener('progress', (event) => {
				if (event.lengthComputable) {
					uploadProgress = Math.round((event.loaded / event.total) * 100);
				}
			});

			xhr.addEventListener('load', () => {
				if (xhr.status >= 200 && xhr.status < 300) {
					resolve();
				} else {
					reject(new Error(`Upload failed: ${xhr.status} ${xhr.statusText}`));
				}
			});

			xhr.addEventListener('error', () => {
				reject(new Error('Network error during upload. Check your connection.'));
			});

			xhr.addEventListener('abort', () => {
				reject(new Error('Upload cancelled'));
			});

			xhr.addEventListener('loadend', () => {
				activeXhr = null;
			});

			const proxyUrl = `/api/weekly-video/${roomSlug}/upload?video_guid=${encodeURIComponent(videoGuid)}`;
			xhr.open('PUT', proxyUrl, true);
			xhr.setRequestHeader('Content-Type', file.type || 'application/octet-stream');
			xhr.withCredentials = true;
			xhr.send(file);
		});
	}

	async function waitForBunnyProcessing(videoGuid: string): Promise<BunnyProcessingResult> {
		for (let attempt = 0; attempt < PROCESSING_MAX_ATTEMPTS; attempt++) {
			// Check abort flag before each poll
			if (processingAborted) {
				throw new Error('Processing cancelled');
			}

			await new Promise((resolve) => setTimeout(resolve, PROCESSING_POLL_INTERVAL_MS));

			// Check again after wait
			if (processingAborted) {
				throw new Error('Processing cancelled');
			}

			try {
				const response = await fetch(
					`/api/weekly-video/${roomSlug}/upload/status/${encodeURIComponent(videoGuid)}`,
					{ credentials: 'include' }
				);

				if (!response.ok) {
					// Non-2xx status — continue polling, server might be busy
					continue;
				}

				const data: BunnyProcessingResult = await response.json();

				// Bunny.net status codes: 4 = ready, 5 = failed
				if (data.status === 'ready' || data.status === 4) {
					return data;
				}

				if (data.status === 'failed' || data.status === 5) {
					throw new Error('Video processing failed on Bunny.net. Try uploading again.');
				}

				// Any other status — continue polling
			} catch (err) {
				// Rethrow cancellation and explicit failures
				if (err instanceof Error) {
					if (err.message.includes('cancelled') || err.message.includes('failed')) {
						throw err;
					}
				}
				// Network errors — continue polling
			}
		}

		// Timeout after all attempts
		const totalSeconds = (PROCESSING_MAX_ATTEMPTS * PROCESSING_POLL_INTERVAL_MS) / 1000;
		throw new Error(`Video processing timed out after ${totalSeconds} seconds. Please try again.`);
	}

	function selectThumbnail(index: number) {
		selectedThumbnailIndex = index;
		form.thumbnail_url = generatedThumbnails[index] || '';
	}

	function clearFile() {
		// Abort active upload if any
		if (activeXhr) {
			activeXhr.abort();
			activeXhr = null;
		}
		processingAborted = true;

		videoFile = null;
		uploadProgress = 0;
		uploadStatus = 'idle';
		generatedThumbnails = [];
		selectedThumbnailIndex = 0;
		form.video_url = '';
		form.video_guid = '';
		form.thumbnail_url = '';
		form.duration = '';

		// Reset for next upload
		processingAborted = false;
	}

	function getUploadStatusText(): string {
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

	async function handleSubmit() {
		if (!isFormValid) return;
		isSaving = true;
		errorMessage = '';

		try {
			await weeklyVideoApi.create({
				room_slug: roomSlug,
				week_of: form.week_of,
				week_title: form.week_title || `Week of ${form.week_of}`,
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
		// Abort any in-progress operations
		if (activeXhr) {
			activeXhr.abort();
			activeXhr = null;
		}
		processingAborted = true;

		resetForm();
		onClose();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') handleClose();
	}

	function handleThumbnailSelect(index: number) {
		selectedThumbnailIndex = index;
		form.thumbnail_url = generatedThumbnails[index] || '';
	}
</script>

{#if isOpen}
	<!-- Screen reader status announcements -->
	<div class="sr-only" role="status" aria-live="polite" aria-atomic="true">
		{statusAnnouncement}
	</div>

	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
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
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							width="22"
							height="22"
						>
							<path
								d="M23 7l-7 5 7 5V7zM14 5H3a2 2 0 00-2 2v10a2 2 0 002 2h11a2 2 0 002-2V7a2 2 0 00-2-2z"
							/>
						</svg>
					</div>
					<div class="header-text">
						<h3 id="modal-title">Upload Weekly Video</h3>
						<p class="header-subtitle">Publish a new weekly breakdown</p>
					</div>
				</div>
				<button class="modal-close" onclick={handleClose} aria-label="Close modal">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="20"
						height="20"
					>
						<path d="M18 6L6 18M6 6l12 12" />
					</svg>
				</button>
			</div>

			{#if errorMessage}
				<div class="error-banner" role="alert" aria-live="assertive">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="18"
						height="18"
						aria-hidden="true"
					>
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
						onclick={() => (uploadMode = 'file')}
					>
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							width="18"
							height="18"
						>
							<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
						</svg>
						Upload File
					</button>
					<button
						type="button"
						class="mode-tab"
						class:active={uploadMode === 'url'}
						onclick={() => (uploadMode = 'url')}
					>
						<svg
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							width="18"
							height="18"
						>
							<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
							<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
						</svg>
						Paste URL
					</button>
				</div>

				<form
					class="modal-form"
					onsubmit={(e) => {
						e.preventDefault();
						handleSubmit();
					}}
				>
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

							<!-- Thumbnail Selection - Extracted Component -->
							<ThumbnailSelector
								thumbnails={generatedThumbnails}
								selected={selectedThumbnailIndex}
								onselect={handleThumbnailSelect}
							/>
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
										<button
											type="button"
											class="btn-remove"
											onclick={clearFile}
											aria-label="Remove file"
										>
											<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
												<line x1="18" y1="6" x2="6" y2="18"></line>
												<line x1="6" y1="6" x2="18" y2="18"></line>
											</svg>
										</button>
									{/if}
								</div>

								{#if uploadStatus !== 'idle'}
									<UploadProgress
										progress={uploadProgress}
										status={uploadStatus}
										statusText={getUploadStatusText()}
									/>
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
							<!-- File Drop Zone - Extracted Component -->
							<FileDropZone
								accept={[...ALLOWED_VIDEO_TYPES]}
								maxSizeBytes={MAX_FILE_SIZE_BYTES}
								onfile={handleFileSelect}
								onerror={handleFileError}
							/>
						{/if}
					{:else}
						<!-- URL MODE -->
						<div class="url-input-section">
							<label for="video_url" class="input-label">Bunny.net Embed URL</label>
							<input
								id="video_url"
								name="video_url"
								type="url"
								bind:value={form.video_url}
								placeholder="https://iframe.mediadelivery.net/embed/..."
								class="form-input url-input"
								required
							/>
							<p class="input-hint">Paste the embed URL from your Bunny.net video library</p>
						</div>

						{#if canShowPreview}
							<div class="video-preview">
								<iframe
									src={form.video_url}
									title="Video Preview"
									frameborder="0"
									allow="autoplay; fullscreen"
								></iframe>
							</div>
						{/if}

						<!-- Manual Thumbnail/Duration for URL mode -->
						<div class="form-row">
							<div class="form-group">
								<label for="thumbnail_url_manual">Thumbnail URL</label>
								<input
									id="thumbnail_url_manual"
									name="thumbnail_url_manual"
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
									name="duration_manual"
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
							<!-- Date Picker - Extracted Component -->
							<DatePicker
								value={form.week_of}
								onchange={(date) => (form.week_of = date)}
								label="Week Of"
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
								placeholder="Auto-generated from date"
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
							<svg
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2"
								width="16"
								height="16"
							>
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
									<svg
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										width="16"
										height="16"
									>
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
	   MODAL PORTAL - ICT 7+ Grade: GUARANTEED stacking above ALL navbars
	   Uses isolation: isolate to create new stacking context that ignores
	   any z-index from AdminToolbar (10100), NavBar (10001), or any other element.
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-portal {
		position: fixed;
		inset: 0;
		z-index: 2147483647; /* Max 32-bit signed int - absolutely nothing can be above this */
		isolation: isolate; /* Creates new stacking context */
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 16px;
	}

	.modal-backdrop {
		position: fixed; /* Changed from absolute to fixed for full coverage */
		inset: 0;
		z-index: -1; /* Behind modal content but covers everything else */
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(16px);
		-webkit-backdrop-filter: blur(16px);
		animation: backdropFade 0.2s ease-out;
	}

	@keyframes backdropFade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL CONTAINER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-container {
		position: relative;
		background: var(--color-bg-card);
		border-radius: 16px;
		width: 100%;
		max-width: 480px;
		max-height: calc(100vh - 32px);
		max-height: calc(100dvh - 32px);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow:
			0 0 0 1px rgba(0, 0, 0, 0.08),
			0 20px 40px -8px rgba(0, 0, 0, 0.4);
		animation: modalSlideUp 0.25s cubic-bezier(0.16, 1, 0.3, 1);
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
	   MODE TABS
	   ═══════════════════════════════════════════════════════════════════════════ */
	.mode-tabs {
		display: flex;
		gap: 8px;
		padding: 16px 24px;
		background: var(--color-bg-subtle);
		border-bottom: 1px solid var(--color-border-default);
	}

	.mode-tab {
		display: flex;
		align-items: center;
		gap: 8px;
		flex: 1;
		padding: 12px 16px;
		background: var(--color-bg-card);
		border: 2px solid var(--color-border-default);
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s ease;
		justify-content: center;
	}

	.mode-tab:hover {
		border-color: var(--color-border-strong);
		color: var(--color-text-tertiary);
	}

	.mode-tab.active {
		background: var(--color-brand-primary);
		border-color: var(--color-brand-primary);
		color: var(--color-bg-card);
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
		color: var(--color-profit);
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
		color: var(--color-text-primary);
		word-break: break-word;
	}

	.file-size {
		font-size: 0.8125rem;
		color: var(--color-text-muted);
	}

	.btn-remove {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: rgba(239, 68, 68, 0.2);
		border: none;
		color: var(--color-loss);
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

	/* Progress bar styles moved to UploadProgress component */

	.btn-upload-bunny {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		margin-top: 1rem;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, var(--color-brand-primary), #1a5a7e);
		border: none;
		border-radius: 8px;
		color: var(--color-bg-card);
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
		color: var(--color-profit);
	}

	.success-text {
		color: var(--color-profit-dark);
		font-weight: 600;
		margin: 0 0 0.5rem;
	}

	.video-url-display {
		color: var(--color-text-muted);
		font-size: 0.8125rem;
		word-break: break-all;
		margin: 0 0 1rem;
	}

	.btn-link {
		background: transparent;
		border: none;
		color: var(--color-brand-primary);
		font-size: 0.875rem;
		cursor: pointer;
		text-decoration: underline;
	}

	.btn-link:hover {
		color: var(--color-brand-primary-hover);
	}

	/* NOTE: Thumbnail, Drop Zone, Progress, and DatePicker styles moved to extracted components */

	.form-divider {
		height: 1px;
		background: var(--color-border-default);
		margin: 20px 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 24px;
		background: linear-gradient(135deg, var(--color-brand-primary) 0%, #1a4d6e 100%);
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
		color: var(--color-bg-card);
	}

	.header-text h3 {
		margin: 0;
		font-size: 18px;
		font-weight: 700;
		color: var(--color-bg-card);
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
		color: var(--color-bg-card);
		transform: scale(1.05);
	}

	.error-banner {
		display: flex;
		align-items: center;
		gap: 10px;
		margin: 16px 24px 0;
		padding: 14px 16px;
		background: linear-gradient(135deg, var(--color-loss-bg) 0%, var(--color-loss-bg) 100%);
		border: 1px solid var(--color-loss-bg);
		border-radius: 12px;
		color: var(--color-loss);
		font-size: 14px;
		font-weight: 500;
	}

	.modal-form {
		flex: 1;
		overflow-y: auto;
		padding: 20px;
	}

	/* Drop zone styles moved to FileDropZone component */

	.video-preview {
		width: 100%;
		aspect-ratio: 16/9;
		border-radius: 12px;
		overflow: hidden;
		margin-bottom: 16px;
		border: 2px solid var(--color-brand-primary);
	}

	.video-preview iframe {
		width: 100%;
		height: 100%;
	}

	/* URL Input Section */
	.url-input-section {
		margin-bottom: 20px;
	}

	.input-label {
		display: block;
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text-secondary);
		margin-bottom: 8px;
	}

	.url-input {
		font-family: 'SF Mono', Monaco, monospace;
		font-size: 13px;
	}

	.input-hint {
		margin: 6px 0 0;
		font-size: 12px;
		color: var(--color-text-muted);
	}

	/* Date picker styles moved to DatePicker component */

	/* Form Layout */
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
		color: var(--color-text-tertiary);
	}

	.form-input,
	.form-textarea {
		padding: 12px 14px;
		border: 2px solid var(--color-border-default);
		border-radius: 10px;
		font-size: 14px;
		font-weight: 500;
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
		transition: all 0.15s ease;
	}

	.form-input:hover,
	.form-textarea:hover {
		border-color: var(--color-border-strong);
	}

	.form-input:focus,
	.form-textarea:focus {
		outline: none;
		border-color: var(--color-brand-primary);
		background: var(--color-bg-card);
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
		border-top: 2px solid var(--color-bg-subtle);
	}

	.archive-notice {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		color: var(--color-text-muted);
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
		background: var(--color-bg-subtle);
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-cancel:hover {
		background: var(--color-border-default);
	}

	.btn-publish {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 28px;
		background: linear-gradient(135deg, var(--color-brand-primary) 0%, #1a4d6e 100%);
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
		background: linear-gradient(
			135deg,
			var(--color-brand-primary-hover) 0%,
			var(--color-brand-primary) 100%
		);
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
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE - ICT 7+ Grade: Perfect on all devices
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Tablets and small laptops */
	@media (max-width: 768px) {
		.modal-container {
			max-width: 95%;
		}

		.mode-tabs {
			padding: 12px 16px;
		}

		.mode-tab {
			padding: 10px 12px;
			font-size: 13px;
		}

		.modal-form {
			padding: 16px;
		}
	}

	/* Mobile devices */
	@media (max-width: 640px) {
		.modal-portal {
			padding: 0;
			align-items: flex-end;
		}

		.modal-container {
			max-width: 100%;
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

		.header-text h3 {
			font-size: 18px;
		}

		.header-subtitle {
			font-size: 13px;
		}

		.mode-tabs {
			padding: 12px 16px;
			gap: 6px;
		}

		.mode-tab {
			padding: 10px 8px;
			font-size: 12px;
			gap: 6px;
		}

		.mode-tab svg {
			width: 16px;
			height: 16px;
		}

		.modal-form {
			padding: 16px;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-group label {
			font-size: 13px;
		}

		.form-group input,
		.form-group textarea {
			font-size: 16px; /* Prevents iOS zoom on focus */
			padding: 12px;
		}

		.file-selected {
			padding: 12px;
		}

		.file-info {
			gap: 10px;
		}

		.file-icon {
			width: 32px;
			height: 32px;
		}

		.file-name {
			font-size: 14px;
		}

		.file-size {
			font-size: 12px;
		}

		.form-actions {
			flex-direction: column;
			gap: 12px;
			padding-top: 16px;
			margin-top: 16px;
		}

		.archive-notice {
			order: 2;
			width: 100%;
			justify-content: center;
			font-size: 12px;
		}

		.action-buttons {
			width: 100%;
			order: 1;
		}

		.btn-cancel,
		.btn-publish {
			flex: 1;
			justify-content: center;
			padding: 14px 20px;
			font-size: 15px;
		}

		.btn-upload-bunny {
			padding: 14px 20px;
			font-size: 15px;
		}
	}

	/* Small mobile devices */
	@media (max-width: 400px) {
		.modal-header {
			padding: 14px 16px;
		}

		.header-icon {
			width: 36px;
			height: 36px;
		}

		.header-text h3 {
			font-size: 16px;
		}

		.header-subtitle {
			font-size: 12px;
		}

		.modal-close {
			width: 36px;
			height: 36px;
		}

		.mode-tabs {
			padding: 10px 12px;
		}

		.mode-tab {
			padding: 8px 6px;
			font-size: 11px;
		}

		.modal-form {
			padding: 12px;
		}

		.form-group label {
			font-size: 12px;
		}

		.btn-cancel,
		.btn-publish {
			padding: 12px 16px;
			font-size: 14px;
		}
	}

	/* Screen reader only */
	.sr-only {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}
</style>

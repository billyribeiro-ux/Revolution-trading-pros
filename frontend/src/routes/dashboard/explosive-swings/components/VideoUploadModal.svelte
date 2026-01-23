<!--
	Video Upload Modal - ICT 7 Frontend Admin Component
	═══════════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT 7+ Grade - January 2026
	
	Drag & drop video upload modal for weekly breakdown videos.
	Supports Bunny.net, Vimeo, and YouTube URLs.
	
	@version 1.0.0
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

	let isSaving = $state(false);
	let errorMessage = $state('');
	let isDragOver = $state(false);

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

	const isFormValid = $derived(
		form.video_url.trim() !== '' && 
		form.video_title.trim() !== '' &&
		form.week_of !== ''
	);

	const embedUrl = $derived(
		form.video_url ? generateEmbedUrl(form.video_url, form.video_platform) : ''
	);

	function resetForm() {
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
		errorMessage = '';
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
		isDragOver = false;
		
		// Check for URL in dropped data
		const url = e.dataTransfer?.getData('text/plain');
		if (url && (url.startsWith('http://') || url.startsWith('https://'))) {
			form.video_url = url;
			form.video_platform = detectPlatform(url);
		}
	}
</script>

{#if isOpen}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div 
		class="modal-overlay" 
		role="dialog" 
		aria-modal="true" 
		aria-labelledby="modal-title"
		tabindex="-1"
		onclick={(e) => e.target === e.currentTarget && handleClose()}
		onkeydown={handleKeydown}
	>
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
				<div class="error-banner">{errorMessage}</div>
			{/if}

			<form class="modal-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
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
					{#if form.video_url && embedUrl}
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
							<path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M17 8l-5-5-5 5M12 3v12" />
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

				<div class="form-row">
					<div class="form-group">
						<label for="thumbnail_url">Thumbnail URL</label>
						<input
							id="thumbnail_url"
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
							type="text"
							bind:value={form.duration}
							placeholder="24:35"
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
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL OVERLAY - Fixed viewport positioning
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(15, 23, 42, 0.75);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 9999;
		padding: 16px;
		backdrop-filter: blur(8px);
		animation: overlayFadeIn 0.2s ease-out;
	}

	@keyframes overlayFadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   MODAL CONTAINER - Proper viewport-safe sizing
	   ═══════════════════════════════════════════════════════════════════════════ */
	.modal-container {
		background: #ffffff;
		border-radius: 20px;
		width: 100%;
		max-width: 600px;
		max-height: calc(100vh - 32px);
		max-height: calc(100dvh - 32px);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		box-shadow: 
			0 0 0 1px rgba(0, 0, 0, 0.05),
			0 25px 50px -12px rgba(0, 0, 0, 0.4),
			0 0 100px -20px rgba(20, 62, 89, 0.3);
		animation: modalSlideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
	}

	@keyframes modalSlideUp {
		from {
			opacity: 0;
			transform: translateY(20px) scale(0.98);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
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
		.modal-overlay {
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

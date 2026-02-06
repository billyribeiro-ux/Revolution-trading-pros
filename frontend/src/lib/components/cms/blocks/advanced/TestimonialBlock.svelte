<!--
/**
 * Testimonial Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Quote with star rating, author name, title, company, and photo
 * Features: 1-5 star rating, photo upload with validation, centered layout
 */
-->

<script lang="ts">
	import { IconStar, IconQuote, IconUser, IconUpload, IconX } from '$lib/icons';
	import { sanitizeURL, validateFile } from '$lib/utils/sanitization';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// ==========================================================================
	// Local State
	// ==========================================================================

	let isUploading = $state(false);
	let uploadError = $state<string | null>(null);
	let fileInputRef = $state<HTMLInputElement | null>(null);
	let urlInputValue = $state('');
	let hoverRating = $state(0);

	// ==========================================================================
	// Derived Values
	// ==========================================================================

	const quote = $derived(
		props.block.content.testimonialQuote || 'This is an amazing product that changed my life!'
	);
	const authorName = $derived(props.block.content.testimonialAuthor || 'John Doe');
	const authorTitle = $derived(props.block.content.testimonialTitle || 'CEO');
	const authorCompany = $derived(props.block.content.testimonialCompany || 'Acme Corp');
	const authorPhoto = $derived(props.block.content.testimonialPhoto || '');
	const rating = $derived(props.block.content.testimonialRating || 5);
	const showPhoto = $derived(props.block.settings?.showPhoto !== false);
	const sanitizedPhotoURL = $derived(authorPhoto ? sanitizeURL(authorPhoto) : '');

	// ==========================================================================
	// Constants
	// ==========================================================================

	const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5MB
	const ALLOWED_TYPES = ['image/jpeg', 'image/png', 'image/webp'];
	const ALLOWED_EXTENSIONS = ['jpg', 'jpeg', 'png', 'webp'];
	const MAX_RATING = 5;

	// ==========================================================================
	// Content Updates
	// ==========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({
			content: { ...props.block.content, ...updates }
		});
	}

	function updateSettings(updates: Partial<Block['settings']>): void {
		props.onUpdate({
			settings: { ...props.block.settings, ...updates }
		});
	}

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text.slice(0, 2000));
	}

	function handleQuoteInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ testimonialQuote: target.textContent || '' });
	}

	function handleAuthorNameInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ testimonialAuthor: target.textContent || '' });
	}

	function handleAuthorTitleInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ testimonialTitle: target.textContent || '' });
	}

	function handleAuthorCompanyInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ testimonialCompany: target.textContent || '' });
	}

	// ==========================================================================
	// Star Rating Handlers
	// ==========================================================================

	function handleStarClick(starValue: number): void {
		if (props.isEditing) {
			updateContent({ testimonialRating: starValue });
		}
	}

	function handleStarHover(starValue: number): void {
		if (props.isEditing) {
			hoverRating = starValue;
		}
	}

	function handleStarLeave(): void {
		hoverRating = 0;
	}

	function handleStarKeyDown(e: KeyboardEvent, starValue: number): void {
		if (props.isEditing && (e.key === 'Enter' || e.key === ' ')) {
			e.preventDefault();
			handleStarClick(starValue);
		}
	}

	// ==========================================================================
	// Photo Upload Handlers
	// ==========================================================================

	function handleFileSelect(e: Event): void {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) {
			processFile(file);
		}
		input.value = '';
	}

	function handleDrop(e: DragEvent): void {
		e.preventDefault();
		const file = e.dataTransfer?.files?.[0];
		if (file) {
			processFile(file);
		}
	}

	function handleDragOver(e: DragEvent): void {
		e.preventDefault();
	}

	async function processFile(file: File): Promise<void> {
		uploadError = null;

		const validation = validateFile(file, {
			maxSize: MAX_FILE_SIZE,
			allowedTypes: ALLOWED_TYPES,
			allowedExtensions: ALLOWED_EXTENSIONS
		});

		if (!validation.valid) {
			uploadError = validation.error || 'Invalid file';
			props.onError?.(new Error(uploadError));
			return;
		}

		isUploading = true;

		try {
			const objectUrl = URL.createObjectURL(file);
			await new Promise((resolve) => setTimeout(resolve, 300));

			updateContent({ testimonialPhoto: objectUrl });
			uploadError = null;
		} catch (err) {
			const error = err instanceof Error ? err.message : 'Failed to upload photo';
			uploadError = error;
			props.onError?.(new Error(error));
		} finally {
			isUploading = false;
		}
	}

	function handleURLSubmit(): void {
		const trimmedUrl = urlInputValue.trim();
		if (!trimmedUrl) return;

		const sanitized = sanitizeURL(trimmedUrl);
		if (sanitized) {
			updateContent({ testimonialPhoto: sanitized });
			urlInputValue = '';
			uploadError = null;
		} else {
			uploadError = 'Invalid URL. Please enter a valid image URL.';
		}
	}

	function handleURLKeyDown(e: KeyboardEvent): void {
		if (e.key === 'Enter') {
			e.preventDefault();
			handleURLSubmit();
		}
	}

	function removePhoto(): void {
		updateContent({ testimonialPhoto: '' });
		uploadError = null;
	}

	function triggerFileInput(): void {
		fileInputRef?.click();
	}

	function handleShowPhotoChange(e: Event): void {
		const target = e.target as HTMLInputElement;
		updateSettings({ showPhoto: target.checked });
	}
</script>

<figure class="testimonial-block" aria-label="Testimonial">
	<!-- Quote Icon -->
	<div class="testimonial-block__quote-icon" aria-hidden="true">
		<IconQuote size={32} />
	</div>

	<!-- Star Rating -->
	<div
		class="testimonial-block__rating"
		class:testimonial-block__rating--editable={props.isEditing}
		role="group"
		aria-label="Rating: {rating} out of {MAX_RATING} stars"
	>
		{#each Array(MAX_RATING) as _, index (index)}
			{@const starValue = index + 1}
			{@const isFilled = starValue <= (hoverRating || rating)}
			{#if props.isEditing}
				<button
					type="button"
					class="testimonial-block__star"
					class:testimonial-block__star--filled={isFilled}
					onclick={() => handleStarClick(starValue)}
					onmouseenter={() => handleStarHover(starValue)}
					onmouseleave={handleStarLeave}
					onkeydown={(e) => handleStarKeyDown(e, starValue)}
					aria-label="Rate {starValue} star{starValue === 1 ? '' : 's'}"
					aria-pressed={starValue === rating}
				>
					<IconStar size={20} />
				</button>
			{:else}
				<span
					class="testimonial-block__star"
					class:testimonial-block__star--filled={isFilled}
					aria-hidden="true"
				>
					<IconStar size={20} />
				</span>
			{/if}
		{/each}
	</div>

	<!-- Quote Text -->
	<blockquote class="testimonial-block__quote">
		{#if props.isEditing}
			<!-- svelte-ignore a11y_no_noninteractive_element_to_interactive_role -->
			<p
				contenteditable="true"
				class="testimonial-block__quote-text"
				oninput={handleQuoteInput}
				onpaste={handlePaste}
				role="textbox"
				aria-label="Testimonial quote"
				aria-multiline="true"
			>
				{quote}
			</p>
		{:else}
			<p class="testimonial-block__quote-text">{quote}</p>
		{/if}
	</blockquote>

	<!-- Author Section -->
	<figcaption class="testimonial-block__author">
		<!-- Author Photo -->
		{#if showPhoto}
			<div class="testimonial-block__photo">
				{#if sanitizedPhotoURL}
					<img src={sanitizedPhotoURL} alt={authorName} class="testimonial-block__photo-img" />
					{#if props.isEditing && props.isSelected}
						<button
							type="button"
							class="testimonial-block__photo-remove"
							onclick={removePhoto}
							aria-label="Remove photo"
						>
							<IconX size={14} />
						</button>
					{/if}
				{:else}
					<div class="testimonial-block__photo-placeholder">
						<IconUser size={24} aria-hidden="true" />
					</div>
				{/if}
			</div>
		{/if}

		<!-- Author Info -->
		<div class="testimonial-block__info">
			{#if props.isEditing}
				<span
					contenteditable="true"
					class="testimonial-block__name"
					oninput={handleAuthorNameInput}
					onpaste={handlePaste}
					role="textbox"
					aria-label="Author name">{authorName}</span
				>
				<span class="testimonial-block__meta">
					<span
						contenteditable="true"
						class="testimonial-block__title"
						oninput={handleAuthorTitleInput}
						onpaste={handlePaste}
						role="textbox"
						aria-label="Author title">{authorTitle}</span
					>
					<span class="testimonial-block__separator">,</span>
					<span
						contenteditable="true"
						class="testimonial-block__company"
						oninput={handleAuthorCompanyInput}
						onpaste={handlePaste}
						role="textbox"
						aria-label="Author company">{authorCompany}</span
					>
				</span>
			{:else}
				<span class="testimonial-block__name">{authorName}</span>
				<span class="testimonial-block__meta">
					<span class="testimonial-block__title">{authorTitle}</span>
					<span class="testimonial-block__separator">,</span>
					<span class="testimonial-block__company">{authorCompany}</span>
				</span>
			{/if}
		</div>
	</figcaption>
</figure>

<!-- Settings Panel (Edit Mode) - Outside figure to fix figcaption position -->
{#if props.isEditing && props.isSelected}
	<div class="testimonial-block__settings">
		<!-- Show Photo Toggle -->
		<label class="testimonial-block__setting">
			<input type="checkbox" checked={showPhoto} onchange={handleShowPhotoChange} />
			<span>Show author photo</span>
		</label>

		<!-- Photo Upload Section -->
		{#if showPhoto}
			<div class="testimonial-block__photo-upload">
				<span class="testimonial-block__setting-label">Author Photo</span>

				{#if isUploading}
					<div class="testimonial-block__upload-loading">
						<div class="testimonial-block__spinner"></div>
						<span>Uploading...</span>
					</div>
				{:else if uploadError}
					<div class="testimonial-block__upload-error">
						<span>{uploadError}</span>
						<button type="button" onclick={() => (uploadError = null)}>Dismiss</button>
					</div>
				{:else}
					<div
						class="testimonial-block__upload-area"
						ondrop={handleDrop}
						ondragover={handleDragOver}
						role="region"
						aria-label="Photo upload area"
					>
						<button type="button" class="testimonial-block__upload-btn" onclick={triggerFileInput}>
							<IconUpload size={16} />
							Upload Photo
						</button>

						<span class="testimonial-block__upload-divider">or</span>

						<div class="testimonial-block__url-input-wrapper">
							<input
								type="url"
								class="testimonial-block__url-input"
								placeholder="Paste image URL..."
								bind:value={urlInputValue}
								onkeydown={handleURLKeyDown}
								aria-label="Photo URL"
							/>
							<button
								type="button"
								class="testimonial-block__url-submit"
								onclick={handleURLSubmit}
								disabled={!urlInputValue.trim()}
							>
								Add
							</button>
						</div>

						<span class="testimonial-block__upload-hint"> JPG, PNG, WebP (max 5MB) </span>
					</div>
				{/if}

				<!-- Hidden file input -->
				<input
					bind:this={fileInputRef}
					type="file"
					accept=".jpg,.jpeg,.png,.webp,image/jpeg,image/png,image/webp"
					onchange={handleFileSelect}
					class="testimonial-block__file-input"
					aria-hidden="true"
					tabindex="-1"
				/>
			</div>
		{/if}
	</div>
{/if}

<style>
	/* ==========================================================================
	 * Block Container
	 * ========================================================================== */

	.testimonial-block {
		margin: 0;
		padding: 2.5rem 2rem;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		border-radius: 16px;
		text-align: center;
		position: relative;
	}

	/* ==========================================================================
	 * Quote Icon
	 * ========================================================================== */

	.testimonial-block__quote-icon {
		display: inline-flex;
		width: 56px;
		height: 56px;
		align-items: center;
		justify-content: center;
		background: #3b82f6;
		border-radius: 50%;
		color: white;
		margin-bottom: 1.5rem;
	}

	/* ==========================================================================
	 * Star Rating
	 * ========================================================================== */

	.testimonial-block__rating {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.25rem;
		margin-bottom: 1.25rem;
	}

	.testimonial-block__star {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		color: #d1d5db;
		transition:
			color 0.15s ease,
			transform 0.15s ease;
	}

	.testimonial-block__star--filled {
		color: #fbbf24;
	}

	.testimonial-block__rating--editable .testimonial-block__star {
		padding: 0.25rem;
		background: none;
		border: none;
		cursor: pointer;
		border-radius: 4px;
	}

	.testimonial-block__rating--editable .testimonial-block__star:hover {
		transform: scale(1.2);
	}

	.testimonial-block__rating--editable .testimonial-block__star:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* ==========================================================================
	 * Quote Text
	 * ========================================================================== */

	.testimonial-block__quote {
		margin: 0 0 1.5rem;
	}

	.testimonial-block__quote-text {
		margin: 0;
		font-size: 1.25rem;
		font-style: italic;
		line-height: 1.7;
		color: #1e293b;
		outline: none;
		max-width: 640px;
		margin-left: auto;
		margin-right: auto;
	}

	.testimonial-block__quote-text:focus {
		background: rgba(59, 130, 246, 0.05);
		border-radius: 8px;
		padding: 0.5rem;
		margin: -0.5rem;
	}

	/* ==========================================================================
	 * Author Section
	 * ========================================================================== */

	.testimonial-block__author {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
	}

	.testimonial-block__photo {
		position: relative;
		width: 56px;
		height: 56px;
		border-radius: 50%;
		overflow: hidden;
		background: #e2e8f0;
		flex-shrink: 0;
	}

	.testimonial-block__photo-img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.testimonial-block__photo-placeholder {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #94a3b8;
	}

	.testimonial-block__photo-remove {
		position: absolute;
		top: -4px;
		right: -4px;
		width: 22px;
		height: 22px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #ef4444;
		border: 2px solid white;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.testimonial-block__photo-remove:hover {
		background: #dc2626;
	}

	.testimonial-block__info {
		text-align: left;
	}

	.testimonial-block__name {
		display: block;
		font-weight: 700;
		font-size: 1rem;
		color: #0f172a;
		outline: none;
	}

	.testimonial-block__name:focus {
		background: rgba(59, 130, 246, 0.1);
		border-radius: 4px;
		padding: 0.125rem 0.25rem;
		margin: -0.125rem -0.25rem;
	}

	.testimonial-block__meta {
		display: block;
		font-size: 0.875rem;
		color: #64748b;
	}

	.testimonial-block__title,
	.testimonial-block__company {
		outline: none;
	}

	.testimonial-block__title:focus,
	.testimonial-block__company:focus {
		background: rgba(59, 130, 246, 0.1);
		border-radius: 4px;
		padding: 0.125rem 0.25rem;
		margin: -0.125rem -0.25rem;
	}

	.testimonial-block__separator {
		margin: 0 0.25rem;
	}

	/* ==========================================================================
	 * Settings Panel
	 * ========================================================================== */

	.testimonial-block__settings {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid #e2e8f0;
		text-align: left;
	}

	.testimonial-block__setting {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #374151;
		cursor: pointer;
	}

	.testimonial-block__setting input[type='checkbox'] {
		width: 16px;
		height: 16px;
		accent-color: #3b82f6;
		cursor: pointer;
	}

	.testimonial-block__setting-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		margin-bottom: 0.5rem;
	}

	.testimonial-block__photo-upload {
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px dashed #e2e8f0;
	}

	.testimonial-block__upload-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: #f8fafc;
		border: 1px dashed #d1d5db;
		border-radius: 8px;
	}

	.testimonial-block__upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: #3b82f6;
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.testimonial-block__upload-btn:hover {
		background: #2563eb;
	}

	.testimonial-block__upload-divider {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.testimonial-block__url-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		width: 100%;
	}

	.testimonial-block__url-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
		outline: none;
		transition: border-color 0.15s ease;
	}

	.testimonial-block__url-input:focus {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.testimonial-block__url-submit {
		padding: 0.5rem 0.75rem;
		background: #1e293b;
		border: none;
		border-radius: 6px;
		color: white;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.testimonial-block__url-submit:hover:not(:disabled) {
		background: #334155;
	}

	.testimonial-block__url-submit:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.testimonial-block__upload-hint {
		font-size: 0.75rem;
		color: #94a3b8;
	}

	.testimonial-block__file-input {
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

	.testimonial-block__upload-loading {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 1rem;
		color: #64748b;
		font-size: 0.875rem;
	}

	.testimonial-block__spinner {
		width: 20px;
		height: 20px;
		border: 2px solid #e2e8f0;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: testimonial-spin 1s linear infinite;
	}

	@keyframes testimonial-spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.testimonial-block__upload-error {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.75rem 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #dc2626;
		font-size: 0.875rem;
	}

	.testimonial-block__upload-error button {
		padding: 0.375rem 0.75rem;
		background: white;
		border: 1px solid #fca5a5;
		border-radius: 4px;
		color: #dc2626;
		font-size: 0.75rem;
		cursor: pointer;
	}

	.testimonial-block__upload-error button:hover {
		background: #fef2f2;
	}

	/* ==========================================================================
	 * Mobile Responsive
	 * ========================================================================== */

	@media (max-width: 640px) {
		.testimonial-block {
			padding: 1.5rem 1rem;
		}

		.testimonial-block__quote-text {
			font-size: 1.125rem;
		}

		.testimonial-block__author {
			flex-direction: column;
			text-align: center;
		}

		.testimonial-block__info {
			text-align: center;
		}

		.testimonial-block__url-input-wrapper {
			flex-direction: column;
		}

		.testimonial-block__url-input {
			width: 100%;
		}

		.testimonial-block__url-submit {
			width: 100%;
		}
	}

	/* ==========================================================================
	 * Dark Mode
	 * ========================================================================== */

	:global(.dark) .testimonial-block {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
	}

	:global(.dark) .testimonial-block__quote-text {
		color: #f1f5f9;
	}

	:global(.dark) .testimonial-block__quote-text:focus {
		background: rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .testimonial-block__photo {
		background: #334155;
	}

	:global(.dark) .testimonial-block__photo-placeholder {
		color: #64748b;
	}

	:global(.dark) .testimonial-block__name {
		color: #f8fafc;
	}

	:global(.dark) .testimonial-block__name:focus {
		background: rgba(59, 130, 246, 0.15);
	}

	:global(.dark) .testimonial-block__meta {
		color: #94a3b8;
	}

	:global(.dark) .testimonial-block__title:focus,
	:global(.dark) .testimonial-block__company:focus {
		background: rgba(59, 130, 246, 0.15);
	}

	:global(.dark) .testimonial-block__star {
		color: #475569;
	}

	:global(.dark) .testimonial-block__star--filled {
		color: #fbbf24;
	}

	:global(.dark) .testimonial-block__settings {
		border-color: #334155;
	}

	:global(.dark) .testimonial-block__setting {
		color: #e2e8f0;
	}

	:global(.dark) .testimonial-block__setting-label {
		color: #94a3b8;
	}

	:global(.dark) .testimonial-block__photo-upload {
		border-color: #334155;
	}

	:global(.dark) .testimonial-block__upload-area {
		background: #0f172a;
		border-color: #475569;
	}

	:global(.dark) .testimonial-block__url-input {
		background: #1e293b;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .testimonial-block__url-input:focus {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
	}

	:global(.dark) .testimonial-block__url-input::placeholder {
		color: #64748b;
	}

	:global(.dark) .testimonial-block__url-submit {
		background: #f1f5f9;
		color: #0f172a;
	}

	:global(.dark) .testimonial-block__url-submit:hover:not(:disabled) {
		background: white;
	}

	:global(.dark) .testimonial-block__upload-hint,
	:global(.dark) .testimonial-block__upload-divider {
		color: #64748b;
	}

	:global(.dark) .testimonial-block__upload-loading {
		color: #94a3b8;
	}

	:global(.dark) .testimonial-block__spinner {
		border-color: #334155;
		border-top-color: #3b82f6;
	}

	:global(.dark) .testimonial-block__upload-error {
		background: rgba(239, 68, 68, 0.1);
		border-color: #7f1d1d;
		color: #f87171;
	}

	:global(.dark) .testimonial-block__upload-error button {
		background: #1e293b;
		border-color: #7f1d1d;
		color: #f87171;
	}

	:global(.dark) .testimonial-block__upload-error button:hover {
		background: rgba(239, 68, 68, 0.1);
	}
</style>

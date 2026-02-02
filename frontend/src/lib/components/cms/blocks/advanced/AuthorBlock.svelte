<!--
/**
 * Author Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Author bio box with photo, title, bio, and social links
 * Features: Photo upload with validation, social link management, responsive design
 */
-->

<script lang="ts">
	import { IconUser, IconLink, IconUpload, IconX, IconPlus
	} from '$lib/icons';
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

	const props: Props = $props();

	// ==========================================================================
	// Local State
	// ==========================================================================

	let isUploading = $state(false);
	let uploadError = $state<string | null>(null);
	let fileInputRef = $state<HTMLInputElement | null>(null);
	let urlInputValue = $state('');
	let showUrlInput = $state(false);

	// ==========================================================================
	// Constants
	// ==========================================================================

	const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5MB for profile photos
	const ALLOWED_TYPES = ['image/jpeg', 'image/png', 'image/gif', 'image/webp'];
	const PLATFORM_OPTIONS = [
		{ value: 'twitter', label: 'Twitter' },
		{ value: 'linkedin', label: 'LinkedIn' },
		{ value: 'facebook', label: 'Facebook' },
		{ value: 'instagram', label: 'Instagram' },
		{ value: 'youtube', label: 'YouTube' },
		{ value: 'github', label: 'GitHub' },
		{ value: 'website', label: 'Website' }
	];

	// ==========================================================================
	// Derived Values
	// ==========================================================================

	const name = $derived(props.block.content.authorName || 'Author Name');
	const title = $derived(props.block.content.authorTitle || '');
	const bio = $derived(props.block.content.authorBio || 'Author bio goes here. Share a brief description about the author.');
	const photo = $derived(props.block.content.authorPhoto || '');
	const socials = $derived<Array<{ platform: string; url: string }>>(props.block.content.authorSocials || []);
	const sanitizedPhotoURL = $derived(photo ? sanitizeURL(photo) : '');

	// ==========================================================================
	// Content Updates
	// ==========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text.slice(0, 2000));
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
			allowedTypes: ALLOWED_TYPES
		});

		if (!validation.valid) {
			uploadError = validation.error || 'Invalid file';
			props.onError?.(new Error(uploadError));
			return;
		}

		isUploading = true;

		try {
			// Create object URL for preview (in production, upload to CDN)
			const objectUrl = URL.createObjectURL(file);
			await new Promise((resolve) => setTimeout(resolve, 300));

			updateContent({ authorPhoto: objectUrl });
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
			updateContent({ authorPhoto: sanitized });
			urlInputValue = '';
			showUrlInput = false;
			uploadError = null;
		} else {
			uploadError = 'Invalid URL. Please enter a valid image URL.';
		}
	}

	function handleURLKeyDown(e: KeyboardEvent): void {
		if (e.key === 'Enter') {
			e.preventDefault();
			handleURLSubmit();
		} else if (e.key === 'Escape') {
			showUrlInput = false;
			urlInputValue = '';
		}
	}

	function triggerFileInput(): void {
		fileInputRef?.click();
	}

	function removePhoto(): void {
		updateContent({ authorPhoto: '' });
		uploadError = null;
	}

	// ==========================================================================
	// Social Links Handlers
	// ==========================================================================

	function addSocial(): void {
		const newSocials = [...socials, { platform: 'twitter', url: '' }];
		updateContent({ authorSocials: newSocials });
	}

	function updateSocial(index: number, field: 'platform' | 'url', value: string): void {
		const newSocials = socials.map((s, i) =>
			i === index ? { ...s, [field]: value } : s
		);
		updateContent({ authorSocials: newSocials });
	}

	function removeSocial(index: number): void {
		const newSocials = socials.filter((_, i) => i !== index);
		updateContent({ authorSocials: newSocials });
	}

	function getSanitizedSocialUrl(url: string): string {
		return sanitizeURL(url) || '#';
	}
</script>

<div
	class="author-block"
	class:author-block--selected={props.isSelected}
	class:author-block--editing={props.isEditing}
	role="article"
	aria-label="About the author"
>
	<!-- Author Photo Section -->
	<div class="author-block__photo-section">
		{#if sanitizedPhotoURL}
			<div class="author-block__photo">
				<img src={sanitizedPhotoURL} alt={name} loading="lazy" />
				{#if props.isEditing && props.isSelected}
					<button
						type="button"
						class="author-block__photo-remove"
						onclick={removePhoto}
						aria-label="Remove photo"
					>
						<IconX size={14} />
					</button>
				{/if}
			</div>
		{:else if props.isEditing}
			<div
				class="author-block__photo-placeholder"
				class:author-block__photo-placeholder--uploading={isUploading}
				class:author-block__photo-placeholder--error={!!uploadError}
				ondrop={handleDrop}
				ondragover={handleDragOver}
				role="button"
				tabindex="0"
				onclick={triggerFileInput}
				onkeydown={(e) => e.key === 'Enter' && triggerFileInput()}
				aria-label="Upload author photo"
			>
				{#if isUploading}
					<div class="author-block__spinner" aria-label="Uploading"></div>
				{:else}
					<IconUser size={32} />
					<span class="author-block__upload-hint">Upload photo</span>
				{/if}
			</div>
			<input
				bind:this={fileInputRef}
				type="file"
				accept="image/*"
				onchange={handleFileSelect}
				class="author-block__file-input"
				aria-hidden="true"
				tabindex="-1"
			/>
		{:else}
			<div class="author-block__photo-placeholder author-block__photo-placeholder--empty">
				<IconUser size={32} />
			</div>
		{/if}

		<!-- Photo URL Input (Edit Mode) -->
		{#if props.isEditing && props.isSelected && !sanitizedPhotoURL}
			<div class="author-block__photo-actions">
				{#if showUrlInput}
					<div class="author-block__url-input-wrapper">
						<input
							type="url"
							class="author-block__url-input"
							placeholder="Paste image URL..."
							bind:value={urlInputValue}
							onkeydown={handleURLKeyDown}
							aria-label="Photo URL"
						/>
						<button
							type="button"
							class="author-block__url-submit"
							onclick={handleURLSubmit}
							disabled={!urlInputValue.trim()}
						>
							Add
						</button>
					</div>
				{:else}
					<button
						type="button"
						class="author-block__url-toggle"
						onclick={() => (showUrlInput = true)}
					>
						or paste URL
					</button>
				{/if}
			</div>
		{/if}

		<!-- Upload Error -->
		{#if uploadError}
			<div class="author-block__upload-error" role="alert">
				{uploadError}
			</div>
		{/if}
	</div>

	<!-- Author Content Section -->
	<div class="author-block__content">
		<div class="author-block__header">
			<span class="author-block__label">Written by</span>

			<!-- Author Name -->
			{#if props.isEditing}
				<h3
					contenteditable="true"
					class="author-block__name"
					oninput={(e) => updateContent({ authorName: (e.target as HTMLElement).textContent || '' })}
					onpaste={handlePaste}
					data-placeholder="Author Name"
					role="textbox"
					aria-label="Author name"
				>{name}</h3>
			{:else}
				<h3 class="author-block__name">{name}</h3>
			{/if}

			<!-- Author Title -->
			{#if title || props.isEditing}
				{#if props.isEditing}
					<p
						contenteditable="true"
						class="author-block__title"
						class:author-block__title--empty={!title}
						oninput={(e) => updateContent({ authorTitle: (e.target as HTMLElement).textContent || '' })}
						onpaste={handlePaste}
						data-placeholder="Job title or role"
						role="textbox"
						aria-label="Author title"
					>{title}</p>
				{:else if title}
					<p class="author-block__title">{title}</p>
				{/if}
			{/if}
		</div>

		<!-- Author Bio -->
		{#if props.isEditing}
			<p
				contenteditable="true"
				class="author-block__bio"
				oninput={(e) => updateContent({ authorBio: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
				data-placeholder="Author bio..."
				role="textbox"
				aria-label="Author bio"
			>{bio}</p>
		{:else}
			<p class="author-block__bio">{bio}</p>
		{/if}

		<!-- Social Links -->
		{#if socials.length > 0 || props.isEditing}
			<div class="author-block__socials">
				{#each socials as social, index (index)}
					{#if props.isEditing}
						<div class="author-block__social-edit">
							<select
								value={social.platform}
								onchange={(e) => updateSocial(index, 'platform', (e.target as HTMLSelectElement).value)}
								aria-label="Social platform"
							>
								{#each PLATFORM_OPTIONS as option (option.value)}
									<option value={option.value}>{option.label}</option>
								{/each}
							</select>
							<input
								type="url"
								placeholder="https://..."
								value={social.url}
								oninput={(e) => updateSocial(index, 'url', (e.target as HTMLInputElement).value)}
								aria-label="Social URL"
							/>
							<button
								type="button"
								class="author-block__social-remove"
								onclick={() => removeSocial(index)}
								aria-label="Remove social link"
							>
								<IconX size={14} />
							</button>
						</div>
					{:else if social.url}
						<a
							href={getSanitizedSocialUrl(social.url)}
							class="author-block__social-link"
							target="_blank"
							rel="noopener noreferrer"
							aria-label={social.platform}
						>
							<IconLink size={16} />
							<span class="author-block__social-platform">{social.platform}</span>
						</a>
					{/if}
				{/each}

				{#if props.isEditing}
					<button
						type="button"
						class="author-block__social-add"
						onclick={addSocial}
						aria-label="Add social link"
					>
						<IconPlus size={14} />
						<span>Add Social</span>
					</button>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	/* ==========================================================================
	 * Block Container
	 * ========================================================================== */

	.author-block {
		display: flex;
		gap: 1.5rem;
		padding: 1.5rem;
		background: #f8fafc;
		border-radius: 16px;
		transition: all 0.2s ease;
	}

	.author-block--selected {
		box-shadow: 0 0 0 2px #3b82f6;
	}

	/* ==========================================================================
	 * Photo Section
	 * ========================================================================== */

	.author-block__photo-section {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		flex-shrink: 0;
	}

	.author-block__photo {
		position: relative;
		width: 96px;
		height: 96px;
		border-radius: 50%;
		overflow: hidden;
		background: #e2e8f0;
	}

	.author-block__photo img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.author-block__photo-remove {
		position: absolute;
		top: 0;
		right: 0;
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #dc2626;
		border: 2px solid white;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		opacity: 0;
		transition: opacity 0.15s ease;
	}

	.author-block__photo:hover .author-block__photo-remove,
	.author-block--selected .author-block__photo-remove {
		opacity: 1;
	}

	.author-block__photo-placeholder {
		width: 96px;
		height: 96px;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.25rem;
		background: #e2e8f0;
		border: 2px dashed #94a3b8;
		border-radius: 50%;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.author-block__photo-placeholder:hover {
		border-color: #3b82f6;
		background: #f1f5f9;
		color: #3b82f6;
	}

	.author-block__photo-placeholder--empty {
		border-style: solid;
		cursor: default;
	}

	.author-block__photo-placeholder--uploading {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.author-block__photo-placeholder--error {
		border-color: #dc2626;
		background: #fef2f2;
	}

	.author-block__upload-hint {
		font-size: 0.625rem;
		text-transform: uppercase;
		letter-spacing: 0.025em;
	}

	.author-block__spinner {
		width: 24px;
		height: 24px;
		border: 2px solid #e2e8f0;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: author-spin 1s linear infinite;
	}

	@keyframes author-spin {
		to { transform: rotate(360deg); }
	}

	.author-block__file-input {
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

	.author-block__photo-actions {
		width: 100%;
		max-width: 96px;
	}

	.author-block__url-toggle {
		width: 100%;
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 0.6875rem;
		cursor: pointer;
		transition: color 0.15s ease;
	}

	.author-block__url-toggle:hover {
		color: #3b82f6;
	}

	.author-block__url-input-wrapper {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.author-block__url-input {
		width: 100%;
		padding: 0.375rem;
		font-size: 0.6875rem;
		border: 1px solid #d1d5db;
		border-radius: 4px;
		outline: none;
	}

	.author-block__url-input:focus {
		border-color: #3b82f6;
	}

	.author-block__url-submit {
		padding: 0.25rem 0.5rem;
		background: #1e293b;
		border: none;
		border-radius: 4px;
		color: white;
		font-size: 0.6875rem;
		cursor: pointer;
	}

	.author-block__url-submit:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.author-block__upload-error {
		font-size: 0.6875rem;
		color: #dc2626;
		text-align: center;
		max-width: 120px;
	}

	/* ==========================================================================
	 * Content Section
	 * ========================================================================== */

	.author-block__content {
		flex: 1;
		min-width: 0;
	}

	.author-block__header {
		margin-bottom: 0.75rem;
	}

	.author-block__label {
		display: block;
		font-size: 0.6875rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.25rem;
	}

	.author-block__name {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 700;
		color: #0f172a;
		line-height: 1.3;
		outline: none;
	}

	.author-block__name:empty::before {
		content: attr(data-placeholder);
		color: #94a3b8;
	}

	.author-block--editing .author-block__name {
		padding: 0.25rem 0.5rem;
		margin: -0.25rem -0.5rem;
		border-radius: 6px;
		cursor: text;
	}

	.author-block--editing .author-block__name:hover {
		background: rgba(59, 130, 246, 0.05);
	}

	.author-block--editing .author-block__name:focus {
		background: rgba(59, 130, 246, 0.1);
	}

	.author-block__title {
		margin: 0.25rem 0 0;
		font-size: 0.875rem;
		font-weight: 500;
		color: #3b82f6;
		outline: none;
	}

	.author-block__title:empty::before,
	.author-block__title--empty:empty::before {
		content: attr(data-placeholder);
		color: #94a3b8;
		font-style: italic;
	}

	.author-block--editing .author-block__title {
		padding: 0.125rem 0.5rem;
		margin-left: -0.5rem;
		margin-right: -0.5rem;
		border-radius: 4px;
		cursor: text;
	}

	.author-block--editing .author-block__title:hover {
		background: rgba(59, 130, 246, 0.05);
	}

	.author-block--editing .author-block__title:focus {
		background: rgba(59, 130, 246, 0.1);
	}

	.author-block__bio {
		margin: 0;
		font-size: 0.9375rem;
		line-height: 1.6;
		color: #475569;
		outline: none;
	}

	.author-block__bio:empty::before {
		content: attr(data-placeholder);
		color: #94a3b8;
	}

	.author-block--editing .author-block__bio {
		padding: 0.375rem 0.5rem;
		margin: -0.375rem -0.5rem;
		border-radius: 6px;
		cursor: text;
	}

	.author-block--editing .author-block__bio:hover {
		background: rgba(59, 130, 246, 0.05);
	}

	.author-block--editing .author-block__bio:focus {
		background: rgba(59, 130, 246, 0.1);
	}

	/* ==========================================================================
	 * Social Links
	 * ========================================================================== */

	.author-block__socials {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 1rem;
		align-items: center;
	}

	.author-block__social-link {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: #e2e8f0;
		border-radius: 8px;
		color: #475569;
		font-size: 0.8125rem;
		text-decoration: none;
		transition: all 0.15s ease;
	}

	.author-block__social-link:hover {
		background: #3b82f6;
		color: white;
	}

	.author-block__social-platform {
		text-transform: capitalize;
	}

	.author-block__social-edit {
		display: flex;
		gap: 0.375rem;
		align-items: center;
		width: 100%;
	}

	.author-block__social-edit select {
		padding: 0.375rem 0.5rem;
		font-size: 0.8125rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		color: #1e293b;
		outline: none;
		cursor: pointer;
	}

	.author-block__social-edit select:focus {
		border-color: #3b82f6;
	}

	.author-block__social-edit input {
		flex: 1;
		min-width: 120px;
		padding: 0.375rem 0.5rem;
		font-size: 0.8125rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		outline: none;
	}

	.author-block__social-edit input:focus {
		border-color: #3b82f6;
	}

	.author-block__social-remove {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #fee2e2;
		border: none;
		border-radius: 6px;
		color: #dc2626;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.author-block__social-remove:hover {
		background: #fecaca;
	}

	.author-block__social-add {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: 1px dashed #d1d5db;
		border-radius: 8px;
		color: #64748b;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.author-block__social-add:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	/* ==========================================================================
	 * Responsive - Mobile Stacking with Center Alignment
	 * ========================================================================== */

	@media (max-width: 640px) {
		.author-block {
			flex-direction: column;
			align-items: center;
			text-align: center;
			padding: 1.25rem;
		}

		.author-block__photo-section {
			margin-bottom: 0.5rem;
		}

		.author-block__content {
			display: flex;
			flex-direction: column;
			align-items: center;
		}

		.author-block__header {
			text-align: center;
		}

		.author-block__bio {
			text-align: center;
		}

		.author-block__socials {
			justify-content: center;
		}

		.author-block__social-edit {
			flex-wrap: wrap;
			justify-content: center;
		}

		.author-block__social-edit input {
			width: 100%;
			min-width: auto;
		}
	}

	/* ==========================================================================
	 * Dark Mode
	 * ========================================================================== */

	:global(.dark) .author-block {
		background: #1e293b;
	}

	:global(.dark) .author-block--selected {
		box-shadow: 0 0 0 2px #3b82f6;
	}

	:global(.dark) .author-block__photo {
		background: #334155;
	}

	:global(.dark) .author-block__photo-placeholder {
		background: #334155;
		border-color: #64748b;
		color: #94a3b8;
	}

	:global(.dark) .author-block__photo-placeholder:hover {
		background: #475569;
		border-color: #3b82f6;
		color: #3b82f6;
	}

	:global(.dark) .author-block__photo-placeholder--empty {
		background: #334155;
	}

	:global(.dark) .author-block__photo-placeholder--uploading {
		background: rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .author-block__photo-placeholder--error {
		background: rgba(220, 38, 38, 0.1);
		border-color: #dc2626;
	}

	:global(.dark) .author-block__spinner {
		border-color: #475569;
		border-top-color: #3b82f6;
	}

	:global(.dark) .author-block__url-toggle {
		color: #94a3b8;
	}

	:global(.dark) .author-block__url-toggle:hover {
		color: #3b82f6;
	}

	:global(.dark) .author-block__url-input {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .author-block__url-submit {
		background: #f1f5f9;
		color: #0f172a;
	}

	:global(.dark) .author-block__label {
		color: #94a3b8;
	}

	:global(.dark) .author-block__name {
		color: #f8fafc;
	}

	:global(.dark) .author-block__name:empty::before {
		color: #64748b;
	}

	:global(.dark) .author-block--editing .author-block__name:hover {
		background: rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .author-block--editing .author-block__name:focus {
		background: rgba(59, 130, 246, 0.15);
	}

	:global(.dark) .author-block__title {
		color: #60a5fa;
	}

	:global(.dark) .author-block__title:empty::before,
	:global(.dark) .author-block__title--empty:empty::before {
		color: #64748b;
	}

	:global(.dark) .author-block--editing .author-block__title:hover {
		background: rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .author-block--editing .author-block__title:focus {
		background: rgba(59, 130, 246, 0.15);
	}

	:global(.dark) .author-block__bio {
		color: #94a3b8;
	}

	:global(.dark) .author-block__bio:empty::before {
		color: #64748b;
	}

	:global(.dark) .author-block--editing .author-block__bio:hover {
		background: rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .author-block--editing .author-block__bio:focus {
		background: rgba(59, 130, 246, 0.15);
	}

	:global(.dark) .author-block__social-link {
		background: #334155;
		color: #94a3b8;
	}

	:global(.dark) .author-block__social-link:hover {
		background: #3b82f6;
		color: white;
	}

	:global(.dark) .author-block__social-edit select {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .author-block__social-edit input {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .author-block__social-remove {
		background: rgba(220, 38, 38, 0.2);
		color: #f87171;
	}

	:global(.dark) .author-block__social-remove:hover {
		background: rgba(220, 38, 38, 0.3);
	}

	:global(.dark) .author-block__social-add {
		border-color: #475569;
		color: #94a3b8;
	}

	:global(.dark) .author-block__social-add:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	:global(.dark) .author-block__upload-error {
		color: #f87171;
	}
</style>

<!--
	Image Block - Image with caption
	═══════════════════════════════════════════════════════════════════════════════

	Displays an image with optional caption and alt text.

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import type { ContentBlock } from '$lib/stores/editor.svelte';
	import IconPhoto from '@tabler/icons-svelte/icons/photo';
	import IconUpload from '@tabler/icons-svelte/icons/upload';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: ContentBlock;
		readonly?: boolean;
		onUpdate?: (data: Record<string, unknown>) => void;
	}

	let { block, readonly = false, onUpdate }: Props = $props();

	// ==========================================================================
	// Derived
	// ==========================================================================

	let data = $derived(block.data as {
		assetId?: string;
		url?: string;
		alt?: string;
		caption?: string;
		width?: string;
	});

	let imageUrl = $derived(data.url ?? '');
	let hasImage = $derived(!!imageUrl || !!data.assetId);

	// ==========================================================================
	// Handlers
	// ==========================================================================

	function handleUrlChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, url: target.value });
	}

	function handleAltChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, alt: target.value });
	}

	function handleCaptionChange(e: Event) {
		const target = e.target as HTMLInputElement;
		onUpdate?.({ ...data, caption: target.value });
	}
</script>

<div class="image-block">
	{#if hasImage}
		<figure class="image-figure">
			<img
				src={imageUrl}
				alt={data.alt || 'Image'}
				class="image-preview"
			/>
			{#if data.caption}
				<figcaption class="image-caption">{data.caption}</figcaption>
			{/if}
		</figure>

		{#if !readonly}
			<div class="image-controls">
				<div class="control-group">
					<label class="control-label">Alt text</label>
					<input
						type="text"
						class="control-input"
						value={data.alt ?? ''}
						oninput={handleAltChange}
						placeholder="Describe the image..."
					/>
				</div>
				<div class="control-group">
					<label class="control-label">Caption</label>
					<input
						type="text"
						class="control-input"
						value={data.caption ?? ''}
						oninput={handleCaptionChange}
						placeholder="Add a caption..."
					/>
				</div>
			</div>
		{/if}
	{:else}
		<div class="image-placeholder">
			<div class="placeholder-icon">
				<IconPhoto size={32} />
			</div>
			<p class="placeholder-text">Add an image</p>
			{#if !readonly}
				<div class="placeholder-actions">
					<input
						type="text"
						class="url-input"
						placeholder="Paste image URL..."
						oninput={handleUrlChange}
					/>
					<button type="button" class="upload-btn">
						<IconUpload size={16} />
						Upload
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.image-block {
		width: 100%;
	}

	.image-figure {
		margin: 0;
	}

	.image-preview {
		max-width: 100%;
		height: auto;
		border-radius: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
	}

	.image-caption {
		margin-top: 0.5rem;
		font-size: 0.8125rem;
		color: #94a3b8;
		text-align: center;
		font-style: italic;
	}

	.image-controls {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-top: 1rem;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.5rem;
	}

	.control-group {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.control-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #64748b;
	}

	.control-input {
		padding: 0.5rem 0.75rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #f1f5f9;
		font-size: 0.875rem;
	}

	.control-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	/* Placeholder */
	.image-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		padding: 2rem;
		background: rgba(0, 0, 0, 0.15);
		border: 2px dashed rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		text-align: center;
	}

	.placeholder-icon {
		color: #475569;
	}

	.placeholder-text {
		margin: 0;
		font-size: 0.875rem;
		color: #64748b;
	}

	.placeholder-actions {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		max-width: 300px;
	}

	.url-input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: rgba(30, 41, 59, 0.5);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #f1f5f9;
		font-size: 0.8125rem;
		text-align: center;
	}

	.url-input:focus {
		outline: none;
		border-color: rgba(99, 102, 241, 0.5);
	}

	.upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: rgba(99, 102, 241, 0.15);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 0.375rem;
		color: #818cf8;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.upload-btn:hover {
		background: rgba(99, 102, 241, 0.25);
	}
</style>

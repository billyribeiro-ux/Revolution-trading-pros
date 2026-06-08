<script lang="ts">
	import IconLoader from '@tabler/icons-svelte-runes/icons/loader';
	import IconPhoto from '@tabler/icons-svelte-runes/icons/photo';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import type { PostState } from './types';

	type Props = {
		post: PostState;
		uploadingImage: boolean;
		uploadError: string;
		onUpload: (event: Event) => void;
		onRemove: () => void;
	};

	let { post = $bindable(), uploadingImage, uploadError, onUpload, onRemove }: Props = $props();
</script>

<div class="sidebar-panel">
	<h3>Featured Image</h3>

	{#if uploadingImage}
		<div class="upload-loading">
			<IconLoader size={48} class="spin" />
			<span>Uploading image...</span>
		</div>
	{:else if post.featured_image}
		<div class="featured-image-preview">
			<!-- TODO(cls): width/height needed — user-uploaded blog featured image; capture intrinsic dims on upload and store alongside URL -->
			<img src={post.featured_image} alt={post.featured_image_alt || 'Featured'} loading="lazy" />
			<button type="button" class="remove-image" onclick={onRemove}>
				<IconX size={16} />
			</button>
		</div>

		<div class="form-group">
			<label for="img-title">Image Title</label>
			<input
				id="img-title"
				name="img-title"
				type="text"
				bind:value={post.featured_image_title}
				placeholder="Image title for SEO"
			/>
		</div>

		<div class="form-group">
			<label for="img-alt">Alt Text</label>
			<input
				id="img-alt"
				name="img-alt"
				type="text"
				bind:value={post.featured_image_alt}
				placeholder="Describe the image for accessibility"
			/>
		</div>

		<div class="form-group">
			<label for="img-caption">Caption</label>
			<input
				id="img-caption"
				name="img-caption"
				type="text"
				bind:value={post.featured_image_caption}
				placeholder="Image caption displayed below image"
			/>
		</div>

		<div class="form-group">
			<label for="img-description">Description</label>
			<textarea
				id="img-description"
				bind:value={post.featured_image_description}
				placeholder="Detailed description of the image"
				rows="3"
			></textarea>
		</div>
	{:else}
		<label class={{ 'upload-box': true, disabled: uploadingImage }}>
			<input type="file" accept="image/*" onchange={onUpload} hidden disabled={uploadingImage} />
			<IconPhoto size={48} />
			<span>Click to upload featured image</span>
		</label>
		{#if uploadError}
			<p class="upload-error">{uploadError}</p>
		{/if}
	{/if}
</div>

<style>
	.sidebar-panel {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.sidebar-panel h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #f0f0f0;
	}

	.sidebar-panel input[type='text'] {
		width: 100%;
		padding: 0.625rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.9rem;
		color: #1a1a1a;
		background: white;
	}

	.upload-box {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		border: 2px dashed #e5e5e5;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		color: #999;
	}

	.upload-box:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.upload-box span {
		margin-top: 0.5rem;
		font-size: 0.9rem;
	}

	.upload-box.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.upload-error {
		color: #ef4444;
		font-size: 0.85rem;
		margin-top: 0.5rem;
		text-align: center;
	}

	.featured-image-preview {
		position: relative;
		margin-bottom: 1rem;
	}

	.featured-image-preview img {
		width: 100%;
		border-radius: 6px;
	}

	.remove-image {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.7);
		color: white;
		border: none;
		border-radius: 50%;
		cursor: pointer;
	}

	.upload-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: #3b82f6;
	}

	.upload-loading span {
		margin-top: 0.5rem;
		font-size: 0.9rem;
		color: #666;
	}

	.upload-loading :global(.spin) {
		animation: spin 1s linear infinite;
	}

	.sidebar-panel textarea {
		width: 100%;
		padding: 0.625rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
		color: #1a1a1a;
		background: white;
	}

	.sidebar-panel textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #1a1a1a;
		font-size: 0.95rem;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>

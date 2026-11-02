<script lang="ts">
	/**
	 * R24-C extraction (2026-05-20): name + slug + short/long description +
	 * thumbnail URL. Parent binds `formData` so the child stays a thin form;
	 * `getFieldError` is passed in (parent owns validation state).
	 */
	import { IconPhoto } from '#lib/icons/index.js';
	import type { ProductFormData } from './types';

	interface Props {
		formData: ProductFormData;
		getFieldError: (field: string) => string | undefined;
		onSlugBlur: () => void;
	}

	let { formData = $bindable(), getFieldError, onSlugBlur }: Props = $props();
</script>

<!-- Name -->
<div class={['form-group', { 'has-error': getFieldError('name') }]}>
	<label for="name">Product Name *</label>
	<input autocomplete="off"
		id="name"
		name="name"
		type="text"
		bind:value={formData.name}
		onblur={onSlugBlur}
		placeholder="e.g., Advanced Trading Course"
	/>
	{#if getFieldError('name')}
		<span class="field-error">{getFieldError('name')}</span>
	{/if}
</div>

<!-- Slug -->
<div class={['form-group', { 'has-error': getFieldError('slug') }]}>
	<label for="slug">URL Slug *</label>
	<div class="slug-input">
		<span class="slug-prefix">/products/</span>
		<input autocomplete="off"
			id="slug"
			name="slug"
			type="text"
			bind:value={formData.slug}
			placeholder="advanced-trading-course"
		/>
	</div>
	{#if getFieldError('slug')}
		<span class="field-error">{getFieldError('slug')}</span>
	{/if}
</div>

<!-- Description -->
<div class="form-group">
	<label for="description">Short Description</label>
	<textarea
		id="description"
		bind:value={formData.description}
		placeholder="Brief description of the product..."
		rows="3"></textarea>
</div>

<!-- Long Description -->
<div class="form-group">
	<label for="long_description">Full Description</label>
	<textarea
		id="long_description"
		bind:value={formData.long_description}
		placeholder="Detailed description with features and benefits..."
		rows="6"></textarea>
</div>

<!-- Thumbnail -->
<div class="form-group">
	<label for="thumbnail">
		<IconPhoto size={16} />
		Thumbnail URL
	</label>
	<input autocomplete="off"
		id="thumbnail"
		name="thumbnail"
		type="url"
		bind:value={formData.thumbnail}
		placeholder="https://example.com/image.jpg"
	/>
</div>

<style>
	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group.has-error input {
		border-color: var(--error-base, #da3633);
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.form-group input[type='text'],
	.form-group input[type='url'],
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary-500, #e6b800);
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
	}

	.field-error {
		display: block;
		color: var(--error-emphasis, #f85149);
		font-size: 0.8125rem;
		margin-top: 0.375rem;
	}

	/* Slug Input */
	.slug-input {
		display: flex;
		align-items: center;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		overflow: hidden;
	}

	.slug-prefix {
		padding: 0.75rem;
		background: var(--bg-base, #0d1117);
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		font-family: monospace;
		border-right: 1px solid var(--border-default, #30363d);
	}

	.slug-input input {
		border: none;
		border-radius: 0;
		background: transparent;
	}

	.slug-input input:focus {
		box-shadow: none;
	}
</style>

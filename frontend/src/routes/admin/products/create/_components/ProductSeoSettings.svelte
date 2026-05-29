<script lang="ts">
	/**
	 * R24-C extraction (2026-05-20): collapsible SEO settings block (meta
	 * title, meta description, indexable checkbox, canonical URL). The visible
	 * / hidden toggle state lives here — it has no side effects on the parent
	 * form, so the parent doesn't need to know about it.
	 */
	import type { ProductFormData } from './types';

	interface Props {
		formData: ProductFormData;
	}

	let { formData = $bindable() }: Props = $props();

	let showSeoSettings = $state(false);
</script>

<button type="button" class="seo-toggle" onclick={() => (showSeoSettings = !showSeoSettings)}>
	{showSeoSettings ? 'Hide' : 'Show'} SEO Settings
</button>

{#if showSeoSettings}
	<div class="seo-settings">
		<div class="form-group">
			<label for="meta_title">Meta Title</label>
			<input
				id="meta_title"
				name="meta_title"
				type="text"
				bind:value={formData.meta_title}
				placeholder="SEO title for search engines"
			/>
		</div>

		<div class="form-group">
			<label for="meta_description">Meta Description</label>
			<textarea
				id="meta_description"
				bind:value={formData.meta_description}
				placeholder="SEO description for search engines..."
				rows="2"
			></textarea>
		</div>

		<div class="form-group">
			<label class="checkbox-label">
				<input id="indexable" name="indexable" type="checkbox" bind:checked={formData.indexable} />
				<span>Allow search engine indexing</span>
			</label>
		</div>

		<div class="form-group">
			<label for="canonical_url">Canonical URL</label>
			<input
				id="canonical_url"
				name="canonical_url"
				type="url"
				bind:value={formData.canonical_url}
				placeholder="https://example.com/canonical-page"
			/>
		</div>
	</div>
{/if}

<style>
	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.form-group input[type='text'],
	.form-group input[type='url'],
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.25);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
	}

	/* Checkbox */
	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: #f1f5f9;
	}

	.checkbox-label input[type='checkbox'] {
		width: 20px;
		height: 20px;
		cursor: pointer;
		accent-color: #3b82f6;
	}

	/* SEO Toggle */
	.seo-toggle {
		width: 100%;
		padding: 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px dashed rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: 1rem;
	}

	.seo-toggle:hover {
		background: rgba(148, 163, 184, 0.15);
		color: #f1f5f9;
	}

	.seo-settings {
		padding: 1.5rem;
		background: rgba(148, 163, 184, 0.05);
		border-radius: 12px;
		border: 1px solid rgba(148, 163, 184, 0.15);
		margin-bottom: 1.5rem;
	}
</style>

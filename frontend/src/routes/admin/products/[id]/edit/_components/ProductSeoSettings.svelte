<script lang="ts">
	/**
	 * R20-C extraction (2026-05-20): SEO toggle + collapsible block.
	 * Owns the local `expanded` toggle (was `showSeoSettings` in the parent).
	 * Mutates the parent's `formData` proxy directly for the four SEO fields.
	 */
	interface FormFields {
		meta_title: string;
		meta_description: string;
		indexable: boolean;
		canonical_url: string;
	}

	interface Props {
		formData: FormFields;
	}

	let { formData = $bindable() }: Props = $props();

	let expanded = $state(false);
</script>

<button type="button" class="seo-toggle" onclick={() => (expanded = !expanded)}>
	{expanded ? 'Hide' : 'Show'} SEO Settings
</button>

{#if expanded}
	<div class="seo-settings">
		<div class="form-group">
			<label for="meta_title">Meta Title</label>
			<input autocomplete="off"
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
				rows="2"></textarea>
		</div>

		<div class="form-group">
			<label class="checkbox-label">
				<input
					id="edit-indexable"
					name="edit-indexable"
					type="checkbox"
					bind:checked={formData.indexable}
				/>
				<span>Allow search engine indexing</span>
			</label>
		</div>

		<div class="form-group">
			<label for="canonical_url">Canonical URL</label>
			<input autocomplete="off"
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
	.seo-toggle {
		width: 100%;
		padding: 0.75rem;
		background: var(--bg-surface, #1c2128);
		border: 1px dashed var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.2s;
		margin-bottom: 1rem;
	}

	.seo-toggle:hover {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.seo-settings {
		padding: 1.5rem;
		background: var(--bg-surface, #1c2128);
		border-radius: 12px;
		border: 1px solid var(--border-muted, #21262d);
		margin-bottom: 1.5rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: block;
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
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.15);
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: var(--text-primary, #f0f6fc);
	}

	.checkbox-label input[type='checkbox'] {
		width: 20px;
		height: 20px;
		cursor: pointer;
		accent-color: #3b82f6;
	}
</style>

<script lang="ts">
	export let meta: any;

	$: {
		// Auto-generate meta title from post title if not set
		if (!meta.meta_title && meta.title) {
			meta.meta_title = meta.title;
		}
	}
</script>

<div class="seo-fields">
	<div class="form-group">
		<label for="meta-title">
			Meta Title
			<span class="char-count">{(meta.meta_title || '').length}/60</span>
		</label>
		<input
			id="meta-title"
			type="text"
			bind:value={meta.meta_title}
			placeholder="SEO title for search engines"
			maxlength="70"
		/>
		<div class="hint">Recommended: 50-60 characters</div>
	</div>

	<div class="form-group">
		<label for="meta-description">
			Meta Description
			<span class="char-count">{(meta.meta_description || '').length}/160</span>
		</label>
		<textarea
			id="meta-description"
			bind:value={meta.meta_description}
			placeholder="Brief description for search results"
			rows="3"
			maxlength="170"
		></textarea>
		<div class="hint">Recommended: 150-160 characters</div>
	</div>

	<div class="form-group">
		<label for="canonical-url">Canonical URL</label>
		<input
			id="canonical-url"
			type="url"
			bind:value={meta.canonical_url}
			placeholder="https://example.com/post-url"
		/>
		<div class="hint">Leave empty to use default URL</div>
	</div>

	<div class="form-group">
		<label>
			<input type="checkbox" bind:checked={meta.indexable} />
			Allow search engines to index this post
		</label>
	</div>
</div>

<style>
	.seo-fields {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-weight: 500;
		color: #1a1a1a;
		font-size: 0.95rem;
	}

	.char-count {
		font-size: 0.85rem;
		color: #999;
		font-weight: 400;
	}

	.form-group input[type='text'],
	.form-group input[type='url'],
	.form-group textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
	}

	.form-group textarea {
		resize: vertical;
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.hint {
		font-size: 0.85rem;
		color: #999;
	}

	.form-group label:has(input[type='checkbox']) {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-weight: 400;
	}

	.form-group input[type='checkbox'] {
		width: auto;
		cursor: pointer;
	}
</style>

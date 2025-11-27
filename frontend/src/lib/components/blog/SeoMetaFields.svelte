<script lang="ts">
	import { IconX, IconPlus } from '@tabler/icons-svelte';
	
	interface Props {
		meta: any;
	}

	let { meta = $bindable() }: Props = $props();
	
	let newKeyword = $state('');

	// Initialize keywords array if not present
	$effect(() => {
		if (!meta.meta_keywords) {
			meta.meta_keywords = [];
		}
	});

	$effect(() => {
		// Auto-generate meta title from post title if not set
		if (!meta.meta_title && meta.title) {
			meta.meta_title = meta.title;
		}
	});

	function addKeyword() {
		if (newKeyword.trim() && !meta.meta_keywords.includes(newKeyword.trim())) {
			meta.meta_keywords = [...meta.meta_keywords, newKeyword.trim()];
			newKeyword = '';
		}
	}

	function removeKeyword(keyword: string) {
		meta.meta_keywords = meta.meta_keywords.filter((k: string) => k !== keyword);
	}

	function handleKeywordKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			addKeyword();
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
		<label for="meta-keywords">Focus Keywords</label>
		<div class="keywords-container">
			{#if meta.meta_keywords && meta.meta_keywords.length > 0}
				<div class="keywords-list">
					{#each meta.meta_keywords as keyword}
						<span class="keyword-tag">
							{keyword}
							<button type="button" class="remove-keyword" onclick={() => removeKeyword(keyword)}>
								<IconX size={14} />
							</button>
						</span>
					{/each}
				</div>
			{/if}
			<div class="keyword-input-row">
				<input
					id="meta-keywords"
					type="text"
					bind:value={newKeyword}
					placeholder="Add a keyword..."
					onkeydown={handleKeywordKeydown}
				/>
				<button type="button" class="add-keyword-btn" onclick={addKeyword}>
					<IconPlus size={16} />
				</button>
			</div>
		</div>
		<div class="hint">Add keywords that describe your content (press Enter to add)</div>
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

	/* Keywords styles */
	.keywords-container {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.keywords-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.keyword-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: #e0f2fe;
		color: #0369a1;
		border-radius: 20px;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.remove-keyword {
		display: flex;
		align-items: center;
		justify-content: center;
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		color: #0369a1;
		opacity: 0.7;
	}

	.remove-keyword:hover {
		opacity: 1;
	}

	.keyword-input-row {
		display: flex;
		gap: 0.5rem;
	}

	.keyword-input-row input {
		flex: 1;
	}

	.add-keyword-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		transition: background 0.2s;
	}

	.add-keyword-btn:hover {
		background: #2563eb;
	}
</style>

<script lang="ts">
	/**
	 * SEO Meta Fields Component - Svelte 5
	 * Form fields for managing blog post SEO metadata
	 *
	 * @version 2.0.0 - January 2026
	 * Updated: TypeScript strict mode, modern CSS patterns
	 */
	import { IconX, IconPlus } from '$lib/icons';

	/**
	 * SEO metadata structure for blog posts
	 */
	interface SeoMetadata {
		title?: string;
		meta_title?: string;
		meta_description?: string;
		meta_keywords?: string[];
		canonical_url?: string;
		indexable?: boolean;
	}

	interface Props {
		meta: SeoMetadata;
	}

	let props: Props = $props();

	// Access bindable meta from props
	let meta = $derived(props.meta);

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

	function addKeyword(): void {
		const trimmedKeyword = newKeyword.trim();
		if (trimmedKeyword && !meta.meta_keywords?.includes(trimmedKeyword)) {
			meta.meta_keywords = [...(meta.meta_keywords ?? []), trimmedKeyword];
			newKeyword = '';
		}
	}

	function removeKeyword(keyword: string): void {
		meta.meta_keywords = meta.meta_keywords?.filter((k: string) => k !== keyword) ?? [];
	}

	function handleKeywordKeydown(e: KeyboardEvent): void {
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
	/* 2026 CSS Standards: CSS Layers, oklch colors, container queries */
	@layer components {
		.seo-fields {
			--seo-text-primary: oklch(0.25 0.01 270);
			--seo-text-secondary: oklch(0.55 0.01 270);
			--seo-border: oklch(0.9 0.01 270);
			--seo-accent: oklch(0.6 0.19 250);
			--seo-accent-hover: oklch(0.5 0.21 250);
			--seo-tag-bg: oklch(0.95 0.05 220);
			--seo-tag-text: oklch(0.4 0.15 220);
			--seo-bg: oklch(1 0 0);
			--seo-focus-ring: oklch(0.6 0.19 250 / 0.4);

			display: flex;
			flex-direction: column;
			gap: 1.5rem;
			container-type: inline-size;
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
			color: var(--seo-text-primary);
			font-size: 0.95rem;
		}

		.char-count {
			font-size: 0.85rem;
			color: var(--seo-text-secondary);
			font-weight: 400;
		}

		.form-group input[type='text'],
		.form-group input[type='url'],
		.form-group textarea {
			width: 100%;
			padding: 0.75rem;
			border: 1px solid var(--seo-border);
			border-radius: 6px;
			font-size: 0.95rem;
			font-family: inherit;
			color: var(--seo-text-primary);
			background: var(--seo-bg);
			transition:
				border-color 0.2s ease,
				box-shadow 0.2s ease;
		}

		.form-group textarea {
			resize: vertical;
		}

		.form-group input:focus,
		.form-group textarea:focus {
			outline: none;
			border-color: var(--seo-accent);
			box-shadow: 0 0 0 3px var(--seo-focus-ring);
		}

		.hint {
			font-size: 0.85rem;
			color: var(--seo-text-secondary);
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
			accent-color: var(--seo-accent);
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
			background: var(--seo-tag-bg);
			color: var(--seo-tag-text);
			border-radius: 20px;
			font-size: 0.875rem;
			font-weight: 500;
			transition: background-color 0.2s ease;
		}

		.keyword-tag:hover {
			background: color-mix(in oklch, var(--seo-tag-bg) 85%, var(--seo-accent));
		}

		.remove-keyword {
			display: flex;
			align-items: center;
			justify-content: center;
			background: none;
			border: none;
			padding: 0;
			cursor: pointer;
			color: var(--seo-tag-text);
			opacity: 0.7;
			transition: opacity 0.2s ease;
		}

		.remove-keyword:hover {
			opacity: 1;
		}

		.remove-keyword:focus-visible {
			outline: 2px solid var(--seo-accent);
			outline-offset: 2px;
			border-radius: 2px;
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
			background: var(--seo-accent);
			color: white;
			border: none;
			border-radius: 6px;
			cursor: pointer;
			transition:
				background-color 0.2s ease,
				transform 0.1s ease;
		}

		.add-keyword-btn:hover {
			background: var(--seo-accent-hover);
		}

		.add-keyword-btn:active {
			transform: scale(0.95);
		}

		.add-keyword-btn:focus-visible {
			outline: 2px solid var(--seo-accent);
			outline-offset: 2px;
		}

		/* Container query responsive adjustments */
		@container (max-width: 400px) {
			.form-group label {
				flex-direction: column;
				align-items: flex-start;
				gap: 0.25rem;
			}

			.keyword-input-row {
				flex-direction: column;
			}

			.add-keyword-btn {
				width: 100%;
			}
		}
	}
</style>

<!--
/**
 * Related Posts Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Grid of 2-4 related post cards with image, category, title, excerpt, and link
 * Production-ready with accessibility, dark mode, and responsive design
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { IconPhoto, IconExternalLink, IconLayoutGrid, IconList } from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import type { Block, BlockContent, BlockSettings } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// =========================================================================
	// Types
	// =========================================================================

	interface RelatedPost {
		id: string;
		title: string;
		excerpt: string;
		image: string;
		category: string;
		url: string;
	}

	type LayoutMode = 'grid' | 'list';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	// =========================================================================
	// Props
	// =========================================================================

	let props: Props = $props();

	// =========================================================================
	// Mock Data (API integration placeholder)
	// =========================================================================

	const MOCK_POSTS: RelatedPost[] = [
		{
			id: '1',
			title: 'Mastering Technical Analysis for Day Trading',
			excerpt:
				'Learn the essential technical analysis indicators and patterns that professional day traders use to identify high-probability setups.',
			image: '/images/placeholder-trading-1.jpg',
			category: 'Trading',
			url: '/blog/technical-analysis-day-trading'
		},
		{
			id: '2',
			title: 'Risk Management Strategies Every Trader Needs',
			excerpt:
				'Discover proven risk management techniques to protect your capital and maximize your trading performance over the long term.',
			image: '/images/placeholder-trading-2.jpg',
			category: 'Education',
			url: '/blog/risk-management-strategies'
		},
		{
			id: '3',
			title: 'Understanding Market Psychology and Sentiment',
			excerpt:
				'Explore how market psychology affects price movements and learn to read sentiment indicators for better trading decisions.',
			image: '/images/placeholder-trading-3.jpg',
			category: 'Analysis',
			url: '/blog/market-psychology-sentiment'
		},
		{
			id: '4',
			title: 'Building a Winning Trading Plan',
			excerpt:
				'Step-by-step guide to creating a comprehensive trading plan that aligns with your goals, risk tolerance, and trading style.',
			image: '/images/placeholder-trading-4.jpg',
			category: 'Strategy',
			url: '/blog/building-trading-plan'
		}
	];

	// =========================================================================
	// Derived Values
	// =========================================================================

	let title = $derived(props.block.content.relatedPostsTitle || 'Related Articles');
	let layout = $derived((props.block.content.relatedPostsLayout as LayoutMode) || 'grid');
	let postCount = $derived(props.block.content.relatedPostsCount || 3);
	let showExcerpt = $derived(props.block.settings.relatedPostsShowExcerpt !== false);
	let showCategory = $derived(props.block.settings.relatedPostsShowCategory !== false);

	let displayPosts = $derived(MOCK_POSTS.slice(0, Math.min(Math.max(postCount, 2), 4)));

	// =========================================================================
	// Handlers
	// =========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function updateSettings(updates: Partial<BlockSettings>): void {
		props.onUpdate({ settings: { ...props.block.settings, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function handleCardClick(e: MouseEvent, _post: RelatedPost): void {
		if (props.isEditing) {
			e.preventDefault();
		}
	}

	function handleKeyDown(e: KeyboardEvent, post: RelatedPost): void {
		if (e.key === 'Enter' || e.key === ' ') {
			if (!props.isEditing) {
				window.location.href = post.url;
			}
		}
	}
</script>

<section class="related-posts-block" aria-label="Related articles">
	<!-- Section Header -->
	<header class="related-posts-header">
		{#if props.isEditing}
			<h3
				contenteditable="true"
				class="related-posts-title"
				oninput={(e) =>
					updateContent({ relatedPostsTitle: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>
				{title}
			</h3>
		{:else}
			<h3 class="related-posts-title">{title}</h3>
		{/if}

		{#if props.isEditing && props.isSelected}
			<div class="layout-toggle">
				<button
					type="button"
					class="toggle-btn"
					class:active={layout === 'grid'}
					aria-label="Grid layout"
					onclick={() => updateContent({ relatedPostsLayout: 'grid' })}
				>
					<IconLayoutGrid size={18} aria-hidden="true" />
				</button>
				<button
					type="button"
					class="toggle-btn"
					class:active={layout === 'list'}
					aria-label="List layout"
					onclick={() => updateContent({ relatedPostsLayout: 'list' })}
				>
					<IconList size={18} aria-hidden="true" />
				</button>
			</div>
		{/if}
	</header>

	<!-- Posts Grid/List -->
	<div
		class="related-posts-container layout-{layout}"
		class:cols-2={postCount === 2}
		class:cols-3={postCount === 3}
		class:cols-4={postCount === 4}
		role="list"
		aria-label="Related posts list"
	>
		{#each displayPosts as post (post.id)}
			<article class="post-card" role="listitem">
				{#if props.isEditing}
					<!-- Edit Mode: Placeholder Card -->
					<div class="post-card-inner">
						<div class="post-image">
							<div class="image-placeholder">
								<IconPhoto size={32} aria-hidden="true" />
								<span>Post Image</span>
							</div>
						</div>
						<div class="post-content">
							{#if showCategory}
								<span class="post-category">{post.category}</span>
							{/if}
							<h4 class="post-title">{post.title}</h4>
							{#if showExcerpt}
								<p class="post-excerpt">{post.excerpt}</p>
							{/if}
							<span class="post-link">
								Read more
								<IconExternalLink size={14} aria-hidden="true" />
							</span>
						</div>
					</div>
				{:else}
					<!-- View Mode: Actual Link Card -->
					<a
						href={sanitizeURL(post.url) || '#'}
						class="post-card-inner"
						onclick={(e) => handleCardClick(e, post)}
						onkeydown={(e) => handleKeyDown(e, post)}
					>
						<div class="post-image">
							{#if post.image}
								<img src={sanitizeURL(post.image)} alt={post.title} loading="lazy" />
							{:else}
								<div class="image-placeholder">
									<IconPhoto size={32} aria-hidden="true" />
								</div>
							{/if}
						</div>
						<div class="post-content">
							{#if showCategory}
								<span class="post-category">{post.category}</span>
							{/if}
							<h4 class="post-title">{post.title}</h4>
							{#if showExcerpt}
								<p class="post-excerpt">{post.excerpt}</p>
							{/if}
							<span class="post-link">
								Read more
								<IconExternalLink size={14} aria-hidden="true" />
							</span>
						</div>
					</a>
				{/if}
			</article>
		{/each}
	</div>
</section>

<!-- Settings Panel (Edit Mode) -->
{#if props.isEditing && props.isSelected}
	<div class="related-posts-settings">
		<label class="setting-field">
			<span>Number of Posts:</span>
			<select
				value={postCount}
				onchange={(e) =>
					updateContent({ relatedPostsCount: parseInt((e.target as HTMLSelectElement).value, 10) })}
			>
				<option value="2">2 Posts</option>
				<option value="3">3 Posts</option>
				<option value="4">4 Posts</option>
			</select>
		</label>

		<label class="setting-checkbox">
			<input
				type="checkbox"
				checked={showCategory}
				onchange={(e) =>
					updateSettings({ relatedPostsShowCategory: (e.target as HTMLInputElement).checked })}
			/>
			<span>Show Category</span>
		</label>

		<label class="setting-checkbox">
			<input
				type="checkbox"
				checked={showExcerpt}
				onchange={(e) =>
					updateSettings({ relatedPostsShowExcerpt: (e.target as HTMLInputElement).checked })}
			/>
			<span>Show Excerpt</span>
		</label>

		<div class="settings-note">
			<small
				>Posts will be automatically populated from your content library based on category and tags.</small
			>
		</div>
	</div>
{/if}

<style>
	/* Container */
	.related-posts-block {
		padding: 1.5rem 0;
	}

	/* Header */
	.related-posts-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1.5rem;
		gap: 1rem;
	}

	.related-posts-title {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 700;
		color: #0f172a;
		outline: none;
	}

	/* Layout Toggle */
	.layout-toggle {
		display: flex;
		gap: 0.25rem;
		padding: 0.25rem;
		background: #f1f5f9;
		border-radius: 8px;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		border-radius: 6px;
		background: transparent;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.toggle-btn:hover {
		color: #3b82f6;
	}

	.toggle-btn.active {
		background: white;
		color: #3b82f6;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	/* Posts Container */
	.related-posts-container {
		display: grid;
		gap: 1.5rem;
	}

	.related-posts-container.layout-grid {
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
	}

	.related-posts-container.layout-grid.cols-2 {
		grid-template-columns: repeat(2, 1fr);
	}

	.related-posts-container.layout-grid.cols-3 {
		grid-template-columns: repeat(3, 1fr);
	}

	.related-posts-container.layout-grid.cols-4 {
		grid-template-columns: repeat(4, 1fr);
	}

	.related-posts-container.layout-list {
		grid-template-columns: 1fr;
	}

	/* Post Card */
	.post-card {
		position: relative;
	}

	.post-card-inner {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
		text-decoration: none;
		color: inherit;
		transition: all 0.2s ease;
	}

	.layout-list .post-card-inner {
		flex-direction: row;
	}

	a.post-card-inner:hover {
		border-color: #3b82f6;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
		transform: translateY(-2px);
	}

	a.post-card-inner:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* Post Image */
	.post-image {
		position: relative;
		aspect-ratio: 16/9;
		background: #f3f4f6;
		overflow: hidden;
		flex-shrink: 0;
	}

	.layout-list .post-image {
		width: 200px;
		aspect-ratio: auto;
		height: 100%;
		min-height: 140px;
	}

	.post-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease;
	}

	a.post-card-inner:hover .post-image img {
		transform: scale(1.05);
	}

	.image-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		height: 100%;
		color: #9ca3af;
		font-size: 0.75rem;
	}

	/* Post Content */
	.post-content {
		display: flex;
		flex-direction: column;
		padding: 1.25rem;
		flex: 1;
	}

	.post-category {
		display: inline-block;
		width: fit-content;
		padding: 0.25rem 0.625rem;
		margin-bottom: 0.75rem;
		background: #eff6ff;
		color: #3b82f6;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.025em;
		border-radius: 4px;
	}

	.post-title {
		margin: 0 0 0.625rem;
		font-size: 1.0625rem;
		font-weight: 700;
		color: #0f172a;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.post-excerpt {
		margin: 0 0 1rem;
		font-size: 0.875rem;
		line-height: 1.6;
		color: #64748b;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		flex: 1;
	}

	.post-link {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: #3b82f6;
		margin-top: auto;
	}

	/* Settings Panel */
	.related-posts-settings {
		display: flex;
		gap: 1.5rem;
		flex-wrap: wrap;
		align-items: center;
		margin-top: 1.5rem;
		padding: 1rem;
		background: #f8fafc;
		border-radius: 8px;
		border: 1px solid #e2e8f0;
	}

	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #475569;
	}

	.setting-field select {
		padding: 0.375rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.setting-checkbox {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		color: #475569;
		cursor: pointer;
	}

	.setting-checkbox input {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	.settings-note {
		flex-basis: 100%;
		color: #64748b;
	}

	.settings-note small {
		font-size: 0.75rem;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.related-posts-container.layout-grid.cols-4 {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 768px) {
		.related-posts-container.layout-grid.cols-3,
		.related-posts-container.layout-grid.cols-4 {
			grid-template-columns: 1fr;
		}

		.layout-list .post-card-inner {
			flex-direction: column;
		}

		.layout-list .post-image {
			width: 100%;
			height: auto;
			aspect-ratio: 16/9;
		}
	}

	/* Dark Mode */
	:global(.dark) .related-posts-title {
		color: #f8fafc;
	}

	:global(.dark) .layout-toggle {
		background: #334155;
	}

	:global(.dark) .toggle-btn {
		color: #94a3b8;
	}

	:global(.dark) .toggle-btn:hover {
		color: #60a5fa;
	}

	:global(.dark) .toggle-btn.active {
		background: #1e293b;
		color: #60a5fa;
	}

	:global(.dark) .post-card-inner {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) a.post-card-inner:hover {
		border-color: #60a5fa;
	}

	:global(.dark) .post-image {
		background: #0f172a;
	}

	:global(.dark) .image-placeholder {
		color: #475569;
	}

	:global(.dark) .post-category {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	:global(.dark) .post-title {
		color: #f8fafc;
	}

	:global(.dark) .post-excerpt {
		color: #94a3b8;
	}

	:global(.dark) .post-link {
		color: #60a5fa;
	}

	:global(.dark) .related-posts-settings {
		background: #1e293b;
		border-color: #334155;
	}

	:global(.dark) .setting-field,
	:global(.dark) .setting-checkbox {
		color: #94a3b8;
	}

	:global(.dark) .setting-field select {
		background: #0f172a;
		border-color: #475569;
		color: #e2e8f0;
	}

	:global(.dark) .settings-note {
		color: #64748b;
	}
</style>

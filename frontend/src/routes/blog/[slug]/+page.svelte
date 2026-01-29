<script lang="ts">
	/**
	 * Blog Post Page - Svelte 5 Runes Implementation
	 * ICT11+ Production-Grade with Full Analytics & Engagement Features
	 * @version 4.0.0 - January 2026
	 * Updated: CSS layers, oklch colors, container queries, modern image optimization
	 */
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import BlurHashImage from '$lib/components/ui/BlurHashImage.svelte';
	import TableOfContents from '$lib/components/blog/TableOfContents.svelte';
	import FloatingTocWidget from '$lib/components/blog/FloatingTocWidget.svelte';
	import ReadingProgress from '$lib/components/blog/ReadingProgress.svelte';
	import SocialShare from '$lib/components/blog/SocialShare.svelte';
	import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
	import type { Post } from '$lib/types/post';
	import { sanitizeBlogContent } from '$lib/utils/sanitize';
	import MarketingFooter from '$lib/components/sections/MarketingFooter.svelte';
	import {
		initReadingAnalytics,
		calculateReadingTime,
		formatReadingTime
	} from '$lib/utils/readingAnalytics';

	// Import print stylesheet
	import '$lib/styles/print.css';

	interface Category {
		id: number;
		name: string;
		slug: string;
		description: string | null;
	}

	interface Tag {
		id: number;
		name: string;
		slug: string;
	}

	interface BlogPost extends Post {
		categories?: Category[];
		tags?: Tag[];
		views?: number;
		author_image?: string | null;
		blurhash?: string | null;
		featured_image_blurhash?: string | null;
	}

	// Svelte 5 state runes
	let post = $state<BlogPost | null>(null);
	let loading = $state(true);
	let error = $state('');

	// Derived values from page store
	let slug = $derived(page.params['slug']!);

	// Load post when slug changes
	$effect(() => {
		if (slug) {
			loadPost(slug);
		}
	});

	async function loadPost(currentSlug: string) {
		try {
			loading = true;
			error = '';

			const response = await apiFetch<BlogPost>(API_ENDPOINTS.posts.single(currentSlug));
			post = response;
		} catch (err) {
			error = err instanceof Error ? err.message : 'An error occurred';
			console.error('Error loading post:', err);
		} finally {
			loading = false;
		}
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function goBack() {
		goto('/blog');
	}

	interface ContentBlock {
		type: string;
		data?: {
			text?: string;
			items?: string[];
			[key: string]: unknown;
		};
	}

	// Calculate reading time from post content
	let readingTime = $derived.by(() => {
		if (!post) return 0;
		// Extract text from content blocks
		const text =
			post.content_blocks
				?.map((block: ContentBlock) => {
					if (block.type === 'paragraph' || block.type === 'heading') {
						return block.data?.text || '';
					}
					if (block.type === 'list') {
						return (block.data?.items || []).join(' ');
					}
					return '';
				})
				.join(' ') ||
			post.excerpt ||
			'';
		return calculateReadingTime(text);
	});

	// Initialize reading analytics when post loads
	$effect(() => {
		if (!browser || !post) return;
		{
			const cleanup = initReadingAnalytics({
				postId: post.id,
				slug: post.slug,
				contentSelector: '.post-body',
				options: {
					debug: import.meta.env.DEV,
					endpoint: '/api/analytics/reading'
				}
			});
			return cleanup;
		}
	});

	// Track social shares
	function handleSocialShare(platform: string) {
		console.log(`Shared on ${platform}:`, post?.slug);
	}

	// Derived SEO values - Svelte 5 runes
	let articleSchema = $derived(
		post &&
			(post.schema_markup || {
				'@context': 'https://schema.org',
				'@type': 'Article',
				headline: post.title,
				description: post.excerpt || post.meta_description || '',
				image:
					post.featured_image ||
					'https://revolution-trading-pros.pages.dev/revolution-trading-pros.png',
				datePublished: post.published_at,
				dateModified: post.published_at,
				author: {
					'@type': 'Person',
					name: post.author?.name || 'Revolution Trading Pros'
				},
				publisher: {
					'@type': 'Organization',
					name: 'Revolution Trading Pros',
					logo: {
						'@type': 'ImageObject',
						url: 'https://revolution-trading-pros.pages.dev/revolution-trading-pros.png'
					}
				},
				mainEntityOfPage: {
					'@type': 'WebPage',
					'@id': `https://revolution-trading-pros.pages.dev/blog/${post.slug}`
				}
			})
	);

	let seoTitle = $derived(post && (post.meta_title || post.title));
	let seoDescription = $derived(post && (post.meta_description || post.excerpt || ''));
</script>

{#if post}
	<SEOHead
		title={seoTitle ?? post.title}
		description={seoDescription ?? ''}
		canonical={`/blog/${post.slug}`}
		ogType="article"
		ogImage={post.featured_image}
		author={post.author?.name ?? null}
		publishedTime={post.published_at}
		schema={articleSchema}
	/>
{/if}

<!-- Reading Progress Indicator -->
<ReadingProgress contentSelector=".post-body" height={4} color="#3b82f6" position="top" />

<div class="blog-post-container">
	{#if loading}
		<div class="loading" role="status" aria-live="polite" aria-busy="true">
			<div class="spinner" aria-hidden="true"></div>
			<p>Loading post...</p>
		</div>
	{:else if error}
		<div class="error" role="alert" aria-live="assertive">
			<h2>Oops!</h2>
			<p>{error}</p>
			<button class="btn-back" onclick={goBack} aria-label="Go back to blog listing">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="20"
					height="20"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					aria-hidden="true"
				>
					<polyline points="15 18 9 12 15 6"></polyline>
				</svg>
				Back to Blog
			</button>
		</div>
	{:else if post}
		<article class="blog-post">
			<div class="post-header">
				<button class="btn-back-header" onclick={goBack} aria-label="Go back to blog listing">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						aria-hidden="true"
					>
						<polyline points="15 18 9 12 15 6"></polyline>
					</svg>
					Back to Blog
				</button>

				{#if post.categories && post.categories.length > 0}
					<div class="header-categories">
						{#each post.categories as category}
							<span class="category-badge">{category.name}</span>
						{/each}
					</div>
				{/if}

				<h1 class="post-title">{post.title}</h1>

				{#if post.excerpt}
					<p class="post-lead">{post.excerpt}</p>
				{/if}

				<div class="post-meta">
					<div class="author-info">
						{#if post.author_image}
							<img
								src={post.author_image}
								alt={post.author?.name ?? 'Author'}
								class="author-avatar"
							/>
						{:else if post.author}
							<div class="author-avatar-placeholder">
								{post.author.name.charAt(0).toUpperCase()}
							</div>
						{/if}
						<div class="author-details">
							{#if post.author}
								<span class="author-name">{post.author.name}</span>
							{/if}
							<div class="meta-row">
								<span class="date">{formatDate(post.published_at)}</span>
								<span class="separator">•</span>
								<span class="reading-time">{formatReadingTime(readingTime)}</span>
								{#if post.views !== undefined}
									<span class="separator">•</span>
									<span class="views">{post.views.toLocaleString()} views</span>
								{/if}
							</div>
						</div>
					</div>
				</div>
			</div>

			{#if post.featured_image}
				<div class="featured-image">
					<BlurHashImage
						src={post.featured_image}
						alt={post.title}
						blurhash={(post.featured_image_blurhash || post.blurhash) ?? null}
						width="100%"
						height="100%"
						loading="eager"
						decoding="async"
						sizes="(max-width: 768px) 100vw, 800px"
					/>
				</div>
			{/if}

			<div class="post-body">
				<!-- Table of Contents -->
				{#if post.content_blocks && post.content_blocks.length > 0}
					<TableOfContents
						contentBlocks={post.content_blocks}
						title="In This Article"
						minHeadings={2}
						maxDepth={4}
						showNumbers={true}
						collapsible={true}
						defaultExpanded={true}
						sticky={false}
						showProgress={true}
						smoothScroll={true}
						highlightActive={true}
						position="inline"
					/>
				{/if}

				<div class="content">
					{#if post.content_blocks && post.content_blocks.length > 0}
						<!-- Structured content blocks (sanitized for XSS protection) -->
						{#each post.content_blocks as block}
							{#if block.type === 'paragraph'}
								<p>{@html sanitizeBlogContent(block.data?.text || '')}</p>
							{:else if block.type === 'heading'}
								{#if block.data?.level === 1}
									<h1>{@html sanitizeBlogContent(block.data?.text || '')}</h1>
								{:else if block.data?.level === 2}
									<h2>{@html sanitizeBlogContent(block.data?.text || '')}</h2>
								{:else if block.data?.level === 3}
									<h3>{@html sanitizeBlogContent(block.data?.text || '')}</h3>
								{:else if block.data?.level === 4}
									<h4>{@html sanitizeBlogContent(block.data?.text || '')}</h4>
								{:else}
									<h5>{@html sanitizeBlogContent(block.data?.text || '')}</h5>
								{/if}
							{:else if block.type === 'list'}
								{#if block.data?.style === 'ordered'}
									<ol>
										{#each block.data?.items || [] as item}
											<li>{@html sanitizeBlogContent(item)}</li>
										{/each}
									</ol>
								{:else}
									<ul>
										{#each block.data?.items || [] as item}
											<li>{@html sanitizeBlogContent(item)}</li>
										{/each}
									</ul>
								{/if}
							{:else if block.type === 'quote'}
								<blockquote>{@html sanitizeBlogContent(block.data?.text || '')}</blockquote>
							{:else if block.type === 'code'}
								<pre><code>{block.data?.code || ''}</code></pre>
							{:else if block.type === 'image'}
								<figure role="figure" aria-label={block.data?.caption || 'Blog image'}>
									<img
										src={block.data?.url || ''}
										alt={block.data?.caption || 'Blog post image'}
										loading="lazy"
										decoding="async"
										width={block.data?.width || undefined}
										height={block.data?.height || undefined}
									/>
									{#if block.data?.caption}
										<figcaption>{block.data.caption}</figcaption>
									{/if}
								</figure>
							{/if}
						{/each}
					{:else}
						<!-- Fallback: HTML content -->
						<p class="no-content">Content coming soon...</p>
					{/if}
				</div>

				{#if post.tags && post.tags.length > 0}
					<div class="tags-section">
						<h3>Tags</h3>
						<div class="tags">
							{#each post.tags as tag}
								<span class="tag">{tag.name}</span>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Social Share Section -->
				<div class="share-section">
					<SocialShare
						url={`https://revolution-trading-pros.pages.dev/blog/${post.slug}`}
						title={post.title}
						description={post.excerpt || post.meta_description || ''}
						hashtags="trading,forex,stocks"
						onShare={handleSocialShare}
						size="medium"
					/>
				</div>
			</div>

			<div class="post-footer">
				<button class="btn-back-footer" onclick={goBack} aria-label="Go back to blog listing">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="20"
						height="20"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						aria-hidden="true"
					>
						<polyline points="15 18 9 12 15 6"></polyline>
					</svg>
					Back to Blog
				</button>
			</div>
		</article>

		<!-- Floating Table of Contents Widget -->
		{#if post.content_blocks && post.content_blocks.length > 0}
			<FloatingTocWidget
				contentBlocks={post.content_blocks}
				showAfterScroll={400}
				title="Contents"
			/>
		{/if}
	{/if}
</div>

<MarketingFooter />

<style>
	/* 2026 CSS Standards: CSS Layers, oklch colors, container queries, color-mix */
	@layer base, components, utilities;

	@layer components {
		.blog-post-container {
			/* Modern oklch color palette for dark theme */
			--post-bg-start: oklch(0.15 0.02 260);
			--post-bg-end: oklch(0.2 0.02 260);
			--post-text-primary: oklch(0.95 0.01 260);
			--post-text-secondary: oklch(0.8 0.01 260);
			--post-text-muted: oklch(0.65 0.02 260);
			--post-accent-blue: oklch(0.7 0.15 240);
			--post-accent-purple: oklch(0.7 0.15 290);
			--post-card-bg: oklch(0.2 0.02 260 / 0.5);
			--post-border: oklch(0.65 0.02 260 / 0.1);

			background: linear-gradient(135deg, var(--post-bg-start) 0%, var(--post-bg-end) 100%);
			padding: 4rem 2rem;
			container-type: inline-size;
		}

		.blog-post {
			max-width: 800px;
			margin: 0 auto;
		}

		.post-header {
			margin-bottom: 3rem;
		}

		.btn-back-header,
		.btn-back,
		.btn-back-footer {
			display: inline-flex;
			align-items: center;
			gap: 0.5rem;
			padding: 0.75rem 1.5rem;
			background: color-mix(in oklch, var(--post-bg-end) 80%, transparent);
			backdrop-filter: blur(10px);
			color: var(--post-text-primary);
			border: 1px solid color-mix(in oklch, var(--post-text-muted) 20%, transparent);
			border-radius: 8px;
			font-weight: 600;
			cursor: pointer;
			transition: all 0.3s;
			margin-bottom: 2rem;
		}

		.btn-back-header:hover,
		.btn-back:hover,
		.btn-back-footer:hover {
			background: color-mix(in oklch, var(--post-accent-blue) 20%, transparent);
			border-color: var(--post-accent-blue);
			transform: translateX(-4px);
		}

		.btn-back-header:focus-visible,
		.btn-back:focus-visible,
		.btn-back-footer:focus-visible {
			outline: 2px solid var(--post-accent-blue);
			outline-offset: 2px;
		}

		.header-categories {
			display: flex;
			flex-wrap: wrap;
			gap: 0.75rem;
			margin-bottom: 1.5rem;
		}

		.category-badge {
			padding: 0.5rem 1rem;
			background: linear-gradient(135deg, var(--post-accent-blue), var(--post-accent-purple));
			color: white;
			font-size: 0.875rem;
			font-weight: 600;
			border-radius: 16px;
			text-transform: uppercase;
			letter-spacing: 0.05em;
		}

		.post-title {
			font-size: 3rem;
			font-weight: 800;
			color: var(--post-text-primary);
			line-height: 1.2;
			margin-bottom: 1rem;
		}

		.post-lead {
			font-size: 1.25rem;
			color: var(--post-text-secondary);
			line-height: 1.6;
			margin-bottom: 2rem;
		}

		.post-meta {
			padding: 1.5rem;
			background: var(--post-card-bg);
			backdrop-filter: blur(10px);
			border-radius: 12px;
			border: 1px solid var(--post-border);
		}

		.author-info {
			display: flex;
			align-items: center;
			gap: 1rem;
		}

		.author-avatar {
			width: 56px;
			height: 56px;
			border-radius: 50%;
			object-fit: cover;
			border: 2px solid var(--post-accent-blue);
		}

		.author-avatar-placeholder {
			width: 56px;
			height: 56px;
			border-radius: 50%;
			background: linear-gradient(135deg, var(--post-accent-blue), var(--post-accent-purple));
			display: flex;
			align-items: center;
			justify-content: center;
			font-size: 1.5rem;
			font-weight: 700;
			color: white;
		}

		.author-details {
			display: flex;
			flex-direction: column;
			gap: 0.25rem;
		}

		.author-name {
			font-size: 1.125rem;
			font-weight: 700;
			color: var(--post-text-primary);
		}

		.meta-row {
			display: flex;
			align-items: center;
			gap: 0.5rem;
			font-size: 0.875rem;
			color: var(--post-text-muted);
		}

		.separator {
			color: color-mix(in oklch, var(--post-text-muted) 50%, transparent);
		}

		.featured-image {
			width: 100%;
			height: 400px;
			border-radius: 16px;
			overflow: hidden;
			margin-bottom: 3rem;
			position: relative;
		}

		.featured-image::before {
			content: '';
			position: absolute;
			inset: 0;
			border-radius: 16px;
			padding: 2px;
			background: linear-gradient(135deg, var(--post-accent-blue), var(--post-accent-purple));
			-webkit-mask:
				linear-gradient(#fff 0 0) content-box,
				linear-gradient(#fff 0 0);
			mask:
				linear-gradient(#fff 0 0) content-box,
				linear-gradient(#fff 0 0);
			-webkit-mask-composite: xor;
			mask-composite: exclude;
			opacity: 0.3;
		}

		.featured-image :global(img) {
			width: 100%;
			height: 100%;
			object-fit: cover;
		}

		.post-body {
			background: var(--post-card-bg);
			backdrop-filter: blur(10px);
			border-radius: 16px;
			border: 1px solid var(--post-border);
			padding: 3rem;
			margin-bottom: 3rem;
		}

		.content {
			color: color-mix(in oklch, var(--post-text-primary) 90%, var(--post-text-secondary));
			font-size: 1.125rem;
			line-height: 1.8;
		}

		.content :global(h1),
		.content :global(h2),
		.content :global(h3),
		.content :global(h4),
		.content :global(h5),
		.content :global(h6) {
			color: var(--post-text-primary);
			font-weight: 700;
			margin-top: 2rem;
			margin-bottom: 1rem;
		}

		.content :global(h1) {
			font-size: 2.5rem;
		}
		.content :global(h2) {
			font-size: 2rem;
		}
		.content :global(h3) {
			font-size: 1.75rem;
		}
		.content :global(h4) {
			font-size: 1.5rem;
		}

		.content :global(p) {
			margin-bottom: 1.5rem;
		}

		.content :global(a) {
			color: var(--post-accent-blue);
			text-decoration: underline;
			transition: color 0.3s;
		}

		.content :global(a:hover) {
			color: color-mix(in oklch, var(--post-accent-blue) 80%, white);
		}

		.content :global(ul),
		.content :global(ol) {
			margin: 1.5rem 0;
			padding-left: 2rem;
		}

		.content :global(li) {
			margin-bottom: 0.75rem;
		}

		.content :global(blockquote) {
			border-left: 4px solid var(--post-accent-blue);
			padding-left: 1.5rem;
			margin: 2rem 0;
			font-style: italic;
			color: var(--post-text-secondary);
		}

		.content :global(code) {
			background: oklch(0.1 0.02 260 / 0.8);
			padding: 0.25rem 0.5rem;
			border-radius: 4px;
			font-family: 'Courier New', monospace;
			font-size: 0.9em;
			color: var(--post-accent-purple);
		}

		.content :global(pre) {
			background: oklch(0.1 0.02 260 / 0.8);
			padding: 1.5rem;
			border-radius: 8px;
			overflow-x: auto;
			margin: 2rem 0;
		}

		.content :global(pre code) {
			background: none;
			padding: 0;
		}

		.content :global(img) {
			max-width: 100%;
			height: auto;
			border-radius: 8px;
			margin: 2rem 0;
		}

		.content :global(figure) {
			margin: 2rem 0;
		}

		.content :global(figure img) {
			width: 100%;
			border-radius: 8px;
		}

		.content :global(figcaption) {
			text-align: center;
			font-size: 0.9rem;
			color: var(--post-text-muted);
			margin-top: 0.75rem;
		}

		.content :global(table) {
			width: 100%;
			border-collapse: collapse;
			margin: 2rem 0;
		}

		.content :global(th),
		.content :global(td) {
			padding: 0.75rem;
			border: 1px solid color-mix(in oklch, var(--post-text-muted) 20%, transparent);
		}

		.content :global(th) {
			background: color-mix(in oklch, var(--post-accent-blue) 10%, transparent);
			font-weight: 600;
		}

		.no-content {
			text-align: center;
			color: var(--post-text-muted);
			font-style: italic;
			padding: 2rem 0;
		}

		.tags-section {
			margin-top: 3rem;
			padding-top: 2rem;
			border-top: 1px solid var(--post-border);
		}

		.tags-section h3 {
			color: var(--post-text-primary);
			font-size: 1.25rem;
			font-weight: 700;
			margin-bottom: 1rem;
		}

		.tags {
			display: flex;
			flex-wrap: wrap;
			gap: 0.75rem;
		}

		.tag {
			padding: 0.5rem 1rem;
			background: color-mix(in oklch, var(--post-text-muted) 10%, transparent);
			color: var(--post-text-muted);
			font-size: 0.875rem;
			border-radius: 8px;
			border: 1px solid color-mix(in oklch, var(--post-text-muted) 20%, transparent);
			transition: all 0.3s;
		}

		.tag:hover {
			background: color-mix(in oklch, var(--post-accent-blue) 20%, transparent);
			border-color: var(--post-accent-blue);
			color: var(--post-accent-blue);
		}

		/* Share Section */
		.share-section {
			margin-top: 2rem;
			padding-top: 2rem;
			border-top: 1px solid var(--post-border);
		}

		/* Reading Time */
		.reading-time {
			color: var(--post-accent-blue);
			font-weight: 500;
		}

		.post-footer {
			text-align: center;
		}

		.loading,
		.error {
			max-width: 600px;
			margin: 4rem auto;
			text-align: center;
		}

		.loading {
			color: var(--post-text-muted);
		}

		.error {
			--error-color: oklch(0.65 0.2 25);
			color: var(--error-color);
		}

		.error h2 {
			font-size: 2rem;
			margin-bottom: 1rem;
		}

		.error button {
			margin-top: 2rem;
		}

		.spinner {
			width: 48px;
			height: 48px;
			border: 4px solid color-mix(in oklch, var(--post-text-muted) 20%, transparent);
			border-top-color: var(--post-accent-blue);
			border-radius: 50%;
			animation: spin 1s linear infinite;
			margin: 0 auto 1rem;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	/* Container query responsive adjustments */
	@container (max-width: 768px) {
		.blog-post-container {
			padding: 2rem 1rem;
		}

		.post-title {
			font-size: 2rem;
		}

		.post-lead {
			font-size: 1.0625rem;
		}

		.featured-image {
			height: 250px;
		}

		.post-body {
			padding: 2rem 1.5rem;
		}

		.content {
			font-size: 1rem;
		}

		.content :global(h1) {
			font-size: 1.75rem;
		}
		.content :global(h2) {
			font-size: 1.5rem;
		}
		.content :global(h3) {
			font-size: 1.25rem;
		}
	}

	/* Fallback media query for browsers without container query support */
	@supports not (container-type: inline-size) {
		@media (max-width: 768px) {
			.blog-post-container {
				padding: 2rem 1rem;
			}

			.post-title {
				font-size: 2rem;
			}

			.post-lead {
				font-size: 1.0625rem;
			}

			.featured-image {
				height: 250px;
			}

			.post-body {
				padding: 2rem 1.5rem;
			}

			.content {
				font-size: 1rem;
			}

			.content :global(h1) {
				font-size: 1.75rem;
			}
			.content :global(h2) {
				font-size: 1.5rem;
			}
			.content :global(h3) {
				font-size: 1.25rem;
			}
		}
	}
</style>

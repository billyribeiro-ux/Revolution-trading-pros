<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import TableOfContents from '$lib/components/blog/TableOfContents.svelte';
	import FloatingTocWidget from '$lib/components/blog/FloatingTocWidget.svelte';
	import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
	import type { Post } from '$lib/types/post';
	import { sanitizeBlogContent } from '$lib/utils/sanitize';

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
	}

	let post: BlogPost | null = null;
	let loading = true;
	let error = '';
	let slug = '';

	async function loadPost() {
		try {
			loading = true;
			error = '';

			const response = await apiFetch<BlogPost>(API_ENDPOINTS.posts.single(slug));
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

	$: {
		slug = $page.params.slug;
		if (slug) {
			loadPost();
		}
	}

	// Generate article schema if not provided by backend
	$: articleSchema =
		post &&
		(post.schema_markup || {
			'@context': 'https://schema.org',
			'@type': 'Article',
			headline: post.title,
			description: post.excerpt || post.meta_description || '',
			image: post.featured_image || 'https://revolutiontradingpros.com/revolution-trading-pros.png',
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
					url: 'https://revolutiontradingpros.com/revolution-trading-pros.png'
				}
			},
			mainEntityOfPage: {
				'@type': 'WebPage',
				'@id': `https://revolutiontradingpros.com/blog/${post.slug}`
			}
		});

	$: seoTitle = post && (post.meta_title || post.title);
	$: seoDescription = post && (post.meta_description || post.excerpt || '');
</script>

{#if post}
	<SEOHead
		title={seoTitle}
		description={seoDescription}
		canonical={`/blog/${post.slug}`}
		ogType="article"
		ogImage={post.featured_image}
		author={post.author?.name}
		publishedTime={post.published_at}
		schema={articleSchema}
	/>
{/if}

<div class="blog-post-container">
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading post...</p>
		</div>
	{:else if error}
		<div class="error">
			<h2>Oops!</h2>
			<p>{error}</p>
			<button class="btn-back" on:click={goBack}>
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
				>
					<polyline points="15 18 9 12 15 6"></polyline>
				</svg>
				Back to Blog
			</button>
		</div>
	{:else if post}
		<article class="blog-post">
			<div class="post-header">
				<button class="btn-back-header" on:click={goBack}>
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
							<img src={post.author_image} alt={post.author.name} class="author-avatar" />
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
					<img src={post.featured_image} alt={post.title} />
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
								<figure>
									<img src={block.data?.url || ''} alt={block.data?.caption || ''} />
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
			</div>

			<div class="post-footer">
				<button class="btn-back-footer" on:click={goBack}>
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

<style>
	.blog-post-container {
		min-height: 100vh;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		padding: 4rem 2rem;
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
		background: rgba(30, 41, 59, 0.8);
		backdrop-filter: blur(10px);
		color: #f1f5f9;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s;
		margin-bottom: 2rem;
	}

	.btn-back-header:hover,
	.btn-back:hover,
	.btn-back-footer:hover {
		background: rgba(96, 165, 250, 0.2);
		border-color: #60a5fa;
		transform: translateX(-4px);
	}

	.header-categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.category-badge {
		padding: 0.5rem 1rem;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
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
		color: #f1f5f9;
		line-height: 1.2;
		margin-bottom: 1rem;
	}

	.post-lead {
		font-size: 1.25rem;
		color: #cbd5e1;
		line-height: 1.6;
		margin-bottom: 2rem;
	}

	.post-meta {
		padding: 1.5rem;
		background: rgba(30, 41, 59, 0.5);
		backdrop-filter: blur(10px);
		border-radius: 12px;
		border: 1px solid rgba(148, 163, 184, 0.1);
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
		border: 2px solid #60a5fa;
	}

	.author-avatar-placeholder {
		width: 56px;
		height: 56px;
		border-radius: 50%;
		background: linear-gradient(135deg, #60a5fa, #a78bfa);
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
		color: #f1f5f9;
	}

	.meta-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #94a3b8;
	}

	.separator {
		color: #475569;
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
		background: linear-gradient(135deg, #60a5fa, #a78bfa);
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

	.featured-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.post-body {
		background: rgba(30, 41, 59, 0.5);
		backdrop-filter: blur(10px);
		border-radius: 16px;
		border: 1px solid rgba(148, 163, 184, 0.1);
		padding: 3rem;
		margin-bottom: 3rem;
	}

	.content {
		color: #e2e8f0;
		font-size: 1.125rem;
		line-height: 1.8;
	}

	.content :global(h1),
	.content :global(h2),
	.content :global(h3),
	.content :global(h4),
	.content :global(h5),
	.content :global(h6) {
		color: #f1f5f9;
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
		color: #60a5fa;
		text-decoration: underline;
		transition: color 0.3s;
	}

	.content :global(a:hover) {
		color: #93c5fd;
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
		border-left: 4px solid #60a5fa;
		padding-left: 1.5rem;
		margin: 2rem 0;
		font-style: italic;
		color: #cbd5e1;
	}

	.content :global(code) {
		background: rgba(15, 23, 42, 0.8);
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-family: 'Courier New', monospace;
		font-size: 0.9em;
		color: #a78bfa;
	}

	.content :global(pre) {
		background: rgba(15, 23, 42, 0.8);
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
		color: #94a3b8;
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
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.content :global(th) {
		background: rgba(96, 165, 250, 0.1);
		font-weight: 600;
	}

	.no-content {
		text-align: center;
		color: #94a3b8;
		font-style: italic;
		padding: 2rem 0;
	}

	.tags-section {
		margin-top: 3rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.tags-section h3 {
		color: #f1f5f9;
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
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		font-size: 0.875rem;
		border-radius: 8px;
		border: 1px solid rgba(148, 163, 184, 0.2);
		transition: all 0.3s;
	}

	.tag:hover {
		background: rgba(96, 165, 250, 0.2);
		border-color: #60a5fa;
		color: #60a5fa;
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
		color: #94a3b8;
	}

	.error {
		color: #f87171;
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
		border: 4px solid rgba(148, 163, 184, 0.2);
		border-top-color: #60a5fa;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 1rem;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

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
</style>

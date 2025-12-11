<script lang="ts">
	/**
	 * Blog List Page - Svelte 5 Runes Implementation
	 * @version 3.0.0 - December 2024
	 * Features: BlurHash images, Link prefetching, $state/$derived/$effect runes
	 */
	import { preloadData } from '$app/navigation';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import BlurHashImage from '$lib/components/ui/BlurHashImage.svelte';
	import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
	import type { Post } from '$lib/types/post';

	// ============================================================================
	// TypeScript Interfaces
	// ============================================================================

	interface Category {
		id: number;
		name: string;
		slug: string;
	}

	interface Tag {
		id: number;
		name: string;
		slug: string;
	}

	interface BlogPost extends Post {
		categories?: Category[];
		tags?: Tag[];
		blurhash?: string | null;
		featured_image_blurhash?: string | null;
	}

	interface PaginatedResponse {
		data: BlogPost[];
		current_page: number;
		last_page: number;
		per_page: number;
		total: number;
	}

	// ============================================================================
	// State Management - Svelte 5 Runes
	// ============================================================================

	let posts = $state<BlogPost[]>([]);
	let currentPage = $state(1);
	let lastPage = $state(1);
	let loading = $state(true);
	let error = $state('');
	let abortController = $state<AbortController | null>(null);
	let prefetchedSlugs = $state(new Set<string>());

	const MAX_RETRIES = 3;
	const RETRY_DELAY = 2000;

	// ============================================================================
	// Derived Values
	// ============================================================================

	let hasNextPage = $derived(currentPage < lastPage);
	let hasPrevPage = $derived(currentPage > 1);
	let showPagination = $derived(lastPage > 1);
	let isEmpty = $derived(!loading && !error && posts.length === 0);

	// ============================================================================
	// Performance & Error Handling
	// ============================================================================

	async function loadPosts(page: number = 1, retry = 0): Promise<void> {
		try {
			loading = true;
			error = '';

			// Cancel any existing requests
			if (abortController) {
				abortController.abort();
			}

			abortController = new AbortController();
			const timeoutId = setTimeout(() => abortController?.abort(), 10000);

			try {
				const response = await apiFetch<PaginatedResponse>(
					`${API_ENDPOINTS.posts.list}?page=${page}`
				);

				clearTimeout(timeoutId);

				if (!response.data || !Array.isArray(response.data)) {
					throw new Error('Invalid response format from server');
				}

				// Sanitize and validate post data
				posts = response.data.map((post) => ({
					...post,
					title: sanitizeText(post.title),
					excerpt: sanitizeText(post.excerpt || ''),
					categories: post.categories || [],
					tags: post.tags || []
				}));

				currentPage = response.current_page || 1;
				lastPage = response.last_page || 1;
			} catch (fetchError) {
				clearTimeout(timeoutId);

				if (fetchError instanceof Error && fetchError.name === 'AbortError') {
					if (retry < MAX_RETRIES) {
						const delay = RETRY_DELAY * Math.pow(2, retry);
						await new Promise((resolve) => setTimeout(resolve, delay));
						return loadPosts(page, retry + 1);
					} else {
						throw new Error('Request timed out. Please check your connection and try again.');
					}
				}

				throw fetchError;
			}
		} catch (err) {
			if (retry < MAX_RETRIES && err instanceof TypeError && err.message.includes('fetch')) {
				const delay = RETRY_DELAY * Math.pow(2, retry);
				await new Promise((resolve) => setTimeout(resolve, delay));
				return loadPosts(page, retry + 1);
			}

			error = err instanceof Error ? err.message : 'An unexpected error occurred';
			console.error('Error loading posts:', err);
		} finally {
			loading = false;
			abortController = null;
		}
	}

	function sanitizeText(text: string): string {
		if (typeof document === 'undefined') return text;
		const div = document.createElement('div');
		div.textContent = text;
		return div.innerHTML;
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'long',
			day: 'numeric'
		});
	}

	function nextPage() {
		if (hasNextPage) {
			loadPosts(currentPage + 1);
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function prevPage() {
		if (hasPrevPage) {
			loadPosts(currentPage - 1);
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function retryLoad() {
		loadPosts(currentPage);
	}

	// ============================================================================
	// Link Prefetching - Preload post data on hover
	// ============================================================================

	function prefetchPost(slug: string) {
		if (prefetchedSlugs.has(slug)) return;

		// Mark as prefetched to avoid duplicates
		prefetchedSlugs = new Set([...prefetchedSlugs, slug]);

		// Use SvelteKit's preloadData for route prefetching
		preloadData(`/blog/${slug}`).catch(() => {
			// Silently fail - prefetching is optional
		});
	}

	// ============================================================================
	// Lifecycle - Svelte 5 $effect
	// ============================================================================

	$effect(() => {
		loadPosts();

		return () => {
			// Cleanup: abort any pending requests
			if (abortController) {
				abortController.abort();
			}
		};
	});

	// Structured data for blog list
	const blogSchema = {
		'@context': 'https://schema.org',
		'@type': 'Blog',
		name: 'Revolution Trading Pros Blog',
		description: 'Expert trading insights, tutorials, and market analysis',
		url: 'https://revolutiontradingpros.com/blog',
		publisher: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros',
			logo: {
				'@type': 'ImageObject',
				url: 'https://revolutiontradingpros.com/revolution-trading-pros.png'
			}
		}
	};
</script>

<svelte:head>
	<!-- Prefetch hints for common blog posts -->
	{#each posts.slice(0, 3) as post}
		<link rel="prefetch" href="/blog/{post.slug}" />
	{/each}
</svelte:head>

<SEOHead
	title="Blog - Trading Insights & Tutorials"
	description="Expert trading insights, tutorials, and market analysis from Revolution Trading Pros. Learn day trading, swing trading, options strategies, and more."
	canonical="/blog"
	ogType="website"
	schema={blogSchema}
/>

<div class="blog-container">
	<div class="blog-header">
		<h1 class="blog-title">Blog</h1>
		<p class="blog-subtitle">Insights, tutorials, and updates from our team</p>
	</div>

	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading posts...</p>
		</div>
	{:else if error}
		<div class="error">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="error-icon"
			>
				<circle cx="12" cy="12" r="10"></circle>
				<line x1="12" y1="8" x2="12" y2="12"></line>
				<line x1="12" y1="16" x2="12.01" y2="16"></line>
			</svg>
			<h3>Oops! Something went wrong</h3>
			<p>{error}</p>
			<button class="btn-retry" onclick={retryLoad}>
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
					<polyline points="23 4 23 10 17 10"></polyline>
					<polyline points="1 20 1 14 7 14"></polyline>
					<path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
				</svg>
				Try Again
			</button>
		</div>
	{:else if isEmpty}
		<div class="empty">
			<p>No blog posts published yet.</p>
		</div>
	{:else}
		<div class="posts-grid">
			{#each posts as post (post.id)}
				<a
					href="/blog/{post.slug}"
					class="post-card"
					onmouseenter={() => prefetchPost(post.slug)}
					onfocus={() => prefetchPost(post.slug)}
				>
					<div class="post-image">
						{#if post.featured_image}
							<BlurHashImage
								src={post.featured_image}
								alt={post.title}
								blurhash={post.featured_image_blurhash || post.blurhash}
								width="100%"
								height="200px"
							/>
							<div class="image-overlay"></div>
						{:else}
							<div class="placeholder-content">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="48"
									height="48"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
									<circle cx="8.5" cy="8.5" r="1.5"></circle>
									<polyline points="21 15 16 10 5 21"></polyline>
								</svg>
							</div>
						{/if}
					</div>

					<div class="post-content">
						<div class="post-meta">
							{#if post.author}
								<span class="author">{post.author.name}</span>
								<span class="separator">•</span>
							{/if}
							<span class="date">{formatDate(post.published_at)}</span>
						</div>

						<h2 class="post-title">{post.title}</h2>

						{#if post.excerpt}
							<p class="post-excerpt">{post.excerpt}</p>
						{/if}

						{#if post.categories && post.categories.length > 0}
							<div class="categories">
								{#each post.categories as category (category.id)}
									<span class="category-tag">{category.name}</span>
								{/each}
							</div>
						{/if}

						{#if post.tags && post.tags.length > 0}
							<div class="tags">
								{#each post.tags as tag (tag.id)}
									<span class="tag">{tag.name}</span>
								{/each}
							</div>
						{/if}

						<div class="read-more-wrapper">
							<span class="read-more-btn">
								<span class="btn-text">Read More</span>
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="18"
									height="18"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
									class="arrow-icon"
								>
									<line x1="5" y1="12" x2="19" y2="12"></line>
									<polyline points="12 5 19 12 12 19"></polyline>
								</svg>
							</span>
						</div>
					</div>
				</a>
			{/each}
		</div>

		{#if showPagination}
			<nav class="pagination" aria-label="Blog pagination">
				<button
					class="btn-pagination"
					onclick={prevPage}
					disabled={!hasPrevPage}
					aria-label="Previous page"
				>
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
					Previous
				</button>

				<span class="page-info" aria-live="polite">
					Page {currentPage} of {lastPage}
				</span>

				<button
					class="btn-pagination"
					onclick={nextPage}
					disabled={!hasNextPage}
					aria-label="Next page"
				>
					Next
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
						<polyline points="9 18 15 12 9 6"></polyline>
					</svg>
				</button>
			</nav>
		{/if}
	{/if}
</div>

<style>
	.blog-container {
		min-height: 100vh;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		padding: 4rem 2rem;
	}

	.blog-header {
		max-width: 1200px;
		margin: 0 auto 4rem;
		text-align: center;
	}

	.blog-title {
		font-size: 3.5rem;
		font-weight: 800;
		background: linear-gradient(135deg, #60a5fa, #a78bfa, #ec4899);
		background-size: 200% 200%;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		animation: gradient-shift 4s ease infinite;
		margin-bottom: 1rem;
	}

	.blog-subtitle {
		font-size: 1.25rem;
		color: #94a3b8;
		font-weight: 300;
	}

	.posts-grid {
		max-width: 1200px;
		margin: 0 auto;
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		gap: 2rem;
	}

	.post-card {
		display: block;
		text-decoration: none;
		color: inherit;
		background: rgba(30, 41, 59, 0.5);
		backdrop-filter: blur(10px);
		border-radius: 16px;
		overflow: hidden;
		border: 1px solid rgba(148, 163, 184, 0.1);
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
	}

	.post-card::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: 16px;
		padding: 1px;
		background: linear-gradient(135deg, #60a5fa, #a78bfa);
		-webkit-mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		-webkit-mask-composite: xor;
		mask-composite: exclude;
		opacity: 0;
		transition: opacity 0.3s;
	}

	.post-card:hover {
		transform: translateY(-8px);
		box-shadow:
			0 20px 40px -12px rgba(96, 165, 250, 0.3),
			0 0 0 1px rgba(96, 165, 250, 0.1);
	}

	.post-card:hover::before {
		opacity: 0.6;
	}

	.post-image {
		width: 100%;
		height: 200px;
		position: relative;
		overflow: hidden;
		background: linear-gradient(135deg, #1e293b, #334155);
	}

	.post-card:hover .post-image :global(img) {
		transform: scale(1.05);
	}

	.image-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(to bottom, transparent 0%, rgba(15, 23, 42, 0.8) 100%);
		pointer-events: none;
	}

	.placeholder-content {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #475569;
	}

	.post-content {
		padding: 1.5rem;
	}

	.post-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: #94a3b8;
		margin-bottom: 1rem;
	}

	.separator {
		color: #475569;
	}

	.post-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
		line-height: 1.3;
	}

	.post-excerpt {
		font-size: 1rem;
		color: #cbd5e1;
		line-height: 1.6;
		margin-bottom: 1rem;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.category-tag {
		padding: 0.25rem 0.75rem;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
		font-size: 0.75rem;
		font-weight: 600;
		border-radius: 12px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.tag {
		padding: 0.25rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		font-size: 0.75rem;
		border-radius: 8px;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	/* Read More Button Styles */
	.read-more-wrapper {
		margin-top: 1.25rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.read-more-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, rgba(96, 165, 250, 0.1), rgba(167, 139, 250, 0.1));
		color: #60a5fa;
		font-size: 0.875rem;
		font-weight: 600;
		border-radius: 8px;
		border: 1px solid rgba(96, 165, 250, 0.3);
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
		overflow: hidden;
	}

	.read-more-btn::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #60a5fa, #a78bfa);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.read-more-btn .btn-text,
	.read-more-btn .arrow-icon {
		position: relative;
		z-index: 1;
	}

	.read-more-btn .arrow-icon {
		transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.post-card:hover .read-more-btn {
		border-color: transparent;
		color: white;
		transform: translateX(4px);
		box-shadow: 0 8px 20px -8px rgba(96, 165, 250, 0.5);
	}

	.post-card:hover .read-more-btn::before {
		opacity: 1;
	}

	.post-card:hover .read-more-btn .arrow-icon {
		transform: translateX(4px);
	}

	.pagination {
		max-width: 1200px;
		margin: 3rem auto 0;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 2rem;
	}

	.btn-pagination {
		display: flex;
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
	}

	.btn-pagination:hover:not(:disabled) {
		background: rgba(96, 165, 250, 0.2);
		border-color: #60a5fa;
		transform: translateY(-2px);
	}

	.btn-pagination:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-info {
		color: #94a3b8;
		font-weight: 600;
	}

	.loading,
	.error,
	.empty {
		max-width: 1200px;
		margin: 4rem auto;
		text-align: center;
		color: #94a3b8;
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

	.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 16px;
		padding: 3rem 2rem;
	}

	.error-icon {
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	.error h3 {
		color: #f87171;
		font-size: 1.5rem;
		font-weight: 700;
		margin-bottom: 1rem;
	}

	.error p {
		color: #fca5a5;
		font-size: 1.125rem;
		margin-bottom: 2rem;
	}

	.btn-retry {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #ef4444, #dc2626);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
	}

	.btn-retry:hover {
		background: linear-gradient(135deg, #dc2626, #b91c1c);
		transform: translateY(-2px);
		box-shadow: 0 8px 16px rgba(239, 68, 68, 0.3);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes gradient-shift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	@media (max-width: 768px) {
		.blog-container {
			padding: 2rem 1rem;
		}

		.blog-title {
			font-size: 2.5rem;
		}

		.posts-grid {
			grid-template-columns: 1fr;
			gap: 1.5rem;
		}

		.pagination {
			flex-direction: column;
			gap: 1rem;
		}
	}
</style>

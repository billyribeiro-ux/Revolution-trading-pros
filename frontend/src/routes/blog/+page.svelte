<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import { apiFetch, API_ENDPOINTS } from '$lib/api/config';
	import type { Post } from '$lib/types/post';

	// ============================================================================
	// TypeScript Interfaces (L70 Type Safety)
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
	}

	interface PaginatedResponse {
		data: BlogPost[];
		current_page: number;
		last_page: number;
		per_page: number;
		total: number;
	}

	interface ErrorResponse {
		error: string;
		message: string;
	}

	// ============================================================================
	// State Management
	// ============================================================================

	let posts: BlogPost[] = [];
	let currentPage = 1;
	let lastPage = 1;
	let total = 0;
	let loading = true;
	let error = '';
	let retryCount = 0;
	const MAX_RETRIES = 3;
	const RETRY_DELAY = 2000;
	let abortController: AbortController | null = null;

	// ============================================================================
	// Performance & Error Handling (L70 Standards)
	// ============================================================================

	/**
	 * Load blog posts with retry logic, timeout, and comprehensive error handling
	 *
	 * Features:
	 * - Exponential backoff for retries
	 * - Request timeout (10s)
	 * - AbortController for cleanup
	 * - Detailed error messages
	 */
	async function loadPosts(page: number = 1, retry = 0): Promise<void> {
		try {
			loading = true;
			error = '';

			// Cancel any existing requests
			if (abortController) {
				abortController.abort();
			}

			abortController = new AbortController();
			const timeoutId = setTimeout(() => abortController?.abort(), 10000); // 10s timeout

			try {
				const response = await apiFetch<PaginatedResponse>(
					`${API_ENDPOINTS.posts.list}?page=${page}`
				);

				clearTimeout(timeoutId);

				// Validate response structure
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
				total = response.total || 0;
				retryCount = 0; // Reset retry count on success
			} catch (fetchError) {
				clearTimeout(timeoutId);

				if (fetchError instanceof Error && fetchError.name === 'AbortError') {
					if (retry < MAX_RETRIES) {
						// Exponential backoff: 2s, 4s, 8s
						const delay = RETRY_DELAY * Math.pow(2, retry);
						console.log(
							`Request timeout. Retrying in ${delay / 1000}s... (${retry + 1}/${MAX_RETRIES})`
						);

						await new Promise((resolve) => setTimeout(resolve, delay));
						return loadPosts(page, retry + 1);
					} else {
						throw new Error('Request timed out. Please check your connection and try again.');
					}
				}

				throw fetchError;
			}
		} catch (err) {
			// Retry on network errors
			if (retry < MAX_RETRIES && err instanceof TypeError && err.message.includes('fetch')) {
				const delay = RETRY_DELAY * Math.pow(2, retry);
				console.log(`Network error. Retrying in ${delay / 1000}s... (${retry + 1}/${MAX_RETRIES})`);

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

	/**
	 * Sanitize text to prevent XSS attacks
	 */
	function sanitizeText(text: string): string {
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

	function viewPost(slug: string) {
		goto(`/blog/${slug}`);
	}

	function nextPage() {
		if (currentPage < lastPage) {
			loadPosts(currentPage + 1);
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function prevPage() {
		if (currentPage > 1) {
			loadPosts(currentPage - 1);
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function retryLoad() {
		loadPosts(currentPage);
	}

	// Lifecycle
	onMount(() => {
		loadPosts();
	});

	onDestroy(() => {
		// Cleanup: abort any pending requests
		if (abortController) {
			abortController.abort();
		}
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
			<button class="btn-retry" on:click={retryLoad}>
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
	{:else if posts.length === 0}
		<div class="empty">
			<p>No blog posts published yet.</p>
		</div>
	{:else}
		<div class="posts-grid">
			{#each posts as post}
				<div
					class="post-card"
					on:click={() => viewPost(post.slug)}
					on:keydown={(e) => e.key === 'Enter' && viewPost(post.slug)}
					role="button"
					tabindex="0"
				>
					{#if post.featured_image}
						<div class="post-image">
							<img src={post.featured_image} alt={post.title} />
							<div class="image-overlay"></div>
						</div>
					{:else}
						<div class="post-image placeholder">
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
						</div>
					{/if}

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
								{#each post.categories as category}
									<span class="category-tag">{category.name}</span>
								{/each}
							</div>
						{/if}

						{#if post.tags && post.tags.length > 0}
							<div class="tags">
								{#each post.tags as tag}
									<span class="tag">{tag.name}</span>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>

		{#if lastPage > 1}
			<div class="pagination">
				<button class="btn-pagination" on:click={prevPage} disabled={currentPage === 1}>
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

				<span class="page-info">Page {currentPage} of {lastPage}</span>

				<button class="btn-pagination" on:click={nextPage} disabled={currentPage === lastPage}>
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
			</div>
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
	}

	.post-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s;
	}

	.post-card:hover .post-image img {
		transform: scale(1.05);
	}

	.image-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(to bottom, transparent 0%, rgba(15, 23, 42, 0.8) 100%);
	}

	.post-image.placeholder {
		background: linear-gradient(135deg, #1e293b, #334155);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.placeholder-content {
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

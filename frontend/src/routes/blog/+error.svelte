<script lang="ts">
	/**
	 * Blog Error Page - Svelte 5 Implementation
	 *
	 * Provides blog-themed error handling with:
	 * - User-friendly error messages
	 * - Blog-specific navigation actions
	 * - Error tracking integration
	 * - Styled to match blog aesthetic
	 *
	 * @version 1.0.0 - January 2026
	 */
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Error details from SvelteKit
	let status = $derived(page.status);
	let errorMessage = $derived(page.error?.message || 'An unexpected error occurred');

	// Blog-specific error configuration
	interface ErrorConfig {
		title: string;
		description: string;
		suggestion: string;
	}

	const errorConfig: Record<number, ErrorConfig> = {
		404: {
			title: 'Post Not Found',
			description: "The blog post you're looking for doesn't exist or may have been moved.",
			suggestion: 'Try browsing our latest posts or use the search to find what you need.'
		},
		500: {
			title: 'Something Went Wrong',
			description: 'We encountered an error loading this content.',
			suggestion: 'Please try again in a moment. Our team has been notified.'
		},
		503: {
			title: 'Temporarily Unavailable',
			description: 'Our blog is undergoing maintenance.',
			suggestion: 'Please check back shortly. We appreciate your patience.'
		}
	};

	// Get config for current error or default
	let config = $derived(
		errorConfig[status] || {
			title: 'Error Loading Content',
			description: errorMessage,
			suggestion: 'Please try again or return to the blog.'
		}
	);

	// Track error for analytics
	$effect(() => {
		if (browser && status >= 400) {
			// Log error in development
			if (import.meta.env.DEV) {
				console.error(`[Blog Error ${status}]`, errorMessage, page.url.pathname);
			}

			// Send to analytics
			try {
				if (typeof window !== 'undefined' && 'gtag' in window) {
					(window as { gtag: (...args: unknown[]) => void }).gtag('event', 'exception', {
						description: `Blog ${status}: ${errorMessage}`,
						fatal: status >= 500,
						page_category: 'blog'
					});
				}
			} catch {
				// Silently fail analytics - non-critical
			}
		}
	});

	function handleRetry() {
		if (browser) {
			window.location.reload();
		}
	}

	function goToBlog() {
		goto('/blog');
	}

	function goBack() {
		if (browser) {
			window.history.back();
		}
	}
</script>

<SEOHead title="{config.title} - Blog" description={config.description} noindex />

<div class="blog-error-container">
	<div class="error-content">
		<!-- Error Status Badge -->
		<div class="error-badge">
			<span class="error-code">{status}</span>
		</div>

		<!-- Error Icon -->
		<div class="error-icon" aria-hidden="true">
			{#if status === 404}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="64"
					height="64"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<circle cx="11" cy="11" r="8"></circle>
					<path d="m21 21-4.3-4.3"></path>
					<path d="M8 8l6 6"></path>
				</svg>
			{:else if status >= 500}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="64"
					height="64"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path
						d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"
					></path>
				</svg>
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="64"
					height="64"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<circle cx="12" cy="12" r="10"></circle>
					<line x1="12" y1="8" x2="12" y2="12"></line>
					<line x1="12" y1="16" x2="12.01" y2="16"></line>
				</svg>
			{/if}
		</div>

		<!-- Error Content -->
		<h1 class="error-title">{config.title}</h1>
		<p class="error-description">{config.description}</p>
		<p class="error-suggestion">{config.suggestion}</p>

		<!-- Development Details -->
		{#if import.meta.env.DEV && errorMessage !== config.description}
			<details class="error-details">
				<summary>Technical Details (Dev Only)</summary>
				<pre>{errorMessage}</pre>
				<p class="error-path">Path: {page.url.pathname}</p>
			</details>
		{/if}

		<!-- Actions -->
		<div class="error-actions">
			{#if status === 404}
				<button type="button" class="btn btn-primary" onclick={goToBlog}>
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
						<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
					</svg>
					Browse All Posts
				</button>
				<button type="button" class="btn btn-secondary" onclick={goBack}>
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
					Go Back
				</button>
			{:else if status >= 500}
				<button type="button" class="btn btn-primary" onclick={handleRetry}>
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
				<button type="button" class="btn btn-secondary" onclick={goToBlog}>
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
						<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"></path>
					</svg>
					Browse Posts
				</button>
			{:else}
				<button type="button" class="btn btn-primary" onclick={goToBlog}> Return to Blog </button>
				<button type="button" class="btn btn-secondary" onclick={goBack}> Go Back </button>
			{/if}
		</div>

		<!-- Help Link -->
		<p class="error-help">
			Still having trouble?
			<a href="/support">Contact Support</a>
		</p>
	</div>
</div>

<style>
	.blog-error-container {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		padding: 3rem 2rem;
	}

	.error-content {
		max-width: 520px;
		text-align: center;
	}

	.error-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 1.5rem;
	}

	.error-code {
		display: inline-block;
		font-size: 4rem;
		font-weight: 800;
		background: linear-gradient(135deg, #60a5fa, #a78bfa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		line-height: 1;
	}

	.error-icon {
		color: #60a5fa;
		margin-bottom: 1.5rem;
		opacity: 0.8;
	}

	.error-title {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.error-description {
		font-size: 1.125rem;
		color: #cbd5e1;
		margin-bottom: 0.75rem;
		line-height: 1.6;
	}

	.error-suggestion {
		font-size: 1rem;
		color: #94a3b8;
		margin-bottom: 2rem;
		line-height: 1.6;
	}

	.error-details {
		margin-bottom: 2rem;
		text-align: left;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 0.75rem;
		padding: 1rem;
	}

	.error-details summary {
		color: #94a3b8;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 0.75rem;
		font-weight: 500;
	}

	.error-details pre {
		color: #f87171;
		font-size: 0.75rem;
		overflow-x: auto;
		white-space: pre-wrap;
		word-break: break-word;
		margin: 0 0 0.5rem 0;
		padding: 0.75rem;
		background: rgba(15, 23, 42, 0.5);
		border-radius: 0.5rem;
	}

	.error-path {
		font-size: 0.75rem;
		color: #64748b;
		margin: 0;
	}

	.error-actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
		margin-bottom: 2rem;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 1.5rem;
		font-size: 1rem;
		font-weight: 600;
		text-decoration: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #60a5fa, #a78bfa);
		color: #fff;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px rgba(96, 165, 250, 0.3);
	}

	.btn-secondary {
		background: rgba(30, 41, 59, 0.8);
		color: #f1f5f9;
		border: 1px solid rgba(148, 163, 184, 0.2);
	}

	.btn-secondary:hover {
		background: rgba(96, 165, 250, 0.2);
		border-color: #60a5fa;
	}

	.error-help {
		font-size: 0.875rem;
		color: #64748b;
	}

	.error-help a {
		color: #60a5fa;
		text-decoration: none;
		font-weight: 500;
	}

	.error-help a:hover {
		text-decoration: underline;
	}

	@media (max-width: 480px) {
		.blog-error-container {
			padding: 2rem 1rem;
		}

		.error-code {
			font-size: 3rem;
		}

		.error-title {
			font-size: 1.5rem;
		}

		.error-actions {
			flex-direction: column;
		}

		.btn {
			width: 100%;
		}
	}
</style>

<script lang="ts">
	/**
	 * Global Error Boundary - Enterprise L8 Pattern
	 *
	 * Provides graceful error handling with:
	 * - User-friendly error messages
	 * - Error code specific layouts
	 * - Recovery actions
	 * - Error reporting integration
	 *
	 * @version 1.0.0 - L8 Principal Engineer
	 */
	import { page } from '$app/stores';
	import { browser } from '$app/environment';
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Error details from SvelteKit
	let status = $derived($page.status);
	let message = $derived($page.error?.message || 'An unexpected error occurred');

	// Error configuration by status code
	const errorConfig: Record<number, { title: string; description: string; icon: string }> = {
		400: {
			title: 'Bad Request',
			description: 'The request could not be understood. Please check your input and try again.',
			icon: '⚠️'
		},
		401: {
			title: 'Unauthorized',
			description: 'You need to be logged in to access this page.',
			icon: '🔒'
		},
		403: {
			title: 'Access Denied',
			description: "You don't have permission to view this page.",
			icon: '🚫'
		},
		404: {
			title: 'Page Not Found',
			description: "The page you're looking for doesn't exist or has been moved.",
			icon: '🔍'
		},
		500: {
			title: 'Server Error',
			description: 'Something went wrong on our end. Our team has been notified.',
			icon: '🔧'
		},
		502: {
			title: 'Bad Gateway',
			description: 'Our servers are temporarily unavailable. Please try again shortly.',
			icon: '🌐'
		},
		503: {
			title: 'Service Unavailable',
			description: "We're performing maintenance. Please check back soon.",
			icon: '🛠️'
		}
	};

	// Get config for current error or default
	let config = $derived(
		errorConfig[status] || {
			title: 'Error',
			description: message,
			icon: '❌'
		}
	);

	// Track error for analytics (only in browser)
	$effect(() => {
		if (browser && status >= 400) {
			// Log error to console in development
			if (import.meta.env.DEV) {
				console.error(`[Error ${status}]`, message, $page.url.pathname);
			}

			// Send to analytics/error tracking service
			try {
				if (typeof window !== 'undefined' && 'gtag' in window) {
					(window as { gtag: (...args: unknown[]) => void }).gtag('event', 'exception', {
						description: `${status}: ${message}`,
						fatal: status >= 500
					});
				}
			} catch {
				// Silently fail analytics
			}
		}
	});

	function handleRetry() {
		if (browser) {
			window.location.reload();
		}
	}

	function handleGoBack() {
		if (browser) {
			window.history.back();
		}
	}
</script>

<SEOHead title="{config.title} - Revolution Trading Pros" description={config.description} noindex />

<main class="error-page">
	<div class="error-container">
		<!-- Error Icon -->
		<div class="error-icon" aria-hidden="true">
			{config.icon}
		</div>

		<!-- Error Status -->
		<div class="error-status">
			<span class="error-code">{status}</span>
		</div>

		<!-- Error Content -->
		<h1 class="error-title">{config.title}</h1>
		<p class="error-description">{config.description}</p>

		{#if import.meta.env.DEV && message !== config.description}
			<details class="error-details">
				<summary>Technical Details</summary>
				<pre>{message}</pre>
			</details>
		{/if}

		<!-- Actions -->
		<div class="error-actions">
			{#if status === 401}
				<a href="/login?redirect={encodeURIComponent($page.url.pathname)}" class="btn btn-primary">
					Sign In
				</a>
			{:else if status === 404}
				<a href="/" class="btn btn-primary"> Go Home </a>
				<button type="button" class="btn btn-secondary" onclick={handleGoBack}> Go Back </button>
			{:else if status >= 500}
				<button type="button" class="btn btn-primary" onclick={handleRetry}> Try Again </button>
				<a href="/" class="btn btn-secondary"> Go Home </a>
			{:else}
				<a href="/" class="btn btn-primary"> Go Home </a>
				<button type="button" class="btn btn-secondary" onclick={handleGoBack}> Go Back </button>
			{/if}
		</div>

		<!-- Support Link -->
		<p class="error-support">
			Need help?
			<a href="/dashboard/support">Contact Support</a>
		</p>
	</div>
</main>

<style>
	.error-page {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		padding: 2rem;
	}

	.error-container {
		max-width: 480px;
		text-align: center;
	}

	.error-icon {
		font-size: 4rem;
		margin-bottom: 1rem;
		filter: grayscale(0.2);
	}

	.error-status {
		margin-bottom: 1.5rem;
	}

	.error-code {
		display: inline-block;
		font-size: 5rem;
		font-weight: 800;
		background: linear-gradient(135deg, #f97316, #ea580c);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		line-height: 1;
	}

	.error-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: #fff;
		margin-bottom: 0.75rem;
	}

	.error-description {
		font-size: 1.125rem;
		color: #94a3b8;
		margin-bottom: 2rem;
		line-height: 1.6;
	}

	.error-details {
		margin-bottom: 2rem;
		text-align: left;
		background: #1e293b;
		border-radius: 0.5rem;
		padding: 1rem;
	}

	.error-details summary {
		color: #94a3b8;
		cursor: pointer;
		font-size: 0.875rem;
		margin-bottom: 0.5rem;
	}

	.error-details pre {
		color: #f87171;
		font-size: 0.75rem;
		overflow-x: auto;
		white-space: pre-wrap;
		word-break: break-word;
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
		padding: 0.875rem 1.75rem;
		font-size: 1rem;
		font-weight: 600;
		text-decoration: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-primary {
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: #fff;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 20px rgba(249, 115, 22, 0.3);
	}

	.btn-secondary {
		background: #334155;
		color: #fff;
	}

	.btn-secondary:hover {
		background: #475569;
	}

	.error-support {
		font-size: 0.875rem;
		color: #64748b;
	}

	.error-support a {
		color: #f97316;
		text-decoration: none;
	}

	.error-support a:hover {
		text-decoration: underline;
	}

	@media (max-width: 480px) {
		.error-code {
			font-size: 4rem;
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

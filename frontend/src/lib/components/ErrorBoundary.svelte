<script lang="ts">
	/**
	 * Error Boundary Component - Apple ICT9+ Error Resilience
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Catches and handles errors in child components with:
	 * - Graceful fallback UI
	 * - Error logging and reporting
	 * - Recovery options
	 * - User-friendly messaging
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { log } from '$lib/api/enterprise/tracing';
	import { serializeError } from '$lib/api/enterprise/errors';
	import { IconAlertTriangle, IconRefresh } from '$lib/icons';

	// Props
	interface Props {
		/** Fallback content when error occurs */
		fallback?: import('svelte').Snippet<[{ error: Error; reset: () => void }]>;
		/** Callback when error is caught */
		onError?: (error: Error, errorInfo: { componentStack?: string }) => void;
		/** Whether to show detailed error in development */
		showDetails?: boolean;
		/** Title shown in error UI */
		title?: string;
		/** Message shown in error UI */
		message?: string;
		/** Children content */
		children: import('svelte').Snippet;
	}

	let {
		fallback,
		onError,
		showDetails = import.meta.env.DEV,
		title = 'Something went wrong',
		message = "We're sorry, but something unexpected happened.",
		children
	}: Props = $props();

	// State
	let hasError = $state(false);
	let error = $state<Error | null>(null);
	let errorInfo = $state<{ componentStack?: string }>({});

	// Reset error state
	function reset() {
		hasError = false;
		error = null;
		errorInfo = {};
	}

	// Handle error
	function handleError(err: Error, info: { componentStack?: string } = {}) {
		hasError = true;
		error = err;
		errorInfo = info;

		// Log error
		log('error', `[ErrorBoundary] ${err.message}`, undefined, {
			error: serializeError(err),
			componentStack: info.componentStack
		});

		// Call error callback
		onError?.(err, info);
	}

	// Global error handler for uncaught errors
	onMount(() => {
		if (!browser) return;

		const handleGlobalError = (event: ErrorEvent) => {
			// Only catch errors from our components
			if (event.error instanceof Error) {
				handleError(event.error);
				event.preventDefault();
			}
		};

		const handleUnhandledRejection = (event: PromiseRejectionEvent) => {
			if (event.reason instanceof Error) {
				handleError(event.reason);
				event.preventDefault();
			}
		};

		window.addEventListener('error', handleGlobalError);
		window.addEventListener('unhandledrejection', handleUnhandledRejection);

		return () => {
			window.removeEventListener('error', handleGlobalError);
			window.removeEventListener('unhandledrejection', handleUnhandledRejection);
		};
	});
</script>

{#if hasError && error}
	{#if fallback}
		{@render fallback({ error, reset })}
	{:else}
		<div class="error-boundary">
			<div class="error-content">
				<div class="error-icon">
					<IconAlertTriangle size={48} stroke={1.5} />
				</div>
				<h2 class="error-title">{title}</h2>
				<p class="error-message">{message}</p>

				{#if showDetails && error}
					<details class="error-details">
						<summary>Technical Details</summary>
						<div class="error-stack">
							<p><strong>Error:</strong> {error.name}</p>
							<p><strong>Message:</strong> {error.message}</p>
							{#if error.stack}
								<pre>{error.stack}</pre>
							{/if}
							{#if errorInfo.componentStack}
								<p><strong>Component Stack:</strong></p>
								<pre>{errorInfo.componentStack}</pre>
							{/if}
						</div>
					</details>
				{/if}

				<div class="error-actions">
					<button class="btn-retry" onclick={reset}>
						<IconRefresh size={18} />
						Try Again
					</button>
					<button class="btn-home" onclick={() => (window.location.href = '/')}>
						Go to Home
					</button>
				</div>
			</div>
		</div>
	{/if}
{:else}
	{@render children()}
{/if}

<style>
	.error-boundary {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 300px;
		padding: 2rem;
	}

	.error-content {
		max-width: 500px;
		text-align: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 16px;
		padding: 2.5rem;
		backdrop-filter: blur(10px);
	}

	.error-icon {
		color: #f87171;
		margin-bottom: 1.5rem;
		display: flex;
		justify-content: center;
	}

	.error-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.75rem 0;
	}

	.error-message {
		color: #94a3b8;
		margin: 0 0 1.5rem 0;
		line-height: 1.6;
	}

	.error-details {
		text-align: left;
		margin-bottom: 1.5rem;
		background: rgba(15, 23, 42, 0.5);
		border-radius: 8px;
		padding: 1rem;
	}

	.error-details summary {
		cursor: pointer;
		color: #64748b;
		font-size: 0.875rem;
		margin-bottom: 0.5rem;
	}

	.error-details summary:hover {
		color: #94a3b8;
	}

	.error-stack {
		font-size: 0.75rem;
		color: #64748b;
		overflow-x: auto;
	}

	.error-stack p {
		margin: 0.25rem 0;
	}

	.error-stack pre {
		background: rgba(0, 0, 0, 0.3);
		padding: 0.75rem;
		border-radius: 4px;
		overflow-x: auto;
		font-size: 0.7rem;
		margin: 0.5rem 0;
		max-height: 200px;
		overflow-y: auto;
	}

	.error-actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.btn-retry {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: #0D1117;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-retry:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(230, 184, 0, 0.3);
	}

	.btn-home {
		padding: 0.75rem 1.5rem;
		background: rgba(148, 163, 184, 0.1);
		color: #94a3b8;
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-home:hover {
		background: rgba(148, 163, 184, 0.2);
		border-color: rgba(148, 163, 184, 0.3);
		color: #f1f5f9;
	}
</style>

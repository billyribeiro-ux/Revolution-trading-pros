<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ErrorBoundary Component - Explosive Swings Error Resilience
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Catches and handles errors in child components with:
	 * - Graceful fallback UI
	 * - Error logging
	 * - Recovery options
	 * - User-friendly messaging
	 *
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */

	import { browser } from '$app/environment';

	interface Props {
		/** Fallback content when error occurs */
		fallback?: import('svelte').Snippet<[Error]>;
		/** Callback when error is caught */
		onError?: (error: Error) => void;
		/** Children content */
		children: import('svelte').Snippet;
	}

	let { fallback, onError, children }: Props = $props();

	let error = $state<Error | null>(null);
	let hasError = $state(false);

	function handleError(e: Error) {
		error = e;
		hasError = true;
		onError?.(e);

		// Log to monitoring
		console.error('[ErrorBoundary]', e);
	}

	function reset() {
		error = null;
		hasError = false;
	}

	// Global error handler for uncaught errors within boundary
	$effect(() => {
		if (!browser) return;

		const handleGlobalError = (event: ErrorEvent) => {
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
		{@render fallback(error)}
	{:else}
		<div class="error-boundary" role="alert" aria-live="assertive">
			<div class="error-content">
				<div class="error-icon">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
						width="48"
						height="48"
						aria-hidden="true"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
						/>
					</svg>
				</div>
				<h3 class="error-title">Something went wrong</h3>
				<p class="error-message">{error.message}</p>
				<button type="button" class="retry-btn" onclick={reset}>
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="16"
						height="16"
						aria-hidden="true"
					>
						<path d="M1 4v6h6" />
						<path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10" />
					</svg>
					Try again
				</button>
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
		min-height: 200px;
		padding: var(--space-6);
	}

	.error-content {
		text-align: center;
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss-border);
		border-radius: var(--radius-xl);
		padding: var(--space-8);
		max-width: 400px;
	}

	.error-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: var(--color-bg-subtle);
		border-radius: var(--radius-xl);
		margin-bottom: var(--space-5);
		color: var(--color-loss);
	}

	.error-title {
		font-size: var(--text-lg);
		font-weight: var(--font-semibold);
		color: var(--color-loss);
		margin: 0 0 var(--space-2) 0;
	}

	.error-message {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		margin: 0 0 var(--space-5) 0;
		line-height: var(--leading-relaxed);
	}

	.retry-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-5);
		background: var(--color-brand-primary);
		color: white;
		border: none;
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		cursor: pointer;
		transition: var(--transition-colors);
	}

	.retry-btn:hover {
		background: var(--color-brand-primary-hover);
	}

	.retry-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary);
		outline-offset: 2px;
	}

	.retry-btn:active {
		transform: scale(0.98);
	}
</style>

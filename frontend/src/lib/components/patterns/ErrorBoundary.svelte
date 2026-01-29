<script lang="ts">
	/**
	 * ErrorBoundary Component
	 * Catches and displays errors gracefully
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		fallback?: Snippet<[Error]>;
		onError?: (error: Error) => void;
	}

	const { children, fallback, onError }: Props = $props();

	let error = $state<Error | null>(null);

	function handleError(e: unknown) {
		const err = e instanceof Error ? e : new Error(String(e));
		error = err;
		onError?.(err);
		console.error('[ErrorBoundary] Caught error:', err);
	}

	function reset() {
		error = null;
	}
</script>

<svelte:boundary onerror={handleError}>
	{#if error}
		{#if fallback}
			{@render fallback(error)}
		{:else}
			<div class="error-boundary">
				<div class="error-icon">⚠️</div>
				<h3 class="error-title">Something went wrong</h3>
				<p class="error-message">{error.message}</p>
				<button type="button" class="error-retry" onclick={reset}> Try again </button>
			</div>
		{/if}
	{:else}
		{@render children()}
	{/if}
</svelte:boundary>

<style>
	.error-boundary {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		text-align: center;
		background: var(--color-bg-error, #fef2f2);
		border: 1px solid var(--color-border-error, #fecaca);
		border-radius: var(--radius-lg, 0.5rem);
	}

	.error-icon {
		font-size: 2rem;
		margin-bottom: 1rem;
	}

	.error-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--color-text-error, #dc2626);
		margin: 0 0 0.5rem;
	}

	.error-message {
		font-size: 0.875rem;
		color: var(--color-text-secondary, #6b7280);
		margin: 0 0 1rem;
	}

	.error-retry {
		padding: 0.5rem 1rem;
		background: var(--color-primary, #6366f1);
		color: white;
		border: none;
		border-radius: var(--radius-md, 0.375rem);
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s;
	}

	.error-retry:hover {
		background: var(--color-primary-hover, #4f46e5);
	}
</style>

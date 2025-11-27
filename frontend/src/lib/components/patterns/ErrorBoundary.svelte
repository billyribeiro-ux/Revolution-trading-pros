<script lang="ts">
	/**
	 * ErrorBoundary - Catches and displays errors gracefully
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import { IconAlertTriangle, IconRefresh } from '@tabler/icons-svelte';
	import { createEventDispatcher } from 'svelte';

	export let error: Error | null = null;
	export let showDetails = false;
	export let onRetry: (() => void) | null = null;

	const dispatch = createEventDispatcher();

	function handleRetry() {
		error = null;
		if (onRetry) {
			onRetry();
		}
		dispatch('retry');
	}

	function handleReport() {
		// In production, this would send to error tracking service
		console.error('Error reported:', error);
		dispatch('report', { error });
	}
</script>

{#if error}
	<div class="error-boundary">
		<div class="error-content">
			<div class="error-icon">
				<IconAlertTriangle size={48} />
			</div>
			
			<h2 class="error-title">Something went wrong</h2>
			
			<p class="error-message">
				{error.message || 'An unexpected error occurred'}
			</p>
			
			{#if showDetails && error.stack}
				<details class="error-details">
					<summary>Technical Details</summary>
					<pre>{error.stack}</pre>
				</details>
			{/if}
			
			<div class="error-actions">
				{#if onRetry}
					<button class="btn-primary" on:click={handleRetry}>
						<IconRefresh size={18} />
						Try Again
					</button>
				{/if}
				<button class="btn-secondary" on:click={handleReport}>
					Report Issue
				</button>
			</div>
		</div>
	</div>
{:else}
	<slot />
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
		text-align: center;
		max-width: 500px;
	}

	.error-icon {
		color: var(--color-rtp-error, #ef4444);
		margin-bottom: 1.5rem;
	}

	.error-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-rtp-text, #f1f5f9);
		margin: 0 0 0.75rem 0;
	}

	.error-message {
		font-size: 1rem;
		color: var(--color-rtp-muted, #94a3b8);
		margin: 0 0 1.5rem 0;
	}

	.error-details {
		text-align: left;
		margin-bottom: 1.5rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: var(--radius-md, 0.5rem);
		padding: 1rem;
	}

	.error-details summary {
		cursor: pointer;
		font-weight: 600;
		color: var(--color-rtp-error, #f87171);
		margin-bottom: 0.5rem;
	}

	.error-details pre {
		font-size: 0.75rem;
		color: var(--color-rtp-muted, #94a3b8);
		overflow-x: auto;
		white-space: pre-wrap;
		word-break: break-all;
		margin: 0;
	}

	.error-actions {
		display: flex;
		gap: 0.75rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, var(--color-rtp-primary, #6366f1), var(--color-rtp-indigo, #8b5cf6));
		color: white;
		border: none;
		border-radius: var(--radius-md, 0.5rem);
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
	}

	.btn-secondary {
		padding: 0.75rem 1.5rem;
		background: transparent;
		color: var(--color-rtp-muted, #94a3b8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: var(--radius-md, 0.5rem);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: rgba(99, 102, 241, 0.1);
		color: var(--color-rtp-text, #f1f5f9);
	}
</style>

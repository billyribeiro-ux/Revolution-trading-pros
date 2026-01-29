<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ApiErrorDisplay Component - API Error State Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Displays API errors with:
	 * - User-friendly messages
	 * - Error codes (in dev mode)
	 * - Retry functionality
	 * - Compact and full variants
	 *
	 * @version 1.0.0
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */

	import { isApiError, getUserFriendlyMessage } from '$lib/api/enterprise/errors';
	import type { EnterpriseApiError } from '$lib/api/enterprise/types';

	interface Props {
		error: EnterpriseApiError | Error | string | null;
		onRetry?: () => void;
		compact?: boolean;
	}

	const { error, onRetry, compact = false }: Props = $props();

	const errorMessage = $derived.by(() => {
		if (!error) return null;
		if (typeof error === 'string') return error;
		return getUserFriendlyMessage(error);
	});

	const errorCode = $derived.by(() => {
		if (!error || typeof error === 'string') return null;
		if (isApiError(error)) return error.code;
		return null;
	});

	const isDev = $derived(import.meta.env.DEV);
</script>

{#if error}
	<div class="api-error" class:compact role="alert" aria-live="polite">
		<div class="error-content">
			<svg
				class="error-icon"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				width={compact ? 16 : 20}
				height={compact ? 16 : 20}
				aria-hidden="true"
			>
				<circle cx="12" cy="12" r="10" />
				<line x1="12" y1="8" x2="12" y2="12" />
				<line x1="12" y1="16" x2="12.01" y2="16" />
			</svg>

			<div class="error-body">
				<span class="error-message">{errorMessage}</span>
				{#if isDev && errorCode}
					<span class="error-code">({errorCode})</span>
				{/if}
			</div>
		</div>

		{#if onRetry}
			<button type="button" class="retry-btn" onclick={onRetry} aria-label="Retry loading">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					width="14"
					height="14"
					aria-hidden="true"
				>
					<path d="M1 4v6h6" />
					<path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10" />
				</svg>
				Retry
			</button>
		{/if}
	</div>
{/if}

<style>
	.api-error {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
		padding: var(--space-4);
		background: var(--color-loss-bg);
		border: 1px solid var(--color-loss-border);
		border-radius: var(--radius-lg);
	}

	.api-error.compact {
		padding: var(--space-3);
		border-radius: var(--radius-md);
	}

	.error-content {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		flex: 1;
		min-width: 0;
	}

	.error-icon {
		flex-shrink: 0;
		color: var(--color-loss);
		margin-top: 2px;
	}

	.error-body {
		display: flex;
		flex-wrap: wrap;
		align-items: baseline;
		gap: var(--space-2);
		min-width: 0;
	}

	.error-message {
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		color: var(--color-loss);
		line-height: var(--leading-snug);
	}

	.compact .error-message {
		font-size: var(--text-xs);
	}

	.error-code {
		font-size: var(--text-xs);
		font-family: var(--font-mono);
		color: var(--color-text-tertiary);
		background: var(--color-bg-subtle);
		padding: 2px 6px;
		border-radius: var(--radius-sm);
	}

	.retry-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: var(--space-1) var(--space-3);
		background: transparent;
		border: 1px solid var(--color-loss-border);
		border-radius: var(--radius-sm);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		color: var(--color-loss);
		cursor: pointer;
		transition: var(--transition-colors);
		flex-shrink: 0;
	}

	.retry-btn:hover {
		background: var(--color-loss-border);
	}

	.retry-btn:focus-visible {
		outline: 2px solid var(--color-loss);
		outline-offset: 2px;
	}

	.retry-btn:active {
		transform: scale(0.98);
	}

	/* Responsive */
	@media (max-width: 480px) {
		.api-error {
			flex-direction: column;
			align-items: stretch;
			gap: var(--space-3);
		}

		.retry-btn {
			justify-content: center;
		}
	}
</style>

<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ErrorBanner Component - Error State Display
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Displays error messages with optional retry functionality
	 * @version 1.0.0 - Nuclear Refactor
	 * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
	 */

	interface Props {
		message: string;
		onRetry?: () => void;
		variant?: 'error' | 'warning' | 'info';
	}

	const { message, onRetry, variant = 'error' }: Props = $props();

	const variantConfig = $derived.by(() => {
		switch (variant) {
			case 'warning':
				return {
					bgColor: 'var(--color-watching-bg)',
					borderColor: 'var(--color-watching-border)',
					textColor: 'var(--color-watching-text)',
					iconColor: 'var(--color-watching)'
				};
			case 'info':
				return {
					bgColor: 'var(--color-update-bg)',
					borderColor: 'var(--color-update-border)',
					textColor: 'var(--color-update-text)',
					iconColor: 'var(--color-update)'
				};
			case 'error':
			default:
				return {
					bgColor: 'var(--color-loss-bg)',
					borderColor: 'var(--color-loss-border)',
					textColor: 'var(--color-loss)',
					iconColor: 'var(--color-loss)'
				};
		}
	});
</script>

<div
	class="error-banner"
	role="alert"
	aria-live="polite"
	style="
		--banner-bg: {variantConfig.bgColor};
		--banner-border: {variantConfig.borderColor};
		--banner-text: {variantConfig.textColor};
		--banner-icon: {variantConfig.iconColor};
	"
>
	<div class="banner-content">
		<svg
			class="banner-icon"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			width="20"
			height="20"
			aria-hidden="true"
		>
			{#if variant === 'error'}
				<circle cx="12" cy="12" r="10" />
				<line x1="12" y1="8" x2="12" y2="12" />
				<line x1="12" y1="16" x2="12.01" y2="16" />
			{:else if variant === 'warning'}
				<path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" />
				<line x1="12" y1="9" x2="12" y2="13" />
				<line x1="12" y1="17" x2="12.01" y2="17" />
			{:else}
				<circle cx="12" cy="12" r="10" />
				<line x1="12" y1="16" x2="12" y2="12" />
				<line x1="12" y1="8" x2="12.01" y2="8" />
			{/if}
		</svg>

		<span class="banner-message">{message}</span>
	</div>

	{#if onRetry}
		<button type="button" class="retry-btn" onclick={onRetry} aria-label="Retry loading">
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
			Retry
		</button>
	{/if}
</div>

<style>
	.error-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
		padding: var(--space-3) var(--space-4);
		background: var(--banner-bg);
		border: 1px solid var(--banner-border);
		border-radius: var(--radius-md);
		margin-bottom: var(--space-4);
	}

	.banner-content {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		flex: 1;
		min-width: 0;
	}

	.banner-icon {
		flex-shrink: 0;
		color: var(--banner-icon);
	}

	.banner-message {
		font-size: var(--text-sm);
		font-weight: var(--font-medium);
		color: var(--banner-text);
		line-height: var(--leading-snug);
	}

	.retry-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: var(--space-1) var(--space-3);
		background: transparent;
		border: 1px solid var(--banner-border);
		border-radius: var(--radius-sm);
		font-size: var(--text-sm);
		font-weight: var(--font-semibold);
		color: var(--banner-text);
		cursor: pointer;
		transition: var(--transition-colors);
		flex-shrink: 0;
	}

	.retry-btn:hover {
		background: var(--banner-border);
	}

	.retry-btn:focus-visible {
		outline: 2px solid var(--banner-icon);
		outline-offset: 2px;
	}

	.retry-btn:active {
		transform: scale(0.98);
	}

	/* Responsive */
	@media (max-width: 480px) {
		.error-banner {
			flex-direction: column;
			align-items: stretch;
			gap: var(--space-3);
		}

		.retry-btn {
			justify-content: center;
		}
	}
</style>

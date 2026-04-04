<script lang="ts">
	/**
	 * Toast - Accessible Notification Component
	 * ==========================================
	 * WCAG 2.1 AA compliant toast notifications with:
	 * - aria-live region for screen reader announcements
	 * - Reduced motion support
	 * - Loading state support
	 * - Proper focus management
	 *
	 * @version 2.0.0 - Accessibility Enhanced
	 * @accessibility WCAG 2.1 AA compliant
	 */
	import { toasts, removeToast, type ToastType } from '$lib/stores/toast.svelte';
	import { Icon, IconAlertCircle, IconCheck, IconInfoCircle, IconLoader, IconX } from '$lib/icons';

	// Local derived from getter
	const toastList = $derived(toasts.value);

	const icons: Record<ToastType, string> = {
		success: IconCheck,
		error: IconX,
		warning: IconAlertCircle,
		info: IconInfoCircle,
		loading: IconLoader
	};

	// Determine aria-live politeness based on toast type
	function getAriaLive(type: ToastType): 'polite' | 'assertive' {
		// Errors should be assertive (announced immediately)
		if (type === 'error') return 'assertive';
		// Everything else is polite (announced when convenient)
		return 'polite';
	}
</script>

<!-- Screen reader live region - announces new toasts -->
<div class="toast-container" role="region" aria-label="Notifications">
	{#each toastList as toast (toast.id)}
		{@const iconStr = icons[toast.type]}
		<div
			class="toast-item"
			data-type={toast.type}
			role="status"
			aria-live={getAriaLive(toast.type)}
			aria-atomic="true"
		>
			<!-- Icon -->
			<span
				class="toast-icon"
				class:toast-icon-spin={toast.type === 'loading'}
				aria-hidden="true"
			>
				<Icon icon={iconStr} size={20} />
			</span>

			<!-- Message -->
			<p class="toast-message">{toast.message}</p>

			<!-- Dismiss button (only if dismissible) -->
			{#if toast.dismissible}
				<button
					type="button"
					onclick={() => removeToast(toast.id)}
					class="toast-dismiss"
					aria-label="Dismiss notification"
				>
					<Icon icon={IconX} size={16} aria-hidden="true" />
				</button>
			{/if}
		</div>
	{/each}
</div>

<style>
	.toast-container {
		position: fixed;
		inset-block-start: var(--space-4);
		inset-inline-end: var(--space-4);
		z-index: 50;
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.toast-item {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-inline: var(--space-4);
		padding-block: var(--space-3);
		border-radius: var(--radius-lg);
		border-inline-start: 4px solid;
		box-shadow: var(--shadow-lg);
		min-inline-size: 300px;
		max-inline-size: 28rem;

		&[data-type='success'] {
			background-color: oklch(0.97 0.03 145);
			border-color: oklch(0.6 0.18 150);
			color: oklch(0.35 0.1 150);
		}

		&[data-type='error'] {
			background-color: oklch(0.97 0.02 25);
			border-color: oklch(0.58 0.24 27);
			color: oklch(0.4 0.15 25);
		}

		&[data-type='warning'] {
			background-color: oklch(0.97 0.04 90);
			border-color: oklch(0.75 0.18 85);
			color: oklch(0.4 0.1 80);
		}

		&[data-type='info'] {
			background-color: oklch(0.97 0.02 250);
			border-color: oklch(0.55 0.2 260);
			color: oklch(0.4 0.12 250);
		}

		&[data-type='loading'] {
			background-color: oklch(0.97 0.002 265);
			border-color: oklch(0.65 0.01 265);
			color: oklch(0.4 0.01 265);
		}
	}

	.toast-icon {
		flex-shrink: 0;
	}

	.toast-message {
		flex: 1;
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
	}

	.toast-dismiss {
		flex-shrink: 0;
		color: oklch(0.55 0.01 265);
		border-radius: var(--radius-sm);
		padding: var(--space-1);
		transition: color var(--duration-fast) var(--ease-default);

		&:hover { color: oklch(0.35 0.01 265); }

		&:focus {
			outline: none;
			box-shadow: 0 0 0 2px oklch(0.55 0.01 265 / 50%);
		}
	}

	@media (prefers-reduced-motion: no-preference) {
		.toast-item {
			animation: toast-slide-in 0.3s ease-out;
		}

		.toast-icon-spin {
			animation: spin 1s linear infinite;
		}
	}

	@keyframes toast-slide-in {
		from { transform: translateX(100%); opacity: 0; }
		to { transform: translateX(0); opacity: 1; }
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>

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
	import {
		IconCheck,
		IconX,
		IconAlertCircle,
		IconInfoCircle,
		IconLoader
	} from '$lib/icons';
	import { toasts, removeToast, type ToastType } from '$lib/stores/toast';

	const icons: Record<ToastType, typeof IconCheck> = {
		success: IconCheck,
		error: IconX,
		warning: IconAlertCircle,
		info: IconInfoCircle,
		loading: IconLoader
	};

	const colors: Record<ToastType, string> = {
		success: 'bg-green-50 border-green-500 text-green-800',
		error: 'bg-red-50 border-red-500 text-red-800',
		warning: 'bg-yellow-50 border-yellow-500 text-yellow-800',
		info: 'bg-blue-50 border-blue-500 text-blue-800',
		loading: 'bg-gray-50 border-gray-400 text-gray-700'
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
<div
	class="fixed top-4 right-4 z-50 space-y-2"
	role="region"
	aria-label="Notifications"
>
	{#each $toasts as toast (toast.id)}
		{@const IconComponent = icons[toast.type]}
		<div
			class="flex items-center gap-3 px-4 py-3 rounded-lg border-l-4 shadow-lg
				min-w-[300px] max-w-md motion-safe:animate-slide-in motion-reduce:opacity-100
				{colors[toast.type]}"
			role="status"
			aria-live={getAriaLive(toast.type)}
			aria-atomic="true"
		>
			<!-- Icon -->
			<span
				class="flex-shrink-0"
				class:motion-safe:animate-spin={toast.type === 'loading'}
				aria-hidden="true"
			>
				<IconComponent size={20} />
			</span>

			<!-- Message -->
			<p class="flex-1 text-sm font-medium">{toast.message}</p>

			<!-- Dismiss button (only if dismissible) -->
			{#if toast.dismissible}
				<button
					type="button"
					onclick={() => removeToast(toast.id)}
					class="flex-shrink-0 text-gray-500 hover:text-gray-700 transition-colors
						rounded p-1 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
					aria-label="Dismiss notification"
				>
					<IconX size={16} aria-hidden="true" />
				</button>
			{/if}
		</div>
	{/each}
</div>

<style>
	@keyframes slide-in {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}

	.motion-safe\:animate-slide-in {
		animation: slide-in 0.3s ease-out;
	}

	/* Respect reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.motion-safe\:animate-slide-in {
			animation: none;
		}

		.motion-safe\:animate-spin {
			animation: none;
		}
	}
</style>

<script lang="ts">
	/**
	 * Modal - Accessible Dialog Component
	 * ====================================
	 * WCAG 2.1 AA compliant modal dialog with:
	 * - Focus trap (keyboard navigation stays within modal)
	 * - Return focus to trigger element on close
	 * - Proper ARIA attributes
	 * - Escape key handling
	 * - Backdrop click to close
	 *
	 * @version 2.0.0 - Accessibility Enhanced
	 * @accessibility WCAG 2.1 AA compliant
	 */
	import type { Snippet } from 'svelte';
	import { IconX } from '$lib/icons';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	interface Props {
		open?: boolean;
		title?: string;
		size?: 'sm' | 'md' | 'lg' | 'xl' | 'fullscreen';
		onclose?: () => void;
		children?: Snippet;
		footer?: Snippet;
		closeOnBackdrop?: boolean;
		closeOnEscape?: boolean;
		initialFocusSelector?: string;
	}

	let {
		open = $bindable(false),
		title = '',
		size = 'md',
		onclose,
		children,
		footer,
		closeOnBackdrop = true,
		closeOnEscape = true,
		initialFocusSelector
	}: Props = $props();

	// Generate unique ID for ARIA attributes
	const modalId = `modal-${Math.random().toString(36).substring(2, 9)}`;
	const titleId = `${modalId}-title`;
	const descId = `${modalId}-desc`;

	const sizes: Record<string, string> = {
		sm: 'modal-size-sm',
		md: 'modal-size-md',
		lg: 'modal-size-lg',
		xl: 'modal-size-xl',
		fullscreen: 'modal-size-fullscreen'
	};

	// Store the element that triggered the modal
	let modalElement: HTMLDivElement | null = $state(null);
	let previouslyFocused: Element | null = $state(null);

	// Focusable elements selector
	const FOCUSABLE_SELECTOR =
		'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])';

	function close() {
		open = false;
		onclose?.();

		// Return focus to trigger element
		if (browser && previouslyFocused && previouslyFocused instanceof HTMLElement) {
			previouslyFocused.focus();
		}
	}

	function handleBackdropClick(e: MouseEvent) {
		if (closeOnBackdrop && e.target === e.currentTarget) {
			close();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (!open) return;

		if (e.key === 'Escape' && closeOnEscape) {
			e.preventDefault();
			close();
			return;
		}

		// Focus trap - Tab and Shift+Tab
		if (e.key === 'Tab' && modalElement) {
			const focusableElements = modalElement.querySelectorAll(FOCUSABLE_SELECTOR);
			const focusable = Array.from(focusableElements) as HTMLElement[];

			if (focusable.length === 0) return;

			const firstFocusable = focusable[0];
			const lastFocusable = focusable[focusable.length - 1];

			if (e.shiftKey) {
				// Shift + Tab
				if (document.activeElement === firstFocusable) {
					e.preventDefault();
					lastFocusable?.focus();
				}
			} else {
				// Tab
				if (document.activeElement === lastFocusable) {
					e.preventDefault();
					firstFocusable?.focus();
				}
			}
		}
	}

	// Handle focus when modal opens
	$effect(() => {
		if (open && browser) {
			// Store currently focused element to return focus later
			previouslyFocused = document.activeElement;

			// Prevent body scroll
			document.body.style.overflow = 'hidden';

			// Focus management - wait for DOM update
			requestAnimationFrame(() => {
				if (modalElement) {
					// Try initial focus selector first
					if (initialFocusSelector) {
						const initialElement = modalElement.querySelector(initialFocusSelector) as HTMLElement;
						if (initialElement) {
							initialElement.focus();
							return;
						}
					}

					// Otherwise focus first focusable element, or the modal itself
					const focusableElements = modalElement.querySelectorAll(FOCUSABLE_SELECTOR);
					if (focusableElements.length > 0) {
						(focusableElements[0] as HTMLElement).focus();
					} else {
						modalElement.focus();
					}
				}
			});
		} else if (!open && browser) {
			// Restore body scroll
			document.body.style.overflow = '';
		}
	});

	// Cleanup on unmount
	onMount(() => {
		return () => {
			if (browser) {
				document.body.style.overflow = '';
			}
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!-- Modal Backdrop - 2026 Mobile-First Responsive -->
	<div class="modal-backdrop-2026" onclick={handleBackdropClick} role="presentation">
		<!-- Backdrop overlay -->
		<div class="modal-overlay-2026" aria-hidden="true"></div>

		<!-- Swipe indicator for mobile -->
		<div class="modal-swipe-indicator" aria-hidden="true"></div>

		<!-- Modal Panel -->
		<div
			bind:this={modalElement}
			class="modal-panel-2026 {sizes[size]}"
			role="dialog"
			aria-modal="true"
			aria-labelledby={title ? titleId : undefined}
			aria-describedby={descId}
			tabindex="-1"
		>
			<!-- Header - Sticky -->
			{#if title}
				<div class="modal-header-2026">
					<h2 id={titleId} class="modal-title-2026">
						{title}
					</h2>
					<button
						type="button"
						onclick={close}
						class="modal-close-2026"
						aria-label="Close dialog"
					>
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>
			{:else}
				<!-- Close button for title-less modals -->
				<button
					type="button"
					onclick={close}
					class="modal-close-absolute-2026"
					aria-label="Close dialog"
				>
					<IconX size={20} aria-hidden="true" />
				</button>
			{/if}

			<!-- Body - Scrollable -->
			<div id={descId} class="modal-body-2026">
				{@render children?.()}
			</div>

			<!-- Footer - Sticky -->
			{#if footer}
				<div class="modal-footer-2026">
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 RESPONSIVE MODAL - Mobile-First Design
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.modal-backdrop-2026 {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: flex-end;
		justify-content: center;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
	}

	.modal-overlay-2026 {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		transition: opacity 0.2s ease;
	}

	.modal-swipe-indicator {
		position: fixed;
		top: 8px;
		left: 50%;
		transform: translateX(-50%);
		width: 36px;
		height: 4px;
		background: rgba(255, 255, 255, 0.4);
		border-radius: 2px;
		z-index: 60;
	}

	/* Modal Panel - Mobile: Full screen bottom sheet */
	.modal-panel-2026 {
		position: fixed;
		inset: 0;
		z-index: 51;
		display: flex;
		flex-direction: column;
		background: white;
		border-radius: 0;
		max-height: 100dvh;
		overflow: hidden;
		padding-top: env(safe-area-inset-top, 0);
		padding-bottom: env(safe-area-inset-bottom, 0);
	}

	.modal-panel-2026:focus {
		outline: none;
	}

	/* Size variants - Mobile */
	.modal-size-sm,
	.modal-size-md,
	.modal-size-lg,
	.modal-size-xl {
		width: 100%;
	}

	.modal-size-fullscreen {
		width: 100%;
		height: 100%;
		margin: 0;
		border-radius: 0;
	}

	/* Sticky Header */
	.modal-header-2026 {
		position: sticky;
		top: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		padding-top: calc(1rem + env(safe-area-inset-top, 0));
		border-bottom: 1px solid #e5e7eb;
		background: rgba(255, 255, 255, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		flex-shrink: 0;
	}

	.modal-title-2026 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #111827;
		margin: 0;
	}

	/* Touch target: 44x44px minimum */
	.modal-close-2026,
	.modal-close-absolute-2026 {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 44px;
		min-height: 44px;
		width: 44px;
		height: 44px;
		color: #6b7280;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.15s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.modal-close-2026:hover,
	.modal-close-absolute-2026:hover {
		color: #374151;
		background: #f3f4f6;
	}

	.modal-close-2026:focus-visible,
	.modal-close-absolute-2026:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.modal-close-absolute-2026 {
		position: absolute;
		top: calc(1rem + env(safe-area-inset-top, 0));
		right: 1rem;
		z-index: 10;
	}

	/* Scrollable Body */
	.modal-body-2026 {
		flex: 1;
		padding: 1rem;
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		overscroll-behavior: contain;
	}

	/* Sticky Footer */
	.modal-footer-2026 {
		position: sticky;
		bottom: 0;
		z-index: 10;
		padding: 1rem;
		padding-bottom: calc(1rem + env(safe-area-inset-bottom, 0));
		border-top: 1px solid #e5e7eb;
		background: #f9fafb;
		flex-shrink: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* sm: 640px+ - Centered modal */
	@media (min-width: 640px) {
		.modal-backdrop-2026 {
			align-items: center;
			padding: 1.5rem;
		}

		.modal-swipe-indicator {
			display: none;
		}

		.modal-panel-2026 {
			position: relative;
			inset: auto;
			max-height: 85vh;
			border-radius: 0.75rem;
			box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
			padding-top: 0;
			padding-bottom: 0;
		}

		.modal-size-sm {
			max-width: 24rem;
		}

		.modal-size-md {
			max-width: 32rem;
		}

		.modal-size-lg {
			max-width: 42rem;
		}

		.modal-size-xl {
			max-width: 56rem;
		}

		.modal-header-2026 {
			padding: 1.25rem 1.5rem;
			padding-top: 1.25rem;
			border-radius: 0.75rem 0.75rem 0 0;
		}

		.modal-title-2026 {
			font-size: 1.25rem;
		}

		.modal-close-absolute-2026 {
			top: 1rem;
		}

		.modal-body-2026 {
			padding: 1.5rem;
		}

		.modal-footer-2026 {
			padding: 1rem 1.5rem;
			padding-bottom: 1rem;
			border-radius: 0 0 0.75rem 0.75rem;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.modal-header-2026 {
			padding: 1.5rem;
		}

		.modal-body-2026 {
			padding: 1.5rem;
		}
	}

	/* lg: 1024px+ */
	@media (min-width: 1024px) {
		.modal-panel-2026 {
			border-radius: 1rem;
		}

		.modal-header-2026 {
			border-radius: 1rem 1rem 0 0;
		}

		.modal-footer-2026 {
			border-radius: 0 0 1rem 1rem;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.modal-overlay-2026 {
			transition: none;
		}

		.modal-close-2026,
		.modal-close-absolute-2026 {
			transition: none;
		}
	}

	@media (prefers-contrast: high) {
		.modal-panel-2026 {
			border: 2px solid #000;
		}

		.modal-close-2026,
		.modal-close-absolute-2026 {
			border: 2px solid currentColor;
		}
	}

	/* Landscape orientation adjustments */
	@media (max-height: 500px) and (orientation: landscape) {
		.modal-panel-2026 {
			max-height: 100dvh;
		}

		.modal-header-2026 {
			padding: 0.75rem 1rem;
		}

		.modal-body-2026 {
			padding: 0.75rem 1rem;
		}

		.modal-footer-2026 {
			padding: 0.75rem 1rem;
		}
	}
</style>

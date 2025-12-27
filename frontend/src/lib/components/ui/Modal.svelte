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
		sm: 'max-w-md',
		md: 'max-w-lg',
		lg: 'max-w-2xl',
		xl: 'max-w-4xl',
		fullscreen: 'max-w-none w-full h-full m-0 rounded-none'
	};

	// Store the element that triggered the modal
	let _triggerElement: HTMLElement | null = $state(null);
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
						const initialElement = modalElement.querySelector(
							initialFocusSelector
						) as HTMLElement;
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
	<!-- Modal Backdrop -->
	<div
		class="fixed inset-0 z-50 overflow-y-auto"
		onclick={handleBackdropClick}
		role="presentation"
	>
		<div
			class="flex min-h-screen items-center justify-center p-4"
			class:p-0={size === 'fullscreen'}
		>
			<!-- Backdrop overlay -->
			<div
				class="fixed inset-0 bg-black/50 transition-opacity"
				aria-hidden="true"
			></div>

			<!-- Modal Panel -->
			<div
				bind:this={modalElement}
				class="relative bg-white rounded-lg shadow-xl {sizes[size]} w-full focus:outline-none"
				role="dialog"
				aria-modal="true"
				aria-labelledby={title ? titleId : undefined}
				aria-describedby={descId}
				tabindex="-1"
			>
				<!-- Header -->
				{#if title}
					<div class="flex items-center justify-between px-6 py-4 border-b border-gray-200">
						<h2
							id={titleId}
							class="text-lg font-semibold text-gray-900"
						>
							{title}
						</h2>
						<button
							type="button"
							onclick={close}
							class="text-gray-400 hover:text-gray-600 transition-colors rounded-lg p-1
								focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
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
						class="absolute top-4 right-4 text-gray-400 hover:text-gray-600 transition-colors
							rounded-lg p-1 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
						aria-label="Close dialog"
					>
						<IconX size={20} aria-hidden="true" />
					</button>
				{/if}

				<!-- Body -->
				<div id={descId} class="px-6 py-4">
					{@render children?.()}
				</div>

				<!-- Footer -->
				{#if footer}
					<div class="px-6 py-4 border-t border-gray-200 bg-gray-50 rounded-b-lg">
						{@render footer()}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

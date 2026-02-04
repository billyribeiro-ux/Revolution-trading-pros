<script lang="ts">
	/**
	 * Modal Component - Accessible & Animated
	 * Displays content in an overlay with backdrop
	 */
	import type { Snippet } from 'svelte';
	import { IconX } from '$lib/icons';

	interface Props {
		isOpen?: boolean;
		title?: string;
		onClose?: () => void;
		children?: Snippet;
	}

	let { isOpen = false, title = '', onClose = () => {}, children }: Props = $props();

	let _modalRef: HTMLDivElement | null = $state(null);

	// Close on Escape key
	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Escape' && isOpen) {
			onClose();
		}
	}

	// Close on backdrop click
	function handleBackdropClick(event: MouseEvent) {
		if (event.target === event.currentTarget) {
			onClose();
		}
	}

	// Prevent body scroll when modal is open
	$effect(() => {
		if (typeof document !== 'undefined') {
			document.body.style.overflow = isOpen ? 'hidden' : '';
		}
		// Cleanup on unmount
		return () => {
			if (typeof document !== 'undefined') {
				document.body.style.overflow = '';
			}
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<div
		class="modal-backdrop"
		onclick={handleBackdropClick}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
		role="presentation"
		tabindex="-1"
	>
		<div
			bind:this={modalRef}
			class="modal-container"
			role="dialog"
			aria-modal="true"
			aria-labelledby="modal-title"
			tabindex="-1"
		>
			<!-- Header -->
			<div class="modal-header">
				<h2 id="modal-title" class="modal-title">{title}</h2>
				<button type="button" class="modal-close" onclick={onClose} aria-label="Close modal">
					<IconX size={24} stroke={2} />
				</button>
			</div>

			<!-- Content -->
			<div class="modal-content">
				{@render children?.()}
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   2026 RESPONSIVE MODAL - Mobile-First Design
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Backdrop */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		align-items: flex-end; /* Mobile: bottom sheet style */
		justify-content: center;
		padding: 0;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		animation: fadeIn 0.2s ease-out;
		/* Safe area support for notched devices */
		padding-bottom: env(safe-area-inset-bottom, 0);
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Modal Container - Mobile First (Full screen bottom sheet) */
	.modal-container {
		position: fixed;
		inset: 0;
		width: 100%;
		max-height: 100dvh;
		background: linear-gradient(135deg, #0a0a0a 0%, #1a1a2e 100%);
		border: none;
		border-radius: 0;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		overflow: hidden;
		display: flex;
		flex-direction: column;
		animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		/* Safe area insets */
		padding-top: env(safe-area-inset-top, 0);
		padding-bottom: env(safe-area-inset-bottom, 0);
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(100%);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Header - Sticky */
	.modal-header {
		position: sticky;
		top: 0;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1rem;
		padding-top: calc(1rem + env(safe-area-inset-top, 0));
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(10, 10, 10, 0.95);
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		flex-shrink: 0;
	}

	.modal-title {
		font-size: 1.125rem;
		font-weight: 700;
		color: white;
		margin: 0;
	}

	/* Touch target: 44x44px minimum */
	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 44px;
		min-height: 44px;
		width: 44px;
		height: 44px;
		padding: 0;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		color: rgba(255, 255, 255, 0.6);
		cursor: pointer;
		transition: all 0.2s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.modal-close:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.2);
		color: white;
		transform: scale(1.05);
	}

	.modal-close:active {
		transform: scale(0.95);
	}

	.modal-close:focus-visible {
		outline: 2px solid rgba(255, 255, 255, 0.5);
		outline-offset: 2px;
	}

	/* Content - Scrollable */
	.modal-content {
		flex: 1;
		padding: 1.5rem 1rem;
		padding-bottom: calc(1.5rem + env(safe-area-inset-bottom, 0));
		overflow-y: auto;
		-webkit-overflow-scrolling: touch;
		overscroll-behavior: contain;
		color: rgba(255, 255, 255, 0.8);
		line-height: 1.7;
	}

	/* Scrollbar styling */
	.modal-content::-webkit-scrollbar {
		width: 6px;
	}

	.modal-content::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 3px;
	}

	.modal-content::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.2);
		border-radius: 3px;
	}

	.modal-content::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Swipe-to-dismiss indicator for mobile */
	.modal-container::before {
		content: '';
		position: absolute;
		top: 8px;
		left: 50%;
		transform: translateX(-50%);
		width: 36px;
		height: 4px;
		background: rgba(255, 255, 255, 0.3);
		border-radius: 2px;
		z-index: 11;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ - Extra small devices */
	@media (min-width: 360px) {
		.modal-header {
			padding: 1rem 1.25rem;
		}

		.modal-content {
			padding: 1.5rem 1.25rem;
		}
	}

	/* sm: 640px+ - Small tablets, large phones landscape */
	@media (min-width: 640px) {
		.modal-backdrop {
			align-items: center;
			padding: 1.5rem;
		}

		.modal-container {
			position: relative;
			inset: auto;
			max-width: 500px;
			max-height: 85vh;
			border: 1px solid rgba(255, 255, 255, 0.1);
			border-radius: 1rem;
			padding-top: 0;
			padding-bottom: 0;
		}

		.modal-container::before {
			display: none; /* Hide swipe indicator on desktop */
		}

		@keyframes slideUp {
			from {
				opacity: 0;
				transform: translateY(20px) scale(0.95);
			}
			to {
				opacity: 1;
				transform: translateY(0) scale(1);
			}
		}

		.modal-header {
			padding: 1.25rem 1.5rem;
			padding-top: 1.25rem;
			border-radius: 1rem 1rem 0 0;
		}

		.modal-title {
			font-size: 1.25rem;
		}

		.modal-content {
			padding: 1.5rem;
			padding-bottom: 1.5rem;
			max-height: calc(85vh - 80px);
		}
	}

	/* md: 768px+ - Tablets */
	@media (min-width: 768px) {
		.modal-container {
			max-width: 560px;
		}

		.modal-header {
			padding: 1.5rem 2rem;
		}

		.modal-title {
			font-size: 1.5rem;
		}

		.modal-content {
			padding: 2rem;
		}
	}

	/* lg: 1024px+ - Desktop */
	@media (min-width: 1024px) {
		.modal-container {
			max-width: 600px;
			border-radius: 1.5rem;
		}

		.modal-header {
			border-radius: 1.5rem 1.5rem 0 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY & PREFERENCES
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.modal-backdrop,
		.modal-container {
			animation: none;
		}

		.modal-close {
			transition: none;
		}
	}

	@media (prefers-contrast: high) {
		.modal-container {
			border: 2px solid white;
		}

		.modal-close {
			border: 2px solid white;
		}
	}

	/* Landscape orientation adjustments */
	@media (max-height: 500px) and (orientation: landscape) {
		.modal-container {
			max-height: 100dvh;
		}

		.modal-header {
			padding: 0.75rem 1rem;
		}

		.modal-content {
			padding: 0.75rem 1rem;
		}
	}
</style>

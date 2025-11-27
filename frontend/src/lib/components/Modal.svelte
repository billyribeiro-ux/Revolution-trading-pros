<script lang="ts">
	/**
	 * Modal Component - Accessible & Animated
	 * Displays content in an overlay with backdrop
	 */
	import { onMount } from 'svelte';
	import type { Snippet } from 'svelte';
	import { IconX } from '@tabler/icons-svelte';

	interface Props {
		isOpen?: boolean;
		title?: string;
		onClose?: () => void;
		children?: Snippet;
	}

	let {
		isOpen = false,
		title = '',
		onClose = () => {},
		children
	}: Props = $props();

	let modalRef: HTMLDivElement;

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
	});

	onMount(() => {
		return () => {
			if (typeof document !== 'undefined') {
				document.body.style.overflow = '';
			}
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_interactive_supports_focus -->
	<div
		class="modal-backdrop"
		onclick={handleBackdropClick}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="modal-title"
		tabindex="-1"
	>
		<div bind:this={modalRef} class="modal-container">
			<!-- Header -->
			<div class="modal-header">
				<h2 id="modal-title" class="modal-title">{title}</h2>
				<button
					type="button"
					class="modal-close"
					onclick={onClose}
					aria-label="Close modal"
				>
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
	/* Backdrop */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		animation: fadeIn 0.2s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Modal Container */
	.modal-container {
		position: relative;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		background: linear-gradient(135deg, #0a0a0a 0%, #1a1a2e 100%);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1.5rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		overflow: hidden;
		animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
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

	/* Header */
	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.5rem 2rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.02);
	}

	.modal-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
		margin: 0;
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		padding: 0;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.5rem;
		color: rgba(255, 255, 255, 0.6);
		cursor: pointer;
		transition: all 0.2s ease;
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

	/* Content */
	.modal-content {
		padding: 2rem;
		overflow-y: auto;
		max-height: calc(90vh - 80px);
		color: rgba(255, 255, 255, 0.8);
		line-height: 1.7;
	}

	/* Scrollbar styling */
	.modal-content::-webkit-scrollbar {
		width: 8px;
	}

	.modal-content::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 4px;
	}

	.modal-content::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.2);
		border-radius: 4px;
	}

	.modal-content::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Responsive */
	@media (max-width: 640px) {
		.modal-container {
			max-width: 100%;
			max-height: 100vh;
			border-radius: 0;
		}

		.modal-header {
			padding: 1rem 1.5rem;
		}

		.modal-title {
			font-size: 1.25rem;
		}

		.modal-content {
			padding: 1.5rem;
			max-height: calc(100vh - 70px);
		}
	}
</style>

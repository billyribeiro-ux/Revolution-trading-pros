<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';
	import DialogPortal from './dialog-portal.svelte';
	import XIcon from '@tabler/icons-svelte-runes/icons/x';
	import type { Snippet } from 'svelte';
	import * as Dialog from './index.js';
	import { type WithoutChildrenOrChild } from '$lib/utils.js';
	import type { ComponentProps } from 'svelte';

	type ContentProps = WithoutChildrenOrChild<DialogPrimitive.ContentProps> & {
		portalProps?: WithoutChildrenOrChild<ComponentProps<typeof DialogPortal>>;
		children: Snippet;
		showCloseButton?: boolean;
	};

	let {
		ref = $bindable(null),
		class: className,
		portalProps,
		showCloseButton = true,
		children,
		...restProps
	}: ContentProps = $props();
</script>

<DialogPortal {...portalProps}>
	<Dialog.Overlay />
	<DialogPrimitive.Content
		bind:ref
		data-slot="dialog-content"
		class={['ui-dialog-content', className]}
		{...restProps}
	>
		<!-- Mobile swipe indicator -->
		<div class="ui-dialog-swipe-indicator" aria-hidden="true"></div>

		<!-- Content wrapper with proper padding -->
		<div class="ui-dialog-content-body">
			{@render children?.()}
		</div>

		{#if showCloseButton}
			<DialogPrimitive.Close class="ui-dialog-close">
				<XIcon />
				<span class="ui-dialog-close-label">Close</span>
			</DialogPrimitive.Close>
		{/if}
	</DialogPrimitive.Content>
</DialogPortal>

<style>
	:global(.ui-dialog-content) {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		width: 100%;
		flex-direction: column;
		border: 0;
		border-radius: 0;
		background: var(--dialog-background, #ffffff);
		color: var(--dialog-foreground, #0f172a);
		box-shadow:
			0 20px 25px -5px rgba(15, 23, 42, 0.1),
			0 8px 10px -6px rgba(15, 23, 42, 0.1);
		padding-top: env(safe-area-inset-top);
		padding-bottom: env(safe-area-inset-bottom);
	}

	:global(.ui-dialog-content[data-state='open']) {
		animation: ui-dialog-mobile-in 0.2s ease-out;
	}

	:global(.ui-dialog-content[data-state='closed']) {
		animation: ui-dialog-mobile-out 0.16s ease-in;
	}

	.ui-dialog-swipe-indicator {
		position: absolute;
		top: 0.5rem;
		left: 50%;
		width: 2.25rem;
		height: 0.25rem;
		transform: translateX(-50%);
		border-radius: 999px;
		background: rgba(100, 116, 139, 0.3);
	}

	.ui-dialog-content-body {
		flex: 1;
		overflow-y: auto;
		overscroll-behavior: contain;
		-webkit-overflow-scrolling: touch;
		padding: 1.5rem 1rem 1rem;
	}

	:global(.ui-dialog-close) {
		position: absolute;
		inset-block-start: 0.75rem;
		inset-inline-end: 0.75rem;
		display: flex;
		width: 2.75rem;
		min-width: 44px;
		height: 2.75rem;
		min-height: 44px;
		align-items: center;
		justify-content: center;
		border-radius: 0.375rem;
		opacity: 0.7;
		outline: none;
		touch-action: manipulation;
		transition:
			background-color 0.18s ease,
			box-shadow 0.18s ease,
			opacity 0.18s ease;
	}

	:global(.ui-dialog-close:hover) {
		background: var(--dialog-close-hover-background, #f1f5f9);
		opacity: 1;
	}

	:global(.ui-dialog-close:focus-visible) {
		box-shadow:
			0 0 0 2px var(--dialog-background, #ffffff),
			0 0 0 4px var(--dialog-ring, #3b82f6);
	}

	:global(.ui-dialog-close:disabled) {
		pointer-events: none;
	}

	:global(.ui-dialog-close svg) {
		pointer-events: none;
		width: 1.25rem;
		height: 1.25rem;
		flex-shrink: 0;
	}

	.ui-dialog-close-label {
		position: absolute;
		width: 1px;
		height: 1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		clip-path: inset(50%);
	}

	@keyframes ui-dialog-mobile-in {
		from {
			opacity: 0;
			transform: translateY(100%);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes ui-dialog-mobile-out {
		from {
			opacity: 1;
			transform: translateY(0);
		}
		to {
			opacity: 0;
			transform: translateY(100%);
		}
	}

	@keyframes ui-dialog-modal-in {
		from {
			opacity: 0;
			transform: translate(-50%, -48%) scale(0.95);
		}
		to {
			opacity: 1;
			transform: translate(-50%, -50%) scale(1);
		}
	}

	@keyframes ui-dialog-modal-out {
		from {
			opacity: 1;
			transform: translate(-50%, -50%) scale(1);
		}
		to {
			opacity: 0;
			transform: translate(-50%, -48%) scale(0.95);
		}
	}

	@media (min-width: 640px) {
		:global(.ui-dialog-content) {
			inset: auto;
			top: 50%;
			left: 50%;
			max-width: 32rem;
			max-height: 85vh;
			transform: translate(-50%, -50%);
			border: 1px solid var(--dialog-border, #cbd5e1);
			border-radius: 0.5rem;
			padding: 0;
		}

		:global(.ui-dialog-content[data-state='open']) {
			animation-name: ui-dialog-modal-in;
		}

		:global(.ui-dialog-content[data-state='closed']) {
			animation-name: ui-dialog-modal-out;
		}

		.ui-dialog-swipe-indicator {
			display: none;
		}

		.ui-dialog-content-body {
			padding: 1.5rem;
		}

		:global(.ui-dialog-close) {
			inset-block-start: 1rem;
			inset-inline-end: 1rem;
		}
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dialog-content) {
			--dialog-background: #020617;
			--dialog-foreground: #f8fafc;
			--dialog-border: #334155;
			--dialog-close-hover-background: #1e293b;
		}
	}
</style>

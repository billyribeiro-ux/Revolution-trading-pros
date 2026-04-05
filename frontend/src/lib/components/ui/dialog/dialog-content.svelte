<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';
	import DialogPortal from './dialog-portal.svelte';
	import { X as XIcon } from 'phosphor-svelte';
	import type { Snippet } from 'svelte';
	import * as Dialog from './index.js';
	import type { WithoutChildrenOrChild } from '$lib/utils.js';
	import type { ComponentProps } from 'svelte';

	type ContentProps = WithoutChildrenOrChild<DialogPrimitive.ContentProps> & {
		portalProps?: WithoutChildrenOrChild<ComponentProps<typeof DialogPortal>>;
		children: Snippet;
		showCloseButton?: boolean;
	};

	let props: ContentProps = $props();
	let ref = $state<HTMLElement | null>(null);
	let className = $derived(props.class);
	let portalProps = $derived(props.portalProps);
	let showCloseButton = $derived(props.showCloseButton ?? true);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const {
			ref: _,
			class: __,
			portalProps: ___,
			children: ____,
			showCloseButton: _____,
			...rest
		} = props;
		return rest;
	});
</script>

<DialogPortal {...portalProps}>
	<Dialog.Overlay />
	<DialogPrimitive.Content bind:ref data-slot="dialog-content" class={className} {...restProps}>
		<div class="dialog-swipe-indicator" aria-hidden="true"></div>

		<div class="dialog-body">
			{@render props.children?.()}
		</div>

		{#if showCloseButton}
			<DialogPrimitive.Close class="dialog-close-btn">
				<XIcon />
				<span class="sr-only">Close</span>
			</DialogPrimitive.Close>
		{/if}
	</DialogPrimitive.Content>
</DialogPortal>

<style>
	:global([data-slot='dialog-content']) {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		flex-direction: column;
		inline-size: 100%;
		border: 0;
		border-radius: 0;
		background-color: var(--background);
		box-shadow: var(--shadow-lg);
		padding-block-start: env(safe-area-inset-top);
		padding-block-end: env(safe-area-inset-bottom);
		transition-duration: var(--duration-normal);

		&[data-state='open'] {
			animation:
				fade-in var(--duration-normal) var(--ease-default),
				slide-in-from-bottom var(--duration-normal) var(--ease-default);
		}

		&[data-state='closed'] {
			animation:
				fade-out var(--duration-normal) var(--ease-default),
				slide-out-to-bottom var(--duration-normal) var(--ease-default);
		}
	}

	@media (min-width: 640px) {
		:global([data-slot='dialog-content']) {
			inset: auto;
			inset-block-start: 50%;
			inset-inline-start: 50%;
			translate: -50% -50%;
			max-inline-size: 32rem;
			max-block-size: 85vh;
			border-radius: var(--radius-lg);
			border: 1px solid var(--border);
			padding: 0;

			&[data-state='open'] {
				animation:
					fade-in var(--duration-normal) var(--ease-default),
					zoom-in-95 var(--duration-normal) var(--ease-default);
			}

			&[data-state='closed'] {
				animation:
					fade-out var(--duration-normal) var(--ease-default),
					zoom-out-95 var(--duration-normal) var(--ease-default);
			}
		}
	}

	:global(.dialog-swipe-indicator) {
		position: absolute;
		inset-block-start: var(--space-2);
		inset-inline-start: 50%;
		translate: -50% 0;
		inline-size: 2.25rem;
		block-size: 0.25rem;
		background-color: oklch(from var(--muted-foreground) l c h / 30%);
		border-radius: var(--radius-full);

		@media (min-width: 640px) {
			display: none;
		}
	}

	:global(.dialog-body) {
		flex: 1;
		overflow-y: auto;
		overscroll-behavior: contain;
		padding: var(--space-4);
		padding-block-start: var(--space-6);
		-webkit-overflow-scrolling: touch;

		@media (min-width: 640px) {
			padding: var(--space-6);
		}
	}

	:global(.dialog-close-btn) {
		position: absolute;
		inset-inline-end: var(--space-3);
		inset-block-start: var(--space-3);
		min-inline-size: 44px;
		min-block-size: 44px;
		inline-size: 2.75rem;
		block-size: 2.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		opacity: 0.7;
		touch-action: manipulation;
		transition: opacity var(--duration-fast) var(--ease-default);

		&:hover {
			opacity: 1;
			background-color: var(--accent);
		}

		&:focus {
			outline: none;
			box-shadow:
				0 0 0 2px var(--ring),
				0 0 0 4px var(--background);
		}

		&:disabled {
			pointer-events: none;
		}

		& :global(svg) {
			pointer-events: none;
			flex-shrink: 0;
			inline-size: 1.25rem;
			block-size: 1.25rem;
		}

		@media (min-width: 640px) {
			inset-inline-end: var(--space-4);
			inset-block-start: var(--space-4);
		}
	}
</style>

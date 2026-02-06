<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';
	import DialogPortal from './dialog-portal.svelte';
	import XIcon from '@lucide/svelte/icons/x';
	import type { Snippet } from 'svelte';
	import * as Dialog from './index.js';
	import { cn, type WithoutChildrenOrChild } from '$lib/utils.js';
	import type { ComponentProps } from 'svelte';

	type ContentProps = WithoutChildrenOrChild<DialogPrimitive.ContentProps> & {
		portalProps?: WithoutChildrenOrChild<ComponentProps<typeof DialogPortal>>;
		children: Snippet;
		showCloseButton?: boolean;
	};

	let props: ContentProps = $props();
	let ref = $state<HTMLElement | null>(props.ref ?? null);
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
	<DialogPrimitive.Content
		bind:ref
		data-slot="dialog-content"
		class={cn(
			// Base styles - Mobile first (full screen)
			'bg-background fixed inset-0 z-50 flex flex-col w-full border-0 rounded-none shadow-lg',
			// Safe area support
			'pt-[env(safe-area-inset-top)] pb-[env(safe-area-inset-bottom)]',
			// Animations
			'data-[state=open]:animate-in data-[state=closed]:animate-out',
			'data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0',
			'data-[state=closed]:slide-out-to-bottom data-[state=open]:slide-in-from-bottom',
			// sm: 640px+ - Centered modal
			'sm:inset-auto sm:top-[50%] sm:left-[50%] sm:translate-x-[-50%] sm:translate-y-[-50%]',
			'sm:max-w-lg sm:max-h-[85vh] sm:rounded-lg sm:border sm:p-0',
			'sm:data-[state=closed]:zoom-out-95 sm:data-[state=open]:zoom-in-95',
			'sm:data-[state=closed]:slide-out-to-bottom-0 sm:data-[state=open]:slide-in-from-bottom-0',
			// Duration
			'duration-200',
			className
		)}
		{...restProps}
	>
		<!-- Mobile swipe indicator -->
		<div
			class="sm:hidden absolute top-2 left-1/2 -translate-x-1/2 w-9 h-1 bg-muted-foreground/30 rounded-full"
			aria-hidden="true"
		></div>

		<!-- Content wrapper with proper padding -->
		<div
			class="flex-1 overflow-y-auto overscroll-contain p-4 pt-6 sm:p-6 -webkit-overflow-scrolling-touch"
		>
			{@render props.children?.()}
		</div>

		{#if showCloseButton}
			<DialogPrimitive.Close
				class="ring-offset-background focus:ring-ring absolute end-3 top-3 sm:end-4 sm:top-4 min-w-[44px] min-h-[44px] w-11 h-11 flex items-center justify-center rounded-md opacity-70 transition-opacity hover:opacity-100 hover:bg-accent focus:ring-2 focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none touch-manipulation [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-5"
			>
				<XIcon />
				<span class="sr-only">Close</span>
			</DialogPrimitive.Close>
		{/if}
	</DialogPrimitive.Content>
</DialogPortal>

<script lang="ts">
	import type { WithoutChildrenOrChild } from '$lib/utils.js';
	import DropdownMenuPortal from './dropdown-menu-portal.svelte';
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import type { ComponentProps } from 'svelte';

	type ContentProps = DropdownMenuPrimitive.ContentProps & {
		portalProps?: WithoutChildrenOrChild<ComponentProps<typeof DropdownMenuPortal>>;
	};

	let props: ContentProps = $props();
	let ref = $state<HTMLElement | null>(null);
	let sideOffset = $derived(props.sideOffset ?? 4);
	let portalProps = $derived(props.portalProps);
	let className = $derived(props.class);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, sideOffset: __, portalProps: ___, class: ____, ...rest } = props;
		return rest;
	});
</script>

<DropdownMenuPortal {...portalProps}>
	<DropdownMenuPrimitive.Content
		bind:ref
		data-slot="dropdown-menu-content"
		{sideOffset}
		class={className}
		{...restProps}
	/>
</DropdownMenuPortal>

<style>
	:global([data-slot='dropdown-menu-content']) {
		z-index: 50;
		min-inline-size: 8rem;
		max-block-size: var(--bits-dropdown-menu-content-available-height);
		overflow-x: hidden;
		overflow-y: auto;
		border-radius: var(--radius-md);
		border: 1px solid var(--border);
		background-color: var(--popover);
		color: var(--popover-foreground);
		padding: var(--space-1);
		box-shadow: var(--shadow-md);
		outline: none;
		transform-origin: var(--bits-dropdown-menu-content-transform-origin);

		&[data-state='open'] {
			animation: fade-in var(--duration-fast) var(--ease-default),
				zoom-in-95 var(--duration-fast) var(--ease-default);
		}

		&[data-state='closed'] {
			animation: fade-out var(--duration-fast) var(--ease-default),
				zoom-out-95 var(--duration-fast) var(--ease-default);
		}

		&[data-side='bottom'] { animation-name: fade-in, slide-in-from-top-2; }
		&[data-side='top'] { animation-name: fade-in, slide-in-from-bottom-2; }
		&[data-side='left'] { animation-name: fade-in, slide-in-from-end-2; }
		&[data-side='right'] { animation-name: fade-in, slide-in-from-start-2; }
	}
</style>

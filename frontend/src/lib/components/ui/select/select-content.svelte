<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';
	import SelectPortal from './select-portal.svelte';
	import SelectScrollUpButton from './select-scroll-up-button.svelte';
	import SelectScrollDownButton from './select-scroll-down-button.svelte';
	import type { WithoutChild, WithoutChildrenOrChild } from '$lib/utils.js';
	import type { ComponentProps } from 'svelte';

	let {
		ref = $bindable(null),
		class: className,
		sideOffset = 4,
		portalProps,
		children,
		preventScroll = true,
		...restProps
	}: WithoutChild<SelectPrimitive.ContentProps> & {
		portalProps?: WithoutChildrenOrChild<ComponentProps<typeof SelectPortal>>;
	} = $props();
</script>

<SelectPortal {...portalProps}>
	<SelectPrimitive.Content
		bind:ref
		{sideOffset}
		{preventScroll}
		data-slot="select-content"
		class={className}
		{...restProps}
	>
		<SelectScrollUpButton />
		<SelectPrimitive.Viewport class="select-viewport">
			{@render children?.()}
		</SelectPrimitive.Viewport>
		<SelectScrollDownButton />
	</SelectPrimitive.Content>
</SelectPortal>

<style>
	:global([data-slot='select-content']) {
		position: relative;
		z-index: 50;
		min-inline-size: 8rem;
		max-block-size: var(--bits-select-content-available-height);
		overflow-x: hidden;
		overflow-y: auto;
		border-radius: var(--radius-md);
		border: 1px solid var(--border);
		background-color: var(--popover);
		color: var(--popover-foreground);
		box-shadow: var(--shadow-md);
		transform-origin: var(--bits-select-content-transform-origin);

		&[data-state='open'] {
			animation:
				fade-in var(--duration-fast) var(--ease-default),
				zoom-in-95 var(--duration-fast) var(--ease-default);
		}

		&[data-state='closed'] {
			animation:
				fade-out var(--duration-fast) var(--ease-default),
				zoom-out-95 var(--duration-fast) var(--ease-default);
		}

		&[data-side='bottom'] {
			translate: 0 0.25rem;
		}
		&[data-side='top'] {
			translate: 0 -0.25rem;
		}
		&[data-side='left'] {
			translate: -0.25rem 0;
		}
		&[data-side='right'] {
			translate: 0.25rem 0;
		}
	}

	:global(.select-viewport) {
		block-size: var(--bits-select-anchor-height);
		inline-size: 100%;
		min-inline-size: var(--bits-select-anchor-width);
		scroll-margin-block: var(--space-1);
		padding: var(--space-1);
	}
</style>

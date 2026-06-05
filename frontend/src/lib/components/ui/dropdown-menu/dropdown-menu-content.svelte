<script lang="ts">
	import { type WithoutChildrenOrChild } from '$lib/utils.js';
	import DropdownMenuPortal from './dropdown-menu-portal.svelte';
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import type { ComponentProps } from 'svelte';

	type ContentProps = DropdownMenuPrimitive.ContentProps & {
		portalProps?: WithoutChildrenOrChild<ComponentProps<typeof DropdownMenuPortal>>;
	};

	let {
		ref = $bindable(null),
		sideOffset = 4,
		portalProps,
		class: className,
		...restProps
	}: ContentProps = $props();
</script>

<DropdownMenuPortal {...portalProps}>
	<DropdownMenuPrimitive.Content
		bind:ref
		data-slot="dropdown-menu-content"
		{sideOffset}
		class={['ui-dropdown-menu-content', className]}
		{...restProps}
	/>
</DropdownMenuPortal>

<style>
	:global(.ui-dropdown-menu-content) {
		--dropdown-menu-translate-x: 0;
		--dropdown-menu-translate-y: 0;

		z-index: 50;
		min-width: 8rem;
		max-height: var(--bits-dropdown-menu-content-available-height);
		overflow-x: hidden;
		overflow-y: auto;
		transform: translate(var(--dropdown-menu-translate-x), var(--dropdown-menu-translate-y));
		transform-origin: var(--bits-dropdown-menu-content-transform-origin);
		border: 1px solid var(--dropdown-menu-border, #cbd5e1);
		border-radius: 0.375rem;
		background: var(--dropdown-menu-background, #ffffff);
		color: var(--dropdown-menu-foreground, #0f172a);
		box-shadow:
			0 10px 15px -3px rgba(15, 23, 42, 0.1),
			0 4px 6px -4px rgba(15, 23, 42, 0.1);
		padding: 0.25rem;
		outline: none;
	}

	:global(.ui-dropdown-menu-content[data-side='bottom']) {
		--dropdown-menu-translate-y: 0.25rem;
	}

	:global(.ui-dropdown-menu-content[data-side='top']) {
		--dropdown-menu-translate-y: -0.25rem;
	}

	:global(.ui-dropdown-menu-content[data-side='left']) {
		--dropdown-menu-translate-x: -0.25rem;
	}

	:global(.ui-dropdown-menu-content[data-side='right']) {
		--dropdown-menu-translate-x: 0.25rem;
	}

	:global(.ui-dropdown-menu-content[data-state='open']) {
		animation: ui-dropdown-menu-in 0.16s ease-out;
	}

	:global(.ui-dropdown-menu-content[data-state='closed']) {
		animation: ui-dropdown-menu-out 0.12s ease-in;
	}

	@keyframes ui-dropdown-menu-in {
		from {
			opacity: 0;
			transform: translate(var(--dropdown-menu-translate-x), var(--dropdown-menu-translate-y))
				scale(0.95);
		}
		to {
			opacity: 1;
			transform: translate(var(--dropdown-menu-translate-x), var(--dropdown-menu-translate-y))
				scale(1);
		}
	}

	@keyframes ui-dropdown-menu-out {
		from {
			opacity: 1;
			transform: translate(var(--dropdown-menu-translate-x), var(--dropdown-menu-translate-y))
				scale(1);
		}
		to {
			opacity: 0;
			transform: translate(var(--dropdown-menu-translate-x), var(--dropdown-menu-translate-y))
				scale(0.95);
		}
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dropdown-menu-content) {
			--dropdown-menu-border: #475569;
			--dropdown-menu-background: #020617;
			--dropdown-menu-foreground: #f8fafc;
		}
	}
</style>

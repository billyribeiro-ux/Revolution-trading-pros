<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DropdownMenuPrimitive.SubContentProps = $props();
</script>

<DropdownMenuPrimitive.SubContent
	bind:ref
	data-slot="dropdown-menu-sub-content"
	class={className}
	{...restProps}
/>

<style>
	:global([data-slot='dropdown-menu-sub-content']) {
		z-index: 50;
		min-inline-size: 8rem;
		overflow: hidden;
		border-radius: var(--radius-md);
		border: 1px solid var(--border);
		background-color: var(--popover);
		color: var(--popover-foreground);
		padding: var(--space-1);
		box-shadow: var(--shadow-lg);
		transform-origin: var(--bits-dropdown-menu-content-transform-origin);

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
			animation-name: fade-in, slide-in-from-top-2;
		}
		&[data-side='top'] {
			animation-name: fade-in, slide-in-from-bottom-2;
		}
		&[data-side='left'] {
			animation-name: fade-in, slide-in-from-end-2;
		}
		&[data-side='right'] {
			animation-name: fade-in, slide-in-from-start-2;
		}
	}
</style>

<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import type { ComponentProps } from 'svelte';

	type GroupHeadingProps = ComponentProps<typeof DropdownMenuPrimitive.GroupHeading> & {
		inset?: boolean;
	};

	let {
		ref = $bindable(null),
		class: className,
		inset,
		...restProps
	}: GroupHeadingProps = $props();
</script>

<DropdownMenuPrimitive.GroupHeading
	bind:ref
	data-slot="dropdown-menu-group-heading"
	data-inset={inset}
	class={['ui-dropdown-menu-group-heading', className]}
	{...restProps}
/>

<style>
	:global(.ui-dropdown-menu-group-heading) {
		padding: 0.375rem 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		line-height: 1.25rem;
	}

	:global(.ui-dropdown-menu-group-heading[data-inset]) {
		padding-inline-start: 2rem;
	}
</style>

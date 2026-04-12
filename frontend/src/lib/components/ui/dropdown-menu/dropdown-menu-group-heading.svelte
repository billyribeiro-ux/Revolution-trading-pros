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
	class={className}
	{...restProps}
/>

<style>
	:global([data-slot='dropdown-menu-group-heading']) {
		padding-inline: var(--space-2);
		padding-block: var(--space-1-5);
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);

		&[data-inset] {
			padding-inline-start: var(--space-8);
		}
	}
</style>

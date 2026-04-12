<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DropdownMenuPrimitive.SeparatorProps = $props();
</script>

<DropdownMenuPrimitive.Separator
	bind:ref
	data-slot="dropdown-menu-separator"
	class={className}
	{...restProps}
/>

<style>
	:global([data-slot='dropdown-menu-separator']) {
		background-color: var(--border);
		margin-inline: calc(-1 * var(--space-1));
		margin-block: var(--space-1);
		block-size: 1px;
	}
</style>

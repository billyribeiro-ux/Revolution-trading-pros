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
	class={['ui-dropdown-menu-separator', className]}
	{...restProps}
/>

<style>
	:global(.ui-dropdown-menu-separator) {
		height: 1px;
		margin: 0.25rem -0.25rem;
		background: var(--border-default, #30363d);
	}
</style>

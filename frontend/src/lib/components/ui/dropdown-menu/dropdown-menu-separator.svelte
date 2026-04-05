<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';

	let props: DropdownMenuPrimitive.SeparatorProps = $props();
	let ref = $state<HTMLElement | null>(null);
	let className = $derived(props.class);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, class: __, ...rest } = props;
		return rest;
	});
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

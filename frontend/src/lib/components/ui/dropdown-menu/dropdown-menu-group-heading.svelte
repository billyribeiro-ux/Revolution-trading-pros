<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
		import type { ComponentProps } from 'svelte';

	type GroupHeadingProps = ComponentProps<typeof DropdownMenuPrimitive.GroupHeading> & {
		inset?: boolean;
	};

	let props: GroupHeadingProps = $props();
	let ref = $state<HTMLElement | null>(null);
	let className = $derived(props.class);
	let inset = $derived(props.inset);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, class: __, inset: ___, ...rest } = props;
		return rest;
	});
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

<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";
	import type { ComponentProps } from "svelte";

	type GroupHeadingProps = ComponentProps<typeof DropdownMenuPrimitive.GroupHeading> & {
		inset?: boolean;
	};

	let props: GroupHeadingProps = $props();
	let ref = $state<HTMLElement | null>(props.ref ?? null);
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
	class={cn("px-2 py-1.5 text-sm font-semibold data-[inset]:ps-8", className)}
	{...restProps}
/>

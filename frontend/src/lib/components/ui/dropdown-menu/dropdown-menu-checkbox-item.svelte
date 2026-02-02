<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
	import CheckIcon from "@lucide/svelte/icons/check";
	import MinusIcon from "@lucide/svelte/icons/minus";
	import { cn, type WithoutChildrenOrChild } from "$lib/utils.js";
	import type { Snippet } from "svelte";

	type CheckboxItemProps = WithoutChildrenOrChild<DropdownMenuPrimitive.CheckboxItemProps> & {
		children?: Snippet;
	};

	let props: CheckboxItemProps = $props();
	let ref = $state<HTMLElement | null>(props.ref ?? null);
	let checked = $state(props.checked ?? false);
	let indeterminate = $state(props.indeterminate ?? false);
	let className = $derived(props.class);
	let childrenProp = $derived(props.children);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});
	$effect(() => {
		if (props.checked !== undefined && props.checked !== checked) {
			checked = props.checked;
		}
	});
	$effect(() => {
		if (props.indeterminate !== undefined && props.indeterminate !== indeterminate) {
			indeterminate = props.indeterminate;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, checked: __, indeterminate: ___, class: ____, children: _____, ...rest } = props;
		return rest;
	});
</script>

<DropdownMenuPrimitive.CheckboxItem
	bind:ref
	bind:checked
	bind:indeterminate
	data-slot="dropdown-menu-checkbox-item"
	class={cn(
		"focus:bg-accent focus:text-accent-foreground relative flex cursor-default items-center gap-2 rounded-sm py-1.5 ps-8 pe-2 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
		className
	)}
	{...restProps}
>
	{#snippet children({ checked, indeterminate })}
		<span
			class="pointer-events-none absolute start-2 flex size-3.5 items-center justify-center"
		>
			{#if indeterminate}
				<MinusIcon class="size-4" />
			{:else}
				<CheckIcon class={cn("size-4", !checked && "text-transparent")} />
			{/if}
		</span>
		{@render childrenProp?.()}
	{/snippet}
</DropdownMenuPrimitive.CheckboxItem>

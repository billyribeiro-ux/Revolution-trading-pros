<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let props: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
	let ref = $state<HTMLDivElement | null>(props.ref ?? null);
	let className = $derived(props.class);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, class: __, children: ___, ...rest } = props;
		return rest;
	});
</script>

<div
	bind:this={ref}
	data-slot="dialog-header"
	class={cn(
		// Sticky header for mobile
		"sticky top-0 z-10 flex flex-col gap-1.5 text-center sm:text-start",
		// Safe area padding on mobile
		"pt-[calc(0.5rem+env(safe-area-inset-top))] sm:pt-0",
		// Background for sticky effect
		"bg-background/95 backdrop-blur-sm sm:bg-transparent sm:backdrop-blur-none",
		// Padding
		"px-4 pb-3 sm:px-0 sm:pb-0",
		// Border on mobile
		"border-b sm:border-b-0",
		className
	)}
	{...restProps}
>
	{@render props.children?.()}
</div>

<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
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
	{@render children?.()}
</div>

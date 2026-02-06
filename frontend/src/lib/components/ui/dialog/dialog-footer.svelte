<script lang="ts">
	import { cn, type WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

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
	data-slot="dialog-footer"
	class={cn(
		// Sticky footer for mobile
		'sticky bottom-0 z-10 flex flex-col-reverse gap-2 sm:flex-row sm:justify-end',
		// Safe area padding on mobile
		'pb-[calc(1rem+env(safe-area-inset-bottom))] sm:pb-0',
		// Background for sticky effect
		'bg-background/95 backdrop-blur-sm sm:bg-transparent sm:backdrop-blur-none',
		// Padding and border
		'px-4 pt-3 sm:px-0 sm:pt-0',
		'border-t sm:border-t-0',
		// Touch targets
		'[&>button]:min-h-[44px] [&>button]:touch-manipulation',
		className
	)}
	{...restProps}
>
	{@render props.children?.()}
</div>

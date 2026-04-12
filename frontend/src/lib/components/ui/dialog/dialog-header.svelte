<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils.js';
	import type { WithElementRef } from '$lib/utils.js';

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
		'sticky top-0 z-10 flex flex-col gap-1.5 text-center sm:text-start',
		// Safe area padding on mobile
		'pt-[calc(0.5rem+env(safe-area-inset-top))] sm:pt-0',
		// Background for sticky effect
		'bg-background/95 backdrop-blur-sm sm:bg-transparent sm:backdrop-blur-none',
		// Padding
		'px-4 pb-3 sm:px-0 sm:pb-0',
		// Border on mobile
		'border-b sm:border-b-0',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	:global([data-slot='dialog-header']) {
		position: sticky;
		inset-block-start: 0;
		z-index: 10;
		display: flex;
		flex-direction: column;
		gap: var(--space-1-5);
		text-align: center;
		padding-block-start: calc(0.5rem + env(safe-area-inset-top));
		padding-block-end: var(--space-3);
		padding-inline: var(--space-4);
		background-color: oklch(from var(--background) l c h / 95%);
		backdrop-filter: blur(4px);
		border-block-end: 1px solid var(--border);

		@media (min-width: 640px) {
			text-align: start;
			padding-block-start: 0;
			padding-block-end: 0;
			padding-inline: 0;
			background-color: transparent;
			backdrop-filter: none;
			border-block-end: 0;
		}
	}
</style>

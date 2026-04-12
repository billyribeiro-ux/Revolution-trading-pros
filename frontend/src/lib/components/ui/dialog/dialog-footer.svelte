<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
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
	{@render children?.()}
</div>

<style>
	:global([data-slot='dialog-footer']) {
		position: sticky;
		inset-block-end: 0;
		z-index: 10;
		display: flex;
		flex-direction: column-reverse;
		gap: var(--space-2);
		padding-block-end: calc(1rem + env(safe-area-inset-bottom));
		padding-block-start: var(--space-3);
		padding-inline: var(--space-4);
		background-color: oklch(from var(--background) l c h / 95%);
		backdrop-filter: blur(4px);
		border-block-start: 1px solid var(--border);

		& > :global(button) {
			min-block-size: 44px;
			touch-action: manipulation;
		}

		@media (min-width: 640px) {
			flex-direction: row;
			justify-content: flex-end;
			padding: 0;
			background-color: transparent;
			backdrop-filter: none;
			border-block-start: 0;
		}
	}
</style>

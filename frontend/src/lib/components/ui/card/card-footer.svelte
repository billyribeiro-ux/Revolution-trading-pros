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
	data-slot="card-footer"
	class={cn('flex items-center px-6 [.border-t]:pt-6', className)}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	:global([data-slot='card-footer']) {
		display: flex;
		align-items: center;
		padding-inline: var(--space-6);
	}

	:global(.border-t > [data-slot='card-footer']) {
		padding-block-start: var(--space-6);
	}
</style>

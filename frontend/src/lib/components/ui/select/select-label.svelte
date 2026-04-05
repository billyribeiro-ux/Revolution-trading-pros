<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> & {} = $props();
</script>

<div bind:this={ref} data-slot="select-label" class={className} {...restProps}>
	{@render children?.()}
</div>

<style>
	:global([data-slot='select-label']) {
		color: var(--muted-foreground);
		padding-inline: var(--space-2);
		padding-block: var(--space-1-5);
		font-size: var(--text-xs);
	}
</style>

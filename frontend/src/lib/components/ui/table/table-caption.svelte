<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLElement>> = $props();
</script>

<caption bind:this={ref} data-slot="table-caption" class={className} {...restProps}>
	{@render children?.()}
</caption>

<style>
	:global([data-slot='table-caption']) {
		color: var(--muted-foreground);
		margin-block-start: var(--space-4);
		font-size: var(--text-sm);
	}
</style>

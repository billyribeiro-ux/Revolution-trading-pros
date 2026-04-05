<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLThAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLThAttributes> = $props();
</script>

<th bind:this={ref} data-slot="table-head" class={className} {...restProps}>
	{@render children?.()}
</th>

<style>
	:global([data-slot='table-head']) {
		color: var(--foreground);
		block-size: 2.5rem;
		background-clip: padding-box;
		padding-inline: var(--space-2);
		text-align: start;
		vertical-align: middle;
		font-weight: var(--weight-medium);
		white-space: nowrap;

		&:has([role='checkbox']) {
			padding-inline-end: 0;
		}
	}
</style>

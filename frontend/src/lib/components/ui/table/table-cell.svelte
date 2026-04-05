<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLTdAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLTdAttributes> = $props();
</script>

<td bind:this={ref} data-slot="table-cell" class={className} {...restProps}>
	{@render children?.()}
</td>

<style>
	:global([data-slot='table-cell']) {
		background-clip: padding-box;
		padding: var(--space-2);
		vertical-align: middle;
		white-space: nowrap;

		&:has([role='checkbox']) {
			padding-inline-end: 0;
		}
	}
</style>

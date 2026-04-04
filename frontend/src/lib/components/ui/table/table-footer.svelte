<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLTableSectionElement>> = $props();
</script>

<tfoot
	bind:this={ref}
	data-slot="table-footer"
	class={className}
	{...restProps}
>
	{@render children?.()}
</tfoot>

<style>
	:global([data-slot='table-footer']) {
		background-color: oklch(from var(--muted) l c h / 50%);
		border-block-start: 1px solid var(--border);
		font-weight: var(--weight-medium);

		& > :global(tr:last-child) {
			border-block-end: 0;
		}
	}
</style>

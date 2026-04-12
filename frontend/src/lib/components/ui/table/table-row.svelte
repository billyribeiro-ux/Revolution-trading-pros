<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLTableRowElement>> = $props();
</script>

<tr
	bind:this={ref}
	data-slot="table-row"
	class={cn(
		'hover:[&,&>svelte-css-wrapper]:[&>th,td]:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</tr>

<style>
	:global([data-slot='table-row']) {
		border-block-end: 1px solid var(--border);
		transition:
			color var(--duration-fast) var(--ease-default),
			background-color var(--duration-fast) var(--ease-default);

		&:hover :global(th),
		&:hover :global(td) {
			background-color: oklch(from var(--muted) l c h / 50%);
		}

		&[data-state='selected'] {
			background-color: var(--muted);
		}
	}
</style>

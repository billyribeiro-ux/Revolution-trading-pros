<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	let props: WithElementRef<HTMLAttributes<HTMLTableRowElement>> = $props();
	let ref = $state<HTMLElement | null>(null);
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

<tr bind:this={ref} data-slot="table-row" class={className} {...restProps}>
	{@render props.children?.()}
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

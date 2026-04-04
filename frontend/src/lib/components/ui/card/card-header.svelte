<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	let props: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
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

<div bind:this={ref} data-slot="card-header" class={className} {...restProps}>
	{@render props.children?.()}
</div>

<!-- svelte-ignore css_unused_selector -->
<style>
	:global([data-slot='card-header']) {
		container-type: inline-size;
		container-name: card-header;
		display: grid;
		grid-auto-rows: min-content;
		grid-template-rows: auto auto;
		align-items: start;
		gap: var(--space-1-5);
		padding-inline: var(--space-6);

		&:has([data-slot='card-action']) {
			grid-template-columns: 1fr auto;
		}
	}

	:global(.border-b > [data-slot='card-header']) {
		padding-block-end: var(--space-6);
	}
</style>

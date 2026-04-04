<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';

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

<div bind:this={ref} data-slot="card" class={className} {...restProps}>
	{@render props.children?.()}
</div>

<style>
	:global([data-slot='card']) {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		border-radius: var(--radius-xl);
		border: 1px solid var(--border);
		background-color: var(--card);
		color: var(--card-foreground);
		padding-block: var(--space-6);
		box-shadow: var(--shadow-sm);
	}
</style>

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

<div
	bind:this={ref}
	data-slot="card-footer"
	class={className}
	{...restProps}
>
	{@render props.children?.()}
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

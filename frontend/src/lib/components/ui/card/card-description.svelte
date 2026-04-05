<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';

	let props: WithElementRef<HTMLAttributes<HTMLParagraphElement>> = $props();
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

<p bind:this={ref} data-slot="card-description" class={className} {...restProps}>
	{@render props.children?.()}
</p>

<style>
	:global([data-slot='card-description']) {
		color: var(--muted-foreground);
		font-size: var(--text-sm);
	}
</style>

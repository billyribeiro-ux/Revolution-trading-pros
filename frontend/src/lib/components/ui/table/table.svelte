<script lang="ts">
	import type { HTMLTableAttributes } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';

	let props: WithElementRef<HTMLTableAttributes> = $props();
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

<div data-slot="table-container">
	<table bind:this={ref} data-slot="table" class={className} {...restProps}>
		{@render props.children?.()}
	</table>
</div>

<style>
	:global([data-slot='table-container']) {
		position: relative;
		inline-size: 100%;
		overflow-x: auto;
	}

	:global([data-slot='table']) {
		inline-size: 100%;
		caption-side: bottom;
		font-size: var(--text-sm);
	}
</style>

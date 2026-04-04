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

<div bind:this={ref} data-slot="dialog-header" class={className} {...restProps}>
	{@render props.children?.()}
</div>

<style>
	:global([data-slot='dialog-header']) {
		position: sticky;
		inset-block-start: 0;
		z-index: 10;
		display: flex;
		flex-direction: column;
		gap: var(--space-1-5);
		text-align: center;
		padding-block-start: calc(0.5rem + env(safe-area-inset-top));
		padding-block-end: var(--space-3);
		padding-inline: var(--space-4);
		background-color: oklch(from var(--background) l c h / 95%);
		backdrop-filter: blur(4px);
		border-block-end: 1px solid var(--border);

		@media (min-width: 640px) {
			text-align: start;
			padding-block-start: 0;
			padding-block-end: 0;
			padding-inline: 0;
			background-color: transparent;
			backdrop-filter: none;
			border-block-end: 0;
		}
	}
</style>

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

<div bind:this={ref} data-slot="dialog-footer" class={className} {...restProps}>
	{@render props.children?.()}
</div>

<style>
	:global([data-slot='dialog-footer']) {
		position: sticky;
		inset-block-end: 0;
		z-index: 10;
		display: flex;
		flex-direction: column-reverse;
		gap: var(--space-2);
		padding-block-end: calc(1rem + env(safe-area-inset-bottom));
		padding-block-start: var(--space-3);
		padding-inline: var(--space-4);
		background-color: oklch(from var(--background) l c h / 95%);
		backdrop-filter: blur(4px);
		border-block-start: 1px solid var(--border);

		& > :global(button) {
			min-block-size: 44px;
			touch-action: manipulation;
		}

		@media (min-width: 640px) {
			flex-direction: row;
			justify-content: flex-end;
			padding: 0;
			background-color: transparent;
			backdrop-filter: none;
			border-block-start: 0;
		}
	}
</style>

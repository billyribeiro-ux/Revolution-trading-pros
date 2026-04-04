<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	type LabelProps = WithElementRef<HTMLAttributes<HTMLDivElement>> & {
		inset?: boolean;
	};

	let props: LabelProps = $props();
	let ref = $state<HTMLElement | null>(null);
	let className = $derived(props.class);
	let inset = $derived(props.inset);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, class: __, inset: ___, children: ____, ...rest } = props;
		return rest;
	});
</script>

<div
	bind:this={ref}
	data-slot="dropdown-menu-label"
	data-inset={inset}
	class={className}
	{...restProps}
>
	{@render props.children?.()}
</div>

<style>
	:global([data-slot='dropdown-menu-label']) {
		padding-inline: var(--space-2);
		padding-block: var(--space-1-5);
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);

		&[data-inset] {
			padding-inline-start: var(--space-8);
		}
	}
</style>

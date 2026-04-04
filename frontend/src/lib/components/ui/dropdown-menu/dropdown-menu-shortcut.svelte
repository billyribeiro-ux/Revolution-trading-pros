<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';

	let props: WithElementRef<HTMLAttributes<HTMLSpanElement>> = $props();
	let ref = $state<HTMLSpanElement | null>(null);
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

<span
	bind:this={ref}
	data-slot="dropdown-menu-shortcut"
	class={className}
	{...restProps}
>
	{@render props.children?.()}
</span>

<style>
	:global([data-slot='dropdown-menu-shortcut']) {
		color: var(--muted-foreground);
		margin-inline-start: auto;
		font-size: var(--text-xs);
		letter-spacing: 0.1em;
	}
</style>

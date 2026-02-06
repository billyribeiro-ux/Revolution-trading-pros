<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn, type WithElementRef } from '$lib/utils.js';

	let props: WithElementRef<HTMLAttributes<HTMLSpanElement>> = $props();
	let ref = $state<HTMLSpanElement | null>(props.ref ?? null);
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
	class={cn('text-muted-foreground ms-auto text-xs tracking-widest', className)}
	{...restProps}
>
	{@render props.children?.()}
</span>

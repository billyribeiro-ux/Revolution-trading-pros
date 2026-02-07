<script lang="ts">
	import { cn, type WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	type LabelProps = WithElementRef<HTMLAttributes<HTMLDivElement>> & {
		inset?: boolean;
	};

	let props: LabelProps = $props();
	let ref = $state<HTMLElement | null>(props.ref ?? null);
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
	class={cn('px-2 py-1.5 text-sm font-semibold data-[inset]:ps-8', className)}
	{...restProps}
>
	{@render props.children?.()}
</div>

<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let props: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
	let ref = $state<HTMLDivElement | null>(props.ref ?? null);
	let className = $derived(props.class);

	// Sync ref back to parent
	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	// Extract rest props
	let restProps = $derived.by(() => {
		const { ref: _, class: __, children: ___, ...rest } = props;
		return rest;
	});
</script>

<div
	bind:this={ref}
	data-slot="card"
	class={cn(
		"bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm",
		className
	)}
	{...restProps}
>
	{@render props.children?.()}
</div>

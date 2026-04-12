<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils.js';
	import type { WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card"
	class={cn(
		'bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm',
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>

<style>
	:global([data-slot='card']) {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		border-radius: var(--radius-xl);
		border: 1px solid var(--border);
		background-color: var(--card);
		color: var(--card-foreground);
		padding-block: var(--space-6);
		box-shadow: var(--shadow-sm);
	}
</style>

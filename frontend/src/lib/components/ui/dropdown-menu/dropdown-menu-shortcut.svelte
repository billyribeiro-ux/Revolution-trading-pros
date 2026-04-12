<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLSpanElement>> = $props();
</script>

<span
	bind:this={ref}
	data-slot="dropdown-menu-shortcut"
	class={cn('text-muted-foreground ms-auto text-xs tracking-widest', className)}
	{...restProps}
>
	{@render children?.()}
</span>

<style>
	:global([data-slot='dropdown-menu-shortcut']) {
		color: var(--muted-foreground);
		margin-inline-start: auto;
		font-size: var(--text-xs);
		letter-spacing: 0.1em;
	}
</style>

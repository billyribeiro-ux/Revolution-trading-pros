<script lang="ts">
	import type { WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	type LabelProps = WithElementRef<HTMLAttributes<HTMLDivElement>> & {
		inset?: boolean;
	};

	let {
		ref = $bindable(null),
		class: className,
		inset,
		children,
		...restProps
	}: LabelProps = $props();
</script>

<div
	bind:this={ref}
	data-slot="dropdown-menu-label"
	data-inset={inset}
	class={className}
	{...restProps}
>
	{@render children?.()}
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

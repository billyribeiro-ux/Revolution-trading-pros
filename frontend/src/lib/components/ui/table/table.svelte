<script lang="ts">
	import type { HTMLTableAttributes } from 'svelte/elements';
	import type { WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLTableAttributes> = $props();
</script>

<div data-slot="table-container" class="relative w-full overflow-x-auto">
	<table
		bind:this={ref}
		data-slot="table"
		class={cn('w-full caption-bottom text-sm', className)}
		{...restProps}
	>
		{@render children?.()}
	</table>
</div>

<style>
	:global([data-slot='table-container']) {
		position: relative;
		inline-size: 100%;
		overflow-x: auto;
	}

	:global([data-slot='table']) {
		inline-size: 100%;
		caption-side: bottom;
		font-size: var(--text-sm);
	}
</style>

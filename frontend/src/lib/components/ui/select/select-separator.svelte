<script lang="ts">
	import type { Separator as SeparatorPrimitive } from 'bits-ui';
	import { Separator } from '$lib/components/ui/separator/index.js';
	
	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: SeparatorPrimitive.RootProps = $props();
</script>

<Separator
	bind:ref
	data-slot="select-separator"
	class={className}
	{...restProps}
/>

<style>
	:global([data-slot='select-separator']) {
		pointer-events: none;
		background-color: var(--border);
		margin-inline: calc(-1 * var(--space-1));
		margin-block: var(--space-1);
		block-size: 1px;
	}
</style>

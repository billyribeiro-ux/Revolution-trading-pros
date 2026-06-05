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
	class={['ui-select-separator', className]}
	{...restProps}
/>

<style>
	:global(.ui-select-separator) {
		width: auto;
		height: 1px;
		margin: 0.25rem -0.25rem;
		pointer-events: none;
		background: var(--border-default, #30363d);
	}
</style>

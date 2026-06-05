<script lang="ts">
	import { Separator as SeparatorPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		'data-slot': dataSlot = 'separator',
		...restProps
	}: SeparatorPrimitive.RootProps = $props();
</script>

<SeparatorPrimitive.Root
	bind:ref
	data-slot={dataSlot}
	class={['ui-separator', className]}
	{...restProps}
/>

<style>
	:global(.ui-separator) {
		flex-shrink: 0;
		background: var(--border-default, #30363d);
	}

	:global(.ui-separator[data-orientation='horizontal']) {
		width: 100%;
		height: 1px;
	}

	:global(.ui-separator[data-orientation='vertical']) {
		width: 1px;
		min-height: 100%;
	}
</style>

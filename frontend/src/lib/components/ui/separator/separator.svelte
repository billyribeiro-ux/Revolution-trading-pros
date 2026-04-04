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
	class={className}
	{...restProps}
/>

<style>
	:global([data-slot='separator']) {
		background-color: var(--border);
		flex-shrink: 0;

		&[data-orientation='horizontal'] {
			block-size: 1px;
			inline-size: 100%;
		}

		&[data-orientation='vertical'] {
			min-block-size: 100%;
			inline-size: 1px;
		}
	}
</style>

<script lang="ts">
	import { Label as LabelPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: LabelPrimitive.RootProps = $props();
</script>

<LabelPrimitive.Root bind:ref data-slot="label" class={['ui-label', className]} {...restProps} />

<style>
	:global(.ui-label) {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 500;
		line-height: 1;
		user-select: none;
	}

	:global(.ui-label[data-disabled='true']),
	:global(.group[data-disabled='true'] .ui-label) {
		pointer-events: none;
		opacity: 0.5;
	}

	:global(.peer:disabled ~ .ui-label) {
		cursor: not-allowed;
		opacity: 0.5;
	}
</style>

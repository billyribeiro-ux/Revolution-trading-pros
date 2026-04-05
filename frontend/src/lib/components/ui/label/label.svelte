<script lang="ts">
	import { Label as LabelPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: LabelPrimitive.RootProps = $props();
</script>

<LabelPrimitive.Root bind:ref data-slot="label" class={className} {...restProps} />

<style>
	:global([data-slot='label']) {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
		line-height: 1;
		font-weight: var(--weight-medium);
		user-select: none;
	}

	:global(:where([data-disabled='true']) [data-slot='label']) {
		pointer-events: none;
		opacity: 0.5;
	}

	:global([data-slot='label']:has(~ :disabled)) {
		cursor: not-allowed;
		opacity: 0.5;
	}
</style>

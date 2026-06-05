<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DialogPrimitive.DescriptionProps = $props();
</script>

<DialogPrimitive.Description
	bind:ref
	data-slot="dialog-description"
	class={['ui-dialog-description', className]}
	{...restProps}
/>

<style>
	:global(.ui-dialog-description) {
		color: var(--dialog-description-color, #64748b);
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dialog-description) {
			--dialog-description-color: #94a3b8;
		}
	}
</style>

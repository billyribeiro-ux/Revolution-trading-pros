<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DialogPrimitive.TitleProps = $props();
</script>

<DialogPrimitive.Title bind:ref data-slot="dialog-title" class={className} {...restProps} />

<style>
	:global([data-slot='dialog-title']) {
		font-size: var(--text-lg);
		line-height: 1;
		font-weight: var(--weight-semibold);
	}
</style>

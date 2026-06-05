<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DialogPrimitive.TitleProps = $props();
</script>

<DialogPrimitive.Title
	bind:ref
	data-slot="dialog-title"
	class={['ui-dialog-title', className]}
	{...restProps}
/>

<style>
	:global(.ui-dialog-title) {
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1;
	}
</style>

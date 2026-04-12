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
	class={className}
	{...restProps}
/>

<style>
	:global([data-slot='dialog-description']) {
		color: var(--muted-foreground);
		font-size: var(--text-sm);
	}
</style>

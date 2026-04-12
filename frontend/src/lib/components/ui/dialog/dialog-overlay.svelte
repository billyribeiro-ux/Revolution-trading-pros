<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DialogPrimitive.OverlayProps = $props();
</script>

<DialogPrimitive.Overlay bind:ref data-slot="dialog-overlay" class={className} {...restProps} />

<style>
	:global([data-slot='dialog-overlay']) {
		position: fixed;
		inset: 0;
		z-index: 50;
		background-color: oklch(0 0 0 / 50%);

		&[data-state='open'] {
			animation: fade-in var(--duration-normal) var(--ease-default);
		}

		&[data-state='closed'] {
			animation: fade-out var(--duration-normal) var(--ease-default);
		}
	}
</style>

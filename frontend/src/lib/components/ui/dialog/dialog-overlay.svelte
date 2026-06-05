<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DialogPrimitive.OverlayProps = $props();
</script>

<DialogPrimitive.Overlay
	bind:ref
	data-slot="dialog-overlay"
	class={['ui-dialog-overlay', className]}
	{...restProps}
/>

<style>
	:global(.ui-dialog-overlay) {
		position: fixed;
		inset: 0;
		z-index: 50;
		background: rgba(0, 0, 0, 0.5);
	}

	:global(.ui-dialog-overlay[data-state='open']) {
		animation: ui-dialog-overlay-in 0.2s ease-out;
	}

	:global(.ui-dialog-overlay[data-state='closed']) {
		animation: ui-dialog-overlay-out 0.16s ease-in;
	}

	@keyframes ui-dialog-overlay-in {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes ui-dialog-overlay-out {
		from {
			opacity: 1;
		}
		to {
			opacity: 0;
		}
	}
</style>

<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';

	let props: DialogPrimitive.OverlayProps = $props();
	let ref = $state<HTMLElement | null>(null);
	let className = $derived(props.class);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, class: __, ...rest } = props;
		return rest;
	});
</script>

<DialogPrimitive.Overlay
	bind:ref
	data-slot="dialog-overlay"
	class={className}
	{...restProps}
/>

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

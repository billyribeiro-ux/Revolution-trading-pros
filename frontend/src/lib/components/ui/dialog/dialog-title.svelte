<script lang="ts">
	import { Dialog as DialogPrimitive } from 'bits-ui';

	let props: DialogPrimitive.TitleProps = $props();
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

<DialogPrimitive.Title bind:ref data-slot="dialog-title" class={className} {...restProps} />

<style>
	:global([data-slot='dialog-title']) {
		font-size: var(--text-lg);
		line-height: 1;
		font-weight: var(--weight-semibold);
	}
</style>

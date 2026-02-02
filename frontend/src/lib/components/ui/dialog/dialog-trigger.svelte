<script lang="ts">
	import { Dialog as DialogPrimitive } from "bits-ui";

	let props: DialogPrimitive.TriggerProps = $props();
	let ref = $state<HTMLElement | null>(props.ref ?? null);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, ...rest } = props;
		return rest;
	});
</script>

<DialogPrimitive.Trigger bind:ref data-slot="dialog-trigger" {...restProps} />

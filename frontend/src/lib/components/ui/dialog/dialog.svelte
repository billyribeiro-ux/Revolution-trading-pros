<script lang="ts">
	import { Dialog as DialogPrimitive } from "bits-ui";

	let props: DialogPrimitive.RootProps = $props();
	let open = $state(props.open ?? false);

	$effect(() => {
		if (props.open !== undefined && props.open !== open) {
			open = props.open;
		}
	});

	let restProps = $derived.by(() => {
		const { open: _, ...rest } = props;
		return rest;
	});
</script>

<DialogPrimitive.Root bind:open {...restProps} />

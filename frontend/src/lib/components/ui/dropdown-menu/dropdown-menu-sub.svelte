<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";

	let props: DropdownMenuPrimitive.SubProps = $props();
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

<DropdownMenuPrimitive.Sub bind:open {...restProps} />

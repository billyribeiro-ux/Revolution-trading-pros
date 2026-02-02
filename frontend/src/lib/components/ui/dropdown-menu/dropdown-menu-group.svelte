<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";

	let props: DropdownMenuPrimitive.GroupProps = $props();
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

<DropdownMenuPrimitive.Group bind:ref data-slot="dropdown-menu-group" {...restProps} />

<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';

	let props: DropdownMenuPrimitive.CheckboxGroupProps = $props();
	let ref = $state<HTMLElement | null>(props.ref ?? null);
	let value = $state<string[]>(props.value ?? []);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});
	$effect(() => {
		if (props.value !== undefined) {
			value = props.value;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, value: __, ...rest } = props;
		return rest;
	});
</script>

<DropdownMenuPrimitive.CheckboxGroup
	bind:ref
	bind:value
	data-slot="dropdown-menu-checkbox-group"
	{...restProps}
/>

<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';

	let props: DropdownMenuPrimitive.RadioGroupProps = $props();
	let ref = $state<HTMLElement | null>(props.ref ?? null);
	let value = $state<string | undefined>(props.value);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});
	$effect(() => {
		if (props.value !== undefined && props.value !== value) {
			value = props.value;
		}
	});

	let restProps = $derived.by(() => {
		const { ref: _, value: __, ...rest } = props;
		return rest;
	});
</script>

<DropdownMenuPrimitive.RadioGroup
	bind:ref
	bind:value
	data-slot="dropdown-menu-radio-group"
	{...restProps}
/>

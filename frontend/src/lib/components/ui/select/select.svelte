<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';

	let props: SelectPrimitive.RootProps = $props();
	let open = $state(props.open ?? false);
	let value = $state(props.value);

	$effect(() => {
		if (props.open !== undefined && props.open !== open) {
			open = props.open;
		}
	});
	$effect(() => {
		if (props.value !== undefined && props.value !== value) {
			value = props.value;
		}
	});

	let restProps = $derived.by(() => {
		const { open: _, value: __, ...rest } = props;
		return rest;
	});
</script>

<SelectPrimitive.Root bind:open bind:value={value as never} {...restProps} />

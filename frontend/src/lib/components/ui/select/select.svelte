<!-- @migration-task Error while migrating Svelte code: Can only bind to an Identifier or MemberExpression or a `{get, set}` pair
https://svelte.dev/e/bind_invalid_expression -->
<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';

	let props: SelectPrimitive.RootProps = $props();
	let open = $state(false);
	let value = $state<string | string[] | undefined>(undefined);

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

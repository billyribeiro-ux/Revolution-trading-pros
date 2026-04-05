<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';
	import type { ComponentProps } from 'svelte';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: ComponentProps<typeof SelectPrimitive.GroupHeading> = $props();
</script>

<SelectPrimitive.GroupHeading
	bind:ref
	data-slot="select-group-heading"
	class={className}
	{...restProps}
>
	{@render children?.()}
</SelectPrimitive.GroupHeading>

<style>
	:global([data-slot='select-group-heading']) {
		color: var(--muted-foreground);
		padding-inline: var(--space-2);
		padding-block: var(--space-1-5);
		font-size: var(--text-xs);
	}
</style>

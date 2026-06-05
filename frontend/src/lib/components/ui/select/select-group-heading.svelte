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
	class={['ui-select-group-heading', className]}
	{...restProps}
>
	{@render children?.()}
</SelectPrimitive.GroupHeading>

<style>
	:global(.ui-select-group-heading) {
		padding: 0.375rem 0.5rem;
		color: var(--select-group-heading-color, #64748b);
		font-size: 0.75rem;
		line-height: 1rem;
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-select-group-heading) {
			--select-group-heading-color: #94a3b8;
		}
	}
</style>

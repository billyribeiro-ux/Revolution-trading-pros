<script lang="ts">
	import { CaretDown as ChevronDownIcon } from 'phosphor-svelte';
	import { Select as SelectPrimitive } from 'bits-ui';
	import type { WithoutChildrenOrChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: WithoutChildrenOrChild<SelectPrimitive.ScrollDownButtonProps> = $props();
</script>

<SelectPrimitive.ScrollDownButton
	bind:ref
	data-slot="select-scroll-down-button"
	class={className}
	{...restProps}
>
	<ChevronDownIcon class="select-scroll-icon" />
</SelectPrimitive.ScrollDownButton>

<style>
	:global([data-slot='select-scroll-down-button']) {
		display: flex;
		cursor: default;
		align-items: center;
		justify-content: center;
		padding-block: var(--space-1);
	}

	:global(.select-scroll-icon) {
		inline-size: 1rem;
		block-size: 1rem;
	}
</style>

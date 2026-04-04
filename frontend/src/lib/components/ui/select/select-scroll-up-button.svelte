<script lang="ts">
	import { CaretUp as ChevronUpIcon } from 'phosphor-svelte';
	import { Select as SelectPrimitive } from 'bits-ui';
	import type { WithoutChildrenOrChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: WithoutChildrenOrChild<SelectPrimitive.ScrollUpButtonProps> = $props();
</script>

<SelectPrimitive.ScrollUpButton
	bind:ref
	data-slot="select-scroll-up-button"
	class={className}
	{...restProps}
>
	<ChevronUpIcon class="select-scroll-icon" />
</SelectPrimitive.ScrollUpButton>

<style>
	:global([data-slot='select-scroll-up-button']) {
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

<script lang="ts">
	import ChevronDownIcon from '@tabler/icons-svelte-runes/icons/chevron-down';
	import { Select as SelectPrimitive } from 'bits-ui';
	import { type WithoutChildrenOrChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: WithoutChildrenOrChild<SelectPrimitive.ScrollDownButtonProps> = $props();
</script>

<SelectPrimitive.ScrollDownButton
	bind:ref
	data-slot="select-scroll-down-button"
	class={['ui-select-scroll-button', className]}
	{...restProps}
>
	<ChevronDownIcon class="ui-select-scroll-button-icon" />
</SelectPrimitive.ScrollDownButton>

<style>
	:global(.ui-select-scroll-button) {
		display: flex;
		align-items: center;
		justify-content: center;
		padding-block: 0.25rem;
		cursor: default;
	}

	:global(.ui-select-scroll-button-icon) {
		width: 1rem;
		height: 1rem;
	}
</style>

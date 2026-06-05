<script lang="ts">
	import ChevronUpIcon from '@tabler/icons-svelte-runes/icons/chevron-up';
	import { Select as SelectPrimitive } from 'bits-ui';
	import { type WithoutChildrenOrChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: WithoutChildrenOrChild<SelectPrimitive.ScrollUpButtonProps> = $props();
</script>

<SelectPrimitive.ScrollUpButton
	bind:ref
	data-slot="select-scroll-up-button"
	class={['ui-select-scroll-button', className]}
	{...restProps}
>
	<ChevronUpIcon class="ui-select-scroll-button-icon" />
</SelectPrimitive.ScrollUpButton>

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

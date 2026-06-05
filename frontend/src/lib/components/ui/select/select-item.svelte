<script lang="ts">
	import CheckIcon from '@tabler/icons-svelte-runes/icons/check';
	import { Select as SelectPrimitive } from 'bits-ui';
	import { type WithoutChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		value,
		label,
		children: childrenProp,
		...restProps
	}: WithoutChild<SelectPrimitive.ItemProps> = $props();
</script>

<SelectPrimitive.Item
	bind:ref
	{value}
	data-slot="select-item"
	class={['ui-select-item', className]}
	{...restProps}
>
	{#snippet children({ selected, highlighted })}
		<span class="ui-select-item-indicator">
			{#if selected}
				<CheckIcon class="ui-select-item-check" />
			{/if}
		</span>
		{#if childrenProp}
			{@render childrenProp({ selected, highlighted })}
		{:else}
			{label || value}
		{/if}
	{/snippet}
</SelectPrimitive.Item>

<style>
	:global(.ui-select-item) {
		position: relative;
		display: flex;
		width: 100%;
		align-items: center;
		gap: 0.5rem;
		border-radius: 0.125rem;
		padding: 0.375rem 2rem 0.375rem 0.5rem;
		color: var(--select-item-foreground, #0f172a);
		font-size: 0.875rem;
		line-height: 1.25rem;
		cursor: default;
		outline: none;
		user-select: none;
	}

	:global(.ui-select-item[data-highlighted]) {
		background: var(--select-item-highlight-background, #f1f5f9);
		color: var(--select-item-highlight-foreground, #0f172a);
	}

	:global(.ui-select-item[data-disabled]) {
		pointer-events: none;
		opacity: 0.5;
	}

	:global(.ui-select-item svg) {
		pointer-events: none;
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
		color: var(--select-item-icon, #64748b);
	}

	:global(.ui-select-item-indicator) {
		position: absolute;
		inset-inline-end: 0.5rem;
		display: flex;
		width: 0.875rem;
		height: 0.875rem;
		align-items: center;
		justify-content: center;
	}

	:global(.ui-select-item-check) {
		width: 1rem;
		height: 1rem;
	}

	:global(.ui-select-item span:last-child) {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-select-item) {
			--select-item-foreground: #f8fafc;
			--select-item-highlight-background: #1e293b;
			--select-item-highlight-foreground: #f8fafc;
			--select-item-icon: #94a3b8;
		}
	}
</style>

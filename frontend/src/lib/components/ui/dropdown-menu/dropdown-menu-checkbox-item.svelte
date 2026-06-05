<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import CheckIcon from '@tabler/icons-svelte-runes/icons/check';
	import MinusIcon from '@tabler/icons-svelte-runes/icons/minus';
	import { type WithoutChildrenOrChild } from '$lib/utils.js';
	import type { Snippet } from 'svelte';

	type CheckboxItemProps = WithoutChildrenOrChild<DropdownMenuPrimitive.CheckboxItemProps> & {
		children?: Snippet;
	};

	let {
		ref = $bindable(null),
		checked = $bindable(false),
		indeterminate = $bindable(false),
		class: className,
		children: childrenProp,
		...restProps
	}: CheckboxItemProps = $props();
</script>

<DropdownMenuPrimitive.CheckboxItem
	bind:ref
	bind:checked
	bind:indeterminate
	data-slot="dropdown-menu-checkbox-item"
	class={['ui-dropdown-menu-check-item', className]}
	{...restProps}
>
	{#snippet children({ checked, indeterminate })}
		<span class="ui-dropdown-menu-item-indicator">
			{#if indeterminate}
				<MinusIcon class="ui-dropdown-menu-check-icon" />
			{:else}
				<CheckIcon
					class={checked
						? 'ui-dropdown-menu-check-icon'
						: 'ui-dropdown-menu-check-icon ui-dropdown-menu-check-icon--hidden'}
				/>
			{/if}
		</span>
		{@render childrenProp?.()}
	{/snippet}
</DropdownMenuPrimitive.CheckboxItem>

<style>
	:global(.ui-dropdown-menu-check-item) {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 0.125rem;
		padding: 0.375rem 0.5rem 0.375rem 2rem;
		color: var(--dropdown-check-foreground, #0f172a);
		font-size: 0.875rem;
		line-height: 1.25rem;
		cursor: default;
		outline: none;
		user-select: none;
	}

	:global(.ui-dropdown-menu-check-item:focus),
	:global(.ui-dropdown-menu-check-item[data-highlighted]) {
		background: var(--dropdown-check-highlight-background, #f1f5f9);
		color: var(--dropdown-check-highlight-foreground, #0f172a);
	}

	:global(.ui-dropdown-menu-check-item[data-disabled]) {
		pointer-events: none;
		opacity: 0.5;
	}

	:global(.ui-dropdown-menu-check-item svg) {
		pointer-events: none;
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
	}

	:global(.ui-dropdown-menu-item-indicator) {
		position: absolute;
		inset-inline-start: 0.5rem;
		display: flex;
		width: 0.875rem;
		height: 0.875rem;
		align-items: center;
		justify-content: center;
		pointer-events: none;
	}

	:global(.ui-dropdown-menu-check-icon--hidden) {
		color: transparent;
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dropdown-menu-check-item) {
			--dropdown-check-foreground: #f8fafc;
			--dropdown-check-highlight-background: #1e293b;
			--dropdown-check-highlight-foreground: #f8fafc;
		}
	}
</style>

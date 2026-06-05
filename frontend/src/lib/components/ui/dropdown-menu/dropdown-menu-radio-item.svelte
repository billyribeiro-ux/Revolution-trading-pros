<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import CircleIcon from '@tabler/icons-svelte-runes/icons/circle';
	import { type WithoutChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children: childrenProp,
		...restProps
	}: WithoutChild<DropdownMenuPrimitive.RadioItemProps> = $props();
</script>

<DropdownMenuPrimitive.RadioItem
	bind:ref
	data-slot="dropdown-menu-radio-item"
	class={['ui-dropdown-menu-radio-item', className]}
	{...restProps}
>
	{#snippet children({ checked })}
		<span class="ui-dropdown-menu-item-indicator">
			{#if checked}
				<CircleIcon class="ui-dropdown-menu-radio-icon" />
			{/if}
		</span>
		{@render childrenProp?.({ checked })}
	{/snippet}
</DropdownMenuPrimitive.RadioItem>

<style>
	:global(.ui-dropdown-menu-radio-item) {
		position: relative;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 0.125rem;
		padding: 0.375rem 0.5rem 0.375rem 2rem;
		color: var(--dropdown-radio-foreground, #0f172a);
		font-size: 0.875rem;
		line-height: 1.25rem;
		cursor: default;
		outline: none;
		user-select: none;
	}

	:global(.ui-dropdown-menu-radio-item:focus),
	:global(.ui-dropdown-menu-radio-item[data-highlighted]) {
		background: var(--dropdown-radio-highlight-background, #f1f5f9);
		color: var(--dropdown-radio-highlight-foreground, #0f172a);
	}

	:global(.ui-dropdown-menu-radio-item[data-disabled]) {
		pointer-events: none;
		opacity: 0.5;
	}

	:global(.ui-dropdown-menu-radio-item svg) {
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

	:global(.ui-dropdown-menu-radio-icon) {
		width: 0.5rem;
		height: 0.5rem;
		fill: currentColor;
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dropdown-menu-radio-item) {
			--dropdown-radio-foreground: #f8fafc;
			--dropdown-radio-highlight-background: #1e293b;
			--dropdown-radio-highlight-foreground: #f8fafc;
		}
	}
</style>

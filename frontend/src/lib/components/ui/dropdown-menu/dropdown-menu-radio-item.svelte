<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import { Circle as CircleIcon } from 'phosphor-svelte';
	import type { WithoutChild } from '$lib/utils.js';

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
	class={className}
	{...restProps}
>
	{#snippet children({ checked })}
		<span class="ddm-radio-indicator">
			{#if checked}
				<CircleIcon class="ddm-radio-dot" weight="fill" />
			{/if}
		</span>
		{@render childrenProp?.({ checked })}
	{/snippet}
</DropdownMenuPrimitive.RadioItem>

<style>
	:global([data-slot='dropdown-menu-radio-item']) {
		position: relative;
		display: flex;
		cursor: default;
		align-items: center;
		gap: var(--space-2);
		border-radius: var(--radius-sm);
		padding-block: var(--space-1-5);
		padding-inline-start: var(--space-8);
		padding-inline-end: var(--space-2);
		font-size: var(--text-sm);
		outline: none;
		user-select: none;

		&:focus {
			background-color: var(--accent);
			color: var(--accent-foreground);
		}

		&[data-disabled] {
			pointer-events: none;
			opacity: 0.5;
		}

		& :global(svg) {
			pointer-events: none;
			flex-shrink: 0;
		}

		& :global(svg:not([class*='size-'])) {
			inline-size: 1rem;
			block-size: 1rem;
		}
	}

	:global(.ddm-radio-indicator) {
		pointer-events: none;
		position: absolute;
		inset-inline-start: var(--space-2);
		display: flex;
		inline-size: 0.875rem;
		block-size: 0.875rem;
		align-items: center;
		justify-content: center;
	}

	:global(.ddm-radio-dot) {
		inline-size: 0.5rem;
		block-size: 0.5rem;
		fill: currentColor;
	}
</style>

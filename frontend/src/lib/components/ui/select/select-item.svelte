<script lang="ts">
	import { Check as CheckIcon } from 'phosphor-svelte';
	import { Select as SelectPrimitive } from 'bits-ui';
	import type { WithoutChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		value,
		label,
		children: childrenProp,
		...restProps
	}: WithoutChild<SelectPrimitive.ItemProps> = $props();
</script>

<SelectPrimitive.Item bind:ref {value} data-slot="select-item" class={className} {...restProps}>
	{#snippet children({ selected, highlighted })}
		<span class="select-item-check">
			{#if selected}
				<CheckIcon class="select-item-check-icon" />
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
	:global([data-slot='select-item']) {
		position: relative;
		display: flex;
		inline-size: 100%;
		cursor: default;
		align-items: center;
		gap: var(--space-2);
		border-radius: var(--radius-sm);
		padding-block: var(--space-1-5);
		padding-inline-start: var(--space-2);
		padding-inline-end: var(--space-8);
		font-size: var(--text-sm);
		outline: none;
		user-select: none;

		&[data-highlighted] {
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

		& :global(svg:not([class*='text-'])) {
			color: var(--muted-foreground);
		}

		& > :global(span:last-child) {
			display: flex;
			align-items: center;
			gap: var(--space-2);
		}
	}

	:global(.select-item-check) {
		position: absolute;
		inset-inline-end: var(--space-2);
		display: flex;
		inline-size: 0.875rem;
		block-size: 0.875rem;
		align-items: center;
		justify-content: center;
	}

	:global(.select-item-check-icon) {
		inline-size: 1rem;
		block-size: 1rem;
	}
</style>

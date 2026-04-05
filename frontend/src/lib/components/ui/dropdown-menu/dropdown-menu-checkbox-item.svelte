<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import { Check as CheckIcon, Minus as MinusIcon } from 'phosphor-svelte';
	import type { WithoutChildrenOrChild } from '$lib/utils.js';
	import type { Snippet } from 'svelte';

	type CheckboxItemProps = WithoutChildrenOrChild<DropdownMenuPrimitive.CheckboxItemProps> & {
		children?: Snippet;
	};

	let props: CheckboxItemProps = $props();
	let ref = $state<HTMLElement | null>(null);
	let checked = $state(false);
	let indeterminate = $state(false);
	let className = $derived(props.class);
	let childrenProp = $derived(props.children);

	$effect(() => {
		if (props.ref !== undefined && props.ref !== ref) {
			ref = props.ref;
		}
	});
	$effect(() => {
		if (props.checked !== undefined && props.checked !== checked) {
			checked = props.checked;
		}
	});
	$effect(() => {
		if (props.indeterminate !== undefined && props.indeterminate !== indeterminate) {
			indeterminate = props.indeterminate;
		}
	});

	let restProps = $derived.by(() => {
		const {
			ref: _,
			checked: __,
			indeterminate: ___,
			class: ____,
			children: _____,
			...rest
		} = props;
		return rest;
	});
</script>

<DropdownMenuPrimitive.CheckboxItem
	bind:ref
	bind:checked
	bind:indeterminate
	data-slot="dropdown-menu-checkbox-item"
	class={className}
	{...restProps}
>
	{#snippet children({ checked, indeterminate })}
		<span class="ddm-check-indicator">
			{#if indeterminate}
				<MinusIcon class="ddm-check-icon" />
			{:else}
				<CheckIcon
					class={['ddm-check-icon', !checked && 'ddm-check-hidden'].filter(Boolean).join(' ')}
				/>
			{/if}
		</span>
		{@render childrenProp?.()}
	{/snippet}
</DropdownMenuPrimitive.CheckboxItem>

<style>
	:global([data-slot='dropdown-menu-checkbox-item']) {
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

	:global(.ddm-check-indicator) {
		pointer-events: none;
		position: absolute;
		inset-inline-start: var(--space-2);
		display: flex;
		inline-size: 0.875rem;
		block-size: 0.875rem;
		align-items: center;
		justify-content: center;
	}

	:global(.ddm-check-icon) {
		inline-size: 1rem;
		block-size: 1rem;
	}

	:global(.ddm-check-hidden) {
		color: transparent;
	}
</style>

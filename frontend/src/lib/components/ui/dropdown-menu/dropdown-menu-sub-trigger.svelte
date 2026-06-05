<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';
	import ChevronRightIcon from '@tabler/icons-svelte-runes/icons/chevron-right';

	type SubTriggerProps = DropdownMenuPrimitive.SubTriggerProps & {
		inset?: boolean;
	};

	let {
		ref = $bindable(null),
		class: className,
		inset,
		children,
		...restProps
	}: SubTriggerProps = $props();
</script>

<DropdownMenuPrimitive.SubTrigger
	bind:ref
	data-slot="dropdown-menu-sub-trigger"
	data-inset={inset}
	class={['ui-dropdown-menu-sub-trigger', className]}
	{...restProps}
>
	{@render children?.()}
	<ChevronRightIcon class="ui-dropdown-menu-sub-trigger-icon" />
</DropdownMenuPrimitive.SubTrigger>

<style>
	:global(.ui-dropdown-menu-sub-trigger) {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		border-radius: 0.125rem;
		padding: 0.375rem 0.5rem;
		color: var(--dropdown-sub-trigger-foreground, #0f172a);
		font-size: 0.875rem;
		line-height: 1.25rem;
		cursor: default;
		outline: none;
		user-select: none;
	}

	:global(.ui-dropdown-menu-sub-trigger[data-inset]) {
		padding-inline-start: 2rem;
	}

	:global(.ui-dropdown-menu-sub-trigger[data-highlighted]),
	:global(.ui-dropdown-menu-sub-trigger[data-state='open']) {
		background: var(--dropdown-sub-trigger-highlight-background, #f1f5f9);
		color: var(--dropdown-sub-trigger-highlight-foreground, #0f172a);
	}

	:global(.ui-dropdown-menu-sub-trigger[data-disabled]) {
		pointer-events: none;
		opacity: 0.5;
	}

	:global(.ui-dropdown-menu-sub-trigger svg) {
		pointer-events: none;
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
		color: var(--dropdown-sub-trigger-icon, #64748b);
	}

	:global(.ui-dropdown-menu-sub-trigger-icon) {
		margin-inline-start: auto;
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dropdown-menu-sub-trigger) {
			--dropdown-sub-trigger-foreground: #f8fafc;
			--dropdown-sub-trigger-highlight-background: #1e293b;
			--dropdown-sub-trigger-highlight-foreground: #f8fafc;
			--dropdown-sub-trigger-icon: #94a3b8;
		}
	}
</style>

<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';
	import ChevronDownIcon from '@tabler/icons-svelte-runes/icons/chevron-down';
	import { type WithoutChild } from '$lib/utils.js';

	type TriggerProps = WithoutChild<SelectPrimitive.TriggerProps> & {
		size?: 'sm' | 'default';
	};

	let {
		ref = $bindable(null),
		class: className,
		size = 'default',
		children,
		...restProps
	}: TriggerProps = $props();
</script>

<SelectPrimitive.Trigger
	bind:ref
	data-slot="select-trigger"
	data-size={size}
	class={['ui-select-trigger', className]}
	{...restProps}
>
	{@render children?.()}
	<ChevronDownIcon class="ui-select-trigger-icon" />
</SelectPrimitive.Trigger>

<style>
	:global(.ui-select-trigger) {
		display: flex;
		width: 100%;
		min-height: 44px;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		border: 1px solid var(--select-trigger-border, #cbd5e1);
		border-radius: 0.375rem;
		background: var(--select-trigger-background, transparent);
		color: var(--select-trigger-foreground, #0f172a);
		box-shadow: 0 1px 2px rgba(15, 23, 42, 0.06);
		padding: 0.75rem 1rem;
		font-size: 1rem;
		line-height: 1.5rem;
		white-space: nowrap;
		cursor: pointer;
		outline: none;
		user-select: none;
		touch-action: manipulation;
		transition:
			border-color 0.18s ease,
			box-shadow 0.18s ease,
			color 0.18s ease,
			background-color 0.18s ease;
	}

	:global(.ui-select-trigger[data-size='sm']) {
		min-height: 36px;
		padding-block: 0.5rem;
		font-size: 0.875rem;
		line-height: 1.25rem;
	}

	:global(.ui-select-trigger[data-placeholder]) {
		color: var(--select-trigger-placeholder, #64748b);
	}

	:global(.ui-select-trigger:hover) {
		background: var(--select-trigger-hover-background, transparent);
	}

	:global(.ui-select-trigger:focus-visible) {
		border-color: var(--select-trigger-ring, #3b82f6);
		box-shadow: 0 0 0 3px var(--select-trigger-ring-shadow, rgba(59, 130, 246, 0.28));
	}

	:global(.ui-select-trigger[aria-invalid='true']) {
		border-color: var(--select-trigger-invalid, #dc2626);
		box-shadow: 0 0 0 3px var(--select-trigger-invalid-shadow, rgba(220, 38, 38, 0.2));
	}

	:global(.ui-select-trigger:disabled) {
		cursor: not-allowed;
		opacity: 0.5;
	}

	:global(.ui-select-trigger [data-slot='select-value']) {
		display: flex;
		min-width: 0;
		align-items: center;
		gap: 0.5rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	:global(.ui-select-trigger svg) {
		pointer-events: none;
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
		color: var(--select-trigger-icon, #64748b);
	}

	:global(.ui-select-trigger-icon) {
		opacity: 0.5;
	}

	@media (min-width: 640px) {
		:global(.ui-select-trigger) {
			width: fit-content;
		}
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-select-trigger) {
			--select-trigger-border: #475569;
			--select-trigger-background: rgba(30, 41, 59, 0.3);
			--select-trigger-foreground: #f8fafc;
			--select-trigger-placeholder: #94a3b8;
			--select-trigger-hover-background: rgba(30, 41, 59, 0.5);
			--select-trigger-ring-shadow: rgba(59, 130, 246, 0.5);
			--select-trigger-invalid-shadow: rgba(220, 38, 38, 0.4);
			--select-trigger-icon: #94a3b8;
		}
	}
</style>

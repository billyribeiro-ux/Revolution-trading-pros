<script lang="ts">
	import { Select as SelectPrimitive } from 'bits-ui';
	import SelectPortal from './select-portal.svelte';
	import SelectScrollUpButton from './select-scroll-up-button.svelte';
	import SelectScrollDownButton from './select-scroll-down-button.svelte';
	import { type WithoutChild } from '$lib/utils.js';
	import type { ComponentProps } from 'svelte';
	import type { WithoutChildrenOrChild } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		sideOffset = 4,
		portalProps,
		children,
		preventScroll = true,
		...restProps
	}: WithoutChild<SelectPrimitive.ContentProps> & {
		portalProps?: WithoutChildrenOrChild<ComponentProps<typeof SelectPortal>>;
	} = $props();
</script>

<SelectPortal {...portalProps}>
	<SelectPrimitive.Content
		bind:ref
		{sideOffset}
		{preventScroll}
		data-slot="select-content"
		class={['ui-select-content', className]}
		{...restProps}
	>
		<SelectScrollUpButton />
		<SelectPrimitive.Viewport class="ui-select-viewport">
			{@render children?.()}
		</SelectPrimitive.Viewport>
		<SelectScrollDownButton />
	</SelectPrimitive.Content>
</SelectPortal>

<style>
	:global(.ui-select-content) {
		--select-content-translate-x: 0;
		--select-content-translate-y: 0;

		position: relative;
		z-index: 50;
		min-width: 8rem;
		max-height: var(--bits-select-content-available-height);
		overflow-x: hidden;
		overflow-y: auto;
		transform: translate(var(--select-content-translate-x), var(--select-content-translate-y));
		transform-origin: var(--bits-select-content-transform-origin);
		border: 1px solid var(--select-content-border, #cbd5e1);
		border-radius: 0.375rem;
		background: var(--select-content-background, #ffffff);
		color: var(--select-content-foreground, #0f172a);
		box-shadow:
			0 10px 15px -3px rgba(15, 23, 42, 0.1),
			0 4px 6px -4px rgba(15, 23, 42, 0.1);
	}

	:global(.ui-select-content[data-side='bottom']) {
		--select-content-translate-y: 0.25rem;
	}

	:global(.ui-select-content[data-side='top']) {
		--select-content-translate-y: -0.25rem;
	}

	:global(.ui-select-content[data-side='left']) {
		--select-content-translate-x: -0.25rem;
	}

	:global(.ui-select-content[data-side='right']) {
		--select-content-translate-x: 0.25rem;
	}

	:global(.ui-select-content[data-state='open']) {
		animation: ui-select-content-in 0.16s ease-out;
	}

	:global(.ui-select-content[data-state='closed']) {
		animation: ui-select-content-out 0.12s ease-in;
	}

	:global(.ui-select-viewport) {
		width: 100%;
		min-width: var(--bits-select-anchor-width);
		height: var(--bits-select-anchor-height);
		scroll-margin-block: 0.25rem;
		padding: 0.25rem;
	}

	@keyframes ui-select-content-in {
		from {
			opacity: 0;
			transform: translate(var(--select-content-translate-x), var(--select-content-translate-y))
				scale(0.95);
		}
		to {
			opacity: 1;
			transform: translate(var(--select-content-translate-x), var(--select-content-translate-y))
				scale(1);
		}
	}

	@keyframes ui-select-content-out {
		from {
			opacity: 1;
			transform: translate(var(--select-content-translate-x), var(--select-content-translate-y))
				scale(1);
		}
		to {
			opacity: 0;
			transform: translate(var(--select-content-translate-x), var(--select-content-translate-y))
				scale(0.95);
		}
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-select-content) {
			--select-content-border: #475569;
			--select-content-background: #020617;
			--select-content-foreground: #f8fafc;
		}
	}
</style>

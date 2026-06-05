<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from 'bits-ui';

	let {
		ref = $bindable(null),
		class: className,
		...restProps
	}: DropdownMenuPrimitive.SubContentProps = $props();
</script>

<DropdownMenuPrimitive.SubContent
	bind:ref
	data-slot="dropdown-menu-sub-content"
	class={['ui-dropdown-menu-sub-content', className]}
	{...restProps}
/>

<style>
	:global(.ui-dropdown-menu-sub-content) {
		--dropdown-sub-translate-x: 0;
		--dropdown-sub-translate-y: 0;

		z-index: 50;
		min-width: 8rem;
		overflow: hidden;
		transform: translate(var(--dropdown-sub-translate-x), var(--dropdown-sub-translate-y));
		transform-origin: var(--bits-dropdown-menu-content-transform-origin);
		border: 1px solid var(--dropdown-sub-border, #cbd5e1);
		border-radius: 0.375rem;
		background: var(--dropdown-sub-background, #ffffff);
		color: var(--dropdown-sub-foreground, #0f172a);
		box-shadow:
			0 20px 25px -5px rgba(15, 23, 42, 0.1),
			0 8px 10px -6px rgba(15, 23, 42, 0.1);
		padding: 0.25rem;
	}

	:global(.ui-dropdown-menu-sub-content[data-side='bottom']) {
		--dropdown-sub-translate-y: 0.25rem;
	}

	:global(.ui-dropdown-menu-sub-content[data-side='top']) {
		--dropdown-sub-translate-y: -0.25rem;
	}

	:global(.ui-dropdown-menu-sub-content[data-side='left']) {
		--dropdown-sub-translate-x: -0.25rem;
	}

	:global(.ui-dropdown-menu-sub-content[data-side='right']) {
		--dropdown-sub-translate-x: 0.25rem;
	}

	:global(.ui-dropdown-menu-sub-content[data-state='open']) {
		animation: ui-dropdown-sub-in 0.16s ease-out;
	}

	:global(.ui-dropdown-menu-sub-content[data-state='closed']) {
		animation: ui-dropdown-sub-out 0.12s ease-in;
	}

	@keyframes ui-dropdown-sub-in {
		from {
			opacity: 0;
			transform: translate(var(--dropdown-sub-translate-x), var(--dropdown-sub-translate-y))
				scale(0.95);
		}
		to {
			opacity: 1;
			transform: translate(var(--dropdown-sub-translate-x), var(--dropdown-sub-translate-y))
				scale(1);
		}
	}

	@keyframes ui-dropdown-sub-out {
		from {
			opacity: 1;
			transform: translate(var(--dropdown-sub-translate-x), var(--dropdown-sub-translate-y))
				scale(1);
		}
		to {
			opacity: 0;
			transform: translate(var(--dropdown-sub-translate-x), var(--dropdown-sub-translate-y))
				scale(0.95);
		}
	}

	@media (prefers-color-scheme: dark) {
		:global(.ui-dropdown-menu-sub-content) {
			--dropdown-sub-border: #475569;
			--dropdown-sub-background: #020617;
			--dropdown-sub-foreground: #f8fafc;
		}
	}
</style>

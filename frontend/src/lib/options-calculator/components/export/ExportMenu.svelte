<script lang="ts">
	import IconCamera from '@tabler/icons-svelte-runes/icons/camera';
	import IconFileSpreadsheet from '@tabler/icons-svelte-runes/icons/file-spreadsheet';
	import IconLink from '@tabler/icons-svelte-runes/icons/link';
	import IconCode from '@tabler/icons-svelte-runes/icons/code';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import { cubicOut } from 'svelte/easing';
	import { on } from 'svelte/events';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { Attachment } from 'svelte/attachments';
	import type { TransitionConfig } from 'svelte/transition';

	interface Props {
		onExportPNG: () => void;
		onExportCSV: () => void;
		onShareLink: () => void;
		onEmbedCode: () => void;
	}

	let { onExportPNG, onExportCSV, onShareLink, onEmbedCode }: Props = $props();

	const menuId = $props.id();
	const triggerId = `${menuId}-trigger`;

	let isOpen = $state(false);

	interface MenuItem {
		icon: typeof IconCamera;
		label: string;
		/** Visible shortcut hint. */
		shortcut: string;
		/** The same shortcut for assistive technology (`aria-keyshortcuts`). */
		keys?: string;
		run: () => void;
	}

	const ITEMS: MenuItem[] = [
		{
			icon: IconCamera,
			label: 'Export as PNG',
			shortcut: '⌘⇧S',
			keys: 'Control+Shift+S Meta+Shift+S',
			run: () => onExportPNG()
		},
		{
			icon: IconFileSpreadsheet,
			label: 'Export Greeks CSV',
			shortcut: '⌘⇧E',
			keys: 'Control+Shift+E Meta+Shift+E',
			run: () => onExportCSV()
		},
		{
			icon: IconLink,
			label: 'Copy Shareable Link',
			shortcut: '⌘⇧L',
			keys: 'Control+Shift+L Meta+Shift+L',
			run: () => onShareLink()
		},
		{
			icon: IconCode,
			label: 'Get Embed Code',
			shortcut: '',
			run: () => onEmbedCode()
		}
	];

	/** Closes the menu and hands focus back to its trigger, as a menu button should. */
	function closeAndRefocus(): void {
		isOpen = false;
		document.getElementById(triggerId)?.focus();
	}

	function choose(item: MenuItem): void {
		// Refocus first, so a dialog opened by the action returns focus to the trigger on close.
		closeAndRefocus();
		item.run();
	}

	/** Closes the menu on any click outside the trigger and the menu. */
	const closeOnOutsideClick: Attachment<HTMLDivElement> = (container) =>
		on(window, 'click', (event) => {
			if (!isOpen) return;
			if (event.target instanceof Node && container.contains(event.target)) return;
			isOpen = false;
		});

	/** Menu-button keyboard model: arrows/Home/End move between items, Escape and Tab close. */
	function handleMenuKeydown(event: KeyboardEvent & { currentTarget: HTMLDivElement }): void {
		const items = Array.from(
			event.currentTarget.querySelectorAll<HTMLButtonElement>('[role="menuitem"]')
		);
		const index = items.findIndex((item) => item === document.activeElement);
		let next: HTMLButtonElement | undefined;

		switch (event.key) {
			case 'ArrowDown':
				next = items[(index + 1) % items.length];
				break;
			case 'ArrowUp':
				next = items[(index - 1 + items.length) % items.length];
				break;
			case 'Home':
				next = items[0];
				break;
			case 'End':
				next = items[items.length - 1];
				break;
			case 'Escape':
				closeAndRefocus();
				break;
			case 'Tab':
				// Let focus move on naturally; the menu just closes behind it.
				isOpen = false;
				return;
			default:
				return;
		}

		event.preventDefault();
		next?.focus();
	}

	/** Opening the menu moves focus to its first item. */
	const focusFirstItem: Attachment<HTMLDivElement> = (menu) => {
		menu.querySelector<HTMLButtonElement>('[role="menuitem"]')?.focus();
	};

	/** Drops the menu in from 8px above at 95% scale while fading in; instant under reduced motion. */
	function dropIn(_node: Element): TransitionConfig {
		return {
			duration: prefersReducedMotion.current ? 0 : 200,
			easing: cubicOut,
			css: (t, u) => `transform: translateY(${u * -8}px) scale(${0.95 + t * 0.05}); opacity: ${t};`
		};
	}
</script>

<div {@attach closeOnOutsideClick} class="relative">
	<button
		id={triggerId}
		type="button"
		onclick={() => (isOpen = !isOpen)}
		class="flex items-center gap-1.5 text-[10px] font-medium px-2.5 py-1.5 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
		style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
		aria-haspopup="menu"
		aria-expanded={isOpen}
		aria-controls={isOpen ? menuId : undefined}
	>
		<IconCamera size={11} aria-hidden="true" />
		Export
		<!-- The icon's `class` prop is a plain string, so no clsx array here. -->
		<IconChevronDown
			size={9}
			class={isOpen
				? 'transition-transform motion-reduce:transition-none rotate-180'
				: 'transition-transform motion-reduce:transition-none'}
			aria-hidden="true"
		/>
	</button>

	{#if isOpen}
		<div
			id={menuId}
			in:dropIn
			{@attach focusFirstItem}
			onkeydown={handleMenuKeydown}
			class="absolute right-0 top-full mt-1.5 w-56 rounded-xl py-1.5 z-50 outline-none"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 12px 40px rgba(0,0,0,0.35);
				backdrop-filter: blur(20px);
			"
			role="menu"
			tabindex="-1"
			aria-labelledby={triggerId}
		>
			{#each ITEMS as item (item.label)}
				{const Icon = item.icon}
				<button
					type="button"
					onclick={() => choose(item)}
					class="w-full flex items-center gap-2.5 px-3.5 py-2.5 text-xs transition-colors motion-reduce:transition-none cursor-pointer"
					style="color: var(--calc-text-secondary);"
					role="menuitem"
					tabindex="-1"
					aria-keyshortcuts={item.keys}
				>
					<Icon size={14} style="color: var(--calc-text-muted);" aria-hidden="true" />
					<span class="flex-1 text-left">{item.label}</span>
					{#if item.shortcut}
						<kbd
							class="text-[9px] px-1.5 py-0.5 rounded"
							style="
								background: var(--calc-surface-hover);
								color: var(--calc-text-muted);
								border: 1px solid var(--calc-border);
								font-family: var(--calc-font-mono);
							">{item.shortcut}</kbd
						>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>

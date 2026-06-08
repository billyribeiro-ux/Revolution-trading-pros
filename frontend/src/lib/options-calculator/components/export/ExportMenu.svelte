<script lang="ts">
	import IconCamera from '@tabler/icons-svelte-runes/icons/camera';
	import IconFileSpreadsheet from '@tabler/icons-svelte-runes/icons/file-spreadsheet';
	import IconLink from '@tabler/icons-svelte-runes/icons/link';
	import IconCode from '@tabler/icons-svelte-runes/icons/code';
	import IconChevronDown from '@tabler/icons-svelte-runes/icons/chevron-down';
	import gsap from 'gsap';

	interface Props {
		onExportPNG: () => void;
		onExportCSV: () => void;
		onShareLink: () => void;
		onEmbedCode: () => void;
	}

	let { onExportPNG, onExportCSV, onShareLink, onEmbedCode }: Props = $props();

	let isOpen = $state(false);
	let menuEl: HTMLDivElement | undefined;
	let containerEl: HTMLDivElement | undefined;
	let chevronClass = $derived(
		['transition-transform', isOpen && 'rotate-180'].filter(Boolean).join(' ')
	);

	function handleAction(action: () => void): void {
		isOpen = false;
		action();
	}

	function handleClickOutside(e: MouseEvent): void {
		if (containerEl && !containerEl.contains(e.target as Node)) {
			isOpen = false;
		}
	}

	function trackContainer(element: HTMLDivElement) {
		containerEl = element;

		return () => {
			if (containerEl === element) {
				containerEl = undefined;
			}
		};
	}

	function animateMenu(element: HTMLDivElement) {
		menuEl = element;
		gsap.fromTo(
			element,
			{ y: -8, opacity: 0, scale: 0.95 },
			{ y: 0, opacity: 1, scale: 1, duration: 0.2, ease: 'power2.out' }
		);

		return () => {
			if (menuEl === element) {
				menuEl = undefined;
			}
		};
	}

	interface MenuItem {
		icon: typeof IconCamera;
		label: string;
		shortcut: string;
		action: () => void;
	}

	const ITEMS: MenuItem[] = [
		{
			icon: IconCamera,
			label: 'Export as PNG',
			shortcut: '\u2318\u21e7S',
			action: () => handleAction(onExportPNG)
		},
		{
			icon: IconFileSpreadsheet,
			label: 'Export Greeks CSV',
			shortcut: '\u2318\u21e7E',
			action: () => handleAction(onExportCSV)
		},
		{
			icon: IconLink,
			label: 'Copy Shareable Link',
			shortcut: '\u2318\u21e7L',
			action: () => handleAction(onShareLink)
		},
		{
			icon: IconCode,
			label: 'Get Embed Code',
			shortcut: '',
			action: () => handleAction(onEmbedCode)
		}
	];
</script>

<svelte:window onclick={handleClickOutside} />

<div {@attach trackContainer} class="relative">
	<button
		onclick={() => (isOpen = !isOpen)}
		class="flex items-center gap-1.5 text-[10px] font-medium px-2.5 py-1.5 rounded-lg transition-colors cursor-pointer"
		style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
		aria-haspopup="true"
		aria-expanded={isOpen}
	>
		<IconCamera size={11} />
		Export
		<IconChevronDown size={9} class={chevronClass} />
	</button>

	{#if isOpen}
		<div
			{@attach animateMenu}
			class="absolute right-0 top-full mt-1.5 w-56 rounded-xl py-1.5 z-50"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 12px 40px rgba(0,0,0,0.35);
				backdrop-filter: blur(20px);
			"
			role="menu"
		>
			{#each ITEMS as item (item.label)}
				{const Icon = item.icon}
				<button
					onclick={item.action}
					class="w-full flex items-center gap-2.5 px-3.5 py-2.5 text-xs transition-colors cursor-pointer"
					style="color: var(--calc-text-secondary);"
					role="menuitem"
				>
					<Icon size={14} style="color: var(--calc-text-muted);" />
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

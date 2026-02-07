<script lang="ts">
	import { Camera, FileSpreadsheet, Link2, Code2, ChevronDown } from '@lucide/svelte';
	import gsap from 'gsap';

	interface Props {
		onExportPNG: () => void;
		onExportCSV: () => void;
		onShareLink: () => void;
		onEmbedCode: () => void;
	}

	let { onExportPNG, onExportCSV, onShareLink, onEmbedCode }: Props = $props();

	let isOpen = $state(false);
	let menuEl: HTMLDivElement | undefined = $state();
	let containerEl: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (isOpen && menuEl) {
			gsap.fromTo(
				menuEl,
				{ y: -8, opacity: 0, scale: 0.95 },
				{ y: 0, opacity: 1, scale: 1, duration: 0.2, ease: 'power2.out' },
			);
		}
	});

	function handleAction(action: () => void): void {
		isOpen = false;
		action();
	}

	function handleClickOutside(e: MouseEvent): void {
		if (containerEl && !containerEl.contains(e.target as Node)) {
			isOpen = false;
		}
	}

	interface MenuItem {
		icon: typeof Camera;
		label: string;
		shortcut: string;
		action: () => void;
	}

	const ITEMS: MenuItem[] = [
		{ icon: Camera, label: 'Export as PNG', shortcut: '\u2318\u21e7S', action: () => handleAction(onExportPNG) },
		{ icon: FileSpreadsheet, label: 'Export Greeks CSV', shortcut: '\u2318\u21e7E', action: () => handleAction(onExportCSV) },
		{ icon: Link2, label: 'Copy Shareable Link', shortcut: '\u2318\u21e7L', action: () => handleAction(onShareLink) },
		{ icon: Code2, label: 'Get Embed Code', shortcut: '', action: () => handleAction(onEmbedCode) },
	];
</script>

<svelte:window onclick={handleClickOutside} />

<div bind:this={containerEl} class="relative">
	<button
		onclick={() => (isOpen = !isOpen)}
		class="flex items-center gap-1.5 text-[10px] font-medium px-2.5 py-1.5 rounded-lg transition-colors cursor-pointer"
		style="background: var(--calc-surface); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
		aria-haspopup="true"
		aria-expanded={isOpen}
	>
		<Camera size={11} />
		Export
		<ChevronDown size={9} class="transition-transform {isOpen ? 'rotate-180' : ''}" />
	</button>

	{#if isOpen}
		<div
			bind:this={menuEl}
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
				{@const Icon = item.icon}
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
							"
						>{item.shortcut}</kbd>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>

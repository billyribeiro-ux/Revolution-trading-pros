<script lang="ts">
	import { Search } from '@lucide/svelte';
	import gsap from 'gsap';
	import type { CalculatorState } from '../../state/calculator.svelte.js';

	interface Props {
		calc: CalculatorState;
	}

	let { calc }: Props = $props();

	let inputEl: HTMLInputElement | undefined = $state();
	let paletteEl: HTMLDivElement | undefined = $state();
	let query = $state('');
	let selectedIndex = $state(0);

	interface CommandItem {
		id: string;
		label: string;
		shortcut: string;
		category: string;
		action: () => void;
	}

	let commands = $derived.by<CommandItem[]>(() => [
		{ id: 'toggle-call', label: 'Toggle Call/Put', shortcut: 'C', category: 'Input', action: () => { calc.optionType = calc.optionType === 'call' ? 'put' : 'call'; } },
		{ id: 'reset', label: 'Reset Inputs', shortcut: '\u2318R', category: 'Input', action: () => calc.resetInputs() },
		{ id: 'toggle-strategy', label: 'Toggle Strategy Mode', shortcut: '', category: 'Input', action: () => { calc.calculatorMode = calc.calculatorMode === 'strategy' ? 'single' : 'strategy'; } },
		{ id: 'toggle-theme', label: 'Toggle Theme', shortcut: '\u2318\u21e7T', category: 'View', action: () => calc.toggleTheme() },
		{ id: 'toggle-education', label: 'Toggle Education Mode', shortcut: 'E', category: 'View', action: () => { calc.educationMode = !calc.educationMode; } },
		{ id: 'toggle-greeks', label: 'Toggle Advanced Greeks', shortcut: 'G', category: 'View', action: () => { calc.showAdvancedGreeks = !calc.showAdvancedGreeks; } },
		{ id: 'toggle-panel', label: 'Toggle Input Panel', shortcut: '\u2318B', category: 'View', action: () => { calc.isInputPanelOpen = !calc.isInputPanelOpen; } },
		{ id: 'tab-payoff', label: 'Payoff Diagram', shortcut: '1', category: 'Navigation', action: () => { calc.activeTab = 'payoff'; } },
		{ id: 'tab-heatmap', label: 'Greeks Heatmap', shortcut: '2', category: 'Navigation', action: () => { calc.activeTab = 'heatmap'; } },
		{ id: 'tab-surface', label: '3D Surface', shortcut: '3', category: 'Navigation', action: () => { calc.activeTab = 'surface'; } },
		{ id: 'tab-mc', label: 'Monte Carlo', shortcut: '4', category: 'Navigation', action: () => { calc.activeTab = 'montecarlo'; } },
		{ id: 'tab-vol', label: 'Vol Smile', shortcut: '5', category: 'Navigation', action: () => { calc.activeTab = 'volsmile'; } },
		{ id: 'tab-theta', label: 'Theta Decay', shortcut: '6', category: 'Navigation', action: () => { calc.activeTab = 'theta'; } },
		{ id: 'tab-sens', label: 'Sensitivity', shortcut: '7', category: 'Navigation', action: () => { calc.activeTab = 'sensitivity'; } },
		{ id: 'tab-chain', label: 'Options Chain', shortcut: '8', category: 'Navigation', action: () => { calc.activeTab = 'chain'; } },
		{ id: 'save-config', label: 'Save Configuration', shortcut: '\u2318S', category: 'Export', action: () => { calc.showSaveModal = true; } },
		{ id: 'open-configs', label: 'Open Saved Configs', shortcut: '', category: 'Export', action: () => { calc.showSavedConfigs = true; } },
		{ id: 'export-png', label: 'Export as PNG', shortcut: '\u2318\u21e7S', category: 'Export', action: () => { calc.showExportPNG = true; } },
		{ id: 'share-link', label: 'Copy Shareable Link', shortcut: '\u2318\u21e7L', category: 'Export', action: () => { calc.showShareLink = true; } },
		{ id: 'embed-code', label: 'Get Embed Code', shortcut: '', category: 'Export', action: () => { calc.showEmbedCode = true; } },
		{ id: 'shortcuts', label: 'Show Keyboard Shortcuts', shortcut: '?', category: 'Help', action: () => { calc.showShortcutsHelp = true; } },
	]);

	let filtered = $derived(
		query.trim()
			? commands.filter(
					(cmd) =>
						cmd.label.toLowerCase().includes(query.toLowerCase()) ||
						cmd.category.toLowerCase().includes(query.toLowerCase()),
				)
			: commands,
	);

	// Build a flat index map for keyboard navigation
	let flatIndexMap = $derived.by(() => {
		const map = new Map<string, number>();
		let i = 0;
		for (const cmd of filtered) {
			map.set(cmd.id, i++);
		}
		return map;
	});

	$effect(() => {
		if (calc.showCommandPalette) {
			query = '';
			selectedIndex = 0;
			requestAnimationFrame(() => {
				inputEl?.focus();
				if (paletteEl) {
					gsap.fromTo(
						paletteEl,
						{ y: -20, opacity: 0 },
						{ y: 0, opacity: 1, duration: 0.2, ease: 'power2.out' },
					);
				}
			});
		}
	});

	// Reset selectedIndex when filtered list changes
	$effect(() => {
		if (filtered.length > 0 && selectedIndex >= filtered.length) {
			selectedIndex = 0;
		}
	});

	function execute(cmd: CommandItem): void {
		calc.showCommandPalette = false;
		cmd.action();
	}

	function handleKeydown(e: KeyboardEvent): void {
		if (!calc.showCommandPalette) return;

		if (e.key === 'Escape') {
			e.preventDefault();
			calc.showCommandPalette = false;
			return;
		}
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, filtered.length - 1);
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			const cmd = filtered[selectedIndex];
			if (cmd) execute(cmd);
		}
	}

	// Group filtered commands by category for display
	let grouped = $derived.by(() => {
		const groups: Record<string, CommandItem[]> = {};
		for (const cmd of filtered) {
			if (!groups[cmd.category]) groups[cmd.category] = [];
			groups[cmd.category].push(cmd);
		}
		return groups;
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showCommandPalette}
	<div
		class="fixed inset-0 z-[9999] flex items-start justify-center pt-[15vh]"
		role="dialog"
		aria-modal="true"
		aria-label="Command Palette"
	>
		<!-- Backdrop -->
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.5); backdrop-filter: blur(6px);"
			onclick={() => (calc.showCommandPalette = false)}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Palette -->
		<div
			bind:this={paletteEl}
			class="relative z-10 w-full max-w-lg rounded-2xl overflow-hidden"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 24px 80px rgba(0,0,0,0.5);
			"
		>
			<!-- Search Input -->
			<div
				class="flex items-center gap-2.5 px-4 py-3"
				style="border-bottom: 1px solid var(--calc-border);"
			>
				<Search size={14} style="color: var(--calc-text-muted);" />
				<input
					bind:this={inputEl}
					bind:value={query}
					type="text"
					placeholder="Type a command..."
					class="flex-1 text-sm bg-transparent outline-none"
					style="color: var(--calc-text);"
				/>
				<kbd
					class="text-[9px] px-1.5 py-0.5 rounded"
					style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
				>ESC</kbd>
			</div>

			<!-- Results -->
			<div class="max-h-80 overflow-y-auto calc-scrollbar">
				{#each Object.entries(grouped) as [category, cmds] (category)}
					<div class="px-3 pt-2.5 pb-1">
						<span
							class="text-[9px] uppercase tracking-wider font-medium"
							style="color: var(--calc-text-muted);"
						>{category}</span>
					</div>
					{#each cmds as cmd (cmd.id)}
						{@const idx = flatIndexMap.get(cmd.id) ?? -1}
						<button
							onclick={() => execute(cmd)}
							class="w-full flex items-center gap-2.5 px-4 py-2 text-xs cursor-pointer transition-colors"
							style="color: var(--calc-text-secondary); background: {idx === selectedIndex ? 'var(--calc-accent-glow)' : 'transparent'};"
						>
							<span class="flex-1 text-left">{cmd.label}</span>
							{#if cmd.shortcut}
								<kbd
									class="text-[9px] px-1.5 py-0.5 rounded"
									style="
										background: var(--calc-surface-hover);
										color: var(--calc-text-muted);
										border: 1px solid var(--calc-border);
										font-family: var(--calc-font-mono);
									"
								>{cmd.shortcut}</kbd>
							{/if}
						</button>
					{/each}
				{/each}

				{#if filtered.length === 0}
					<div class="px-4 py-8 text-center">
						<p class="text-xs" style="color: var(--calc-text-muted);">No commands found</p>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

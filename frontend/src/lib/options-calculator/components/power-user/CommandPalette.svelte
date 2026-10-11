<script lang="ts">
	import { trapFocus } from '../../utils/focus-trap.js';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import { quadOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import { shortcutDisplay } from './shortcuts.js';
	import { getCalculator } from '../../state/context.js';
	import type { ChartTab } from '../../engine/types.js';

	interface CommandItem {
		id: string;
		label: string;
		/** Key label shown beside the command; empty when it has no shortcut. */
		shortcut: string;
		category: string;
		action: () => void;
	}

	interface CommandGroup {
		category: string;
		commands: CommandItem[];
	}

	const calc = getCalculator();
	const uid = $props.id();

	function showTab(tab: ChartTab): () => void {
		return () => {
			calc.activeTab = tab;
		};
	}

	/**
	 * A static list: every action reads `calc` when it runs, so nothing here is reactive.
	 * Shortcut labels come from `shortcuts.ts`, the same table the key handler uses.
	 */
	const COMMANDS: readonly CommandItem[] = [
		{
			id: 'toggle-call',
			label: 'Toggle Call/Put',
			shortcut: shortcutDisplay('toggle-call-put'),
			category: 'Input',
			action: () => {
				calc.optionType = calc.optionType === 'call' ? 'put' : 'call';
			}
		},
		{
			id: 'reset',
			label: 'Reset Inputs',
			shortcut: shortcutDisplay('reset-inputs'),
			category: 'Input',
			action: () => calc.resetInputs()
		},
		{
			id: 'toggle-strategy',
			label: 'Toggle Strategy Mode',
			shortcut: '',
			category: 'Input',
			action: () => {
				calc.calculatorMode = calc.calculatorMode === 'strategy' ? 'single' : 'strategy';
			}
		},
		{
			id: 'toggle-theme',
			label: 'Toggle Theme',
			shortcut: shortcutDisplay('toggle-theme'),
			category: 'View',
			action: () => calc.toggleTheme()
		},
		{
			id: 'toggle-education',
			label: 'Toggle Education Mode',
			shortcut: shortcutDisplay('toggle-education'),
			category: 'View',
			action: () => {
				calc.educationMode = !calc.educationMode;
			}
		},
		{
			id: 'toggle-greeks',
			label: 'Toggle Advanced Greeks',
			shortcut: shortcutDisplay('toggle-advanced-greeks'),
			category: 'View',
			action: () => {
				calc.showAdvancedGreeks = !calc.showAdvancedGreeks;
			}
		},
		{
			id: 'toggle-panel',
			label: 'Toggle Input Panel',
			shortcut: shortcutDisplay('toggle-panel'),
			category: 'View',
			action: () => {
				calc.isInputPanelOpen = !calc.isInputPanelOpen;
			}
		},
		{
			id: 'tab-payoff',
			label: 'Payoff Diagram',
			shortcut: shortcutDisplay('tab-payoff'),
			category: 'Navigation',
			action: showTab('payoff')
		},
		{
			id: 'tab-heatmap',
			label: 'Greeks Heatmap',
			shortcut: shortcutDisplay('tab-heatmap'),
			category: 'Navigation',
			action: showTab('heatmap')
		},
		{
			id: 'tab-surface',
			label: '3D Surface',
			shortcut: shortcutDisplay('tab-surface'),
			category: 'Navigation',
			action: showTab('surface')
		},
		{
			id: 'tab-mc',
			label: 'Monte Carlo',
			shortcut: shortcutDisplay('tab-montecarlo'),
			category: 'Navigation',
			action: showTab('montecarlo')
		},
		{
			id: 'tab-vol',
			label: 'Vol Smile',
			shortcut: shortcutDisplay('tab-volsmile'),
			category: 'Navigation',
			action: showTab('volsmile')
		},
		{
			id: 'tab-theta',
			label: 'Theta Decay',
			shortcut: shortcutDisplay('tab-theta'),
			category: 'Navigation',
			action: showTab('theta')
		},
		{
			id: 'tab-sens',
			label: 'Sensitivity',
			shortcut: shortcutDisplay('tab-sensitivity'),
			category: 'Navigation',
			action: showTab('sensitivity')
		},
		{
			id: 'tab-chain',
			label: 'Options Chain',
			shortcut: shortcutDisplay('tab-chain'),
			category: 'Navigation',
			action: showTab('chain')
		},
		{
			id: 'save-config',
			label: 'Save Configuration',
			shortcut: shortcutDisplay('save-config'),
			category: 'Export',
			action: () => {
				calc.showSaveModal = true;
			}
		},
		{
			id: 'open-configs',
			label: 'Open Saved Configs',
			shortcut: '',
			category: 'Export',
			action: () => {
				calc.showSavedConfigs = true;
			}
		},
		{
			id: 'export-png',
			label: 'Export as PNG',
			shortcut: shortcutDisplay('export-png'),
			category: 'Export',
			action: () => {
				calc.showExportPNG = true;
			}
		},
		{
			id: 'share-link',
			label: 'Copy Shareable Link',
			shortcut: shortcutDisplay('share-link'),
			category: 'Export',
			action: () => {
				calc.showShareLink = true;
			}
		},
		{
			id: 'embed-code',
			label: 'Get Embed Code',
			shortcut: '',
			category: 'Export',
			action: () => {
				calc.showEmbedCode = true;
			}
		},
		{
			id: 'shortcuts',
			label: 'Show Keyboard Shortcuts',
			shortcut: shortcutDisplay('help'),
			category: 'Help',
			action: () => {
				calc.showShortcutsHelp = true;
			}
		}
	];

	let query = $state('');
	let selectedIndex = $state(0);

	/** Matching commands grouped by category, in the order they are listed. */
	const groups = $derived.by<CommandGroup[]>(() => {
		const needle = query.trim().toLowerCase();
		const result: CommandGroup[] = [];
		for (const command of COMMANDS) {
			const matches =
				!needle ||
				command.label.toLowerCase().includes(needle) ||
				command.category.toLowerCase().includes(needle);
			if (!matches) continue;
			const group = result.find((candidate) => candidate.category === command.category);
			if (group) group.commands.push(command);
			else result.push({ category: command.category, commands: [command] });
		}
		return result;
	});

	/** Matches in on-screen order: the arrow keys walk this list. */
	const ordered = $derived(groups.flatMap((group) => group.commands));
	const activeIndex = $derived(
		ordered.length === 0 ? -1 : Math.min(selectedIndex, ordered.length - 1)
	);
	/** `undefined` when nothing matches (`activeIndex` is then -1 on an empty list). */
	const activeCommand = $derived(ordered.at(activeIndex));

	function optionId(command: CommandItem): string {
		return `${uid}-option-${command.id}`;
	}

	/** Every way the palette closes goes through here, so each opening starts blank. */
	function close(): void {
		calc.showCommandPalette = false;
		query = '';
		selectedIndex = 0;
	}

	function execute(command: CommandItem): void {
		close();
		command.action();
	}

	/** Enter/Space on an option that holds focus (after a click) runs that option. */
	function handleOptionKeydown(event: KeyboardEvent, command: CommandItem): void {
		if (event.key !== 'Enter' && event.key !== ' ') return;
		event.preventDefault();
		execute(command);
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (!calc.showCommandPalette) return;

		switch (event.key) {
			case 'Escape':
				event.preventDefault();
				close();
				break;
			case 'ArrowDown':
				event.preventDefault();
				selectedIndex = Math.min(activeIndex + 1, ordered.length - 1);
				break;
			case 'ArrowUp':
				event.preventDefault();
				selectedIndex = Math.max(activeIndex - 1, 0);
				break;
			case 'Enter':
				event.preventDefault();
				if (activeCommand) execute(activeCommand);
				break;
		}
	}

	/** Keeps the highlighted option visible while the arrow keys walk a long list. */
	const keepInView: Attachment<HTMLElement> = (option) => {
		option.scrollIntoView({ block: 'nearest' });
	};

	/** Focuses the search field on open and hands focus back to the opener on close. */
	const focusSearch: Attachment<HTMLInputElement> = (input) => {
		const opener = document.activeElement;
		input.focus();
		return () => {
			if (opener instanceof HTMLElement && opener.isConnected) opener.focus();
		};
	};
</script>

<svelte:window onkeydown={handleKeydown} />

{#if calc.showCommandPalette}
	<div
		{@attach trapFocus}
		class="fixed inset-0 z-[9999] flex items-start justify-center pt-[15vh]"
		role="dialog"
		aria-modal="true"
		aria-label="Command Palette"
	>
		<!-- Backdrop -->
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.5); backdrop-filter: blur(6px);"
			onclick={close}
			aria-label="Close"
			tabindex={-1}
		></button>

		<!-- Palette -->
		<div
			in:fly={{ y: -20, duration: prefersReducedMotion.current ? 0 : 200, easing: quadOut }}
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
				<IconSearch size={14} style="color: var(--calc-text-muted);" aria-hidden="true" />
				<input
					{@attach focusSearch}
					bind:value={query}
					oninput={() => (selectedIndex = 0)}
					type="text"
					role="combobox"
					aria-label="Search commands"
					aria-autocomplete="list"
					aria-expanded={ordered.length > 0}
					aria-controls="{uid}-listbox"
					aria-activedescendant={activeCommand ? optionId(activeCommand) : undefined}
					autocomplete="off"
					spellcheck="false"
					placeholder="Type a command..."
					class="flex-1 text-sm bg-transparent outline-none"
					style="color: var(--calc-text);"
				/>
				<kbd
					class="text-[9px] px-1.5 py-0.5 rounded"
					style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					>ESC</kbd
				>
			</div>

			<!-- Results -->
			<div class="max-h-80 overflow-y-auto calc-scrollbar">
				<div id="{uid}-listbox" role="listbox" aria-label="Commands">
					{#each groups as group (group.category)}
						<div role="group" aria-labelledby="{uid}-group-{group.category}">
							<div id="{uid}-group-{group.category}" role="presentation" class="px-3 pt-2.5 pb-1">
								<span
									class="text-[9px] uppercase tracking-wider font-medium"
									style="color: var(--calc-text-muted);">{group.category}</span
								>
							</div>
							{#each group.commands as command (command.id)}
								{const active = $derived(command === activeCommand)}
								<div
									{@attach active && keepInView}
									id={optionId(command)}
									role="option"
									aria-selected={active}
									tabindex="-1"
									onclick={() => execute(command)}
									onkeydown={(event) => handleOptionKeydown(event, command)}
									class="w-full flex items-center gap-2.5 px-4 py-2 text-xs cursor-pointer transition-colors motion-reduce:transition-none"
									style:color="var(--calc-text-secondary)"
									style:background={active ? 'var(--calc-accent-glow)' : 'transparent'}
								>
									<span class="flex-1 text-left">{command.label}</span>
									{#if command.shortcut}
										<kbd
											class="text-[9px] px-1.5 py-0.5 rounded"
											style="
												background: var(--calc-surface-hover);
												color: var(--calc-text-muted);
												border: 1px solid var(--calc-border);
												font-family: var(--calc-font-mono);
											">{command.shortcut}</kbd
										>
									{/if}
								</div>
							{/each}
						</div>
					{/each}
				</div>

				{#if ordered.length === 0}
					<div class="px-4 py-8 text-center">
						<p class="text-xs" style="color: var(--calc-text-muted);">No commands found</p>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

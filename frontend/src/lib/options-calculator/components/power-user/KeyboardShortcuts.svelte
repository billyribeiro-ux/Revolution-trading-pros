<script lang="ts">
	import { trapFocus } from '../../utils/focus-trap.js';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconKeyboard from '@tabler/icons-svelte-runes/icons/keyboard';
	import { backOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import type { Attachment } from 'svelte/attachments';
	import {
		SHORTCUTS,
		SHORTCUT_GROUPS,
		TAB_SHORTCUTS,
		isInputFocused,
		isTabShortcut,
		matchesShortcut,
		type ShortcutId,
		type TabShortcutId
	} from './shortcuts.js';
	import { getCalculator } from '../../state/context.js';

	interface Props {
		/** Runs the Greeks CSV export (⌘⇧E); the exporter lives in the parent. */
		onExportCSV?: () => void;
	}

	let { onExportCSV }: Props = $props();

	const calc = getCalculator();
	const uid = $props.id();

	/**
	 * What every non-tab shortcut does. Typed as a complete record, so adding a `ShortcutId`
	 * without an action is a type error rather than a silently dead key.
	 */
	const ACTIONS: Record<Exclude<ShortcutId, TabShortcutId>, () => void> = {
		'command-palette': () => {
			calc.showCommandPalette = true;
		},
		'toggle-theme': () => calc.toggleTheme(),
		'toggle-panel': () => {
			calc.isInputPanelOpen = !calc.isInputPanelOpen;
		},
		'toggle-call-put': () => {
			calc.optionType = calc.optionType === 'call' ? 'put' : 'call';
		},
		'reset-inputs': () => {
			calc.resetInputs();
			calc.addToast('info', 'Inputs reset to defaults');
		},
		'export-png': () => {
			calc.showExportPNG = true;
		},
		'export-csv': () => onExportCSV?.(),
		'share-link': () => {
			calc.showShareLink = true;
		},
		'save-config': () => {
			calc.showSaveModal = true;
		},
		'toggle-education': () => {
			calc.educationMode = !calc.educationMode;
			calc.addToast('info', calc.educationMode ? 'Education mode on' : 'Education mode off');
		},
		'toggle-advanced-greeks': () => {
			calc.showAdvancedGreeks = !calc.showAdvancedGreeks;
		},
		help: () => {
			calc.showShortcutsHelp = true;
		}
	};

	function dispatch(id: ShortcutId): void {
		if (isTabShortcut(id)) calc.activeTab = TAB_SHORTCUTS[id];
		else ACTIONS[id]();
	}

	function closeHelp(): void {
		calc.showShortcutsHelp = false;
	}

	function handleKeydown(event: KeyboardEvent): void {
		// The help overlay is modal: Escape closes it and nothing fires underneath.
		if (calc.showShortcutsHelp) {
			if (event.key === 'Escape') {
				event.preventDefault();
				closeHelp();
			}
			return;
		}
		// The command palette handles its own keys while open.
		if (calc.showCommandPalette) return;

		const shortcut = SHORTCUTS.find((candidate) => matchesShortcut(event, candidate));
		if (!shortcut) return;

		// Single-key shortcuts (no modifier) must not fire while typing in a field.
		const hasModifier = event.ctrlKey || event.metaKey || event.altKey;
		if (!hasModifier && isInputFocused(event)) return;

		event.preventDefault();
		dispatch(shortcut.id);
	}

	/** Moves focus into the overlay when it opens and hands it back to the opener on close. */
	const focusDialog: Attachment<HTMLElement> = (dialog) => {
		const opener = document.activeElement;
		dialog.focus();
		return () => {
			if (opener instanceof HTMLElement && opener.isConnected) opener.focus();
		};
	};
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Keyboard Shortcuts Help Overlay -->
{#if calc.showShortcutsHelp}
	<div
		{@attach trapFocus}
		{@attach focusDialog}
		class="fixed inset-0 z-[9998] flex items-center justify-center p-4 outline-none"
		role="dialog"
		aria-modal="true"
		aria-labelledby="{uid}-title"
		tabindex="-1"
	>
		<button
			type="button"
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.5); backdrop-filter: blur(4px);"
			onclick={closeHelp}
			aria-label="Close"
			tabindex={-1}
		></button>

		<div
			in:scale={{
				start: 0.92,
				duration: prefersReducedMotion.current ? 0 : 250,
				easing: backOut
			}}
			class="relative z-10 w-full max-w-lg rounded-2xl p-5 flex flex-col gap-4 max-h-[80vh] overflow-y-auto calc-scrollbar"
			style="
				background: var(--calc-surface);
				border: 1px solid var(--calc-border);
				box-shadow: 0 24px 80px rgba(0,0,0,0.5);
			"
		>
			<!-- Header -->
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<IconKeyboard size={16} style="color: var(--calc-accent);" aria-hidden="true" />
					<h3
						id="{uid}-title"
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>
						Keyboard Shortcuts
					</h3>
				</div>
				<button
					type="button"
					onclick={closeHelp}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<IconX size={16} aria-hidden="true" />
				</button>
			</div>

			<!-- Shortcut Groups -->
			{#each SHORTCUT_GROUPS as group (group.category)}
				<div class="flex flex-col gap-1.5">
					<h4
						class="text-[10px] uppercase tracking-wider font-semibold"
						style="color: var(--calc-text-muted);"
					>
						{group.label}
					</h4>

					{#each group.shortcuts as shortcut (shortcut.id)}
						<div
							class="flex items-center justify-between py-1.5 px-2 rounded-lg"
							style="background: var(--calc-surface-hover);"
						>
							<span class="text-xs" style="color: var(--calc-text-secondary);">
								{shortcut.description}
							</span>
							<kbd
								class="text-[10px] px-2 py-0.5 rounded font-medium"
								style="
									background: var(--calc-surface);
									color: var(--calc-text);
									border: 1px solid var(--calc-border);
									font-family: var(--calc-font-mono);
									min-width: 28px;
									text-align: center;
								">{shortcut.display}</kbd
							>
						</div>
					{/each}
				</div>
			{/each}

			<p class="text-[10px] text-center" style="color: var(--calc-text-muted);">
				Single-key shortcuts are disabled when typing in input fields.
			</p>
		</div>
	</div>
{/if}

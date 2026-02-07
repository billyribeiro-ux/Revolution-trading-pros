<script lang="ts">
	import { X, Keyboard } from '@lucide/svelte';
	import gsap from 'gsap';
	import { SHORTCUTS, CATEGORY_LABELS, matchesShortcut, isInputFocused } from './shortcuts.js';
	import type { CalculatorState } from '../../state/calculator.svelte.js';
	import type { ChartTab, ShortcutDef } from '../../engine/types.js';

	interface Props {
		calc: CalculatorState;
		onExportCSV?: () => void;
	}

	let { calc, onExportCSV }: Props = $props();

	let overlayEl: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (calc.showShortcutsHelp && overlayEl) {
			gsap.fromTo(
				overlayEl,
				{ scale: 0.92, opacity: 0 },
				{ scale: 1, opacity: 1, duration: 0.25, ease: 'back.out(1.5)' },
			);
		}
	});

	// Group shortcuts by category for the help overlay
	let grouped = $derived.by(() => {
		const groups: Record<string, ShortcutDef[]> = {};
		for (const s of SHORTCUTS) {
			if (!groups[s.category]) groups[s.category] = [];
			groups[s.category].push(s);
		}
		return groups;
	});

	const TAB_MAP: Record<string, ChartTab> = {
		'tab-payoff': 'payoff',
		'tab-heatmap': 'heatmap',
		'tab-surface': 'surface',
		'tab-montecarlo': 'montecarlo',
		'tab-volsmile': 'volsmile',
		'tab-theta': 'theta',
		'tab-sensitivity': 'sensitivity',
		'tab-chain': 'chain',
	};

	/**
	 * Dispatch a shortcut action by ID.
	 * Returns true if the shortcut was handled.
	 */
	function dispatch(id: string): boolean {
		switch (id) {
			case 'command-palette':
				calc.showCommandPalette = true;
				return true;
			case 'toggle-theme':
				calc.toggleTheme();
				return true;
			case 'toggle-panel':
				calc.isInputPanelOpen = !calc.isInputPanelOpen;
				return true;
			case 'toggle-call-put':
				calc.optionType = calc.optionType === 'call' ? 'put' : 'call';
				return true;
			case 'reset-inputs':
				calc.resetInputs();
				calc.addToast('info', 'Inputs reset to defaults');
				return true;
			case 'export-png':
				calc.showExportPNG = true;
				return true;
			case 'export-csv':
				onExportCSV?.();
				return true;
			case 'share-link':
				calc.showShareLink = true;
				return true;
			case 'save-config':
				calc.showSaveModal = true;
				return true;
			case 'toggle-education':
				calc.educationMode = !calc.educationMode;
				calc.addToast('info', calc.educationMode ? 'Education mode on' : 'Education mode off');
				return true;
			case 'toggle-advanced-greeks':
				calc.showAdvancedGreeks = !calc.showAdvancedGreeks;
				return true;
			case 'help':
				calc.showShortcutsHelp = true;
				return true;
			default:
				// Tab shortcuts
				if (id in TAB_MAP) {
					calc.activeTab = TAB_MAP[id];
					return true;
				}
				return false;
		}
	}

	function handleGlobalKeydown(e: KeyboardEvent): void {
		// Don't process shortcuts when modals are open (except Escape)
		if (calc.showCommandPalette || calc.showShortcutsHelp) return;

		// Find matching shortcut
		for (const shortcut of SHORTCUTS) {
			if (matchesShortcut(e, shortcut)) {
				// Single-key shortcuts (no modifier) should not fire in input fields
				const hasModifier = e.ctrlKey || e.metaKey || e.altKey;
				if (!hasModifier && isInputFocused(e)) continue;

				e.preventDefault();
				dispatch(shortcut.id);
				return;
			}
		}
	}
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<!-- Keyboard Shortcuts Help Overlay -->
{#if calc.showShortcutsHelp}
	<div
		class="fixed inset-0 z-[9998] flex items-center justify-center p-4"
		role="dialog"
		aria-modal="true"
		aria-label="Keyboard Shortcuts"
	>
		<button
			class="absolute inset-0 cursor-default"
			style="background: rgba(0,0,0,0.5); backdrop-filter: blur(4px);"
			onclick={() => (calc.showShortcutsHelp = false)}
			aria-label="Close"
			tabindex={-1}
		></button>

		<div
			bind:this={overlayEl}
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
					<Keyboard size={16} style="color: var(--calc-accent);" />
					<h3
						class="text-sm font-semibold"
						style="color: var(--calc-text); font-family: var(--calc-font-display);"
					>Keyboard Shortcuts</h3>
				</div>
				<button
					onclick={() => (calc.showShortcutsHelp = false)}
					class="cursor-pointer p-1 rounded-md"
					style="color: var(--calc-text-muted);"
					aria-label="Close"
				>
					<X size={16} />
				</button>
			</div>

			<!-- Shortcut Groups -->
			{#each Object.entries(grouped) as [category, shortcuts] (category)}
				<div class="flex flex-col gap-1.5">
					<h4
						class="text-[10px] uppercase tracking-wider font-semibold"
						style="color: var(--calc-text-muted);"
					>{CATEGORY_LABELS[category as keyof typeof CATEGORY_LABELS] ?? category}</h4>

					{#each shortcuts as shortcut (shortcut.id)}
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
								"
							>{shortcut.display}</kbd>
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

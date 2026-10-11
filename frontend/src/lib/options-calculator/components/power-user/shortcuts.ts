// ============================================================
// KEYBOARD SHORTCUTS — Definitions & Matching Utilities
// Pure data and helpers shared by KeyboardShortcuts (dispatch + help
// overlay) and CommandPalette (shortcut labels). No DOM access at import.
// ============================================================

import type { ChartTab, ShortcutDef } from '../../engine/types.js';

/** Every calculator shortcut id, so dispatch tables can be checked for completeness. */
export type ShortcutId =
	| 'command-palette'
	| 'tab-payoff'
	| 'tab-heatmap'
	| 'tab-surface'
	| 'tab-montecarlo'
	| 'tab-volsmile'
	| 'tab-theta'
	| 'tab-sensitivity'
	| 'tab-chain'
	| 'toggle-call-put'
	| 'reset-inputs'
	| 'export-png'
	| 'export-csv'
	| 'share-link'
	| 'save-config'
	| 'toggle-theme'
	| 'toggle-panel'
	| 'toggle-advanced-greeks'
	| 'toggle-education'
	| 'help';

/** A `ShortcutDef` whose id is one of the calculator's known shortcuts. */
export interface CalculatorShortcut extends ShortcutDef {
	id: ShortcutId;
}

export type ShortcutCategory = ShortcutDef['category'];

export const SHORTCUTS: readonly CalculatorShortcut[] = [
	// ── Navigation ──────────────────────────────────────
	{
		id: 'command-palette',
		keys: ['ctrl+k', 'meta+k'],
		display: '⌘K',
		description: 'Open command palette',
		category: 'navigation'
	},
	{
		id: 'tab-payoff',
		keys: ['1'],
		display: '1',
		description: 'Payoff diagram',
		category: 'navigation'
	},
	{
		id: 'tab-heatmap',
		keys: ['2'],
		display: '2',
		description: 'Greeks Heatmap',
		category: 'navigation'
	},
	{
		id: 'tab-surface',
		keys: ['3'],
		display: '3',
		description: '3D Surface',
		category: 'navigation'
	},
	{
		id: 'tab-montecarlo',
		keys: ['4'],
		display: '4',
		description: 'Monte Carlo',
		category: 'navigation'
	},
	{
		id: 'tab-volsmile',
		keys: ['5'],
		display: '5',
		description: 'Vol Smile',
		category: 'navigation'
	},
	{
		id: 'tab-theta',
		keys: ['6'],
		display: '6',
		description: 'Theta Decay',
		category: 'navigation'
	},
	{
		id: 'tab-sensitivity',
		keys: ['7'],
		display: '7',
		description: 'Sensitivity',
		category: 'navigation'
	},
	{
		id: 'tab-chain',
		keys: ['8'],
		display: '8',
		description: 'Options Chain',
		category: 'navigation'
	},

	// ── Input ───────────────────────────────────────────
	{
		id: 'toggle-call-put',
		keys: ['c'],
		display: 'C',
		description: 'Toggle Call/Put',
		category: 'input'
	},
	{
		id: 'reset-inputs',
		keys: ['ctrl+r', 'meta+r'],
		display: '⌘R',
		description: 'Reset all inputs',
		category: 'input'
	},

	// ── Export ──────────────────────────────────────────
	{
		id: 'export-png',
		keys: ['ctrl+shift+s', 'meta+shift+s'],
		display: '⌘⇧S',
		description: 'Export as PNG',
		category: 'export'
	},
	{
		id: 'export-csv',
		keys: ['ctrl+shift+e', 'meta+shift+e'],
		display: '⌘⇧E',
		description: 'Export Greeks CSV',
		category: 'export'
	},
	{
		id: 'share-link',
		keys: ['ctrl+shift+l', 'meta+shift+l'],
		display: '⌘⇧L',
		description: 'Copy shareable link',
		category: 'export'
	},
	{
		id: 'save-config',
		keys: ['ctrl+s', 'meta+s'],
		display: '⌘S',
		description: 'Save configuration',
		category: 'export'
	},

	// ── View ────────────────────────────────────────────
	{
		id: 'toggle-theme',
		keys: ['ctrl+shift+t', 'meta+shift+t'],
		display: '⌘⇧T',
		description: 'Toggle theme',
		category: 'view'
	},
	{
		id: 'toggle-panel',
		keys: ['ctrl+b', 'meta+b'],
		display: '⌘B',
		description: 'Toggle input panel',
		category: 'view'
	},
	{
		id: 'toggle-advanced-greeks',
		keys: ['g'],
		display: 'G',
		description: 'Toggle advanced Greeks',
		category: 'view'
	},

	// ── General ─────────────────────────────────────────
	{
		id: 'toggle-education',
		keys: ['e'],
		display: 'E',
		description: 'Toggle education mode',
		category: 'general'
	},
	{
		id: 'help',
		// `KeyboardEvent.key` is already the shifted character ("?", not "/"), so Shift+/
		// normalises to "shift+?"; the bare "?" covers layouts that type it unshifted.
		keys: ['shift+?', '?'],
		display: '?',
		description: 'Show keyboard shortcuts',
		category: 'general'
	}
];

/** The chart tab each navigation shortcut switches to. */
export const TAB_SHORTCUTS = {
	'tab-payoff': 'payoff',
	'tab-heatmap': 'heatmap',
	'tab-surface': 'surface',
	'tab-montecarlo': 'montecarlo',
	'tab-volsmile': 'volsmile',
	'tab-theta': 'theta',
	'tab-sensitivity': 'sensitivity',
	'tab-chain': 'chain'
} as const satisfies Partial<Record<ShortcutId, ChartTab>>;

export type TabShortcutId = keyof typeof TAB_SHORTCUTS;

export function isTabShortcut(id: ShortcutId): id is TabShortcutId {
	return id in TAB_SHORTCUTS;
}

/** Category display names for the help overlay */
export const CATEGORY_LABELS: Record<ShortcutCategory, string> = {
	navigation: 'Navigation',
	input: 'Input',
	export: 'Export',
	view: 'View',
	general: 'General'
};

const CATEGORY_ORDER: readonly ShortcutCategory[] = [
	'navigation',
	'input',
	'export',
	'view',
	'general'
];

export interface ShortcutGroup {
	category: ShortcutCategory;
	label: string;
	shortcuts: CalculatorShortcut[];
}

/** Shortcuts grouped by category, in display order, for the help overlay. */
export const SHORTCUT_GROUPS: readonly ShortcutGroup[] = CATEGORY_ORDER.map((category) => ({
	category,
	label: CATEGORY_LABELS[category],
	shortcuts: SHORTCUTS.filter((shortcut) => shortcut.category === category)
})).filter((group) => group.shortcuts.length > 0);

/** The key label shown for a shortcut (e.g. "⌘⇧T"), for menus that mirror the shortcuts. */
export function shortcutDisplay(id: ShortcutId): string {
	return SHORTCUTS.find((shortcut) => shortcut.id === id)?.display ?? '';
}

/**
 * Convert a KeyboardEvent into a normalized key string.
 * Example: Ctrl+Shift+S → "ctrl+shift+s"
 */
export function eventToKey(event: KeyboardEvent): string {
	const parts: string[] = [];
	if (event.ctrlKey) parts.push('ctrl');
	if (event.metaKey) parts.push('meta');
	if (event.shiftKey) parts.push('shift');
	if (event.altKey) parts.push('alt');

	const key = event.key.toLowerCase();
	if (!['control', 'meta', 'shift', 'alt'].includes(key)) {
		parts.push(key);
	}

	return parts.join('+');
}

/**
 * Check if a keyboard event matches a shortcut definition.
 */
export function matchesShortcut(event: KeyboardEvent, shortcut: ShortcutDef): boolean {
	return shortcut.keys.includes(eventToKey(event));
}

/**
 * Check if the event target is an input-like element
 * (where we should NOT fire single-key shortcuts).
 */
export function isInputFocused(event: KeyboardEvent): boolean {
	const target = event.target;
	if (!(target instanceof HTMLElement)) return false;
	return (
		target instanceof HTMLInputElement ||
		target instanceof HTMLTextAreaElement ||
		target instanceof HTMLSelectElement ||
		target.isContentEditable
	);
}

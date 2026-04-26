// ============================================================
// KEYBOARD SHORTCUTS — Definitions & Matching Utilities
// ============================================================

import type { ShortcutDef } from '../../engine/types.js';

export const SHORTCUTS: ShortcutDef[] = [
	// ── Navigation ──────────────────────────────────────
	{
		id: 'command-palette',
		keys: ['ctrl+k', 'meta+k'],
		display: '\u2318K',
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
		display: '\u2318R',
		description: 'Reset all inputs',
		category: 'input'
	},

	// ── Export ──────────────────────────────────────────
	{
		id: 'export-png',
		keys: ['ctrl+shift+s', 'meta+shift+s'],
		display: '\u2318\u21e7S',
		description: 'Export as PNG',
		category: 'export'
	},
	{
		id: 'export-csv',
		keys: ['ctrl+shift+e', 'meta+shift+e'],
		display: '\u2318\u21e7E',
		description: 'Export Greeks CSV',
		category: 'export'
	},
	{
		id: 'share-link',
		keys: ['ctrl+shift+l', 'meta+shift+l'],
		display: '\u2318\u21e7L',
		description: 'Copy shareable link',
		category: 'export'
	},
	{
		id: 'save-config',
		keys: ['ctrl+s', 'meta+s'],
		display: '\u2318S',
		description: 'Save configuration',
		category: 'export'
	},

	// ── View ────────────────────────────────────────────
	{
		id: 'toggle-theme',
		keys: ['ctrl+shift+t', 'meta+shift+t'],
		display: '\u2318\u21e7T',
		description: 'Toggle theme',
		category: 'view'
	},
	{
		id: 'toggle-panel',
		keys: ['ctrl+b', 'meta+b'],
		display: '\u2318B',
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
		keys: ['shift+/'],
		display: '?',
		description: 'Show keyboard shortcuts',
		category: 'general'
	}
];

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
	const pressed = eventToKey(event);
	return shortcut.keys.includes(pressed);
}

/**
 * Check if the event target is an input-like element
 * (where we should NOT fire single-key shortcuts).
 */
export function isInputFocused(event: KeyboardEvent): boolean {
	const target = event.target as HTMLElement | null;
	if (!target) return false;
	const tag = target.tagName.toLowerCase();
	return tag === 'input' || tag === 'textarea' || tag === 'select' || target.isContentEditable;
}

/** Category display names for the help overlay */
export const CATEGORY_LABELS: Record<ShortcutDef['category'], string> = {
	navigation: 'Navigation',
	input: 'Input',
	export: 'Export',
	view: 'View',
	general: 'General'
};

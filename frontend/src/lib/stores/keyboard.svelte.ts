/**
 * Keyboard Shortcuts Store - Svelte 5 Runes
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Global keyboard shortcut management with:
 * - Configurable shortcuts
 * - Context-aware activation
 * - Shortcut groups
 * - Help modal integration
 *
 * @version 2.0.0 - Svelte 5 Runes Migration (February 2026)
 */

import { browser } from '$app/environment';
import { goto } from '$app/navigation';

// ═══════════════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════════════

export interface KeyboardShortcut {
	id: string;
	keys: string[];
	description: string;
	action: () => void;
	category: string;
	global?: boolean;
	enabled?: boolean;
}

interface KeyboardState {
	shortcuts: KeyboardShortcut[];
	isHelpOpen: boolean;
	activeContext: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Default Shortcuts
// ═══════════════════════════════════════════════════════════════════════════════

const defaultShortcuts: Omit<KeyboardShortcut, 'action'>[] = [
	// Navigation
	{
		id: 'goto-dashboard',
		keys: ['g', 'd'],
		description: 'Go to Dashboard',
		category: 'Navigation',
		global: true
	},
	{
		id: 'goto-analytics',
		keys: ['g', 'a'],
		description: 'Go to Analytics',
		category: 'Navigation',
		global: true
	},
	{
		id: 'goto-blog',
		keys: ['g', 'b'],
		description: 'Go to Blog',
		category: 'Navigation',
		global: true
	},
	{
		id: 'goto-crm',
		keys: ['g', 'c'],
		description: 'Go to CRM',
		category: 'Navigation',
		global: true
	},
	{
		id: 'goto-email',
		keys: ['g', 'e'],
		description: 'Go to Email',
		category: 'Navigation',
		global: true
	},
	{
		id: 'goto-media',
		keys: ['g', 'm'],
		description: 'Go to Media',
		category: 'Navigation',
		global: true
	},
	{
		id: 'goto-settings',
		keys: ['g', 's'],
		description: 'Go to Settings',
		category: 'Navigation',
		global: true
	},

	// Actions
	{
		id: 'search',
		keys: ['Meta', 'k'],
		description: 'Open Command Palette',
		category: 'Actions',
		global: true
	},
	{
		id: 'search-alt',
		keys: ['Control', 'k'],
		description: 'Open Command Palette',
		category: 'Actions',
		global: true
	},
	{
		id: 'notifications',
		keys: ['Meta', 'n'],
		description: 'Toggle Notifications',
		category: 'Actions',
		global: true
	},
	{
		id: 'refresh',
		keys: ['Meta', 'r'],
		description: 'Refresh Data',
		category: 'Actions',
		global: true
	},
	{
		id: 'new-item',
		keys: ['n'],
		description: 'Create New Item',
		category: 'Actions',
		global: true
	},
	{
		id: 'save',
		keys: ['Meta', 's'],
		description: 'Save Changes',
		category: 'Actions',
		global: true
	},

	// UI
	{
		id: 'help',
		keys: ['Shift', '?'],
		description: 'Show Shortcuts',
		category: 'Help',
		global: true
	},
	{
		id: 'escape',
		keys: ['Escape'],
		description: 'Close Modal/Panel',
		category: 'UI',
		global: true
	},
	{
		id: 'toggle-sidebar',
		keys: ['Meta', 'b'],
		description: 'Toggle Sidebar',
		category: 'UI',
		global: true
	},
	{
		id: 'toggle-theme',
		keys: ['Meta', 'Shift', 't'],
		description: 'Toggle Theme',
		category: 'UI',
		global: true
	}
];

// ═══════════════════════════════════════════════════════════════════════════════
// Svelte 5 Reactive State
// ═══════════════════════════════════════════════════════════════════════════════

const initialState: KeyboardState = {
	shortcuts: [],
	isHelpOpen: false,
	activeContext: 'global'
};

let storeState = $state<KeyboardState>(initialState);

// Derived values
const shortcutsValue = $derived(storeState.shortcuts);
const isHelpOpenValue = $derived(storeState.isHelpOpen);
const activeContextValue = $derived(storeState.activeContext);

// Key sequence tracking
let keySequence: string[] = [];
let sequenceTimeout: ReturnType<typeof setTimeout> | null = null;
let isInitialized = false;

// ═══════════════════════════════════════════════════════════════════════════════
// Event Handlers
// ═══════════════════════════════════════════════════════════════════════════════

function handleKeyDown(event: KeyboardEvent) {
	const key = event.key;

	// When the user is typing in an input / textarea / contenteditable, only
	// Escape and platform-modifier shortcuts (Cmd / Ctrl) should fire. Plain
	// letter sequences like "g" then "d" must NOT trigger navigation while
	// the user is composing prose. Bug pre-fix: typing "good" anywhere with a
	// focused textarea fired goto-dashboard.
	const target = event.target as HTMLElement | null;
	const inEditableField =
		!!target &&
		(target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable);

	// Build modifier string
	const modifiers: string[] = [];
	if (event.metaKey) modifiers.push('Meta');
	if (event.ctrlKey) modifiers.push('Control');
	if (event.shiftKey) modifiers.push('Shift');
	if (event.altKey) modifiers.push('Alt');

	// 1. Modifier-based shortcuts (Cmd+K, Cmd+S, etc.) — work even in inputs.
	if (modifiers.length > 0) {
		const shortcut = storeState.shortcuts.find((s) => {
			if (!s.enabled) return false;
			const sModifiers = s.keys.filter((k) => ['Meta', 'Control', 'Shift', 'Alt'].includes(k));
			const sKey = s.keys.find((k) => !['Meta', 'Control', 'Shift', 'Alt'].includes(k));
			return (
				sModifiers.every((m) => modifiers.includes(m)) &&
				modifiers.every((m) => sModifiers.includes(m)) &&
				sKey?.toLowerCase() === key.toLowerCase()
			);
		});

		if (shortcut) {
			event.preventDefault();
			shortcut.action();
			return;
		}
	}

	// 2. Escape — fires regardless of focus context, no preventDefault so the
	//    browser's native Escape handling (e.g. cancelling form composition)
	//    still works.
	if (key === 'Escape') {
		const shortcut = storeState.shortcuts.find((s) => s.id === 'escape' && s.enabled);
		if (shortcut) shortcut.action();
		return;
	}

	// 3. Plain key sequences (g→d, g→a). Suppressed inside editable fields so
	//    typing in inputs doesn't navigate the app.
	if (inEditableField) return;

	if (modifiers.length === 0 && key.length === 1) {
		if (sequenceTimeout) clearTimeout(sequenceTimeout);

		keySequence.push(key.toLowerCase());
		sequenceTimeout = setTimeout(() => {
			keySequence = [];
		}, 500);

		const shortcut = storeState.shortcuts.find((s) => {
			if (!s.enabled) return false;
			if (s.keys.some((k) => ['Meta', 'Control', 'Shift', 'Alt'].includes(k))) return false;
			return (
				s.keys.length === keySequence.length &&
				s.keys.every((k, i) => k.toLowerCase() === keySequence[i])
			);
		});

		if (shortcut) {
			event.preventDefault();
			keySequence = [];
			if (sequenceTimeout) clearTimeout(sequenceTimeout);
			shortcut.action();
		}
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exported Store (Svelte 5 Pattern)
// ═══════════════════════════════════════════════════════════════════════════════

export const keyboard = {
	// Getters
	get state() {
		return storeState;
	},
	get shortcuts() {
		return shortcutsValue;
	},
	get isHelpOpen() {
		return isHelpOpenValue;
	},
	get activeContext() {
		return activeContextValue;
	},

	/**
	 * Initialize keyboard shortcuts
	 */
	init(customActions: Record<string, () => void> = {}) {
		if (!browser || isInitialized) return;

		// Build shortcuts with actions
		const shortcuts: KeyboardShortcut[] = defaultShortcuts.map((s) => ({
			...s,
			enabled: true,
			action:
				customActions[s.id] ||
				(() => {
					// Default navigation actions
					if (s.id.startsWith('goto-')) {
						const path = s.id.replace('goto-', '');
						const pathMap: Record<string, string> = {
							dashboard: '/admin',
							analytics: '/admin/analytics',
							blog: '/admin/blog',
							crm: '/admin/crm',
							email: '/admin/email/campaigns',
							media: '/admin/media',
							settings: '/admin/settings'
						};
						if (pathMap[path]) {
							goto(pathMap[path]);
						}
					}
				})
		}));

		// FIX-2026-04-26: assign the rune ONCE, no spread-read of itself.
		// The previous `storeState = { ...storeState, shortcuts }` read+wrote the
		// same rune in one statement; if a caller wraps init() inside a tracked
		// context (e.g. $effect), that's a self-dependency and the depth guard
		// will trip on the first cleanup. We rebuild the snapshot from known
		// initial fields so there's zero read of `storeState` here.
		// Old: storeState = { ...storeState, shortcuts };
		storeState = {
			shortcuts,
			isHelpOpen: false,
			activeContext: 'global'
		};

		// Add event listener
		window.addEventListener('keydown', handleKeyDown);
		isInitialized = true;
	},

	/**
	 * Destroy keyboard shortcuts
	 */
	destroy() {
		if (!browser) return;
		window.removeEventListener('keydown', handleKeyDown);
		// FIX-2026-04-26: do NOT reset isInitialized here. The gate exists to keep
		// init() idempotent for the lifetime of the module. Clearing it on destroy
		// means any caller that follows the `init()` … `destroy()` pattern from
		// inside a tracked context (the canonical bug) will re-enter init on the
		// next reactive flush. The listener detach above is the only thing
		// destroy() actually needs to do.
		// Old: isInitialized = false;
	},

	/**
	 * Register a custom shortcut
	 */
	register(shortcut: KeyboardShortcut) {
		storeState = {
			...storeState,
			shortcuts: [
				...storeState.shortcuts.filter((s) => s.id !== shortcut.id),
				{ ...shortcut, enabled: true }
			]
		};
	},

	/**
	 * Register an action handler for an existing shortcut ID
	 */
	registerAction(id: string, action: () => void) {
		storeState = {
			...storeState,
			shortcuts: storeState.shortcuts.map((s) =>
				s.id === id ? { ...s, action, enabled: true } : s
			)
		};
	},

	/**
	 * Unregister a shortcut
	 */
	unregister(id: string) {
		storeState = {
			...storeState,
			shortcuts: storeState.shortcuts.filter((s) => s.id !== id)
		};
	},

	/**
	 * Enable/disable a shortcut
	 */
	setEnabled(id: string, enabled: boolean) {
		storeState = {
			...storeState,
			shortcuts: storeState.shortcuts.map((s) => (s.id === id ? { ...s, enabled } : s))
		};
	},

	/**
	 * Set active context
	 */
	setContext(context: string) {
		storeState = { ...storeState, activeContext: context };
	},

	/**
	 * Toggle help modal
	 */
	toggleHelp() {
		storeState = { ...storeState, isHelpOpen: !storeState.isHelpOpen };
	},

	/**
	 * Open help modal
	 */
	openHelp() {
		storeState = { ...storeState, isHelpOpen: true };
	},

	/**
	 * Close help modal
	 */
	closeHelp() {
		storeState = { ...storeState, isHelpOpen: false };
	},

	/**
	 * Get formatted key display
	 */
	formatKeys(keys: string[]): string {
		const keyMap: Record<string, string> = {
			Meta: navigator?.platform?.includes('Mac') ? '⌘' : 'Ctrl',
			Control: 'Ctrl',
			Shift: '⇧',
			Alt: navigator?.platform?.includes('Mac') ? '⌥' : 'Alt',
			Escape: 'Esc',
			ArrowUp: '↑',
			ArrowDown: '↓',
			ArrowLeft: '←',
			ArrowRight: '→'
		};

		return keys.map((k) => keyMap[k] || k.toUpperCase()).join(' + ');
	}
};

export default keyboard;

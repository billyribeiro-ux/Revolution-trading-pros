/**
 * Keyboard Shortcuts Hook
 * ═══════════════════════════════════════════════════════════════════════════
 * Composable logic for keyboard shortcuts with automatic cleanup
 */

// ============================================================================
// Types
// ============================================================================

export interface Shortcut {
	/** The key to listen for (e.g., 's', 'Enter', 'Escape') */
	key: string;
	/** Whether Ctrl/Cmd key must be pressed */
	ctrl?: boolean;
	/** Whether Shift key must be pressed */
	shift?: boolean;
	/** Whether Alt/Option key must be pressed */
	alt?: boolean;
	/** Whether Meta (Cmd on Mac) key must be pressed */
	meta?: boolean;
	/** Handler function to call when shortcut is triggered */
	handler: (event: KeyboardEvent) => void;
	/** Description of what the shortcut does */
	description?: string;
	/** Whether to prevent default browser behavior */
	preventDefault?: boolean;
	/** Whether to stop event propagation */
	stopPropagation?: boolean;
	/** Whether shortcut is currently enabled */
	enabled?: boolean;
	/** Scope/context for the shortcut */
	scope?: string;
	/** Priority (higher = checked first) */
	priority?: number;
}

export interface ShortcutGroup {
	name: string;
	shortcuts: Shortcut[];
}

export interface KeyboardShortcutsOptions {
	/** Shortcuts to register */
	shortcuts: Shortcut[];
	/** Whether shortcuts are enabled */
	enabled?: boolean;
	/** Target element (defaults to document) */
	target?: Document | HTMLElement;
	/** Event type to listen for */
	eventType?: 'keydown' | 'keyup';
	/** Active scope filter */
	scope?: string;
	/** Whether to capture in capture phase */
	capture?: boolean;
}

// ============================================================================
// Constants
// ============================================================================

const MODIFIER_KEYS = ['Control', 'Shift', 'Alt', 'Meta'];

// Normalize key names across browsers
const KEY_ALIASES: Record<string, string> = {
	'esc': 'Escape',
	'return': 'Enter',
	'space': ' ',
	'spacebar': ' ',
	'up': 'ArrowUp',
	'down': 'ArrowDown',
	'left': 'ArrowLeft',
	'right': 'ArrowRight',
	'del': 'Delete',
	'ins': 'Insert'
};

// ============================================================================
// Utility Functions
// ============================================================================

function normalizeKey(key: string): string {
	const lowercaseKey = key.toLowerCase();
	return KEY_ALIASES[lowercaseKey] || key;
}

function isMac(): boolean {
	if (typeof navigator === 'undefined') return false;
	return /Mac|iPod|iPhone|iPad/.test(navigator.platform);
}

function matchesShortcut(event: KeyboardEvent, shortcut: Shortcut): boolean {
	// Check if shortcut is enabled
	if (shortcut.enabled === false) return false;

	// Normalize and compare key
	const eventKey = event.key.length === 1 ? event.key.toLowerCase() : event.key;
	const shortcutKey = normalizeKey(shortcut.key);

	if (eventKey !== shortcutKey && eventKey.toLowerCase() !== shortcutKey.toLowerCase()) {
		return false;
	}

	// Check modifiers
	const ctrlMatch = shortcut.ctrl
		? (isMac() ? event.metaKey : event.ctrlKey)
		: !(isMac() ? event.metaKey && !shortcut.meta : event.ctrlKey);

	const shiftMatch = shortcut.shift ? event.shiftKey : !event.shiftKey;
	const altMatch = shortcut.alt ? event.altKey : !event.altKey;

	// Meta key check (explicit meta requirement)
	const metaMatch = shortcut.meta
		? event.metaKey
		: shortcut.ctrl
			? true // If ctrl is required, we already handled meta above
			: !event.metaKey;

	return ctrlMatch && shiftMatch && altMatch && metaMatch;
}

function formatShortcut(shortcut: Shortcut): string {
	const parts: string[] = [];
	const mac = isMac();

	if (shortcut.ctrl) {
		parts.push(mac ? '⌘' : 'Ctrl');
	}
	if (shortcut.meta && !shortcut.ctrl) {
		parts.push(mac ? '⌘' : 'Win');
	}
	if (shortcut.alt) {
		parts.push(mac ? '⌥' : 'Alt');
	}
	if (shortcut.shift) {
		parts.push(mac ? '⇧' : 'Shift');
	}

	// Format key
	let keyDisplay = shortcut.key;
	const keyDisplayMap: Record<string, string> = {
		'ArrowUp': '↑',
		'ArrowDown': '↓',
		'ArrowLeft': '←',
		'ArrowRight': '→',
		'Enter': '↵',
		'Escape': 'Esc',
		'Backspace': '⌫',
		'Delete': 'Del',
		'Tab': '⇥',
		' ': 'Space'
	};

	keyDisplay = keyDisplayMap[keyDisplay] || keyDisplay.toUpperCase();
	parts.push(keyDisplay);

	return parts.join(mac ? '' : '+');
}

// ============================================================================
// Hook Implementation
// ============================================================================

export function useKeyboardShortcuts(options: KeyboardShortcutsOptions) {
	const {
		shortcuts: initialShortcuts,
		enabled: initialEnabled = true,
		target,
		eventType = 'keydown',
		scope: initialScope,
		capture = false
	} = options;

	// Reactive state
	let shortcuts = $state<Shortcut[]>([...initialShortcuts]);
	let enabled = $state(initialEnabled);
	let activeScope = $state<string | undefined>(initialScope);
	let lastTriggered = $state<Shortcut | null>(null);
	let listenerAttached = $state(false);

	// ========================================================================
	// Event Handler
	// ========================================================================

	function handleKeyDown(event: KeyboardEvent): void {
		// Skip if shortcuts are disabled
		if (!enabled) return;

		// Skip if target is an input element (unless explicitly handled)
		const targetElement = event.target as HTMLElement;
		const isInputElement =
			targetElement.tagName === 'INPUT' ||
			targetElement.tagName === 'TEXTAREA' ||
			targetElement.tagName === 'SELECT' ||
			targetElement.isContentEditable;

		// Skip modifier-only keypresses
		if (MODIFIER_KEYS.includes(event.key)) return;

		// Sort shortcuts by priority (higher first)
		const sortedShortcuts = [...shortcuts].sort(
			(a, b) => (b.priority || 0) - (a.priority || 0)
		);

		for (const shortcut of sortedShortcuts) {
			// Check scope
			if (activeScope && shortcut.scope && shortcut.scope !== activeScope) {
				continue;
			}

			// Check if shortcut matches
			if (matchesShortcut(event, shortcut)) {
				// For input elements, only trigger shortcuts with modifiers
				if (isInputElement && !shortcut.ctrl && !shortcut.alt && !shortcut.meta) {
					continue;
				}

				// Prevent default if specified
				if (shortcut.preventDefault !== false) {
					event.preventDefault();
				}

				// Stop propagation if specified
				if (shortcut.stopPropagation) {
					event.stopPropagation();
				}

				// Call handler
				shortcut.handler(event);
				lastTriggered = shortcut;

				// Only trigger first matching shortcut
				return;
			}
		}
	}

	// ========================================================================
	// Lifecycle Management
	// ========================================================================

	function attach(): void {
		if (listenerAttached) return;

		const targetElement = target || (typeof document !== 'undefined' ? document : null);
		if (!targetElement) return;

		targetElement.addEventListener(eventType, handleKeyDown as EventListener, { capture });
		listenerAttached = true;
	}

	function detach(): void {
		if (!listenerAttached) return;

		const targetElement = target || (typeof document !== 'undefined' ? document : null);
		if (!targetElement) return;

		targetElement.removeEventListener(eventType, handleKeyDown as EventListener, { capture });
		listenerAttached = false;
	}

	// Auto-attach on creation and cleanup
	$effect(() => {
		attach();
		return () => detach();
	});

	// ========================================================================
	// Management Methods
	// ========================================================================

	function addShortcut(shortcut: Shortcut): void {
		// Check for duplicates
		const exists = shortcuts.some(
			(s) =>
				s.key === shortcut.key &&
				s.ctrl === shortcut.ctrl &&
				s.shift === shortcut.shift &&
				s.alt === shortcut.alt &&
				s.meta === shortcut.meta &&
				s.scope === shortcut.scope
		);

		if (!exists) {
			shortcuts = [...shortcuts, shortcut];
		}
	}

	function removeShortcut(shortcut: Partial<Shortcut>): void {
		shortcuts = shortcuts.filter(
			(s) =>
				!(
					s.key === shortcut.key &&
					(shortcut.ctrl === undefined || s.ctrl === shortcut.ctrl) &&
					(shortcut.shift === undefined || s.shift === shortcut.shift) &&
					(shortcut.alt === undefined || s.alt === shortcut.alt) &&
					(shortcut.scope === undefined || s.scope === shortcut.scope)
				)
		);
	}

	function updateShortcut(key: string, updates: Partial<Shortcut>): void {
		shortcuts = shortcuts.map((s) => (s.key === key ? { ...s, ...updates } : s));
	}

	function setEnabled(value: boolean): void {
		enabled = value;
	}

	function setScope(scope: string | undefined): void {
		activeScope = scope;
	}

	function clearShortcuts(): void {
		shortcuts = [];
	}

	function resetShortcuts(): void {
		shortcuts = [...initialShortcuts];
	}

	// ========================================================================
	// Query Methods
	// ========================================================================

	function getShortcut(key: string): Shortcut | undefined {
		return shortcuts.find((s) => s.key === key);
	}

	function getShortcutsByScope(scope: string): Shortcut[] {
		return shortcuts.filter((s) => s.scope === scope);
	}

	function getShortcutsGroupedByScope(): Record<string, Shortcut[]> {
		const groups: Record<string, Shortcut[]> = {};

		for (const shortcut of shortcuts) {
			const scope = shortcut.scope || 'global';
			if (!groups[scope]) {
				groups[scope] = [];
			}
			groups[scope].push(shortcut);
		}

		return groups;
	}

	function hasConflict(shortcut: Shortcut): boolean {
		return shortcuts.some(
			(s) =>
				s.key === shortcut.key &&
				s.ctrl === shortcut.ctrl &&
				s.shift === shortcut.shift &&
				s.alt === shortcut.alt &&
				s.meta === shortcut.meta &&
				(s.scope === shortcut.scope || !s.scope || !shortcut.scope)
		);
	}

	// ========================================================================
	// Return API
	// ========================================================================

	return {
		// Reactive state (via getters)
		get shortcuts() {
			return shortcuts;
		},
		get enabled() {
			return enabled;
		},
		get activeScope() {
			return activeScope;
		},
		get lastTriggered() {
			return lastTriggered;
		},
		get isAttached() {
			return listenerAttached;
		},

		// Lifecycle
		attach,
		detach,

		// Management
		addShortcut,
		removeShortcut,
		updateShortcut,
		setEnabled,
		setScope,
		clearShortcuts,
		resetShortcuts,

		// Query
		getShortcut,
		getShortcutsByScope,
		getShortcutsGroupedByScope,
		hasConflict,
		formatShortcut
	};
}

// ============================================================================
// Preset Shortcut Definitions
// ============================================================================

export const EDITOR_SHORTCUTS: Shortcut[] = [
	{
		key: 's',
		ctrl: true,
		handler: () => {},
		description: 'Save',
		scope: 'editor',
		priority: 100
	},
	{
		key: 'z',
		ctrl: true,
		handler: () => {},
		description: 'Undo',
		scope: 'editor',
		priority: 90
	},
	{
		key: 'z',
		ctrl: true,
		shift: true,
		handler: () => {},
		description: 'Redo',
		scope: 'editor',
		priority: 90
	},
	{
		key: 'y',
		ctrl: true,
		handler: () => {},
		description: 'Redo (alternate)',
		scope: 'editor',
		priority: 89
	},
	{
		key: 'b',
		ctrl: true,
		handler: () => {},
		description: 'Bold',
		scope: 'editor',
		priority: 80
	},
	{
		key: 'i',
		ctrl: true,
		handler: () => {},
		description: 'Italic',
		scope: 'editor',
		priority: 80
	},
	{
		key: 'u',
		ctrl: true,
		handler: () => {},
		description: 'Underline',
		scope: 'editor',
		priority: 80
	},
	{
		key: 'k',
		ctrl: true,
		handler: () => {},
		description: 'Insert Link',
		scope: 'editor',
		priority: 70
	},
	{
		key: 'Enter',
		ctrl: true,
		handler: () => {},
		description: 'Insert Block Below',
		scope: 'editor',
		priority: 60
	},
	{
		key: 'Backspace',
		ctrl: true,
		handler: () => {},
		description: 'Delete Block',
		scope: 'editor',
		priority: 60
	},
	{
		key: 'ArrowUp',
		ctrl: true,
		shift: true,
		handler: () => {},
		description: 'Move Block Up',
		scope: 'editor',
		priority: 50
	},
	{
		key: 'ArrowDown',
		ctrl: true,
		shift: true,
		handler: () => {},
		description: 'Move Block Down',
		scope: 'editor',
		priority: 50
	},
	{
		key: 'd',
		ctrl: true,
		handler: () => {},
		description: 'Duplicate Block',
		scope: 'editor',
		priority: 40
	},
	{
		key: 'Escape',
		handler: () => {},
		description: 'Deselect / Close Panel',
		scope: 'editor',
		priority: 30
	},
	{
		key: '/',
		handler: () => {},
		description: 'Open Block Inserter',
		scope: 'editor',
		priority: 20
	},
	{
		key: 'p',
		ctrl: true,
		shift: true,
		handler: () => {},
		description: 'Preview',
		scope: 'editor',
		priority: 10
	}
];

export const NAVIGATION_SHORTCUTS: Shortcut[] = [
	{
		key: 'ArrowUp',
		handler: () => {},
		description: 'Select Previous Block',
		scope: 'navigation',
		priority: 50
	},
	{
		key: 'ArrowDown',
		handler: () => {},
		description: 'Select Next Block',
		scope: 'navigation',
		priority: 50
	},
	{
		key: 'Home',
		ctrl: true,
		handler: () => {},
		description: 'Select First Block',
		scope: 'navigation',
		priority: 40
	},
	{
		key: 'End',
		ctrl: true,
		handler: () => {},
		description: 'Select Last Block',
		scope: 'navigation',
		priority: 40
	}
];

export const GLOBAL_SHORTCUTS: Shortcut[] = [
	{
		key: '?',
		shift: true,
		handler: () => {},
		description: 'Show Keyboard Shortcuts',
		scope: 'global',
		priority: 100
	},
	{
		key: 'Escape',
		handler: () => {},
		description: 'Close Modal / Panel',
		scope: 'global',
		priority: 90
	}
];

// ============================================================================
// Helper: Create Shortcut Helper Dialog Content
// ============================================================================

export function generateShortcutsHelp(shortcuts: Shortcut[]): ShortcutGroup[] {
	const groups: Record<string, Shortcut[]> = {};

	for (const shortcut of shortcuts) {
		const scope = shortcut.scope || 'General';
		if (!groups[scope]) {
			groups[scope] = [];
		}
		groups[scope].push(shortcut);
	}

	return Object.entries(groups).map(([name, groupShortcuts]) => ({
		name: name.charAt(0).toUpperCase() + name.slice(1),
		shortcuts: groupShortcuts.filter((s) => s.description)
	}));
}

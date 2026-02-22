import { logger } from '$lib/utils/logger';
/**
 * Keyboard Shortcuts System for Content Editor
 *
 * A comprehensive keyboard shortcuts management system that provides:
 * - Cross-platform support (Mac vs Windows/Linux)
 * - Modifier key combinations (Cmd/Ctrl, Shift, Alt, Meta)
 * - Shortcut registration and override capabilities
 * - Context-aware activation (respects input focus)
 * - Human-readable shortcut display strings
 *
 * @module keyboard-shortcuts
 * @version 1.0.0
 */

// ============================================================================
// Type Definitions
// ============================================================================

/**
 * Supported modifier keys for shortcuts
 */
export type ModifierKey = 'cmd' | 'ctrl' | 'shift' | 'alt' | 'meta';

/**
 * Shortcut categories for organization
 */
export type ShortcutCategory = 'system' | 'formatting' | 'blocks' | 'navigation';

/**
 * Shortcut action identifiers
 */
export type ShortcutAction =
	// System shortcuts
	| 'save'
	| 'saveAndPublish'
	| 'undo'
	| 'redo'
	| 'preview'
	| 'publish'
	| 'slashCommands'
	| 'quickActions'
	| 'deselect'
	| 'help'
	// Formatting shortcuts
	| 'bold'
	| 'italic'
	| 'underline'
	| 'inlineCode'
	| 'link'
	| 'heading1'
	| 'heading2'
	| 'heading3'
	| 'bulletList'
	| 'numberedList'
	| 'blockquote'
	// Block shortcuts
	| 'duplicateBlock'
	| 'deleteBlock'
	| 'moveBlockUp'
	| 'moveBlockDown'
	| 'groupBlocks'
	| 'ungroupBlocks'
	// Navigation shortcuts
	| 'selectPrevious'
	| 'selectNext'
	| 'selectFirst'
	| 'selectLast'
	| 'find'
	| 'findAndReplace';

/**
 * Configuration for a single keyboard shortcut
 */
export interface ShortcutConfig {
	/** Unique action identifier */
	action: ShortcutAction;
	/** Primary key (e.g., 's', 'Enter', 'Backspace') */
	key: string;
	/** Required modifier keys */
	modifiers: ModifierKey[];
	/** Human-readable description */
	description: string;
	/** Category for grouping */
	category: ShortcutCategory;
	/** Whether to prevent default browser behavior */
	preventDefault?: boolean;
	/** Whether to stop event propagation */
	stopPropagation?: boolean;
	/** Whether shortcut works when input is focused */
	allowInInput?: boolean;
	/** Whether this shortcut is enabled */
	enabled?: boolean;
}

/**
 * Callback function type for shortcut handlers
 */
export type ShortcutHandler = (event: KeyboardEvent, action: ShortcutAction) => void | boolean;

/**
 * Map of action handlers
 */
export type ShortcutHandlers = Partial<Record<ShortcutAction, ShortcutHandler>>;

/**
 * Options for the useKeyboardShortcuts hook
 */
export interface UseKeyboardShortcutsOptions {
	/** Handlers for each shortcut action */
	handlers: ShortcutHandlers;
	/** Whether to enable shortcuts globally */
	enabled?: boolean;
	/** Custom shortcut overrides */
	overrides?: Partial<Record<ShortcutAction, Partial<ShortcutConfig>>>;
	/** Element to attach listener to (defaults to window) */
	target?: EventTarget;
	/** Callback when any shortcut is triggered */
	onShortcutTriggered?: (action: ShortcutAction, event: KeyboardEvent) => void;
}

/**
 * Result of useKeyboardShortcuts
 */
export interface UseKeyboardShortcutsResult {
	/** Cleanup function to remove listeners */
	destroy: () => void;
	/** Enable all shortcuts */
	enable: () => void;
	/** Disable all shortcuts */
	disable: () => void;
	/** Check if shortcuts are currently enabled */
	isEnabled: () => boolean;
	/** Get current registry instance */
	getRegistry: () => ShortcutRegistry;
}

/**
 * Platform detection result
 */
export interface PlatformInfo {
	isMac: boolean;
	isWindows: boolean;
	isLinux: boolean;
	modifierKey: 'cmd' | 'ctrl';
}

// ============================================================================
// Platform Detection
// ============================================================================

/**
 * Detects the current platform for proper modifier key handling
 * @returns Platform information object
 */
export function detectPlatform(): PlatformInfo {
	if (typeof navigator === 'undefined') {
		// SSR fallback - assume Windows/Linux
		return {
			isMac: false,
			isWindows: true,
			isLinux: false,
			modifierKey: 'ctrl'
		};
	}

	const platform = navigator.platform?.toLowerCase() || '';
	const userAgent = navigator.userAgent?.toLowerCase() || '';

	const isMac = platform.includes('mac') || userAgent.includes('macintosh');
	const isWindows = platform.includes('win') || userAgent.includes('windows');
	const isLinux = platform.includes('linux') && !userAgent.includes('android');

	return {
		isMac,
		isWindows,
		isLinux,
		modifierKey: isMac ? 'cmd' : 'ctrl'
	};
}

/**
 * Cached platform info for performance
 */
let cachedPlatform: PlatformInfo | null = null;

/**
 * Gets cached platform info or detects it
 * @returns Platform information
 */
export function getPlatform(): PlatformInfo {
	if (!cachedPlatform) {
		cachedPlatform = detectPlatform();
	}
	return cachedPlatform;
}

// ============================================================================
// Default Shortcut Definitions
// ============================================================================

/**
 * Default keyboard shortcuts configuration
 * Uses 'cmd' as a platform-agnostic modifier that maps to Cmd on Mac and Ctrl on Windows/Linux
 */
export const DEFAULT_SHORTCUTS: ShortcutConfig[] = [
	// ===== System Shortcuts =====
	{
		action: 'save',
		key: 's',
		modifiers: ['cmd'],
		description: 'Save document',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'saveAndPublish',
		key: 's',
		modifiers: ['cmd', 'shift'],
		description: 'Save and publish document',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'undo',
		key: 'z',
		modifiers: ['cmd'],
		description: 'Undo last action',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'redo',
		key: 'z',
		modifiers: ['cmd', 'shift'],
		description: 'Redo last action',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'preview',
		key: 'p',
		modifiers: ['cmd'],
		description: 'Preview document',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'publish',
		key: 'Enter',
		modifiers: ['cmd'],
		description: 'Publish document',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'slashCommands',
		key: '/',
		modifiers: ['cmd'],
		description: 'Open slash commands menu',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'quickActions',
		key: 'k',
		modifiers: ['cmd'],
		description: 'Open quick actions palette',
		category: 'system',
		preventDefault: true
	},
	{
		action: 'deselect',
		key: 'Escape',
		modifiers: [],
		description: 'Deselect current selection',
		category: 'system',
		preventDefault: false,
		allowInInput: true
	},
	{
		action: 'help',
		key: '?',
		modifiers: ['cmd'],
		description: 'Show keyboard shortcuts help',
		category: 'system',
		preventDefault: true
	},

	// ===== Formatting Shortcuts =====
	{
		action: 'bold',
		key: 'b',
		modifiers: ['cmd'],
		description: 'Toggle bold formatting',
		category: 'formatting',
		preventDefault: true,
		allowInInput: true
	},
	{
		action: 'italic',
		key: 'i',
		modifiers: ['cmd'],
		description: 'Toggle italic formatting',
		category: 'formatting',
		preventDefault: true,
		allowInInput: true
	},
	{
		action: 'underline',
		key: 'u',
		modifiers: ['cmd'],
		description: 'Toggle underline formatting',
		category: 'formatting',
		preventDefault: true,
		allowInInput: true
	},
	{
		action: 'inlineCode',
		key: 'e',
		modifiers: ['cmd'],
		description: 'Toggle inline code formatting',
		category: 'formatting',
		preventDefault: true,
		allowInInput: true
	},
	{
		action: 'link',
		key: 'k',
		modifiers: ['cmd'],
		description: 'Insert or edit link',
		category: 'formatting',
		preventDefault: true,
		allowInInput: true
	},
	{
		action: 'heading1',
		key: '1',
		modifiers: ['cmd', 'shift'],
		description: 'Apply Heading 1 style',
		category: 'formatting',
		preventDefault: true
	},
	{
		action: 'heading2',
		key: '2',
		modifiers: ['cmd', 'shift'],
		description: 'Apply Heading 2 style',
		category: 'formatting',
		preventDefault: true
	},
	{
		action: 'heading3',
		key: '3',
		modifiers: ['cmd', 'shift'],
		description: 'Apply Heading 3 style',
		category: 'formatting',
		preventDefault: true
	},
	{
		action: 'bulletList',
		key: '7',
		modifiers: ['cmd', 'shift'],
		description: 'Toggle bullet list',
		category: 'formatting',
		preventDefault: true
	},
	{
		action: 'numberedList',
		key: '8',
		modifiers: ['cmd', 'shift'],
		description: 'Toggle numbered list',
		category: 'formatting',
		preventDefault: true
	},
	{
		action: 'blockquote',
		key: '9',
		modifiers: ['cmd', 'shift'],
		description: 'Toggle blockquote',
		category: 'formatting',
		preventDefault: true
	},

	// ===== Block Shortcuts =====
	{
		action: 'duplicateBlock',
		key: 'd',
		modifiers: ['cmd'],
		description: 'Duplicate selected block',
		category: 'blocks',
		preventDefault: true
	},
	{
		action: 'deleteBlock',
		key: 'Backspace',
		modifiers: ['cmd'],
		description: 'Delete selected block',
		category: 'blocks',
		preventDefault: true
	},
	{
		action: 'moveBlockUp',
		key: 'ArrowUp',
		modifiers: ['cmd', 'shift'],
		description: 'Move block up',
		category: 'blocks',
		preventDefault: true
	},
	{
		action: 'moveBlockDown',
		key: 'ArrowDown',
		modifiers: ['cmd', 'shift'],
		description: 'Move block down',
		category: 'blocks',
		preventDefault: true
	},
	{
		action: 'groupBlocks',
		key: 'g',
		modifiers: ['cmd'],
		description: 'Group selected blocks',
		category: 'blocks',
		preventDefault: true
	},
	{
		action: 'ungroupBlocks',
		key: 'g',
		modifiers: ['cmd', 'shift'],
		description: 'Ungroup selected blocks',
		category: 'blocks',
		preventDefault: true
	},

	// ===== Navigation Shortcuts =====
	{
		action: 'selectPrevious',
		key: 'ArrowUp',
		modifiers: ['alt'],
		description: 'Select previous block',
		category: 'navigation',
		preventDefault: true
	},
	{
		action: 'selectNext',
		key: 'ArrowDown',
		modifiers: ['alt'],
		description: 'Select next block',
		category: 'navigation',
		preventDefault: true
	},
	{
		action: 'selectFirst',
		key: 'Home',
		modifiers: ['cmd'],
		description: 'Select first block',
		category: 'navigation',
		preventDefault: true
	},
	{
		action: 'selectLast',
		key: 'End',
		modifiers: ['cmd'],
		description: 'Select last block',
		category: 'navigation',
		preventDefault: true
	},
	{
		action: 'find',
		key: 'f',
		modifiers: ['cmd'],
		description: 'Find in document',
		category: 'navigation',
		preventDefault: true
	},
	{
		action: 'findAndReplace',
		key: 'f',
		modifiers: ['cmd', 'shift'],
		description: 'Find and replace in document',
		category: 'navigation',
		preventDefault: true
	}
];

// ============================================================================
// Shortcut Registry Class
// ============================================================================

/**
 * ShortcutRegistry manages keyboard shortcuts registration, matching, and execution.
 *
 * Features:
 * - Cross-platform modifier key handling (Cmd/Ctrl)
 * - Shortcut registration and override
 * - Context-aware activation (input focus detection)
 * - Event handling with preventDefault/stopPropagation support
 *
 * @example
 * ```typescript
 * const registry = new ShortcutRegistry();
 *
 * // Register custom shortcut
 * registry.register({
 *   action: 'save',
 *   key: 's',
 *   modifiers: ['cmd'],
 *   description: 'Save document',
 *   category: 'system'
 * });
 *
 * // Handle keyboard event
 * registry.handleKeyDown(event, {
 *   save: () => logger.info('Saving...')
 * });
 * ```
 */
export class ShortcutRegistry {
	/** Registered shortcuts indexed by action */
	private shortcuts: Map<ShortcutAction, ShortcutConfig> = new Map();

	/** Shortcuts indexed by key for fast lookup */
	private shortcutsByKey: Map<string, ShortcutConfig[]> = new Map();

	/** Platform information */
	private platform: PlatformInfo;

	/** Whether the registry is enabled */
	private enabled: boolean = true;

	/**
	 * Creates a new ShortcutRegistry instance
	 * @param options - Configuration options
	 */
	constructor(options?: {
		/** Initial shortcuts to register */
		shortcuts?: ShortcutConfig[];
		/** Platform override for testing */
		platform?: PlatformInfo;
	}) {
		this.platform = options?.platform || getPlatform();

		// Register default shortcuts
		const initialShortcuts = options?.shortcuts || DEFAULT_SHORTCUTS;
		for (const shortcut of initialShortcuts) {
			this.register(shortcut);
		}
	}

	/**
	 * Registers a keyboard shortcut
	 * @param config - Shortcut configuration
	 * @returns The registry instance for chaining
	 */
	register(config: ShortcutConfig): this {
		const normalizedConfig: ShortcutConfig = {
			...config,
			key: this.normalizeKey(config.key),
			preventDefault: config.preventDefault ?? true,
			stopPropagation: config.stopPropagation ?? false,
			allowInInput: config.allowInInput ?? false,
			enabled: config.enabled ?? true
		};

		this.shortcuts.set(config.action, normalizedConfig);
		this.indexByKey(normalizedConfig);

		return this;
	}

	/**
	 * Unregisters a keyboard shortcut
	 * @param action - The action to unregister
	 * @returns The registry instance for chaining
	 */
	unregister(action: ShortcutAction): this {
		const shortcut = this.shortcuts.get(action);
		if (shortcut) {
			this.shortcuts.delete(action);
			this.removeFromKeyIndex(shortcut);
		}
		return this;
	}

	/**
	 * Overrides an existing shortcut configuration
	 * @param action - The action to override
	 * @param overrides - Partial configuration to merge
	 * @returns The registry instance for chaining
	 */
	override(action: ShortcutAction, overrides: Partial<ShortcutConfig>): this {
		const existing = this.shortcuts.get(action);
		if (existing) {
			this.removeFromKeyIndex(existing);
			const updated = { ...existing, ...overrides, action };
			this.register(updated);
		}
		return this;
	}

	/**
	 * Gets a shortcut configuration by action
	 * @param action - The action to look up
	 * @returns The shortcut config or undefined
	 */
	get(action: ShortcutAction): ShortcutConfig | undefined {
		return this.shortcuts.get(action);
	}

	/**
	 * Gets all registered shortcuts
	 * @returns Array of all shortcut configurations
	 */
	getAll(): ShortcutConfig[] {
		return Array.from(this.shortcuts.values());
	}

	/**
	 * Gets shortcuts filtered by category
	 * @param category - Category to filter by
	 * @returns Array of matching shortcuts
	 */
	getByCategory(category: ShortcutCategory): ShortcutConfig[] {
		return this.getAll().filter((s) => s.category === category);
	}

	/**
	 * Enables or disables the registry
	 * @param enabled - Whether to enable
	 */
	setEnabled(enabled: boolean): void {
		this.enabled = enabled;
	}

	/**
	 * Checks if the registry is enabled
	 * @returns Whether the registry is enabled
	 */
	isEnabled(): boolean {
		return this.enabled;
	}

	/**
	 * Enables or disables a specific shortcut
	 * @param action - The action to toggle
	 * @param enabled - Whether to enable
	 */
	setShortcutEnabled(action: ShortcutAction, enabled: boolean): void {
		const shortcut = this.shortcuts.get(action);
		if (shortcut) {
			shortcut.enabled = enabled;
		}
	}

	/**
	 * Handles a keyboard event, matching against registered shortcuts
	 * @param event - The keyboard event
	 * @param handlers - Map of action handlers
	 * @returns Whether a shortcut was matched and handled
	 */
	handleKeyDown(event: KeyboardEvent, handlers: ShortcutHandlers): boolean {
		if (!this.enabled) {
			return false;
		}

		const matchedShortcut = this.matchEvent(event);
		if (!matchedShortcut) {
			return false;
		}

		// Check if shortcut is enabled
		if (!matchedShortcut.enabled) {
			return false;
		}

		// Check input focus
		if (!matchedShortcut.allowInInput && this.isInputFocused(event)) {
			// Allow escape in inputs by default for deselect
			if (matchedShortcut.action !== 'deselect') {
				return false;
			}
		}

		// Get handler
		const handler = handlers[matchedShortcut.action];
		if (!handler) {
			return false;
		}

		// Prevent default if configured
		if (matchedShortcut.preventDefault) {
			event.preventDefault();
		}

		// Stop propagation if configured
		if (matchedShortcut.stopPropagation) {
			event.stopPropagation();
		}

		// Execute handler
		const result = handler(event, matchedShortcut.action);

		// If handler returns false, don't consider it handled
		return result !== false;
	}

	/**
	 * Matches a keyboard event to a registered shortcut
	 * @param event - The keyboard event
	 * @returns The matched shortcut or null
	 */
	matchEvent(event: KeyboardEvent): ShortcutConfig | null {
		const normalizedKey = this.normalizeKey(event.key);
		const candidates = this.shortcutsByKey.get(normalizedKey);

		if (!candidates || candidates.length === 0) {
			return null;
		}

		// Find the best match (most specific modifier combination)
		let bestMatch: ShortcutConfig | null = null;
		let bestModifierCount = -1;

		for (const shortcut of candidates) {
			if (this.modifiersMatch(event, shortcut.modifiers)) {
				const modifierCount = shortcut.modifiers.length;
				if (modifierCount > bestModifierCount) {
					bestMatch = shortcut;
					bestModifierCount = modifierCount;
				}
			}
		}

		return bestMatch;
	}

	/**
	 * Gets the display string for a shortcut
	 * @param action - The action to get display for
	 * @returns Human-readable shortcut string
	 */
	getDisplayString(action: ShortcutAction): string {
		const shortcut = this.shortcuts.get(action);
		if (!shortcut) {
			return '';
		}
		return formatShortcutDisplay(shortcut, this.platform);
	}

	/**
	 * Gets the platform info
	 * @returns Platform information
	 */
	getPlatform(): PlatformInfo {
		return this.platform;
	}

	// ===== Private Methods =====

	/**
	 * Normalizes a key to a standard format
	 */
	private normalizeKey(key: string): string {
		// Handle special key names
		const keyMap: Record<string, string> = {
			' ': 'Space',
			ArrowLeft: 'ArrowLeft',
			ArrowRight: 'ArrowRight',
			ArrowUp: 'ArrowUp',
			ArrowDown: 'ArrowDown',
			Esc: 'Escape'
		};

		const mapped = keyMap[key];
		if (mapped) {
			return mapped;
		}

		// Normalize to lowercase for letter keys
		if (key.length === 1 && /[a-zA-Z]/.test(key)) {
			return key.toLowerCase();
		}

		return key;
	}

	/**
	 * Indexes a shortcut by its key for fast lookup
	 */
	private indexByKey(shortcut: ShortcutConfig): void {
		const key = shortcut.key;
		const existing = this.shortcutsByKey.get(key) || [];

		// Remove any existing shortcut with same action
		const filtered = existing.filter((s) => s.action !== shortcut.action);
		filtered.push(shortcut);

		this.shortcutsByKey.set(key, filtered);
	}

	/**
	 * Removes a shortcut from the key index
	 */
	private removeFromKeyIndex(shortcut: ShortcutConfig): void {
		const key = shortcut.key;
		const existing = this.shortcutsByKey.get(key);
		if (existing) {
			const filtered = existing.filter((s) => s.action !== shortcut.action);
			if (filtered.length > 0) {
				this.shortcutsByKey.set(key, filtered);
			} else {
				this.shortcutsByKey.delete(key);
			}
		}
	}

	/**
	 * Checks if modifier keys match the event
	 */
	private modifiersMatch(event: KeyboardEvent, modifiers: ModifierKey[]): boolean {
		const isMac = this.platform.isMac;

		// Check each required modifier
		for (const modifier of modifiers) {
			switch (modifier) {
				case 'cmd':
					// 'cmd' is platform-agnostic: Meta on Mac, Ctrl on Windows/Linux
					if (isMac) {
						if (!event.metaKey) return false;
					} else {
						if (!event.ctrlKey) return false;
					}
					break;
				case 'ctrl':
					if (!event.ctrlKey) return false;
					break;
				case 'shift':
					if (!event.shiftKey) return false;
					break;
				case 'alt':
					if (!event.altKey) return false;
					break;
				case 'meta':
					if (!event.metaKey) return false;
					break;
			}
		}

		// Check that no extra modifiers are pressed
		const expectedCmd = modifiers.includes('cmd');
		const expectedCtrl = modifiers.includes('ctrl');
		const expectedShift = modifiers.includes('shift');
		const expectedAlt = modifiers.includes('alt');
		const expectedMeta = modifiers.includes('meta');

		// For 'cmd', we need special handling
		if (isMac) {
			// On Mac, 'cmd' maps to metaKey
			if (event.metaKey !== (expectedCmd || expectedMeta)) return false;
			if (event.ctrlKey !== expectedCtrl) return false;
		} else {
			// On Windows/Linux, 'cmd' maps to ctrlKey
			if (event.ctrlKey !== (expectedCmd || expectedCtrl)) return false;
			if (event.metaKey !== expectedMeta) return false;
		}

		if (event.shiftKey !== expectedShift) return false;
		if (event.altKey !== expectedAlt) return false;

		return true;
	}

	/**
	 * Checks if an input element is currently focused
	 */
	private isInputFocused(event: KeyboardEvent): boolean {
		const target = event.target as HTMLElement | null;
		if (!target) return false;

		const tagName = target.tagName?.toLowerCase();

		// Check for input elements
		if (tagName === 'input' || tagName === 'textarea' || tagName === 'select') {
			return true;
		}

		// Check for contenteditable
		if (target.isContentEditable) {
			return true;
		}

		// Check for contenteditable ancestor
		if (target.closest('[contenteditable="true"]')) {
			return true;
		}

		return false;
	}
}

// ============================================================================
// Display Formatting
// ============================================================================

/**
 * Symbol mappings for different platforms
 */
const MODIFIER_SYMBOLS = {
	mac: {
		cmd: '\u2318', // Command (Cmd)
		ctrl: '\u2303', // Control
		shift: '\u21E7', // Shift
		alt: '\u2325', // Option
		meta: '\u2318' // Command
	},
	other: {
		cmd: 'Ctrl',
		ctrl: 'Ctrl',
		shift: 'Shift',
		alt: 'Alt',
		meta: 'Win'
	}
};

/**
 * Key display mappings
 */
const KEY_DISPLAY: Record<string, { mac: string; other: string }> = {
	Enter: { mac: '\u21A9', other: 'Enter' },
	Escape: { mac: 'Esc', other: 'Esc' },
	Backspace: { mac: '\u232B', other: 'Backspace' },
	Delete: { mac: '\u2326', other: 'Delete' },
	ArrowUp: { mac: '\u2191', other: '\u2191' },
	ArrowDown: { mac: '\u2193', other: '\u2193' },
	ArrowLeft: { mac: '\u2190', other: '\u2190' },
	ArrowRight: { mac: '\u2192', other: '\u2192' },
	Home: { mac: '\u2196', other: 'Home' },
	End: { mac: '\u2198', other: 'End' },
	PageUp: { mac: '\u21DE', other: 'PgUp' },
	PageDown: { mac: '\u21DF', other: 'PgDn' },
	Tab: { mac: '\u21E5', other: 'Tab' },
	Space: { mac: 'Space', other: 'Space' }
};

/**
 * Formats a shortcut for display
 * @param shortcut - The shortcut configuration
 * @param platform - Platform information
 * @returns Human-readable shortcut string
 */
export function formatShortcutDisplay(shortcut: ShortcutConfig, platform?: PlatformInfo): string {
	const plat = platform || getPlatform();
	const isMac = plat.isMac;
	const symbols = isMac ? MODIFIER_SYMBOLS.mac : MODIFIER_SYMBOLS.other;
	const separator = isMac ? '' : '+';

	// Build modifier string
	const modifierOrder: ModifierKey[] = ['ctrl', 'alt', 'shift', 'cmd', 'meta'];
	const modifierParts: string[] = [];

	for (const mod of modifierOrder) {
		if (shortcut.modifiers.includes(mod)) {
			// Skip duplicate meta/cmd on Mac
			if (isMac && mod === 'meta' && shortcut.modifiers.includes('cmd')) {
				continue;
			}
			modifierParts.push(symbols[mod]);
		}
	}

	// Format key
	let keyDisplay = shortcut.key;
	const keyMapping = KEY_DISPLAY[shortcut.key];
	if (keyMapping) {
		keyDisplay = isMac ? keyMapping.mac : keyMapping.other;
	} else if (shortcut.key.length === 1) {
		keyDisplay = shortcut.key.toUpperCase();
	}

	// Combine
	if (modifierParts.length === 0) {
		return keyDisplay;
	}

	if (isMac) {
		return modifierParts.join('') + keyDisplay;
	} else {
		return [...modifierParts, keyDisplay].join(separator);
	}
}

/**
 * Gets the display string for any shortcut action
 * @param action - The action to get display for
 * @param platform - Optional platform override
 * @returns Human-readable shortcut string
 */
export function getShortcutDisplay(action: ShortcutAction, platform?: PlatformInfo): string {
	const shortcut = DEFAULT_SHORTCUTS.find((s) => s.action === action);
	if (!shortcut) {
		return '';
	}
	return formatShortcutDisplay(shortcut, platform);
}

/**
 * Gets display strings for all shortcuts grouped by category
 * @param platform - Optional platform override
 * @returns Object mapping categories to arrays of display info
 */
export function getAllShortcutDisplays(
	platform?: PlatformInfo
): Record<
	ShortcutCategory,
	Array<{ action: ShortcutAction; display: string; description: string }>
> {
	const plat = platform || getPlatform();
	const result: Record<
		ShortcutCategory,
		Array<{ action: ShortcutAction; display: string; description: string }>
	> = {
		system: [],
		formatting: [],
		blocks: [],
		navigation: []
	};

	for (const shortcut of DEFAULT_SHORTCUTS) {
		result[shortcut.category].push({
			action: shortcut.action,
			display: formatShortcutDisplay(shortcut, plat),
			description: shortcut.description
		});
	}

	return result;
}

// ============================================================================
// useKeyboardShortcuts Hook
// ============================================================================

/**
 * Sets up keyboard shortcut handling with automatic cleanup.
 *
 * This function creates a keyboard event listener that matches keyboard events
 * against registered shortcuts and calls the appropriate handlers.
 *
 * Features:
 * - Automatic platform detection for Cmd/Ctrl handling
 * - Respects input/textarea focus by default
 * - Supports custom shortcut overrides
 * - Provides enable/disable controls
 * - Returns cleanup function for proper teardown
 *
 * @example
 * ```typescript
 * // In a Svelte component
 * import { onMount, onDestroy } from 'svelte';
 * import { useKeyboardShortcuts } from '$lib/utils/keyboard-shortcuts';
 *
 * let shortcuts: ReturnType<typeof useKeyboardShortcuts>;
 *
 * onMount(() => {
 *   shortcuts = useKeyboardShortcuts({
 *     handlers: {
 *       save: () => saveDocument(),
 *       bold: () => toggleBold(),
 *       undo: () => history.undo()
 *     },
 *     onShortcutTriggered: (action) => {
 *       logger.info(`Shortcut triggered: ${action}`);
 *     }
 *   });
 * });
 *
 * onDestroy(() => {
 *   shortcuts?.destroy();
 * });
 * ```
 *
 * @param options - Configuration options
 * @returns Object with destroy, enable, disable, isEnabled, and getRegistry functions
 */
export function useKeyboardShortcuts(
	options: UseKeyboardShortcutsOptions
): UseKeyboardShortcutsResult {
	const {
		handlers,
		enabled = true,
		overrides = {},
		target = typeof window !== 'undefined' ? window : null,
		onShortcutTriggered
	} = options;

	// Create registry
	const registry = new ShortcutRegistry();
	registry.setEnabled(enabled);

	// Apply overrides
	for (const [action, override] of Object.entries(overrides)) {
		if (override) {
			registry.override(action as ShortcutAction, override);
		}
	}

	// Create event handler
	const handleKeyDown = (event: Event): void => {
		if (!(event instanceof KeyboardEvent)) {
			return;
		}

		// Wrap handlers to add callback
		const wrappedHandlers: ShortcutHandlers = {};
		for (const [action, handler] of Object.entries(handlers)) {
			if (handler) {
				wrappedHandlers[action as ShortcutAction] = (evt, act) => {
					onShortcutTriggered?.(act, evt);
					return handler(evt, act);
				};
			}
		}

		registry.handleKeyDown(event, wrappedHandlers);
	};

	// Add event listener
	if (target) {
		target.addEventListener('keydown', handleKeyDown as EventListener);
	}

	// Return control functions
	return {
		destroy: () => {
			if (target) {
				target.removeEventListener('keydown', handleKeyDown as EventListener);
			}
		},
		enable: () => {
			registry.setEnabled(true);
		},
		disable: () => {
			registry.setEnabled(false);
		},
		isEnabled: () => {
			return registry.isEnabled();
		},
		getRegistry: () => {
			return registry;
		}
	};
}

// ============================================================================
// Svelte 5 Runes Integration
// ============================================================================

/**
 * Creates a reactive keyboard shortcuts handler for Svelte 5 components.
 *
 * This is designed to work with Svelte 5's new reactivity system and
 * provides a clean integration pattern for component-based usage.
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { createKeyboardShortcuts } from '$lib/utils/keyboard-shortcuts';
 *
 *   const shortcuts = createKeyboardShortcuts({
 *     save: () => saveDocument(),
 *     bold: () => editor.toggleBold()
 *   });
 * </script>
 *
 * <svelte:window on:keydown={shortcuts.handle} />
 * ```
 *
 * @param handlers - Map of action handlers
 * @param options - Additional options
 * @returns Object with handle function and registry
 */
export function createKeyboardShortcuts(
	handlers: ShortcutHandlers,
	options?: {
		overrides?: Partial<Record<ShortcutAction, Partial<ShortcutConfig>>>;
		onTriggered?: (action: ShortcutAction, event: KeyboardEvent) => void;
	}
): {
	handle: (event: KeyboardEvent) => boolean;
	registry: ShortcutRegistry;
	getDisplay: (action: ShortcutAction) => string;
	enable: () => void;
	disable: () => void;
	isEnabled: () => boolean;
} {
	const registry = new ShortcutRegistry();

	// Apply overrides
	if (options?.overrides) {
		for (const [action, override] of Object.entries(options.overrides)) {
			if (override) {
				registry.override(action as ShortcutAction, override);
			}
		}
	}

	return {
		handle: (event: KeyboardEvent): boolean => {
			const wrappedHandlers: ShortcutHandlers = {};
			for (const [action, handler] of Object.entries(handlers)) {
				if (handler) {
					wrappedHandlers[action as ShortcutAction] = (evt, act) => {
						options?.onTriggered?.(act, evt);
						return handler(evt, act);
					};
				}
			}
			return registry.handleKeyDown(event, wrappedHandlers);
		},
		registry,
		getDisplay: (action: ShortcutAction) => registry.getDisplayString(action),
		enable: () => registry.setEnabled(true),
		disable: () => registry.setEnabled(false),
		isEnabled: () => registry.isEnabled()
	};
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Checks if a keyboard event matches a specific shortcut action
 * @param event - The keyboard event to check
 * @param action - The action to match against
 * @param platform - Optional platform override
 * @returns Whether the event matches the shortcut
 */
export function matchesShortcut(
	event: KeyboardEvent,
	action: ShortcutAction,
	platform?: PlatformInfo
): boolean {
	const registry = new ShortcutRegistry({ platform });
	const matched = registry.matchEvent(event);
	return matched?.action === action;
}

/**
 * Creates a shortcut event for testing or programmatic triggering
 * @param action - The action to create an event for
 * @param platform - Optional platform override
 * @returns A KeyboardEvent that would trigger the shortcut
 */
export function createShortcutEvent(
	action: ShortcutAction,
	platform?: PlatformInfo
): KeyboardEvent | null {
	const shortcut = DEFAULT_SHORTCUTS.find((s) => s.action === action);
	if (!shortcut) {
		return null;
	}

	const plat = platform || getPlatform();
	const isMac = plat.isMac;

	const eventInit: KeyboardEventInit = {
		key: shortcut.key,
		bubbles: true,
		cancelable: true,
		ctrlKey: false,
		shiftKey: false,
		altKey: false,
		metaKey: false
	};

	for (const modifier of shortcut.modifiers) {
		switch (modifier) {
			case 'cmd':
				if (isMac) {
					eventInit.metaKey = true;
				} else {
					eventInit.ctrlKey = true;
				}
				break;
			case 'ctrl':
				eventInit.ctrlKey = true;
				break;
			case 'shift':
				eventInit.shiftKey = true;
				break;
			case 'alt':
				eventInit.altKey = true;
				break;
			case 'meta':
				eventInit.metaKey = true;
				break;
		}
	}

	return new KeyboardEvent('keydown', eventInit);
}

/**
 * Parses a shortcut string into a ShortcutConfig
 * @param shortcutString - String like "Cmd+Shift+S" or "Ctrl+S"
 * @param action - The action name
 * @param description - Description for the shortcut
 * @param category - Category for the shortcut
 * @returns Parsed shortcut configuration
 */
export function parseShortcutString(
	shortcutString: string,
	action: ShortcutAction,
	description: string,
	category: ShortcutCategory
): ShortcutConfig {
	const parts = shortcutString.split('+').map((p) => p.trim().toLowerCase());
	const modifiers: ModifierKey[] = [];
	let key = '';

	for (const part of parts) {
		switch (part) {
			case 'cmd':
			case 'command':
			case '\u2318':
				modifiers.push('cmd');
				break;
			case 'ctrl':
			case 'control':
				modifiers.push('ctrl');
				break;
			case 'shift':
			case '\u21E7':
				modifiers.push('shift');
				break;
			case 'alt':
			case 'option':
			case '\u2325':
				modifiers.push('alt');
				break;
			case 'meta':
			case 'win':
				modifiers.push('meta');
				break;
			default:
				key = part;
		}
	}

	return {
		action,
		key,
		modifiers,
		description,
		category
	};
}

/**
 * Gets all shortcuts as a help text string
 * @param platform - Optional platform override
 * @returns Formatted help text
 */
export function getShortcutsHelpText(platform?: PlatformInfo): string {
	const displays = getAllShortcutDisplays(platform);
	const categories: ShortcutCategory[] = ['system', 'formatting', 'blocks', 'navigation'];
	const categoryNames: Record<ShortcutCategory, string> = {
		system: 'System',
		formatting: 'Formatting',
		blocks: 'Blocks',
		navigation: 'Navigation'
	};

	let text = 'Keyboard Shortcuts\n==================\n\n';

	for (const category of categories) {
		text += `${categoryNames[category]}:\n`;
		text += '-'.repeat(categoryNames[category].length + 1) + '\n';

		for (const item of displays[category]) {
			const padding = ' '.repeat(Math.max(0, 20 - item.display.length));
			text += `  ${item.display}${padding}${item.description}\n`;
		}
		text += '\n';
	}

	return text;
}

// ============================================================================
// Default Export
// ============================================================================

export default {
	ShortcutRegistry,
	useKeyboardShortcuts,
	createKeyboardShortcuts,
	getShortcutDisplay,
	getAllShortcutDisplays,
	formatShortcutDisplay,
	matchesShortcut,
	createShortcutEvent,
	parseShortcutString,
	getShortcutsHelpText,
	detectPlatform,
	getPlatform,
	DEFAULT_SHORTCUTS
};

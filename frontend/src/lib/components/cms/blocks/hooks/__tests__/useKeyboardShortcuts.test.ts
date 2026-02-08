/**
 * useKeyboardShortcuts Hook - Comprehensive Test Suite
 * ===============================================================================
 * Unit tests for keyboard shortcuts with automatic cleanup and modifier key support
 *
 * @version 2.0.0
 */

// Import setup first to initialize mocks
import './setup';

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// ============================================================================
// Import Hook and Types
// ============================================================================

import {
	useKeyboardShortcuts,
	EDITOR_SHORTCUTS,
	NAVIGATION_SHORTCUTS,
	GLOBAL_SHORTCUTS,
	generateShortcutsHelp,
	type Shortcut,
	type KeyboardShortcutsOptions
} from '../useKeyboardShortcuts.svelte';

// ============================================================================
// Test Utilities
// ============================================================================

function createKeyboardEvent(
	key: string,
	options: {
		ctrlKey?: boolean;
		shiftKey?: boolean;
		altKey?: boolean;
		metaKey?: boolean;
		target?: HTMLElement;
	} = {}
): KeyboardEvent {
	const event = new KeyboardEvent('keydown', {
		key,
		ctrlKey: options.ctrlKey || false,
		shiftKey: options.shiftKey || false,
		altKey: options.altKey || false,
		metaKey: options.metaKey || false,
		bubbles: true,
		cancelable: true
	});

	if (options.target) {
		Object.defineProperty(event, 'target', {
			value: options.target,
			writable: false
		});
	}

	return event;
}

function createInputElement(tagName: 'INPUT' | 'TEXTAREA' | 'SELECT' = 'INPUT'): HTMLElement {
	const element = document.createElement(tagName);
	return element;
}

function createContentEditableElement(): HTMLElement {
	const element = document.createElement('div');
	element.contentEditable = 'true';
	// JSDOM does not support isContentEditable, so we polyfill it
	Object.defineProperty(element, 'isContentEditable', { value: true, writable: false });
	return element;
}

// ============================================================================
// useKeyboardShortcuts Tests
// ============================================================================

describe('useKeyboardShortcuts', () => {
	let addEventListenerSpy: ReturnType<typeof vi.spyOn>;
	let removeEventListenerSpy: ReturnType<typeof vi.spyOn>;

	beforeEach(() => {
		vi.clearAllMocks();
		addEventListenerSpy = vi.spyOn(document, 'addEventListener');
		removeEventListenerSpy = vi.spyOn(document, 'removeEventListener');
	});

	afterEach(() => {
		addEventListenerSpy.mockRestore();
		removeEventListenerSpy.mockRestore();
	});

	// ========================================================================
	// Lifecycle Tests
	// ========================================================================

	describe('Lifecycle', () => {
		it('registers keyboard listener on mount', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			// Hook should attach listener
			expect(hook.isAttached).toBe(true);
			expect(addEventListenerSpy).toHaveBeenCalledWith(
				'keydown',
				expect.any(Function),
				expect.any(Object)
			);
		});

		it('removes listener on destroy', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			// Manually detach
			hook.detach();

			expect(hook.isAttached).toBe(false);
			expect(removeEventListenerSpy).toHaveBeenCalledWith(
				'keydown',
				expect.any(Function),
				expect.any(Object)
			);
		});

		it('can manually attach and detach', () => {
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler: vi.fn() }]
			});

			// Initially attached
			expect(hook.isAttached).toBe(true);

			// Detach
			hook.detach();
			expect(hook.isAttached).toBe(false);

			// Re-attach
			hook.attach();
			expect(hook.isAttached).toBe(true);
		});

		it('does not double-attach', () => {
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler: vi.fn() }]
			});

			const initialCallCount = addEventListenerSpy.mock.calls.length;

			// Try to attach again
			hook.attach();

			// Should not add another listener
			expect(addEventListenerSpy.mock.calls.length).toBe(initialCallCount);
		});
	});

	// ========================================================================
	// Single Key Matching
	// ========================================================================

	describe('Single Key Matching', () => {
		it('matches single key', () => {
			const handler = vi.fn();
			void useKeyboardShortcuts({
				shortcuts: [{ key: 'Escape', handler }]
			});

			// Dispatch event
			const event = createKeyboardEvent('Escape');
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalledTimes(1);
			expect(handler).toHaveBeenCalledWith(expect.any(KeyboardEvent));
		});

		it('matches letter key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'a', handler }]
			});

			const event = createKeyboardEvent('a');
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});

		it('matches Enter key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'Enter', handler }]
			});

			const event = createKeyboardEvent('Enter');
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});

		it('does not match wrong key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'a', handler }]
			});

			const event = createKeyboardEvent('b');
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Ctrl+Key Matching
	// ========================================================================

	describe('Ctrl+Key Matching', () => {
		it('matches Ctrl+key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});

		it('does not match key without Ctrl', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: false });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});

		it('does not match Ctrl+wrong key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const event = createKeyboardEvent('a', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});

		it('matches Cmd+key on Mac (meta key for ctrl shortcut)', () => {
			// Mock Mac platform
			const originalPlatform = navigator.platform;
			Object.defineProperty(navigator, 'platform', { value: 'MacIntel', configurable: true });

			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const event = createKeyboardEvent('s', { metaKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();

			// Restore
			Object.defineProperty(navigator, 'platform', { value: originalPlatform, configurable: true });
		});
	});

	// ========================================================================
	// Shift+Key Matching
	// ========================================================================

	describe('Shift+Key Matching', () => {
		it('matches Shift+key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: '?', shift: true, handler }]
			});

			const event = createKeyboardEvent('?', { shiftKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});

		it('does not match key without Shift', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: '?', shift: true, handler }]
			});

			const event = createKeyboardEvent('?', { shiftKey: false });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});

		it('does not match Shift+key if shortcut does not require Shift', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'a', handler }] // No shift required
			});

			const event = createKeyboardEvent('a', { shiftKey: true });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Alt+Key Matching
	// ========================================================================

	describe('Alt+Key Matching', () => {
		it('matches Alt+key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'p', alt: true, handler }]
			});

			const event = createKeyboardEvent('p', { altKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});

		it('does not match key without Alt', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'p', alt: true, handler }]
			});

			const event = createKeyboardEvent('p', { altKey: false });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Ctrl+Shift+Key Matching
	// ========================================================================

	describe('Ctrl+Shift+Key Matching', () => {
		it('matches Ctrl+Shift+key', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'z', ctrl: true, shift: true, handler }]
			});

			const event = createKeyboardEvent('z', { ctrlKey: true, shiftKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});

		it('does not match Ctrl+key without Shift', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'z', ctrl: true, shift: true, handler }]
			});

			const event = createKeyboardEvent('z', { ctrlKey: true, shiftKey: false });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});

		it('does not match Shift+key without Ctrl', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'z', ctrl: true, shift: true, handler }]
			});

			const event = createKeyboardEvent('z', { ctrlKey: false, shiftKey: true });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Handler Calling
	// ========================================================================

	describe('Handler Calling', () => {
		it('calls handler on match', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalledTimes(1);
		});

		it('passes event to handler', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalledWith(event);
		});

		it('only calls first matching handler (priority)', () => {
			const handler1 = vi.fn();
			const handler2 = vi.fn();

			useKeyboardShortcuts({
				shortcuts: [
					{ key: 's', ctrl: true, handler: handler1, priority: 100 },
					{ key: 's', ctrl: true, handler: handler2, priority: 50 }
				]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler1).toHaveBeenCalled();
			expect(handler2).not.toHaveBeenCalled();
		});

		it('tracks lastTriggered shortcut', () => {
			const handler = vi.fn();
			const shortcut = { key: 's', ctrl: true, handler, description: 'Save' };

			const hook = useKeyboardShortcuts({
				shortcuts: [shortcut]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(hook.lastTriggered).not.toBeNull();
			expect(hook.lastTriggered?.key).toBe('s');
		});
	});

	// ========================================================================
	// Prevent Default
	// ========================================================================

	describe('Prevent Default', () => {
		it('prevents default on match by default', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			const preventDefaultSpy = vi.spyOn(event, 'preventDefault');

			document.dispatchEvent(event);

			expect(preventDefaultSpy).toHaveBeenCalled();
		});

		it('respects preventDefault: false', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler, preventDefault: false }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			const preventDefaultSpy = vi.spyOn(event, 'preventDefault');

			document.dispatchEvent(event);

			expect(preventDefaultSpy).not.toHaveBeenCalled();
		});

		it('respects stopPropagation option', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler, stopPropagation: true }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			const stopPropagationSpy = vi.spyOn(event, 'stopPropagation');

			document.dispatchEvent(event);

			expect(stopPropagationSpy).toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Input Element Handling
	// ========================================================================

	describe('Input Element Handling', () => {
		it('ignores shortcuts in input elements without modifiers', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'a', handler }]
			});

			const input = createInputElement();
			document.body.appendChild(input);

			const event = createKeyboardEvent('a', { target: input });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();

			document.body.removeChild(input);
		});

		it('triggers shortcuts with modifiers in input elements', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			const input = createInputElement();
			document.body.appendChild(input);

			const event = createKeyboardEvent('s', { ctrlKey: true, target: input });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();

			document.body.removeChild(input);
		});

		it('ignores shortcuts in textarea without modifiers', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'Enter', handler }]
			});

			const textarea = createInputElement('TEXTAREA');
			document.body.appendChild(textarea);

			const event = createKeyboardEvent('Enter', { target: textarea });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();

			document.body.removeChild(textarea);
		});

		it('ignores shortcuts in contenteditable without modifiers', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'a', handler }]
			});

			const editable = createContentEditableElement();
			document.body.appendChild(editable);

			const event = createKeyboardEvent('a', { target: editable });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();

			document.body.removeChild(editable);
		});

		it('triggers shortcuts with Alt modifier in input elements', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'p', alt: true, handler }]
			});

			const input = createInputElement();
			document.body.appendChild(input);

			const event = createKeyboardEvent('p', { altKey: true, target: input });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();

			document.body.removeChild(input);
		});
	});

	// ========================================================================
	// Enabled/Disabled State
	// ========================================================================

	describe('Enabled/Disabled State', () => {
		it('can disable shortcuts globally', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }],
				enabled: true
			});

			// Disable
			hook.setEnabled(false);
			expect(hook.enabled).toBe(false);

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});

		it('can re-enable shortcuts', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }],
				enabled: false
			});

			// Enable
			hook.setEnabled(true);

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});

		it('can disable individual shortcut', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler, enabled: false }]
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Scope Filtering
	// ========================================================================

	describe('Scope Filtering', () => {
		it('filters shortcuts by scope', () => {
			const editorHandler = vi.fn();
			const globalHandler = vi.fn();

			void useKeyboardShortcuts({
				shortcuts: [
					{ key: 's', ctrl: true, handler: editorHandler, scope: 'editor' },
					{ key: 's', ctrl: true, handler: globalHandler, scope: 'global' }
				],
				scope: 'editor'
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(editorHandler).toHaveBeenCalled();
			expect(globalHandler).not.toHaveBeenCalled();
		});

		it('can change scope dynamically', () => {
			const editorHandler = vi.fn();
			const globalHandler = vi.fn();

			const hook = useKeyboardShortcuts({
				shortcuts: [
					{ key: 's', ctrl: true, handler: editorHandler, scope: 'editor' },
					{ key: 's', ctrl: true, handler: globalHandler, scope: 'global' }
				],
				scope: 'editor'
			});

			// Change to global scope
			hook.setScope('global');

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(globalHandler).toHaveBeenCalled();
			expect(editorHandler).not.toHaveBeenCalled();
		});

		it('shortcuts without scope work in any scope', () => {
			const handler = vi.fn();

			useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }], // No scope
				scope: 'editor'
			});

			const event = createKeyboardEvent('s', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).toHaveBeenCalled();
		});
	});

	// ========================================================================
	// Shortcut Management
	// ========================================================================

	describe('Shortcut Management', () => {
		it('adds new shortcut', () => {
			const hook = useKeyboardShortcuts({
				shortcuts: []
			});

			const handler = vi.fn();
			hook.addShortcut({ key: 'n', ctrl: true, handler });

			expect(hook.shortcuts).toHaveLength(1);
			expect(hook.shortcuts[0].key).toBe('n');
		});

		it('removes shortcut', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			hook.removeShortcut({ key: 's', ctrl: true });

			expect(hook.shortcuts).toHaveLength(0);
		});

		it('updates shortcut', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler, description: 'Old' }]
			});

			hook.updateShortcut('s', { description: 'New' });

			expect(hook.shortcuts[0].description).toBe('New');
		});

		it('clears all shortcuts', () => {
			const hook = useKeyboardShortcuts({
				shortcuts: [
					{ key: 's', ctrl: true, handler: vi.fn() },
					{ key: 'z', ctrl: true, handler: vi.fn() }
				]
			});

			hook.clearShortcuts();

			expect(hook.shortcuts).toHaveLength(0);
		});

		it('resets to initial shortcuts', () => {
			const initialShortcut = { key: 's', ctrl: true, handler: vi.fn() };
			const hook = useKeyboardShortcuts({
				shortcuts: [initialShortcut]
			});

			hook.addShortcut({ key: 'n', ctrl: true, handler: vi.fn() });
			expect(hook.shortcuts).toHaveLength(2);

			hook.resetShortcuts();

			expect(hook.shortcuts).toHaveLength(1);
		});

		it('prevents duplicate shortcuts', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler }]
			});

			hook.addShortcut({ key: 's', ctrl: true, handler });

			expect(hook.shortcuts).toHaveLength(1);
		});
	});

	// ========================================================================
	// Query Methods
	// ========================================================================

	describe('Query Methods', () => {
		it('getShortcut returns shortcut by key', () => {
			const handler = vi.fn();
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler, description: 'Save' }]
			});

			const shortcut = hook.getShortcut('s');

			expect(shortcut).toBeDefined();
			expect(shortcut?.description).toBe('Save');
		});

		it('getShortcutsByScope returns filtered shortcuts', () => {
			const hook = useKeyboardShortcuts({
				shortcuts: [
					{ key: 's', ctrl: true, handler: vi.fn(), scope: 'editor' },
					{ key: 'z', ctrl: true, handler: vi.fn(), scope: 'editor' },
					{ key: 'Escape', handler: vi.fn(), scope: 'global' }
				]
			});

			const editorShortcuts = hook.getShortcutsByScope('editor');

			expect(editorShortcuts).toHaveLength(2);
		});

		it('getShortcutsGroupedByScope returns grouped shortcuts', () => {
			const hook = useKeyboardShortcuts({
				shortcuts: [
					{ key: 's', ctrl: true, handler: vi.fn(), scope: 'editor' },
					{ key: 'Escape', handler: vi.fn(), scope: 'global' }
				]
			});

			const grouped = hook.getShortcutsGroupedByScope();

			expect(grouped['editor']).toHaveLength(1);
			expect(grouped['global']).toHaveLength(1);
		});

		it('hasConflict detects duplicate shortcuts', () => {
			const hook = useKeyboardShortcuts({
				shortcuts: [{ key: 's', ctrl: true, handler: vi.fn() }]
			});

			const hasConflict = hook.hasConflict({ key: 's', ctrl: true, handler: vi.fn() });

			expect(hasConflict).toBe(true);
		});
	});

	// ========================================================================
	// Format Shortcut
	// ========================================================================

	describe('formatShortcut', () => {
		it('formats simple key', () => {
			const hook = useKeyboardShortcuts({ shortcuts: [] });
			const result = hook.formatShortcut({ key: 'Escape', handler: vi.fn() });

			expect(result).toContain('Esc');
		});

		it('formats Ctrl+key', () => {
			const hook = useKeyboardShortcuts({ shortcuts: [] });
			const result = hook.formatShortcut({ key: 's', ctrl: true, handler: vi.fn() });

			expect(result).toMatch(/Ctrl|⌘/);
			expect(result).toContain('S');
		});

		it('formats Ctrl+Shift+key', () => {
			const hook = useKeyboardShortcuts({ shortcuts: [] });
			const result = hook.formatShortcut({ key: 'z', ctrl: true, shift: true, handler: vi.fn() });

			expect(result).toMatch(/Ctrl|⌘/);
			expect(result).toMatch(/Shift|⇧/);
		});
	});

	// ========================================================================
	// Modifier Key Handling
	// ========================================================================

	describe('Modifier Key Handling', () => {
		it('ignores modifier-only keypresses', () => {
			const handler = vi.fn();
			useKeyboardShortcuts({
				shortcuts: [{ key: 'Control', handler }]
			});

			const event = createKeyboardEvent('Control', { ctrlKey: true });
			document.dispatchEvent(event);

			expect(handler).not.toHaveBeenCalled();
		});
	});
});

// ============================================================================
// generateShortcutsHelp Tests
// ============================================================================

describe('generateShortcutsHelp', () => {
	it('groups shortcuts by scope', () => {
		const shortcuts: Shortcut[] = [
			{ key: 's', handler: vi.fn(), scope: 'editor', description: 'Save' },
			{ key: 'z', handler: vi.fn(), scope: 'editor', description: 'Undo' },
			{ key: 'Escape', handler: vi.fn(), scope: 'global', description: 'Close' }
		];

		const help = generateShortcutsHelp(shortcuts);

		expect(help).toHaveLength(2);
		expect(help.find((g) => g.name === 'Editor')).toBeDefined();
		expect(help.find((g) => g.name === 'Global')).toBeDefined();
	});

	it('filters out shortcuts without description', () => {
		const shortcuts: Shortcut[] = [
			{ key: 's', handler: vi.fn(), scope: 'editor', description: 'Save' },
			{ key: 'z', handler: vi.fn(), scope: 'editor' } // No description
		];

		const help = generateShortcutsHelp(shortcuts);
		const editorGroup = help.find((g) => g.name === 'Editor');

		expect(editorGroup?.shortcuts).toHaveLength(1);
	});

	it('capitalizes scope names', () => {
		const shortcuts: Shortcut[] = [
			{ key: 's', handler: vi.fn(), scope: 'editor', description: 'Save' }
		];

		const help = generateShortcutsHelp(shortcuts);

		expect(help[0].name).toBe('Editor');
	});

	it('uses "General" for unscoped shortcuts', () => {
		const shortcuts: Shortcut[] = [{ key: 's', handler: vi.fn(), description: 'Save' }];

		const help = generateShortcutsHelp(shortcuts);

		expect(help[0].name).toBe('General');
	});

	it('returns empty array for empty shortcuts', () => {
		const help = generateShortcutsHelp([]);

		expect(help).toHaveLength(0);
	});
});

// ============================================================================
// Preset Shortcuts Tests
// ============================================================================

describe('EDITOR_SHORTCUTS', () => {
	it('contains save shortcut (Ctrl+S)', () => {
		const save = EDITOR_SHORTCUTS.find((s) => s.key === 's' && s.ctrl);

		expect(save).toBeDefined();
		expect(save?.description).toBe('Save');
	});

	it('contains undo shortcut (Ctrl+Z)', () => {
		const undo = EDITOR_SHORTCUTS.find((s) => s.key === 'z' && s.ctrl && !s.shift);

		expect(undo).toBeDefined();
		expect(undo?.description).toBe('Undo');
	});

	it('contains redo shortcut (Ctrl+Shift+Z)', () => {
		const redo = EDITOR_SHORTCUTS.find((s) => s.key === 'z' && s.ctrl && s.shift);

		expect(redo).toBeDefined();
		expect(redo?.description).toBe('Redo');
	});

	it('contains formatting shortcuts', () => {
		const bold = EDITOR_SHORTCUTS.find((s) => s.key === 'b' && s.ctrl);
		const italic = EDITOR_SHORTCUTS.find((s) => s.key === 'i' && s.ctrl);
		const underline = EDITOR_SHORTCUTS.find((s) => s.key === 'u' && s.ctrl);

		expect(bold).toBeDefined();
		expect(italic).toBeDefined();
		expect(underline).toBeDefined();
	});

	it('all shortcuts have editor scope', () => {
		EDITOR_SHORTCUTS.forEach((shortcut) => {
			expect(shortcut.scope).toBe('editor');
		});
	});

	it('all shortcuts have handlers', () => {
		EDITOR_SHORTCUTS.forEach((shortcut) => {
			expect(typeof shortcut.handler).toBe('function');
		});
	});
});

describe('NAVIGATION_SHORTCUTS', () => {
	it('contains arrow navigation', () => {
		const up = NAVIGATION_SHORTCUTS.find((s) => s.key === 'ArrowUp');
		const down = NAVIGATION_SHORTCUTS.find((s) => s.key === 'ArrowDown');

		expect(up).toBeDefined();
		expect(down).toBeDefined();
	});

	it('all shortcuts have navigation scope', () => {
		NAVIGATION_SHORTCUTS.forEach((shortcut) => {
			expect(shortcut.scope).toBe('navigation');
		});
	});
});

describe('GLOBAL_SHORTCUTS', () => {
	it('contains help shortcut (Shift+?)', () => {
		const help = GLOBAL_SHORTCUTS.find((s) => s.key === '?' && s.shift);

		expect(help).toBeDefined();
		expect(help?.description).toBe('Show Keyboard Shortcuts');
	});

	it('contains escape shortcut', () => {
		const escape = GLOBAL_SHORTCUTS.find((s) => s.key === 'Escape');

		expect(escape).toBeDefined();
	});

	it('all shortcuts have global scope', () => {
		GLOBAL_SHORTCUTS.forEach((shortcut) => {
			expect(shortcut.scope).toBe('global');
		});
	});
});

// ============================================================================
// Type Tests
// ============================================================================

describe('Shortcut Type', () => {
	it('accepts minimal properties', () => {
		const shortcut: Shortcut = {
			key: 'a',
			handler: vi.fn()
		};

		expect(shortcut.key).toBe('a');
	});

	it('accepts all modifier keys', () => {
		const shortcut: Shortcut = {
			key: 'a',
			ctrl: true,
			shift: true,
			alt: true,
			meta: true,
			handler: vi.fn()
		};

		expect(shortcut.ctrl).toBe(true);
		expect(shortcut.shift).toBe(true);
		expect(shortcut.alt).toBe(true);
		expect(shortcut.meta).toBe(true);
	});

	it('accepts all optional properties', () => {
		const shortcut: Shortcut = {
			key: 'a',
			handler: vi.fn(),
			description: 'Test',
			preventDefault: false,
			stopPropagation: true,
			enabled: true,
			scope: 'test',
			priority: 50
		};

		expect(shortcut.description).toBe('Test');
		expect(shortcut.priority).toBe(50);
	});
});

describe('KeyboardShortcutsOptions Type', () => {
	it('accepts all options', () => {
		const options: KeyboardShortcutsOptions = {
			shortcuts: [{ key: 'a', handler: vi.fn() }],
			enabled: true,
			target: document,
			eventType: 'keydown',
			scope: 'editor',
			capture: false
		};

		expect(options.shortcuts).toHaveLength(1);
		expect(options.enabled).toBe(true);
	});
});

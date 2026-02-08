/**
 * CMS Block Hooks Tests - Setup
 * ===============================================================================
 * Test setup for CMS block hooks with Svelte 5 runes mocking
 *
 * This setup file handles the mocking of Svelte 5 reactive primitives ($state, $derived, $effect)
 * which cannot be used outside Svelte component context.
 *
 * IMPORTANT: The Svelte compiler transforms .svelte.ts runes into internal runtime calls:
 *   $state(v)    → state(v)        from 'svelte/internal/client'
 *   $derived(v)  → user_derived(v) from 'svelte/internal/client'
 *   $effect(fn)  → user_effect(fn) from 'svelte/internal/client'
 *
 * Global mocks (globalThis.$effect = ...) are BYPASSED because the compiler
 * replaces runes before runtime. We must mock 'svelte/internal/client' instead.
 */

import { expect, afterEach, vi, beforeAll, afterAll } from 'vitest';
import { cleanup } from '@testing-library/svelte';
import * as matchers from '@testing-library/jest-dom/matchers';
import { applySvelteInternalMock, runEffectCleanups } from '../../../../../../test/svelte-internal-mock';

// Apply the Svelte internal mock BEFORE any .svelte.ts imports
applySvelteInternalMock();

// Extend Vitest's expect with jest-dom matchers
expect.extend(matchers);

// Cleanup after each test
afterEach(() => {
	cleanup();
	runEffectCleanups();
});

// ===============================================================================
// Browser API Mocks
// ===============================================================================

// Mock window.matchMedia
Object.defineProperty(window, 'matchMedia', {
	writable: true,
	value: vi.fn().mockImplementation((query) => ({
		matches: false,
		media: query,
		onchange: null,
		addListener: vi.fn(),
		removeListener: vi.fn(),
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn()
	}))
});

// Mock IntersectionObserver
global.IntersectionObserver = class IntersectionObserver {
	disconnect() {}
	observe() {}
	takeRecords() {
		return [];
	}
	unobserve() {}
} as unknown as typeof IntersectionObserver;

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
	disconnect() {}
	observe() {}
	unobserve() {}
} as unknown as typeof ResizeObserver;

// Mock Image so onload fires synchronously (JSDOM doesn't support Image loading)
const OriginalImage = global.Image;
class MockImage {
	src = '';
	naturalWidth = 800;
	naturalHeight = 600;
	width = 800;
	height = 600;
	onload: (() => void) | null = null;
	onerror: (() => void) | null = null;

	set _src(value: string) {
		this.src = value;
		// Fire onload asynchronously to simulate real behavior
		if (this.onload) {
			setTimeout(() => this.onload?.(), 0);
		}
	}

	constructor() {
		// Use a proxy to intercept src assignment
		return new Proxy(this, {
			set(target, prop, value) {
				if (prop === 'src') {
					target.src = value;
					if (target.onload) {
						setTimeout(() => target.onload?.(), 0);
					}
					return true;
				}
				return Reflect.set(target, prop, value);
			}
		});
	}
}
global.Image = MockImage as unknown as typeof Image;

// Mock HTMLCanvasElement.getContext and toBlob
HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({
	drawImage: vi.fn()
}) as any;
HTMLCanvasElement.prototype.toBlob = vi.fn(function (
	this: HTMLCanvasElement,
	callback: BlobCallback
) {
	callback(new Blob(['mock-thumbnail'], { type: 'image/jpeg' }));
}) as any;

// Mock URL.createObjectURL and revokeObjectURL
const mockObjectURLs = new Map<string, Blob>();
let objectURLCounter = 0;

URL.createObjectURL = vi.fn((blob: Blob) => {
	const url = `blob:mock-url-${++objectURLCounter}`;
	mockObjectURLs.set(url, blob);
	return url;
});

URL.revokeObjectURL = vi.fn((url: string) => {
	mockObjectURLs.delete(url);
});

// Mock clipboard API
Object.assign(navigator, {
	clipboard: {
		writeText: vi.fn().mockResolvedValue(undefined),
		readText: vi.fn().mockResolvedValue('')
	}
});

// Mock crypto
if (!global.crypto) {
	global.crypto = {
		getRandomValues: (arr: Uint8Array) => {
			for (let i = 0; i < arr.length; i++) {
				arr[i] = Math.floor(Math.random() * 256);
			}
			return arr;
		},
		randomUUID: () => {
			return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
				const r = (Math.random() * 16) | 0;
				const v = c === 'x' ? r : (r & 0x3) | 0x8;
				return v.toString(16);
			});
		}
	} as unknown as Crypto;
}

// ===============================================================================
// Console Suppression
// ===============================================================================

const originalError = console.error;
const originalWarn = console.warn;

beforeAll(() => {
	// Suppress expected console errors
	console.error = (...args: unknown[]) => {
		if (
			typeof args[0] === 'string' &&
			(args[0].includes('Not implemented') ||
				args[0].includes('Error loading') ||
				args[0].includes('Context') ||
				args[0].includes('$state') ||
				args[0].includes('$derived') ||
				args[0].includes('$effect'))
		) {
			return;
		}
		originalError.call(console, ...args);
	};

	// Suppress expected console warnings
	console.warn = (...args: unknown[]) => {
		if (
			typeof args[0] === 'string' &&
			(args[0].includes('Unsafe URL') || args[0].includes('deprecated'))
		) {
			return;
		}
		originalWarn.call(console, ...args);
	};
});

afterAll(() => {
	console.error = originalError;
	console.warn = originalWarn;
});

// ===============================================================================
// Test Utilities
// ===============================================================================

/**
 * Creates a mock File object for testing
 */
export function createMockFile(
	name: string = 'test.jpg',
	type: string = 'image/jpeg',
	size: number = 1024
): File {
	const content = new Array(size).fill('a').join('');
	return new File([content], name, { type });
}

/**
 * Creates a mock block for validation testing
 */
export function createMockBlock(overrides: Record<string, unknown> = {}) {
	return {
		id: `test-${Date.now()}`,
		type: 'paragraph' as const,
		content: { text: '' },
		settings: {},
		metadata: {
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
			version: 1
		},
		...overrides
	};
}

/**
 * Creates a mock drag event
 */
export function createDragEvent(files: File[]): DragEvent {
	return {
		preventDefault: vi.fn(),
		stopPropagation: vi.fn(),
		dataTransfer: {
			files,
			items: files.map((file) => ({
				kind: 'file',
				type: file.type,
				getAsFile: () => file
			})),
			types: ['Files']
		}
	} as unknown as DragEvent;
}

/**
 * Creates a mock keyboard event
 */
export function createKeyboardEvent(
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

/**
 * Waits for async operations to complete
 */
export async function waitFor(ms: number = 0): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Flushes all pending promises
 */
export async function flushPromises(): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, 0));
}

// ===============================================================================
// Type Declarations for Globals
// ===============================================================================

// Note: Svelte 5 runes ($state, $derived, $effect) are compile-time constructs
// and their types are provided by Svelte itself. The mocks above are assigned
// to globalThis for test runtime compatibility.

/**
 * Test Setup
 * ═══════════════════════════════════════════════════════════════════════════
 * Global test configuration and utilities
 */

import { expect, afterEach, vi, beforeAll, afterAll } from 'vitest';
import { cleanup } from '@testing-library/svelte';
import * as matchers from '@testing-library/jest-dom/matchers';

// Extend Vitest's expect with jest-dom matchers
expect.extend(matchers);

// Cleanup after each test
afterEach(() => {
	cleanup();
});

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
	constructor() {}
	disconnect() {}
	observe() {}
	takeRecords() {
		return [];
	}
	unobserve() {}
} as unknown as typeof IntersectionObserver;

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
	constructor() {}
	disconnect() {}
	observe() {}
	unobserve() {}
} as unknown as typeof ResizeObserver;

// Mock performance APIs
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const perf = performance as any;
if (!perf.mark) perf.mark = vi.fn();
if (!perf.measure) perf.measure = vi.fn();
if (!perf.getEntriesByName) perf.getEntriesByName = vi.fn().mockReturnValue([]);
if (!perf.clearMarks) perf.clearMarks = vi.fn();
if (!perf.clearMeasures) perf.clearMeasures = vi.fn();

// Mock crypto for nonce generation
if (!global.crypto) {
	global.crypto = {
		getRandomValues: (arr: Uint8Array) => {
			for (let i = 0; i < arr.length; i++) {
				arr[i] = Math.floor(Math.random() * 256);
			}
			return arr;
		}
	} as unknown as Crypto;
}

// Mock clipboard API
Object.assign(navigator, {
	clipboard: {
		writeText: vi.fn().mockResolvedValue(undefined),
		readText: vi.fn().mockResolvedValue('')
	}
});

// Suppress console errors in tests (optional)
const originalError = console.error;
beforeAll(() => {
	console.error = (...args: unknown[]) => {
		if (
			typeof args[0] === 'string' &&
			args[0].includes('Not implemented: HTMLFormElement.prototype.submit')
		) {
			return;
		}
		originalError.call(console, ...args);
	};
});

afterAll(() => {
	console.error = originalError;
});

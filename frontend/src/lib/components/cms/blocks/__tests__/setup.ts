/**
 * CMS Block Tests - Setup
 * ===============================================================================
 * Test setup and utilities for CMS block component tests
 * Apple Principal Engineer ICT 7+ Standard
 */

import { expect, afterEach, vi, beforeAll, afterAll, beforeEach } from 'vitest';
import { cleanup } from '@testing-library/svelte';
import * as matchers from '@testing-library/jest-dom/matchers';
import { BlockStateManager } from '$lib/stores/blockState.svelte';

// Extend Vitest's expect with jest-dom matchers
expect.extend(matchers);

// Restore real document for @testing-library/svelte
// The observability setup mocks document which breaks component rendering
beforeEach(() => {
	// Ensure we have proper DOM elements - restore if mocked
	if (typeof document !== 'undefined' && !document.body) {
		// Restore the real document from jsdom if it was mocked
		vi.unstubAllGlobals();
	}
});

// Cleanup after each test
afterEach(() => {
	cleanup();
});

// Safe window property assignment
if (typeof window !== 'undefined') {
	// Mock window.matchMedia
	Object.defineProperty(window, 'matchMedia', {
		writable: true,
		configurable: true,
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
}

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

// Mock URL.createObjectURL
URL.createObjectURL = vi.fn().mockReturnValue('blob:mock-url');
URL.revokeObjectURL = vi.fn();

// Mock clipboard API
if (typeof navigator !== 'undefined') {
	Object.defineProperty(navigator, 'clipboard', {
		writable: true,
		configurable: true,
		value: {
			writeText: vi.fn().mockResolvedValue(undefined),
			readText: vi.fn().mockResolvedValue('')
		}
	});
}

// Suppress console errors for expected test scenarios
const originalError = console.error;
beforeAll(() => {
	// Unstub globals that may have been stubbed by other setup files
	vi.unstubAllGlobals();

	console.error = (...args: unknown[]) => {
		if (
			typeof args[0] === 'string' &&
			(args[0].includes('Not implemented') ||
				args[0].includes('Error loading') ||
				args[0].includes('Context'))
		) {
			return;
		}
		originalError.call(console, ...args);
	};
});

afterAll(() => {
	console.error = originalError;
});

// ===============================================================================
// Test Utilities
// ===============================================================================

/**
 * Creates a mock BlockStateManager for testing
 */
export function createMockStateManager(): BlockStateManager {
	return new BlockStateManager();
}

/**
 * Creates a mock block object with default values
 */
export function createMockBlock(overrides: Record<string, unknown> = {}) {
	return {
		id: `test-${Date.now()}`,
		type: 'image',
		content: {},
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
 * Creates a mock File object for upload testing
 */
export function createMockFile(
	name: string,
	type: string,
	size: number = 1024
): File {
	const content = new Array(size).fill('a').join('');
	return new File([content], name, { type });
}

/**
 * Simulates a file drop event
 */
export function createDropEvent(files: File[]): DragEvent {
	const dataTransfer = {
		files,
		items: files.map((file) => ({
			kind: 'file',
			type: file.type,
			getAsFile: () => file
		})),
		types: ['Files']
	};

	return {
		preventDefault: vi.fn(),
		stopPropagation: vi.fn(),
		dataTransfer
	} as unknown as DragEvent;
}

/**
 * Wait for async state updates
 */
export async function waitForState(ms: number = 100): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, ms));
}

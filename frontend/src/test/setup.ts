import { expect, afterEach, vi, beforeAll, afterAll } from 'vitest';
import { cleanup } from '@testing-library/svelte';
import * as matchers from '@testing-library/jest-dom/matchers';

expect.extend(matchers);

afterEach(() => {
	cleanup();
});

// Mock window.matchMedia (use plain function, not vi.fn(), so mockReset doesn't strip it)
Object.defineProperty(window, 'matchMedia', {
	writable: true,
	value: (query: string) => ({
		matches: false,
		media: query,
		onchange: null,
		addListener: () => {},
		removeListener: () => {},
		addEventListener: () => {},
		removeEventListener: () => {},
		dispatchEvent: () => false
	})
});

// Mock IntersectionObserver
global.IntersectionObserver = class IntersectionObserver {
	disconnect() {}
	observe() {}
	takeRecords() {
		return [];
	}
	unobserve() {}
} as any;

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
	disconnect() {}
	observe() {}
	unobserve() {}
} as any;

// Mock performance APIs
const perf = performance as any;
if (!('mark' in perf)) {
	perf.mark = vi.fn();
}
if (!('measure' in perf)) {
	perf.measure = vi.fn();
}
if (!('getEntriesByName' in perf)) {
	perf.getEntriesByName = vi.fn().mockReturnValue([]);
}
if (!('clearMarks' in perf)) {
	perf.clearMarks = vi.fn();
}
if (!('clearMeasures' in perf)) {
	perf.clearMeasures = vi.fn();
}

// Mock crypto
if (!global.crypto) {
	global.crypto = {
		getRandomValues: (arr: Uint8Array) => {
			for (let i = 0; i < arr.length; i++) {
				arr[i] = Math.floor(Math.random() * 256);
			}
			return arr;
		}
	} as any;
}

// Mock HTMLCanvasElement.toDataURL (returns null in JSDOM, breaks image format detection)
HTMLCanvasElement.prototype.toDataURL = function (type?: string) {
	return `data:${type || 'image/png'};base64,`;
};

// Polyfill DataTransfer (not available in JSDOM)
if (typeof globalThis.DataTransfer === 'undefined') {
	globalThis.DataTransfer = class DataTransfer {
		private data: Map<string, string> = new Map();
		items: any[] = [];
		files: any[] = [];
		types: string[] = [];
		dropEffect = 'none';
		effectAllowed = 'all';
		setData(format: string, data: string) {
			this.data.set(format, data);
			if (!this.types.includes(format)) this.types.push(format);
		}
		getData(format: string) {
			return this.data.get(format) || '';
		}
		clearData() {
			this.data.clear();
			this.types = [];
		}
		setDragImage() {}
	} as any;
}

// Polyfill ClipboardEvent (not available in JSDOM)
if (typeof globalThis.ClipboardEvent === 'undefined') {
	globalThis.ClipboardEvent = class ClipboardEvent extends Event {
		clipboardData: DataTransfer | null;
		constructor(type: string, eventInitDict?: ClipboardEventInit) {
			super(type, eventInitDict);
			this.clipboardData = eventInitDict?.clipboardData ?? null;
		}
	} as any;
}

// Mock clipboard
Object.assign(navigator, {
	clipboard: {
		writeText: vi.fn().mockResolvedValue(undefined),
		readText: vi.fn().mockResolvedValue('')
	}
});

// Suppress console errors for expected test failures
const originalError = console.error;
beforeAll(() => {
	console.error = (...args: any[]) => {
		if (
			typeof args[0] === 'string' &&
			(args[0].includes('Not implemented: HTMLFormElement.prototype.submit') ||
				args[0].includes('Not implemented: HTMLFormElement.prototype.requestSubmit'))
		) {
			return;
		}
		originalError.call(console, ...args);
	};
});

afterAll(() => {
	console.error = originalError;
});

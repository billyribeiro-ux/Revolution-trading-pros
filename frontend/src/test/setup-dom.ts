/**
 * DOM-environment test setup (jsdom): browser API mocks and polyfills.
 *
 * Loaded by `setup.ts` only when the test environment provides a DOM, so test
 * files that opt into `// @vitest-environment node` — server-only code that must
 * not depend on a DOM — share the rest of the setup.
 */
import { vi } from 'vitest';

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
} as unknown as typeof IntersectionObserver;

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
	disconnect() {}
	observe() {}
	unobserve() {}
} as unknown as typeof ResizeObserver;

// Mock HTMLCanvasElement.toDataURL (returns null in JSDOM, breaks image format detection)
HTMLCanvasElement.prototype.toDataURL = function (type?: string) {
	return `data:${type || 'image/png'};base64,`;
};

// Mock HTMLCanvasElement.getContext - jsdom returns null; tests with image upload helpers
// trigger production code that throws "Failed to get canvas context" otherwise.
// The cast is wide because getContext is overloaded across multiple context kinds.
(HTMLCanvasElement.prototype as { getContext: unknown }).getContext = function () {
	return {
		drawImage: () => {},
		fillRect: () => {},
		clearRect: () => {},
		getImageData: () => ({ data: new Uint8ClampedArray() }),
		putImageData: () => {},
		createImageData: () => ({ data: new Uint8ClampedArray() }),
		setTransform: () => {},
		fillText: () => {},
		strokeText: () => {},
		measureText: () => ({ width: 0 }),
		beginPath: () => {},
		closePath: () => {},
		moveTo: () => {},
		lineTo: () => {},
		stroke: () => {},
		fill: () => {},
		arc: () => {},
		rect: () => {},
		save: () => {},
		restore: () => {},
		scale: () => {},
		rotate: () => {},
		translate: () => {}
	};
};

// Polyfill DataTransfer (not available in JSDOM)
if (typeof globalThis.DataTransfer === 'undefined') {
	globalThis.DataTransfer = class DataTransfer {
		private data: Map<string, string> = new Map();
		items: unknown[] = [];
		files: unknown[] = [];
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
	} as unknown as typeof DataTransfer;
}

// Polyfill document.execCommand (removed in jsdom 29 — still used by ParagraphBlock paste handler)
if (typeof document.execCommand !== 'function') {
	document.execCommand = (() => true) as typeof document.execCommand;
}

// Polyfill ClipboardEvent (not available in JSDOM)
if (typeof globalThis.ClipboardEvent === 'undefined') {
	globalThis.ClipboardEvent = class ClipboardEvent extends Event {
		clipboardData: DataTransfer | null;
		constructor(type: string, eventInitDict?: ClipboardEventInit) {
			super(type, eventInitDict);
			this.clipboardData = eventInitDict?.clipboardData ?? null;
		}
	} as unknown as typeof ClipboardEvent;
}

// Mock clipboard
Object.assign(navigator, {
	clipboard: {
		writeText: vi.fn().mockResolvedValue(undefined),
		readText: vi.fn().mockResolvedValue('')
	}
});

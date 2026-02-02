/**
 * Vitest Setup - Global Mocks
 * Uses vi.stubGlobal for proper vitest integration
 */

import { vi } from 'vitest';

// Mock localStorage
const mockLocalStorage = {
	data: {} as Record<string, string>,
	getItem: vi.fn((key: string) => mockLocalStorage.data[key] || null),
	setItem: vi.fn((key: string, value: string) => {
		mockLocalStorage.data[key] = value;
	}),
	removeItem: vi.fn((key: string) => {
		delete mockLocalStorage.data[key];
	}),
	clear: vi.fn(() => {
		mockLocalStorage.data = {};
	}),
	get length() {
		return Object.keys(mockLocalStorage.data).length;
	},
	key: vi.fn((index: number) => Object.keys(mockLocalStorage.data)[index] || null)
};

// Mock timers
let timerId = 0;
const mockSetTimeout = vi.fn((cb: () => void, ms?: number) => {
	const id = ++timerId;
	if (ms === 0 || ms === undefined) {
		Promise.resolve().then(cb);
	}
	return id;
});
const mockSetInterval = vi.fn((_cb: () => void, _ms?: number) => {
	return ++timerId;
});
const mockClearTimeout = vi.fn();
const mockClearInterval = vi.fn();

// Mock window object
const mockWindow = {
	location: {
		href: 'https://example.com/test-page',
		pathname: '/test-page',
		origin: 'https://example.com'
	},
	dataLayer: [] as unknown[],
	gtag: vi.fn((...args: unknown[]) => {
		mockWindow.dataLayer.push(args);
	}),
	addEventListener: vi.fn(),
	removeEventListener: vi.fn(),
	requestIdleCallback: vi.fn(
		(cb: (deadline: { didTimeout: boolean; timeRemaining: () => number }) => void) => {
			cb({ didTimeout: false, timeRemaining: () => 50 });
			return 1;
		}
	),
	requestAnimationFrame: vi.fn((cb: () => void) => {
		cb();
		return 1;
	}),
	cancelAnimationFrame: vi.fn(),
	screen: { width: 1920, height: 1080 },
	innerWidth: 1920,
	innerHeight: 1080,
	localStorage: mockLocalStorage,
	setTimeout: mockSetTimeout,
	setInterval: mockSetInterval,
	clearTimeout: mockClearTimeout,
	clearInterval: mockClearInterval
};

// Mock document object with script loading simulation
const mockDocument = {
	title: 'Test Page',
	referrer: 'https://google.com',
	head: {
		appendChild: vi.fn((element: { onload?: (() => void) | null }) => {
			// Simulate async script load
			if (element.onload) {
				Promise.resolve().then(() => element.onload!());
			}
		})
	},
	createElement: vi.fn(() => {
		const element = {
			id: '',
			src: '',
			async: false,
			defer: false,
			onload: null as (() => void) | null,
			onerror: null as (() => void) | null,
			setAttribute: vi.fn()
		};
		return element;
	}),
	getElementById: vi.fn(() => null),
	visibilityState: 'visible' as DocumentVisibilityState,
	addEventListener: vi.fn(),
	removeEventListener: vi.fn()
};

// Mock navigator object
const mockNavigator = {
	userAgent: 'TestUserAgent/1.0',
	language: 'en-US',
	sendBeacon: vi.fn(() => true)
};

// Mock Intl
const mockIntl = {
	DateTimeFormat: vi.fn(() => ({
		resolvedOptions: () => ({ timeZone: 'America/New_York' })
	}))
};

// Mock fetch
const mockFetch = vi.fn(() => Promise.resolve({ ok: true }));

// Use vi.stubGlobal for proper vitest integration
// Note: Only stub globals that don't break jsdom for component tests
vi.stubGlobal('localStorage', mockLocalStorage);
vi.stubGlobal('fetch', mockFetch);

// Only stub window/document/navigator if NOT in jsdom environment
// This allows @testing-library/svelte component tests to work properly
const isJsdom = typeof document !== 'undefined' && document.body !== undefined;

if (!isJsdom) {
	vi.stubGlobal('window', mockWindow);
	vi.stubGlobal('document', mockDocument);
	vi.stubGlobal('navigator', mockNavigator);
}

vi.stubGlobal('Intl', mockIntl);

// Export mocks for test assertions
export {
	mockLocalStorage,
	mockWindow,
	mockDocument,
	mockNavigator,
	mockIntl,
	mockFetch,
	mockSetTimeout,
	mockSetInterval,
	mockClearTimeout,
	mockClearInterval
};

// Reset function for use in beforeEach
export function resetMocks() {
	mockLocalStorage.data = {};
	mockWindow.dataLayer = [];
	vi.clearAllMocks();
}

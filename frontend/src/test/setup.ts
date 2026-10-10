import { afterEach, vi, beforeAll, afterAll } from 'vitest';
import { cleanup } from '@testing-library/svelte';
// `/vitest` entry registers jest-dom matchers at runtime AND extends
// Vitest's `Assertion` interface with their TypeScript types, so
// .test.ts files type-check correctly for `toBeInTheDocument`,
// `toHaveAttribute`, `toHaveStyle`, etc. The previous `/matchers`
// import only added runtime matchers — types were missing, surfacing
// as ~686 svelte-check errors across 18 __tests__/ files.
// https://github.com/testing-library/jest-dom#with-vitest
import '@testing-library/jest-dom/vitest';

afterEach(() => {
	cleanup();
});

// Browser API mocks — only where the environment has a DOM (jsdom). Files that
// declare `// @vitest-environment node` run without them.
if (typeof window !== 'undefined') {
	await import('./setup-dom.js');
}

// Mock performance APIs
const perf = performance as unknown as Record<string, unknown>;
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
	} as unknown as Crypto;
}

// Suppress console errors for expected test failures
const originalError = console.error;
beforeAll(() => {
	console.error = (...args: unknown[]) => {
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

/**
 * Svelte 5 Internal Client Mock for Vitest
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * When Svelte compiles .svelte.ts files, it transforms runes into internal
 * runtime calls:
 *   $state(v)    → state(v)      from 'svelte/internal/client'
 *   $derived(v)  → derived(v)    from 'svelte/internal/client'
 *   $effect(fn)  → user_effect(fn) from 'svelte/internal/client'
 *   $props()     → prop(...)     from 'svelte/internal/client'
 *
 * The real runtime validates that $effect runs inside a component context
 * (active_effect !== null), which throws `effect_orphan` in unit tests.
 *
 * This mock provides test-safe implementations that execute synchronously
 * without requiring a Svelte component context.
 */

import { vi } from 'vitest';

// ═══════════════════════════════════════════════════════════════════════════════
// REACTIVE STATE MOCK
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Simple reactive source mock. In tests, we use a plain object with a .v
 * property to simulate Svelte's Source<T> signal.
 */
interface MockSource<T = unknown> {
	f: number;
	v: T;
	reactions: null;
	equals: (a: unknown, b: unknown) => boolean;
	rv: number;
	wv: number;
	/** If present, this is a derived source that re-evaluates on get() */
	_derivedFn?: () => T;
}

function createMockSource<T>(value: T): MockSource<T> {
	return {
		f: 0,
		v: value,
		reactions: null,
		equals: (a, b) => Object.is(a, b),
		rv: 0,
		wv: 0
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// EFFECT CLEANUP TRACKING
// ═══════════════════════════════════════════════════════════════════════════════

const effectCleanups: Array<() => void> = [];

export function runEffectCleanups(): void {
	for (const cleanup of effectCleanups) {
		try {
			cleanup();
		} catch {
			// Ignore cleanup errors in tests
		}
	}
	effectCleanups.length = 0;
}

// ═══════════════════════════════════════════════════════════════════════════════
// MOCK IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

/** Mock for $state(value) — compiled to state(value) */
export function mockState<T>(value: T): MockSource<T> {
	return createMockSource(value);
}

/** Mock for get(signal) — returns the signal's value, re-evaluating derived sources */
export function mockGet<T>(signal: MockSource<T> | T): T {
	if (signal && typeof signal === 'object' && 'v' in (signal as object)) {
		const src = signal as MockSource<T>;
		// Re-evaluate derived sources on every get() so mock changes propagate
		if (src._derivedFn) {
			try {
				src.v = src._derivedFn();
			} catch {
				// Derived may fail in test context
			}
		}
		return src.v;
	}
	return signal as T;
}

/** Mock for set(signal, value) — sets the signal's value */
export function mockSet<T>(signal: MockSource<T>, value: T): T {
	signal.v = value;
	return value;
}

/** Mock for $effect(fn) — compiled to user_effect(fn) */
export function mockUserEffect(fn: () => void | (() => void)): void {
	try {
		const cleanup = fn();
		if (typeof cleanup === 'function') {
			effectCleanups.push(cleanup);
		}
	} catch {
		// Effects may reference DOM or other unavailable APIs in tests
	}
}

/** Mock for $effect.pre(fn) — compiled to user_pre_effect(fn) */
export function mockUserPreEffect(fn: () => void | (() => void)): void {
	mockUserEffect(fn);
}

/** Mock for $derived(expr) — compiled to derived(fn). Stores fn for lazy re-evaluation. */
export function mockDerived<T>(fn: () => T): MockSource<T> {
	let initialValue: T;
	try {
		initialValue = fn();
	} catch {
		initialValue = undefined as T;
	}
	const source = createMockSource(initialValue);
	source._derivedFn = fn;
	return source;
}

/** Mock for push/pop component context */
export function mockPush(): void {}
export function mockPop(): void {}

// ═══════════════════════════════════════════════════════════════════════════════
// APPLY MOCK TO VITEST
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Call this in your test setup to mock `svelte/internal/client`.
 * Must be called BEFORE any .svelte.ts imports.
 */
export function applySvelteInternalMock(): void {
	vi.mock('svelte/internal/client', async (importOriginal) => {
		const original = (await importOriginal()) as Record<string, unknown>;
		return {
			...original,
			// Override the functions that throw outside component context
			state: mockState,
			mutable_source: mockState,
			set: mockSet,
			get: mockGet,
			safe_get: mockGet,
			user_effect: mockUserEffect,
			user_pre_effect: mockUserPreEffect,
			effect: mockUserEffect,
			render_effect: mockUserEffect,
			template_effect: vi.fn(),
			deferred_template_effect: vi.fn(),
			derived: mockDerived,
			user_derived: mockDerived,
			derived_safe_equal: mockDerived,
			async_derived: mockDerived,
			push: mockPush,
			pop: mockPop,
			push_reaction_value: vi.fn(),
			effect_root: (fn: () => void) => {
				fn();
				return () => {};
			},
			effect_tracking: () => false,
			untrack: <T>(fn: () => T) => fn(),
			prop: (_props: Record<string, unknown>, key: string, _flags?: number, fallback?: unknown) => {
				return _props?.[key] ?? fallback;
			},
			rest_props: (props: Record<string, unknown>, ...keys: string[]) => {
				const rest: Record<string, unknown> = {};
				for (const key of Object.keys(props)) {
					if (!keys.includes(key)) {
						rest[key] = props[key];
					}
				}
				return rest;
			},
			spread_props: (...sources: Record<string, unknown>[]) => {
				return Object.assign({}, ...sources);
			},
			mutate: (_source: MockSource, value: unknown) => value,
			update: vi.fn(),
			update_pre: vi.fn(),
			// Snapshot just returns the value
			snapshot: <T>(value: T) => value
		};
	});
}

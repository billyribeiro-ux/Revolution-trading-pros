/**
 * ===============================================================================
 * Test Helper Utilities
 * ===============================================================================
 *
 * @description Utility functions for testing Explosive Swings components
 * @version 1.0.0
 * @standards Apple Principal Engineer ICT 7+ | Vitest January 2026 Patterns
 *
 * This module provides:
 * - Component rendering helpers with providers
 * - Async test utilities
 * - DOM query helpers
 * - Event simulation helpers
 */

import { vi } from 'vitest';

// ===============================================================================
// COMPONENT RENDERING HELPERS
// ===============================================================================

/**
 * Render a Svelte component with common providers/context
 *
 * Note: Since @testing-library/svelte may not be available,
 * this provides a minimal implementation. For full testing,
 * install @testing-library/svelte.
 *
 * @example
 * ```ts
 * const { component, container } = renderWithProviders(MyComponent, {
 *   isAdmin: true,
 *   initialData: mockData
 * });
 * ```
 */
export function renderWithProviders<T extends Record<string, unknown>>(
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	component: any,
	props: T = {} as T,
	options: RenderOptions = {}
) {
	const container = options.container ?? document.createElement('div');
	document.body.appendChild(container);

	// Create component instance
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const instance = new (component as any)({
		target: container,
		props
	});

	return {
		component: instance,
		container,
		unmount: () => {
			instance.$destroy();
			container.remove();
		},
		rerender: (newProps: Partial<T>) => {
			instance.$set(newProps);
		}
	};
}

export interface RenderOptions {
	container?: HTMLElement;
}

// ===============================================================================
// ASYNC UTILITIES
// ===============================================================================

/**
 * Wait for all pending microtasks and promises to resolve
 */
export function flushPromises(): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, 0));
}

/**
 * Wait for the next animation frame
 */
export function waitForAnimationFrame(): Promise<void> {
	return new Promise((resolve) => requestAnimationFrame(() => resolve()));
}

/**
 * Wait for loading states to finish (polling approach)
 *
 * @param checkFn - Function that returns true when loading is complete
 * @param timeout - Maximum wait time in ms (default: 5000)
 * @param interval - Polling interval in ms (default: 50)
 */
export async function waitForLoadingToFinish(
	checkFn: () => boolean = () => true,
	timeout = 5000,
	interval = 50
): Promise<void> {
	const startTime = Date.now();

	while (!checkFn()) {
		if (Date.now() - startTime > timeout) {
			throw new Error(`Timed out waiting for loading to finish after ${timeout}ms`);
		}
		await new Promise((resolve) => setTimeout(resolve, interval));
	}
}

/**
 * Wait for a specific condition to be true
 */
export async function waitFor(
	condition: () => boolean | Promise<boolean>,
	options: { timeout?: number; interval?: number } = {}
): Promise<void> {
	const { timeout = 5000, interval = 50 } = options;
	const startTime = Date.now();

	while (!(await condition())) {
		if (Date.now() - startTime > timeout) {
			throw new Error(`Condition not met within ${timeout}ms`);
		}
		await new Promise((resolve) => setTimeout(resolve, interval));
	}
}

/**
 * Wait for specified milliseconds
 */
export function wait(ms: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Create a deferred promise for testing async flows
 */
export function createDeferred<T = void>() {
	let resolve!: (value: T | PromiseLike<T>) => void;
	let reject!: (reason?: unknown) => void;

	const promise = new Promise<T>((res, rej) => {
		resolve = res;
		reject = rej;
	});

	return { promise, resolve, reject };
}

// ===============================================================================
// DOM QUERY HELPERS
// ===============================================================================

/**
 * Query element by data-testid attribute
 */
export function getByTestId(container: HTMLElement, testId: string): HTMLElement | null {
	return container.querySelector(`[data-testid="${testId}"]`);
}

/**
 * Query all elements by data-testid attribute
 */
export function getAllByTestId(container: HTMLElement, testId: string): HTMLElement[] {
	return Array.from(container.querySelectorAll(`[data-testid="${testId}"]`));
}

/**
 * Query element by text content
 */
export function getByText(container: HTMLElement, text: string | RegExp): HTMLElement | null {
	const elements = Array.from(container.querySelectorAll('*'));
	return (
		(elements.find((el) => {
			const content = el.textContent?.trim() ?? '';
			return typeof text === 'string' ? content === text : text.test(content);
		}) as HTMLElement) ?? null
	);
}

/**
 * Query all elements by text content
 */
export function getAllByText(container: HTMLElement, text: string | RegExp): HTMLElement[] {
	const elements = Array.from(container.querySelectorAll('*'));
	return elements.filter((el) => {
		const content = el.textContent?.trim() ?? '';
		return typeof text === 'string' ? content === text : text.test(content);
	}) as HTMLElement[];
}

/**
 * Query element by role
 */
export function getByRole(container: HTMLElement, role: string): HTMLElement | null {
	return container.querySelector(`[role="${role}"]`);
}

/**
 * Query all elements by role
 */
export function getAllByRole(container: HTMLElement, role: string): HTMLElement[] {
	return Array.from(container.querySelectorAll(`[role="${role}"]`));
}

// ===============================================================================
// EVENT SIMULATION HELPERS
// ===============================================================================

/**
 * Simulate a click event on an element
 */
export function click(element: HTMLElement): void {
	element.dispatchEvent(new MouseEvent('click', { bubbles: true, cancelable: true }));
}

/**
 * Simulate typing into an input element
 */
export function type(element: HTMLInputElement | HTMLTextAreaElement, text: string): void {
	element.focus();
	element.value = text;
	element.dispatchEvent(new Event('input', { bubbles: true }));
	element.dispatchEvent(new Event('change', { bubbles: true }));
}

/**
 * Simulate selecting an option in a select element
 */
export function select(element: HTMLSelectElement, value: string): void {
	element.value = value;
	element.dispatchEvent(new Event('change', { bubbles: true }));
}

/**
 * Simulate form submission
 */
export function submitForm(form: HTMLFormElement): void {
	form.dispatchEvent(new Event('submit', { bubbles: true, cancelable: true }));
}

/**
 * Simulate keyboard event
 */
export function keyDown(element: HTMLElement, key: string, options: Partial<KeyboardEventInit> = {}): void {
	element.dispatchEvent(
		new KeyboardEvent('keydown', {
			key,
			bubbles: true,
			cancelable: true,
			...options
		})
	);
}

/**
 * Simulate focus event
 */
export function focus(element: HTMLElement): void {
	element.dispatchEvent(new FocusEvent('focus', { bubbles: true }));
}

/**
 * Simulate blur event
 */
export function blur(element: HTMLElement): void {
	element.dispatchEvent(new FocusEvent('blur', { bubbles: true }));
}

// ===============================================================================
// ASSERTION HELPERS
// ===============================================================================

/**
 * Assert that a value is defined (type narrowing)
 */
export function assertDefined<T>(
	value: T | undefined | null,
	message = 'Expected value to be defined'
): asserts value is T {
	if (value === undefined || value === null) {
		throw new Error(message);
	}
}

/**
 * Assert that an element exists in the DOM
 */
export function assertElementExists(
	element: HTMLElement | null,
	message = 'Expected element to exist'
): asserts element is HTMLElement {
	if (!element) {
		throw new Error(message);
	}
}

/**
 * Assert element has specific text content
 */
export function assertHasText(element: HTMLElement, expected: string | RegExp): void {
	const actual = element.textContent?.trim() ?? '';
	if (typeof expected === 'string') {
		if (!actual.includes(expected)) {
			throw new Error(`Expected element to contain "${expected}", but got "${actual}"`);
		}
	} else if (!expected.test(actual)) {
		throw new Error(`Expected element text "${actual}" to match ${expected}`);
	}
}

/**
 * Assert element has specific class
 */
export function assertHasClass(element: HTMLElement, className: string): void {
	if (!element.classList.contains(className)) {
		throw new Error(
			`Expected element to have class "${className}", but it has "${element.className}"`
		);
	}
}

/**
 * Assert element is visible
 */
export function assertIsVisible(element: HTMLElement): void {
	const style = window.getComputedStyle(element);
	if (
		style.display === 'none' ||
		style.visibility === 'hidden' ||
		style.opacity === '0'
	) {
		throw new Error('Expected element to be visible');
	}
}

// ===============================================================================
// MOCK DATA GENERATORS
// ===============================================================================

/**
 * Generate a random ticker symbol
 */
export function randomTicker(): string {
	const tickers = ['AAPL', 'NVDA', 'TSLA', 'MSFT', 'META', 'GOOGL', 'AMZN', 'AMD', 'NFLX', 'SPY'];
	return tickers[Math.floor(Math.random() * tickers.length)];
}

/**
 * Generate a random price in a range
 */
export function randomPrice(min = 100, max = 500): number {
	return Math.round((Math.random() * (max - min) + min) * 100) / 100;
}

/**
 * Generate a random percentage change
 */
export function randomPercentChange(min = -10, max = 15): number {
	return Math.round((Math.random() * (max - min) + min) * 100) / 100;
}

/**
 * Generate a random date within the last N days
 */
export function randomDateWithinDays(days: number): Date {
	const now = Date.now();
	const past = now - days * 24 * 60 * 60 * 1000;
	return new Date(past + Math.random() * (now - past));
}

// ===============================================================================
// CLEANUP UTILITIES
// ===============================================================================

/**
 * Clean up all test artifacts from the DOM
 */
export function cleanup(): void {
	document.body.innerHTML = '';
}

/**
 * Create an auto-cleanup function for use with afterEach
 */
export function setupCleanup(): () => void {
	const cleanupFns: Array<() => void> = [];

	const register = (fn: () => void) => {
		cleanupFns.push(fn);
	};

	const runCleanup = () => {
		cleanupFns.forEach((fn) => fn());
		cleanupFns.length = 0;
		cleanup();
	};

	return runCleanup;
}

// ===============================================================================
// TIMER UTILITIES
// ===============================================================================

/**
 * Setup fake timers and return cleanup function
 */
export function useFakeTimers() {
	vi.useFakeTimers();
	return () => {
		vi.useRealTimers();
	};
}

/**
 * Advance timers by specified milliseconds
 */
export async function advanceTimersBy(ms: number): Promise<void> {
	vi.advanceTimersByTime(ms);
	await flushPromises();
}

/**
 * Run all pending timers
 */
export async function runAllTimers(): Promise<void> {
	vi.runAllTimers();
	await flushPromises();
}

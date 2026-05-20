/**
 * logger — Unit Tests (R24-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * The logger module is the production-safe console wrapper imported in
 * many places (api/client.svelte.ts, auth flows, etc). The CLAUDE.md
 * hard rule it enforces:
 *
 *   "All console.* in production must be a no-op to prevent
 *    information leakage and improve performance."
 *
 * Why this test matters:
 *   - Once a logger.X() call slips through in prod, it leaks endpoint
 *     paths, request IDs, sometimes JWT preludes (see audit 2026-05-17).
 *   - The gate is `import.meta.env.DEV`. If a refactor flips that to
 *     `import.meta.env.PROD` or accidentally evaluates `isDev` at the
 *     wrong scope, every method becomes a live console emit in prod.
 *
 * What we test:
 *   1. In DEV mode (the default vitest environment): every method calls
 *      through to console.*.
 *   2. The structural contract: every documented method exists and is
 *      a function (defense against accidental shape regression).
 *   3. perfLogger.start/end behave correctly when paired and tolerate
 *      mismatched end-without-start (currently swallowed by try/catch).
 *
 * Why we don't test the "prod = noop" branch:
 *   - vitest evaluates `import.meta.env.DEV` at module import time.
 *     Toggling it after import doesn't reassign the bound noop refs.
 *   - The prod branch is the trivial side (literally `() => {}`) — a
 *     test of "noop does nothing" is tautological. The risk is the
 *     DEV branch wiring up to the right console method, which is what
 *     these tests cover.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { logger, perfLogger } from '../logger';

describe('logger (dev mode — vitest default)', () => {
	let consoleLogSpy: ReturnType<typeof vi.spyOn>;
	let consoleWarnSpy: ReturnType<typeof vi.spyOn>;
	let consoleErrorSpy: ReturnType<typeof vi.spyOn>;
	let consoleDebugSpy: ReturnType<typeof vi.spyOn>;
	let consoleInfoSpy: ReturnType<typeof vi.spyOn>;
	let consoleTraceSpy: ReturnType<typeof vi.spyOn>;

	beforeEach(() => {
		consoleLogSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
		consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
		consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
		consoleDebugSpy = vi.spyOn(console, 'debug').mockImplementation(() => {});
		consoleInfoSpy = vi.spyOn(console, 'info').mockImplementation(() => {});
		consoleTraceSpy = vi.spyOn(console, 'trace').mockImplementation(() => {});
	});

	afterEach(() => {
		consoleLogSpy.mockRestore();
		consoleWarnSpy.mockRestore();
		consoleErrorSpy.mockRestore();
		consoleDebugSpy.mockRestore();
		consoleInfoSpy.mockRestore();
		consoleTraceSpy.mockRestore();
	});

	it('logger.log forwards to console.log', () => {
		logger.log('hello', { id: 1 });
		expect(consoleLogSpy).toHaveBeenCalledWith('hello', { id: 1 });
	});

	it('logger.warn forwards to console.warn', () => {
		logger.warn('warning');
		expect(consoleWarnSpy).toHaveBeenCalledWith('warning');
	});

	it('logger.error forwards to console.error with the full args list', () => {
		const err = new Error('boom');
		logger.error('Critical', err);
		expect(consoleErrorSpy).toHaveBeenCalledWith('Critical', err);
	});

	it('logger.debug forwards to console.debug', () => {
		logger.debug('debug-line');
		expect(consoleDebugSpy).toHaveBeenCalledWith('debug-line');
	});

	it('logger.info forwards to console.info', () => {
		logger.info('info-line');
		expect(consoleInfoSpy).toHaveBeenCalledWith('info-line');
	});

	it('logger.trace forwards to console.trace', () => {
		logger.trace('trace-line');
		expect(consoleTraceSpy).toHaveBeenCalledWith('trace-line');
	});

	it('exposes the documented interface as functions', () => {
		// Defensive shape contract — protects against accidental
		// regression where a refactor drops a method or changes its kind.
		expect(typeof logger.log).toBe('function');
		expect(typeof logger.warn).toBe('function');
		expect(typeof logger.error).toBe('function');
		expect(typeof logger.debug).toBe('function');
		expect(typeof logger.info).toBe('function');
		expect(typeof logger.trace).toBe('function');
		expect(typeof logger.group).toBe('function');
		expect(typeof logger.groupEnd).toBe('function');
		expect(typeof logger.table).toBe('function');
		expect(typeof logger.time).toBe('function');
		expect(typeof logger.timeEnd).toBe('function');
	});

	it('logger.group / groupEnd forward to console', () => {
		const groupSpy = vi.spyOn(console, 'group').mockImplementation(() => {});
		const groupEndSpy = vi.spyOn(console, 'groupEnd').mockImplementation(() => {});
		try {
			logger.group('section');
			logger.groupEnd();
			expect(groupSpy).toHaveBeenCalledWith('section');
			expect(groupEndSpy).toHaveBeenCalled();
		} finally {
			groupSpy.mockRestore();
			groupEndSpy.mockRestore();
		}
	});

	it('logger.table forwards to console.table', () => {
		const tableSpy = vi.spyOn(console, 'table').mockImplementation(() => {});
		try {
			logger.table([{ a: 1 }]);
			expect(tableSpy).toHaveBeenCalledWith([{ a: 1 }]);
		} finally {
			tableSpy.mockRestore();
		}
	});

	it('logger.time / timeEnd forward to console (paired)', () => {
		const timeSpy = vi.spyOn(console, 'time').mockImplementation(() => {});
		const timeEndSpy = vi.spyOn(console, 'timeEnd').mockImplementation(() => {});
		try {
			logger.time('label');
			logger.timeEnd('label');
			expect(timeSpy).toHaveBeenCalledWith('label');
			expect(timeEndSpy).toHaveBeenCalledWith('label');
		} finally {
			timeSpy.mockRestore();
			timeEndSpy.mockRestore();
		}
	});
});

describe('perfLogger (dev mode)', () => {
	let markSpy: ReturnType<typeof vi.spyOn>;
	let measureSpy: ReturnType<typeof vi.spyOn>;
	let consoleLogSpy: ReturnType<typeof vi.spyOn>;

	beforeEach(() => {
		// Clear any previous marks/measures so the spies see a clean slate.
		performance.clearMarks();
		performance.clearMeasures();
		markSpy = vi.spyOn(performance, 'mark');
		measureSpy = vi.spyOn(performance, 'measure');
		consoleLogSpy = vi.spyOn(console, 'log').mockImplementation(() => {});
	});

	afterEach(() => {
		markSpy.mockRestore();
		measureSpy.mockRestore();
		consoleLogSpy.mockRestore();
		performance.clearMarks();
		performance.clearMeasures();
	});

	it('start() places a `<label>-start` performance mark', () => {
		perfLogger.start('render');
		expect(markSpy).toHaveBeenCalledWith('render-start');
	});

	it('end() places a `<label>-end` mark and measures between them', () => {
		perfLogger.start('render');
		perfLogger.end('render');
		expect(markSpy).toHaveBeenCalledWith('render-end');
		expect(measureSpy).toHaveBeenCalledWith('render', 'render-start', 'render-end');
		// Log line emitted iff a measure was created. We assert at least one
		// console.log call mentioning the label.
		const logged = consoleLogSpy.mock.calls.some(
			(call: unknown[]) => typeof call[0] === 'string' && (call[0] as string).includes('[PERF] render')
		);
		expect(logged).toBe(true);
	});

	it('end() without prior start() is swallowed (NEGATIVE — no throw)', () => {
		// Documented contract: try/catch silences the "mark not found" error.
		expect(() => perfLogger.end('orphan')).not.toThrow();
	});
});

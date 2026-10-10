/**
 * Monte Carlo off the main thread: `runMC` must hand the worker plain,
 * structured-cloneable data, and must fall back to the main thread whenever a
 * worker cannot produce a result.
 *
 * jsdom has no `Worker`, so `FakeWorker` stands in for one. It reproduces the
 * two properties that matter: messages cross the boundary through
 * `structuredClone` (which, like a real `postMessage`, throws `DataCloneError`
 * on a Svelte `$state` proxy), and the code that runs on the other side is the
 * real worker module.
 */

import { afterEach, beforeAll, describe, expect, it, vi } from 'vitest';
import { logger } from '#lib/utils/logger.js';
import { createCalculatorState } from '../calculator.svelte.js';
import type {
	MonteCarloWorkerRequest,
	MonteCarloWorkerResponse
} from '../../engine/monte-carlo.worker.js';

vi.mock('#lib/utils/logger.js', () => ({
	logger: { warn: vi.fn(), error: vi.fn(), info: vi.fn(), debug: vi.fn(), log: vi.fn() }
}));

/** The global scope the worker module installs its `onmessage` handler on. */
interface WorkerScope {
	onmessage: ((event: { data: MonteCarloWorkerRequest }) => void) | null;
	postMessage: (response: MonteCarloWorkerResponse) => void;
}

const scope: WorkerScope = { onmessage: null, postMessage: () => undefined };

beforeAll(async () => {
	vi.stubGlobal('self', scope);
	await import('../../engine/monte-carlo.worker.js');
	vi.unstubAllGlobals();
});

type Behaviour = 'respond' | 'hold' | 'load-error';

class FakeWorker {
	static instances: FakeWorker[] = [];
	static behaviour: Behaviour = 'respond';
	/** Requests the real worker handler has answered. */
	static answered = 0;

	onmessage: ((event: { data: MonteCarloWorkerResponse }) => void) | null = null;
	onerror: ((event: { message: string }) => void) | null = null;
	readonly options: WorkerOptions | undefined;
	terminated = false;
	private pending: MonteCarloWorkerRequest | undefined;

	constructor(_url: URL | string, options?: WorkerOptions) {
		this.options = options;
		FakeWorker.instances.push(this);
	}

	postMessage(data: MonteCarloWorkerRequest): void {
		const request = structuredClone(data);
		if (FakeWorker.behaviour === 'load-error') {
			queueMicrotask(() => this.onerror?.({ message: 'worker script blocked' }));
			return;
		}
		this.pending = request;
		if (FakeWorker.behaviour === 'respond') queueMicrotask(() => this.deliver());
	}

	/** Runs the real worker handler on the pending request and posts its answer back. */
	deliver(): void {
		const request = this.pending;
		if (!request || this.terminated) return;
		scope.postMessage = (response) => this.onmessage?.({ data: structuredClone(response) });
		// The handler answers through the global `self`, as it does in a real worker.
		const previousSelf: unknown = globalThis.self;
		vi.stubGlobal('self', scope);
		try {
			scope.onmessage?.({ data: request });
			FakeWorker.answered++;
		} finally {
			vi.stubGlobal('self', previousSelf);
		}
	}

	terminate(): void {
		this.terminated = true;
	}
}

afterEach(() => {
	vi.unstubAllGlobals();
	FakeWorker.instances = [];
	FakeWorker.behaviour = 'respond';
	FakeWorker.answered = 0;
	vi.mocked(logger.warn).mockClear();
});

describe('runMC', () => {
	it('simulates in a module worker with cloneable data', async () => {
		vi.stubGlobal('Worker', FakeWorker);
		const calc = createCalculatorState();
		// Assigning makes the config a deep $state proxy, which postMessage cannot clone.
		calc.monteCarloConfig = { numPaths: 200, numSteps: 20, seed: 7 };

		await calc.runMC();

		expect(FakeWorker.instances).toHaveLength(1);
		const [worker] = FakeWorker.instances;
		expect(worker?.options).toEqual({ type: 'module' });
		expect(worker?.terminated).toBe(true);
		// Answered by the worker itself, not by the main-thread fallback.
		expect(FakeWorker.answered).toBe(1);
		expect(logger.warn).not.toHaveBeenCalled();
		expect(calc.monteCarloResult?.paths.length).toBeGreaterThan(0);
		expect(calc.isMonteCarloRunning).toBe(false);
	});

	it('produces the same result as the main thread for a seeded run', async () => {
		vi.stubGlobal('Worker', FakeWorker);
		const viaWorker = createCalculatorState();
		viaWorker.monteCarloConfig = { numPaths: 100, numSteps: 10, seed: 42 };
		await viaWorker.runMC();
		expect(FakeWorker.answered).toBe(1);

		vi.stubGlobal('Worker', undefined);
		const mainThread = createCalculatorState();
		mainThread.monteCarloConfig = { numPaths: 100, numSteps: 10, seed: 42 };
		await mainThread.runMC();

		expect(viaWorker.monteCarloResult?.stats).toEqual(mainThread.monteCarloResult?.stats);
	});

	it('falls back to the main thread when workers are unavailable', async () => {
		vi.stubGlobal('Worker', undefined);
		const calc = createCalculatorState();
		calc.monteCarloConfig = { numPaths: 100, numSteps: 10 };

		await calc.runMC();

		expect(logger.warn).toHaveBeenCalledOnce();
		expect(calc.monteCarloResult?.paths.length).toBeGreaterThan(0);
		expect(calc.isMonteCarloRunning).toBe(false);
	});

	it('falls back to the main thread when the worker fails to load', async () => {
		vi.stubGlobal('Worker', FakeWorker);
		FakeWorker.behaviour = 'load-error';
		const calc = createCalculatorState();
		calc.monteCarloConfig = { numPaths: 100, numSteps: 10 };

		await calc.runMC();

		expect(FakeWorker.instances[0]?.terminated).toBe(true);
		expect(FakeWorker.answered).toBe(0);
		expect(logger.warn).toHaveBeenCalledOnce();
		expect(calc.monteCarloResult?.paths.length).toBeGreaterThan(0);
		expect(calc.isMonteCarloRunning).toBe(false);
	});

	it('keeps the newest run when an older one finishes last', async () => {
		vi.stubGlobal('Worker', FakeWorker);
		FakeWorker.behaviour = 'hold';
		const calc = createCalculatorState();

		calc.monteCarloConfig = { numPaths: 100, numSteps: 10, seed: 1 };
		const first = calc.runMC();
		calc.monteCarloConfig = { numPaths: 300, numSteps: 10, seed: 2 };
		const second = calc.runMC();
		const [older, newer] = FakeWorker.instances;

		newer?.deliver();
		await second;
		const newest = calc.monteCarloResult;
		expect(calc.isMonteCarloRunning).toBe(false);

		older?.deliver();
		await first;
		expect(FakeWorker.answered).toBe(2);
		expect(calc.monteCarloResult).toBe(newest);
	});
});

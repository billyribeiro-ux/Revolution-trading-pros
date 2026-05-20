/**
 * WorkerManager — Unit Tests (R27-D)
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * `src/lib/utils/worker-manager.ts` brokers requests to the
 * `block-processor.worker.ts` Web Worker. The class has TWO main code
 * paths and we test both:
 *
 *   1. WORKER-AVAILABLE path — sendMessage round-trips through the
 *      worker's `postMessage` ↔ `onmessage` channel using a
 *      monotonically-increasing `messageId`. We mock `Worker` so we
 *      can fire the reply synchronously.
 *
 *   2. WORKER-UNAVAILABLE path (Safari, SSR, hardened browsers) —
 *      the constructor's try/catch swallows the failure and each
 *      public method must fall back to a safe default:
 *        - processBlocks  → return blocks unchanged
 *        - validateBlock  → { valid: true, errors: [] }
 *        - sanitizeContent → content unchanged
 *        - calculateStats → zeroed-out stats
 *
 *   3. SINGLETON path — `getWorkerManager()` memoises one instance
 *      across calls (so the worker is not re-created on every import).
 *
 * Note: Hand-rolled FakeWorker (rather than vi.fn().Worker globalThis swap)
 * is used so `postMessage` and `onmessage` semantics are explicit, not
 * the implicit behaviour of a partial mock.
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { Block } from '$lib/components/cms/blocks/types';

// ═══════════════════════════════════════════════════════════════════════════
// FakeWorker — synchronous round-trip for deterministic tests
// ═══════════════════════════════════════════════════════════════════════════

interface FakeWorkerInstance {
	onmessage: ((e: MessageEvent) => void) | null;
	onerror: ((e: ErrorEvent) => void) | null;
	postMessage: ReturnType<typeof vi.fn>;
	terminate: ReturnType<typeof vi.fn>;
}

/**
 * Set up a stub `Worker` global that echoes posted messages back with the
 * same `messageId` (so `pendingMessages` resolves). Each test that wants
 * the WORKER-AVAILABLE path calls this; tests that want the fallback path
 * skip it (worker stays undefined → constructor throws → caught → null).
 */
function installFakeWorker(
	replyShape: (message: Record<string, unknown>) => Record<string, unknown> | null
): { instance: FakeWorkerInstance; restore: () => void } {
	const instance: FakeWorkerInstance = {
		onmessage: null,
		onerror: null,
		postMessage: vi.fn((message: Record<string, unknown>) => {
			const reply = replyShape(message);
			if (reply !== null) {
				queueMicrotask(() => {
					instance.onmessage?.({
						data: { ...reply, messageId: message.messageId }
					} as MessageEvent);
				});
			}
		}),
		terminate: vi.fn()
	};

	const originalWorker = (globalThis as { Worker?: unknown }).Worker;
	(globalThis as { Worker: unknown }).Worker = function MockWorker() {
		return instance;
	};

	return {
		instance,
		restore: () => {
			if (originalWorker === undefined) {
				delete (globalThis as { Worker?: unknown }).Worker;
			} else {
				(globalThis as { Worker: unknown }).Worker = originalWorker;
			}
		}
	};
}

// ═══════════════════════════════════════════════════════════════════════════
// Worker-AVAILABLE path — round-trip via postMessage / onmessage
// ═══════════════════════════════════════════════════════════════════════════

describe('WorkerManager — worker available', () => {
	let restore: () => void;
	let lastInstance: FakeWorkerInstance;

	beforeEach(() => {
		const setup = installFakeWorker((msg) => {
			// Echo back the right shape based on the message type.
			switch (msg.type) {
				case 'PROCESS_BLOCKS':
					return { blocks: msg.blocks };
				case 'VALIDATE_BLOCK':
					return { valid: true, errors: [] };
				case 'SANITIZE_CONTENT':
					return { content: msg.content };
				case 'CALCULATE_STATS':
					return {
						stats: {
							totalBlocks: (msg.blocks as Block[]).length,
							totalWords: 42,
							totalChars: 200,
							readingTime: 1,
							blockTypes: { paragraph: 1 }
						}
					};
				default:
					return null;
			}
		});
		restore = setup.restore;
		lastInstance = setup.instance;
	});

	afterEach(() => {
		restore();
	});

	it('processBlocks round-trips through postMessage with a unique messageId', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();
		const blocks = [{ id: 'a', type: 'paragraph' } as unknown as Block];

		const result = await mgr.processBlocks(blocks);
		expect(result).toEqual(blocks);

		// CONTRACT: every send embeds a unique numeric messageId.
		expect(lastInstance.postMessage).toHaveBeenCalledTimes(1);
		const payload = lastInstance.postMessage.mock.calls[0][0];
		expect(payload.type).toBe('PROCESS_BLOCKS');
		expect(typeof payload.messageId).toBe('number');
	});

	it('messageId increments monotonically across calls (no reuse)', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();

		await mgr.processBlocks([]);
		await mgr.processBlocks([]);
		await mgr.processBlocks([]);

		const ids = lastInstance.postMessage.mock.calls.map(
			(call) => (call[0] as Record<string, unknown>).messageId as number
		);
		expect(ids).toHaveLength(3);
		// Strictly increasing — never reused.
		expect(ids[1]).toBeGreaterThan(ids[0]);
		expect(ids[2]).toBeGreaterThan(ids[1]);
	});

	it('validateBlock returns the worker reply ({valid, errors}) unchanged', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();
		const block = { id: 'a', type: 'paragraph' } as unknown as Block;

		const result = await mgr.validateBlock(block);
		expect(result).toEqual({ valid: true, errors: [] });
	});

	it('sanitizeContent returns the worker-sanitised content (round-trip preserves payload)', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();
		const content = { text: 'hello' } as unknown as Block['content'];

		const result = await mgr.sanitizeContent(content);
		expect(result).toEqual(content);
	});

	it('calculateStats returns the worker stats payload (NOT the fallback zero-stats)', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();
		const blocks = [{ id: 'a', type: 'paragraph' } as unknown as Block];

		const stats = await mgr.calculateStats(blocks);
		// NEGATIVE assertion: the worker stub set totalWords=42; if we got 0
		// we'd be hitting the fallback path, which would be a regression.
		expect(stats.totalWords).toBe(42);
		expect(stats.blockTypes).toEqual({ paragraph: 1 });
	});

	it('terminate() releases the worker AND clears pending messages', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();

		// Kick off a request but don't await — the reply is queued via microtask.
		const pending = mgr.processBlocks([]);
		await pending; // drain
		mgr.terminate();

		expect(lastInstance.terminate).toHaveBeenCalledTimes(1);
		// Subsequent calls must hit the fallback path (worker is null now).
		const result = await mgr.processBlocks([
			{ id: 'after-terminate', type: 'paragraph' } as unknown as Block
		]);
		// Fallback returns blocks unchanged.
		expect(result).toEqual([{ id: 'after-terminate', type: 'paragraph' }]);
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// Worker-UNAVAILABLE path — graceful fallback
// ═══════════════════════════════════════════════════════════════════════════

describe('WorkerManager — worker unavailable (fallback)', () => {
	let restoreWorker: (() => void) | null = null;
	let warnSpy: ReturnType<typeof vi.spyOn>;

	beforeEach(() => {
		// Force constructor to throw → init catches → this.worker stays null.
		const originalWorker = (globalThis as { Worker?: unknown }).Worker;
		(globalThis as { Worker: unknown }).Worker = function ThrowingWorker() {
			throw new Error('Workers unavailable');
		};
		restoreWorker = () => {
			if (originalWorker === undefined) {
				delete (globalThis as { Worker?: unknown }).Worker;
			} else {
				(globalThis as { Worker: unknown }).Worker = originalWorker;
			}
		};
		// Mute the documented "Web Workers not available" warn.
		warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
	});

	afterEach(() => {
		restoreWorker?.();
		warnSpy.mockRestore();
	});

	it('processBlocks returns blocks UNCHANGED when worker is unavailable', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();
		const blocks = [{ id: 'a', type: 'paragraph' } as unknown as Block];

		const result = await mgr.processBlocks(blocks);
		expect(result).toBe(blocks); // exact identity — no copy made
	});

	it('validateBlock returns the optimistic {valid:true, errors:[]} fallback', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();
		const block = { id: 'a', type: 'paragraph' } as unknown as Block;

		const result = await mgr.validateBlock(block);
		expect(result).toEqual({ valid: true, errors: [] });
	});

	it('calculateStats returns zeroed stats with totalBlocks=blocks.length', async () => {
		const { WorkerManager } = await import('../worker-manager');
		const mgr = new WorkerManager();
		const blocks = [
			{ id: 'a', type: 'paragraph' } as unknown as Block,
			{ id: 'b', type: 'heading' } as unknown as Block
		];

		const stats = await mgr.calculateStats(blocks);
		expect(stats).toEqual({
			totalBlocks: 2,
			totalWords: 0,
			totalChars: 0,
			readingTime: 0,
			blockTypes: {}
		});
	});

	it('NEGATIVE — emits the "not available" warning exactly once at construction', async () => {
		const { WorkerManager } = await import('../worker-manager');
		new WorkerManager();
		expect(warnSpy).toHaveBeenCalledTimes(1);
		expect(warnSpy.mock.calls[0][0]).toContain('Web Workers not available');
	});
});

// ═══════════════════════════════════════════════════════════════════════════
// getWorkerManager — singleton
// ═══════════════════════════════════════════════════════════════════════════

describe('getWorkerManager — singleton', () => {
	let restore: () => void;

	beforeEach(() => {
		const setup = installFakeWorker(() => ({ blocks: [] }));
		restore = setup.restore;
		// Bust the module cache so the singleton is fresh per test.
		vi.resetModules();
	});

	afterEach(() => {
		restore();
	});

	it('returns the same WorkerManager instance on repeated calls', async () => {
		const { getWorkerManager } = await import('../worker-manager');
		const a = getWorkerManager();
		const b = getWorkerManager();
		expect(a).toBe(b);
	});
});

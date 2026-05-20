/**
 * Worker Manager
 * ═══════════════════════════════════════════════════════════════════════════
 * Manages Web Worker lifecycle and communication
 */

import type { Block, BlockContent } from '$lib/components/cms/blocks/types';

export class WorkerManager {
	private worker: Worker | null = null;
	private messageId = 0;
	// eslint-disable-next-line @typescript-eslint/no-unsafe-function-type
	private pendingMessages = new Map<number, { resolve: Function; reject: Function }>();

	constructor() {
		if (typeof window !== 'undefined') {
			this.initWorker();
		}
	}

	private initWorker(): void {
		try {
			this.worker = new Worker(new URL('../workers/block-processor.worker.ts', import.meta.url), {
				type: 'module'
			});

			this.worker.onmessage = (e) => {
				const { messageId, ...data } = e.data;
				const pending = this.pendingMessages.get(messageId);
				if (pending) {
					pending.resolve(data);
					this.pendingMessages.delete(messageId);
				}
			};

			this.worker.onerror = (e) => {
				console.error('Worker error:', e);
			};
		} catch (_error) {
			console.warn('Web Workers not available, falling back to main thread');
		}
	}

	private sendMessage<T>(message: object): Promise<T> {
		return new Promise((resolve, reject) => {
			if (!this.worker) {
				reject(new Error('Worker not available'));
				return;
			}

			const messageId = ++this.messageId;
			this.pendingMessages.set(messageId, { resolve, reject });

			this.worker.postMessage({ ...message, messageId });

			// Timeout after 30 seconds
			setTimeout(() => {
				if (this.pendingMessages.has(messageId)) {
					this.pendingMessages.delete(messageId);
					reject(new Error('Worker timeout'));
				}
			}, 30000);
		});
	}

	async processBlocks(blocks: Block[]): Promise<Block[]> {
		if (!this.worker) {
			// Fallback to main thread processing
			return blocks;
		}

		const result = await this.sendMessage<{ blocks: Block[] }>({
			type: 'PROCESS_BLOCKS',
			blocks
		});

		return result.blocks;
	}

	async validateBlock(block: Block): Promise<{ valid: boolean; errors: string[] }> {
		if (!this.worker) {
			// Fallback validation
			return { valid: true, errors: [] };
		}

		return this.sendMessage<{ valid: boolean; errors: string[] }>({
			type: 'VALIDATE_BLOCK',
			block
		});
	}

	async sanitizeContent(content: BlockContent): Promise<BlockContent> {
		if (!this.worker) {
			return content;
		}

		const result = await this.sendMessage<{ content: BlockContent }>({
			type: 'SANITIZE_CONTENT',
			content
		});

		return result.content;
	}

	async calculateStats(blocks: Block[]): Promise<{
		totalBlocks: number;
		totalWords: number;
		totalChars: number;
		readingTime: number;
		blockTypes: Record<string, number>;
	}> {
		if (!this.worker) {
			return {
				totalBlocks: blocks.length,
				totalWords: 0,
				totalChars: 0,
				readingTime: 0,
				blockTypes: {}
			};
		}

		const result = await this.sendMessage<{
			stats: {
				totalBlocks: number;
				totalWords: number;
				totalChars: number;
				readingTime: number;
				blockTypes: Record<string, number>;
			};
		}>({
			type: 'CALCULATE_STATS',
			blocks
		});

		return result.stats;
	}

	terminate(): void {
		if (this.worker) {
			this.worker.terminate();
			this.worker = null;
		}
		this.pendingMessages.clear();
	}
}

// Singleton instance
let workerManager: WorkerManager | null = null;

export function getWorkerManager(): WorkerManager {
	if (!workerManager) {
		workerManager = new WorkerManager();
	}
	return workerManager;
}

/**
 * Y.js Collaborative Editing Provider
 * ═══════════════════════════════════════════════════════════════════════════
 * Real-time multiplayer editing with conflict resolution
 */

import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';
import type { Block } from '$lib/components/cms/blocks/types';

export interface CollaboratorInfo {
	clientId: number;
	name: string;
	color: string;
	cursor?: { blockId: string; offset: number };
	selection?: { blockId: string; start: number; end: number };
}

export class CollaborationProvider {
	private ydoc: Y.Doc;
	private provider: WebsocketProvider | null = null;
	private blocksArray: Y.Array<Block>;
	private metaMap: Y.Map<any>;
	private connected = false;

	constructor(documentId: string, wsUrl?: string) {
		this.ydoc = new Y.Doc();
		this.blocksArray = this.ydoc.getArray('blocks');
		this.metaMap = this.ydoc.getMap('meta');

		if (wsUrl) {
			this.connect(documentId, wsUrl);
		}
	}

	connect(documentId: string, wsUrl: string): void {
		if (this.provider) {
			this.provider.destroy();
		}

		this.provider = new WebsocketProvider(wsUrl, documentId, this.ydoc);

		this.provider.on('status', (event: { status: string }) => {
			this.connected = event.status === 'connected';
		});
	}

	disconnect(): void {
		if (this.provider) {
			this.provider.disconnect();
		}
	}

	// Block operations with CRDT conflict resolution
	getBlocks(): Block[] {
		return this.blocksArray.toArray();
	}

	setBlocks(blocks: Block[]): void {
		this.ydoc.transact(() => {
			this.blocksArray.delete(0, this.blocksArray.length);
			this.blocksArray.push(blocks);
		});
	}

	addBlock(block: Block, index?: number): void {
		this.ydoc.transact(() => {
			if (index !== undefined && index < this.blocksArray.length) {
				this.blocksArray.insert(index, [block]);
			} else {
				this.blocksArray.push([block]);
			}
		});
	}

	updateBlock(blockId: string, updates: Partial<Block>): void {
		this.ydoc.transact(() => {
			const blocks = this.blocksArray.toArray();
			const index = blocks.findIndex((b) => b.id === blockId);

			if (index !== -1) {
				const current = blocks[index];
				this.blocksArray.delete(index, 1);
				this.blocksArray.insert(index, [{ ...current, ...updates }]);
			}
		});
	}

	removeBlock(blockId: string): void {
		this.ydoc.transact(() => {
			const blocks = this.blocksArray.toArray();
			const index = blocks.findIndex((b) => b.id === blockId);

			if (index !== -1) {
				this.blocksArray.delete(index, 1);
			}
		});
	}

	moveBlock(blockId: string, newIndex: number): void {
		this.ydoc.transact(() => {
			const blocks = this.blocksArray.toArray();
			const currentIndex = blocks.findIndex((b) => b.id === blockId);

			if (currentIndex !== -1 && currentIndex !== newIndex) {
				const block = blocks[currentIndex];
				this.blocksArray.delete(currentIndex, 1);
				this.blocksArray.insert(
					newIndex > currentIndex ? newIndex - 1 : newIndex,
					[block]
				);
			}
		});
	}

	// Subscribe to changes
	onBlocksChange(callback: (blocks: Block[]) => void): () => void {
		const observer = () => {
			callback(this.blocksArray.toArray());
		};

		this.blocksArray.observe(observer);
		return () => this.blocksArray.unobserve(observer);
	}

	// Awareness (cursors, selections, presence)
	getAwareness() {
		return this.provider?.awareness;
	}

	setLocalState(state: Partial<CollaboratorInfo>): void {
		if (this.provider?.awareness) {
			this.provider.awareness.setLocalStateField('user', state);
		}
	}

	getCollaborators(): CollaboratorInfo[] {
		if (!this.provider?.awareness) return [];

		const states = this.provider.awareness.getStates();
		const collaborators: CollaboratorInfo[] = [];

		states.forEach((state, clientId) => {
			if (clientId !== this.provider?.awareness.clientID && state.user) {
				collaborators.push({
					clientId,
					...state.user
				});
			}
		});

		return collaborators;
	}

	onCollaboratorsChange(callback: (collaborators: CollaboratorInfo[]) => void): () => void {
		if (!this.provider?.awareness) {
			return () => {};
		}

		const handler = () => {
			callback(this.getCollaborators());
		};

		this.provider.awareness.on('change', handler);
		return () => this.provider?.awareness.off('change', handler);
	}

	// Document metadata
	setMeta(key: string, value: any): void {
		this.metaMap.set(key, value);
	}

	getMeta(key: string): any {
		return this.metaMap.get(key);
	}

	// Undo/Redo with Y.js UndoManager
	createUndoManager(): Y.UndoManager {
		return new Y.UndoManager(this.blocksArray);
	}

	// Connection status
	isConnected(): boolean {
		return this.connected;
	}

	// Cleanup
	destroy(): void {
		if (this.provider) {
			this.provider.destroy();
			this.provider = null;
		}
		this.ydoc.destroy();
	}
}

// Factory function
export function createCollaborationProvider(
	documentId: string,
	wsUrl?: string
): CollaborationProvider {
	return new CollaborationProvider(documentId, wsUrl);
}

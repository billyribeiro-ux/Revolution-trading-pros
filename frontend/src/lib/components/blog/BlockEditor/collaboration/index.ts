/**
 * Collaboration Module - Real-time Collaborative Editing
 * ═══════════════════════════════════════════════════════════════════════════
 * Main exports and useCollaboration hook for Yjs-based collaborative editing
 *
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+ | Svelte 5 Runes Syntax
 *
 * Features:
 * - useCollaboration() hook for easy integration
 * - Real-time block synchronization via CRDT
 * - User presence and cursor tracking
 * - Offline persistence support
 * - Automatic reconnection
 */

import { browser } from '$app/environment';
import * as Y from 'yjs';
import {
	createYjsProvider,
	generateUserColor,
	generateClientId,
	type YjsProviderInstance,
	type ConnectionStatus
} from './yjs-provider';
import {
	createAwarenessManager,
	type AwarenessManagerInstance,
	type Collaborator,
	type LocalUser,
	type CursorPosition,
	type TextSelection
} from './awareness';
import type { Block } from '../types';

// ═══════════════════════════════════════════════════════════════════════════════
// RE-EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

export {
	createYjsProvider,
	generateUserColor,
	generateClientId,
	type YjsProviderInstance,
	type ConnectionStatus
} from './yjs-provider';

export {
	createAwarenessManager,
	getContrastColor,
	formatLastActive,
	isUserActive,
	type AwarenessManagerInstance,
	type Collaborator,
	type LocalUser,
	type UserPresence,
	type CursorPosition,
	type TextSelection
} from './awareness';

export { default as CollaboratorCursors } from './CollaboratorCursors.svelte';

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/** Collaboration hook options */
export interface UseCollaborationOptions {
	/** User ID for presence */
	userId: string;
	/** User display name */
	userName: string;
	/** User color (optional, will be generated if not provided) */
	userColor?: string;
	/** User avatar URL (optional) */
	userAvatar?: string;
	/** Whether to enable offline persistence */
	enablePersistence?: boolean;
	/** Initial blocks to sync if document is empty */
	initialBlocks?: Block[];
	/** Callback when connection status changes */
	onStatusChange?: (status: ConnectionStatus) => void;
	/** Callback when collaborators change */
	onCollaboratorsChange?: (collaborators: Collaborator[]) => void;
	/** Callback when blocks change remotely */
	onRemoteChange?: (blocks: Block[]) => void;
}

/** Collaboration hook return type */
export interface CollaborationState {
	/** Whether connected to the collaboration server */
	isConnected: boolean;
	/** Current connection status */
	connectionStatus: ConnectionStatus;
	/** List of current collaborators */
	collaborators: Collaborator[];
	/** Local user information */
	localUser: LocalUser;
	/** Yjs Array of blocks for direct manipulation */
	yBlocks: Y.Array<Block>;
	/** Whether the document is synced */
	isSynced: boolean;
	/** Any connection error */
	error: Error | null;
	/** Update a block at a specific index */
	updateBlock: (index: number, block: Block) => void;
	/** Insert a block at a specific index */
	insertBlock: (index: number, block: Block) => void;
	/** Delete a block at a specific index */
	deleteBlock: (index: number) => void;
	/** Move a block from one index to another */
	moveBlock: (fromIndex: number, toIndex: number) => void;
	/** Set the currently selected block */
	setSelection: (blockId: string | null) => void;
	/** Update cursor position */
	updateCursor: (position: CursorPosition | null) => void;
	/** Update text selection */
	updateTextSelection: (selection: TextSelection | null) => void;
	/** Mark user as typing */
	setTyping: (isTyping: boolean) => void;
	/** Get current blocks array */
	getBlocks: () => Block[];
	/** Replace all blocks */
	setBlocks: (blocks: Block[]) => void;
	/** Force reconnection */
	reconnect: () => void;
	/** Disconnect from collaboration */
	disconnect: () => void;
	/** Cleanup function */
	destroy: () => void;
}

// ═══════════════════════════════════════════════════════════════════════════════
// USE COLLABORATION HOOK
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates a reactive collaboration state for real-time editing.
 * Uses Svelte 5 Runes for reactivity.
 *
 * @param postId - Unique identifier for the collaborative document
 * @param options - Collaboration options
 * @returns CollaborationState with reactive properties and methods
 *
 * @example
 * ```svelte
 * <script lang="ts">
 *   import { useCollaboration } from './collaboration';
 *
 *   const collaboration = useCollaboration('post-123', {
 *     userId: 'user-1',
 *     userName: 'John Doe',
 *     onRemoteChange: (blocks) => {
 *       console.log('Remote change:', blocks);
 *     }
 *   });
 *
 *   // Access reactive state
 *   $effect(() => {
 *     console.log('Connected:', collaboration.isConnected);
 *     console.log('Collaborators:', collaboration.collaborators);
 *   });
 *
 *   // Update a block
 *   collaboration.updateBlock(0, updatedBlock);
 *
 *   // Cleanup on unmount
 *   onDestroy(() => collaboration.destroy());
 * </script>
 * ```
 */
export function useCollaboration(
	postId: string,
	options: UseCollaborationOptions
): CollaborationState {
	const {
		userId,
		userName,
		userColor = generateUserColor(),
		userAvatar,
		enablePersistence = true,
		initialBlocks = [],
		onStatusChange,
		onCollaboratorsChange,
		onRemoteChange
	} = options;

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE STATE (Svelte 5 Runes)
	// ═══════════════════════════════════════════════════════════════════════════

	let isConnected = $state(false);
	let connectionStatus = $state<ConnectionStatus>('disconnected');
	let collaborators = $state<Collaborator[]>([]);
	let isSynced = $state(false);
	let error = $state<Error | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// LOCAL USER
	// ═══════════════════════════════════════════════════════════════════════════

	const localUser: LocalUser = {
		id: userId,
		name: userName,
		color: userColor,
		avatar: userAvatar
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// PROVIDER INSTANCES
	// ═══════════════════════════════════════════════════════════════════════════

	let provider: YjsProviderInstance | null = null;
	let awarenessManager: AwarenessManagerInstance | null = null;
	let yBlocks: Y.Array<Block>;

	// Track if we're observing the yBlocks array
	let isObserving = false;

	// ═══════════════════════════════════════════════════════════════════════════
	// INITIALIZATION
	// ═══════════════════════════════════════════════════════════════════════════

	if (browser) {
		// Create Yjs provider
		provider = createYjsProvider({
			roomId: `blog-post-${postId}`,
			user: localUser,
			enablePersistence,
			onStatusChange: (status) => {
				connectionStatus = status;
				isConnected = status === 'connected' || status === 'syncing' || status === 'synced';
				error = null;

				if (onStatusChange) {
					onStatusChange(status);
				}
			},
			onSync: () => {
				isSynced = true;

				// Initialize with initial blocks if document is empty
				if (yBlocks.length === 0 && initialBlocks.length > 0) {
					yBlocks.doc?.transact(() => {
						for (const block of initialBlocks) {
							yBlocks.push([block]);
						}
					});
				}
			},
			onError: (err) => {
				error = err;
				console.error('[Collaboration] Error:', err);
			}
		});

		yBlocks = provider.yBlocks;

		// Set up block change observer
		const observeChanges = (event: Y.YArrayEvent<Block>) => {
			// Only notify for remote changes
			if (event.transaction.local) return;

			if (onRemoteChange) {
				onRemoteChange(yBlocks.toArray());
			}
		};

		yBlocks.observe(observeChanges);
		isObserving = true;

		// Create awareness manager
		const awareness = provider.getAwareness();
		if (awareness) {
			awarenessManager = createAwarenessManager({
				awareness,
				user: localUser,
				onCollaboratorsChange: (newCollaborators) => {
					collaborators = newCollaborators;

					if (onCollaboratorsChange) {
						onCollaboratorsChange(newCollaborators);
					}
				},
				onCollaboratorJoin: (collaborator) => {
					console.log('[Collaboration] Collaborator joined:', collaborator.name);
				},
				onCollaboratorLeave: (collaborator) => {
					console.log('[Collaboration] Collaborator left:', collaborator.name);
				}
			});
		}

		// Connect to the server
		provider.connect();
	} else {
		// SSR fallback - create a dummy yBlocks
		const doc = new Y.Doc();
		yBlocks = doc.getArray<Block>('blocks');
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// BLOCK OPERATIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Update a block at a specific index
	 */
	function updateBlock(index: number, block: Block): void {
		if (index < 0 || index >= yBlocks.length) {
			console.warn('[Collaboration] Invalid block index:', index);
			return;
		}

		yBlocks.doc?.transact(() => {
			yBlocks.delete(index, 1);
			yBlocks.insert(index, [block]);
		});

		awarenessManager?.setTyping(true);
	}

	/**
	 * Insert a block at a specific index
	 */
	function insertBlock(index: number, block: Block): void {
		const safeIndex = Math.max(0, Math.min(index, yBlocks.length));

		yBlocks.doc?.transact(() => {
			yBlocks.insert(safeIndex, [block]);
		});
	}

	/**
	 * Delete a block at a specific index
	 */
	function deleteBlock(index: number): void {
		if (index < 0 || index >= yBlocks.length) {
			console.warn('[Collaboration] Invalid block index:', index);
			return;
		}

		yBlocks.doc?.transact(() => {
			yBlocks.delete(index, 1);
		});
	}

	/**
	 * Move a block from one index to another
	 */
	function moveBlock(fromIndex: number, toIndex: number): void {
		if (fromIndex < 0 || fromIndex >= yBlocks.length) {
			console.warn('[Collaboration] Invalid from index:', fromIndex);
			return;
		}

		const block = yBlocks.get(fromIndex);
		if (!block) return;

		yBlocks.doc?.transact(() => {
			yBlocks.delete(fromIndex, 1);

			// Adjust target index if needed
			const adjustedToIndex = toIndex > fromIndex ? toIndex - 1 : toIndex;
			const safeToIndex = Math.max(0, Math.min(adjustedToIndex, yBlocks.length));

			yBlocks.insert(safeToIndex, [block]);
		});
	}

	/**
	 * Get current blocks array
	 */
	function getBlocks(): Block[] {
		return yBlocks.toArray();
	}

	/**
	 * Replace all blocks
	 */
	function setBlocks(blocks: Block[]): void {
		yBlocks.doc?.transact(() => {
			yBlocks.delete(0, yBlocks.length);
			for (const block of blocks) {
				yBlocks.push([block]);
			}
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PRESENCE OPERATIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Set the currently selected block
	 */
	function setSelection(blockId: string | null): void {
		awarenessManager?.updateSelection(blockId);
	}

	/**
	 * Update cursor position
	 */
	function updateCursor(position: CursorPosition | null): void {
		awarenessManager?.updateCursor(position);
	}

	/**
	 * Update text selection within a block
	 */
	function updateTextSelection(selection: TextSelection | null): void {
		awarenessManager?.updateTextSelection(selection);
	}

	/**
	 * Mark user as typing
	 */
	function setTyping(isTypingNow: boolean): void {
		awarenessManager?.setTyping(isTypingNow);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// CONNECTION OPERATIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Force reconnection
	 */
	function reconnect(): void {
		provider?.reconnect();
	}

	/**
	 * Disconnect from collaboration
	 */
	function disconnect(): void {
		provider?.disconnect();
	}

	/**
	 * Cleanup function
	 */
	function destroy(): void {
		console.log('[Collaboration] Destroying');

		// Destroy awareness manager
		if (awarenessManager) {
			awarenessManager.destroy();
			awarenessManager = null;
		}

		// Destroy provider
		if (provider) {
			provider.destroy();
			provider = null;
		}

		// Reset state
		isConnected = false;
		connectionStatus = 'disconnected';
		collaborators = [];
		isSynced = false;
		error = null;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN COLLABORATION STATE
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		get isConnected() { return isConnected; },
		get connectionStatus() { return connectionStatus; },
		get collaborators() { return collaborators; },
		get localUser() { return localUser; },
		get yBlocks() { return yBlocks; },
		get isSynced() { return isSynced; },
		get error() { return error; },
		updateBlock,
		insertBlock,
		deleteBlock,
		moveBlock,
		setSelection,
		updateCursor,
		updateTextSelection,
		setTyping,
		getBlocks,
		setBlocks,
		reconnect,
		disconnect,
		destroy
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Generate a unique block ID
 */
export function generateBlockId(): string {
	return `block-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
}

/**
 * Create a new block with defaults
 */
export function createBlock(
	type: Block['type'],
	content: Partial<Block['content']> = {},
	settings: Partial<Block['settings']> = {}
): Block {
	return {
		id: generateBlockId(),
		type,
		content: { ...content },
		settings: { ...settings },
		metadata: {
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
			version: 1
		}
	};
}

/**
 * Check if two blocks are equal (deep comparison)
 */
export function blocksEqual(a: Block, b: Block): boolean {
	return JSON.stringify(a) === JSON.stringify(b);
}

/**
 * Merge block updates preserving metadata
 */
export function mergeBlockUpdate(
	original: Block,
	updates: Partial<Block>
): Block {
	return {
		...original,
		...updates,
		metadata: {
			...original.metadata,
			...(updates.metadata || {}),
			updatedAt: new Date().toISOString(),
			version: (original.metadata?.version || 0) + 1
		}
	};
}

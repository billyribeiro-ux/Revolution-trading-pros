/**
 * Offline Support Store - Apple ICT9+ Design (Svelte 5 Runes)
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Manages offline state with:
 * - Network status detection
 * - Service worker registration
 * - Cache management
 * - Sync queue for offline actions
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 */

import { browser } from '$app/environment';

export interface PendingAction {
	id: string;
	type: 'create' | 'update' | 'delete';
	resource: string;
	data: Record<string, unknown>;
	timestamp: number;
	retryCount: number;
}

interface OfflineState {
	isOnline: boolean;
	lastOnline: number | null;
	serviceWorkerStatus: 'installing' | 'installed' | 'activating' | 'activated' | 'error' | null;
	cacheSize: number;
	pendingActions: PendingAction[];
	syncInProgress: boolean;
}

const STORAGE_KEY = 'rtp_pending_actions';

// Load pending actions from localStorage
function loadPendingActions(): PendingAction[] {
	if (!browser) return [];

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		return stored ? JSON.parse(stored) : [];
	} catch (e) {
		console.error('Failed to load pending actions:', e);
		return [];
	}
}

// Save pending actions to localStorage
function savePendingActions(actions: PendingAction[]) {
	if (!browser) return;

	try {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(actions));
	} catch (e) {
		console.error('Failed to save pending actions:', e);
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════════

const initialState: OfflineState = {
	isOnline: browser ? navigator.onLine : true,
	lastOnline: null,
	serviceWorkerStatus: null,
	cacheSize: 0,
	pendingActions: loadPendingActions(),
	syncInProgress: false
};

let offlineState = $state<OfflineState>({ ...initialState });

// ═══════════════════════════════════════════════════════════════════════════════
// Sync Functions
// ═══════════════════════════════════════════════════════════════════════════════

async function performSync(_action: PendingAction): Promise<void> {
	// Simulate API call - in real implementation, this would call the actual API
	return new Promise((resolve, reject) => {
		setTimeout(() => {
			// Simulate 90% success rate
			if (Math.random() > 0.1) {
				resolve();
			} else {
				reject(new Error('Sync failed'));
			}
		}, 500);
	});
}

async function syncPendingActions() {
	if (
		!offlineState.isOnline ||
		offlineState.syncInProgress ||
		offlineState.pendingActions.length === 0
	) {
		return;
	}

	offlineState = { ...offlineState, syncInProgress: true };

	const actionsToSync = [...offlineState.pendingActions];
	const failedActions: PendingAction[] = [];

	for (const action of actionsToSync) {
		try {
			// Attempt to sync the action
			await performSync(action);
		} catch (error) {
			console.error('Failed to sync action:', action.id, error);
			// Increment retry count and keep for later
			if (action.retryCount < 3) {
				failedActions.push({
					...action,
					retryCount: action.retryCount + 1
				});
			}
		}
	}

	offlineState = {
		...offlineState,
		pendingActions: failedActions,
		syncInProgress: false
	};

	savePendingActions(failedActions);
}

// Setup network listeners
if (browser) {
	window.addEventListener('online', () => {
		offlineState = {
			...offlineState,
			isOnline: true,
			lastOnline: Date.now()
		};
		// Trigger sync when back online
		syncPendingActions();
	});

	window.addEventListener('offline', () => {
		offlineState = {
			...offlineState,
			isOnline: false
		};
	});

	// Check for service worker support
	if ('serviceWorker' in navigator) {
		navigator.serviceWorker.ready
			.then(() => {
				offlineState = {
					...offlineState,
					serviceWorkerStatus: 'activated'
				};
			})
			.catch(() => {
				offlineState = {
					...offlineState,
					serviceWorkerStatus: 'error'
				};
			});
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Offline Store API
// ═══════════════════════════════════════════════════════════════════════════════

export const offlineStore = {
	get state() {
		return offlineState;
	},

	get isOnline() {
		return offlineState.isOnline;
	},

	get pendingActions() {
		return offlineState.pendingActions;
	},

	get syncInProgress() {
		return offlineState.syncInProgress;
	},

	// Queue an action for sync
	queueAction(type: PendingAction['type'], resource: string, data: Record<string, unknown>) {
		const action: PendingAction = {
			id: crypto.randomUUID(),
			type,
			resource,
			data,
			timestamp: Date.now(),
			retryCount: 0
		};

		const newActions = [...offlineState.pendingActions, action];
		savePendingActions(newActions);
		offlineState = {
			...offlineState,
			pendingActions: newActions
		};

		// Try to sync immediately if online
		if (offlineState.isOnline) {
			syncPendingActions();
		}
	},

	// Remove a pending action
	removeAction(id: string) {
		const newActions = offlineState.pendingActions.filter((a) => a.id !== id);
		savePendingActions(newActions);
		offlineState = {
			...offlineState,
			pendingActions: newActions
		};
	},

	// Clear all pending actions
	clearPendingActions() {
		savePendingActions([]);
		offlineState = {
			...offlineState,
			pendingActions: []
		};
	},

	// Manual sync trigger
	sync: syncPendingActions,

	// Update cache size
	updateCacheSize(size: number) {
		offlineState = {
			...offlineState,
			cacheSize: size
		};
	},

	// Clear cache
	async clearCache() {
		if (!browser || !('caches' in window)) return;

		try {
			const cacheNames = await caches.keys();
			await Promise.all(cacheNames.map((name) => caches.delete(name)));
			offlineState = {
				...offlineState,
				cacheSize: 0
			};
		} catch (error) {
			console.error('Failed to clear cache:', error);
		}
	}
};

// ═══════════════════════════════════════════════════════════════════════════════
// Getter Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════════

export function getIsOnline() {
	return offlineState.isOnline;
}
export function getHasPendingActions() {
	return offlineState.pendingActions.length > 0;
}
export function getPendingActionsCount() {
	return offlineState.pendingActions.length;
}
export function getIsSyncing() {
	return offlineState.syncInProgress;
}

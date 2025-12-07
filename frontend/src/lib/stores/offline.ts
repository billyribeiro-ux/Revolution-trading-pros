/**
 * Offline Support Store - Apple ICT9+ Design
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * Manages offline state with:
 * - Network status detection
 * - Service worker registration
 * - Cache management
 * - Sync queue for offline actions
 *
 * @version 1.0.0
 */

import { writable, derived, get } from 'svelte/store';
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

function createOfflineStore() {
	const initialState: OfflineState = {
		isOnline: browser ? navigator.onLine : true,
		lastOnline: null,
		serviceWorkerStatus: null,
		cacheSize: 0,
		pendingActions: loadPendingActions(),
		syncInProgress: false
	};

	const { subscribe, set, update } = writable<OfflineState>(initialState);

	// Setup network listeners
	if (browser) {
		window.addEventListener('online', () => {
			update(state => ({
				...state,
				isOnline: true,
				lastOnline: Date.now()
			}));
			// Trigger sync when back online
			syncPendingActions();
		});

		window.addEventListener('offline', () => {
			update(state => ({
				...state,
				isOnline: false
			}));
		});

		// Check for service worker support
		if ('serviceWorker' in navigator) {
			navigator.serviceWorker.ready.then(() => {
				update(state => ({
					...state,
					serviceWorkerStatus: 'activated'
				}));
			}).catch(() => {
				update(state => ({
					...state,
					serviceWorkerStatus: 'error'
				}));
			});
		}
	}

	async function syncPendingActions() {
		const state = get({ subscribe });
		if (!state.isOnline || state.syncInProgress || state.pendingActions.length === 0) {
			return;
		}

		update(s => ({ ...s, syncInProgress: true }));

		const actionsToSync = [...state.pendingActions];
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

		update(s => ({
			...s,
			pendingActions: failedActions,
			syncInProgress: false
		}));

		savePendingActions(failedActions);
	}

	async function performSync(action: PendingAction): Promise<void> {
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

	return {
		subscribe,

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

			update(state => {
				const newActions = [...state.pendingActions, action];
				savePendingActions(newActions);
				return {
					...state,
					pendingActions: newActions
				};
			});

			// Try to sync immediately if online
			const state = get({ subscribe });
			if (state.isOnline) {
				syncPendingActions();
			}
		},

		// Remove a pending action
		removeAction(id: string) {
			update(state => {
				const newActions = state.pendingActions.filter(a => a.id !== id);
				savePendingActions(newActions);
				return {
					...state,
					pendingActions: newActions
				};
			});
		},

		// Clear all pending actions
		clearPendingActions() {
			update(state => {
				savePendingActions([]);
				return {
					...state,
					pendingActions: []
				};
			});
		},

		// Manual sync trigger
		sync: syncPendingActions,

		// Update cache size
		updateCacheSize(size: number) {
			update(state => ({
				...state,
				cacheSize: size
			}));
		},

		// Clear cache
		async clearCache() {
			if (!browser || !('caches' in window)) return;

			try {
				const cacheNames = await caches.keys();
				await Promise.all(
					cacheNames.map(name => caches.delete(name))
				);
				update(state => ({
					...state,
					cacheSize: 0
				}));
			} catch (error) {
				console.error('Failed to clear cache:', error);
			}
		}
	};
}

export const offlineStore = createOfflineStore();

// Derived stores
export const isOnline = derived(offlineStore, $store => $store.isOnline);
export const hasPendingActions = derived(offlineStore, $store => $store.pendingActions.length > 0);
export const pendingActionsCount = derived(offlineStore, $store => $store.pendingActions.length);
export const isSyncing = derived(offlineStore, $store => $store.syncInProgress);

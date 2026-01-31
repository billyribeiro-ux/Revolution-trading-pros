/**
 * Awareness - User Presence and Cursor Tracking
 * ═══════════════════════════════════════════════════════════════════════════
 * Real-time user presence tracking for collaborative editing
 *
 * @version 1.0.0 - January 2026
 * @standards Apple Principal Engineer ICT Level 7+ | Svelte 5 Runes Syntax
 *
 * Features:
 * - Track user presence (name, color, cursor position)
 * - Show who's currently editing
 * - Idle detection (mark away after 2 minutes)
 * - Clean up stale users
 * - Selection tracking per block
 */

import { browser } from '$app/environment';
import type { Awareness as YAwareness } from 'y-protocols/awareness';

// ═══════════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/** Time in milliseconds before a user is considered idle */
const IDLE_TIMEOUT = 2 * 60 * 1000; // 2 minutes

/** Time in milliseconds before a stale user is cleaned up */
const STALE_TIMEOUT = 5 * 60 * 1000; // 5 minutes

/** Activity check interval */
const ACTIVITY_CHECK_INTERVAL = 30 * 1000; // 30 seconds

/** Events that indicate user activity */
const ACTIVITY_EVENTS = ['mousedown', 'mousemove', 'keydown', 'scroll', 'touchstart'];

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/** User presence state */
export interface UserPresence {
	/** Unique user identifier */
	id: string;
	/** Display name */
	name: string;
	/** User's assigned color */
	color: string;
	/** Optional avatar URL */
	avatar?: string;
	/** Current cursor position (x, y relative to editor) */
	cursor: CursorPosition | null;
	/** Currently selected block ID */
	selectedBlockId: string | null;
	/** Text selection within a block */
	selection: TextSelection | null;
	/** Last activity timestamp */
	lastActive: number;
	/** Whether user is idle */
	isIdle: boolean;
	/** Whether user is currently typing */
	isTyping: boolean;
}

/** Cursor position data */
export interface CursorPosition {
	/** X position relative to editor container */
	x: number;
	/** Y position relative to editor container */
	y: number;
	/** Block ID the cursor is over */
	blockId: string | null;
}

/** Text selection within a block */
export interface TextSelection {
	/** Block ID containing the selection */
	blockId: string;
	/** Selection anchor offset */
	anchorOffset: number;
	/** Selection focus offset */
	focusOffset: number;
	/** Selected text content */
	selectedText: string;
}

/** Collaborator data (remote user) */
export interface Collaborator extends UserPresence {
	/** Yjs client ID */
	clientId: number;
}

/** Local user data */
export interface LocalUser {
	/** Unique user identifier */
	id: string;
	/** Display name */
	name: string;
	/** User's assigned color */
	color: string;
	/** Optional avatar URL */
	avatar?: string;
}

/** Awareness state changes */
export interface AwarenessChange {
	/** Client IDs that were added */
	added: number[];
	/** Client IDs that were updated */
	updated: number[];
	/** Client IDs that were removed */
	removed: number[];
}

/** Awareness manager options */
export interface AwarenessManagerOptions {
	/** Yjs awareness instance */
	awareness: YAwareness;
	/** Local user information */
	user: LocalUser;
	/** Callback when collaborators change */
	onCollaboratorsChange?: (collaborators: Collaborator[]) => void;
	/** Callback when a collaborator joins */
	onCollaboratorJoin?: (collaborator: Collaborator) => void;
	/** Callback when a collaborator leaves */
	onCollaboratorLeave?: (collaborator: Collaborator) => void;
}

/** Awareness manager instance */
export interface AwarenessManagerInstance {
	/** Current collaborators (excluding self) */
	collaborators: Collaborator[];
	/** Local user presence */
	localPresence: UserPresence;
	/** Update cursor position */
	updateCursor: (position: CursorPosition | null) => void;
	/** Update selected block */
	updateSelection: (blockId: string | null) => void;
	/** Update text selection */
	updateTextSelection: (selection: TextSelection | null) => void;
	/** Mark as typing */
	setTyping: (isTyping: boolean) => void;
	/** Force activity update */
	markActive: () => void;
	/** Cleanup and destroy */
	destroy: () => void;
}

// ═══════════════════════════════════════════════════════════════════════════════
// AWARENESS MANAGER FACTORY
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Creates an awareness manager for tracking user presence in collaborative editing.
 *
 * @param options - Manager configuration options
 * @returns AwarenessManagerInstance with presence tracking methods
 *
 * @example
 * ```typescript
 * const awareness = createAwarenessManager({
 *   awareness: provider.getAwareness(),
 *   user: { id: 'user-1', name: 'John', color: '#ff0000' },
 *   onCollaboratorsChange: (collaborators) => {
 *     console.log('Collaborators:', collaborators);
 *   }
 * });
 *
 * // Update cursor position
 * awareness.updateCursor({ x: 100, y: 200, blockId: 'block-1' });
 *
 * // Cleanup on unmount
 * awareness.destroy();
 * ```
 */
export function createAwarenessManager(
	options: AwarenessManagerOptions
): AwarenessManagerInstance {
	const {
		awareness,
		user,
		onCollaboratorsChange,
		onCollaboratorJoin,
		onCollaboratorLeave
	} = options;

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	/** Current collaborators map by client ID */
	let collaboratorsMap = new Map<number, Collaborator>();

	/** Local user presence state */
	let localPresence: UserPresence = {
		id: user.id,
		name: user.name,
		color: user.color,
		avatar: user.avatar,
		cursor: null,
		selectedBlockId: null,
		selection: null,
		lastActive: Date.now(),
		isIdle: false,
		isTyping: false
	};

	/** Activity tracking timer */
	let activityTimer: ReturnType<typeof setInterval> | null = null;

	/** Idle check timer */
	let idleCheckTimer: ReturnType<typeof setInterval> | null = null;

	/** Last activity timestamp */
	let lastActivityTime = Date.now();

	/** Typing timeout */
	let typingTimeout: ReturnType<typeof setTimeout> | null = null;

	// ═══════════════════════════════════════════════════════════════════════════
	// HELPER FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Get collaborators array (excluding self)
	 */
	function getCollaborators(): Collaborator[] {
		return Array.from(collaboratorsMap.values());
	}

	/**
	 * Update local awareness state
	 */
	function updateLocalState(updates: Partial<UserPresence>): void {
		localPresence = { ...localPresence, ...updates };

		awareness.setLocalStateField('user', {
			...localPresence,
			lastActive: Date.now()
		});
	}

	/**
	 * Handle awareness state changes
	 */
	function handleAwarenessChange(change: AwarenessChange): void {
		const localClientId = awareness.clientID;
		const previousCollaborators = new Map(collaboratorsMap);

		// Process all states
		const states = awareness.getStates();

		// Clear and rebuild collaborators map
		collaboratorsMap.clear();

		states.forEach((state: Record<string, any>, clientId: number) => {
			// Skip local client
			if (clientId === localClientId) return;

			const userState = state.user as UserPresence | undefined;
			if (!userState) return;

			// Check if user is stale
			const now = Date.now();
			const lastActive = userState.lastActive || 0;
			const isStale = now - lastActive > STALE_TIMEOUT;

			if (isStale) return;

			// Check if user is idle
			const isIdle = now - lastActive > IDLE_TIMEOUT;

			const collaborator: Collaborator = {
				...userState,
				clientId,
				isIdle
			};

			collaboratorsMap.set(clientId, collaborator);
		});

		// Detect joins and leaves
		const currentIds = new Set(collaboratorsMap.keys());
		const previousIds = new Set(previousCollaborators.keys());

		// Handle new collaborators
		for (const clientId of currentIds) {
			if (!previousIds.has(clientId)) {
				const collaborator = collaboratorsMap.get(clientId);
				if (collaborator && onCollaboratorJoin) {
					onCollaboratorJoin(collaborator);
				}
			}
		}

		// Handle departed collaborators
		for (const clientId of previousIds) {
			if (!currentIds.has(clientId)) {
				const collaborator = previousCollaborators.get(clientId);
				if (collaborator && onCollaboratorLeave) {
					onCollaboratorLeave(collaborator);
				}
			}
		}

		// Notify of changes
		if (onCollaboratorsChange) {
			onCollaboratorsChange(getCollaborators());
		}
	}

	/**
	 * Handle user activity
	 */
	function handleActivity(): void {
		lastActivityTime = Date.now();

		if (localPresence.isIdle) {
			updateLocalState({ isIdle: false });
		}
	}

	/**
	 * Check for idle state
	 */
	function checkIdleState(): void {
		const now = Date.now();
		const timeSinceActivity = now - lastActivityTime;

		if (timeSinceActivity > IDLE_TIMEOUT && !localPresence.isIdle) {
			updateLocalState({ isIdle: true });
		}

		// Also clean up stale collaborators
		handleAwarenessChange({ added: [], updated: [], removed: [] });
	}

	/**
	 * Set up activity tracking
	 */
	function setupActivityTracking(): void {
		if (!browser) return;

		// Listen for activity events
		for (const event of ACTIVITY_EVENTS) {
			document.addEventListener(event, handleActivity, { passive: true });
		}

		// Start idle check interval
		idleCheckTimer = setInterval(checkIdleState, ACTIVITY_CHECK_INTERVAL);
	}

	/**
	 * Clean up activity tracking
	 */
	function cleanupActivityTracking(): void {
		if (!browser) return;

		// Remove activity event listeners
		for (const event of ACTIVITY_EVENTS) {
			document.removeEventListener(event, handleActivity);
		}

		// Clear timers
		if (activityTimer) {
			clearInterval(activityTimer);
			activityTimer = null;
		}

		if (idleCheckTimer) {
			clearInterval(idleCheckTimer);
			idleCheckTimer = null;
		}

		if (typingTimeout) {
			clearTimeout(typingTimeout);
			typingTimeout = null;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// INITIALIZATION
	// ═══════════════════════════════════════════════════════════════════════════

	// Set initial local state
	awareness.setLocalStateField('user', localPresence);

	// Subscribe to awareness changes
	awareness.on('change', handleAwarenessChange);

	// Initial collaborators sync
	handleAwarenessChange({ added: [], updated: [], removed: [] });

	// Set up activity tracking
	setupActivityTracking();

	// ═══════════════════════════════════════════════════════════════════════════
	// PUBLIC METHODS
	// ═══════════════════════════════════════════════════════════════════════════

	/**
	 * Update cursor position
	 */
	function updateCursor(position: CursorPosition | null): void {
		updateLocalState({ cursor: position });
		handleActivity();
	}

	/**
	 * Update selected block
	 */
	function updateSelection(blockId: string | null): void {
		updateLocalState({ selectedBlockId: blockId });
		handleActivity();
	}

	/**
	 * Update text selection within a block
	 */
	function updateTextSelection(selection: TextSelection | null): void {
		updateLocalState({ selection });
		handleActivity();
	}

	/**
	 * Set typing state with auto-reset
	 */
	function setTyping(isTyping: boolean): void {
		// Clear existing timeout
		if (typingTimeout) {
			clearTimeout(typingTimeout);
			typingTimeout = null;
		}

		updateLocalState({ isTyping });
		handleActivity();

		// Auto-reset typing after 3 seconds of no input
		if (isTyping) {
			typingTimeout = setTimeout(() => {
				updateLocalState({ isTyping: false });
			}, 3000);
		}
	}

	/**
	 * Force activity update
	 */
	function markActive(): void {
		handleActivity();
		updateLocalState({ lastActive: Date.now() });
	}

	/**
	 * Cleanup and destroy the manager
	 */
	function destroy(): void {
		// Remove awareness listener
		awareness.off('change', handleAwarenessChange);

		// Clear local state from awareness
		awareness.setLocalState(null);

		// Clean up activity tracking
		cleanupActivityTracking();

		// Clear collaborators
		collaboratorsMap.clear();

		console.log('[Awareness] Destroyed');
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// RETURN MANAGER INSTANCE
	// ═══════════════════════════════════════════════════════════════════════════

	return {
		get collaborators() { return getCollaborators(); },
		get localPresence() { return localPresence; },
		updateCursor,
		updateSelection,
		updateTextSelection,
		setTyping,
		markActive,
		destroy
	};
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Generate a random user color for presence
 */
export function generateUserColor(): string {
	const colors = [
		'#f87171', // red
		'#fb923c', // orange
		'#fbbf24', // amber
		'#a3e635', // lime
		'#34d399', // emerald
		'#22d3ee', // cyan
		'#60a5fa', // blue
		'#a78bfa', // violet
		'#f472b6', // pink
		'#e879f9'  // fuchsia
	];
	return colors[Math.floor(Math.random() * colors.length)];
}

/**
 * Get contrasting text color (black or white) for a background color
 */
export function getContrastColor(hexColor: string): string {
	// Remove # if present
	const hex = hexColor.replace('#', '');

	// Parse RGB values
	const r = parseInt(hex.substring(0, 2), 16);
	const g = parseInt(hex.substring(2, 4), 16);
	const b = parseInt(hex.substring(4, 6), 16);

	// Calculate relative luminance
	const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

	return luminance > 0.5 ? '#000000' : '#ffffff';
}

/**
 * Format time since last active
 */
export function formatLastActive(timestamp: number): string {
	const now = Date.now();
	const diff = now - timestamp;

	if (diff < 60000) {
		return 'Just now';
	} else if (diff < 3600000) {
		const minutes = Math.floor(diff / 60000);
		return `${minutes}m ago`;
	} else if (diff < 86400000) {
		const hours = Math.floor(diff / 3600000);
		return `${hours}h ago`;
	} else {
		const days = Math.floor(diff / 86400000);
		return `${days}d ago`;
	}
}

/**
 * Check if a user is considered active (not idle)
 */
export function isUserActive(lastActive: number): boolean {
	return Date.now() - lastActive < IDLE_TIMEOUT;
}

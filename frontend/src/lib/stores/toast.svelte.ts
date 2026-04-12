/**
 * Toast Notification Store - Svelte 5 Runes
 * =========================
 * Enterprise-grade notification system with:
 * - Maximum queue limit to prevent memory leaks
 * - Timeout cleanup on early dismissal
 * - Type-safe toast types
 * - Loading toast support
 *
 * @version 3.1.0 - Svelte 5.55+ (April 2026 sweep)
 *  - Replace built-in Map with `SvelteMap` for the cleanup tracker
 *    (recommended pattern for reactive `.svelte.ts` modules).
 *  - Provide a back-compat `addToast(...)` shim so the old
 *    `$lib/utils/toast` writable-store callers can migrate by
 *    swapping a single import path.
 */

import { SvelteMap } from 'svelte/reactivity';

// =============================================================================
// Types
// =============================================================================

export type ToastType = 'success' | 'error' | 'warning' | 'info' | 'loading';

export interface Toast {
	id: string;
	type: ToastType;
	message: string;
	duration: number;
	dismissible: boolean;
}

interface ToastState {
	toasts: Toast[];
}

// =============================================================================
// Configuration
// =============================================================================

const MAX_TOASTS = 5; // Maximum number of toasts to display
const DEFAULT_DURATION = 4000;

// =============================================================================
// Svelte 5 Reactive State
// =============================================================================

let storeState = $state<ToastState>({ toasts: [] });

// Derived value for toasts array
const toastsValue = $derived(storeState.toasts);

// =============================================================================
// Internal State
// =============================================================================

let idCounter = 0;
// Track timeout IDs for cleanup. Uses `SvelteMap` (vs. the built-in `Map`)
// per Svelte 5 best practice for any mutable Map living in a `.svelte.ts`
// module — keeps reactive consumers consistent if any reads are added later.
const timeoutIds = new SvelteMap<string, ReturnType<typeof setTimeout>>();

function generateId(): string {
	return `toast-${Date.now()}-${++idCounter}`;
}

function pushToast(toast: Omit<Toast, 'id'>): string {
	const id = generateId();
	const newToast: Toast = { ...toast, id };

	let newToasts = [...storeState.toasts, newToast];

	// Enforce maximum queue limit - remove oldest toasts
	if (newToasts.length > MAX_TOASTS) {
		const toRemove = newToasts.slice(0, newToasts.length - MAX_TOASTS);
		// Cleanup timeouts for removed toasts
		toRemove.forEach((t) => {
			const timeoutId = timeoutIds.get(t.id);
			if (timeoutId) {
				clearTimeout(timeoutId);
				timeoutIds.delete(t.id);
			}
		});
		newToasts = newToasts.slice(-MAX_TOASTS);
	}

	storeState = { toasts: newToasts };

	// Auto dismiss after duration (if duration > 0)
	if (toast.duration > 0) {
		const timeoutId = setTimeout(() => {
			dismiss(id);
		}, toast.duration);
		timeoutIds.set(id, timeoutId);
	}

	return id;
}

function dismiss(id: string): void {
	// Clear timeout if exists
	const timeoutId = timeoutIds.get(id);
	if (timeoutId) {
		clearTimeout(timeoutId);
		timeoutIds.delete(id);
	}

	storeState = {
		toasts: storeState.toasts.filter((t) => t.id !== id)
	};
}

function dismissAll(): void {
	// Clear all timeouts
	timeoutIds.forEach((timeoutId) => clearTimeout(timeoutId));
	timeoutIds.clear();

	storeState = { toasts: [] };
}

function updateToast(id: string, updates: Partial<Omit<Toast, 'id'>>): void {
	storeState = {
		toasts: storeState.toasts.map((t) => (t.id === id ? { ...t, ...updates } : t))
	};

	// If duration is updated and > 0, set new timeout
	if (updates.duration && updates.duration > 0) {
		// Clear existing timeout
		const existingTimeout = timeoutIds.get(id);
		if (existingTimeout) {
			clearTimeout(existingTimeout);
		}

		const timeoutId = setTimeout(() => {
			dismiss(id);
		}, updates.duration);
		timeoutIds.set(id, timeoutId);
	}
}

// =============================================================================
// Exported Store (Svelte 5 Pattern)
// =============================================================================

export const toastStore = {
	// Getters
	get state() {
		return storeState;
	},
	get toasts() {
		return toastsValue;
	},

	/**
	 * Show a success toast
	 */
	success(message: string, duration = DEFAULT_DURATION): string {
		return pushToast({
			type: 'success',
			message,
			duration,
			dismissible: true
		});
	},

	/**
	 * Show an error toast (longer duration by default)
	 */
	error(message: string, duration = 6000): string {
		return pushToast({
			type: 'error',
			message,
			duration,
			dismissible: true
		});
	},

	/**
	 * Show a warning toast
	 */
	warning(message: string, duration = 5000): string {
		return pushToast({
			type: 'warning',
			message,
			duration,
			dismissible: true
		});
	},

	/**
	 * Show an info toast
	 */
	info(message: string, duration = DEFAULT_DURATION): string {
		return pushToast({
			type: 'info',
			message,
			duration,
			dismissible: true
		});
	},

	/**
	 * Show a loading toast (no auto-dismiss)
	 * Returns ID so you can update it later
	 */
	loading(message: string): string {
		return pushToast({
			type: 'loading',
			message,
			duration: 0, // No auto-dismiss
			dismissible: false
		});
	},

	/**
	 * Show a custom toast
	 */
	show(options: Partial<Omit<Toast, 'id'>> & { message: string }): string {
		return pushToast({
			type: options.type ?? 'info',
			message: options.message,
			duration: options.duration ?? DEFAULT_DURATION,
			dismissible: options.dismissible ?? true
		});
	},

	/**
	 * Update an existing toast
	 */
	update: updateToast,

	/**
	 * Promise-based toast for async operations
	 * Shows loading, then success/error based on promise result
	 */
	async promise<T>(
		promise: Promise<T>,
		options: {
			loading: string;
			success: string | ((data: T) => string);
			error: string | ((err: unknown) => string);
		}
	): Promise<T> {
		const id = pushToast({
			type: 'loading',
			message: options.loading,
			duration: 0,
			dismissible: false
		});

		try {
			const result = await promise;
			const successMessage =
				typeof options.success === 'function' ? options.success(result) : options.success;

			updateToast(id, {
				type: 'success',
				message: successMessage,
				duration: DEFAULT_DURATION,
				dismissible: true
			});

			return result;
		} catch (err) {
			const errorMessage = typeof options.error === 'function' ? options.error(err) : options.error;

			updateToast(id, {
				type: 'error',
				message: errorMessage,
				duration: 6000,
				dismissible: true
			});

			throw err;
		}
	},

	/**
	 * Dismiss a specific toast
	 */
	dismiss,

	/**
	 * Dismiss all toasts
	 */
	dismissAll
};

// =============================================================================
// Legacy Exports for Backward Compatibility
// =============================================================================

// Derived store for easy access to toasts array
export const toasts = {
	get value() {
		return toastsValue;
	}
};

// Re-export common functions for convenience
export const { success, error, warning, info, loading, show, promise } = toastStore;

// Alias for component usage
export const removeToast = toastStore.dismiss;

/**
 * Legacy `addToast({ type, message, duration? })` shim.
 *
 * Pre-runes call sites import this from the now-removed `$lib/utils/toast`
 * module. The function is preserved here so the migration is purely a
 * single-line import-path swap, with no behavioural change at the call site.
 *
 * Prefer the typed shortcuts (`toastStore.success`, `toastStore.error`, ...)
 * for new code.
 */
export function addToast(toast: {
	type?: ToastType;
	message: string;
	duration?: number;
	dismissible?: boolean;
}): string {
	return toastStore.show({
		type: toast.type ?? 'info',
		message: toast.message,
		duration: toast.duration,
		dismissible: toast.dismissible
	});
}

export default toastStore;

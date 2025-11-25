/**
 * Toast Notification Store
 * Enterprise-grade notification system
 */

import { writable, derived } from 'svelte/store';

export interface Toast {
	id: string;
	type: 'success' | 'error' | 'warning' | 'info';
	message: string;
	duration: number;
	dismissible: boolean;
}

interface ToastState {
	toasts: Toast[];
}

function createToastStore() {
	const { subscribe, update } = writable<ToastState>({
		toasts: []
	});

	let idCounter = 0;

	function generateId(): string {
		return `toast-${Date.now()}-${++idCounter}`;
	}

	function addToast(toast: Omit<Toast, 'id'>): string {
		const id = generateId();
		const newToast: Toast = { ...toast, id };

		update((state) => ({
			toasts: [...state.toasts, newToast]
		}));

		// Auto dismiss after duration
		if (toast.duration > 0) {
			setTimeout(() => {
				dismiss(id);
			}, toast.duration);
		}

		return id;
	}

	function dismiss(id: string) {
		update((state) => ({
			toasts: state.toasts.filter((t) => t.id !== id)
		}));
	}

	function dismissAll() {
		update(() => ({ toasts: [] }));
	}

	return {
		subscribe,

		/**
		 * Show a success toast
		 */
		success(message: string, duration = 4000) {
			return addToast({
				type: 'success',
				message,
				duration,
				dismissible: true
			});
		},

		/**
		 * Show an error toast
		 */
		error(message: string, duration = 6000) {
			return addToast({
				type: 'error',
				message,
				duration,
				dismissible: true
			});
		},

		/**
		 * Show a warning toast
		 */
		warning(message: string, duration = 5000) {
			return addToast({
				type: 'warning',
				message,
				duration,
				dismissible: true
			});
		},

		/**
		 * Show an info toast
		 */
		info(message: string, duration = 4000) {
			return addToast({
				type: 'info',
				message,
				duration,
				dismissible: true
			});
		},

		/**
		 * Show a custom toast
		 */
		show(options: Partial<Omit<Toast, 'id'>> & { message: string }) {
			return addToast({
				type: options.type || 'info',
				message: options.message,
				duration: options.duration ?? 4000,
				dismissible: options.dismissible ?? true
			});
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
}

export const toastStore = createToastStore();

// Derived store for easy access to toasts array
export const toasts = derived(toastStore, ($store) => $store.toasts);

export default toastStore;

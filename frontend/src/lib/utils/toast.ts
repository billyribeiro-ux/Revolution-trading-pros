import { writable } from 'svelte/store';

export interface Toast {
	id: string;
	type: 'success' | 'error' | 'warning' | 'info';
	message: string;
	duration?: number;
}

export const toasts = writable<Toast[]>([]);

export function addToast(toast: Omit<Toast, 'id'>) {
	const id = Math.random().toString(36).substring(2, 9);
	const newToast = { ...toast, id };

	toasts.update((t) => [...t, newToast]);

	if (toast.duration !== 0) {
		setTimeout(() => {
			removeToast(id);
		}, toast.duration || 5000);
	}
}

export function removeToast(id: string) {
	toasts.update((t) => t.filter((toast) => toast.id !== id));
}

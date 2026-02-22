/**
 * Revolution Trading Pros - Enterprise Notification Store (Svelte 5 Runes)
 * ======================================================
 * Centralized notification management with persistence,
 * categorization, and real-time updates.
 *
 * @version 2.0.0 - Svelte 5 Runes Migration
 * @author Revolution Trading Pros
 * @level L8 Principal Engineer
 */

import { browser } from '$app/environment';
import { logger } from '$lib/utils/logger';
import { websocketService, type NotificationPayload } from '$lib/services/websocket';

export type NotificationType = 'info' | 'success' | 'warning' | 'error' | 'system';
export type NotificationPriority = 'low' | 'normal' | 'high' | 'urgent';

export interface Notification {
	id: string;
	type: NotificationType;
	priority: NotificationPriority;
	title: string;
	message: string;
	timestamp: Date;
	read: boolean;
	dismissed: boolean;
	action?: {
		label: string;
		href?: string;
		onClick?: () => void;
	};
	metadata?: Record<string, unknown>;
}

interface NotificationState {
	notifications: Notification[];
	isLoading: boolean;
	hasNewNotifications: boolean;
}

const STORAGE_KEY = 'rtp_notifications';
const MAX_STORED_NOTIFICATIONS = 100;

function generateId(): string {
	return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

// ═══════════════════════════════════════════════════════════════════════════
// State Loading
// ═══════════════════════════════════════════════════════════════════════════

function getInitialState(): NotificationState {
	if (browser) {
		try {
			const stored = localStorage.getItem(STORAGE_KEY);
			if (stored) {
				const parsed = JSON.parse(stored);
				return {
					notifications: parsed.notifications.map((n: Notification) => ({
						...n,
						timestamp: new Date(n.timestamp)
					})),
					isLoading: false,
					hasNewNotifications: parsed.notifications.some((n: Notification) => !n.read)
				};
			}
		} catch (error) {
			logger.error('Failed to load notifications:', error);
		}
	}
	return {
		notifications: [],
		isLoading: false,
		hasNewNotifications: false
	};
}

// Persist to localStorage
function persist(state: NotificationState) {
	if (browser) {
		try {
			// Only store limited number of notifications
			const toStore = {
				notifications: state.notifications.slice(0, MAX_STORED_NOTIFICATIONS)
			};
			localStorage.setItem(STORAGE_KEY, JSON.stringify(toStore));
		} catch (error) {
			logger.error('Failed to persist notifications:', error);
		}
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// State (Svelte 5 Runes)
// ═══════════════════════════════════════════════════════════════════════════

let notificationState = $state<NotificationState>(getInitialState());

// ═══════════════════════════════════════════════════════════════════════════
// Sound and Browser Notification Helpers
// ═══════════════════════════════════════════════════════════════════════════

function playNotificationSound() {
	try {
		// Create a simple beep using Web Audio API
		const audioContext = new (
			window.AudioContext ||
			(window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext
		)();
		const oscillator = audioContext.createOscillator();
		const gainNode = audioContext.createGain();

		oscillator.connect(gainNode);
		gainNode.connect(audioContext.destination);

		oscillator.frequency.value = 800;
		oscillator.type = 'sine';
		gainNode.gain.value = 0.1;

		oscillator.start();
		oscillator.stop(audioContext.currentTime + 0.1);
	} catch {
		// Audio not available
	}
}

async function showBrowserNotification(notification: Notification) {
	if (!('Notification' in window)) return;

	if (Notification.permission === 'default') {
		await Notification.requestPermission();
	}

	if (Notification.permission === 'granted') {
		new Notification(notification.title, {
			body: notification.message,
			icon: '/favicon.png',
			tag: notification.id
		});
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// WebSocket Integration
// ═══════════════════════════════════════════════════════════════════════════

let wsUnsubscribe: (() => void) | null = null;

function initWebSocket(userId?: string) {
	if (!browser || !userId) {
		return;
	}

	// Subscribe to user notifications via WebSocket
	wsUnsubscribe = websocketService.subscribeToNotifications(
		userId,
		(notification: NotificationPayload) => {
			// Convert WebSocket notification to our format
			notificationStore.add({
				id: notification.id,
				type: notification.type,
				priority: notification.priority,
				title: notification.title,
				message: notification.message,
				timestamp: new Date(notification.timestamp),
				read: notification.read,
				dismissed: notification.dismissed,
				...(notification.action && {
					action: {
						label: notification.action.label,
						...(notification.action.href && { href: notification.action.href })
					}
				}),
				...(notification.metadata && {
					metadata: notification.metadata as Record<string, unknown>
				})
			});
		}
	);
}

function destroyWebSocket() {
	if (wsUnsubscribe) {
		wsUnsubscribe();
		wsUnsubscribe = null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// Notification Store API
// ═══════════════════════════════════════════════════════════════════════════

export const notificationStore = {
	get state() {
		return notificationState;
	},

	get notifications() {
		return notificationState.notifications;
	},

	get hasNewNotifications() {
		return notificationState.hasNewNotifications;
	},

	add(notification: Partial<Notification> & { title: string; message: string }) {
		const newNotification: Notification = {
			id: notification.id || generateId(),
			type: notification.type || 'info',
			priority: notification.priority || 'normal',
			title: notification.title,
			message: notification.message,
			timestamp: notification.timestamp || new Date(),
			read: notification.read ?? false,
			dismissed: notification.dismissed ?? false,
			...(notification.action && { action: notification.action }),
			...(notification.metadata && { metadata: notification.metadata })
		};

		const newState = {
			...notificationState,
			notifications: [newNotification, ...notificationState.notifications].slice(
				0,
				MAX_STORED_NOTIFICATIONS
			),
			hasNewNotifications: true
		};
		persist(newState);
		notificationState = newState;

		// Play notification sound for high priority
		if (browser && (notification.priority === 'high' || notification.priority === 'urgent')) {
			playNotificationSound();
		}

		// Show browser notification for urgent
		if (browser && notification.priority === 'urgent') {
			showBrowserNotification(newNotification);
		}

		return newNotification.id;
	},

	markAsRead(id: string) {
		const newState = {
			...notificationState,
			notifications: notificationState.notifications.map((n) =>
				n.id === id ? { ...n, read: true } : n
			),
			hasNewNotifications: notificationState.notifications.some((n) => n.id !== id && !n.read)
		};
		persist(newState);
		notificationState = newState;
	},

	markAllAsRead() {
		const newState = {
			...notificationState,
			notifications: notificationState.notifications.map((n) => ({ ...n, read: true })),
			hasNewNotifications: false
		};
		persist(newState);
		notificationState = newState;
	},

	dismiss(id: string) {
		const newState = {
			...notificationState,
			notifications: notificationState.notifications.map((n) =>
				n.id === id ? { ...n, dismissed: true, read: true } : n
			)
		};
		persist(newState);
		notificationState = newState;
	},

	remove(id: string) {
		const newState = {
			...notificationState,
			notifications: notificationState.notifications.filter((n) => n.id !== id),
			hasNewNotifications: notificationState.notifications.some((n) => n.id !== id && !n.read)
		};
		persist(newState);
		notificationState = newState;
	},

	clear() {
		const newState: NotificationState = {
			notifications: [],
			isLoading: false,
			hasNewNotifications: false
		};
		notificationState = newState;
		persist(newState);
	},

	initWebSocket,
	destroyWebSocket,

	// Convenience methods for different types
	info(title: string, message: string, options?: Partial<Notification>) {
		return this.add({ ...options, type: 'info', title, message });
	},

	success(title: string, message: string, options?: Partial<Notification>) {
		return this.add({ ...options, type: 'success', title, message });
	},

	warning(title: string, message: string, options?: Partial<Notification>) {
		return this.add({ ...options, type: 'warning', title, message });
	},

	error(title: string, message: string, options?: Partial<Notification>) {
		return this.add({ ...options, type: 'error', title, message });
	},

	system(title: string, message: string, options?: Partial<Notification>) {
		return this.add({ ...options, type: 'system', title, message });
	}
};

// ═══════════════════════════════════════════════════════════════════════════
// Getter Functions (Svelte 5 - cannot export $derived from modules)
// ═══════════════════════════════════════════════════════════════════════════

export function getNotifications(): Notification[] {
	return notificationState.notifications;
}

export function getUnreadNotifications(): Notification[] {
	return notificationState.notifications.filter((n) => !n.read && !n.dismissed);
}

export function getUnreadCount(): number {
	return notificationState.notifications.filter((n) => !n.read && !n.dismissed).length;
}

export function getHasNewNotifications(): boolean {
	return notificationState.hasNewNotifications;
}

export function getUrgentNotifications(): Notification[] {
	return notificationState.notifications.filter((n) => n.priority === 'urgent' && !n.dismissed);
}

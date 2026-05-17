/**
 * Service Worker Registration - Apple ICT 11+ Pattern
 * ══════════════════════════════════════════════════════════════════════════════
 * SvelteKit handles service worker registration automatically when
 * serviceWorker: { register: true } is set in svelte.config.js
 *
 * This module provides utilities for manual control when needed
 * ══════════════════════════════════════════════════════════════════════════════
 */

import { dev } from '$app/environment';

export async function registerServiceWorker(): Promise<void> {
	// SvelteKit handles registration automatically via svelte.config.js
	// This function is kept for backward compatibility but is now a no-op
	// The service worker is registered from src/service-worker.ts

	if (typeof window === 'undefined' || !('serviceWorker' in navigator)) {
		return;
	}

	if (dev) {
		const registrations = await navigator.serviceWorker.getRegistrations();
		await Promise.all(registrations.map((registration) => registration.unregister()));

		if ('caches' in window) {
			const keys = await caches.keys();
			await Promise.all(keys.map((key) => caches.delete(key)));
		}

		return;
	}

	// Listen for service worker updates and prompt user to refresh
	navigator.serviceWorker.addEventListener('controllerchange', () => {
		console.info('[SW] New service worker activated');
	});
}

/**
 * Force update the service worker (useful after deployment)
 */
export async function updateServiceWorker(): Promise<void> {
	if (typeof window === 'undefined' || !('serviceWorker' in navigator)) {
		return;
	}

	try {
		const registration = await navigator.serviceWorker.getRegistration();
		if (registration) {
			await registration.update();
			console.info('[SW] Service worker update triggered');
		}
	} catch (error) {
		console.error('[SW] Update failed:', error);
	}
}

/**
 * Skip waiting and activate new service worker immediately
 */
export async function skipWaiting(): Promise<void> {
	if (typeof window === 'undefined' || !('serviceWorker' in navigator)) {
		return;
	}

	const registration = await navigator.serviceWorker.getRegistration();
	if (registration?.waiting) {
		registration.waiting.postMessage({ type: 'SKIP_WAITING' });
	}
}

/**
 * Unregister service worker (for debugging)
 */
export async function unregisterServiceWorker(): Promise<void> {
	if (typeof window === 'undefined' || !('serviceWorker' in navigator)) {
		return;
	}

	try {
		const registrations = await navigator.serviceWorker.getRegistrations();
		for (const registration of registrations) {
			await registration.unregister();
		}
		console.info('Service worker unregistered');
	} catch (error) {
		console.error('Service worker unregistration failed:', error);
	}
}

/**
 * Service Worker Registration - Google L11+ Pattern
 * ══════════════════════════════════════════════════════════════════════════════
 * Registers service worker for offline support and instant subsequent loads
 * Non-blocking registration that doesn't impact initial page load
 * ══════════════════════════════════════════════════════════════════════════════
 */

export async function registerServiceWorker(): Promise<void> {
	// Only register in production and if service workers are supported
	if (
		typeof window === 'undefined' ||
		!('serviceWorker' in navigator) ||
		import.meta.env.DEV
	) {
		return;
	}

	try {
		// Register after page load to not block initial render
		if (document.readyState === 'complete') {
			await register();
		} else {
			window.addEventListener('load', register);
		}
	} catch (error) {
		console.error('Service worker registration failed:', error);
	}
}

async function register(): Promise<void> {
	try {
		const registration = await navigator.serviceWorker.register('/sw.js', {
			scope: '/',
			// Update service worker in background without blocking
			updateViaCache: 'none'
		});

		console.log('Service worker registered:', registration.scope);

		// Check for updates periodically (every hour)
		setInterval(() => {
			registration.update();
		}, 60 * 60 * 1000);

		// Listen for updates
		registration.addEventListener('updatefound', () => {
			const newWorker = registration.installing;
			if (!newWorker) return;

			newWorker.addEventListener('statechange', () => {
				if (newWorker.state === 'installed' && navigator.serviceWorker.controller) {
					// New service worker available, notify user
					console.log('New version available! Refresh to update.');
					// You can show a toast notification here
				}
			});
		});
	} catch (error) {
		console.error('Service worker registration failed:', error);
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
		console.log('Service worker unregistered');
	} catch (error) {
		console.error('Service worker unregistration failed:', error);
	}
}

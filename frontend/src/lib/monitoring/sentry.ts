/**
 * Sentry Error Tracking
 * ═══════════════════════════════════════════════════════════════════════════
 */

import * as Sentry from '@sentry/sveltekit';
import { dev } from '$app/environment';
import { logger } from '$lib/utils/logger';

const SENTRY_DSN = import.meta.env.VITE_SENTRY_DSN;

export function initSentry(): void {
	if (dev || !SENTRY_DSN) return;

	Sentry.init({
		dsn: SENTRY_DSN,
		environment: import.meta.env.MODE,
		tracesSampleRate: 0.1,
		replaysSessionSampleRate: 0.1,
		replaysOnErrorSampleRate: 1.0,

		integrations: [
			Sentry.replayIntegration({
				maskAllText: false,
				blockAllMedia: false
			})
		],

		beforeSend(event, hint) {
			// Filter out certain errors
			if (event.exception) {
				const error = hint.originalException;
				if (error instanceof Error) {
					// Ignore network errors
					if (error.message.includes('NetworkError')) {
						return null;
					}
					// Ignore cancelled requests
					if (error.message.includes('AbortError')) {
						return null;
					}
				}
			}
			return event;
		}
	});
}

/**
 * Capture exception
 */
export function captureException(error: Error, context?: Record<string, any>): void {
	if (dev) {
		logger.error('Error:', error, context);
		return;
	}

	Sentry.captureException(error, {
		extra: context
	});
}

/**
 * Capture message
 */
export function captureMessage(message: string, level: Sentry.SeverityLevel = 'info'): void {
	if (dev) {
		logger.info(`[${level}]`, message);
		return;
	}

	Sentry.captureMessage(message, level);
}

/**
 * Set user context
 */
export function setUser(user: { id: string; email?: string; username?: string }): void {
	if (dev) return;

	Sentry.setUser(user);
}

/**
 * Clear user context
 */
export function clearUser(): void {
	if (dev) return;

	Sentry.setUser(null);
}

/**
 * Add breadcrumb
 */
export function addBreadcrumb(breadcrumb: Sentry.Breadcrumb): void {
	if (dev) return;

	Sentry.addBreadcrumb(breadcrumb);
}

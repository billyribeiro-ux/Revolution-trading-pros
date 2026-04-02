/**
 * Error Monitoring — Logger-Based Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 * Replaces Sentry with structured logging. Same API surface for seamless
 * migration. Plug in any future error tracking service here.
 */

import { dev } from '$app/environment';
import { logger } from '$lib/utils/logger';

export type SeverityLevel = 'fatal' | 'error' | 'warning' | 'log' | 'info' | 'debug';

export interface Breadcrumb {
	type?: string;
	category?: string;
	message?: string;
	data?: Record<string, unknown>;
	level?: SeverityLevel;
	timestamp?: number;
}

let _currentUser: { id: string; email?: string; username?: string } | null = null;

export function initSentry(): void {
	// No-op — logger is always available
	if (!dev) {
		logger.info('[monitoring] Error tracking initialized (logger mode)');
	}
}

export function captureException(error: Error, context?: Record<string, unknown>): void {
	const userTag = _currentUser ? ` [user:${_currentUser.id}]` : '';
	logger.error(`[monitoring]${userTag} Exception:`, error.message, context ?? '');
}

export function captureMessage(message: string, level: SeverityLevel = 'info'): void {
	const userTag = _currentUser ? ` [user:${_currentUser.id}]` : '';
	switch (level) {
		case 'fatal':
		case 'error':
			logger.error(`[monitoring]${userTag}`, message);
			break;
		case 'warning':
			logger.warn(`[monitoring]${userTag}`, message);
			break;
		default:
			logger.info(`[monitoring]${userTag} [${level}]`, message);
	}
}

export function setUser(user: { id: string; email?: string; username?: string }): void {
	_currentUser = user;
}

export function clearUser(): void {
	_currentUser = null;
}

export function addBreadcrumb(breadcrumb: Breadcrumb): void {
	if (dev) {
		logger.info('[breadcrumb]', breadcrumb.category ?? '', breadcrumb.message ?? '');
	}
}

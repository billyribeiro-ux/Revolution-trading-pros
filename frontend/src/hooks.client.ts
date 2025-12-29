/**
 * SvelteKit Client Hooks - Enterprise Error Tracking & Performance Monitoring
 *
 * Implements client-side error handling and performance monitoring:
 * - Unhandled error capture
 * - Navigation error tracking
 * - Performance metrics collection
 * - Error reporting to external services
 *
 * @version 1.0.0 - L8 Principal Engineer
 * @see https://kit.svelte.dev/docs/hooks#client-hooks
 */

import type { HandleClientError } from '@sveltejs/kit';

/**
 * Error severity levels for categorization
 */
type ErrorSeverity = 'low' | 'medium' | 'high' | 'critical';

/**
 * Error metadata for enhanced tracking
 */
interface ErrorMetadata {
	severity: ErrorSeverity;
	category: string;
	userAgent: string;
	url: string;
	timestamp: string;
	sessionId?: string;
	userId?: number;
}

/**
 * Determine error severity based on error type and message
 */
function getErrorSeverity(error: unknown): ErrorSeverity {
	const message = error instanceof Error ? error.message.toLowerCase() : String(error).toLowerCase();

	// Critical: Security or data integrity issues
	if (
		message.includes('unauthorized') ||
		message.includes('forbidden') ||
		message.includes('xss') ||
		message.includes('injection')
	) {
		return 'critical';
	}

	// High: System failures
	if (
		message.includes('network') ||
		message.includes('timeout') ||
		message.includes('server error') ||
		message.includes('500')
	) {
		return 'high';
	}

	// Medium: User-facing issues
	if (
		message.includes('not found') ||
		message.includes('invalid') ||
		message.includes('failed')
	) {
		return 'medium';
	}

	// Low: Minor issues
	return 'low';
}

/**
 * Categorize error by type
 */
function categorizeError(error: unknown): string {
	if (error instanceof TypeError) return 'type_error';
	if (error instanceof ReferenceError) return 'reference_error';
	if (error instanceof SyntaxError) return 'syntax_error';
	if (error instanceof RangeError) return 'range_error';

	const message = error instanceof Error ? error.message.toLowerCase() : '';

	if (message.includes('network') || message.includes('fetch')) return 'network_error';
	if (message.includes('chunk') || message.includes('module')) return 'loading_error';
	if (message.includes('storage') || message.includes('quota')) return 'storage_error';
	if (message.includes('permission') || message.includes('denied')) return 'permission_error';

	return 'unknown_error';
}

/**
 * Get session ID from localStorage if available
 */
function getSessionId(): string | undefined {
	try {
		return localStorage.getItem('rtp_session_id') || undefined;
	} catch {
		return undefined;
	}
}

/**
 * Get user ID from auth state if available
 */
function getUserId(): number | undefined {
	try {
		const authState = localStorage.getItem('rtp_auth_state');
		if (authState) {
			const parsed = JSON.parse(authState);
			return parsed?.user?.id;
		}
	} catch {
		// Silently fail
	}
	return undefined;
}

/**
 * Report error to external tracking service
 */
async function reportError(
	error: unknown,
	metadata: ErrorMetadata,
	errorId: string
): Promise<void> {
	// Only report in production
	if (import.meta.env.DEV) {
		console.group(`🚨 Client Error [${errorId}]`);
		console.error('Error:', error);
		console.info('Metadata:', metadata);
		console.groupEnd();
		return;
	}

	try {
		// Google Analytics 4 error tracking
		if (typeof window !== 'undefined' && 'gtag' in window) {
			(window as { gtag: (...args: unknown[]) => void }).gtag('event', 'exception', {
				description: error instanceof Error ? error.message : String(error),
				fatal: metadata.severity === 'critical' || metadata.severity === 'high',
				error_id: errorId,
				error_category: metadata.category,
				error_severity: metadata.severity
			});
		}

		// Send to custom error tracking endpoint (if configured)
		const errorEndpoint = import.meta.env['VITE_ERROR_TRACKING_URL'];
		if (errorEndpoint) {
			await fetch(errorEndpoint, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					errorId,
					message: error instanceof Error ? error.message : String(error),
					stack: error instanceof Error ? error.stack : undefined,
					...metadata
				}),
				// Don't block UI for error reporting
				keepalive: true
			});
		}
	} catch (reportingError) {
		// Silently fail - don't create infinite error loops
		if (import.meta.env.DEV) {
			console.warn('Failed to report error:', reportingError);
		}
	}
}

/**
 * Generate a unique error ID for tracking
 */
function generateErrorId(): string {
	return `err_${Date.now().toString(36)}_${Math.random().toString(36).substring(2, 7)}`;
}

/**
 * Check if error is a stale chunk/module loading error
 * This happens when Cloudflare Pages serves old cached HTML pointing to deleted chunks
 */
function isStaleChunkError(error: unknown): boolean {
	if (!(error instanceof Error)) return false;
	const message = error.message.toLowerCase();
	return (
		message.includes('failed to fetch dynamically imported module') ||
		message.includes('loading chunk') ||
		message.includes('loading css chunk') ||
		(message.includes('failed to fetch') && message.includes('immutable'))
	);
}

/**
 * Force reload the page to get fresh chunks
 * Uses a sessionStorage flag to prevent infinite reload loops
 */
function forceReloadForFreshChunks(): void {
	const RELOAD_KEY = 'rtp_chunk_reload';
	const lastReload = sessionStorage.getItem(RELOAD_KEY);
	const now = Date.now();
	
	// Prevent reload loop - only reload once per 30 seconds
	if (lastReload && now - parseInt(lastReload, 10) < 30000) {
		console.warn('[ChunkError] Already reloaded recently, not reloading again');
		return;
	}
	
	sessionStorage.setItem(RELOAD_KEY, now.toString());
	console.info('[ChunkError] Stale chunks detected, forcing page reload...');
	window.location.reload();
}

/**
 * Client-side error handler
 *
 * This function is called when an error is thrown during:
 * - Rendering
 * - Navigation
 * - Client-side component lifecycle
 */
export const handleError: HandleClientError = async ({ error, event, status: _status, message }) => {
	const errorId = generateErrorId();

	// ICT11+ Pattern: Handle stale chunk errors from Cloudflare Pages cache
	// When a new deployment happens, old HTML may reference chunks that no longer exist
	if (typeof window !== 'undefined' && isStaleChunkError(error)) {
		forceReloadForFreshChunks();
		// Return a friendly message while reload happens
		return {
			message: 'Updating to latest version...',
			errorId
		};
	}

	// Build error metadata
	const sessionId = getSessionId();
	const userId = getUserId();
	const metadata: ErrorMetadata = {
		severity: getErrorSeverity(error),
		category: categorizeError(error),
		userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown',
		url: event.url.href,
		timestamp: new Date().toISOString(),
		...(sessionId && { sessionId }),
		...(userId && { userId })
	};

	// Report error asynchronously
	reportError(error, metadata, errorId);

	// Development: Log full error details
	if (import.meta.env.DEV) {
		console.error(`[${errorId}] Unhandled client error:`, error);
	}

	// Return user-friendly error message
	// Include errorId for support reference
	return {
		message: import.meta.env.DEV
			? `${message} (Error ID: ${errorId})`
			: 'An unexpected error occurred. Please try again.',
		errorId
	};
};

/**
 * Global unhandled rejection handler
 * Catches Promise rejections that weren't caught by application code
 */
if (typeof window !== 'undefined') {
	window.addEventListener('unhandledrejection', (event) => {
		const errorId = generateErrorId();
		const error = event.reason;

		// ICT11+ Pattern: Handle stale chunk errors in unhandled promise rejections
		// This catches dynamic import failures that happen outside SvelteKit's error boundary
		if (isStaleChunkError(error)) {
			event.preventDefault(); // Prevent console error spam
			forceReloadForFreshChunks();
			return;
		}

		const sessionId = getSessionId();
		const userId = getUserId();
		const metadata: ErrorMetadata = {
			severity: getErrorSeverity(error),
			category: 'unhandled_promise',
			userAgent: navigator.userAgent,
			url: window.location.href,
			timestamp: new Date().toISOString(),
			...(sessionId && { sessionId }),
			...(userId && { userId })
		};

		reportError(error, metadata, errorId);

		if (import.meta.env.DEV) {
			console.error(`[${errorId}] Unhandled promise rejection:`, error);
		}
	});

	/**
	 * Global error handler for uncaught exceptions
	 */
	window.addEventListener('error', (event) => {
		// Ignore errors from browser extensions or external scripts
		if (event.filename && !event.filename.includes(window.location.origin)) {
			return;
		}

		const errorId = generateErrorId();
		const error = event.error || new Error(event.message);

		const sessionId = getSessionId();
		const userId = getUserId();
		const metadata: ErrorMetadata = {
			severity: getErrorSeverity(error),
			category: categorizeError(error),
			userAgent: navigator.userAgent,
			url: window.location.href,
			timestamp: new Date().toISOString(),
			...(sessionId && { sessionId }),
			...(userId && { userId })
		};

		reportError(error, metadata, errorId);
	});

	/**
	 * Performance monitoring - Report Core Web Vitals
	 */
	if ('PerformanceObserver' in window) {
		try {
			// Observe Largest Contentful Paint (LCP)
			const lcpObserver = new PerformanceObserver((list) => {
				const entries = list.getEntries();
				const lastEntry = entries[entries.length - 1] as PerformanceEntry & { startTime: number };

				if (import.meta.env.DEV) {
					console.info(`📊 LCP: ${lastEntry.startTime.toFixed(2)}ms`);
				}

				// Report to analytics
				if (typeof window !== 'undefined' && 'gtag' in window) {
					(window as { gtag: (...args: unknown[]) => void }).gtag('event', 'web_vitals', {
						metric_name: 'LCP',
						metric_value: lastEntry.startTime,
						metric_rating: lastEntry.startTime < 2500 ? 'good' : lastEntry.startTime < 4000 ? 'needs_improvement' : 'poor'
					});
				}
			});
			lcpObserver.observe({ type: 'largest-contentful-paint', buffered: true });

			// Observe First Input Delay (FID)
			const fidObserver = new PerformanceObserver((list) => {
				const entries = list.getEntries();
				for (const entry of entries) {
					const fidEntry = entry as PerformanceEventTiming;
					const fid = fidEntry.processingStart - fidEntry.startTime;

					if (import.meta.env.DEV) {
						console.info(`📊 FID: ${fid.toFixed(2)}ms`);
					}

					if (typeof window !== 'undefined' && 'gtag' in window) {
						(window as { gtag: (...args: unknown[]) => void }).gtag('event', 'web_vitals', {
							metric_name: 'FID',
							metric_value: fid,
							metric_rating: fid < 100 ? 'good' : fid < 300 ? 'needs_improvement' : 'poor'
						});
					}
				}
			});
			fidObserver.observe({ type: 'first-input', buffered: true });

			// Observe Cumulative Layout Shift (CLS)
			let clsValue = 0;
			const clsObserver = new PerformanceObserver((list) => {
				for (const entry of list.getEntries()) {
					const layoutShiftEntry = entry as PerformanceEntry & { hadRecentInput: boolean; value: number };
					if (!layoutShiftEntry.hadRecentInput) {
						clsValue += layoutShiftEntry.value;
					}
				}
			});
			clsObserver.observe({ type: 'layout-shift', buffered: true });

			// Report CLS on page hide
			document.addEventListener('visibilitychange', () => {
				if (document.visibilityState === 'hidden') {
					if (import.meta.env.DEV) {
						console.info(`📊 CLS: ${clsValue.toFixed(4)}`);
					}

					if (typeof window !== 'undefined' && 'gtag' in window) {
						(window as { gtag: (...args: unknown[]) => void }).gtag('event', 'web_vitals', {
							metric_name: 'CLS',
							metric_value: clsValue,
							metric_rating: clsValue < 0.1 ? 'good' : clsValue < 0.25 ? 'needs_improvement' : 'poor'
						});
					}
				}
			});
		} catch {
			// PerformanceObserver not fully supported
		}
	}
}

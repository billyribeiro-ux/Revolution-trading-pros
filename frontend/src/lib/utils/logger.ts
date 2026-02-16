/**
 * Production-safe logger utility
 *
 * Only logs in development mode. In production, all logs are suppressed
 * to prevent information leakage and improve performance.
 *
 * @example
 * import { logger } from '$lib/utils/logger';
 * logger.log('Debug info'); // Only logs in dev
 * logger.error('Critical error', error); // Only logs in dev
 */

const isDev = import.meta.env.DEV;

type LogLevel = 'log' | 'warn' | 'error' | 'debug' | 'info' | 'trace';

interface Logger {
	log: (...args: unknown[]) => void;
	warn: (...args: unknown[]) => void;
	error: (...args: unknown[]) => void;
	debug: (...args: unknown[]) => void;
	info: (...args: unknown[]) => void;
	trace: (...args: unknown[]) => void;
	group: (label: string) => void;
	groupEnd: () => void;
	table: (data: unknown) => void;
	time: (label: string) => void;
	timeEnd: (label: string) => void;
}

const noop = (): void => {};

/**
 * Creates a logging function that only executes in development
 */
const createLogFn = (level: LogLevel): ((...args: unknown[]) => void) => {
	if (!isDev) return noop;
	return (...args: unknown[]): void => {
		console[level](...args);
	};
};

/**
 * Production-safe logger
 * All methods are no-ops in production
 */
export const logger: Logger = {
	log: createLogFn('log'),
	warn: createLogFn('warn'),
	error: createLogFn('error'),
	debug: createLogFn('debug'),
	info: createLogFn('info'),
	trace: createLogFn('trace'),
	group: isDev ? (label: string) => console.group(label) : noop,
	groupEnd: isDev ? () => console.groupEnd() : noop,
	table: isDev ? (data: unknown) => console.table(data) : noop,
	time: isDev ? (label: string) => console.time(label) : noop,
	timeEnd: isDev ? (label: string) => console.timeEnd(label) : noop
};

/**
 * Performance logger for timing operations
 * Only active in development
 */
export const perfLogger = {
	start: (label: string): void => {
		if (isDev) {
			performance.mark(`${label}-start`);
		}
	},
	end: (label: string): void => {
		if (isDev) {
			performance.mark(`${label}-end`);
			try {
				performance.measure(label, `${label}-start`, `${label}-end`);
				const measure = performance.getEntriesByName(label)[0];
				if (measure) {
					logger.info(`[PERF] ${label}: ${measure.duration.toFixed(2)}ms`);
				}
			} catch {
				// Ignore if marks don't exist
			}
		}
	}
};

export default logger;

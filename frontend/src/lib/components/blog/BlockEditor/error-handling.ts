/**
 * Block Error Handling Utilities - Enterprise Error Management
 * =============================================================================
 *
 * Provides comprehensive error handling for the block editor:
 * - Error type classification
 * - Block error capture and logging
 * - Recovery mechanisms for common issues
 * - Serialization for backend reporting
 * - User-friendly error messages
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 * @since January 2026
 */

import { writable, derived, type Readable, type Writable } from 'svelte/store';
import type { Block, BlockType, BlockContent, BlockSettings, BlockMetadata } from './types';
import { BLOCK_DEFINITIONS } from './types';

// =============================================================================
// Error Types
// =============================================================================

/**
 * Error type classification for block editor errors
 */
export const ErrorType = {
	/** Errors during component rendering */
	RENDER_ERROR: 'RENDER_ERROR',
	/** Block content/settings validation failures */
	VALIDATION_ERROR: 'VALIDATION_ERROR',
	/** Errors saving block data */
	SAVE_ERROR: 'SAVE_ERROR',
	/** Network-related failures */
	NETWORK_ERROR: 'NETWORK_ERROR',
	/** AI generation/processing errors */
	AI_ERROR: 'AI_ERROR',
	/** Media upload/processing errors */
	MEDIA_ERROR: 'MEDIA_ERROR',
	/** Unknown/uncategorized errors */
	UNKNOWN_ERROR: 'UNKNOWN_ERROR'
} as const;

export type ErrorTypeValue = (typeof ErrorType)[keyof typeof ErrorType];

/**
 * Error severity levels
 */
export const ErrorSeverity = {
	/** Critical - Block completely unusable */
	CRITICAL: 'critical',
	/** High - Major functionality broken */
	HIGH: 'high',
	/** Medium - Some features affected */
	MEDIUM: 'medium',
	/** Low - Minor issues, mostly functional */
	LOW: 'low'
} as const;

export type ErrorSeverityValue = (typeof ErrorSeverity)[keyof typeof ErrorSeverity];

// =============================================================================
// Interfaces
// =============================================================================

/**
 * Serialized error for transport/storage
 */
export interface SerializedError {
	/** Error name/constructor */
	name: string;
	/** Error message */
	message: string;
	/** Stack trace (if available) */
	stack?: string;
	/** Error type classification */
	type: ErrorTypeValue;
	/** Error severity */
	severity: ErrorSeverityValue;
	/** Timestamp when error occurred */
	timestamp: string;
	/** Block ID where error occurred */
	blockId?: string;
	/** Block type where error occurred */
	blockType?: BlockType;
	/** Whether recovery was attempted */
	recoveryAttempted: boolean;
	/** Whether error is recoverable */
	isRecoverable: boolean;
	/** Additional context data */
	context?: Record<string, unknown>;
	/** Cause chain (if Error.cause is present) */
	cause?: SerializedError;
}

/**
 * Block error context for logging
 */
export interface BlockErrorContext {
	/** Component name where error occurred */
	componentName?: string;
	/** Action being performed when error occurred */
	action?: string;
	/** User ID (if available) */
	userId?: string;
	/** Session ID */
	sessionId?: string;
	/** Browser info */
	userAgent?: string;
	/** Page URL */
	url?: string;
	/** Additional metadata */
	metadata?: Record<string, unknown>;
}

/**
 * Block error record for tracking
 */
export interface BlockErrorRecord {
	/** Unique error ID */
	id: string;
	/** Block that caused the error */
	block: Block;
	/** The error itself */
	error: Error;
	/** Serialized version */
	serialized: SerializedError;
	/** Context information */
	context: BlockErrorContext;
	/** When the error occurred */
	timestamp: Date;
	/** Number of recovery attempts */
	recoveryAttempts: number;
	/** Whether block was recovered */
	recovered: boolean;
}

/**
 * Error tracking store state
 */
export interface ErrorTrackingState {
	/** All captured errors */
	errors: BlockErrorRecord[];
	/** Errors by block ID */
	errorsByBlock: Map<string, BlockErrorRecord[]>;
	/** Total error count */
	totalCount: number;
	/** Unrecovered error count */
	unrecoveredCount: number;
	/** Whether there are critical errors */
	hasCriticalErrors: boolean;
}

// =============================================================================
// User-Friendly Error Messages
// =============================================================================

/**
 * User-friendly messages by error type
 */
const USER_FRIENDLY_MESSAGES: Record<ErrorTypeValue, string> = {
	[ErrorType.RENDER_ERROR]: 'This block encountered a display error.',
	[ErrorType.VALIDATION_ERROR]: 'The block content is invalid.',
	[ErrorType.SAVE_ERROR]: 'Unable to save this block.',
	[ErrorType.NETWORK_ERROR]: 'Network connection issue. Please check your connection.',
	[ErrorType.AI_ERROR]: 'AI content generation failed.',
	[ErrorType.MEDIA_ERROR]: 'Media processing failed.',
	[ErrorType.UNKNOWN_ERROR]: 'An unexpected error occurred.'
};

/**
 * Block-type specific error messages
 */
const BLOCK_TYPE_MESSAGES: Partial<Record<BlockType, Record<ErrorTypeValue, string>>> = {
	image: {
		[ErrorType.RENDER_ERROR]: 'Unable to display this image.',
		[ErrorType.MEDIA_ERROR]: 'Image upload or processing failed.',
		[ErrorType.NETWORK_ERROR]: 'Unable to load image. Check your connection.',
		[ErrorType.VALIDATION_ERROR]: 'Invalid image format or size.',
		[ErrorType.SAVE_ERROR]: 'Failed to save image block.',
		[ErrorType.AI_ERROR]: 'AI image generation failed.',
		[ErrorType.UNKNOWN_ERROR]: 'An unexpected error occurred with this image.'
	},
	video: {
		[ErrorType.RENDER_ERROR]: 'Unable to display this video.',
		[ErrorType.MEDIA_ERROR]: 'Video processing failed.',
		[ErrorType.NETWORK_ERROR]: 'Unable to load video. Check your connection.',
		[ErrorType.VALIDATION_ERROR]: 'Invalid video URL or format.',
		[ErrorType.SAVE_ERROR]: 'Failed to save video block.',
		[ErrorType.AI_ERROR]: 'AI video generation failed.',
		[ErrorType.UNKNOWN_ERROR]: 'An unexpected error occurred with this video.'
	},
	chart: {
		[ErrorType.RENDER_ERROR]: 'Unable to render trading chart.',
		[ErrorType.NETWORK_ERROR]: 'Unable to fetch chart data. Check your connection.',
		[ErrorType.VALIDATION_ERROR]: 'Invalid ticker symbol or chart configuration.',
		[ErrorType.SAVE_ERROR]: 'Failed to save chart block.',
		[ErrorType.AI_ERROR]: 'AI chart analysis failed.',
		[ErrorType.MEDIA_ERROR]: 'Chart data processing failed.',
		[ErrorType.UNKNOWN_ERROR]: 'An unexpected error occurred with this chart.'
	},
	code: {
		[ErrorType.RENDER_ERROR]: 'Unable to display code block.',
		[ErrorType.VALIDATION_ERROR]: 'Invalid code syntax or language.',
		[ErrorType.SAVE_ERROR]: 'Failed to save code block.',
		[ErrorType.AI_ERROR]: 'AI code generation failed.',
		[ErrorType.NETWORK_ERROR]: 'Network error loading code highlighter.',
		[ErrorType.MEDIA_ERROR]: 'Code processing failed.',
		[ErrorType.UNKNOWN_ERROR]: 'An unexpected error occurred with this code block.'
	},
	aiGenerated: {
		[ErrorType.RENDER_ERROR]: 'Unable to display AI content.',
		[ErrorType.AI_ERROR]: 'AI content generation failed. Please try again.',
		[ErrorType.NETWORK_ERROR]: 'Unable to reach AI service. Check your connection.',
		[ErrorType.VALIDATION_ERROR]: 'Invalid AI prompt or configuration.',
		[ErrorType.SAVE_ERROR]: 'Failed to save AI-generated content.',
		[ErrorType.MEDIA_ERROR]: 'AI content processing failed.',
		[ErrorType.UNKNOWN_ERROR]: 'An unexpected error occurred with AI generation.'
	},
	embed: {
		[ErrorType.RENDER_ERROR]: 'Unable to display embedded content.',
		[ErrorType.NETWORK_ERROR]: 'Unable to load embed. Check your connection.',
		[ErrorType.VALIDATION_ERROR]: 'Invalid embed URL or unsupported platform.',
		[ErrorType.SAVE_ERROR]: 'Failed to save embed block.',
		[ErrorType.AI_ERROR]: 'AI embed processing failed.',
		[ErrorType.MEDIA_ERROR]: 'Embed content processing failed.',
		[ErrorType.UNKNOWN_ERROR]: 'An unexpected error occurred with this embed.'
	}
};

// =============================================================================
// Error Classification
// =============================================================================

/**
 * Known recoverable error patterns
 */
const RECOVERABLE_PATTERNS: Array<{ pattern: RegExp; type: ErrorTypeValue }> = [
	{ pattern: /undefined is not an object/i, type: ErrorType.RENDER_ERROR },
	{ pattern: /cannot read propert/i, type: ErrorType.RENDER_ERROR },
	{ pattern: /null is not an object/i, type: ErrorType.RENDER_ERROR },
	{ pattern: /is not a function/i, type: ErrorType.RENDER_ERROR },
	{ pattern: /network|fetch|timeout|abort/i, type: ErrorType.NETWORK_ERROR },
	{ pattern: /failed to fetch/i, type: ErrorType.NETWORK_ERROR },
	{ pattern: /validation|invalid|required/i, type: ErrorType.VALIDATION_ERROR },
	{ pattern: /save|persist|store/i, type: ErrorType.SAVE_ERROR },
	{ pattern: /ai|generate|openai|claude|gpt/i, type: ErrorType.AI_ERROR },
	{ pattern: /media|image|video|audio|upload/i, type: ErrorType.MEDIA_ERROR }
];

/**
 * Classify an error by type
 */
export function classifyError(error: Error): ErrorTypeValue {
	const message = error.message.toLowerCase();
	const name = error.name.toLowerCase();
	const combined = `${name} ${message}`;

	for (const { pattern, type } of RECOVERABLE_PATTERNS) {
		if (pattern.test(combined)) {
			return type;
		}
	}

	return ErrorType.UNKNOWN_ERROR;
}

/**
 * Determine error severity
 */
export function determineErrorSeverity(error: Error, type: ErrorTypeValue): ErrorSeverityValue {
	// Network errors are usually medium - transient
	if (type === ErrorType.NETWORK_ERROR) {
		return ErrorSeverity.MEDIUM;
	}

	// Validation errors are low - user can fix
	if (type === ErrorType.VALIDATION_ERROR) {
		return ErrorSeverity.LOW;
	}

	// AI errors are medium - can retry
	if (type === ErrorType.AI_ERROR) {
		return ErrorSeverity.MEDIUM;
	}

	// Render errors can be critical
	if (type === ErrorType.RENDER_ERROR) {
		// Check if it's a complete render failure
		if (/cannot read|undefined|null/i.test(error.message)) {
			return ErrorSeverity.HIGH;
		}
		return ErrorSeverity.MEDIUM;
	}

	// Save errors are high - data loss risk
	if (type === ErrorType.SAVE_ERROR) {
		return ErrorSeverity.HIGH;
	}

	return ErrorSeverity.MEDIUM;
}

// =============================================================================
// Core Functions
// =============================================================================

/**
 * Capture and log a block error
 */
export function captureBlockError(
	error: Error,
	block: Block,
	context: BlockErrorContext = {}
): BlockErrorRecord {
	const type = classifyError(error);
	const severity = determineErrorSeverity(error, type);
	const serialized = serializeError(error, block, type, severity);
	const timestamp = new Date();

	const record: BlockErrorRecord = {
		id: `err_${timestamp.getTime()}_${Math.random().toString(36).slice(2, 8)}`,
		block: structuredClone(block), // Preserve block data
		error,
		serialized,
		context: {
			...context,
			url: typeof window !== 'undefined' ? window.location.href : undefined,
			userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : undefined
		},
		timestamp,
		recoveryAttempts: 0,
		recovered: false
	};

	// Log to console
	console.error(`[BlockError] ${type} in ${block.type} block (${block.id}):`, error);
	if (import.meta.env.DEV) {
		console.error('[BlockError] Context:', context);
		console.error('[BlockError] Block data:', block);
	}

	// Add to error tracking store
	addErrorToStore(record);

	// Optionally send to backend
	if (typeof window !== 'undefined' && !import.meta.env.DEV) {
		reportErrorToBackend(record).catch((e) => {
			console.warn('[BlockError] Failed to report error to backend:', e);
		});
	}

	return record;
}

/**
 * Attempt to recover a block by fixing common issues
 */
export function recoverBlock(block: Block): Block {
	const recovered = structuredClone(block);
	const definition = BLOCK_DEFINITIONS[block.type];

	// Reset content to defaults if corrupted
	if (!recovered.content || typeof recovered.content !== 'object') {
		recovered.content = {
			...(definition?.defaultContent ?? {})
		} as BlockContent;
	}

	// Reset settings to defaults if corrupted
	if (!recovered.settings || typeof recovered.settings !== 'object') {
		recovered.settings = {
			...(definition?.defaultSettings ?? {})
		} as BlockSettings;
	}

	// Ensure metadata exists
	if (!recovered.metadata || typeof recovered.metadata !== 'object') {
		recovered.metadata = {
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
			version: 1
		} as BlockMetadata;
	}

	// Type-specific recovery
	switch (block.type) {
		case 'paragraph':
		case 'heading':
		case 'quote':
		case 'pullquote':
		case 'callout':
		case 'button':
			// Text blocks - ensure text content exists
			if (typeof recovered.content.text !== 'string') {
				recovered.content.text = '';
			}
			break;

		case 'code':
		case 'preformatted':
			// Code blocks - ensure code content exists
			if (typeof recovered.content.code !== 'string') {
				recovered.content.code = '';
			}
			if (!recovered.content.language) {
				recovered.content.language = 'javascript';
			}
			break;

		case 'list':
			// List blocks - ensure items array exists
			if (!Array.isArray(recovered.content.listItems)) {
				recovered.content.listItems = [''];
			}
			if (!recovered.content.listType) {
				recovered.content.listType = 'bullet';
			}
			break;

		case 'checklist':
			// Checklist blocks - ensure items array exists
			if (!Array.isArray(recovered.content.items)) {
				recovered.content.items = [];
			}
			break;

		case 'image':
		case 'video':
		case 'audio':
		case 'file':
			// Media blocks - clear invalid URLs
			if (recovered.content.mediaUrl && typeof recovered.content.mediaUrl !== 'string') {
				delete recovered.content.mediaUrl;
			}
			break;

		case 'embed':
			// Embed blocks - validate URL
			if (recovered.content.embedUrl && typeof recovered.content.embedUrl !== 'string') {
				delete recovered.content.embedUrl;
			}
			break;

		case 'columns':
		case 'group':
		case 'row':
		case 'buttons':
		case 'card':
		case 'cta':
		case 'toggle':
			// Container blocks - ensure children array exists
			if (!Array.isArray(recovered.content.children)) {
				recovered.content.children = [];
			}
			break;

		case 'chart':
		case 'ticker':
			// Trading blocks - ensure ticker exists
			if (!recovered.content.ticker || typeof recovered.content.ticker !== 'string') {
				recovered.content.ticker = 'SPY';
			}
			break;

		case 'aiGenerated':
		case 'aiSummary':
		case 'aiTranslation':
			// AI blocks - preserve prompt, clear output
			if (typeof recovered.content.aiPrompt !== 'string') {
				recovered.content.aiPrompt = '';
			}
			break;

		case 'html':
			// HTML blocks - ensure html content is string
			if (typeof recovered.content.html !== 'string') {
				recovered.content.html = '';
			}
			break;
	}

	// Update metadata to reflect recovery
	recovered.metadata.updatedAt = new Date().toISOString();
	recovered.metadata.version = (recovered.metadata.version || 0) + 1;

	return recovered;
}

/**
 * Serialize an error for transport/storage
 */
export function serializeError(
	error: Error,
	block?: Block,
	type?: ErrorTypeValue,
	severity?: ErrorSeverityValue
): SerializedError {
	const errorType = type ?? classifyError(error);
	const errorSeverity = severity ?? determineErrorSeverity(error, errorType);

	const serialized: SerializedError = {
		name: error.name,
		message: error.message,
		stack: error.stack,
		type: errorType,
		severity: errorSeverity,
		timestamp: new Date().toISOString(),
		blockId: block?.id,
		blockType: block?.type,
		recoveryAttempted: false,
		isRecoverable: isRecoverable(error)
	};

	// Serialize cause chain
	if (error.cause instanceof Error) {
		serialized.cause = serializeError(error.cause);
	}

	return serialized;
}

/**
 * Check if an error is likely recoverable
 */
export function isRecoverable(error: Error): boolean {
	const type = classifyError(error);

	// Network errors are usually recoverable with retry
	if (type === ErrorType.NETWORK_ERROR) {
		return true;
	}

	// Validation errors are recoverable with correct input
	if (type === ErrorType.VALIDATION_ERROR) {
		return true;
	}

	// AI errors are often transient
	if (type === ErrorType.AI_ERROR) {
		return true;
	}

	// Render errors from missing data are recoverable
	if (type === ErrorType.RENDER_ERROR) {
		// Check for common recoverable patterns
		if (/undefined|null|cannot read|property/i.test(error.message)) {
			return true;
		}
	}

	// Check for specific unrecoverable patterns
	const unrecoverablePatterns = [
		/out of memory/i,
		/maximum call stack/i,
		/script error/i,
		/syntax error/i,
		/reference error/i
	];

	for (const pattern of unrecoverablePatterns) {
		if (pattern.test(error.message) || pattern.test(error.name)) {
			return false;
		}
	}

	// Default to potentially recoverable
	return true;
}

/**
 * Get user-friendly error message
 */
export function getErrorMessage(error: Error, blockType?: BlockType): string {
	const type = classifyError(error);

	// Try block-type specific message first
	if (blockType && BLOCK_TYPE_MESSAGES[blockType]) {
		const typeMessages = BLOCK_TYPE_MESSAGES[blockType];
		if (typeMessages && typeMessages[type]) {
			return typeMessages[type];
		}
	}

	// Fall back to generic message for error type
	return USER_FRIENDLY_MESSAGES[type] || USER_FRIENDLY_MESSAGES[ErrorType.UNKNOWN_ERROR];
}

// =============================================================================
// Error Tracking Store
// =============================================================================

/**
 * Internal error store
 */
const errorStore: Writable<ErrorTrackingState> = writable({
	errors: [],
	errorsByBlock: new Map(),
	totalCount: 0,
	unrecoveredCount: 0,
	hasCriticalErrors: false
});

/**
 * Add error to store
 */
function addErrorToStore(record: BlockErrorRecord): void {
	errorStore.update((state) => {
		const errors = [...state.errors, record];
		const errorsByBlock = new Map(state.errorsByBlock);

		const blockErrors = errorsByBlock.get(record.block.id) || [];
		errorsByBlock.set(record.block.id, [...blockErrors, record]);

		const unrecoveredCount = errors.filter((e) => !e.recovered).length;
		const hasCriticalErrors = errors.some(
			(e) => !e.recovered && e.serialized.severity === ErrorSeverity.CRITICAL
		);

		return {
			errors,
			errorsByBlock,
			totalCount: errors.length,
			unrecoveredCount,
			hasCriticalErrors
		};
	});
}

/**
 * Mark an error as recovered
 */
export function markErrorRecovered(errorId: string): void {
	errorStore.update((state) => {
		const errors = state.errors.map((e) => (e.id === errorId ? { ...e, recovered: true } : e));

		const unrecoveredCount = errors.filter((e) => !e.recovered).length;
		const hasCriticalErrors = errors.some(
			(e) => !e.recovered && e.serialized.severity === ErrorSeverity.CRITICAL
		);

		return {
			...state,
			errors,
			unrecoveredCount,
			hasCriticalErrors
		};
	});
}

/**
 * Clear errors for a specific block
 */
export function clearBlockErrors(blockId: string): void {
	errorStore.update((state) => {
		const errors = state.errors.filter((e) => e.block.id !== blockId);
		const errorsByBlock = new Map(state.errorsByBlock);
		errorsByBlock.delete(blockId);

		const unrecoveredCount = errors.filter((e) => !e.recovered).length;
		const hasCriticalErrors = errors.some(
			(e) => !e.recovered && e.serialized.severity === ErrorSeverity.CRITICAL
		);

		return {
			errors,
			errorsByBlock,
			totalCount: errors.length,
			unrecoveredCount,
			hasCriticalErrors
		};
	});
}

/**
 * Clear all errors
 */
export function clearAllErrors(): void {
	errorStore.set({
		errors: [],
		errorsByBlock: new Map(),
		totalCount: 0,
		unrecoveredCount: 0,
		hasCriticalErrors: false
	});
}

/**
 * Exported error tracking store for UI indicators
 */
export const errorTracking: Readable<ErrorTrackingState> = derived(errorStore, ($store) => $store);

/**
 * Get errors for a specific block
 */
export const getBlockErrors = (
	blockId: string
): Readable<{ errors: BlockErrorRecord[]; hasErrors: boolean; hasCritical: boolean }> => {
	return derived(errorStore, ($store) => {
		const errors = $store.errorsByBlock.get(blockId) || [];
		return {
			errors,
			hasErrors: errors.length > 0,
			hasCritical: errors.some(
				(e) => !e.recovered && e.serialized.severity === ErrorSeverity.CRITICAL
			)
		};
	});
};

// =============================================================================
// Backend Reporting
// =============================================================================

/**
 * Report error to backend
 */
async function reportErrorToBackend(record: BlockErrorRecord): Promise<void> {
	// Check if reporting is enabled
	const reportingEnabled =
		typeof window !== 'undefined' && (window as { __ERROR_REPORTING_ENABLED__?: boolean }).__ERROR_REPORTING_ENABLED__;

	if (!reportingEnabled) {
		return;
	}

	const payload = {
		errorId: record.id,
		error: record.serialized,
		context: record.context,
		blockType: record.block.type,
		blockId: record.block.id,
		timestamp: record.timestamp.toISOString()
	};

	try {
		const response = await fetch('/api/errors/report', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(payload)
		});

		if (!response.ok) {
			console.warn('[BlockError] Backend error reporting failed:', response.status);
		}
	} catch (e) {
		// Silently fail - don't cause more errors while reporting errors
		console.warn('[BlockError] Backend error reporting failed:', e);
	}
}

// =============================================================================
// Utility Exports
// =============================================================================

/**
 * Create a default block with minimum valid state
 */
export function createDefaultBlock(type: BlockType, id?: string): Block {
	const definition = BLOCK_DEFINITIONS[type];
	const now = new Date().toISOString();

	return {
		id: id || `block_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
		type,
		content: {
			...definition?.defaultContent
		} as BlockContent,
		settings: {
			...definition?.defaultSettings
		} as BlockSettings,
		metadata: {
			createdAt: now,
			updatedAt: now,
			version: 1
		}
	};
}

/**
 * Check if block state is valid
 */
export function isBlockStateValid(block: Block): boolean {
	if (!block || typeof block !== 'object') return false;
	if (!block.id || typeof block.id !== 'string') return false;
	if (!block.type || typeof block.type !== 'string') return false;
	if (!block.content || typeof block.content !== 'object') return false;
	if (!block.settings || typeof block.settings !== 'object') return false;
	if (!block.metadata || typeof block.metadata !== 'object') return false;

	return true;
}

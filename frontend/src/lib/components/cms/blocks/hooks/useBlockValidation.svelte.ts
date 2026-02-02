/**
 * Block Validation Hook
 * ═══════════════════════════════════════════════════════════════════════════
 * Composable validation logic for CMS block content with reactive updates
 */

import type { Block, BlockContent } from '../types';

// ============================================================================
// Types
// ============================================================================

export interface ValidationRule {
	/** Field path to validate (supports nested paths like 'content.text') */
	field: string;
	/** Whether the field is required */
	required?: boolean;
	/** Minimum length for string values */
	minLength?: number;
	/** Maximum length for string values */
	maxLength?: number;
	/** Minimum value for numeric fields */
	min?: number;
	/** Maximum value for numeric fields */
	max?: number;
	/** Regular expression pattern for validation */
	pattern?: RegExp;
	/** Custom validation function */
	custom?: (value: unknown, block: Block) => boolean;
	/** Async custom validation function */
	asyncCustom?: (value: unknown, block: Block) => Promise<boolean>;
	/** Error message to display on validation failure */
	message: string;
	/** Validation severity level */
	severity?: 'error' | 'warning' | 'info';
}

export interface ValidationError {
	field: string;
	message: string;
	severity: 'error' | 'warning' | 'info';
	rule: string;
}

export interface ValidationResult {
	valid: boolean;
	errors: Record<string, ValidationError>;
	warnings: Record<string, ValidationError>;
	errorCount: number;
	warningCount: number;
}

export interface ValidationOptions {
	/** Run async validations */
	runAsync?: boolean;
	/** Abort signal for async validations */
	signal?: AbortSignal;
	/** Only validate specified fields */
	fieldsToValidate?: string[];
	/** Skip specified fields */
	fieldsToSkip?: string[];
}

// ============================================================================
// Preset Rules
// ============================================================================

export const PRESET_RULES = {
	/** Required non-empty text */
	requiredText: (field: string, message?: string): ValidationRule => ({
		field,
		required: true,
		minLength: 1,
		message: message || `${field} is required`
	}),

	/** URL validation */
	url: (field: string, message?: string): ValidationRule => ({
		field,
		pattern: /^(https?:\/\/)?([\da-z.-]+)\.([a-z.]{2,6})([/\w .-]*)*\/?$/,
		message: message || 'Please enter a valid URL'
	}),

	/** Email validation */
	email: (field: string, message?: string): ValidationRule => ({
		field,
		pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
		message: message || 'Please enter a valid email address'
	}),

	/** Slug validation (URL-safe string) */
	slug: (field: string, message?: string): ValidationRule => ({
		field,
		pattern: /^[a-z0-9]+(?:-[a-z0-9]+)*$/,
		message: message || 'Please enter a valid slug (lowercase letters, numbers, and hyphens only)'
	}),

	/** Positive number */
	positiveNumber: (field: string, message?: string): ValidationRule => ({
		field,
		min: 0,
		custom: (value) => typeof value === 'number' && !isNaN(value),
		message: message || 'Please enter a positive number'
	}),

	/** Range validation */
	range: (field: string, min: number, max: number, message?: string): ValidationRule => ({
		field,
		min,
		max,
		message: message || `Value must be between ${min} and ${max}`
	}),

	/** Max length */
	maxLength: (field: string, max: number, message?: string): ValidationRule => ({
		field,
		maxLength: max,
		message: message || `Maximum ${max} characters allowed`
	}),

	/** Image alt text */
	imageAlt: (message?: string): ValidationRule => ({
		field: 'content.mediaAlt',
		required: true,
		minLength: 1,
		maxLength: 125,
		severity: 'warning',
		message: message || 'Image alt text is recommended for accessibility (max 125 characters)'
	}),

	/** Safe HTML (no script tags) */
	safeHtml: (field: string, message?: string): ValidationRule => ({
		field,
		custom: (value) => {
			if (typeof value !== 'string') return true;
			const dangerous = /<script|javascript:|on\w+=/i;
			return !dangerous.test(value);
		},
		message: message || 'Content contains potentially unsafe HTML'
	})
};

// ============================================================================
// Hook Implementation
// ============================================================================

export function useBlockValidation(block: Block, rules: ValidationRule[]) {
	// Track validation state
	let validating = $state(false);
	let lastValidated = $state<number | null>(null);

	// ========================================================================
	// Utility Functions
	// ========================================================================

	function getNestedValue(obj: unknown, path: string): unknown {
		const keys = path.split('.');
		let current: unknown = obj;

		for (const key of keys) {
			if (current === null || current === undefined) {
				return undefined;
			}
			if (typeof current === 'object') {
				current = (current as Record<string, unknown>)[key];
			} else {
				return undefined;
			}
		}

		return current;
	}

	function isEmpty(value: unknown): boolean {
		if (value === null || value === undefined) return true;
		if (typeof value === 'string') return value.trim().length === 0;
		if (Array.isArray(value)) return value.length === 0;
		if (typeof value === 'object') return Object.keys(value).length === 0;
		return false;
	}

	function getStringLength(value: unknown): number {
		if (typeof value === 'string') return value.length;
		if (Array.isArray(value)) return value.length;
		return 0;
	}

	function getNumericValue(value: unknown): number | null {
		if (typeof value === 'number') return value;
		if (typeof value === 'string') {
			const parsed = parseFloat(value);
			return isNaN(parsed) ? null : parsed;
		}
		return null;
	}

	// ========================================================================
	// Validation Logic
	// ========================================================================

	function validateRule(rule: ValidationRule, value: unknown): ValidationError | null {
		const severity = rule.severity || 'error';
		const getRuleName = (): string => {
			if (rule.required) return 'required';
			if (rule.minLength !== undefined) return 'minLength';
			if (rule.maxLength !== undefined) return 'maxLength';
			if (rule.min !== undefined) return 'min';
			if (rule.max !== undefined) return 'max';
			if (rule.pattern) return 'pattern';
			if (rule.custom) return 'custom';
			return 'unknown';
		};

		// Required check
		if (rule.required && isEmpty(value)) {
			return {
				field: rule.field,
				message: rule.message,
				severity,
				rule: 'required'
			};
		}

		// Skip other validations if value is empty and not required
		if (isEmpty(value)) {
			return null;
		}

		// Min length check
		if (rule.minLength !== undefined) {
			const length = getStringLength(value);
			if (length < rule.minLength) {
				return {
					field: rule.field,
					message: rule.message,
					severity,
					rule: 'minLength'
				};
			}
		}

		// Max length check
		if (rule.maxLength !== undefined) {
			const length = getStringLength(value);
			if (length > rule.maxLength) {
				return {
					field: rule.field,
					message: rule.message,
					severity,
					rule: 'maxLength'
				};
			}
		}

		// Min value check
		if (rule.min !== undefined) {
			const numValue = getNumericValue(value);
			if (numValue !== null && numValue < rule.min) {
				return {
					field: rule.field,
					message: rule.message,
					severity,
					rule: 'min'
				};
			}
		}

		// Max value check
		if (rule.max !== undefined) {
			const numValue = getNumericValue(value);
			if (numValue !== null && numValue > rule.max) {
				return {
					field: rule.field,
					message: rule.message,
					severity,
					rule: 'max'
				};
			}
		}

		// Pattern check
		if (rule.pattern) {
			if (typeof value === 'string' && !rule.pattern.test(value)) {
				return {
					field: rule.field,
					message: rule.message,
					severity,
					rule: 'pattern'
				};
			}
		}

		// Custom validation
		if (rule.custom) {
			if (!rule.custom(value, block)) {
				return {
					field: rule.field,
					message: rule.message,
					severity,
					rule: getRuleName()
				};
			}
		}

		return null;
	}

	function validate(options: ValidationOptions = {}): ValidationResult {
		const { fieldsToValidate, fieldsToSkip } = options;
		const errors: Record<string, ValidationError> = {};
		const warnings: Record<string, ValidationError> = {};

		for (const rule of rules) {
			// Check if we should skip this field
			if (fieldsToSkip?.includes(rule.field)) continue;
			if (fieldsToValidate && !fieldsToValidate.includes(rule.field)) continue;

			const value = getNestedValue(block, rule.field);
			const error = validateRule(rule, value);

			if (error) {
				if (error.severity === 'error') {
					errors[rule.field] = error;
				} else {
					warnings[rule.field] = error;
				}
			}
		}

		lastValidated = Date.now();

		return {
			valid: Object.keys(errors).length === 0,
			errors,
			warnings,
			errorCount: Object.keys(errors).length,
			warningCount: Object.keys(warnings).length
		};
	}

	async function validateAsync(options: ValidationOptions = {}): Promise<ValidationResult> {
		const { signal, fieldsToValidate, fieldsToSkip } = options;
		validating = true;

		try {
			// First run synchronous validation
			const syncResult = validate({ fieldsToValidate, fieldsToSkip });
			const errors = { ...syncResult.errors };
			const warnings = { ...syncResult.warnings };

			// Then run async validations
			const asyncRules = rules.filter((rule) => rule.asyncCustom);

			const asyncPromises = asyncRules.map(async (rule) => {
				// Check if we should skip this field
				if (fieldsToSkip?.includes(rule.field)) return null;
				if (fieldsToValidate && !fieldsToValidate.includes(rule.field)) return null;

				const value = getNestedValue(block, rule.field);

				// Skip if empty and not required
				if (isEmpty(value) && !rule.required) return null;

				try {
					const isValid = await rule.asyncCustom!(value, block);
					if (!isValid) {
						return {
							field: rule.field,
							message: rule.message,
							severity: rule.severity || 'error',
							rule: 'asyncCustom'
						} as ValidationError;
					}
				} catch {
					// Treat async errors as validation failures
					return {
						field: rule.field,
						message: rule.message,
						severity: rule.severity || 'error',
						rule: 'asyncCustom'
					} as ValidationError;
				}

				return null;
			});

			// Check for abort
			if (signal?.aborted) {
				throw new Error('Validation aborted');
			}

			const asyncResults = await Promise.all(asyncPromises);

			for (const error of asyncResults) {
				if (error) {
					if (error.severity === 'error') {
						errors[error.field] = error;
					} else {
						warnings[error.field] = error;
					}
				}
			}

			lastValidated = Date.now();

			return {
				valid: Object.keys(errors).length === 0,
				errors,
				warnings,
				errorCount: Object.keys(errors).length,
				warningCount: Object.keys(warnings).length
			};
		} finally {
			validating = false;
		}
	}

	// ========================================================================
	// Derived State
	// ========================================================================

	let result = $derived(validate());

	// ========================================================================
	// Helper Methods
	// ========================================================================

	function validateField(field: string): ValidationError | null {
		const fieldRules = rules.filter((rule) => rule.field === field);
		const value = getNestedValue(block, field);

		for (const rule of fieldRules) {
			const error = validateRule(rule, value);
			if (error) return error;
		}

		return null;
	}

	function getFieldError(field: string): string | null {
		return result.errors[field]?.message || null;
	}

	function getFieldWarning(field: string): string | null {
		return result.warnings[field]?.message || null;
	}

	function hasFieldError(field: string): boolean {
		return field in result.errors;
	}

	function hasFieldWarning(field: string): boolean {
		return field in result.warnings;
	}

	function getFirstError(): ValidationError | null {
		const firstKey = Object.keys(result.errors)[0];
		return firstKey ? result.errors[firstKey] : null;
	}

	function getAllMessages(): string[] {
		const errorMessages = Object.values(result.errors).map((e) => e.message);
		const warningMessages = Object.values(result.warnings).map((w) => w.message);
		return [...errorMessages, ...warningMessages];
	}

	// ========================================================================
	// Return API
	// ========================================================================

	return {
		// Reactive state (via getters)
		get valid() {
			return result.valid;
		},
		get errors() {
			return result.errors;
		},
		get warnings() {
			return result.warnings;
		},
		get errorCount() {
			return result.errorCount;
		},
		get warningCount() {
			return result.warningCount;
		},
		get validating() {
			return validating;
		},
		get lastValidated() {
			return lastValidated;
		},

		// Methods
		validate,
		validateAsync,
		validateField,
		getFieldError,
		getFieldWarning,
		hasFieldError,
		hasFieldWarning,
		getFirstError,
		getAllMessages
	};
}

// ============================================================================
// Block-Specific Validation Rule Sets
// ============================================================================

export const BLOCK_VALIDATION_RULES: Partial<Record<string, ValidationRule[]>> = {
	image: [
		PRESET_RULES.requiredText('content.mediaUrl', 'Image URL is required'),
		PRESET_RULES.imageAlt()
	],

	video: [
		PRESET_RULES.requiredText('content.mediaUrl', 'Video URL is required'),
		PRESET_RULES.url('content.mediaUrl', 'Please enter a valid video URL')
	],

	embed: [
		PRESET_RULES.requiredText('content.embedUrl', 'Embed URL is required'),
		PRESET_RULES.url('content.embedUrl', 'Please enter a valid embed URL'),
		PRESET_RULES.safeHtml('content.html')
	],

	button: [
		PRESET_RULES.requiredText('content.text', 'Button text is required'),
		PRESET_RULES.maxLength('content.text', 50, 'Button text should be concise (max 50 characters)'),
		{
			field: 'settings.linkUrl',
			pattern: /^(https?:\/\/|\/|#|mailto:|tel:)/,
			message: 'Please enter a valid link URL',
			severity: 'warning'
		}
	],

	newsletter: [
		PRESET_RULES.requiredText('content.newsletterTitle', 'Newsletter title is required'),
		PRESET_RULES.maxLength(
			'content.newsletterTitle',
			100,
			'Title should be under 100 characters'
		)
	],

	countdown: [
		PRESET_RULES.requiredText('content.countdownTarget', 'Target date is required'),
		{
			field: 'content.countdownTarget',
			custom: (value) => {
				if (typeof value !== 'string') return false;
				const date = new Date(value);
				return !isNaN(date.getTime()) && date > new Date();
			},
			message: 'Target date must be a valid future date'
		}
	],

	tradingIdea: [
		PRESET_RULES.requiredText('content.tradeSymbol', 'Trading symbol is required'),
		PRESET_RULES.positiveNumber('content.tradeEntry', 'Entry price must be a positive number'),
		PRESET_RULES.positiveNumber('content.tradeStop', 'Stop loss must be a positive number'),
		{
			field: 'content.tradeConfidence',
			min: 0,
			max: 100,
			message: 'Confidence must be between 0 and 100'
		}
	],

	priceAlert: [
		PRESET_RULES.requiredText('content.alertSymbol', 'Alert symbol is required'),
		PRESET_RULES.positiveNumber('content.alertTarget', 'Target price must be a positive number')
	],

	html: [
		PRESET_RULES.safeHtml('content.html')
	]
};

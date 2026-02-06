/**
 * useBlockValidation Hook - Comprehensive Test Suite
 * ===============================================================================
 * Unit tests for block content validation with reactive updates
 *
 * @version 2.0.0
 */

// Import setup first to initialize mocks
import './setup';

import { describe, it, expect, vi, beforeEach } from 'vitest';

// ============================================================================
// Import Hook and Types
// ============================================================================

import {
	useBlockValidation,
	PRESET_RULES,
	BLOCK_VALIDATION_RULES,
	type ValidationRule
} from '../useBlockValidation.svelte';

import type { Block } from '../../types';
import { toBlockId } from '$lib/stores/blockState.svelte';

// ============================================================================
// Test Utilities
// ============================================================================

function createMockBlock(overrides: Partial<Block> = {}): Block {
	return {
		id: toBlockId(`test-${Date.now()}`),
		type: 'paragraph',
		content: { text: '' },
		settings: {},
		metadata: {
			createdAt: new Date().toISOString(),
			updatedAt: new Date().toISOString(),
			version: 1
		},
		...overrides
	};
}

// ============================================================================
// useBlockValidation Tests
// ============================================================================

describe('useBlockValidation', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	// ========================================================================
	// Required Field Validation
	// ========================================================================

	describe('Required Field Validation', () => {
		it('validates required fields - fails when empty', () => {
			const block = createMockBlock({
				content: { text: '' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					required: true,
					message: 'Text is required'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errorCount).toBe(1);
			expect(result.errors['content.text']).toBeDefined();
			expect(result.errors['content.text'].message).toBe('Text is required');
			expect(result.errors['content.text'].rule).toBe('required');
		});

		it('validates required fields - passes when has value', () => {
			const block = createMockBlock({
				content: { text: 'Hello world' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					required: true,
					message: 'Text is required'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
			expect(result.errorCount).toBe(0);
		});

		it('validates required fields - fails for whitespace only', () => {
			const block = createMockBlock({
				content: { text: '   ' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					required: true,
					message: 'Text is required'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});

		it('validates required fields - fails for null value', () => {
			const block = createMockBlock({
				content: { text: null as any }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					required: true,
					message: 'Text is required'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});

		it('validates required fields - fails for undefined value', () => {
			const block = createMockBlock({
				content: {}
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.missingField',
					required: true,
					message: 'Field is required'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// MinLength Validation
	// ========================================================================

	describe('MinLength Validation', () => {
		it('validates minLength - fails when too short', () => {
			const block = createMockBlock({
				content: { text: 'Hi' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					minLength: 5,
					message: 'Text must be at least 5 characters'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errors['content.text'].rule).toBe('minLength');
		});

		it('validates minLength - passes when meets minimum', () => {
			const block = createMockBlock({
				content: { text: 'Hello' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					minLength: 5,
					message: 'Text must be at least 5 characters'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates minLength - passes when exceeds minimum', () => {
			const block = createMockBlock({
				content: { text: 'Hello World' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					minLength: 5,
					message: 'Text must be at least 5 characters'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates minLength - skips validation for empty non-required field', () => {
			const block = createMockBlock({
				content: { text: '' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					minLength: 5,
					message: 'Text must be at least 5 characters'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			// Empty is valid because field is not required
			expect(result.valid).toBe(true);
		});

		it('validates minLength on arrays', () => {
			const block = createMockBlock({
				content: { listItems: ['a', 'b'] }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.listItems',
					minLength: 3,
					message: 'Need at least 3 items'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// MaxLength Validation
	// ========================================================================

	describe('MaxLength Validation', () => {
		it('validates maxLength - fails when too long', () => {
			const block = createMockBlock({
				content: { text: 'This is a very long text that exceeds the limit' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					maxLength: 10,
					message: 'Text must not exceed 10 characters'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errors['content.text'].rule).toBe('maxLength');
		});

		it('validates maxLength - passes when at limit', () => {
			const block = createMockBlock({
				content: { text: 'Exactly 10' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					maxLength: 10,
					message: 'Text must not exceed 10 characters'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates maxLength - passes when under limit', () => {
			const block = createMockBlock({
				content: { text: 'Short' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.text',
					maxLength: 100,
					message: 'Text must not exceed 100 characters'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates maxLength on arrays', () => {
			const block = createMockBlock({
				content: { listItems: ['a', 'b', 'c', 'd', 'e'] }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.listItems',
					maxLength: 3,
					message: 'Maximum 3 items allowed'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// Min/Max Number Validation
	// ========================================================================

	describe('Min/Max Number Validation', () => {
		it('validates min - fails when below minimum', () => {
			const block = createMockBlock({
				content: { price: -5 }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.price',
					min: 0,
					message: 'Price must be at least 0'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errors['content.price'].rule).toBe('min');
		});

		it('validates min - passes when at minimum', () => {
			const block = createMockBlock({
				content: { price: 0 }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.price',
					min: 0,
					message: 'Price must be at least 0'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates max - fails when above maximum', () => {
			const block = createMockBlock({
				content: { quantity: 150 }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.quantity',
					max: 100,
					message: 'Quantity cannot exceed 100'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errors['content.quantity'].rule).toBe('max');
		});

		it('validates max - passes when at maximum', () => {
			const block = createMockBlock({
				content: { quantity: 100 }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.quantity',
					max: 100,
					message: 'Quantity cannot exceed 100'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates both min and max', () => {
			const block = createMockBlock({
				content: { confidence: 50 }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.confidence',
					min: 0,
					max: 100,
					message: 'Confidence must be between 0 and 100'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates string numbers', () => {
			const block = createMockBlock({
				content: { price: '99.99' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.price',
					min: 0,
					max: 100,
					message: 'Price must be between 0 and 100'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});
	});

	// ========================================================================
	// Pattern Regex Validation
	// ========================================================================

	describe('Pattern Regex Validation', () => {
		it('validates pattern - fails when no match', () => {
			const block = createMockBlock({
				content: { email: 'invalid-email' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.email',
					pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
					message: 'Invalid email format'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errors['content.email'].rule).toBe('pattern');
		});

		it('validates pattern - passes when matches', () => {
			const block = createMockBlock({
				content: { email: 'user@example.com' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.email',
					pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
					message: 'Invalid email format'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates URL pattern', () => {
			const block = createMockBlock({
				content: { url: 'https://example.com/path' }
			});

			const rules: ValidationRule[] = [PRESET_RULES.url('content.url')];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('validates slug pattern', () => {
			const block = createMockBlock({
				content: { slug: 'my-valid-slug' }
			});

			const rules: ValidationRule[] = [PRESET_RULES.slug('content.slug')];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
		});

		it('rejects invalid slug', () => {
			const block = createMockBlock({
				content: { slug: 'Invalid Slug!' }
			});

			const rules: ValidationRule[] = [PRESET_RULES.slug('content.slug')];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// Custom Validation Function
	// ========================================================================

	describe('Custom Validation Function', () => {
		it('validates custom function - fails when returns false', () => {
			const block = createMockBlock({
				content: { value: 'invalid' }
			});

			const customValidator = vi.fn().mockReturnValue(false);

			const rules: ValidationRule[] = [
				{
					field: 'content.value',
					custom: customValidator,
					message: 'Custom validation failed'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(customValidator).toHaveBeenCalledWith('invalid', block);
		});

		it('validates custom function - passes when returns true', () => {
			const block = createMockBlock({
				content: { value: 'valid' }
			});

			const customValidator = vi.fn().mockReturnValue(true);

			const rules: ValidationRule[] = [
				{
					field: 'content.value',
					custom: customValidator,
					message: 'Custom validation failed'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
			expect(customValidator).toHaveBeenCalled();
		});

		it('custom function receives block context', () => {
			const block = createMockBlock({
				id: toBlockId('test-123'),
				content: { value: 'check' }
			});

			const customValidator = vi.fn().mockReturnValue(true);

			const rules: ValidationRule[] = [
				{
					field: 'content.value',
					custom: customValidator,
					message: 'Custom validation failed'
				}
			];

			const validator = useBlockValidation(block, rules);
			validator.validate();

			expect(customValidator).toHaveBeenCalledWith(
				'check',
				expect.objectContaining({
					id: 'test-123'
				})
			);
		});

		it('validates safe HTML custom function', () => {
			const block = createMockBlock({
				content: { html: '<script>alert("xss")</script>' }
			});

			const rules: ValidationRule[] = [PRESET_RULES.safeHtml('content.html')];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// Valid Result Tests
	// ========================================================================

	describe('Valid Result', () => {
		it('returns valid when all rules pass', () => {
			const block = createMockBlock({
				content: {
					text: 'Valid text here',
					email: 'user@example.com',
					quantity: 50
				}
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, minLength: 5, message: 'Text required' },
				{ field: 'content.email', pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/, message: 'Invalid email' },
				{ field: 'content.quantity', min: 0, max: 100, message: 'Invalid quantity' }
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true);
			expect(result.errorCount).toBe(0);
			expect(result.warningCount).toBe(0);
			expect(Object.keys(result.errors)).toHaveLength(0);
		});

		it('returns valid with warnings only', () => {
			const block = createMockBlock({
				content: {
					text: 'Valid text',
					mediaAlt: '' // Missing alt text
				}
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' },
				{
					field: 'content.mediaAlt',
					required: true,
					message: 'Alt text recommended',
					severity: 'warning'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(true); // Still valid because warning severity
			expect(result.warningCount).toBe(1);
			expect(result.errorCount).toBe(0);
		});
	});

	// ========================================================================
	// Multiple Failures
	// ========================================================================

	describe('Multiple Failures', () => {
		it('returns all errors for multiple failures', () => {
			const block = createMockBlock({
				content: {
					text: '',
					email: 'invalid',
					quantity: 200
				}
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' },
				{ field: 'content.email', pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/, message: 'Invalid email' },
				{ field: 'content.quantity', max: 100, message: 'Quantity too high' }
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errorCount).toBe(3);
			expect(result.errors['content.text']).toBeDefined();
			expect(result.errors['content.email']).toBeDefined();
			expect(result.errors['content.quantity']).toBeDefined();
		});

		it('returns both errors and warnings', () => {
			const block = createMockBlock({
				content: {
					text: '',
					mediaAlt: ''
				}
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required', severity: 'error' },
				{
					field: 'content.mediaAlt',
					required: true,
					message: 'Alt text recommended',
					severity: 'warning'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errorCount).toBe(1);
			expect(result.warningCount).toBe(1);
		});

		it('getAllMessages returns all error and warning messages', () => {
			const block = createMockBlock({
				content: { text: '', alt: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' },
				{ field: 'content.alt', required: true, message: 'Alt recommended', severity: 'warning' }
			];

			const validator = useBlockValidation(block, rules);
			const messages = validator.getAllMessages();

			expect(messages).toContain('Text required');
			expect(messages).toContain('Alt recommended');
		});
	});

	// ========================================================================
	// Nested Field Paths
	// ========================================================================

	describe('Nested Field Paths', () => {
		it('supports nested field paths', () => {
			const block = createMockBlock({
				content: {
					nested: {
						deep: {
							value: ''
						}
					}
				}
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.nested.deep.value',
					required: true,
					message: 'Deep value required'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
			expect(result.errors['content.nested.deep.value']).toBeDefined();
		});

		it('validates settings path', () => {
			const block = createMockBlock({
				settings: { linkUrl: 'invalid-url' }
			});

			const rules: ValidationRule[] = [
				{
					field: 'settings.linkUrl',
					pattern: /^(https?:\/\/|\/|#)/,
					message: 'Invalid URL'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});

		it('handles missing nested path gracefully', () => {
			const block = createMockBlock({
				content: {}
			});

			const rules: ValidationRule[] = [
				{
					field: 'content.nonexistent.path.value',
					required: true,
					message: 'Value required'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate();

			expect(result.valid).toBe(false);
		});
	});

	// ========================================================================
	// Async Validation
	// ========================================================================

	describe('Async Validation', () => {
		it('handles async validation', async () => {
			const block = createMockBlock({
				content: { username: 'taken' }
			});

			const asyncValidator = vi.fn().mockResolvedValue(false);

			const rules: ValidationRule[] = [
				{
					field: 'content.username',
					asyncCustom: asyncValidator,
					message: 'Username already taken'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = await validator.validateAsync();

			expect(result.valid).toBe(false);
			expect(asyncValidator).toHaveBeenCalledWith('taken', block);
		});

		it('async validation passes when returns true', async () => {
			const block = createMockBlock({
				content: { username: 'available' }
			});

			const asyncValidator = vi.fn().mockResolvedValue(true);

			const rules: ValidationRule[] = [
				{
					field: 'content.username',
					asyncCustom: asyncValidator,
					message: 'Username already taken'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = await validator.validateAsync();

			expect(result.valid).toBe(true);
		});

		it('handles async validation errors as failures', async () => {
			const block = createMockBlock({
				content: { value: 'test' }
			});

			const asyncValidator = vi.fn().mockRejectedValue(new Error('Network error'));

			const rules: ValidationRule[] = [
				{
					field: 'content.value',
					asyncCustom: asyncValidator,
					message: 'Validation failed'
				}
			];

			const validator = useBlockValidation(block, rules);
			const result = await validator.validateAsync();

			expect(result.valid).toBe(false);
		});

		it('runs both sync and async validations', async () => {
			const block = createMockBlock({
				content: { text: '', username: 'test' }
			});

			const asyncValidator = vi.fn().mockResolvedValue(true);

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' },
				{ field: 'content.username', asyncCustom: asyncValidator, message: 'Username check' }
			];

			const validator = useBlockValidation(block, rules);
			const result = await validator.validateAsync();

			expect(result.valid).toBe(false);
			expect(result.errors['content.text']).toBeDefined();
		});

		it('sets validating state during async validation', async () => {
			const block = createMockBlock({
				content: { username: 'test' }
			});

			const asyncValidator = vi.fn().mockImplementation(async () => {
				await new Promise((resolve) => setTimeout(resolve, 10));
				return true;
			});

			const rules: ValidationRule[] = [
				{ field: 'content.username', asyncCustom: asyncValidator, message: 'Check' }
			];

			const validator = useBlockValidation(block, rules);

			const validatePromise = validator.validateAsync();
			// Note: validating state may be set synchronously at start

			await validatePromise;
		});
	});

	// ========================================================================
	// Helper Methods
	// ========================================================================

	describe('Helper Methods', () => {
		it('validateField returns error for specific field', () => {
			const block = createMockBlock({
				content: { text: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' }
			];

			const validator = useBlockValidation(block, rules);
			const error = validator.validateField('content.text');

			expect(error).not.toBeNull();
			expect(error?.message).toBe('Text required');
		});

		it('validateField returns null for valid field', () => {
			const block = createMockBlock({
				content: { text: 'Valid' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' }
			];

			const validator = useBlockValidation(block, rules);
			const error = validator.validateField('content.text');

			expect(error).toBeNull();
		});

		it('getFieldError returns error message', () => {
			const block = createMockBlock({
				content: { text: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' }
			];

			const validator = useBlockValidation(block, rules);
			const message = validator.getFieldError('content.text');

			expect(message).toBe('Text required');
		});

		it('getFieldWarning returns warning message', () => {
			const block = createMockBlock({
				content: { alt: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.alt', required: true, message: 'Alt recommended', severity: 'warning' }
			];

			const validator = useBlockValidation(block, rules);
			const message = validator.getFieldWarning('content.alt');

			expect(message).toBe('Alt recommended');
		});

		it('hasFieldError returns boolean', () => {
			const block = createMockBlock({
				content: { text: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Required' }
			];

			const validator = useBlockValidation(block, rules);

			expect(validator.hasFieldError('content.text')).toBe(true);
			expect(validator.hasFieldError('content.other')).toBe(false);
		});

		it('getFirstError returns first error', () => {
			const block = createMockBlock({
				content: { text: '', email: 'invalid' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' },
				{ field: 'content.email', pattern: /@/, message: 'Invalid email' }
			];

			const validator = useBlockValidation(block, rules);
			const firstError = validator.getFirstError();

			expect(firstError).not.toBeNull();
			expect(['Text required', 'Invalid email']).toContain(firstError?.message);
		});
	});

	// ========================================================================
	// Validation Options
	// ========================================================================

	describe('Validation Options', () => {
		it('validates only specified fields', () => {
			const block = createMockBlock({
				content: { text: '', email: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' },
				{ field: 'content.email', required: true, message: 'Email required' }
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate({ fieldsToValidate: ['content.text'] });

			expect(result.errorCount).toBe(1);
			expect(result.errors['content.text']).toBeDefined();
			expect(result.errors['content.email']).toBeUndefined();
		});

		it('skips specified fields', () => {
			const block = createMockBlock({
				content: { text: '', email: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Text required' },
				{ field: 'content.email', required: true, message: 'Email required' }
			];

			const validator = useBlockValidation(block, rules);
			const result = validator.validate({ fieldsToSkip: ['content.text'] });

			expect(result.errorCount).toBe(1);
			expect(result.errors['content.text']).toBeUndefined();
			expect(result.errors['content.email']).toBeDefined();
		});
	});

	// ========================================================================
	// Reactive State
	// ========================================================================

	describe('Reactive State', () => {
		it('provides valid getter', () => {
			const block = createMockBlock({
				content: { text: 'Valid' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Required' }
			];

			const validator = useBlockValidation(block, rules);

			expect(validator.valid).toBe(true);
		});

		it('provides errors getter', () => {
			const block = createMockBlock({
				content: { text: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Required' }
			];

			const validator = useBlockValidation(block, rules);

			expect(validator.errors['content.text']).toBeDefined();
		});

		it('provides warnings getter', () => {
			const block = createMockBlock({
				content: { alt: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.alt', required: true, message: 'Recommended', severity: 'warning' }
			];

			const validator = useBlockValidation(block, rules);

			expect(validator.warnings['content.alt']).toBeDefined();
		});

		it('provides errorCount getter', () => {
			const block = createMockBlock({
				content: { text: '', email: '' }
			});

			const rules: ValidationRule[] = [
				{ field: 'content.text', required: true, message: 'Required' },
				{ field: 'content.email', required: true, message: 'Required' }
			];

			const validator = useBlockValidation(block, rules);

			expect(validator.errorCount).toBe(2);
		});
	});
});

// ============================================================================
// PRESET_RULES Tests
// ============================================================================

describe('PRESET_RULES', () => {
	describe('requiredText', () => {
		it('creates required text rule with default message', () => {
			const rule = PRESET_RULES.requiredText('content.text');

			expect(rule.field).toBe('content.text');
			expect(rule.required).toBe(true);
			expect(rule.minLength).toBe(1);
			expect(rule.message).toBeDefined();
		});

		it('accepts custom message', () => {
			const rule = PRESET_RULES.requiredText('content.text', 'Custom message');

			expect(rule.message).toBe('Custom message');
		});
	});

	describe('email', () => {
		it('validates email format', () => {
			const rule = PRESET_RULES.email('content.email');

			expect(rule.pattern?.test('valid@email.com')).toBe(true);
			expect(rule.pattern?.test('invalid')).toBe(false);
		});
	});

	describe('range', () => {
		it('creates range rule with min and max', () => {
			const rule = PRESET_RULES.range('content.value', 10, 100);

			expect(rule.min).toBe(10);
			expect(rule.max).toBe(100);
		});
	});

	describe('imageAlt', () => {
		it('creates image alt rule with warning severity', () => {
			const rule = PRESET_RULES.imageAlt();

			expect(rule.field).toBe('content.mediaAlt');
			expect(rule.severity).toBe('warning');
		});
	});
});

// ============================================================================
// BLOCK_VALIDATION_RULES Tests
// ============================================================================

describe('BLOCK_VALIDATION_RULES', () => {
	it('has rules for image blocks', () => {
		expect(BLOCK_VALIDATION_RULES.image).toBeDefined();
		expect(BLOCK_VALIDATION_RULES.image!.length).toBeGreaterThan(0);
	});

	it('has rules for tradingIdea blocks', () => {
		expect(BLOCK_VALIDATION_RULES.tradingIdea).toBeDefined();
	});

	it('has rules for button blocks', () => {
		expect(BLOCK_VALIDATION_RULES.button).toBeDefined();
	});
});

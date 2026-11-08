import { afterEach, describe, expect, it, vi } from 'vite-plus/test';
import { validateField, type FormField } from './forms';

const field = {
	field_type: 'number',
	label: 'Quantity',
	name: 'quantity',
	required: true,
	order: 0,
	width: 12
} satisfies FormField;

describe('form field validation', () => {
	afterEach(() => vi.unstubAllGlobals());

	it('accepts zero and false as present answers while rejecting blanks', async () => {
		expect(await validateField(field, 0)).toEqual({ valid: true, errors: [] });
		expect(await validateField(field, false)).toEqual({ valid: true, errors: [] });
		for (const value of [null, '', '   ', []]) {
			expect(await validateField(field, value)).toEqual({
				valid: false,
				errors: ['Quantity is required']
			});
		}
	});

	it('rejects a negative async validation receipt without a message', async () => {
		vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ valid: false })));
		expect(await validateField({ ...field, validation: { async: true } }, 3)).toEqual({
			valid: false,
			errors: ['Quantity could not be validated']
		});
	});

	it('fails closed when the async validator is unavailable', async () => {
		vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new TypeError('Network unavailable')));
		expect(await validateField({ ...field, validation: { async: true } }, 3)).toEqual({
			valid: false,
			errors: ['Quantity could not be validated']
		});
	});

	it('rejects a malformed async validation receipt', async () => {
		vi.stubGlobal('fetch', vi.fn().mockResolvedValue(Response.json({ valid: 'true' })));
		expect(await validateField({ ...field, validation: { async: true } }, 3)).toEqual({
			valid: false,
			errors: ['Quantity could not be validated']
		});
	});

	it('reports an invalid configured pattern instead of throwing', async () => {
		expect(await validateField({ ...field, validation: { pattern: '[' } }, 'abc')).toEqual({
			valid: false,
			errors: ['Quantity validation pattern is invalid']
		});
	});
});

import { expect, it } from 'vite-plus/test';
import { mergeHeaders } from '../http-headers';

it('preserves authenticated and custom headers for records, tuples and Headers instances', () => {
	for (const custom of [
		{ 'X-Request-ID': 'fixture', accept: 'text/csv' },
		[
			['X-Request-ID', 'fixture'],
			['accept', 'text/csv']
		],
		new Headers({ 'X-Request-ID': 'fixture', accept: 'text/csv' })
	] satisfies HeadersInit[]) {
		const request = new Request('https://fixture.example.test/export', {
			headers: mergeHeaders(
				{ Authorization: 'Bearer fixture-only', Accept: 'application/json' },
				custom
			)
		});
		expect(request.headers.get('authorization')).toBe('Bearer fixture-only');
		expect(request.headers.get('x-request-id')).toBe('fixture');
		expect(request.headers.get('accept')).toBe('text/csv');
		expect(request.headers.has('0')).toBe(false);
	}
});

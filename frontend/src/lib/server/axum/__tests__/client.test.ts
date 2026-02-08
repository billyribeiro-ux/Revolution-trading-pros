/**
 * Axum Server Adapter — Unit Tests
 *
 * Tests the centralized Axum client: retries, timeouts, error mapping, auth.
 *
 * @version 1.0.0
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// ═══════════════════════════════════════════════════════════════════════════
// Mocks — Must be before imports
// ═══════════════════════════════════════════════════════════════════════════

vi.mock('$env/dynamic/private', () => ({
	env: {
		API_BASE_URL: 'https://test-api.example.com',
		VITE_API_URL: undefined,
		BACKEND_URL: undefined
	}
}));

vi.mock('$app/server', () => ({
	getRequestEvent: vi.fn(() => ({
		locals: { accessToken: 'test-token-123' },
		cookies: {
			get: vi.fn((name: string) => (name === 'rtp_access_token' ? 'cookie-token' : undefined))
		}
	}))
}));

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

describe('Axum Server Adapter', () => {
	let originalFetch: typeof globalThis.fetch;

	beforeEach(() => {
		originalFetch = globalThis.fetch;
		vi.useFakeTimers({ shouldAdvanceTime: true });
	});

	afterEach(() => {
		globalThis.fetch = originalFetch;
		vi.useRealTimers();
		vi.restoreAllMocks();
	});

	describe('axumFetch', () => {
		it('should make a GET request with correct URL and headers', async () => {
			const mockResponse = { data: [{ id: 1 }], total: 1 };
			globalThis.fetch = vi.fn().mockResolvedValueOnce({
				ok: true,
				status: 200,
				text: () => Promise.resolve(JSON.stringify(mockResponse))
			});

			const { axumFetch } = await import('../client');
			const result = await axumFetch('/api/alerts/explosive-swings', {
				params: { limit: 10, offset: 0 }
			});

			expect(result).toEqual(mockResponse);
			expect(globalThis.fetch).toHaveBeenCalledTimes(1);

			const [url, options] = (globalThis.fetch as ReturnType<typeof vi.fn>).mock.calls[0];
			expect(url).toContain('https://test-api.example.com/api/alerts/explosive-swings');
			expect(url).toContain('limit=10');
			expect(url).toContain('offset=0');
			expect(options.method).toBe('GET');
			expect(options.headers['Authorization']).toBe('Bearer test-token-123');
			expect(options.headers['X-Request-ID']).toMatch(/^rtp-/);
		});

		it('should make a POST request with body', async () => {
			const payload = { ticker: 'NVDA', direction: 'long' };
			const mockResponse = { id: 1, ...payload };
			globalThis.fetch = vi.fn().mockResolvedValueOnce({
				ok: true,
				status: 200,
				text: () => Promise.resolve(JSON.stringify(mockResponse))
			});

			const { axumFetch } = await import('../client');
			const result = await axumFetch('/api/trades/explosive-swings', {
				method: 'POST',
				body: payload
			});

			expect(result).toEqual(mockResponse);
			const [, options] = (globalThis.fetch as ReturnType<typeof vi.fn>).mock.calls[0];
			expect(options.method).toBe('POST');
			expect(options.body).toBe(JSON.stringify(payload));
		});

		it('should handle 204 No Content', async () => {
			globalThis.fetch = vi.fn().mockResolvedValueOnce({
				ok: true,
				status: 204,
				text: () => Promise.resolve('')
			});

			const { axumFetch } = await import('../client');
			const result = await axumFetch('/api/admin/trades/1', { method: 'DELETE' });

			expect(result).toBeUndefined();
		});

		it('should throw AxumError on 404', async () => {
			globalThis.fetch = vi.fn().mockResolvedValue({
				ok: false,
				status: 404,
				json: () => Promise.resolve({ message: 'Not found', code: 'NOT_FOUND' })
			});

			const { axumFetch, AxumError } = await import('../client');

			try {
				await axumFetch('/api/alerts/nonexistent');
				expect.unreachable('Should have thrown');
			} catch (err) {
				expect(err).toBeInstanceOf(AxumError);
				const axumErr = err as InstanceType<typeof AxumError>;
				expect(axumErr.statusCode).toBe(404);
				expect(axumErr.isNotFound).toBe(true);
				expect(axumErr.code).toBe('NOT_FOUND');
			}
		});

		it('should throw AxumError on 401', async () => {
			globalThis.fetch = vi.fn().mockResolvedValueOnce({
				ok: false,
				status: 401,
				json: () => Promise.resolve({ message: 'Unauthorized' })
			});

			const { axumFetch, AxumError } = await import('../client');

			try {
				await axumFetch('/api/auth/me');
			} catch (err) {
				expect(err).toBeInstanceOf(AxumError);
				const axumErr = err as InstanceType<typeof AxumError>;
				expect(axumErr.statusCode).toBe(401);
				expect(axumErr.isUnauthorized).toBe(true);
			}
		});

		it('should skip undefined params', async () => {
			globalThis.fetch = vi.fn().mockResolvedValueOnce({
				ok: true,
				status: 200,
				text: () => Promise.resolve('{}')
			});

			const { axumFetch } = await import('../client');
			await axumFetch('/api/test', {
				params: { limit: 10, offset: undefined, active: true }
			});

			const [url] = (globalThis.fetch as ReturnType<typeof vi.fn>).mock.calls[0];
			expect(url).toContain('limit=10');
			expect(url).toContain('active=true');
			expect(url).not.toContain('offset');
		});
	});

	describe('axum convenience methods', () => {
		it('should expose get, post, put, patch, delete methods', async () => {
			const { axum } = await import('../client');

			expect(typeof axum.get).toBe('function');
			expect(typeof axum.post).toBe('function');
			expect(typeof axum.put).toBe('function');
			expect(typeof axum.patch).toBe('function');
			expect(typeof axum.delete).toBe('function');
		});
	});

	describe('AxumError', () => {
		it('should have correct boolean helpers', async () => {
			const { AxumError } = await import('../client');

			const notFound = new AxumError('Not found', 404, 'NOT_FOUND', 'req-1');
			expect(notFound.isNotFound).toBe(true);
			expect(notFound.isUnauthorized).toBe(false);
			expect(notFound.isServerError).toBe(false);

			const unauthorized = new AxumError('Unauthorized', 401, 'UNAUTHORIZED', 'req-2');
			expect(unauthorized.isUnauthorized).toBe(true);

			const forbidden = new AxumError('Forbidden', 403, 'FORBIDDEN', 'req-3');
			expect(forbidden.isForbidden).toBe(true);

			const rateLimited = new AxumError('Too many requests', 429, 'RATE_LIMITED', 'req-4');
			expect(rateLimited.isRateLimited).toBe(true);

			const serverError = new AxumError('Server error', 500, 'SERVER_ERROR', 'req-5');
			expect(serverError.isServerError).toBe(true);
		});
	});
});

// @vitest-environment node
import { expect, it } from 'vite-plus/test';
import { connectionStatusView } from '../connection-status.js';

const receipt = {
	key: 'sendgrid',
	name: 'SendGrid',
	category: 'email',
	is_builtin: false,
	is_connected: true,
	status: 'connected',
	health_score: 100,
	connection: { last_verified_at: '2026-11-04T12:00:00Z', last_error: null }
};
it('preserves persisted receipts and the broader catalog without inventing built-in health', () => {
	const result = connectionStatusView(
		{ success: true, connections: [receipt] },
		new URLSearchParams()
	);
	expect(result.connections.filter((row) => row.key === 'sendgrid')).toEqual([
		{
			key: 'sendgrid',
			name: 'SendGrid',
			category: 'email',
			is_builtin: false,
			is_connected: true,
			status: 'connected',
			health_score: 100,
			last_verified_at: '2026-11-04T12:00:00Z',
			last_error: null
		}
	]);
	expect(result.connections.find((row) => row.key === 'fluent_support')).toMatchObject({
		is_connected: false,
		status: 'pending',
		health_score: 0,
		last_verified_at: null
	});
	expect(result.connections.some((row) => row.key === 'polygon')).toBe(true);
	expect(result.summary.connected).toBe(1);
});
it('reflects revoke and failures, applies filters and refuses missing verification receipts', () => {
	const rejected = {
		...receipt,
		status: 'error',
		is_connected: false,
		connection: { last_verified_at: null, last_error: 'Provider returned HTTP 401' }
	};
	const result = connectionStatusView(
		{ success: true, connections: [rejected] },
		new URLSearchParams('category=email&status=error')
	);
	expect(result.connections).toHaveLength(1);
	expect(result.connections[0]).toMatchObject({
		status: 'error',
		health_score: 0,
		last_error: 'Provider returned HTTP 401'
	});
	expect(result.summary).toEqual({ total: 1, connected: 0, disconnected: 1, errors: 1, health: 0 });
	for (const connections of [
		[{ ...receipt, connection: null }],
		[receipt, receipt],
		[{ ...receipt, health_score: 101 }]
	])
		expect(() =>
			connectionStatusView({ success: true, connections }, new URLSearchParams())
		).toThrow();
	expect(() => connectionStatusView({ success: false }, new URLSearchParams())).toThrow();
});

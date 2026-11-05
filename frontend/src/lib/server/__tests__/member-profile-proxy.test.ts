// @vitest-environment node
import { afterEach, expect, it, vi } from 'vite-plus/test';
vi.mock('$app/env/private', () => ({ API_BASE_URL: 'http://backend.test', BACKEND_URL: '' }));
const auth = vi.hoisted(() => ({
	admin: vi.fn(() => ({ token: 'fixture-token', user: { id: 2 } }))
}));
vi.mock('../auth.js', () => ({ requireAdmin: auth.admin }));
import { proxyMemberProfile } from '../member-profile-proxy';
function event(method = 'GET', search = '', content?: unknown) {
	const url = new URL(`http://frontend.test/api/admin/members/9/notes${search}`);
	return {
		url,
		params: { id: '9' },
		request: new Request(url, {
			method,
			...(content === undefined
				? {}
				: { headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(content) })
		})
	} as Parameters<typeof proxyMemberProfile>[0];
}
const note = {
	id: 17,
	user_id: 9,
	content: 'Saved\nactual note',
	created_by: 2,
	created_by_name: 'Recorded actor',
	created_at: '2026-11-05T20:00:00.000000Z',
	updated_at: '2026-11-05T20:00:00.000000Z'
};
const acknowledgment = {
	schema_version: 1,
	metrics_scope: 'member_note_mutation',
	member_id: 9,
	note
};
afterEach(() => {
	vi.clearAllMocks();
});
it('retains exact history paging/filtering once, omits browser credentials and prevents cached private reads', async () => {
	const upstream = vi.fn<typeof fetch>().mockResolvedValue(Response.json({ activity: [] }));
	const response = await proxyMemberProfile(
		event('GET', '?page=2&per_page=10&action=member_note_created'),
		'members',
		'activity',
		undefined,
		upstream
	);
	expect(response.status).toBe(200);
	expect(upstream).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/members/9/activity?page=2&per_page=10&action=member_note_created',
		expect.objectContaining({
			method: 'GET',
			credentials: 'omit',
			redirect: 'error',
			cache: 'no-store',
			headers: { Accept: 'application/json', Authorization: 'Bearer fixture-token' }
		})
	);
	expect(response.headers.get('cache-control')).toContain('no-store');
	expect(response.headers.get('vary')).toBe('Authorization, Cookie');
});
it('refuses invalid identities, duplicate or unsupported queries and malformed note bodies before upstream', async () => {
	const upstream = vi.fn<typeof fetch>();
	for (const input of [
		event('GET', '?page=0'),
		event('GET', '?per_page=101'),
		event('GET', '?page=1&page=2'),
		event('GET', '?action=not-for-notes'),
		event('GET', '?unknown=1'),
		event('POST', '', { content: ' ' }),
		event('POST', '', { content: 'A', created_by: 77 }),
		event('POST', '', { content: '\0' }),
		event('POST', '', { content: 'A'.repeat(10001) })
	]) {
		const response = await proxyMemberProfile(input, 'members', 'notes', undefined, upstream);
		expect(response.status).toBe(400);
	}
	const input = event();
	input.params.id = '9223372036854775808';
	expect((await proxyMemberProfile(input, 'members', 'notes', undefined, upstream)).status).toBe(
		400
	);
	expect(upstream).not.toHaveBeenCalled();
});
it('sends the unchanged validated content once and relays its actual saved row', async () => {
	const upstream = vi.fn<typeof fetch>().mockResolvedValue(Response.json(acknowledgment));
	const response = await proxyMemberProfile(
		event('POST', '', { content: note.content }),
		'member-management',
		'notes',
		undefined,
		upstream
	);
	expect(response.status).toBe(200);
	expect(await response.json()).toEqual(acknowledgment);
	expect(upstream).toHaveBeenCalledExactlyOnceWith(
		'http://backend.test/api/admin/member-management/9/notes',
		expect.objectContaining({ method: 'POST', body: JSON.stringify({ content: note.content }) })
	);
});
it('never retries a note after a network timeout or an uncertain successful acknowledgment', async () => {
	for (const result of ['network', 'malformed', 'wrong-owner', 'wrong-content', 'wrong-actor']) {
		const upstream = vi.fn<typeof fetch>();
		if (result === 'network') upstream.mockRejectedValue(new Error('Network response lost'));
		else
			upstream.mockResolvedValue(
				Response.json(
					result === 'malformed'
						? { message: 'Saved' }
						: {
								...acknowledgment,
								note: {
									...note,
									...(result === 'wrong-owner'
										? { user_id: 10 }
										: result === 'wrong-actor'
											? { created_by: 77 }
											: { content: 'Other content' })
								}
							}
				)
			);
		const response = await proxyMemberProfile(
			event('POST', '', { content: note.content }),
			'members',
			'notes',
			undefined,
			upstream
		);
		expect(response.status).toBe(result === 'network' ? 503 : 502);
		expect(await response.json()).toMatchObject({
			outcome: 'unconfirmed',
			reconcile_history_before_retry: true
		});
		expect(upstream).toHaveBeenCalledTimes(1);
	}
});
it('accepts explicit idempotent deletion and rejects a success for another note', async () => {
	const upstream = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(
			Response.json({ ...acknowledgment, note: undefined, note_id: 17, removed: false })
		)
		.mockResolvedValueOnce(
			Response.json({ ...acknowledgment, note: undefined, note_id: 18, removed: true })
		);
	expect(
		(await proxyMemberProfile(event('DELETE'), 'members', 'notes', '17', upstream)).status
	).toBe(200);
	expect(
		(await proxyMemberProfile(event('DELETE'), 'members', 'notes', '17', upstream)).status
	).toBe(502);
	expect(upstream).toHaveBeenCalledTimes(2);
});
it('preserves a JSON storage error and marks an uncertain mutation for reconciliation without retrying', async () => {
	const upstream = vi
		.fn<typeof fetch>()
		.mockResolvedValue(Response.json({ message: 'Audit unavailable' }, { status: 503 }));
	const response = await proxyMemberProfile(
		event('POST', '', { content: note.content }),
		'members',
		'notes',
		undefined,
		upstream
	);
	expect(response.status).toBe(503);
	expect(await response.json()).toMatchObject({
		message: 'Audit unavailable',
		outcome: 'unconfirmed',
		reconcile_history_before_retry: true
	});
	expect(upstream).toHaveBeenCalledTimes(1);
});
it('preserves real role/storage errors without converting them into empty successful history', async () => {
	const upstream = vi
		.fn<typeof fetch>()
		.mockResolvedValueOnce(new Response('Admin access required', { status: 403 }))
		.mockResolvedValueOnce(Response.json({ message: 'Audit unavailable' }, { status: 503 }))
		.mockResolvedValueOnce(new Response('invalid-json'));
	for (const status of [403, 503, 502]) {
		const response = await proxyMemberProfile(event(), 'members', 'notes', undefined, upstream);
		expect(response.status).toBe(status);
		expect((await response.json()).notes).toBeUndefined();
	}
	expect(upstream).toHaveBeenCalledTimes(3);
});
it('performs no upstream attempt when the global admin guard rejects the session', async () => {
	const upstream = vi.fn<typeof fetch>();
	auth.admin.mockImplementationOnce(() => {
		throw new Error('Forbidden');
	});
	await expect(
		proxyMemberProfile(event(), 'members', undefined, undefined, upstream)
	).rejects.toThrow('Forbidden');
	expect(upstream).not.toHaveBeenCalled();
});

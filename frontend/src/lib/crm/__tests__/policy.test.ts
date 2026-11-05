// @vitest-environment node
import { expect, it, vi } from 'vite-plus/test';
import source from '../../../../../api/assets/policies/crm-permissions.json?raw';
import { parseCrmCapabilities, type CrmCapabilities } from '../capabilities.js';
import {
	CrmAccessError,
	createCrmRequestContext,
	crmAccessFailure
} from '../../server/crm-context.js';
import {
	crmActions,
	crmBaseAllowed,
	crmFactsAllowed,
	hashCrmPolicy,
	matchCrmPage,
	matchCrmRoute,
	parseCrmJson,
	parseCrmPolicy,
	resolveCrmOperation,
	type CrmAction,
	type CrmGrant,
	type CrmPrincipal
} from '../policy.js';

const policy = parseCrmPolicy(source);
const hash = '7edb80533c772940bc713c4d558722c4e63dfd919629d4e99f4848fba5a26eb0';
const grant = (module: string, action: CrmAction): CrmGrant => ({ module, action, allowed: true });
const principal = (permissions: CrmGrant[], global_admin = false): CrmPrincipal => ({
	permissions,
	global_admin
});
function capabilities(
	permissions: CrmGrant[] = [],
	extra: Partial<CrmCapabilities> = {}
): CrmCapabilities {
	return {
		user_id: '17',
		global_admin: false,
		manager_id: permissions.length ? '2' : null,
		role_id: permissions.length ? '3' : null,
		role_version: permissions.length ? 1 : null,
		assignment_version: permissions.length ? 1 : null,
		permissions,
		policy_version: 1,
		policy_hash: hash,
		security_management_requires_global_admin: true,
		default_role_is_automatic: false,
		metrics_scope: 'current_crm_capabilities',
		...extra
	};
}
function route(method: string, path: string) {
	const url = new URL(path, 'https://fixture.test');
	return matchCrmRoute(policy, method, url.pathname, url.searchParams);
}
function requiredRoute(method: string, path: string) {
	const result = route(method, path);
	if (!result) throw new Error('Missing canonical fixture route');
	return result;
}
function context(fetch: typeof globalThis.fetch) {
	return createCrmRequestContext({
		policy,
		policyHash: Promise.resolve(hash),
		userId: '17',
		token: 'owned-fixture-token',
		apiBase: 'http://127.0.0.1:8281',
		fetch
	});
}

it('matches the Rust policy hash and resolves every declared page operation', async () => {
	expect(await hashCrmPolicy(source)).toBe(hash);
	expect(policy.pages).toHaveLength(64);
	for (const page of policy.pages)
		for (const operation of page.operations)
			expect(resolveCrmOperation(policy, operation)).toBeDefined();
	expect(Object.isFrozen(policy.routes[0])).toBe(true);
});
it('rejects duplicate escaped keys, malformed/nonfinite JSON and excess nesting', () => {
	for (const value of ['{"a":1,"\\u0061":2}', '{"a":true,}', '[1,]', '[1e999]', 'true false'])
		expect(() => parseCrmJson(value)).toThrow();
	expect(() => parseCrmJson('['.repeat(34) + '0' + ']'.repeat(34))).toThrow();
});
it('rejects unknown policy fields, route ambiguity and unresolved page operations', () => {
	expect(() => parseCrmPolicy(JSON.stringify({ ...policy, extra: true }))).toThrow();
	expect(() =>
		parseCrmPolicy(JSON.stringify({ ...policy, routes: [...policy.routes, policy.routes[0]] }))
	).toThrow();
	expect(() =>
		parseCrmPolicy(
			JSON.stringify({
				...policy,
				pages: [
					{
						path: '/unknown',
						params: {},
						mode: 'all',
						operations: [{ method: 'GET', path: '/unknown', query_equals: {} }]
					}
				]
			})
		)
	).toThrow();
});
it('matches bounded i64 and UUID identities and disambiguates exact selector queries', () => {
	expect(route('GET', '/api/admin/crm/contacts/9223372036854775807')).toBeDefined();
	for (const path of [
		'/contacts/0',
		'/contacts/01',
		'/contacts/9223372036854775808',
		'/contacts/%31',
		'/contacts/1/',
		'/contacts/1/notes/extra',
		'/imports/4'
	])
		expect(route('GET', '/api/admin/crm' + path)).toBeUndefined();
	expect(route('GET', '/api/admin/crm/imports/12345678-1234-1234-1234-123456789abc')).toBeDefined();
	expect(route('GET', '/api/admin/crm/imports/options?kind=tags')).toBeDefined();
	expect(route('GET', '/api/admin/crm/imports/options?kind=lists')).toBeDefined();
	for (const query of ['?search=one&search=two', '?page=1&page=2'])
		expect(route('GET', '/api/admin/crm/contacts' + query)).toBeUndefined();
	for (const query of ['', '?kind=users', '?kind=tags&kind=lists', '?kind=tags&kind=tags'])
		expect(route('GET', '/api/admin/crm/imports/options' + query)).toBeUndefined();
});
it('admits exact CRM aliases while preserving native and unfinished route boundaries', () => {
	for (const path of ['/admin/contacts', '/crm/contacts/2', '/admin/crm/managers/roles/2/edit'])
		expect(matchCrmPage(policy, path)).toBeDefined();
	for (const path of [
		'/email',
		'/admin/email/templates',
		'/admin/crm/abandoned-carts',
		'/admin/crm/managers/add',
		'/crm/contacts/uuid'
	])
		expect(matchCrmPage(policy, path)).toBeUndefined();
	expect(route('GET', '/api/admin/email/sequences')).toBeUndefined();
});
it('requires actual cross-module read bundles and never expands manager manage to global roster/security writes', () => {
	const contacts = requiredRoute('GET', '/api/admin/crm/contacts');
	expect(crmBaseAllowed(policy, principal([grant('contacts', 'view')]), contacts)).toBe(false);
	expect(
		crmBaseAllowed(policy, principal([grant('contacts', 'view'), grant('deals', 'view')]), contacts)
	).toBe(true);
	for (const [method, path] of [
		['GET', '/managers/users'],
		['POST', '/managers']
	])
		expect(
			crmBaseAllowed(
				policy,
				principal([grant('managers', 'manage')]),
				requiredRoute(method, '/api/admin/crm' + path)
			)
		).toBe(false);
	expect(
		crmBaseAllowed(
			policy,
			principal([grant('sequences', 'manage')]),
			requiredRoute('POST', '/api/admin/crm/sequences')
		)
	).toBe(true);
});
it('requires both narrow template picker prerequisites without granting native APIs', () => {
	const picker = requiredRoute('GET', '/api/admin/crm/contacts/native-templates');
	expect(crmBaseAllowed(policy, principal([grant('contacts', 'manage')]), picker)).toBe(false);
	expect(
		crmBaseAllowed(
			policy,
			principal([grant('contacts', 'manage'), grant('templates', 'view')]),
			picker
		)
	).toBe(true);
});
it('requires manage for supplied execution status and fails closed on missing or extra typed facts', () => {
	const selected = requiredRoute('PUT', '/api/admin/crm/sequences/2');
	const expected = policy.facts.filter(
		(fact) => selected.conditional.includes(fact.id) && fact.context === 'body'
	);
	const status = expected.find((fact) =>
		fact.all.some((required) => required.module === 'sequences' && required.action === 'manage')
	);
	if (!status) throw new Error('Missing actual status assertion policy');
	const actual = Object.fromEntries(expected.map((fact) => [fact.id, false]));
	const editor = principal([grant('sequences', 'edit'), grant('sequences', 'view')]);
	expect(crmFactsAllowed(policy, editor, selected, 'body', actual)).toBe(true);
	expect(crmFactsAllowed(policy, editor, selected, 'body', { ...actual, [status.id]: true })).toBe(
		false
	);
	const falseGrant = grant('sequences', 'manage');
	Object.defineProperty(falseGrant, 'allowed', { value: false });
	expect(
		crmFactsAllowed(policy, principal([falseGrant]), selected, 'body', {
			...actual,
			[status.id]: true
		})
	).toBe(false);
	expect(() => crmFactsAllowed(policy, editor, selected, 'body', {})).toThrow();
	expect(() =>
		crmFactsAllowed(policy, editor, selected, 'body', { ...actual, extra: false })
	).toThrow();
});
it('accepts a genuine unassigned user but rejects another identity, stale policy, malformed grants and incomplete expansion', () => {
	expect(
		parseCrmCapabilities(JSON.stringify(capabilities()), policy, hash, '17').permissions
	).toEqual([]);
	for (const value of [
		capabilities([], { user_id: '18' }),
		capabilities([], { policy_hash: '0'.repeat(64) }),
		capabilities([], { manager_id: '2' }),
		capabilities([grant('contacts', 'manage')]),
		{ ...capabilities(), is_admin: true },
		{ ...capabilities(), permissions: [{ module: 'contacts', action: 'view', allowed: false }] }
	])
		expect(() => parseCrmCapabilities(JSON.stringify(value), policy, hash, '17')).toThrow();
	const all = policy.modules.flatMap((module) =>
		crmActions.map((action) => grant(module.id, action))
	);
	expect(
		parseCrmCapabilities(
			JSON.stringify(capabilities(all, { global_admin: true })),
			policy,
			hash,
			'17'
		).global_admin
	).toBe(true);
	expect(() =>
		parseCrmCapabilities(
			JSON.stringify(capabilities([], { global_admin: true })),
			policy,
			hash,
			'17'
		)
	).toThrow();
});
it('binds current capabilities to one request and forwards the verified token only to the configured private origin', async () => {
	const fetch = vi
		.fn<typeof globalThis.fetch>()
		.mockResolvedValue(
			Response.json(capabilities([grant('contacts', 'view'), grant('deals', 'view')]))
		);
	const current = context(fetch);
	await Promise.all([
		current.requireOperation('GET', '/api/admin/crm/contacts', new URLSearchParams()),
		current.requirePage('/admin/contacts')
	]);
	expect(fetch).toHaveBeenCalledOnce();
	expect(fetch.mock.calls[0]?.[0]).toEqual(
		new URL('http://127.0.0.1:8281/api/admin/crm/permissions')
	);
	expect(fetch.mock.calls[0]?.[1]).toMatchObject({
		credentials: 'omit',
		redirect: 'error',
		cache: 'no-store',
		headers: { Authorization: 'Bearer owned-fixture-token' }
	});
});
it('preserves failed context closure, separates requests, and distinguishes401/403 from503', async () => {
	const failed = vi.fn<typeof globalThis.fetch>().mockRejectedValue(new Error('Owned outage'));
	const current = context(failed);
	await expect(current.current()).rejects.toMatchObject({ status: 503 });
	await expect(current.current()).rejects.toMatchObject({ status: 503 });
	expect(failed).toHaveBeenCalledOnce();
	const successful = vi
		.fn<typeof globalThis.fetch>()
		.mockImplementation(async () => Response.json(capabilities()));
	await context(successful).current();
	await context(successful).current();
	expect(successful).toHaveBeenCalledTimes(2);
	for (const status of [401, 403, 500, 503])
		await expect(
			context(async () => Response.json({}, { status })).current()
		).rejects.toMatchObject({ status: status === 401 || status === 403 ? status : 503 });
	await expect(
		context(async () => Response.json(capabilities())).requireOperation(
			'GET',
			'/api/admin/crm/contacts',
			new URLSearchParams()
		)
	).rejects.toMatchObject({ status: 403 });
});
it('cannot forward native/unknown paths and applies private noindex failure headers', async () => {
	const fetch = vi.fn<typeof globalThis.fetch>();
	await expect(
		context(fetch).requireOperation('GET', '/api/admin/email/templates', new URLSearchParams())
	).rejects.toMatchObject({ status: 404 });
	expect(fetch).not.toHaveBeenCalled();
	for (const status of [401, 403, 404, 503] as const) {
		const response = crmAccessFailure(new CrmAccessError(status, 'Fixture denial'));
		expect(response.status).toBe(status);
		expect(response.headers.get('Cache-Control')).toBe('private, no-store');
		expect(response.headers.get('Vary')).toBe('Authorization, Cookie');
		expect(response.headers.get('X-Robots-Tag')).toBe('noindex, nofollow');
	}
});

it('does not expose aggregate member-directory campaign/history totals through dashboard grants', () => {
	const stats = requiredRoute('GET', '/api/admin/crm/stats');
	const viewer = principal([
		grant('dashboard', 'view'),
		grant('contacts', 'view'),
		grant('deals', 'view'),
		grant('campaigns', 'view')
	]);
	expect(crmBaseAllowed(policy, principal([grant('dashboard', 'view')]), stats)).toBe(false);
	expect(crmBaseAllowed(policy, viewer, stats)).toBe(true);
	expect(crmFactsAllowed(policy, viewer, stats, 'record', { record_member_audience: false })).toBe(
		true
	);
	expect(crmFactsAllowed(policy, viewer, stats, 'record', { record_member_audience: true })).toBe(
		false
	);
	expect(crmFactsAllowed(policy, viewer, stats, 'history', { history_member_audience: true })).toBe(
		false
	);
	expect(() => crmFactsAllowed(policy, viewer, stats, 'history', {})).toThrow();
});

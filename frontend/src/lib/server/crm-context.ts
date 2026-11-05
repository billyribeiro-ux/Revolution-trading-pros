import { parseCrmCapabilities, type CrmCapabilities } from '../crm/capabilities.js';
import {
	crmBaseAllowed,
	crmPageAllowed,
	matchCrmPage,
	matchCrmRoute,
	type CrmPage,
	type CrmPolicy,
	type CrmRoute
} from '../crm/policy.js';

export class CrmAccessError extends Error {
	constructor(
		readonly status: 401 | 403 | 404 | 503,
		message: string
	) {
		super(message);
		this.name = 'CrmAccessError';
	}
}
export interface CrmContextOptions {
	readonly policy: CrmPolicy;
	readonly policyHash: Promise<string>;
	readonly userId: string;
	readonly token: string;
	readonly apiBase: string;
	readonly fetch: typeof fetch;
}
export interface CrmRequestContext {
	current(): Promise<CrmCapabilities>;
	requireOperation(
		method: string,
		path: string,
		query: URLSearchParams
	): Promise<{ capabilities: CrmCapabilities; route: CrmRoute }>;
	requirePage(path: string): Promise<{ capabilities: CrmCapabilities; page: CrmPage }>;
}
/** Construct inside one authenticated request. A rejected promise stays rejected for that request. */
export function createCrmRequestContext(options: CrmContextOptions): CrmRequestContext {
	let pending: Promise<CrmCapabilities> | undefined;
	async function read(): Promise<CrmCapabilities> {
		if (!options.token) throw new CrmAccessError(401, 'Authentication required');
		try {
			const base = new URL(options.apiBase);
			if (
				!['http:', 'https:'].includes(base.protocol) ||
				base.username ||
				base.password ||
				base.search ||
				base.hash ||
				base.pathname !== '/'
			)
				throw new Error('Invalid configured API origin');
			const response = await options.fetch(
				new URL(options.policy.api_prefix + options.policy.capabilities_path, base),
				{
					method: 'GET',
					headers: {
						Authorization: `Bearer ${options.token}`,
						Accept: 'application/json'
					},
					credentials: 'omit',
					redirect: 'error',
					cache: 'no-store',
					signal: AbortSignal.timeout(10000)
				}
			);
			if (response.status === 401) throw new CrmAccessError(401, 'Authentication required');
			if (response.status === 403) throw new CrmAccessError(403, 'CRM access denied');
			if (
				!response.ok ||
				!response.headers.get('Content-Type')?.toLowerCase().startsWith('application/json')
			)
				throw new Error('CRM capabilities unavailable');
			return parseCrmCapabilities(
				await response.text(),
				options.policy,
				await options.policyHash,
				options.userId
			);
		} catch (error) {
			if (error instanceof CrmAccessError) throw error;
			throw new CrmAccessError(503, 'CRM authorization is unavailable; no action was authorized');
		}
	}
	const current = () => (pending ??= read());
	return Object.freeze({
		current,
		async requireOperation(method: string, path: string, query: URLSearchParams) {
			const route = matchCrmRoute(options.policy, method, path, query);
			if (!route) throw new CrmAccessError(404, 'CRM endpoint not found');
			const capabilities = await current();
			if (!crmBaseAllowed(options.policy, capabilities, route))
				throw new CrmAccessError(403, 'Your CRM role does not allow this action');
			// Record/body/history/import-plan facts are checked by the typed backend handler, never assumed false here.
			return { capabilities, route };
		},
		async requirePage(path: string) {
			const page = matchCrmPage(options.policy, path);
			if (!page) throw new CrmAccessError(404, 'CRM page not found');
			const capabilities = await current();
			if (!crmPageAllowed(options.policy, capabilities, page))
				throw new CrmAccessError(403, 'Your CRM role does not allow this page');
			return { capabilities, page };
		}
	});
}
export function crmPrivateHeaders(): Headers {
	return new Headers({
		'Cache-Control': 'private, no-store',
		Vary: 'Authorization, Cookie',
		'X-Robots-Tag': 'noindex, nofollow',
		'X-Content-Type-Options': 'nosniff'
	});
}
export function crmAccessFailure(error: unknown): Response {
	const failure =
		error instanceof CrmAccessError
			? error
			: new CrmAccessError(503, 'CRM authorization is unavailable; no action was authorized');
	return Response.json(
		{ error: failure.message },
		{ status: failure.status, headers: crmPrivateHeaders() }
	);
}

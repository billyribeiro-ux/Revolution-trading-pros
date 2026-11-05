import type { RequestEvent } from '@sveltejs/kit';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';
import { currentCrmPolicy } from '../config/crm-policy.js';
import { CrmAccessError, createCrmRequestContext, type CrmRequestContext } from './crm-context.js';

export interface CrmContextBinding {
	readonly userId: string;
	readonly token: string;
	readonly context: CrmRequestContext;
}
/** Auth hook has already validated this identity. Keep bearer data in server locals only. */
export function crmRequestContext(event: RequestEvent): CrmRequestContext {
	const user = event.locals.user;
	const token = event.locals.accessToken;
	if (!user || !token) throw new CrmAccessError(401, 'Authentication required');
	const userId = String(user.id),
		prior = event.locals.crmContext;
	if (prior) {
		if (prior.userId !== userId || prior.token !== token)
			throw new CrmAccessError(503, 'CRM request identity changed; no action was authorized');
		return prior.context;
	}
	try {
		const compiled = currentCrmPolicy();
		const context = createCrmRequestContext({
			policy: compiled.policy,
			policyHash: compiled.hash,
			userId,
			token,
			apiBase: API_BASE_URL || BACKEND_URL || 'http://localhost:8080',
			fetch
		});
		event.locals.crmContext = Object.freeze({ userId, token, context });
		return context;
	} catch (error) {
		if (error instanceof CrmAccessError) throw error;
		throw new CrmAccessError(503, 'CRM authorization is unavailable; no action was authorized');
	}
}
/** Static/base gate only. Backend typed handlers recheck all current grants and actual conditional facts. */
export async function requireCrmOperation(event: RequestEvent): Promise<{ token: string }> {
	await crmRequestContext(event).requireOperation(
		event.request.method,
		event.url.pathname,
		event.url.searchParams
	);
	const token = event.locals.accessToken;
	if (!token) throw new CrmAccessError(401, 'Authentication required');
	return { token };
}
/** Kit data transport must resolve to the same exact underlying page, not another permission namespace. */
export function crmPagePath(event: Pick<RequestEvent, 'url' | 'isDataRequest'>): string {
	return event.isDataRequest && event.url.pathname.endsWith('/__data.json')
		? event.url.pathname.slice(0, -'/__data.json'.length)
		: event.url.pathname;
}

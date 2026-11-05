import { currentCrmPolicy } from '#lib/config/crm-policy.js';
import { crmBaseAllowed, matchCrmRoute } from '../policy.js';
import { isCrmClientRequest } from '../request-gate.js';
/** Explicit global-admin browser fixture for transport tests; no production gate override. */
export function authorizedCrmRequestFixture(): () => void {
	if (typeof window === 'undefined') return () => undefined;
	const listener = (event: Event) => {
		if (!(event instanceof CustomEvent) || !isCrmClientRequest(event.detail)) return;
		const detail = event.detail,
			policy = currentCrmPolicy().policy,
			route = matchCrmRoute(policy, detail.method, detail.path, detail.query);
		detail.checked = true;
		if (!route || !crmBaseAllowed(policy, { global_admin: true, permissions: [] }, route)) {
			detail.reason = 'Fixture request is outside the canonical policy';
			event.preventDefault();
		}
	};
	window.addEventListener('crm-request', listener);
	return () => window.removeEventListener('crm-request', listener);
}

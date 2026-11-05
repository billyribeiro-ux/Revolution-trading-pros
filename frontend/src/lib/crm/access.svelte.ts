import { createContext, onDestroy } from 'svelte';
import { isCrmClientRequest } from './request-gate.js';
import { currentCrmPolicy } from '#lib/config/crm-policy.js';
import { parseCrmCapabilities, type CrmCapabilities } from './capabilities.js';
import {
	crmBaseAllowed,
	crmPageAllowed,
	matchCrmPage,
	matchCrmRoute,
	resolveCrmOperation,
	type CrmMethod,
	type CrmPrincipal
} from './policy.js';
export interface CrmAccess {
	readonly capabilities: CrmCapabilities | null;
	readonly global: boolean;
	readonly error: string;
	can(method: CrmMethod, path: string, query?: Record<string, string>): boolean;
	page(path: string): boolean;
	clear(): void;
	refresh(): Promise<void>;
}
const [readAccess, writeAccess] = createContext<CrmAccess>();
export function useCrmAccess(): CrmAccess {
	return readAccess();
}
/** One instance per root layout/SSR request; never place grants in a module singleton or authStore. */
export function provideCrmAccess(
	initial: () => CrmCapabilities | undefined,
	userId: () => string | null | undefined,
	globalIdentity: () => boolean,
	identity: () => unknown = initial
): CrmAccess {
	let override = $state.raw<{ source: unknown; value: CrmCapabilities | null } | null>(null);
	let failure = $state('');
	let generation = 0;
	const capabilities = $derived(
		override && override.source === identity() ? override.value : (initial() ?? null)
	);
	const global = $derived(
		override && override.source === identity()
			? override.value?.global_admin === true
			: initial() !== undefined
				? capabilities?.global_admin === true
				: globalIdentity()
	);
	function principal(): CrmPrincipal | null {
		return capabilities ?? (global ? { global_admin: true, permissions: [] } : null);
	}
	const access: CrmAccess = {
		get capabilities() {
			return capabilities;
		},
		get global() {
			return global;
		},
		get error() {
			return failure;
		},
		can(method, path, query = {}) {
			try {
				const policy = currentCrmPolicy().policy,
					route = resolveCrmOperation(policy, { method, path, query_equals: query }),
					actor = principal();
				return !!route && !!actor && crmBaseAllowed(policy, actor, route);
			} catch {
				return false;
			}
		},
		page(path) {
			if (global) return true;
			try {
				const policy = currentCrmPolicy().policy,
					page = matchCrmPage(policy, path),
					actor = principal();
				return !!page && !!actor && crmPageAllowed(policy, actor, page);
			} catch {
				return false;
			}
		},
		clear() {
			generation++;
			override = { source: identity(), value: null };
			failure = 'CRM access changed. Refresh permissions before continuing.';
		},
		async refresh() {
			const source = identity(),
				expected = userId(),
				request = ++generation;
			override = { source, value: null };
			failure = '';
			try {
				if (!expected) throw new Error('Authentication required');
				const response = await fetch('/api/admin/crm/permissions', {
					cache: 'no-store',
					credentials: 'same-origin',
					redirect: 'error',
					headers: { Accept: 'application/json' },
					signal: AbortSignal.timeout(10000)
				});
				if (!response.ok)
					throw new Error(
						response.status === 401
							? 'Authentication required'
							: response.status === 403
								? 'CRM access denied'
								: 'CRM authorization is unavailable'
					);
				const compiled = currentCrmPolicy(),
					value = parseCrmCapabilities(
						await response.text(),
						compiled.policy,
						await compiled.hash,
						expected
					);
				if (request === generation && source === identity()) override = { source, value };
			} catch (cause) {
				if (request === generation && source === identity()) {
					override = { source, value: null };
					failure = cause instanceof Error ? cause.message : 'CRM authorization is unavailable';
				}
				throw cause;
			}
		}
	};
	if (typeof window !== 'undefined') {
		const gate = (event: Event) => {
			if (!(event instanceof CustomEvent) || !isCrmClientRequest(event.detail)) return;
			const detail = event.detail;
			detail.checked = true;
			try {
				const policy = currentCrmPolicy().policy,
					route = matchCrmRoute(policy, detail.method, detail.path, detail.query),
					actor = principal();
				if (!route || !actor || !crmBaseAllowed(policy, actor, route)) {
					detail.reason = route
						? 'Your current CRM permissions do not include this action and its read dependencies.'
						: 'This CRM action is not implemented by the registered API.';
					event.preventDefault();
				}
			} catch {
				detail.reason = 'CRM authorization is unavailable';
				event.preventDefault();
			}
		};
		window.addEventListener('crm-request', gate);
		onDestroy(() => window.removeEventListener('crm-request', gate));
	}
	writeAccess(access);
	return access;
}

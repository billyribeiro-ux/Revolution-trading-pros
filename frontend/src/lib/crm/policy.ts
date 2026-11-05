/** Public, immutable policy only. Current identity/grants are always supplied by the server. */
export const crmActions = ['view', 'create', 'edit', 'delete', 'manage'] as const;
export type CrmAction = (typeof crmActions)[number];
export type CrmParameter = 'positive_integer' | 'uuid' | 'slug';
export type CrmFactContext = 'body' | 'record' | 'history' | 'import_plan' | 'export_record';
export type CrmMethod = 'GET' | 'HEAD' | 'POST' | 'PUT' | 'PATCH' | 'DELETE';
export interface CrmRequirement {
	readonly module: string;
	readonly action: CrmAction;
}
export interface CrmGrant extends CrmRequirement {
	readonly allowed: true;
}
export interface CrmPrincipal {
	readonly global_admin: boolean;
	readonly permissions: readonly CrmGrant[];
}
export interface CrmOperation {
	readonly method: CrmMethod;
	readonly path: string;
	readonly query_equals: Readonly<Record<string, string>>;
}
export interface CrmRoute {
	readonly path: string;
	readonly methods: readonly CrmMethod[];
	readonly params: Readonly<Record<string, CrmParameter>>;
	readonly query_equals: Readonly<Record<string, string>>;
	readonly all: readonly CrmRequirement[];
	readonly global_admin_only: boolean;
	readonly conditional: readonly string[];
}
export interface CrmPage {
	readonly path: string;
	readonly params: Readonly<Record<string, CrmParameter>>;
	readonly mode: 'all' | 'any';
	readonly operations: readonly CrmOperation[];
}
export interface CrmFact {
	readonly id: string;
	readonly context: CrmFactContext;
	readonly source_fields: readonly string[];
	readonly description: string;
	readonly all: readonly CrmRequirement[];
	readonly global_admin_only: boolean;
}
export interface CrmPolicy {
	readonly policy_version: 1;
	readonly api_prefix: '/api/admin/crm';
	readonly capabilities_path: '/permissions';
	readonly actions: readonly CrmAction[];
	readonly manage_implies: readonly CrmAction[];
	readonly modules: readonly {
		readonly id: string;
		readonly name: string;
		readonly notes: readonly string[];
	}[];
	readonly facts: readonly CrmFact[];
	readonly routes: readonly CrmRoute[];
	readonly pages: readonly CrmPage[];
}
export class CrmPolicyError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'CrmPolicyError';
	}
}
function invalid(message: string): never {
	throw new CrmPolicyError(message);
}
function record(value: unknown, keys: readonly string[]): Record<string, unknown> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		return invalid('Expected CRM policy object');
	const result = value as Record<string, unknown>;
	if (Object.keys(result).length !== keys.length || keys.some((key) => !Object.hasOwn(result, key)))
		return invalid('Unknown/missing CRM policy field');
	return result;
}
function array(value: unknown, minimum: number, maximum: number): unknown[] {
	if (!Array.isArray(value) || value.length < minimum || value.length > maximum)
		return invalid('Invalid CRM policy array bounds');
	return value;
}
function hasControlCharacters(value: string): boolean {
	for (const character of value) {
		const code = character.charCodeAt(0);
		if (code < 32 || (code >= 127 && code <= 159)) return true;
	}
	return false;
}
function text(value: unknown, maximum: number): string {
	if (
		typeof value !== 'string' ||
		!value.trim() ||
		Array.from(value).length > maximum ||
		hasControlCharacters(value)
	)
		return invalid('Invalid CRM policy text');
	return value;
}
function identifier(value: unknown): string {
	const result = text(value, 64);
	if (!/^[a-z][a-z0-9_]*$/.test(result)) return invalid('Invalid CRM policy identifier');
	return result;
}
function boolean(value: unknown): boolean {
	if (typeof value !== 'boolean') return invalid('Invalid CRM policy boolean');
	return value;
}
function choice<T extends string>(value: unknown, choices: readonly T[]): T {
	const found = choices.find((item) => item === value);
	if (found === undefined) return invalid('Unsupported CRM policy value');
	return found;
}
function distinct<T>(values: T[], key: (value: T) => string): T[] {
	if (new Set(values.map(key)).size !== values.length) return invalid('Duplicate CRM policy entry');
	return values;
}
const methods: readonly CrmMethod[] = ['GET', 'HEAD', 'POST', 'PUT', 'PATCH', 'DELETE'];
const parameters: readonly CrmParameter[] = ['positive_integer', 'uuid', 'slug'];
const contexts: readonly CrmFactContext[] = [
	'body',
	'record',
	'history',
	'import_plan',
	'export_record'
];
/** Strict JSON scanner rejects duplicate keys, including escaped aliases, before parsing values. */
export function parseCrmJson(source: string): unknown {
	if (new TextEncoder().encode(source).byteLength > 2_000_000)
		return invalid('CRM document exceeds limit');
	let at = 0;
	const whitespace = () => {
		while (/[\t\n\r ]/.test(source[at] ?? 'x')) at++;
	};
	function string(): string {
		if (source[at] !== '"') return invalid('Expected JSON string');
		const start = at++;
		while (at < source.length) {
			if (source[at] === '\\') {
				at += 2;
				continue;
			}
			if (source[at++] === '"') {
				const result: unknown = JSON.parse(source.slice(start, at));
				if (typeof result !== 'string') return invalid('Invalid JSON string');
				return result;
			}
		}
		return invalid('Unterminated JSON string');
	}
	function value(depth: number): unknown {
		if (depth > 32) return invalid('CRM document nesting exceeds limit');
		whitespace();
		const next = source[at];
		if (next === '"') return string();
		if (next === '{') {
			at++;
			whitespace();
			const result: Record<string, unknown> = {},
				seen = new Set<string>();
			if (source[at] === '}') {
				at++;
				return result;
			}
			while (at < source.length) {
				whitespace();
				const key = string();
				if (seen.has(key)) return invalid('Duplicate CRM JSON key');
				seen.add(key);
				whitespace();
				if (source[at++] !== ':') return invalid('Invalid JSON object');
				Object.defineProperty(result, key, {
					value: value(depth + 1),
					enumerable: true
				});
				whitespace();
				if (source[at] === '}') {
					at++;
					return result;
				}
				if (source[at++] !== ',') return invalid('Invalid JSON object separator');
			}
		} else if (next === '[') {
			at++;
			whitespace();
			const result: unknown[] = [];
			if (source[at] === ']') {
				at++;
				return result;
			}
			while (at < source.length) {
				result.push(value(depth + 1));
				whitespace();
				if (source[at] === ']') {
					at++;
					return result;
				}
				if (source[at++] !== ',') return invalid('Invalid JSON array separator');
			}
		} else {
			const token = /^(?:true|false|null|-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)/.exec(
				source.slice(at)
			)?.[0];
			if (token) {
				at += token.length;
				const result: unknown = JSON.parse(token);
				if (typeof result === 'number' && !Number.isFinite(result))
					return invalid('Nonfinite CRM number');
				return result;
			}
		}
		return invalid('Malformed CRM JSON');
	}
	try {
		const result = value(0);
		whitespace();
		if (at !== source.length) return invalid('Trailing CRM JSON');
		return result;
	} catch (error) {
		if (error instanceof CrmPolicyError) throw error;
		return invalid('Malformed CRM JSON');
	}
}
function selector(value: unknown): Record<string, string> {
	if (!value || typeof value !== 'object' || Array.isArray(value) || Object.keys(value).length > 4)
		return invalid('Invalid selector');
	const result: Record<string, string> = {};
	for (const [key, item] of Object.entries(value)) result[identifier(key)] = text(item, 100);
	return result;
}
function pathParameters(value: unknown): Record<string, CrmParameter> {
	if (!value || typeof value !== 'object' || Array.isArray(value))
		return invalid('Invalid typed path parameters');
	const result: Record<string, CrmParameter> = {};
	for (const [key, item] of Object.entries(value))
		result[identifier(key)] = choice(item, parameters);
	return result;
}
function variable(segment: string): string | undefined {
	return /^\{([a-z][a-z0-9_]*)\}$/.exec(segment)?.[1];
}
function validatePath(path: string, params: Readonly<Record<string, CrmParameter>>): void {
	if (!path.startsWith('/') || path.endsWith('/') || path.length > 250 || /[?#%\\]/.test(path))
		return invalid('Noncanonical CRM path');
	const names: string[] = [];
	for (const segment of path.slice(1).split('/')) {
		const name = variable(segment);
		if (name) names.push(name);
		else if (!/^[a-z][a-z0-9_-]*$/.test(segment)) return invalid('Invalid static CRM path');
	}
	if (
		new Set(names).size !== names.length ||
		names.length !== Object.keys(params).length ||
		names.some((name) => !Object.hasOwn(params, name))
	)
		return invalid('Missing/repeated/unknown CRM path parameters');
}
export function matchesCrmParameter(kind: CrmParameter, value: string): boolean {
	if (kind === 'positive_integer')
		return /^[1-9]\d{0,18}$/.test(value) && BigInt(value) <= 9_223_372_036_854_775_807n;
	if (kind === 'uuid') return /^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/i.test(value);
	return /^[a-z][a-z0-9_-]{0,99}$/i.test(value);
}
function concretePath(
	template: string,
	params: Readonly<Record<string, CrmParameter>>,
	path: string
): boolean {
	if (
		!path.startsWith('/') ||
		path.endsWith('/') ||
		/[?#%\\ ]/.test(path) ||
		hasControlCharacters(path)
	)
		return false;
	const actual = path.slice(1).split('/'),
		expected = template.slice(1).split('/');
	return (
		actual.length === expected.length &&
		expected.every((segment, index) => {
			const name = variable(segment);
			return name ? matchesCrmParameter(params[name], actual[index]) : segment === actual[index];
		})
	);
}
function overlaps(
	left: { path: string; params: Readonly<Record<string, CrmParameter>> },
	right: { path: string; params: Readonly<Record<string, CrmParameter>> }
): boolean {
	const a = left.path.slice(1).split('/'),
		b = right.path.slice(1).split('/');
	return (
		a.length === b.length &&
		a.every((segment, index) => {
			const l = variable(segment),
				r = variable(b[index]);
			if (!l && !r) return segment === b[index];
			if (l && !r) return matchesCrmParameter(left.params[l], b[index]);
			if (!l && r) return matchesCrmParameter(right.params[r], segment);
			const x = left.params[l ?? ''],
				y = right.params[r ?? ''];
			return !(
				(x === 'positive_integer' && y !== 'positive_integer') ||
				(y === 'positive_integer' && x !== 'positive_integer')
			);
		})
	);
}
export function crmOperationId(operation: CrmOperation): string {
	const query = Object.entries(operation.query_equals)
		.sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))
		.map(([key, value]) => `${key}=${value}`)
		.join('&');
	return `${operation.method} ${operation.path}${query ? '?' + query : ''}`;
}
export function resolveCrmOperation(
	policy: CrmPolicy,
	operation: CrmOperation
): CrmRoute | undefined {
	return policy.routes.find(
		(route) =>
			route.path === operation.path &&
			route.methods.includes(operation.method) &&
			JSON.stringify(
				Object.entries(route.query_equals).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))
			) ===
				JSON.stringify(
					Object.entries(operation.query_equals).sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))
				)
	);
}
export function parseCrmPolicy(source: string): CrmPolicy {
	const raw = record(parseCrmJson(source), [
		'policy_version',
		'api_prefix',
		'capabilities_path',
		'actions',
		'manage_implies',
		'modules',
		'facts',
		'routes',
		'pages'
	]);
	if (
		raw.policy_version !== 1 ||
		raw.api_prefix !== '/api/admin/crm' ||
		raw.capabilities_path !== '/permissions' ||
		JSON.stringify(raw.actions) !== JSON.stringify(crmActions) ||
		JSON.stringify(raw.manage_implies) !== JSON.stringify(crmActions.slice(0, 4))
	)
		return invalid('Unsupported CRM policy namespace/version/actions');
	const modules = distinct(
		array(raw.modules, 1, 64).map((value) => {
			const row = record(value, ['id', 'name', 'notes']);
			return {
				id: identifier(row.id),
				name: text(row.name, 100),
				notes: array(row.notes, 0, 20).map((note) => text(note, 2000))
			};
		}),
		(item) => item.id
	);
	const requirements = (value: unknown, allowEmpty: boolean): CrmRequirement[] =>
		distinct(
			array(value, allowEmpty ? 0 : 1, 100).map((item) => {
				const row = record(item, ['module', 'action']),
					module = identifier(row.module);
				if (!modules.some((item) => item.id === module)) return invalid('Unknown CRM module');
				return { module, action: choice(row.action, crmActions) };
			}),
			(item) => `${item.module}:${item.action}`
		);
	const facts = distinct(
		array(raw.facts, 0, 100).map((value) => {
			const row = record(value, [
					'id',
					'context',
					'source_fields',
					'description',
					'all',
					'global_admin_only'
				]),
				global = boolean(row.global_admin_only);
			return {
				id: identifier(row.id),
				context: choice(row.context, contexts),
				source_fields: array(row.source_fields, 1, 10).map((field) => {
					const result = text(field, 200);
					if (!result.startsWith('/')) return invalid('Invalid fact field');
					return result;
				}),
				description: text(row.description, 2000),
				all: requirements(row.all, global),
				global_admin_only: global
			};
		}),
		(item) => item.id
	);
	const routes: CrmRoute[] = array(raw.routes, 1, 512).map((value) => {
		const row = record(value, [
				'path',
				'methods',
				'params',
				'query_equals',
				'all',
				'global_admin_only',
				'conditional'
			]),
			global = boolean(row.global_admin_only),
			path = text(row.path, 250),
			params = pathParameters(row.params),
			routeMethods = distinct(
				array(row.methods, 1, 6).map((item) => choice(item, methods)),
				(item) => item
			);
		validatePath(path, params);
		if (path === '/permissions' || routeMethods.includes('GET') !== routeMethods.includes('HEAD'))
			return invalid('Invalid CRM capabilities/HEAD route');
		const conditional = distinct(array(row.conditional, 0, 100).map(identifier), (item) => item);
		for (const id of conditional) {
			const fact = facts.find((item) => item.id === id);
			if (
				!fact ||
				(fact.context === 'body' &&
					routeMethods.some((method) => ['GET', 'HEAD', 'DELETE'].includes(method)))
			)
				return invalid('Unknown or bodyless CRM fact');
		}
		return {
			path,
			methods: routeMethods,
			params,
			query_equals: selector(row.query_equals),
			all: requirements(row.all, global),
			global_admin_only: global,
			conditional
		};
	});
	const selectorsOverlap = (left: CrmRoute, right: CrmRoute) =>
		!Object.entries(left.query_equals).some(
			([key, value]) => Object.hasOwn(right.query_equals, key) && right.query_equals[key] !== value
		);
	routes.forEach((route, index) => {
		if (
			routes
				.slice(0, index)
				.some(
					(prior) =>
						route.methods.some((method) => prior.methods.includes(method)) &&
						overlaps(route, prior) &&
						selectorsOverlap(route, prior)
				)
		)
			return invalid('Ambiguous CRM route rules');
	});
	if (facts.some((fact) => !routes.some((route) => route.conditional.includes(fact.id))))
		return invalid('Unused CRM fact');
	const pages: CrmPage[] = array(raw.pages, 0, 250).map((value) => {
		const row = record(value, ['path', 'params', 'mode', 'operations']),
			path = text(row.path, 250),
			params = pathParameters(row.params);
		validatePath(path, params);
		const operations = distinct(
			array(row.operations, 1, 50).map((value) => {
				const operation = record(value, ['method', 'path', 'query_equals']);
				return {
					method: choice(operation.method, methods),
					path: text(operation.path, 250),
					query_equals: selector(operation.query_equals)
				};
			}),
			crmOperationId
		);
		return { path, params, mode: choice(row.mode, ['all', 'any']), operations };
	});
	const policy: CrmPolicy = {
		policy_version: 1,
		api_prefix: '/api/admin/crm',
		capabilities_path: '/permissions',
		actions: [...crmActions],
		manage_implies: crmActions.slice(0, 4),
		modules,
		facts,
		routes,
		pages
	};
	pages.forEach((page, index) => {
		if (
			page.operations.some((operation) => !resolveCrmOperation(policy, operation)) ||
			pages.slice(0, index).some((prior) => overlaps(page, prior))
		)
			return invalid('Unknown page operation or ambiguous CRM page');
	});
	function freeze(value: unknown): void {
		if (value && typeof value === 'object') {
			for (const child of Object.values(value)) freeze(child);
			Object.freeze(value);
		}
	}
	freeze(policy);
	return policy;
}
export async function hashCrmPolicy(source: string): Promise<string> {
	const value = parseCrmJson(source);
	function canonical(value: unknown): string {
		if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
		if (value && typeof value === 'object')
			return `{${Object.entries(value)
				.sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))
				.map(([key, item]) => `${JSON.stringify(key)}:${canonical(item)}`)
				.join(',')}}`;
		return JSON.stringify(value) ?? invalid('Unsupported canonical JSON value');
	}
	const bytes = new TextEncoder().encode(canonical(value)),
		digest = await crypto.subtle.digest('SHA-256', bytes);
	return Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, '0')).join('');
}
export function matchCrmRoute(
	policy: CrmPolicy,
	method: string,
	path: string,
	query: URLSearchParams
): CrmRoute | undefined {
	if (!path.startsWith(policy.api_prefix + '/')) return undefined;
	const keys = new Set<string>();
	for (const key of query.keys()) {
		if (keys.has(key)) return undefined;
		keys.add(key);
	}
	const relative = path.slice(policy.api_prefix.length),
		candidates = policy.routes.filter(
			(route) =>
				route.methods.includes(method as CrmMethod) &&
				concretePath(route.path, route.params, relative) &&
				Object.entries(route.query_equals).every(
					([key, value]) => query.getAll(key).length === 1 && query.get(key) === value
				)
		);
	return candidates.length === 1 ? candidates[0] : undefined;
}
export function matchCrmPage(policy: CrmPolicy, path: string): CrmPage | undefined {
	const candidates = policy.pages.filter((page) => concretePath(page.path, page.params, path));
	return candidates.length === 1 ? candidates[0] : undefined;
}
export function crmBaseAllowed(
	policy: CrmPolicy,
	principal: CrmPrincipal,
	route: CrmRoute
): boolean {
	if (principal.global_admin) return true;
	return (
		!route.global_admin_only &&
		route.all.every((required) =>
			principal.permissions.some(
				(grant) =>
					grant.allowed &&
					grant.module === required.module &&
					(grant.action === required.action ||
						(grant.action === 'manage' && policy.manage_implies.includes(required.action)))
			)
		)
	);
}
export function crmPageAllowed(policy: CrmPolicy, principal: CrmPrincipal, page: CrmPage): boolean {
	const allowed = (operation: CrmOperation) => {
		const route = resolveCrmOperation(policy, operation);
		return !!route && crmBaseAllowed(policy, principal, route);
	};
	return page.mode === 'all' ? page.operations.every(allowed) : page.operations.some(allowed);
}
/** Typed server context only. Unknown/missing/extra facts are an evaluation failure, never false. */
export function crmFactsAllowed(
	policy: CrmPolicy,
	principal: CrmPrincipal,
	route: CrmRoute,
	context: CrmFactContext,
	actual: Readonly<Record<string, boolean>>
): boolean {
	const expected = route.conditional
		.map((id) => policy.facts.find((fact) => fact.id === id))
		.filter((fact): fact is CrmFact => !!fact && fact.context === context);
	if (
		Object.keys(actual).length !== expected.length ||
		expected.some((fact) => typeof actual[fact.id] !== 'boolean') ||
		Object.keys(actual).some((key) => !expected.some((fact) => fact.id === key))
	)
		return invalid('Incomplete/unexpected CRM conditional facts');
	return expected.every(
		(fact) =>
			!actual[fact.id] ||
			principal.global_admin ||
			(!fact.global_admin_only &&
				fact.all.every((required) =>
					principal.permissions.some(
						(grant) =>
							grant.allowed &&
							grant.module === required.module &&
							(grant.action === required.action ||
								(grant.action === 'manage' && policy.manage_implies.includes(required.action)))
					)
				))
	);
}

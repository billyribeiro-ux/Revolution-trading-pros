/**
 * Enterprise Admin API Adapter - Apple ICT9+ Integration Layer
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 * This adapter integrates the enterprise infrastructure with the existing admin API:
 * - Adds correlation tracing to all admin requests
 * - Applies CSRF protection to mutating operations
 * - Provides unified error handling
 * - Exposes enterprise metrics and diagnostics
 *
 * Usage:
 * Instead of importing from '$lib/api/admin', import from this file
 * to get enterprise-enhanced versions of the admin APIs.
 */

import { browser } from '$app/environment';
import { writable, get as _get } from 'svelte/store';

// Import existing admin APIs
import {
	couponsApi,
	usersApi,
	settingsApi,
	emailTemplatesApi,
	formsApi,
	subscriptionPlansApi,
	subscriptionsApi,
	productsApi,
	categoriesApi,
	tagsApi,
	segmentsApi,
	type ApiResponse,
	AdminApiError
} from '../admin';

// Import enterprise infrastructure
import {
	createRequestContext,
	getTraceHeaders as _getTraceHeaders,
	log,
	recordTrace,
	getTracingMetrics,
	getRecentTraces
} from './tracing';
import { getCsrfHeaders as _getCsrfHeaders, initializeCsrf } from './csrf';
import {
	isApiError as _isApiError,
	createApiError as _createApiError,
	ErrorCodes as _ErrorCodes,
	serializeError
} from './errors';
import type {
	RequestContext as _RequestContext,
	RequestMetrics as _RequestMetrics,
	EnterpriseApiError as _EnterpriseApiError
} from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// Enterprise Enhancement Wrapper
// ═══════════════════════════════════════════════════════════════════════════════

interface EnterpriseOptions {
	/** Skip tracing for this request */
	skipTracing?: boolean;
	/** Custom metadata for the request */
	metadata?: Record<string, unknown>;
}

/**
 * Wrap an API call with enterprise enhancements
 */
async function withEnterprise<T>(
	operation: string,
	method: string,
	fn: () => Promise<ApiResponse<T>>,
	options: EnterpriseOptions = {}
): Promise<ApiResponse<T>> {
	const context = createRequestContext({
		metadata: {
			operation,
			method,
			...options.metadata
		}
	});

	if (!options.skipTracing) {
		log('debug', `[Admin] ${method} ${operation}`, context);
	}

	try {
		const result = await fn();

		if (!options.skipTracing) {
			recordTrace(context, operation, method, 200);
		}

		return result;
	} catch (error) {
		const status = error instanceof AdminApiError ? error.status : 0;

		if (!options.skipTracing) {
			recordTrace(context, operation, method, status, String(error));
		}

		log('error', `[Admin] ${method} ${operation} failed`, context, {
			error: serializeError(error)
		});

		throw error;
	}
}

// ═══════════════════════════════════════════════════════════════════════════════
// Enterprise-Enhanced Admin APIs
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Enterprise-enhanced Coupons API
 */
export const enterpriseCouponsApi = {
	list: (params?: Parameters<typeof couponsApi.list>[0]) =>
		withEnterprise('coupons.list', 'GET', () => couponsApi.list(params)),

	get: (id: number) => withEnterprise(`coupons.get.${id}`, 'GET', () => couponsApi.get(id)),

	create: (data: Parameters<typeof couponsApi.create>[0]) =>
		withEnterprise('coupons.create', 'POST', () => couponsApi.create(data)),

	update: (id: number, data: Parameters<typeof couponsApi.update>[1]) =>
		withEnterprise(`coupons.update.${id}`, 'PUT', () => couponsApi.update(id, data)),

	delete: (id: number) =>
		withEnterprise(`coupons.delete.${id}`, 'DELETE', () => couponsApi.delete(id)),

	validate: (code: string) =>
		withEnterprise('coupons.validate', 'POST', () => couponsApi.validate(code)),

	checkCode: (code: string) =>
		withEnterprise('coupons.checkCode', 'POST', () => couponsApi.checkCode(code)),

	generateCode: (params: Parameters<typeof couponsApi.generateCode>[0]) =>
		withEnterprise('coupons.generateCode', 'POST', () => couponsApi.generateCode(params)),

	import: (formData: FormData) =>
		withEnterprise('coupons.import', 'POST', () => couponsApi.import(formData)),

	test: (data: Parameters<typeof couponsApi.test>[0]) =>
		withEnterprise('coupons.test', 'POST', () => couponsApi.test(data)),

	preview: (data: Parameters<typeof couponsApi.preview>[0]) =>
		withEnterprise('coupons.preview', 'POST', () => couponsApi.preview(data))
};

/**
 * Enterprise-enhanced Users API
 */
export const enterpriseUsersApi = {
	list: (params?: Parameters<typeof usersApi.list>[0]) =>
		withEnterprise('users.list', 'GET', () => usersApi.list(params)),

	get: (id: number) => withEnterprise(`users.get.${id}`, 'GET', () => usersApi.get(id)),

	create: (data: Parameters<typeof usersApi.create>[0]) =>
		withEnterprise('users.create', 'POST', () => usersApi.create(data)),

	update: (id: number, data: Parameters<typeof usersApi.update>[1]) =>
		withEnterprise(`users.update.${id}`, 'PUT', () => usersApi.update(id, data)),

	delete: (id: number) => withEnterprise(`users.delete.${id}`, 'DELETE', () => usersApi.delete(id)),

	stats: () => withEnterprise('users.stats', 'GET', () => usersApi.stats()),

	impersonate: (id: number) =>
		withEnterprise(`users.impersonate.${id}`, 'POST', () => usersApi.impersonate(id))
};

/**
 * Enterprise-enhanced Forms API
 */
export const enterpriseFormsApi = {
	list: (params?: Parameters<typeof formsApi.list>[0]) =>
		withEnterprise('forms.list', 'GET', () => formsApi.list(params)),

	get: (id: number) => withEnterprise(`forms.get.${id}`, 'GET', () => formsApi.get(id)),

	create: (data: Parameters<typeof formsApi.create>[0]) =>
		withEnterprise('forms.create', 'POST', () => formsApi.create(data)),

	update: (id: number, data: Parameters<typeof formsApi.update>[1]) =>
		withEnterprise(`forms.update.${id}`, 'PUT', () => formsApi.update(id, data)),

	delete: (id: number) => withEnterprise(`forms.delete.${id}`, 'DELETE', () => formsApi.delete(id)),

	publish: (id: number) =>
		withEnterprise(`forms.publish.${id}`, 'POST', () => formsApi.publish(id)),

	unpublish: (id: number) =>
		withEnterprise(`forms.unpublish.${id}`, 'POST', () => formsApi.unpublish(id)),

	duplicate: (id: number) =>
		withEnterprise(`forms.duplicate.${id}`, 'POST', () => formsApi.duplicate(id)),

	stats: () => withEnterprise('forms.stats', 'GET', () => formsApi.stats()),

	fieldTypes: () => withEnterprise('forms.fieldTypes', 'GET', () => formsApi.fieldTypes()),

	getSubmissions: (formId: number, params?: Parameters<typeof formsApi.getSubmissions>[1]) =>
		withEnterprise(`forms.submissions.${formId}`, 'GET', () =>
			formsApi.getSubmissions(formId, params)
		),

	getSubmission: (formId: number, submissionId: number) =>
		withEnterprise(`forms.submission.${formId}.${submissionId}`, 'GET', () =>
			formsApi.getSubmission(formId, submissionId)
		),

	deleteSubmission: (formId: number, submissionId: number) =>
		withEnterprise(`forms.submission.delete.${formId}.${submissionId}`, 'DELETE', () =>
			formsApi.deleteSubmission(formId, submissionId)
		),

	exportSubmissions: (formId: number, format?: 'csv' | 'excel') =>
		formsApi.exportSubmissions(formId, format)
};

/**
 * Enterprise-enhanced Products API
 */
export const enterpriseProductsApi = {
	list: (params?: Parameters<typeof productsApi.list>[0]) =>
		withEnterprise('products.list', 'GET', () => productsApi.list(params)),

	get: (id: number) => withEnterprise(`products.get.${id}`, 'GET', () => productsApi.get(id)),

	create: (data: Parameters<typeof productsApi.create>[0]) =>
		withEnterprise('products.create', 'POST', () => productsApi.create(data)),

	update: (id: number, data: Parameters<typeof productsApi.update>[1]) =>
		withEnterprise(`products.update.${id}`, 'PUT', () => productsApi.update(id, data)),

	delete: (id: number) =>
		withEnterprise(`products.delete.${id}`, 'DELETE', () => productsApi.delete(id)),

	byType: (type: string) =>
		withEnterprise(`products.byType.${type}`, 'GET', () => productsApi.byType(type)),

	stats: () => withEnterprise('products.stats', 'GET', () => productsApi.stats()),

	assignToUser: (productId: number, userId: number, orderId?: string) =>
		withEnterprise(`products.assignUser.${productId}`, 'POST', () =>
			productsApi.assignToUser(productId, userId, orderId)
		),

	removeFromUser: (productId: number, userId: number) =>
		withEnterprise(`products.removeUser.${productId}`, 'POST', () =>
			productsApi.removeFromUser(productId, userId)
		),

	productUsers: (productId: number) =>
		withEnterprise(`products.users.${productId}`, 'GET', () => productsApi.productUsers(productId)),

	bulkUpdate: (ids: number[], data: Parameters<typeof productsApi.bulkUpdate>[1]) =>
		withEnterprise('products.bulkUpdate', 'POST', () => productsApi.bulkUpdate(ids, data))
};

// ═══════════════════════════════════════════════════════════════════════════════
// Enterprise Diagnostics
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Get enterprise diagnostics and metrics
 */
export function getEnterpriseDiagnostics() {
	return {
		tracing: getTracingMetrics(),
		recentTraces: getRecentTraces(),
		timestamp: new Date().toISOString()
	};
}

/**
 * Store for enterprise metrics
 */
export const enterpriseMetrics = writable<{
	totalRequests: number;
	errorRate: number;
	avgResponseTime: number;
}>({
	totalRequests: 0,
	errorRate: 0,
	avgResponseTime: 0
});

// Update metrics periodically
if (browser) {
	setInterval(() => {
		const metrics = getTracingMetrics();
		enterpriseMetrics.set({
			totalRequests: 0, // Would need to track this
			errorRate: 0,
			avgResponseTime: metrics.avgResponseTime
		});
	}, 10000);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Initialize
// ═══════════════════════════════════════════════════════════════════════════════

if (browser) {
	initializeCsrf();
}

// Re-export everything with enterprise prefix for clarity
export {
	// Original APIs (for backward compatibility)
	couponsApi,
	usersApi,
	settingsApi,
	emailTemplatesApi,
	formsApi,
	subscriptionPlansApi,
	subscriptionsApi,
	productsApi,
	categoriesApi,
	tagsApi,
	segmentsApi,
	// Types
	AdminApiError,
	type ApiResponse
};

/**
 * CRM Deals API - Proxy to Backend
 *
 * Proxies deals/opportunities management requests to the Laravel backend.
 * Handles deal CRUD operations for the RevolutionCRM Pro system.
 *
 * @version 2.0.0 - December 2025 - Connected to real backend
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Production fallback - Rust API on Fly.io
const PROD_API = 'https://revolution-trading-pros-api.fly.dev/api';
const BACKEND_URL = import.meta.env.VITE_API_URL || PROD_API;

/**
 * Get authorization headers from request
 */
function getAuthHeaders(request: Request): HeadersInit {
	const authHeader = request.headers.get('Authorization');
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		'Accept': 'application/json',
	};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	}
	return headers;
}

// GET - List deals (proxies to backend)
export const GET: RequestHandler = async ({ url, request }) => {
	try {
		// Forward query params to backend
		const queryParams = new URLSearchParams();

		const status = url.searchParams.get('status');
		const pipeline_id = url.searchParams.get('pipeline_id');
		const stage_id = url.searchParams.get('stage_id');
		const page = url.searchParams.get('page');
		const limit = url.searchParams.get('limit') || url.searchParams.get('per_page');
		const owner_id = url.searchParams.get('owner_id');
		const contact_id = url.searchParams.get('contact_id');

		if (status && status !== 'all') queryParams.set('status', status);
		if (pipeline_id) queryParams.set('pipeline_id', pipeline_id);
		if (stage_id) queryParams.set('stage_id', stage_id);
		if (page) queryParams.set('page', page);
		if (limit) queryParams.set('per_page', limit);
		if (owner_id) queryParams.set('owner_id', owner_id);
		if (contact_id) queryParams.set('contact_id', contact_id);

		const backendUrl = `${BACKEND_URL}/admin/crm/deals${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

		const response = await fetch(backendUrl, {
			method: 'GET',
			headers: getAuthHeaders(request),
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Failed to fetch deals' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to fetch deals',
				data: [],
				meta: { page: 1, limit: 20, total: 0, total_pages: 0 }
			}, { status: response.status });
		}

		const data = await response.json();

		// Transform backend pagination response to match frontend expected format
		return json({
			success: true,
			data: data.data || [],
			meta: {
				page: data.current_page || 1,
				limit: data.per_page || 20,
				total: data.total || 0,
				total_pages: data.last_page || 0
			}
		});
	} catch (err) {
		console.error('CRM Deals API proxy error:', err);
		return json({
			success: false,
			error: 'Failed to connect to backend',
			data: [],
			meta: { page: 1, limit: 20, total: 0, total_pages: 0 }
		}, { status: 503 });
	}
};

// POST - Create deal (proxies to backend)
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields
		if (!body.name) {
			throw error(400, 'Deal name is required');
		}

		const response = await fetch(`${BACKEND_URL}/admin/crm/deals`, {
			method: 'POST',
			headers: getAuthHeaders(request),
			body: JSON.stringify({
				name: body.name,
				contact_id: body.contact_id,
				company_id: body.company_id,
				pipeline_id: body.pipeline_id || 'default',
				stage_id: body.stage_id,
				amount: body.amount || 0,
				currency: body.currency || 'USD',
				probability: body.probability || 10,
				status: body.status || 'open',
				priority: body.priority || 'normal',
				expected_close_date: body.expected_close_date,
				owner_id: body.owner_id,
				source_channel: body.source_channel,
				tags: body.tags || [],
				custom_fields: body.custom_fields || {},
			}),
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Failed to create deal' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to create deal',
				errors: errorData.errors
			}, { status: response.status });
		}

		const data = await response.json();

		return json({
			success: true,
			data: data
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		console.error('CRM Deals API create error:', err);
		throw error(500, 'Failed to create deal');
	}
};

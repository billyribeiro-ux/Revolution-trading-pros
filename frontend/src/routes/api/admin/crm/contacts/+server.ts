/**
 * CRM Contacts API - Proxy to Backend
 *
 * Proxies contact management requests to the Rust API backend.
 * Handles contact CRUD operations for the RevolutionCRM Pro system.
 *
 * @version 2.0.0 - December 2025 - Connected to real backend
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';


// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = PROD_BACKEND;

/**
 * Get authorization headers from request
 */
function getAuthHeaders(request: Request): HeadersInit {
	const authHeader = request.headers.get('Authorization');
	const headers: HeadersInit = {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	};
	if (authHeader) {
		headers['Authorization'] = authHeader;
	}
	return headers;
}

// GET - List contacts (proxies to backend)
export const GET: RequestHandler = async ({ url, request }) => {
	try {
		// Forward query params to backend
		const queryParams = new URLSearchParams();

		const status = url.searchParams.get('status');
		const search = url.searchParams.get('search');
		const page = url.searchParams.get('page');
		const limit = url.searchParams.get('limit') || url.searchParams.get('per_page');
		const lifecycle_stage = url.searchParams.get('lifecycle_stage');
		const owner_id = url.searchParams.get('owner_id');
		const sort_by = url.searchParams.get('sort_by');
		const sort_order = url.searchParams.get('sort_order');

		if (status && status !== 'all') queryParams.set('status', status);
		if (search) queryParams.set('search', search);
		if (page) queryParams.set('page', page);
		if (limit) queryParams.set('per_page', limit);
		if (lifecycle_stage) queryParams.set('lifecycle_stage', lifecycle_stage);
		if (owner_id) queryParams.set('owner_id', owner_id);
		if (sort_by) queryParams.set('sort_by', sort_by);
		if (sort_order) queryParams.set('sort_order', sort_order);

		const backendUrl = `${BACKEND_URL}/api/admin/crm/contacts${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

		const response = await fetch(backendUrl, {
			method: 'GET',
			headers: getAuthHeaders(request)
		});

		if (!response.ok) {
			const errorData = await response
				.json()
				.catch(() => ({ message: 'Failed to fetch contacts' }));
			return json(
				{
					success: false,
					error: errorData.message || 'Failed to fetch contacts',
					data: [],
					meta: { page: 1, limit: 20, total: 0, total_pages: 0 }
				},
				{ status: response.status }
			);
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
		console.error('CRM Contacts API proxy error:', err);
		return json(
			{
				success: false,
				error: 'Failed to connect to backend',
				data: [],
				meta: { page: 1, limit: 20, total: 0, total_pages: 0 }
			},
			{ status: 503 }
		);
	}
};

// POST - Create contact (proxies to backend)
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields
		if (!body.email) {
			throw error(400, 'Email is required');
		}

		const response = await fetch(`${BACKEND_URL}/api/admin/crm/contacts`, {
			method: 'POST',
			headers: getAuthHeaders(request),
			body: JSON.stringify({
				email: body.email,
				first_name: body.first_name || body.name?.split(' ')[0] || '',
				last_name: body.last_name || body.name?.split(' ').slice(1).join(' ') || '',
				phone: body.phone,
				job_title: body.job_title,
				source: body.source || 'manual',
				owner_id: body.owner_id,
				status: body.status || 'lead'
			})
		});

		if (!response.ok) {
			const errorData = await response
				.json()
				.catch(() => ({ message: 'Failed to create contact' }));

			// Handle duplicate email error
			if (response.status === 422 && errorData.errors?.email) {
				throw error(409, 'Contact with this email already exists');
			}

			return json(
				{
					success: false,
					error: errorData.message || 'Failed to create contact',
					errors: errorData.errors
				},
				{ status: response.status }
			);
		}

		const data = await response.json();

		return json(
			{
				success: true,
				data: data
			},
			{ status: 201 }
		);
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		console.error('CRM Contacts API create error:', err);
		throw error(500, 'Failed to create contact');
	}
};

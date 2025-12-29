/**
 * Dashboard Memberships API - Proxy to Backend
 *
 * Proxies membership requests to the Rust API backend.
 * Handles user memberships, access levels, and subscription status.
 *
 * @version 2.0.0 - December 2025 - Connected to real backend
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

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

// GET - List user memberships (proxies to backend)
export const GET: RequestHandler = async ({ url, request }) => {
	try {
		// Check for admin endpoint or user endpoint
		const isAdmin = url.searchParams.get('admin') === 'true';

		// Forward query params to backend
		const queryParams = new URLSearchParams();

		const page = url.searchParams.get('page');
		const limit = url.searchParams.get('limit') || url.searchParams.get('per_page');
		const status = url.searchParams.get('status');
		const type = url.searchParams.get('type');

		if (page) queryParams.set('page', page);
		if (limit) queryParams.set('per_page', limit);
		if (status) queryParams.set('status', status);
		if (type) queryParams.set('type', type);

		// Use admin products endpoint for admin or user memberships endpoint for regular users
		let backendUrl: string;
		if (isAdmin) {
			// Admin: Get all membership products
			queryParams.set('type', 'membership');
			backendUrl = `${BACKEND_URL}/admin/products${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
		} else {
			// User: Get user's memberships
			backendUrl = `${BACKEND_URL}/me/memberships${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
		}

		const response = await fetch(backendUrl, {
			method: 'GET',
			headers: getAuthHeaders(request),
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Failed to fetch memberships' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to fetch memberships',
				data: {
					memberships: [],
					user_memberships: [],
					pagination: { page: 1, limit: 12, total: 0, total_pages: 0 }
				}
			}, { status: response.status });
		}

		const data = await response.json();

		// Transform response based on endpoint type
		if (isAdmin) {
			// Admin response - product list
			const memberships = (data.data || []).map((item: any) => ({
				id: item.id,
				slug: item.slug,
				name: item.name,
				description: item.description || '',
				short_description: item.short_description || '',
				thumbnail_url: item.thumbnail_url || item.image_url || '',
				banner_url: item.banner_url,
				tier: item.tier || 'basic',
				category: item.category || 'membership',
				features: item.features || [],
				price_monthly: item.price_monthly,
				price_annual: item.price_annual,
				price_lifetime: item.price_lifetime,
				has_trading_room: item.has_trading_room ?? false,
				has_alerts: item.has_alerts ?? false,
				has_learning_center: item.has_learning_center ?? false,
				has_daily_videos: item.has_daily_videos ?? false,
				instructor: item.instructor,
				trading_room_schedule: item.trading_room_schedule,
				is_active: item.is_active ?? true,
				is_featured: item.is_featured ?? false,
				sort_order: item.sort_order || 0,
				created_at: item.created_at,
				updated_at: item.updated_at
			}));

			return json({
				success: true,
				data: {
					memberships,
					pagination: {
						page: data.current_page || 1,
						limit: data.per_page || 12,
						total: data.total || 0,
						total_pages: data.last_page || 0
					}
				}
			});
		} else {
			// User response - their memberships
			const userMemberships = Array.isArray(data) ? data : (data.data || []);

			return json({
				success: true,
				data: {
					user_memberships: userMemberships.map((item: any) => ({
						id: item.id,
						user_id: item.user_id,
						membership_id: item.membership_id || item.product_id,
						membership: item.membership || item.product || {},
						status: item.status || 'active',
						subscription_type: item.subscription_type || item.billing_cycle || 'monthly',
						started_at: item.started_at || item.created_at,
						expires_at: item.expires_at || item.end_date,
						cancelled_at: item.cancelled_at,
						auto_renew: item.auto_renew ?? true,
						payment_method: item.payment_method
					}))
				}
			});
		}
	} catch (err) {
		console.error('Memberships API proxy error:', err);
		return json({
			success: false,
			error: 'Failed to connect to backend',
			data: {
				memberships: [],
				user_memberships: [],
				pagination: { page: 1, limit: 12, total: 0, total_pages: 0 }
			}
		}, { status: 503 });
	}
};

// POST - Create membership product (admin) or assign membership (user)
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Determine if this is admin creating product or assigning membership
		const isAssignment = body.user_id && body.membership_id;

		let backendUrl: string;
		let requestBody: Record<string, any>;

		if (isAssignment) {
			// Assign membership to user
			backendUrl = `${BACKEND_URL}/admin/products/${body.membership_id}/assign-user`;
			requestBody = {
				user_id: body.user_id,
				subscription_type: body.subscription_type || 'monthly',
				auto_renew: body.auto_renew ?? true
			};
		} else {
			// Create new membership product
			if (!body.name) {
				throw error(400, 'Membership name is required');
			}

			backendUrl = `${BACKEND_URL}/admin/products`;
			requestBody = {
				name: body.name,
				slug: body.slug || body.name.toLowerCase().replace(/[^a-z0-9]+/g, '-'),
				type: 'membership',
				description: body.description || '',
				short_description: body.short_description || '',
				thumbnail_url: body.thumbnail_url || '',
				tier: body.tier || 'basic',
				category: body.category || 'membership',
				features: body.features || [],
				price_monthly: body.price_monthly,
				price_annual: body.price_annual,
				price_lifetime: body.price_lifetime,
				has_trading_room: body.has_trading_room ?? false,
				has_alerts: body.has_alerts ?? false,
				has_learning_center: body.has_learning_center ?? false,
				has_daily_videos: body.has_daily_videos ?? false,
				instructor_id: body.instructor_id,
				is_active: body.is_active ?? true,
				is_featured: body.is_featured ?? false,
				sort_order: body.sort_order ?? 0
			};
		}

		const response = await fetch(backendUrl, {
			method: 'POST',
			headers: getAuthHeaders(request),
			body: JSON.stringify(requestBody),
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({ message: 'Failed to process membership request' }));
			return json({
				success: false,
				error: errorData.message || 'Failed to process membership request',
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
		console.error('Memberships API create error:', err);
		throw error(500, 'Failed to process membership request');
	}
};

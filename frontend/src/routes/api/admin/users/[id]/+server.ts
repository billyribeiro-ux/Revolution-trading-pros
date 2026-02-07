/**
 * Single User API Endpoint
 *
 * Handles individual user retrieval, update, and deletion with backend fallback.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

// Mock users data (same as in +server.ts for consistency)
const mockUsers: Record<number, any> = {
	1: {
		id: 1,
		name: 'Admin User',
		first_name: 'Admin',
		last_name: 'User',
		email: 'admin@revolutiontrading.com',
		email_verified_at: '2025-01-01T00:00:00Z',
		roles: [{ name: 'super-admin' }, { name: 'admin' }],
		is_active: true,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	2: {
		id: 2,
		name: 'John Doe',
		first_name: 'John',
		last_name: 'Doe',
		email: 'john@example.com',
		email_verified_at: '2025-06-15T00:00:00Z',
		roles: [{ name: 'member' }],
		is_active: true,
		created_at: '2025-06-15T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	3: {
		id: 3,
		name: 'Jane Smith',
		first_name: 'Jane',
		last_name: 'Smith',
		email: 'jane@example.com',
		email_verified_at: '2025-07-20T00:00:00Z',
		roles: [{ name: 'member' }],
		is_active: true,
		created_at: '2025-07-20T00:00:00Z',
		updated_at: '2025-11-15T00:00:00Z'
	}
};

// Try to fetch from backend
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	const BACKEND_URL = PROD_BACKEND;
	if (!BACKEND_URL) return null;

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				...options?.headers
			}
		});

		if (!response.ok) {
			// If 404, return null to use mock data
			if (response.status === 404) return null;
			// For other errors, also return null
			return null;
		}
		return await response.json();
	} catch (_err) {
		return null;
	}
}

// GET - Get single user
export const GET: RequestHandler = async ({ params, request }) => {
	const userId = parseInt(params.id ?? '0');

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/users/${userId}`, {
		headers: { Authorization: request.headers.get('Authorization') || '' }
	});

	if (backendData?.data) {
		return json(backendData);
	}

	// Fallback to mock data
	const user = mockUsers[userId];

	if (!user) {
		// Return a generic mock user for any ID
		return json({
			data: {
				id: userId,
				name: 'User',
				first_name: 'User',
				last_name: '',
				email: `user${userId}@revolutiontrading.com`,
				email_verified_at: new Date().toISOString(),
				roles: [{ name: 'member' }],
				is_active: true,
				created_at: new Date().toISOString(),
				updated_at: new Date().toISOString()
			},
			_mock: true,
			_message: 'Using mock data. Backend not available.'
		});
	}

	return json({
		data: user,
		_mock: true
	});
};

// PUT - Update user
export const PUT: RequestHandler = async ({ params, request }) => {
	const userId = parseInt(params.id ?? '0');
	const body = await request.json();

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/users/${userId}`, {
		method: 'PUT',
		headers: { Authorization: request.headers.get('Authorization') || '' },
		body: JSON.stringify(body)
	});

	if (backendData?.data) {
		return json(backendData);
	}

	// Mock update
	const existingUser = mockUsers[userId] || {
		id: userId,
		email: body.email || `user${userId}@example.com`,
		created_at: new Date().toISOString()
	};

	const updatedUser = {
		...existingUser,
		...body,
		id: userId,
		roles: body.roles?.map((r: string) => ({ name: r })) || existingUser.roles || [],
		updated_at: new Date().toISOString()
	};

	// Update mock data
	mockUsers[userId] = updatedUser;

	return json({
		data: updatedUser,
		message: 'User updated successfully',
		_mock: true
	});
};

// DELETE - Delete user
export const DELETE: RequestHandler = async ({ params, request }) => {
	const userId = parseInt(params.id ?? '0');

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/users/${userId}`, {
		method: 'DELETE',
		headers: { Authorization: request.headers.get('Authorization') || '' }
	});

	if (backendData?.success !== undefined) {
		return json(backendData);
	}

	// Mock delete
	if (mockUsers[userId]) {
		delete mockUsers[userId];
	}

	return json({
		success: true,
		message: 'User deleted successfully',
		_mock: true
	});
};

/**
 * Users API Endpoint
 *
 * Handles user listing with backend fallback to mock data.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Mock users data
const mockUsers = [
	{
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
	{
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
	{
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
];

// Try to fetch from backend
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	const BACKEND_URL = env.BACKEND_URL;
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

		if (!response.ok) return null;
		return await response.json();
	} catch (error) {
		return null;
	}
}

// GET - List users
export const GET: RequestHandler = async ({ url, request }) => {
	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/users?${url.searchParams.toString()}`, {
		headers: { Authorization: request.headers.get('Authorization') || '' }
	});

	if (backendData?.data) {
		return json(backendData);
	}

	// Fallback to mock data
	return json({
		data: mockUsers,
		meta: {
			total: mockUsers.length,
			page: 1,
			per_page: 20,
			last_page: 1
		},
		_mock: true
	});
};

// POST - Create user
export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/users', {
		method: 'POST',
		headers: { Authorization: request.headers.get('Authorization') || '' },
		body: JSON.stringify(body)
	});

	if (backendData?.data) {
		return json(backendData);
	}

	// Mock response
	const newUser = {
		id: mockUsers.length + 1,
		...body,
		email_verified_at: null,
		roles: body.roles?.map((r: string) => ({ name: r })) || [],
		is_active: true,
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	return json({
		data: newUser,
		message: 'User created successfully',
		_mock: true
	});
};

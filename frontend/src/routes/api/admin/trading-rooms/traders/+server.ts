/**
 * Traders API Endpoint
 *
 * Handles trader management for trading rooms.
 * Returns mock data when backend is not connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

// Mock traders data
const mockTraders = [
	{
		id: 1,
		name: 'Mike Thompson',
		slug: 'mike-thompson',
		title: 'Head Day Trader',
		bio: 'Over 15 years of trading experience specializing in momentum day trading.',
		photo_url: null,
		email: 'mike@revolutiontrading.com',
		social_links: { twitter: '@miketrading' },
		specialties: ['Day Trading', 'Momentum', 'Options'],
		is_active: true,
		sort_order: 1,
		daily_videos_count: 45,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 2,
		name: 'Sarah Chen',
		slug: 'sarah-chen',
		title: 'Senior Swing Trader',
		bio: 'Expert in technical analysis and swing trading strategies.',
		photo_url: null,
		email: 'sarah@revolutiontrading.com',
		social_links: { twitter: '@sarahtrading' },
		specialties: ['Swing Trading', 'Technical Analysis', 'Charts'],
		is_active: true,
		sort_order: 2,
		daily_videos_count: 32,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 3,
		name: 'James Wilson',
		slug: 'james-wilson',
		title: 'Options Specialist',
		bio: 'SPX and options trading expert with focus on risk management.',
		photo_url: null,
		email: 'james@revolutiontrading.com',
		social_links: { twitter: '@jamesoptions' },
		specialties: ['Options', 'SPX', 'Risk Management'],
		is_active: true,
		sort_order: 3,
		daily_videos_count: 28,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 4,
		name: 'Emily Davis',
		slug: 'emily-davis',
		title: 'Small Account Mentor',
		bio: 'Specializes in growing small accounts with disciplined trading.',
		photo_url: null,
		email: 'emily@revolutiontrading.com',
		social_links: { twitter: '@emilytrading' },
		specialties: ['Small Accounts', 'Position Sizing', 'Discipline'],
		is_active: true,
		sort_order: 4,
		daily_videos_count: 22,
		created_at: '2025-01-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	}
];

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

		if (!response.ok) return null;
		return await response.json();
	} catch (_error) {
		return null;
	}
}

// GET - List traders
export const GET: RequestHandler = async ({ url, request }) => {
	const activeOnly = url.searchParams.get('active_only') === 'true';

	// Try backend first
	const backendData = await fetchFromBackend(
		`/api/admin/trading-rooms/traders?${url.searchParams.toString()}`,
		{
			headers: { Authorization: request.headers.get('Authorization') || '' }
		}
	);

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	let filteredTraders = [...mockTraders];

	if (activeOnly) {
		filteredTraders = filteredTraders.filter((t) => t.is_active);
	}

	filteredTraders.sort((a, b) => a.sort_order - b.sort_order);

	return json({
		success: true,
		data: filteredTraders,
		_mock: true
	});
};

// POST - Create trader
export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/trading-rooms/traders', {
		method: 'POST',
		headers: { Authorization: request.headers.get('Authorization') || '' },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock response
	const newTrader = {
		id: mockTraders.length + 1,
		...body,
		slug: body.slug || body.name.toLowerCase().replace(/\s+/g, '-'),
		is_active: body.is_active ?? true,
		sort_order: mockTraders.length + 1,
		daily_videos_count: 0,
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	mockTraders.push(newTrader);

	return json({
		success: true,
		data: newTrader,
		_mock: true
	});
};

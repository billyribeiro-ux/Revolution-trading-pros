/**
 * Trading Rooms API Endpoint
 *
 * Handles trading room management. Returns mock data when backend is not connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { BACKEND_URL } from '$env/static/private';

// Mock trading rooms data (matches the migration seed data)
const mockTradingRooms = [
	{
		id: 1,
		name: 'Day Trading Room',
		slug: 'day-trading-room',
		type: 'trading_room',
		description: 'Live day trading room with real-time market analysis and trade alerts.',
		short_description: 'Live day trading with expert traders',
		icon: 'st-icon-day-trading',
		color: '#0984ae',
		image_url: null,
		logo_url: null,
		is_active: true,
		is_featured: true,
		sort_order: 1,
		features: ['Live Trading', 'Real-time Alerts', 'Daily Analysis', 'Q&A Sessions'],
		schedule: { market_open: '9:00 AM ET', market_close: '4:00 PM ET' },
		metadata: null,
		daily_videos_count: 45,
		learning_content_count: 12,
		archives_count: 180,
		created_at: '2025-12-08T00:00:00Z',
		updated_at: '2025-12-08T00:00:00Z'
	},
	{
		id: 2,
		name: 'Swing Trading Room',
		slug: 'swing-trading-room',
		type: 'trading_room',
		description: 'Swing trading strategies for multi-day to multi-week positions.',
		short_description: 'Multi-day swing trading strategies',
		icon: 'st-icon-swing-trading',
		color: '#10b981',
		image_url: null,
		logo_url: null,
		is_active: true,
		is_featured: true,
		sort_order: 2,
		features: ['Swing Setups', 'Weekly Analysis', 'Position Management', 'Trade Reviews'],
		schedule: { sessions: 'Daily market analysis + weekly planning' },
		metadata: null,
		daily_videos_count: 32,
		learning_content_count: 8,
		archives_count: 95,
		created_at: '2025-12-08T00:00:00Z',
		updated_at: '2025-12-08T00:00:00Z'
	},
	{
		id: 3,
		name: 'Small Account Mentorship',
		slug: 'small-account-mentorship',
		type: 'trading_room',
		description: 'Specialized mentorship for growing small trading accounts with disciplined strategies.',
		short_description: 'Grow your small account the right way',
		icon: 'st-icon-mentorship',
		color: '#f59e0b',
		image_url: null,
		logo_url: null,
		is_active: true,
		is_featured: true,
		sort_order: 3,
		features: ['Account Building', 'Risk Management', 'Position Sizing', '1-on-1 Guidance'],
		schedule: { sessions: 'Live sessions + on-demand content' },
		metadata: null,
		daily_videos_count: 28,
		learning_content_count: 15,
		archives_count: 60,
		created_at: '2025-12-08T00:00:00Z',
		updated_at: '2025-12-08T00:00:00Z'
	},
	{
		id: 4,
		name: 'SPX Profit Pulse',
		slug: 'spx-profit-pulse',
		type: 'alert_service',
		description: 'Real-time SPX trading alerts with entry, target, and stop levels.',
		short_description: 'SPX options trading alerts',
		icon: 'st-icon-alerts',
		color: '#ef4444',
		image_url: null,
		logo_url: null,
		is_active: true,
		is_featured: true,
		sort_order: 10,
		features: ['Real-time Alerts', 'Entry/Exit Levels', 'Risk Management', 'SMS & Email'],
		schedule: null,
		metadata: null,
		daily_videos_count: 15,
		learning_content_count: 5,
		archives_count: 0,
		created_at: '2025-12-08T00:00:00Z',
		updated_at: '2025-12-08T00:00:00Z'
	},
	{
		id: 5,
		name: 'Explosive Swing',
		slug: 'explosive-swing',
		type: 'alert_service',
		description: 'High-probability swing trade alerts for explosive moves.',
		short_description: 'Explosive swing trade alerts',
		icon: 'st-icon-explosive',
		color: '#8b5cf6',
		image_url: null,
		logo_url: null,
		is_active: true,
		is_featured: true,
		sort_order: 11,
		features: ['Swing Alerts', 'Technical Analysis', 'Weekly Watchlist', 'Trade Tracking'],
		schedule: null,
		metadata: null,
		daily_videos_count: 10,
		learning_content_count: 3,
		archives_count: 0,
		created_at: '2025-12-08T00:00:00Z',
		updated_at: '2025-12-08T00:00:00Z'
	}
];

// Try to fetch from backend, fallback to mock data
async function fetchFromBackend(endpoint: string, options?: RequestInit): Promise<any | null> {
	if (!BACKEND_URL) return null;

	try {
		const response = await fetch(`${BACKEND_URL}${endpoint}`, {
			...options,
			headers: {
				'Content-Type': 'application/json',
				'Accept': 'application/json',
				...options?.headers
			}
		});

		if (!response.ok) {
			console.warn(`Backend returned ${response.status} for ${endpoint}`);
			return null;
		}

		return await response.json();
	} catch (error) {
		console.warn(`Failed to fetch from backend: ${error}`);
		return null;
	}
}

// GET - List trading rooms
export const GET: RequestHandler = async ({ url, request }) => {
	const type = url.searchParams.get('type');
	const activeOnly = url.searchParams.get('active_only') === 'true';
	const withCounts = url.searchParams.get('with_counts') === 'true';

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/trading-rooms?${url.searchParams.toString()}`, {
		headers: { Authorization: request.headers.get('Authorization') || '' }
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	let filteredRooms = [...mockTradingRooms];

	if (type) {
		filteredRooms = filteredRooms.filter(r => r.type === type);
	}

	if (activeOnly) {
		filteredRooms = filteredRooms.filter(r => r.is_active);
	}

	filteredRooms.sort((a, b) => a.sort_order - b.sort_order);

	return json({
		success: true,
		data: filteredRooms,
		_mock: true,
		_message: 'Using mock data. Run database migrations to enable full functionality.'
	});
};

// POST - Create trading room
export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/trading-rooms', {
		method: 'POST',
		headers: { Authorization: request.headers.get('Authorization') || '' },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Mock response
	const newRoom = {
		id: mockTradingRooms.length + 1,
		...body,
		slug: body.slug || body.name.toLowerCase().replace(/\s+/g, '-'),
		is_active: body.is_active ?? true,
		is_featured: body.is_featured ?? false,
		sort_order: mockTradingRooms.length + 1,
		daily_videos_count: 0,
		learning_content_count: 0,
		archives_count: 0,
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	mockTradingRooms.push(newRoom);

	return json({
		success: true,
		data: newRoom,
		_mock: true
	});
};

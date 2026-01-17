/**
 * Admin Schedules API Endpoint
 * ═══════════════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 11+ Grade - January 2026
 *
 * Handles trading room schedule management with full CRUD operations.
 * Returns mock data when backend is not connected.
 *
 * @version 1.0.0
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// ═══════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════

interface ScheduleEvent {
	id: number;
	room_id: string;
	title: string;
	description: string | null;
	trader_name: string | null;
	day_of_week: number;
	start_time: string;
	end_time: string;
	timezone: string;
	is_active: boolean;
	room_type: 'live' | 'recorded' | 'hybrid';
	recurrence: 'weekly' | 'biweekly' | 'monthly' | null;
	exceptions: any[];
	created_at: string;
	updated_at: string;
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK DATA
// ═══════════════════════════════════════════════════════════════════════════

const mockSchedules: ScheduleEvent[] = [
	// Day Trading Room Schedules
	{
		id: 1,
		room_id: 'day-trading-room',
		title: 'Pre-Market Analysis',
		description: 'Daily pre-market analysis and game plan for the trading day.',
		trader_name: 'Taylor Horton',
		day_of_week: 1, // Monday
		start_time: '08:30',
		end_time: '09:15',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 2,
		room_id: 'day-trading-room',
		title: 'Morning Trading Session',
		description: 'Live trading during market open with real-time entries and exits.',
		trader_name: 'Taylor Horton',
		day_of_week: 1,
		start_time: '09:30',
		end_time: '11:30',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 3,
		room_id: 'day-trading-room',
		title: 'Pre-Market Analysis',
		description: 'Daily pre-market analysis and game plan.',
		trader_name: 'Taylor Horton',
		day_of_week: 2, // Tuesday
		start_time: '08:30',
		end_time: '09:15',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 4,
		room_id: 'day-trading-room',
		title: 'Morning Trading Session',
		description: 'Live trading during market open.',
		trader_name: 'Taylor Horton',
		day_of_week: 2,
		start_time: '09:30',
		end_time: '11:30',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 5,
		room_id: 'day-trading-room',
		title: 'Pre-Market Analysis',
		description: 'Daily pre-market analysis and game plan.',
		trader_name: 'Taylor Horton',
		day_of_week: 3, // Wednesday
		start_time: '08:30',
		end_time: '09:15',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 6,
		room_id: 'day-trading-room',
		title: 'Morning Trading Session',
		description: 'Live trading during market open.',
		trader_name: 'Taylor Horton',
		day_of_week: 3,
		start_time: '09:30',
		end_time: '11:30',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 7,
		room_id: 'day-trading-room',
		title: 'Pre-Market Analysis',
		description: 'Daily pre-market analysis and game plan.',
		trader_name: 'Taylor Horton',
		day_of_week: 4, // Thursday
		start_time: '08:30',
		end_time: '09:15',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 8,
		room_id: 'day-trading-room',
		title: 'Morning Trading Session',
		description: 'Live trading during market open.',
		trader_name: 'Taylor Horton',
		day_of_week: 4,
		start_time: '09:30',
		end_time: '11:30',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 9,
		room_id: 'day-trading-room',
		title: 'Pre-Market Analysis',
		description: 'Daily pre-market analysis and game plan.',
		trader_name: 'Taylor Horton',
		day_of_week: 5, // Friday
		start_time: '08:30',
		end_time: '09:15',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 10,
		room_id: 'day-trading-room',
		title: 'Morning Trading Session',
		description: 'Live trading during market open.',
		trader_name: 'Taylor Horton',
		day_of_week: 5,
		start_time: '09:30',
		end_time: '11:30',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	// Swing Trading Room
	{
		id: 11,
		room_id: 'swing-trading-room',
		title: 'Weekly Swing Setup Review',
		description: 'Review of potential swing trade setups for the week.',
		trader_name: 'Michael Chen',
		day_of_week: 1,
		start_time: '16:00',
		end_time: '17:00',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 12,
		room_id: 'swing-trading-room',
		title: 'Mid-Week Position Update',
		description: 'Update on current swing positions and adjustments.',
		trader_name: 'Michael Chen',
		day_of_week: 3,
		start_time: '16:00',
		end_time: '16:45',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	// Small Account Mentorship
	{
		id: 13,
		room_id: 'small-account-mentorship',
		title: 'Small Account Strategy Session',
		description: 'Focused session on building small accounts with proper risk management.',
		trader_name: 'Sarah Johnson',
		day_of_week: 2,
		start_time: '19:00',
		end_time: '20:30',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	{
		id: 14,
		room_id: 'small-account-mentorship',
		title: 'Q&A and Trade Review',
		description: 'Member Q&A and review of trade ideas.',
		trader_name: 'Sarah Johnson',
		day_of_week: 4,
		start_time: '19:00',
		end_time: '20:00',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'live',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	// SPX Profit Pulse
	{
		id: 15,
		room_id: 'spx-profit-pulse',
		title: 'SPX Morning Prep',
		description: 'SPX levels and trade plan for the day.',
		trader_name: 'Alex Rivera',
		day_of_week: 1,
		start_time: '09:00',
		end_time: '09:25',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'recorded',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	},
	// Explosive Swings
	{
		id: 16,
		room_id: 'explosive-swings',
		title: 'Weekly Watchlist Release',
		description: 'New weekly watchlist with explosive swing candidates.',
		trader_name: 'David Park',
		day_of_week: 0, // Sunday
		start_time: '18:00',
		end_time: '19:00',
		timezone: 'America/New_York',
		is_active: true,
		room_type: 'recorded',
		recurrence: 'weekly',
		exceptions: [],
		created_at: '2025-12-01T00:00:00Z',
		updated_at: '2025-12-01T00:00:00Z'
	}
];

let scheduleIdCounter = mockSchedules.length + 1;

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND HELPER
// ═══════════════════════════════════════════════════════════════════════════

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

// ═══════════════════════════════════════════════════════════════════════════
// GET - List schedules (with optional room_id filter)
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ url, request }) => {
	const roomId = url.searchParams.get('room_id');
	const activeOnly = url.searchParams.get('active_only') === 'true';
	const dayOfWeek = url.searchParams.get('day_of_week');

	// Try backend first
	const backendData = await fetchFromBackend(
		`/api/admin/schedules?${url.searchParams.toString()}`,
		{
			headers: { Authorization: request.headers.get('Authorization') || '' }
		}
	);

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	let filteredSchedules = [...mockSchedules];

	if (roomId) {
		filteredSchedules = filteredSchedules.filter((s) => s.room_id === roomId);
	}

	if (activeOnly) {
		filteredSchedules = filteredSchedules.filter((s) => s.is_active);
	}

	if (dayOfWeek !== null && dayOfWeek !== '') {
		filteredSchedules = filteredSchedules.filter((s) => s.day_of_week === parseInt(dayOfWeek));
	}

	// Sort by day of week, then by start time
	filteredSchedules.sort((a, b) => {
		if (a.day_of_week !== b.day_of_week) {
			return a.day_of_week - b.day_of_week;
		}
		return a.start_time.localeCompare(b.start_time);
	});

	return json({
		success: true,
		data: filteredSchedules,
		meta: {
			total: filteredSchedules.length,
			room_id: roomId
		},
		_mock: true,
		_message: 'Using mock data. Connect backend for full functionality.'
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// POST - Create schedule
// ═══════════════════════════════════════════════════════════════════════════

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.json();

	// Try backend first
	const backendData = await fetchFromBackend('/api/admin/schedules', {
		method: 'POST',
		headers: { Authorization: request.headers.get('Authorization') || '' },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Validate required fields
	if (
		!body.room_id ||
		!body.title ||
		body.day_of_week === undefined ||
		!body.start_time ||
		!body.end_time
	) {
		return json(
			{
				success: false,
				error: 'Missing required fields: room_id, title, day_of_week, start_time, end_time'
			},
			{ status: 400 }
		);
	}

	// Create new schedule
	const newSchedule: ScheduleEvent = {
		id: scheduleIdCounter++,
		room_id: body.room_id,
		title: body.title,
		description: body.description || null,
		trader_name: body.trader_name || null,
		day_of_week: body.day_of_week,
		start_time: body.start_time,
		end_time: body.end_time,
		timezone: body.timezone || 'America/New_York',
		is_active: body.is_active ?? true,
		room_type: body.room_type || 'live',
		recurrence: body.recurrence || 'weekly',
		exceptions: [],
		created_at: new Date().toISOString(),
		updated_at: new Date().toISOString()
	};

	mockSchedules.push(newSchedule);

	return json({
		success: true,
		data: newSchedule,
		message: 'Schedule created successfully',
		_mock: true
	});
};

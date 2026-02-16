/**
 * Admin Schedules API - Individual Schedule Endpoint
 * ═══════════════════════════════════════════════════════════════════════════════════
 * Apple Principal Engineer ICT 11+ Grade - January 2026
 *
 * Handles individual schedule operations: GET, PUT, DELETE
 *
 * @version 1.0.0
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { logger } from '$lib/utils/logger';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

// ═══════════════════════════════════════════════════════════════════════════
// SHARED MOCK DATA STORE (in real app, this would be a database)
// ═══════════════════════════════════════════════════════════════════════════

// Import mock data reference - in production this would be database queries
const mockSchedules: any[] = [];

// Initialize with some data
function getOrInitMockData(): any[] {
	if (mockSchedules.length === 0) {
		// Default mock schedules
		mockSchedules.push({
			id: 1,
			room_id: 'day-trading-room',
			title: 'Pre-Market Analysis',
			description: 'Daily pre-market analysis and game plan for the trading day.',
			trader_name: 'Taylor Horton',
			day_of_week: 1,
			start_time: '08:30',
			end_time: '09:15',
			timezone: 'America/New_York',
			is_active: true,
			room_type: 'live',
			recurrence: 'weekly',
			exceptions: [],
			created_at: '2025-12-01T00:00:00Z',
			updated_at: '2025-12-01T00:00:00Z'
		});
	}
	return mockSchedules;
}

// ═══════════════════════════════════════════════════════════════════════════
// BACKEND HELPER
// ═══════════════════════════════════════════════════════════════════════════

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
			logger.warn(`Backend returned ${response.status} for ${endpoint}`);
			return null;
		}

		return await response.json();
	} catch (error) {
		logger.warn(`Failed to fetch from backend: ${error}`);
		return null;
	}
}

// ═══════════════════════════════════════════════════════════════════════════
// GET - Get single schedule by ID
// ═══════════════════════════════════════════════════════════════════════════

export const GET: RequestHandler = async ({ params, request }) => {
	const id = parseInt(params.id || '0');

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/schedules/${id}`, {
		headers: { Authorization: request.headers.get('Authorization') || '' }
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const schedules = getOrInitMockData();
	const schedule = schedules.find((s) => s.id === id);

	if (!schedule) {
		return json(
			{
				success: false,
				error: 'Schedule not found'
			},
			{ status: 404 }
		);
	}

	return json({
		success: true,
		data: schedule,
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// PUT - Update schedule
// ═══════════════════════════════════════════════════════════════════════════

export const PUT: RequestHandler = async ({ params, request }) => {
	const id = parseInt(params.id || '0');
	const body = await request.json();

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/schedules/${id}`, {
		method: 'PUT',
		headers: { Authorization: request.headers.get('Authorization') || '' },
		body: JSON.stringify(body)
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const schedules = getOrInitMockData();
	const scheduleIndex = schedules.findIndex((s) => s.id === id);

	if (scheduleIndex === -1) {
		return json(
			{
				success: false,
				error: 'Schedule not found'
			},
			{ status: 404 }
		);
	}

	// Update the schedule
	const updatedSchedule = {
		...schedules[scheduleIndex],
		...body,
		id: schedules[scheduleIndex].id, // Preserve ID
		updated_at: new Date().toISOString()
	};

	schedules[scheduleIndex] = updatedSchedule;

	return json({
		success: true,
		data: updatedSchedule,
		message: 'Schedule updated successfully',
		_mock: true
	});
};

// ═══════════════════════════════════════════════════════════════════════════
// DELETE - Delete schedule
// ═══════════════════════════════════════════════════════════════════════════

export const DELETE: RequestHandler = async ({ params, request }) => {
	const id = parseInt(params.id || '0');

	// Try backend first
	const backendData = await fetchFromBackend(`/api/admin/schedules/${id}`, {
		method: 'DELETE',
		headers: { Authorization: request.headers.get('Authorization') || '' }
	});

	if (backendData?.success) {
		return json(backendData);
	}

	// Fallback to mock data
	const schedules = getOrInitMockData();
	const scheduleIndex = schedules.findIndex((s) => s.id === id);

	if (scheduleIndex === -1) {
		return json(
			{
				success: false,
				error: 'Schedule not found'
			},
			{ status: 404 }
		);
	}

	// Remove the schedule
	schedules.splice(scheduleIndex, 1);

	return json({
		success: true,
		message: 'Schedule deleted successfully',
		_mock: true
	});
};

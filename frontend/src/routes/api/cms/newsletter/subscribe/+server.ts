import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
// FIX-2026-04-26: '#lib/config.js' reads import.meta.env.VITE_API_URL (client env) — replaced with canonical private env pattern
// import { API_URL } from '#lib/config.js';
import { API_BASE_URL, BACKEND_URL } from '$app/env/private';

const API_URL = `${API_BASE_URL || BACKEND_URL || 'http://localhost:8080'}/api`;

interface SubscribeRequest {
	email: string;
	source?: string;
	tags?: string[];
}
interface SubscribeResponse {
	success: boolean;
	message?: string;
	error?: string;
}

// Email validation regex
const EMAIL_REGEX = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

export const POST: RequestHandler = async ({ request, fetch, cookies }) => {
	try {
		const body: SubscribeRequest = await request.json();

		// Validate email
		if (!body.email || typeof body.email !== 'string') {
			return json({ success: false, error: 'Email is required' } as SubscribeResponse, {
				status: 400
			});
		}

		const email = body.email.trim().toLowerCase();

		if (!EMAIL_REGEX.test(email)) {
			return json({ success: false, error: 'Invalid email format' } as SubscribeResponse, {
				status: 400
			});
		}

		// Get auth token if available
		// FIX-2026-04-26: comment-out, verify, delete in follow-up. Wrong cookie name — login proxy sets rtp_access_token, not auth_token.
		// const token = cookies.get('auth_token');
		const token = cookies.get('rtp_access_token');

		// Forward to backend API
		const response = await fetch(`${API_URL}/newsletter/subscribe`, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				...(token ? { Authorization: `Bearer ${token}` } : {})
			},
			body: JSON.stringify({
				email,
				source: body.source || 'cms-block',
				tags: body.tags || []
			})
		});

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));
			return json(
				{
					success: false,
					error: errorData.message || 'Subscription failed'
				} as SubscribeResponse,
				{ status: response.status }
			);
		}

		return json({
			success: true,
			message: 'Successfully subscribed to newsletter!'
		} as SubscribeResponse);
	} catch (error) {
		console.error('[Newsletter Subscribe] Error:', error);
		return json({ success: false, error: 'Internal server error' } as SubscribeResponse, {
			status: 500
		});
	}
};

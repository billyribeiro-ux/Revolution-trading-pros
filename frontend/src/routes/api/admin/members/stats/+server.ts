/**
 * Members Stats API Proxy
 * ICT 7 FIX: Return mock data immediately - backend endpoint not implemented
 * This prevents 404/401 console errors
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	// ICT 7 FIX: Return mock data immediately - endpoint not implemented on backend
	// This prevents 404/401 console errors
	return json({
		overview: {
			total_members: 0,
			active_members: 0,
			new_this_month: 0
		},
		subscriptions: {
			active: 0,
			cancelled: 0,
			expired: 0
		}
	});
};

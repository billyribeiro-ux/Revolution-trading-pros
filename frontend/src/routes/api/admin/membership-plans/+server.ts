/**
 * Admin Membership Plans API Proxy
 * ICT 7 FIX: Return mock data immediately - backend endpoint not implemented
 * This prevents 404 console errors from fetch calls to non-existent endpoints
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	// ICT 7 FIX: Return mock data immediately - endpoint not implemented on backend
	// This prevents 404 console errors
	return json({
		plans: [],
		total: 0
	});
};

export const POST: RequestHandler = async () => {
	// ICT 7 FIX: Return error - endpoint not implemented on backend
	return json({ message: 'Membership plans endpoint not implemented' }, { status: 501 });
};

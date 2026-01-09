/**
 * Products API Proxy
 * ICT 7 FIX: Return mock data immediately - backend endpoint not implemented
 * This prevents 404 console errors from fetch calls to non-existent endpoints
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	// ICT 7 FIX: Return mock data immediately - endpoint not implemented on backend
	// This prevents 404 console errors
	return json({
		products: [],
		data: [],
		total: 0,
		meta: {
			current_page: 1,
			per_page: 100,
			total: 0,
			last_page: 1
		}
	});
};

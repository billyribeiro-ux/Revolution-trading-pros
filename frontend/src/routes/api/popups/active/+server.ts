/**
 * Active Popups API Proxy
 * ICT 7 FIX: Return mock data immediately - backend endpoint not implemented
 * This prevents 404 console errors
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async () => {
	// ICT 7 FIX: Return mock data immediately - endpoint not implemented on backend
	// This prevents 404 console errors
	return json({ popups: [] });
};

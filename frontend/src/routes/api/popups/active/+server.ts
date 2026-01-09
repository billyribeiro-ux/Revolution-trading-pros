/**
 * Active Popups API
 * Returns popups that should be displayed on the current page
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

// Production fallback - Rust API on Fly.io
const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';
const BACKEND_URL = env.BACKEND_URL || env.VITE_API_URL || PROD_BACKEND;

export const GET: RequestHandler = async ({ url, fetch }) => {
	// ICT 7 FIX: Always return empty popups - endpoint not implemented on backend
	// This prevents 404 console errors
	return json({ popups: [] });
};

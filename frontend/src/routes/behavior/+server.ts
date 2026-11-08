import { redirect, type RequestHandler } from '@sveltejs/kit';

/** Legacy public URL; the working dashboard requires admin authentication. */
export const GET: RequestHandler = () => redirect(308, '/admin/behavior');

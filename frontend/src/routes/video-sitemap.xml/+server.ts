import type { RequestHandler } from '@sveltejs/kit';
import { sourceSitemap } from '#lib/server/seo-publications.js';
export const prerender = false;
export const GET: RequestHandler = ({ url }) => sourceSitemap(url, 'video');

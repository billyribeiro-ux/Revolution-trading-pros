import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberConfig } from '#lib/server/member-config-proxy.js';
export const GET: RequestHandler = (event) => proxyMemberConfig(event, 'filters');
export const PUT: RequestHandler = (event) => proxyMemberConfig(event, 'filters');
export const DELETE: RequestHandler = (event) => proxyMemberConfig(event, 'filters');

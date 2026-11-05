import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberFacts } from '#lib/server/member-facts-proxy.js';
export const GET: RequestHandler = (event) => proxyMemberFacts(event, 'services');

import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberConfig } from '#lib/server/member-config-proxy.js';
export const POST: RequestHandler = (event) => proxyMemberConfig(event, 'tags', 'bulk-assign');

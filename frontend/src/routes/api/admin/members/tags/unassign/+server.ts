import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberConfig } from '#lib/server/member-config-proxy.js';
export const DELETE: RequestHandler = (event) => proxyMemberConfig(event, 'tags', 'unassign');

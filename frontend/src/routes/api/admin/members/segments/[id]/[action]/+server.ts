import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberConfig } from '#lib/server/member-config-proxy.js';
export const GET: RequestHandler = (event) =>
	proxyMemberConfig(event, 'segments', `${event.params.id}/${event.params.action}`);

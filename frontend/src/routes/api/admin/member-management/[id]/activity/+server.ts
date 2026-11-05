import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberProfile } from '#lib/server/member-profile-proxy.js';

export const GET: RequestHandler = (event) =>
	proxyMemberProfile(event, 'member-management', 'activity');

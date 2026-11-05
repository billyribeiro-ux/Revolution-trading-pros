import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberProfile } from '#lib/server/member-profile-proxy.js';

export const GET: RequestHandler = (event) => proxyMemberProfile(event, 'members', 'notes');
export const POST: RequestHandler = (event) => proxyMemberProfile(event, 'members', 'notes');

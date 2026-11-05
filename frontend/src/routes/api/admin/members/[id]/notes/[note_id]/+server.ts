import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberProfile } from '#lib/server/member-profile-proxy.js';

export const DELETE: RequestHandler = (event) =>
	proxyMemberProfile(event, 'members', 'notes', event.params.note_id);

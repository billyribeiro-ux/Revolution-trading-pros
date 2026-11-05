import type { RequestHandler } from '@sveltejs/kit';
import { proxySequences } from '#lib/server/sequence-proxy.js';

export const POST: RequestHandler = (event) =>
	proxySequences({ ...event, params: { ...event.params, rest: 'events' } });

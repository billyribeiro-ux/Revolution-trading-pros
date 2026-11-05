import type { RequestHandler } from '@sveltejs/kit';
import { proxySequences } from '#lib/server/sequence-proxy.js';
export const GET: RequestHandler = proxySequences;
export const POST: RequestHandler = proxySequences;
export const PUT: RequestHandler = proxySequences;
export const DELETE: RequestHandler = proxySequences;

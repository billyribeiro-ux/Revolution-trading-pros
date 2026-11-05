import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberOrderExport } from '#lib/server/member-order-export-proxy.js';
export const GET: RequestHandler = (event) => proxyMemberOrderExport(event, 'members');

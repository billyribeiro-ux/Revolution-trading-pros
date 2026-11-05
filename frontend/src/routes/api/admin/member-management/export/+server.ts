import type { RequestHandler } from './$types';
import { proxyMemberExport } from '#lib/server/member-export-proxy.js';
export const GET: RequestHandler = (event) => proxyMemberExport(event, true);

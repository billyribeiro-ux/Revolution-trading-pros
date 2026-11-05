import type { RequestHandler } from '@sveltejs/kit';
import { proxyCrmTransfers } from '#lib/server/crm-transfer-proxy.js';
export const GET: RequestHandler = (event) => proxyCrmTransfers(event, 'exports');

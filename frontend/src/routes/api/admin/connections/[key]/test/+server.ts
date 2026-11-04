import type { RequestHandler } from './$types';
import { proxyConnections } from '#lib/server/connection-proxy.js';
export const POST: RequestHandler = (event) => proxyConnections(event, 'test');

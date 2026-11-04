import type { RequestHandler } from './$types';
import { proxyConnections } from '#lib/server/connection-proxy.js';
export const GET: RequestHandler = (event) => proxyConnections(event, 'detail');
export const DELETE: RequestHandler = (event) => proxyConnections(event, 'detail');

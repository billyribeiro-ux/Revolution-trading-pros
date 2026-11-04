import type { RequestHandler } from './$types';
import { proxySmtp } from '#lib/server/smtp-proxy.js';
export const GET: RequestHandler = (event) => proxySmtp(event);
export const POST: RequestHandler = (event) => proxySmtp(event);

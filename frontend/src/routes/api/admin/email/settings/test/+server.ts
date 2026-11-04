import type { RequestHandler } from './$types';
import { proxySmtp } from '#lib/server/smtp-proxy.js';
export const POST: RequestHandler = (event) => proxySmtp(event, true);

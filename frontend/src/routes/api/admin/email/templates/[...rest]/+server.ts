import type { RequestHandler } from '@sveltejs/kit';
import { proxyEmailTemplates } from '#lib/server/email-template-proxy.js';
export const GET: RequestHandler = proxyEmailTemplates;
export const POST: RequestHandler = proxyEmailTemplates;
export const PUT: RequestHandler = proxyEmailTemplates;
export const PATCH: RequestHandler = proxyEmailTemplates;
export const DELETE: RequestHandler = proxyEmailTemplates;

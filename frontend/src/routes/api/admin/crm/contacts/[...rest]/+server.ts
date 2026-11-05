import type { RequestHandler } from '@sveltejs/kit';
import { proxyCrmContacts } from '#lib/server/crm-contact-proxy.js';
export const GET: RequestHandler = proxyCrmContacts;
export const POST: RequestHandler = proxyCrmContacts;
export const PUT: RequestHandler = proxyCrmContacts;
export const DELETE: RequestHandler = proxyCrmContacts;

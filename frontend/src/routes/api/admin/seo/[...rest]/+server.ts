import type { RequestHandler } from '@sveltejs/kit';
import { proxyPublicationAdmin } from '#lib/server/seo-publications.js';
const proxy: RequestHandler = (event) => proxyPublicationAdmin(event, 'seo');
export { proxy as GET, proxy as POST, proxy as PUT, proxy as DELETE };

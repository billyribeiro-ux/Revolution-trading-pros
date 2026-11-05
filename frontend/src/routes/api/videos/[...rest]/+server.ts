import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberVideo } from '#lib/server/member-videos.js';
const proxy: RequestHandler = (event) => proxyMemberVideo(event);
export { proxy as GET, proxy as POST };

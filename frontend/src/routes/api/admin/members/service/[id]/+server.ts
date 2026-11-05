import type { RequestHandler } from '@sveltejs/kit';
import { proxyMemberFacts } from '#lib/server/member-facts-proxy.js';
export const GET: RequestHandler = (event) => {
	if (
		!event.params.id ||
		!/^[1-9]\d*$/.test(event.params.id) ||
		!Number.isSafeInteger(Number(event.params.id))
	)
		return Response.json({ message: 'Invalid membership plan identifier' }, { status: 400 });
	return proxyMemberFacts(event, `service/${event.params.id}`);
};

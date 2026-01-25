import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

/**
 * Redirect old /trade-tracker/ route to new /trades/ route
 * @deprecated This route has been replaced by /trades/
 */
export const load: PageServerLoad = async () => {
	throw redirect(301, '/dashboard/explosive-swings/trades');
};

import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

// The admin layout is CSR; this alias must run its redirect on the HTML request.
export const ssr = true;
export const prerender = false;

export const load: PageServerLoad = () => {
	redirect(307, '/admin/analytics/cohorts?create=1');
};

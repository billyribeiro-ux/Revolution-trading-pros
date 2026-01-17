/**
 * Explosive Swings Start Here - Server Load Function
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * SSR pre-fetch for getting started content
 *
 * @version 1.0.0
 */

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	// Start Here page is primarily static content
	// Could fetch dynamic onboarding status or checklist in the future
	return {};
};

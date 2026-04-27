/**
 * Admin Subscription Plan Price-Change API Proxy (STUB — 501)
 *
 * FIX-2026-04-26 (audit 02 §P0-4): The Subscription Plans page POSTs to
 * `/api/admin/subscriptions/plans/:id/price` to change a plan's price.
 * Backend exists at `subscriptions_admin.rs::change_plan_price` but the
 * operation TOUCHES STRIPE (creates a new Stripe Price, optionally migrates
 * existing subscribers with proration).
 *
 * Per task instructions for this audit cluster (P0-4):
 *   "Build a stub proxy that returns proper 501 'not implemented' rather
 *    than the page hanging."
 *
 * The stub keeps the page from hanging on a 405 from SvelteKit and gives
 * the admin a clear "wired-but-unwired" signal. Wiring this proxy through to
 * the backend should happen as part of the deferred Stripe-state-machine
 * pass tracked in `02-members-subscriptions-DEFERRED.md` §D1, not here —
 * accidentally double-charging or migrating the wrong customers is too high
 * blast-radius for a single principal-engineer pass.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireSuperadmin } from '$lib/server/auth';

export const POST: RequestHandler = async (event) => {
	// Auth-gate even the stub so unauthorized callers can't probe the surface.
	requireSuperadmin(event);
	return json(
		{
			error: 'Not implemented',
			detail:
				'Plan price changes touch Stripe (create new Price, optionally migrate ' +
				'subscribers with proration). Deferred per audit 02 §P0-4. See ' +
				'docs/audits/admin-2026-04-26/02-members-subscriptions-DEFERRED.md §D1.'
		},
		{ status: 501 }
	);
};

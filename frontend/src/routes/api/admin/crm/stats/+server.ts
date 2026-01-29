/**
 * CRM Stats API - RevolutionCRM Pro
 *
 * Fetches CRM statistics from backend.
 *
 * @version 2.0.0 - January 2026
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

const PROD_BACKEND = 'https://revolution-trading-pros-api.fly.dev';

export const GET: RequestHandler = async ({ request }) => {
	const backendUrl = PROD_BACKEND;
	const authHeader = request.headers.get('Authorization') || '';

	try {
		const response = await fetch(`${backendUrl}/api/admin/crm/stats`, {
			headers: {
				'Content-Type': 'application/json',
				Accept: 'application/json',
				Authorization: authHeader
			}
		});

		if (response.ok) {
			const data = await response.json();
			return json({ success: true, data });
		}

		console.warn(`Backend CRM stats returned ${response.status}`);
	} catch (err) {
		console.warn('Backend CRM stats not available:', err);
	}

	// Return empty stats on error
	return json({
		success: true,
		data: {
			total_contacts: 0,
			new_this_month: 0,
			contacts_by_status: {},
			contacts_by_source: {},
			active_deals: 0,
			deal_value: 0,
			won_deals_this_month: 0,
			won_value_this_month: 0,
			average_deal_size: 0,
			win_rate: 0,
			average_sales_cycle: 0,
			pipeline_stages: [],
			activities_this_week: 0,
			emails_sent: 0,
			meetings_scheduled: 0,
			tasks_completed: 0,
			total_revenue: 0,
			mrr: 0,
			arr: 0,
			lifetime_value_avg: 0,
			avg_lead_score: 0,
			avg_health_score: 0,
			contacts_at_risk: 0,
			contacts_engaged: 0,
			trends: { contacts_growth: 0, deals_growth: 0, revenue_growth: 0 }
		}
	});
};

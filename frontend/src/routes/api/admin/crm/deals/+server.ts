/**
 * CRM Deals API - RevolutionCRM Pro
 *
 * Manages deals/opportunities for the built-in CRM system.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

interface Deal {
	id: string;
	name: string;
	contact_id: string;
	contact_name?: string;
	company_id?: string;
	pipeline_id: string;
	stage_id: string;
	stage_name: string;
	amount: number;
	currency: string;
	probability: number;
	status: 'open' | 'won' | 'lost' | 'abandoned';
	priority: 'low' | 'normal' | 'high' | 'urgent';
	expected_close_date: string;
	close_date?: string;
	owner_id: string;
	owner_name: string;
	source_channel?: string;
	tags: string[];
	created_at: string;
	updated_at: string;
}

// In-memory storage with sample data
const deals: Map<string, Deal> = new Map();

// Initialize with sample data
function initializeSampleData() {
	if (deals.size > 0) return;

	const sampleDeals: Deal[] = [
		{
			id: 'deal_1',
			name: 'Enterprise Trading Package - FinTech Solutions',
			contact_id: 'contact_6',
			contact_name: 'Amanda Taylor',
			pipeline_id: 'pipeline_main',
			stage_id: 'negotiation',
			stage_name: 'Negotiation',
			amount: 24999,
			currency: 'USD',
			probability: 75,
			status: 'open',
			priority: 'high',
			expected_close_date: new Date(Date.now() + 14 * 24 * 60 * 60 * 1000).toISOString(),
			owner_id: 'user_1',
			owner_name: 'Sales Team',
			source_channel: 'event',
			tags: ['enterprise', 'priority'],
			created_at: new Date(Date.now() - 21 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'deal_2',
			name: 'Pro Membership Upgrade - Michael Chen',
			contact_id: 'contact_3',
			contact_name: 'Michael Chen',
			pipeline_id: 'pipeline_main',
			stage_id: 'proposal',
			stage_name: 'Proposal',
			amount: 1499,
			currency: 'USD',
			probability: 50,
			status: 'open',
			priority: 'normal',
			expected_close_date: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000).toISOString(),
			owner_id: 'user_1',
			owner_name: 'Sales Team',
			source_channel: 'trial',
			tags: ['upgrade', 'trial-conversion'],
			created_at: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'deal_3',
			name: 'VIP Coaching Package - Robert Davis',
			contact_id: 'contact_5',
			contact_name: 'Robert Davis',
			pipeline_id: 'pipeline_main',
			stage_id: 'qualified',
			stage_name: 'Qualified',
			amount: 4999,
			currency: 'USD',
			probability: 30,
			status: 'open',
			priority: 'normal',
			expected_close_date: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
			owner_id: 'user_1',
			owner_name: 'Sales Team',
			source_channel: 'webinar',
			tags: ['coaching', 'high-value'],
			created_at: new Date(Date.now() - 10 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'deal_4',
			name: 'Annual Premium - Sarah Johnson (Renewal)',
			contact_id: 'contact_2',
			contact_name: 'Sarah Johnson',
			pipeline_id: 'pipeline_renewals',
			stage_id: 'closed',
			stage_name: 'Closed Won',
			amount: 12999,
			currency: 'USD',
			probability: 100,
			status: 'won',
			priority: 'high',
			expected_close_date: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
			close_date: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
			owner_id: 'user_1',
			owner_name: 'Sales Team',
			source_channel: 'renewal',
			tags: ['renewal', 'vip'],
			created_at: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		}
	];

	for (const deal of sampleDeals) {
		deals.set(deal.id, deal);
	}
}

// Initialize on module load
initializeSampleData();

// GET - List deals
export const GET: RequestHandler = async ({ url }) => {
	const status = url.searchParams.get('status');
	const pipeline_id = url.searchParams.get('pipeline_id');
	const stage_id = url.searchParams.get('stage_id');
	const page = parseInt(url.searchParams.get('page') || '1');
	const limit = parseInt(url.searchParams.get('limit') || '20');

	let dealList = Array.from(deals.values());

	// Filter by status
	if (status && status !== 'all') {
		dealList = dealList.filter(d => d.status === status);
	}

	// Filter by pipeline
	if (pipeline_id) {
		dealList = dealList.filter(d => d.pipeline_id === pipeline_id);
	}

	// Filter by stage
	if (stage_id) {
		dealList = dealList.filter(d => d.stage_id === stage_id);
	}

	// Sort by expected close date (soonest first)
	dealList.sort((a, b) =>
		new Date(a.expected_close_date).getTime() - new Date(b.expected_close_date).getTime()
	);

	// Pagination
	const total = dealList.length;
	const offset = (page - 1) * limit;
	const paginatedDeals = dealList.slice(offset, offset + limit);

	return json({
		success: true,
		data: paginatedDeals,
		meta: {
			page,
			limit,
			total,
			total_pages: Math.ceil(total / limit)
		}
	});
};

// POST - Create deal
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		if (!body.name) {
			throw error(400, 'Deal name is required');
		}

		const id = `deal_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

		const deal: Deal = {
			id,
			name: body.name,
			contact_id: body.contact_id || '',
			contact_name: body.contact_name,
			company_id: body.company_id,
			pipeline_id: body.pipeline_id || 'pipeline_main',
			stage_id: body.stage_id || 'lead',
			stage_name: body.stage_name || 'Lead',
			amount: body.amount || 0,
			currency: body.currency || 'USD',
			probability: body.probability || 10,
			status: body.status || 'open',
			priority: body.priority || 'normal',
			expected_close_date: body.expected_close_date || new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
			owner_id: body.owner_id || 'user_1',
			owner_name: body.owner_name || 'Sales Team',
			source_channel: body.source_channel,
			tags: body.tags || [],
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		deals.set(id, deal);

		return json({
			success: true,
			data: deal
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to create deal');
	}
};

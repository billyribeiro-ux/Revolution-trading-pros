/**
 * CRM Contacts API - RevolutionCRM Pro
 *
 * Manages contact records for the built-in CRM system.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

interface Contact {
	id: string;
	email: string;
	name: string;
	first_name: string;
	last_name: string;
	phone?: string;
	company?: string;
	job_title?: string;
	status: 'lead' | 'prospect' | 'customer' | 'churned' | 'unqualified';
	lifecycle_stage: string;
	lead_score: number;
	health_score: number;
	lifetime_value: number;
	source: string;
	tags: string[];
	last_activity_at: string;
	created_at: string;
	updated_at: string;
}

// In-memory storage with sample data
const contacts: Map<string, Contact> = new Map();

// Initialize with sample data
function initializeSampleData() {
	if (contacts.size > 0) return;

	const sampleContacts: Contact[] = [
		{
			id: 'contact_1',
			email: 'john.smith@tradingpro.com',
			name: 'John Smith',
			first_name: 'John',
			last_name: 'Smith',
			phone: '+1 (555) 123-4567',
			company: 'Trading Pro LLC',
			job_title: 'Day Trader',
			status: 'customer',
			lifecycle_stage: 'customer',
			lead_score: 92,
			health_score: 95,
			lifetime_value: 4999,
			source: 'website',
			tags: ['premium', 'options-trader', 'webinar-attendee'],
			last_activity_at: new Date(Date.now() - 2 * 60 * 60 * 1000).toISOString(),
			created_at: new Date(Date.now() - 90 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'contact_2',
			email: 'sarah.johnson@investwise.io',
			name: 'Sarah Johnson',
			first_name: 'Sarah',
			last_name: 'Johnson',
			phone: '+1 (555) 234-5678',
			company: 'InvestWise Capital',
			job_title: 'Portfolio Manager',
			status: 'customer',
			lifecycle_stage: 'evangelist',
			lead_score: 98,
			health_score: 100,
			lifetime_value: 12999,
			source: 'referral',
			tags: ['vip', 'enterprise', 'swing-trader'],
			last_activity_at: new Date(Date.now() - 30 * 60 * 1000).toISOString(),
			created_at: new Date(Date.now() - 180 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'contact_3',
			email: 'michael.chen@gmail.com',
			name: 'Michael Chen',
			first_name: 'Michael',
			last_name: 'Chen',
			phone: '+1 (555) 345-6789',
			status: 'prospect',
			lifecycle_stage: 'mql',
			lead_score: 75,
			health_score: 80,
			lifetime_value: 0,
			source: 'form',
			tags: ['trial-user', 'beginner'],
			last_activity_at: new Date(Date.now() - 24 * 60 * 60 * 1000).toISOString(),
			created_at: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'contact_4',
			email: 'emily.wilson@daytraders.net',
			name: 'Emily Wilson',
			first_name: 'Emily',
			last_name: 'Wilson',
			company: 'Day Traders Network',
			job_title: 'Trading Coach',
			status: 'customer',
			lifecycle_stage: 'customer',
			lead_score: 88,
			health_score: 90,
			lifetime_value: 2499,
			source: 'webinar',
			tags: ['coach', 'futures-trader'],
			last_activity_at: new Date(Date.now() - 4 * 60 * 60 * 1000).toISOString(),
			created_at: new Date(Date.now() - 45 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'contact_5',
			email: 'robert.davis@outlook.com',
			name: 'Robert Davis',
			first_name: 'Robert',
			last_name: 'Davis',
			status: 'lead',
			lifecycle_stage: 'lead',
			lead_score: 45,
			health_score: 60,
			lifetime_value: 0,
			source: 'website',
			tags: ['newsletter'],
			last_activity_at: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000).toISOString(),
			created_at: new Date(Date.now() - 14 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		},
		{
			id: 'contact_6',
			email: 'amanda.taylor@fintech.co',
			name: 'Amanda Taylor',
			first_name: 'Amanda',
			last_name: 'Taylor',
			company: 'FinTech Solutions',
			job_title: 'CTO',
			status: 'prospect',
			lifecycle_stage: 'sql',
			lead_score: 85,
			health_score: 85,
			lifetime_value: 0,
			source: 'event',
			tags: ['enterprise-lead', 'decision-maker'],
			last_activity_at: new Date(Date.now() - 12 * 60 * 60 * 1000).toISOString(),
			created_at: new Date(Date.now() - 21 * 24 * 60 * 60 * 1000).toISOString(),
			updated_at: new Date().toISOString()
		}
	];

	for (const contact of sampleContacts) {
		contacts.set(contact.id, contact);
	}
}

// Initialize on module load
initializeSampleData();

// GET - List contacts
export const GET: RequestHandler = async ({ url }) => {
	const status = url.searchParams.get('status');
	const search = url.searchParams.get('search');
	const page = parseInt(url.searchParams.get('page') || '1');
	const limit = parseInt(url.searchParams.get('limit') || '20');

	let contactList = Array.from(contacts.values());

	// Filter by status
	if (status && status !== 'all') {
		contactList = contactList.filter(c => c.status === status);
	}

	// Search
	if (search) {
		const searchLower = search.toLowerCase();
		contactList = contactList.filter(c =>
			c.name.toLowerCase().includes(searchLower) ||
			c.email.toLowerCase().includes(searchLower) ||
			c.company?.toLowerCase().includes(searchLower)
		);
	}

	// Sort by last activity (most recent first)
	contactList.sort((a, b) =>
		new Date(b.last_activity_at).getTime() - new Date(a.last_activity_at).getTime()
	);

	// Pagination
	const total = contactList.length;
	const offset = (page - 1) * limit;
	const paginatedContacts = contactList.slice(offset, offset + limit);

	return json({
		success: true,
		data: paginatedContacts,
		meta: {
			page,
			limit,
			total,
			total_pages: Math.ceil(total / limit)
		}
	});
};

// POST - Create contact
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		if (!body.email) {
			throw error(400, 'Email is required');
		}

		// Check for duplicate email
		const existing = Array.from(contacts.values()).find(c => c.email === body.email);
		if (existing) {
			throw error(409, 'Contact with this email already exists');
		}

		const id = `contact_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

		const contact: Contact = {
			id,
			email: body.email,
			name: body.name || `${body.first_name || ''} ${body.last_name || ''}`.trim() || body.email,
			first_name: body.first_name || '',
			last_name: body.last_name || '',
			phone: body.phone,
			company: body.company,
			job_title: body.job_title,
			status: body.status || 'lead',
			lifecycle_stage: body.lifecycle_stage || 'lead',
			lead_score: body.lead_score || 0,
			health_score: body.health_score || 50,
			lifetime_value: body.lifetime_value || 0,
			source: body.source || 'manual',
			tags: body.tags || [],
			last_activity_at: new Date().toISOString(),
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		contacts.set(id, contact);

		return json({
			success: true,
			data: contact
		}, { status: 201 });
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to create contact');
	}
};

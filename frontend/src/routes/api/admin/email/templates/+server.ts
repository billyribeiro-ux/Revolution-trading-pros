/**
 * Email Templates API Endpoint
 * Apple ICT11+ Principal Engineer Grade
 *
 * Handles email template CRUD operations
 * Note: In production, this would connect to a database.
 * This implementation provides mock data to prevent SQL errors.
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Mock email templates data
let emailTemplates = [
	{
		id: 1,
		name: 'Welcome Email',
		slug: 'welcome-email',
		subject: 'Welcome to Revolution Trading Pros!',
		email_type: 'transactional',
		is_active: true,
		body_html: '<h1>Welcome!</h1><p>Thank you for joining Revolution Trading Pros.</p>',
		body_text: 'Welcome! Thank you for joining Revolution Trading Pros.',
		created_at: '2024-01-15T10:00:00Z',
		updated_at: '2024-01-15T10:00:00Z'
	},
	{
		id: 2,
		name: 'Password Reset',
		slug: 'password-reset',
		subject: 'Reset Your Password',
		email_type: 'transactional',
		is_active: true,
		body_html: '<h1>Password Reset</h1><p>Click the link below to reset your password.</p>',
		body_text: 'Password Reset - Click the link below to reset your password.',
		created_at: '2024-01-15T10:00:00Z',
		updated_at: '2024-01-15T10:00:00Z'
	},
	{
		id: 3,
		name: 'Subscription Confirmation',
		slug: 'subscription-confirmation',
		subject: 'Your Subscription is Active',
		email_type: 'transactional',
		is_active: true,
		body_html: '<h1>Subscription Confirmed</h1><p>Your subscription is now active.</p>',
		body_text: 'Subscription Confirmed - Your subscription is now active.',
		created_at: '2024-01-15T10:00:00Z',
		updated_at: '2024-01-15T10:00:00Z'
	},
	{
		id: 4,
		name: 'Weekly Newsletter',
		slug: 'weekly-newsletter',
		subject: 'This Week in Trading',
		email_type: 'marketing',
		is_active: true,
		body_html: "<h1>Weekly Newsletter</h1><p>Here are this week's trading insights.</p>",
		body_text: "Weekly Newsletter - Here are this week's trading insights.",
		created_at: '2024-01-15T10:00:00Z',
		updated_at: '2024-01-15T10:00:00Z'
	}
];

export const GET: RequestHandler = async () => {
	try {
		return json({
			success: true,
			data: emailTemplates,
			total: emailTemplates.length
		});
	} catch (error) {
		console.error('Failed to get email templates:', error);
		return json(
			{ success: false, error: 'Failed to load email templates', data: [] },
			{ status: 500 }
		);
	}
};

export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields
		if (!body.name || !body.subject) {
			return json({ success: false, error: 'Name and subject are required' }, { status: 400 });
		}

		// Create new template
		const newTemplate = {
			id: Math.max(...emailTemplates.map((t) => t.id), 0) + 1,
			name: body.name,
			slug: body.slug || body.name.toLowerCase().replace(/\s+/g, '-'),
			subject: body.subject,
			email_type: body.email_type || 'transactional',
			is_active: body.is_active !== false,
			body_html: body.body_html || '',
			body_text: body.body_text || '',
			created_at: new Date().toISOString(),
			updated_at: new Date().toISOString()
		};

		emailTemplates.push(newTemplate);

		return json({
			success: true,
			data: newTemplate,
			message: 'Template created successfully'
		});
	} catch (error) {
		console.error('Failed to create email template:', error);
		return json({ success: false, error: 'Failed to create template' }, { status: 500 });
	}
};

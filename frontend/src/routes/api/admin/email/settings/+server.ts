/**
 * Email Settings API Endpoint
 * Apple ICT11+ Principal Engineer Grade
 *
 * Handles SMTP configuration for email delivery
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';

// Default SMTP settings (in production, these would come from database)
let emailSettings = {
	provider: 'smtp',
	host: '',
	port: 587,
	username: '',
	password: '',
	encryption: 'tls',
	from_address: '',
	from_name: 'Revolution Trading Pros'
};

export const GET: RequestHandler = async () => {
	try {
		// Return settings (with password masked for security)
		return json({
			...emailSettings,
			password: emailSettings.password ? '••••••••' : ''
		});
	} catch (error) {
		console.error('Failed to get email settings:', error);
		return json({ error: 'Failed to load email settings' }, { status: 500 });
	}
};

export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();

		// Validate required fields
		if (!body.host || !body.port || !body.from_address || !body.from_name) {
			return json({ success: false, error: 'Missing required fields' }, { status: 400 });
		}

		// Update settings (only update password if a new one is provided)
		emailSettings = {
			provider: body.provider || 'smtp',
			host: body.host,
			port: parseInt(body.port),
			username: body.username || '',
			password:
				body.password && body.password !== '••••••••' ? body.password : emailSettings.password,
			encryption: body.encryption || 'tls',
			from_address: body.from_address,
			from_name: body.from_name
		};

		// In production, save to database here
		console.log('Email settings updated successfully');

		return json({
			success: true,
			message: 'Settings saved successfully!'
		});
	} catch (error) {
		console.error('Failed to save email settings:', error);
		return json({ success: false, error: 'Failed to save settings' }, { status: 500 });
	}
};

/**
 * Connection Status API
 *
 * Returns the status of all API connections including built-in services.
 * The built-in RevolutionCRM is always marked as connected.
 *
 * @version 1.0.0 - December 2025
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

interface ConnectionStatus {
	key: string;
	name: string;
	category: string;
	is_connected: boolean;
	status: 'connected' | 'disconnected' | 'error' | 'expired' | 'pending';
	health_score: number;
	last_verified_at: string | null;
	last_error: string | null;
	is_builtin?: boolean;
}

// Built-in services that are always available
const BUILTIN_SERVICES: ConnectionStatus[] = [
	{
		key: 'revolution_crm',
		name: 'RevolutionCRM Pro',
		category: 'CRM',
		is_connected: true,
		status: 'connected',
		health_score: 100,
		last_verified_at: new Date().toISOString(),
		last_error: null,
		is_builtin: true
	}
];

// In-memory connection storage (in production, from database)
const storedConnections: Map<string, ConnectionStatus> = new Map();

// All available external services
const EXTERNAL_SERVICES: Omit<ConnectionStatus, 'is_connected' | 'status' | 'health_score' | 'last_verified_at' | 'last_error'>[] = [
	// Analytics
	{ key: 'google_analytics', name: 'Google Analytics 4', category: 'Analytics' },
	{ key: 'mixpanel', name: 'Mixpanel', category: 'Analytics' },
	{ key: 'amplitude', name: 'Amplitude', category: 'Analytics' },
	{ key: 'segment', name: 'Segment', category: 'Analytics' },
	{ key: 'plausible', name: 'Plausible', category: 'Analytics' },

	// SEO
	{ key: 'google_search_console', name: 'Google Search Console', category: 'SEO' },
	{ key: 'bing_webmaster', name: 'Bing Webmaster Tools', category: 'SEO' },
	{ key: 'semrush', name: 'SEMrush', category: 'SEO' },
	{ key: 'ahrefs', name: 'Ahrefs', category: 'SEO' },
	{ key: 'moz', name: 'Moz', category: 'SEO' },

	// Email
	{ key: 'mailchimp', name: 'Mailchimp', category: 'Email' },
	{ key: 'sendgrid', name: 'SendGrid', category: 'Email' },
	{ key: 'mailgun', name: 'Mailgun', category: 'Email' },
	{ key: 'postmark', name: 'Postmark', category: 'Email' },
	{ key: 'sendinblue', name: 'Brevo (Sendinblue)', category: 'Email' },

	// Payment
	{ key: 'stripe', name: 'Stripe', category: 'Payment' },
	{ key: 'paypal', name: 'PayPal', category: 'Payment' },
	{ key: 'square', name: 'Square', category: 'Payment' },

	// CRM (External)
	{ key: 'hubspot', name: 'HubSpot', category: 'CRM' },
	{ key: 'salesforce', name: 'Salesforce', category: 'CRM' },
	{ key: 'pipedrive', name: 'Pipedrive', category: 'CRM' },

	// Social
	{ key: 'facebook', name: 'Facebook', category: 'Social' },
	{ key: 'twitter', name: 'Twitter/X', category: 'Social' },
	{ key: 'linkedin', name: 'LinkedIn', category: 'Social' },
	{ key: 'instagram', name: 'Instagram', category: 'Social' },

	// Cloud/Storage
	{ key: 'aws', name: 'Amazon Web Services', category: 'Cloud' },
	{ key: 'aws_s3', name: 'Amazon S3', category: 'Storage' },
	{ key: 'google_cloud', name: 'Google Cloud', category: 'Cloud' },
	{ key: 'cloudflare', name: 'Cloudflare', category: 'Cloud' },
	{ key: 'cloudflare_r2', name: 'Cloudflare R2', category: 'Storage' },

	// AI
	{ key: 'openai', name: 'OpenAI', category: 'AI' },
	{ key: 'anthropic', name: 'Anthropic', category: 'AI' },

	// Monitoring
	{ key: 'sentry', name: 'Sentry', category: 'Monitoring' },
	{ key: 'datadog', name: 'Datadog', category: 'Monitoring' },
	{ key: 'new_relic', name: 'New Relic', category: 'Monitoring' },

	// Communication
	{ key: 'twilio', name: 'Twilio', category: 'Communication' },
	{ key: 'slack', name: 'Slack', category: 'Communication' },
	{ key: 'discord', name: 'Discord', category: 'Communication' },

	// Trading
	{ key: 'tradier', name: 'Tradier', category: 'Trading' },
	{ key: 'alpaca', name: 'Alpaca', category: 'Trading' },
	{ key: 'polygon', name: 'Polygon.io', category: 'Trading' },

	// Media
	{ key: 'mux', name: 'Mux', category: 'Media' },
	{ key: 'cloudinary', name: 'Cloudinary', category: 'Media' },
	{ key: 'vimeo', name: 'Vimeo', category: 'Media' }
];

export const GET: RequestHandler = async () => {
	// Build connections list combining built-in and external services
	const connections: ConnectionStatus[] = [
		// Built-in services are always connected
		...BUILTIN_SERVICES,

		// External services - check stored connection status
		...EXTERNAL_SERVICES.map(service => {
			const stored = storedConnections.get(service.key);
			return {
				...service,
				is_connected: stored?.is_connected ?? false,
				status: stored?.status ?? 'disconnected' as const,
				health_score: stored?.health_score ?? 0,
				last_verified_at: stored?.last_verified_at ?? null,
				last_error: stored?.last_error ?? null
			};
		})
	];

	// Calculate summary
	const connected = connections.filter(c => c.is_connected);
	const errors = connections.filter(c => c.status === 'error');

	return json({
		success: true,
		connections,
		summary: {
			total: connections.length,
			connected: connected.length,
			disconnected: connections.length - connected.length,
			errors: errors.length,
			health: connected.length > 0
				? Math.round(connected.reduce((sum, c) => sum + c.health_score, 0) / connected.length)
				: 0
		}
	});
};

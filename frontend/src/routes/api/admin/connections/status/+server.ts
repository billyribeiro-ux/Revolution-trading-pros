/**
 * Connection Status API
 *
 * Returns the status of all API connections including built-in services.
 *
 * PRINCIPAL-2026-04-26 (audit 09-system §P0-3 / §P1-1):
 *   - PRE-FIX: this endpoint was publicly reachable with NO auth at all and
 *     leaked the full third-party service inventory (Stripe/SendGrid/AWS/
 *     HubSpot/Salesforce/Sentry/Datadog/Twilio/Tradier/Polygon, plus the
 *     built-in Fluent suite) — reconnaissance gold.
 *   - POST-FIX: requires admin-or-higher cookie auth via `requireAdmin`. The
 *     orphan-callers concern (P1-1) is addressed separately in the audit
 *     RESULTS doc; the file is preserved per the CREATE-not-DELETE rule.
 *
 * NOTE: this file remains a *standalone* in-memory mock today (not a Rust
 * proxy). The canonical truth is `/api/admin/connections` (the proxy in the
 * sibling +server.ts). Until callers consolidate on the proxy, we keep this
 * file behind an admin gate so it cannot be read anonymously.
 *
 * @version 1.1.0 - April 2026 (audit hardening)
 */

import { json } from '@sveltejs/kit';
import type { RequestHandler } from '@sveltejs/kit';
import { requireAdmin } from '$lib/server/auth';

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

/**
 * Build the built-in services list at request time so `last_verified_at`
 * reflects the moment of the request, not module load (audit P3-5).
 */
function buildBuiltinServices(): ConnectionStatus[] {
	const now = new Date().toISOString();
	return [
		{
			key: 'fluent_forms_pro',
			name: 'FluentForms Pro',
			category: 'Fluent',
			is_connected: true,
			status: 'connected',
			health_score: 100,
			last_verified_at: now,
			last_error: null,
			is_builtin: true
		},
		{
			key: 'fluent_crm_pro',
			name: 'FluentCRM Pro',
			category: 'Fluent',
			is_connected: true,
			status: 'connected',
			health_score: 100,
			last_verified_at: now,
			last_error: null,
			is_builtin: true
		},
		{
			key: 'fluent_smtp',
			name: 'FluentSMTP',
			category: 'Fluent',
			is_connected: true,
			status: 'connected',
			health_score: 100,
			last_verified_at: now,
			last_error: null,
			is_builtin: true
		},
		{
			key: 'fluent_support',
			name: 'FluentSupport',
			category: 'Fluent',
			is_connected: true,
			status: 'connected',
			health_score: 100,
			last_verified_at: now,
			last_error: null,
			is_builtin: true
		},
		{
			key: 'fluent_booking',
			name: 'FluentBooking',
			category: 'Fluent',
			is_connected: true,
			status: 'connected',
			health_score: 100,
			last_verified_at: now,
			last_error: null,
			is_builtin: true
		}
	];
}

// In-memory connection storage (in production, from database)
const storedConnections: Map<string, ConnectionStatus> = new Map();

// All available external services
const EXTERNAL_SERVICES: Omit<
	ConnectionStatus,
	'is_connected' | 'status' | 'health_score' | 'last_verified_at' | 'last_error'
>[] = [
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

	// Email Delivery (FluentCRM handles email marketing, these are for transactional/SMTP relay)
	{ key: 'sendgrid', name: 'SendGrid (SMTP)', category: 'Email Delivery' },
	{ key: 'mailgun', name: 'Mailgun (SMTP)', category: 'Email Delivery' },
	{ key: 'postmark', name: 'Postmark (SMTP)', category: 'Email Delivery' },
	{ key: 'amazon_ses', name: 'Amazon SES', category: 'Email Delivery' },

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

export const GET: RequestHandler = async (event) => {
	// PRINCIPAL-2026-04-26 (P0-3): require admin-or-higher cookie auth.
	requireAdmin(event);

	// Build connections list combining built-in and external services
	const connections: ConnectionStatus[] = [
		// Built-in services are always connected
		...buildBuiltinServices(),

		// External services - check stored connection status
		...EXTERNAL_SERVICES.map((service) => {
			const stored = storedConnections.get(service.key);
			return {
				...service,
				is_connected: stored?.is_connected ?? false,
				status: stored?.status ?? ('disconnected' as const),
				health_score: stored?.health_score ?? 0,
				last_verified_at: stored?.last_verified_at ?? null,
				last_error: stored?.last_error ?? null
			};
		})
	];

	// Calculate summary
	const connected = connections.filter((c) => c.is_connected);
	const errors = connections.filter((c) => c.status === 'error');

	return json({
		success: true,
		connections,
		summary: {
			total: connections.length,
			connected: connected.length,
			disconnected: connections.length - connected.length,
			errors: errors.length,
			health:
				connected.length > 0
					? Math.round(connected.reduce((sum, c) => sum + c.health_score, 0) / connected.length)
					: 0
		}
	});
};

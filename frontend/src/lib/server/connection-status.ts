/** Compatibility status view backed by the canonical persisted connection results. */
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

// All available external services
const EXTERNAL_SERVICES: Pick<ConnectionStatus, 'key' | 'name' | 'category'>[] = [
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

const BUILTIN_SERVICES = [
	{ key: 'fluent_forms_pro', name: 'FluentForms Pro', category: 'Fluent' },
	{ key: 'fluent_crm_pro', name: 'FluentCRM Pro', category: 'Fluent' },
	{ key: 'fluent_smtp', name: 'FluentSMTP', category: 'Fluent' },
	{ key: 'fluent_support', name: 'FluentSupport', category: 'Fluent' },
	{ key: 'fluent_booking', name: 'FluentBooking', category: 'Fluent' }
];
const object = (value: unknown): value is Record<string, unknown> =>
	typeof value === 'object' && value !== null && !Array.isArray(value);
const nullableString = (value: unknown): value is string | null =>
	value === null || typeof value === 'string';
export function connectionStatusView(input: unknown, query: URLSearchParams) {
	if (!object(input) || input.success !== true || !Array.isArray(input.connections))
		throw new Error('Invalid connections response');
	const statuses: ConnectionStatus[] = [];
	const keys: string[] = [];
	for (const row of input.connections) {
		if (
			!object(row) ||
			typeof row.key !== 'string' ||
			!/^[a-z][a-z0-9_]{0,99}$/.test(row.key) ||
			keys.includes(row.key) ||
			typeof row.name !== 'string' ||
			typeof row.category !== 'string' ||
			typeof row.is_connected !== 'boolean' ||
			typeof row.health_score !== 'number' ||
			!Number.isFinite(row.health_score) ||
			row.health_score < 0 ||
			row.health_score > 100 ||
			!['connected', 'disconnected', 'error', 'expired', 'pending'].includes(String(row.status)) ||
			row.is_connected !== (row.status === 'connected') ||
			typeof row.is_builtin !== 'boolean'
		)
			throw new Error('Invalid connection status');
		const detail = row.connection;
		if (detail !== null && !object(detail)) throw new Error('Invalid connection receipt');
		const verifiedAt = detail?.last_verified_at ?? null;
		const lastError = detail?.last_error ?? null;
		if (
			!nullableString(verifiedAt) ||
			!nullableString(lastError) ||
			(verifiedAt !== null && !Number.isFinite(Date.parse(verifiedAt))) ||
			(row.is_connected && verifiedAt === null)
		)
			throw new Error('Invalid connection receipt');
		statuses.push({
			key: row.key,
			name: row.name,
			category: row.category,
			is_connected: row.is_connected,
			status: row.status as ConnectionStatus['status'],
			health_score: row.is_connected ? row.health_score : 0,
			last_verified_at: verifiedAt,
			last_error: lastError,
			is_builtin: row.is_builtin
		});
		keys.push(row.key);
	}
	for (const service of [...BUILTIN_SERVICES, ...EXTERNAL_SERVICES]) {
		if (keys.includes(service.key)) continue;
		statuses.push({
			...service,
			is_connected: false,
			status: 'pending',
			health_score: 0,
			last_verified_at: null,
			last_error: 'No verification result is available for this integration',
			is_builtin: BUILTIN_SERVICES.some((builtin) => builtin.key === service.key)
		});
	}
	const category = query.get('category');
	const status = query.get('status');
	const connections = statuses.filter(
		(row) => (!category || row.category === category) && (!status || row.status === status)
	);
	const connected = connections.filter((row) => row.is_connected);
	return {
		success: true,
		connections,
		summary: {
			total: connections.length,
			connected: connected.length,
			disconnected: connections.length - connected.length,
			errors: connections.filter((row) => row.status === 'error').length,
			health: connected.length
				? Math.round(connected.reduce((sum, row) => sum + row.health_score, 0) / connected.length)
				: 0
		}
	};
}

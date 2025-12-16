/**
 * API Connections Management Endpoint
 *
 * Manages third-party API integrations for the platform.
 * Supports payment processors, analytics, email services, and more.
 *
 * @version 1.0.0 - December 2025
 */

import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

// Service definitions with all required fields
interface ServiceField {
	key: string;
	label: string;
	type: 'text' | 'password' | 'url' | 'email';
	required: boolean;
	placeholder?: string;
}

interface ServiceDefinition {
	key: string;
	name: string;
	category: string;
	description: string;
	icon: string;
	color: string;
	docs_url?: string;
	is_oauth: boolean;
	is_builtin?: boolean;
	fields: ServiceField[];
	environments?: string[];
}

interface ConnectionData {
	id: string;
	service_key: string;
	credentials: Record<string, string>;
	environment: string;
	status: 'connected' | 'error' | 'expired' | 'pending';
	health_score: number;
	health_status: string;
	connected_at: string | null;
	last_verified_at: string | null;
	api_calls_today: number;
	api_calls_total: number;
	last_error: string | null;
}

// In-memory storage (production would use database)
const connections: Map<string, ConnectionData> = new Map();

// All available services organized by category
const serviceDefinitions: ServiceDefinition[] = [
	// Payment Processors
	{
		key: 'stripe',
		name: 'Stripe',
		category: 'Payment',
		description: 'Accept payments, manage subscriptions, and handle payouts',
		icon: 'credit-card',
		color: '#635BFF',
		docs_url: 'https://stripe.com/docs',
		is_oauth: false,
		environments: ['production', 'sandbox'],
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true, placeholder: 'sk_live_...' },
			{ key: 'webhook_secret', label: 'Webhook Secret', type: 'password', required: true, placeholder: 'whsec_...' }
		]
	},
	{
		key: 'paypal',
		name: 'PayPal',
		category: 'Payment',
		description: 'PayPal payments, subscriptions, and checkout',
		icon: 'credit-card',
		color: '#003087',
		docs_url: 'https://developer.paypal.com/docs',
		is_oauth: true,
		environments: ['production', 'sandbox'],
		fields: [
			{ key: 'client_id', label: 'Client ID', type: 'text', required: true },
			{ key: 'client_secret', label: 'Client Secret', type: 'password', required: true }
		]
	},
	{
		key: 'square',
		name: 'Square',
		category: 'Payment',
		description: 'Square payments and point of sale integration',
		icon: 'credit-card',
		color: '#006AFF',
		docs_url: 'https://developer.squareup.com/docs',
		is_oauth: false,
		environments: ['production', 'sandbox'],
		fields: [
			{ key: 'access_token', label: 'Access Token', type: 'password', required: true },
			{ key: 'location_id', label: 'Location ID', type: 'text', required: true }
		]
	},

	// Fluent Ecosystem (Built-in CRM & Marketing)
	{
		key: 'fluent_crm_pro',
		name: 'FluentCRM Pro',
		category: 'CRM',
		description: 'Built-in CRM for email marketing, automation, and contact management (Already Installed)',
		icon: 'users',
		color: '#7C3AED',
		docs_url: 'https://fluentcrm.com/docs',
		is_oauth: false,
		is_builtin: true,
		fields: []
	},
	{
		key: 'fluent_forms_pro',
		name: 'FluentForms Pro',
		category: 'Forms',
		description: 'Built-in form builder with advanced features (Already Installed)',
		icon: 'box',
		color: '#10B981',
		docs_url: 'https://fluentforms.com/docs',
		is_oauth: false,
		is_builtin: true,
		fields: []
	},
	{
		key: 'fluent_smtp',
		name: 'FluentSMTP',
		category: 'Email',
		description: 'Built-in SMTP mailer for reliable email delivery (Already Installed)',
		icon: 'mail',
		color: '#3B82F6',
		docs_url: 'https://fluentsmtp.com/docs',
		is_oauth: false,
		is_builtin: true,
		fields: []
	},

	// Email Delivery (Transactional only - no marketing)
	{
		key: 'sendgrid',
		name: 'SendGrid',
		category: 'Email',
		description: 'Transactional email delivery and analytics',
		icon: 'mail',
		color: '#1A82E2',
		docs_url: 'https://docs.sendgrid.com',
		is_oauth: false,
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true }
		]
	},
	// NOTE: Email marketing is handled by built-in FluentCRM Pro
	// External services like ConvertKit and ActiveCampaign removed to avoid duplication

	// Analytics
	{
		key: 'google_analytics',
		name: 'Google Analytics 4',
		category: 'Analytics',
		description: 'Website analytics and conversion tracking',
		icon: 'chart-bar',
		color: '#F9AB00',
		docs_url: 'https://developers.google.com/analytics',
		is_oauth: true,
		fields: [
			{ key: 'measurement_id', label: 'Measurement ID', type: 'text', required: true, placeholder: 'G-XXXXXXXX' },
			{ key: 'api_secret', label: 'API Secret', type: 'password', required: false }
		]
	},
	{
		key: 'mixpanel',
		name: 'Mixpanel',
		category: 'Analytics',
		description: 'Product analytics and user behavior tracking',
		icon: 'chart-bar',
		color: '#7856FF',
		docs_url: 'https://developer.mixpanel.com',
		is_oauth: false,
		fields: [
			{ key: 'project_token', label: 'Project Token', type: 'password', required: true },
			{ key: 'api_secret', label: 'API Secret', type: 'password', required: false }
		]
	},
	{
		key: 'amplitude',
		name: 'Amplitude',
		category: 'Analytics',
		description: 'Product analytics and behavioral insights',
		icon: 'chart-bar',
		color: '#1D3D90',
		docs_url: 'https://www.docs.developers.amplitude.com',
		is_oauth: false,
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true }
		]
	},
	{
		key: 'segment',
		name: 'Segment',
		category: 'Analytics',
		description: 'Customer data platform and event routing',
		icon: 'chart-bar',
		color: '#52BD94',
		docs_url: 'https://segment.com/docs',
		is_oauth: false,
		fields: [
			{ key: 'write_key', label: 'Write Key', type: 'password', required: true }
		]
	},

	// Cloud Storage
	{
		key: 'aws_s3',
		name: 'Amazon S3',
		category: 'Storage',
		description: 'Cloud object storage for media and files',
		icon: 'cloud',
		color: '#FF9900',
		docs_url: 'https://docs.aws.amazon.com/s3',
		is_oauth: false,
		fields: [
			{ key: 'access_key_id', label: 'Access Key ID', type: 'text', required: true },
			{ key: 'secret_access_key', label: 'Secret Access Key', type: 'password', required: true },
			{ key: 'bucket_name', label: 'Bucket Name', type: 'text', required: true },
			{ key: 'region', label: 'Region', type: 'text', required: true, placeholder: 'us-east-1' }
		]
	},
	{
		key: 'cloudflare_r2',
		name: 'Cloudflare R2',
		category: 'Storage',
		description: 'Zero egress fee cloud storage',
		icon: 'cloud',
		color: '#F6821F',
		docs_url: 'https://developers.cloudflare.com/r2',
		is_oauth: false,
		fields: [
			{ key: 'account_id', label: 'Account ID', type: 'text', required: true },
			{ key: 'access_key_id', label: 'Access Key ID', type: 'text', required: true },
			{ key: 'secret_access_key', label: 'Secret Access Key', type: 'password', required: true },
			{ key: 'bucket_name', label: 'Bucket Name', type: 'text', required: true }
		]
	},

	// CRM
	{
		key: 'hubspot',
		name: 'HubSpot',
		category: 'CRM',
		description: 'CRM, marketing, and sales automation',
		icon: 'users',
		color: '#FF7A59',
		docs_url: 'https://developers.hubspot.com',
		is_oauth: true,
		fields: [
			{ key: 'api_key', label: 'Private App Token', type: 'password', required: true }
		]
	},
	{
		key: 'salesforce',
		name: 'Salesforce',
		category: 'CRM',
		description: 'Enterprise CRM and customer management',
		icon: 'users',
		color: '#00A1E0',
		docs_url: 'https://developer.salesforce.com',
		is_oauth: true,
		fields: [
			{ key: 'client_id', label: 'Consumer Key', type: 'text', required: true },
			{ key: 'client_secret', label: 'Consumer Secret', type: 'password', required: true },
			{ key: 'username', label: 'Username', type: 'email', required: true },
			{ key: 'password', label: 'Password + Security Token', type: 'password', required: true }
		]
	},

	// Social Media
	{
		key: 'twitter',
		name: 'Twitter/X',
		category: 'Social',
		description: 'Twitter/X API for social sharing and engagement',
		icon: 'share-2',
		color: '#1DA1F2',
		docs_url: 'https://developer.twitter.com/en/docs',
		is_oauth: true,
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true },
			{ key: 'api_secret', label: 'API Secret', type: 'password', required: true },
			{ key: 'access_token', label: 'Access Token', type: 'password', required: true },
			{ key: 'access_token_secret', label: 'Access Token Secret', type: 'password', required: true }
		]
	},
	{
		key: 'facebook',
		name: 'Facebook',
		category: 'Social',
		description: 'Facebook Graph API for social integration',
		icon: 'share-2',
		color: '#1877F2',
		docs_url: 'https://developers.facebook.com',
		is_oauth: true,
		fields: [
			{ key: 'app_id', label: 'App ID', type: 'text', required: true },
			{ key: 'app_secret', label: 'App Secret', type: 'password', required: true }
		]
	},

	// Communication
	{
		key: 'twilio',
		name: 'Twilio',
		category: 'Communication',
		description: 'SMS, voice, and communication APIs',
		icon: 'message-circle',
		color: '#F22F46',
		docs_url: 'https://www.twilio.com/docs',
		is_oauth: false,
		fields: [
			{ key: 'account_sid', label: 'Account SID', type: 'text', required: true },
			{ key: 'auth_token', label: 'Auth Token', type: 'password', required: true },
			{ key: 'phone_number', label: 'Phone Number', type: 'text', required: true, placeholder: '+1234567890' }
		]
	},
	{
		key: 'slack',
		name: 'Slack',
		category: 'Communication',
		description: 'Slack notifications and team collaboration',
		icon: 'message-circle',
		color: '#4A154B',
		docs_url: 'https://api.slack.com',
		is_oauth: true,
		fields: [
			{ key: 'webhook_url', label: 'Webhook URL', type: 'url', required: true },
			{ key: 'bot_token', label: 'Bot Token', type: 'password', required: false }
		]
	},
	{
		key: 'discord',
		name: 'Discord',
		category: 'Communication',
		description: 'Discord bot and webhook integration',
		icon: 'message-circle',
		color: '#5865F2',
		docs_url: 'https://discord.com/developers/docs',
		is_oauth: true,
		fields: [
			{ key: 'webhook_url', label: 'Webhook URL', type: 'url', required: true },
			{ key: 'bot_token', label: 'Bot Token', type: 'password', required: false }
		]
	},

	// AI & Machine Learning
	{
		key: 'openai',
		name: 'OpenAI',
		category: 'AI',
		description: 'GPT-4, DALL-E, and other AI models',
		icon: 'cpu',
		color: '#412991',
		docs_url: 'https://platform.openai.com/docs',
		is_oauth: false,
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true, placeholder: 'sk-...' }
		]
	},
	{
		key: 'anthropic',
		name: 'Anthropic',
		category: 'AI',
		description: 'Claude AI models for intelligent assistance',
		icon: 'cpu',
		color: '#C4773B',
		docs_url: 'https://docs.anthropic.com',
		is_oauth: false,
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true }
		]
	},

	// Search
	{
		key: 'algolia',
		name: 'Algolia',
		category: 'Search',
		description: 'Fast, reliable site search and discovery',
		icon: 'search',
		color: '#003DFF',
		docs_url: 'https://www.algolia.com/doc',
		is_oauth: false,
		fields: [
			{ key: 'app_id', label: 'Application ID', type: 'text', required: true },
			{ key: 'api_key', label: 'Admin API Key', type: 'password', required: true },
			{ key: 'search_key', label: 'Search API Key', type: 'password', required: true }
		]
	},
	{
		key: 'elasticsearch',
		name: 'Elasticsearch',
		category: 'Search',
		description: 'Distributed search and analytics engine',
		icon: 'search',
		color: '#FEC514',
		docs_url: 'https://www.elastic.co/guide',
		is_oauth: false,
		fields: [
			{ key: 'cloud_id', label: 'Cloud ID', type: 'text', required: true },
			{ key: 'api_key', label: 'API Key', type: 'password', required: true }
		]
	},

	// Trading & Finance
	{
		key: 'tradier',
		name: 'Tradier',
		category: 'Trading',
		description: 'Stock and options trading API',
		icon: 'trending-up',
		color: '#3B82F6',
		docs_url: 'https://documentation.tradier.com',
		is_oauth: true,
		environments: ['production', 'sandbox'],
		fields: [
			{ key: 'access_token', label: 'Access Token', type: 'password', required: true },
			{ key: 'account_id', label: 'Account ID', type: 'text', required: true }
		]
	},
	{
		key: 'alpaca',
		name: 'Alpaca',
		category: 'Trading',
		description: 'Commission-free stock and crypto trading',
		icon: 'trending-up',
		color: '#F7D300',
		docs_url: 'https://alpaca.markets/docs',
		is_oauth: false,
		environments: ['production', 'paper'],
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true },
			{ key: 'api_secret', label: 'API Secret', type: 'password', required: true }
		]
	},
	{
		key: 'polygon',
		name: 'Polygon.io',
		category: 'Trading',
		description: 'Real-time and historical market data',
		icon: 'trending-up',
		color: '#7C3AED',
		docs_url: 'https://polygon.io/docs',
		is_oauth: false,
		fields: [
			{ key: 'api_key', label: 'API Key', type: 'password', required: true }
		]
	},

	// Video & Media
	{
		key: 'mux',
		name: 'Mux',
		category: 'Media',
		description: 'Video streaming and analytics platform',
		icon: 'box',
		color: '#FA50B5',
		docs_url: 'https://docs.mux.com',
		is_oauth: false,
		fields: [
			{ key: 'token_id', label: 'Token ID', type: 'text', required: true },
			{ key: 'token_secret', label: 'Token Secret', type: 'password', required: true }
		]
	},
	{
		key: 'cloudinary',
		name: 'Cloudinary',
		category: 'Media',
		description: 'Image and video management platform',
		icon: 'box',
		color: '#3448C5',
		docs_url: 'https://cloudinary.com/documentation',
		is_oauth: false,
		fields: [
			{ key: 'cloud_name', label: 'Cloud Name', type: 'text', required: true },
			{ key: 'api_key', label: 'API Key', type: 'text', required: true },
			{ key: 'api_secret', label: 'API Secret', type: 'password', required: true }
		]
	},
	{
		key: 'vimeo',
		name: 'Vimeo',
		category: 'Media',
		description: 'Video hosting and streaming',
		icon: 'box',
		color: '#1AB7EA',
		docs_url: 'https://developer.vimeo.com',
		is_oauth: true,
		fields: [
			{ key: 'access_token', label: 'Access Token', type: 'password', required: true }
		]
	}
];

// Helper to get service by key
function getService(key: string): ServiceDefinition | undefined {
	return serviceDefinitions.find(s => s.key === key);
}

// Helper to build category map
function buildCategories() {
	const categories: Record<string, { name: string; icon: string; services: any[] }> = {};

	for (const service of serviceDefinitions) {
		if (!categories[service.category]) {
			categories[service.category] = {
				name: service.category,
				icon: service.icon,
				services: []
			};
		}

		const connection = connections.get(service.key);
		categories[service.category].services.push({
			...service,
			is_connected: !!connection,
			status: connection?.status || 'disconnected',
			connection: connection || null
		});
	}

	return categories;
}

// Helper to build summary
function buildSummary() {
	let total_connected = 0;
	let total_errors = 0;
	let needs_attention = 0;

	for (const [, connection] of connections) {
		if (connection.status === 'connected') total_connected++;
		if (connection.status === 'error') total_errors++;
		if (connection.status === 'error' || connection.status === 'expired') needs_attention++;
	}

	return {
		total_available: serviceDefinitions.length,
		total_connected,
		total_disconnected: serviceDefinitions.length - total_connected,
		total_errors,
		needs_attention
	};
}

// GET - List all connections and available services
export const GET: RequestHandler = async () => {
	const categories = buildCategories();
	const summary = buildSummary();

	// Build flat connections list with service details
	const connectionsList = serviceDefinitions.map(service => {
		const connection = connections.get(service.key);
		return {
			...service,
			is_connected: !!connection,
			status: connection?.status || 'disconnected',
			health_score: connection?.health_score || 0,
			connection: connection || null
		};
	});

	return json({
		success: true,
		connections: connectionsList,
		categories,
		summary
	});
};

// POST - Connect a service (for OAuth or initial setup)
export const POST: RequestHandler = async ({ request }) => {
	try {
		const body = await request.json();
		const { service_key } = body;

		const service = getService(service_key);
		if (!service) {
			throw error(404, `Service '${service_key}' not found`);
		}

		// For OAuth services, generate auth URL
		if (service.is_oauth) {
			// In production, generate actual OAuth URL
			return json({
				success: true,
				data: {
					oauth_url: `https://oauth.example.com/authorize?service=${service_key}`,
					state: crypto.randomUUID()
				}
			});
		}

		return json({
			success: false,
			error: 'Use the connect endpoint for non-OAuth services'
		});
	} catch (err) {
		if (err instanceof Error && 'status' in err) {
			throw err;
		}
		throw error(500, 'Failed to initiate connection');
	}
};

<!--
	URL: /admin/settings
-->

<script lang="ts">
	/**
	 * API Settings & Integrations - Apple ICT7 Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Premium settings dashboard with real API connections, live status monitoring,
	 * and beautiful Apple-inspired UI with glass morphism effects.
	 *
	 * Features:
	 * - Real API connection management with adminFetch utility
	 * - OAuth flow support for Google services
	 * - Live connection testing with actual API calls
	 * - Health monitoring with real metrics
	 * - General settings with API persistence
	 * - Svelte 5 runes: $state, $derived.by, $effect
	 */

	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, scale, slide } from 'svelte/transition';
	import { quintOut, backOut, elasticOut } from 'svelte/easing';
	import { toastStore } from '$lib/stores/toast';
	import { connections, isAnalyticsConnected, isSeoConnected } from '$lib/stores/connections';
	import { adminFetch, adminApi, AdminApiError } from '$lib/utils/adminFetch';

	// Types
	interface ServiceField {
		key: string;
		label: string;
		type: string;
		required: boolean;
		placeholder?: string;
	}

	interface Service {
		key: string;
		name: string;
		category: string;
		description: string;
		icon: string;
		color: string;
		docs_url?: string;
		is_oauth: boolean;
		fields: ServiceField[];
		environments?: string[];
		connection?: ConnectionData | null;
		is_connected: boolean;
		status: 'connected' | 'disconnected' | 'error' | 'expired' | 'pending' | 'connecting';
		health_score?: number;
	}

	interface ConnectionData {
		id: string;
		service_key: string;
		status: string;
		health_score: number;
		health_status: string;
		connected_at: string | null;
		last_verified_at: string | null;
		api_calls_today: number;
		api_calls_total: number;
		last_error: string | null;
	}

	interface Category {
		name: string;
		icon: string;
		services: Service[];
	}

	interface Summary {
		total_available: number;
		total_connected: number;
		total_disconnected: number;
		total_errors: number;
		needs_attention: number;
	}

	// General Settings interfaces
	interface GeneralSettings {
		site_name: string;
		site_url: string;
		admin_email: string;
		timezone: string;
		date_format: string;
		maintenance_mode: boolean;
		debug_mode: boolean;
		allow_registration: boolean;
		require_email_verification: boolean;
		session_lifetime: number;
		api_rate_limit: number;
	}

	interface SettingsCategory {
		key: string;
		name: string;
		icon: string;
		description: string;
	}

	// State
	let allServices = $state<Service[]>([]);
	let categories = $state<Record<string, Category>>({});
	let summary = $state<Summary>({
		total_available: 0,
		total_connected: 0,
		total_disconnected: 0,
		total_errors: 0,
		needs_attention: 0
	});
	let isLoading = $state(true);
	let activeTab = $state<'integrations' | 'general' | 'notifications' | 'email' | 'backup' | 'performance'>('integrations');
	let selectedCategory = $state<string | null>(null);
	let searchQuery = $state('');
	let showConnectModal = $state(false);
	let selectedService = $state<Service | null>(null);
	let isConnecting = $state(false);
	let isTesting = $state(false);
	let isDisconnecting = $state(false);
	let testResult = $state<{ success: boolean; message?: string; error?: string; latency?: number } | null>(null);
	let credentialValues = $state<Record<string, string>>({});
	let selectedEnvironment = $state('production');
	let showDisconnectConfirm = $state(false);
	let disconnectingService = $state<Service | null>(null);
	let refreshInterval: ReturnType<typeof setInterval> | null = null;

	// General Settings State
	let generalSettings = $state<GeneralSettings>({
		site_name: 'Revolution Trading Pros',
		site_url: '',
		admin_email: '',
		timezone: 'UTC',
		date_format: 'MM/DD/YYYY',
		maintenance_mode: false,
		debug_mode: false,
		allow_registration: true,
		require_email_verification: true,
		session_lifetime: 120,
		api_rate_limit: 1000
	});
	let isLoadingSettings = $state(false);
	let isSavingSettings = $state(false);
	let settingsError = $state<string | null>(null);
	let settingsChanged = $state(false);
	let originalSettings = $state<GeneralSettings | null>(null);

	// Notification Settings State
	interface NotificationSettings {
		email_alerts: boolean;
		push_notifications: boolean;
		sms_notifications: boolean;
		digest_frequency: 'instant' | 'hourly' | 'daily' | 'weekly';
		alert_on_new_users: boolean;
		alert_on_payments: boolean;
		alert_on_errors: boolean;
		alert_on_security: boolean;
		quiet_hours_enabled: boolean;
		quiet_hours_start: string;
		quiet_hours_end: string;
	}

	let notificationSettings = $state<NotificationSettings>({
		email_alerts: true,
		push_notifications: true,
		sms_notifications: false,
		digest_frequency: 'daily',
		alert_on_new_users: true,
		alert_on_payments: true,
		alert_on_errors: true,
		alert_on_security: true,
		quiet_hours_enabled: false,
		quiet_hours_start: '22:00',
		quiet_hours_end: '08:00'
	});

	// Email Settings State
	interface EmailSettings {
		smtp_host: string;
		smtp_port: number;
		smtp_username: string;
		smtp_password: string;
		smtp_encryption: 'none' | 'tls' | 'ssl';
		from_name: string;
		from_email: string;
		reply_to: string;
		email_queue_enabled: boolean;
		email_tracking_enabled: boolean;
	}

	let emailSettings = $state<EmailSettings>({
		smtp_host: '',
		smtp_port: 587,
		smtp_username: '',
		smtp_password: '',
		smtp_encryption: 'tls',
		from_name: 'Revolution Trading Pros',
		from_email: '',
		reply_to: '',
		email_queue_enabled: true,
		email_tracking_enabled: true
	});

	// Backup Settings State
	interface BackupSettings {
		auto_backup_enabled: boolean;
		backup_frequency: 'hourly' | 'daily' | 'weekly' | 'monthly';
		backup_retention_days: number;
		backup_storage: 'local' | 's3' | 'gcs' | 'azure';
		include_uploads: boolean;
		include_database: boolean;
		include_logs: boolean;
		compression_enabled: boolean;
		encryption_enabled: boolean;
		last_backup_at: string | null;
		next_backup_at: string | null;
	}

	let backupSettings = $state<BackupSettings>({
		auto_backup_enabled: true,
		backup_frequency: 'daily',
		backup_retention_days: 30,
		backup_storage: 'local',
		include_uploads: true,
		include_database: true,
		include_logs: false,
		compression_enabled: true,
		encryption_enabled: false,
		last_backup_at: null,
		next_backup_at: null
	});

	// Performance Settings State
	interface PerformanceSettings {
		cache_enabled: boolean;
		cache_driver: 'file' | 'redis' | 'memcached';
		cache_ttl: number;
		cdn_enabled: boolean;
		cdn_url: string;
		image_optimization: boolean;
		lazy_loading: boolean;
		minify_html: boolean;
		minify_css: boolean;
		minify_js: boolean;
		gzip_enabled: boolean;
		browser_cache_days: number;
	}

	let performanceSettings = $state<PerformanceSettings>({
		cache_enabled: true,
		cache_driver: 'file',
		cache_ttl: 3600,
		cdn_enabled: false,
		cdn_url: '',
		image_optimization: true,
		lazy_loading: true,
		minify_html: false,
		minify_css: true,
		minify_js: true,
		gzip_enabled: true,
		browser_cache_days: 7
	});

	// Settings category tracking
	let notificationsChanged = $state(false);
	let emailChanged = $state(false);
	let backupChanged = $state(false);
	let performanceChanged = $state(false);

	let isSavingNotifications = $state(false);
	let isSavingEmail = $state(false);
	let isSavingBackup = $state(false);
	let isSavingPerformance = $state(false);
	let isTestingEmail = $state(false);
	let isCreatingBackup = $state(false);

	// Timezone options
	const timezones = [
		'UTC', 'America/New_York', 'America/Chicago', 'America/Denver',
		'America/Los_Angeles', 'Europe/London', 'Europe/Paris', 'Asia/Tokyo',
		'Asia/Shanghai', 'Australia/Sydney'
	];

	// Date format options
	const dateFormats = [
		{ value: 'MM/DD/YYYY', label: 'MM/DD/YYYY (US)' },
		{ value: 'DD/MM/YYYY', label: 'DD/MM/YYYY (EU)' },
		{ value: 'YYYY-MM-DD', label: 'YYYY-MM-DD (ISO)' }
	];

	// Derived - using $derived.by for complex computed values per Svelte 5 spec
	let filteredServices = $derived.by(() => {
		let result = allServices;

		if (selectedCategory) {
			result = result.filter((s) => s.category === selectedCategory);
		}

		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(s) =>
					s.name.toLowerCase().includes(query) ||
					s.description.toLowerCase().includes(query) ||
					s.category.toLowerCase().includes(query)
			);
		}

		return result;
	});

	let connectedServices = $derived(allServices.filter(s => s.is_connected));
	let disconnectedServices = $derived(allServices.filter(s => !s.is_connected));
	let errorServices = $derived(allServices.filter(s => s.status === 'error'));
	let categoryList = $derived(Object.entries(categories) as [string, Category][]);

	// Google-specific services for quick access
	let googleServices = $derived(allServices.filter(s =>
		['google_analytics', 'google_search_console', 'google_tag_manager', 'google_ads'].includes(s.key)
	));

	// Fetch services data
	async function fetchServices() {
		try {
			isLoading = true;
			const data = await adminFetch('/api/admin/connections');
			allServices = data.connections || [];
			categories = data.categories || {};
			summary = data.summary || {
				total_available: 0,
				total_connected: 0,
				total_disconnected: 0,
				total_errors: 0,
				needs_attention: 0
			};
		} catch (error) {
			console.error('Failed to fetch services:', error);
			// Fallback to static services
			initializeStaticServices();
		} finally {
			isLoading = false;
		}
	}

	// Initialize with static service list when API is not available
	function initializeStaticServices() {
		const staticServices: Service[] = [
			// Google Services
			{
				key: 'google_analytics',
				name: 'Google Analytics 4',
				category: 'analytics',
				description: 'Track website traffic and user behavior with GA4',
				icon: 'google-analytics',
				color: '#F9AB00',
				docs_url: 'https://developers.google.com/analytics',
				is_oauth: true,
				fields: [
					{ key: 'measurement_id', label: 'Measurement ID', type: 'text', required: true, placeholder: 'G-XXXXXXXXXX' },
					{ key: 'property_id', label: 'Property ID', type: 'text', required: false, placeholder: '123456789' }
				],
				is_connected: false,
				status: 'disconnected'
			},
			{
				key: 'google_search_console',
				name: 'Google Search Console',
				category: 'analytics',
				description: 'Monitor search performance and indexing status',
				icon: 'google',
				color: '#4285F4',
				docs_url: 'https://search.google.com/search-console',
				is_oauth: true,
				fields: [
					{ key: 'site_url', label: 'Site URL', type: 'url', required: true, placeholder: 'https://yoursite.com' }
				],
				is_connected: false,
				status: 'disconnected'
			},
			{
				key: 'google_tag_manager',
				name: 'Google Tag Manager',
				category: 'analytics',
				description: 'Manage marketing tags without code changes',
				icon: 'google',
				color: '#246FDB',
				docs_url: 'https://tagmanager.google.com',
				is_oauth: false,
				fields: [
					{ key: 'container_id', label: 'Container ID', type: 'text', required: true, placeholder: 'GTM-XXXXXXX' }
				],
				is_connected: false,
				status: 'disconnected'
			},
			{
				key: 'google_ads',
				name: 'Google Ads',
				category: 'analytics',
				description: 'Track conversions and optimize ad campaigns',
				icon: 'google',
				color: '#34A853',
				docs_url: 'https://ads.google.com',
				is_oauth: true,
				fields: [
					{ key: 'conversion_id', label: 'Conversion ID', type: 'text', required: true, placeholder: 'AW-XXXXXXXXX' },
					{ key: 'conversion_label', label: 'Conversion Label', type: 'text', required: false }
				],
				is_connected: false,
				status: 'disconnected'
			},
			// Payment Services
			{
				key: 'stripe',
				name: 'Stripe',
				category: 'payments',
				description: 'Accept payments with credit cards, Apple Pay, Google Pay',
				icon: 'stripe',
				color: '#635BFF',
				docs_url: 'https://stripe.com/docs/api',
				is_oauth: false,
				fields: [
					{ key: 'publishable_key', label: 'Publishable Key', type: 'text', required: true, placeholder: 'pk_live_...' },
					{ key: 'secret_key', label: 'Secret Key', type: 'password', required: true, placeholder: 'sk_live_...' },
					{ key: 'webhook_secret', label: 'Webhook Secret', type: 'password', required: false, placeholder: 'whsec_...' }
				],
				environments: ['production', 'test'],
				is_connected: false,
				status: 'disconnected'
			},
			{
				key: 'paypal',
				name: 'PayPal',
				category: 'payments',
				description: 'Accept PayPal payments and PayPal Credit',
				icon: 'paypal',
				color: '#003087',
				docs_url: 'https://developer.paypal.com',
				is_oauth: true,
				fields: [
					{ key: 'client_id', label: 'Client ID', type: 'text', required: true },
					{ key: 'client_secret', label: 'Client Secret', type: 'password', required: true }
				],
				environments: ['production', 'sandbox'],
				is_connected: false,
				status: 'disconnected'
			},
			// Email Services
			{
				key: 'sendgrid',
				name: 'SendGrid',
				category: 'email',
				description: 'Transactional and marketing email delivery',
				icon: 'sendgrid',
				color: '#1A82E2',
				docs_url: 'https://docs.sendgrid.com',
				is_oauth: false,
				fields: [
					{ key: 'api_key', label: 'API Key', type: 'password', required: true, placeholder: 'SG.xxx...' }
				],
				is_connected: false,
				status: 'disconnected'
			},
			{
				key: 'mailchimp',
				name: 'Mailchimp',
				category: 'email',
				description: 'Email marketing and audience management',
				icon: 'mailchimp',
				color: '#FFE01B',
				is_oauth: true,
				fields: [
					{ key: 'api_key', label: 'API Key', type: 'password', required: true },
					{ key: 'server_prefix', label: 'Server Prefix', type: 'text', required: true, placeholder: 'us1' }
				],
				is_connected: false,
				status: 'disconnected'
			},
			// CRM
			{
				key: 'hubspot',
				name: 'HubSpot',
				category: 'crm',
				description: 'CRM, marketing, sales, and service platform',
				icon: 'hubspot',
				color: '#FF7A59',
				docs_url: 'https://developers.hubspot.com',
				is_oauth: true,
				fields: [
					{ key: 'api_key', label: 'Private App Token', type: 'password', required: true }
				],
				is_connected: false,
				status: 'disconnected'
			},
			// Social
			{
				key: 'facebook',
				name: 'Facebook Pixel',
				category: 'social',
				description: 'Track conversions and build audiences for Facebook ads',
				icon: 'facebook',
				color: '#1877F2',
				is_oauth: true,
				fields: [
					{ key: 'pixel_id', label: 'Pixel ID', type: 'text', required: true, placeholder: '123456789...' }
				],
				is_connected: false,
				status: 'disconnected'
			},
			// AI
			{
				key: 'openai',
				name: 'OpenAI',
				category: 'ai',
				description: 'GPT-4, DALL-E, and AI capabilities',
				icon: 'openai',
				color: '#10A37F',
				docs_url: 'https://platform.openai.com/docs',
				is_oauth: false,
				fields: [
					{ key: 'api_key', label: 'API Key', type: 'password', required: true, placeholder: 'sk-...' },
					{ key: 'organization_id', label: 'Organization ID', type: 'text', required: false }
				],
				is_connected: false,
				status: 'disconnected'
			},
			// Monitoring
			{
				key: 'sentry',
				name: 'Sentry',
				category: 'monitoring',
				description: 'Error tracking and performance monitoring',
				icon: 'sentry',
				color: '#362D59',
				docs_url: 'https://docs.sentry.io',
				is_oauth: false,
				fields: [
					{ key: 'dsn', label: 'DSN', type: 'text', required: true, placeholder: 'https://xxx@xxx.ingest.sentry.io/xxx' }
				],
				is_connected: false,
				status: 'disconnected'
			}
		];

		allServices = staticServices;

		// Group by category
		const grouped: Record<string, Category> = {};
		for (const service of staticServices) {
			if (!grouped[service.category]) {
				grouped[service.category] = {
					name: getCategoryName(service.category),
					icon: getCategoryIcon(service.category),
					services: []
				};
			}
			grouped[service.category].services.push(service);
		}
		categories = grouped;

		summary = {
			total_available: staticServices.length,
			total_connected: 0,
			total_disconnected: staticServices.length,
			total_errors: 0,
			needs_attention: 0
		};
	}

	// Connect service - using adminApi for authenticated requests
	async function connectService() {
		if (!selectedService) return;

		isConnecting = true;
		testResult = null;

		// Validate required fields
		for (const field of selectedService.fields) {
			if (field.required && !credentialValues[field.key]) {
				testResult = { success: false, error: `${field.label} is required` };
				isConnecting = false;
				return;
			}
		}

		try {
			const startTime = performance.now();
			const data = await adminApi.post(`/api/admin/connections/${selectedService.key}/connect`, {
				credentials: credentialValues,
				environment: selectedEnvironment
			});

			const latency = Math.round(performance.now() - startTime);

			if (data.success) {
				// Update local state with reactive assignment
				const idx = allServices.findIndex(s => s.key === selectedService!.key);
				if (idx !== -1) {
					allServices[idx] = {
						...allServices[idx],
						is_connected: true,
						status: 'connected',
						connection: data.connection
					};
				}

				showConnectModal = false;
				toastStore.success(`${selectedService.name} connected successfully! (${latency}ms)`);
				await fetchServices();
			} else {
				testResult = {
					success: false,
					error: data.error || 'Connection failed. Please check your credentials.',
					latency
				};
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				testResult = { success: false, error: error.message };
			} else {
				testResult = { success: false, error: 'Network error. Please check your connection.' };
			}
		} finally {
			isConnecting = false;
		}
	}

	// Test connection - using adminApi for authenticated requests
	async function testConnection() {
		if (!selectedService) return;

		isTesting = true;
		testResult = null;

		try {
			const startTime = performance.now();
			const data = await adminApi.post(`/api/admin/connections/${selectedService.key}/test`, {
				credentials: credentialValues
			});

			const latency = Math.round(performance.now() - startTime);

			testResult = {
				...data,
				latency,
				message: data.success ? `Connection verified in ${latency}ms` : data.error
			};
		} catch (error) {
			if (error instanceof AdminApiError) {
				testResult = { success: false, error: error.message };
			} else {
				testResult = { success: false, error: 'Test failed. Network error.' };
			}
		} finally {
			isTesting = false;
		}
	}

	// Disconnect service - using adminApi for authenticated requests
	async function disconnectService() {
		if (!disconnectingService) return;

		isDisconnecting = true;

		try {
			const data = await adminApi.post(
				`/api/admin/connections/${disconnectingService.key}/disconnect`
			);

			if (data.success) {
				// Update local state with reactive assignment
				const idx = allServices.findIndex(s => s.key === disconnectingService!.key);
				if (idx !== -1) {
					allServices[idx] = {
						...allServices[idx],
						is_connected: false,
						status: 'disconnected',
						connection: null
					};
				}

				showDisconnectConfirm = false;
				toastStore.success(`${disconnectingService.name} disconnected successfully`);
				await fetchServices();
			} else {
				toastStore.error(data.error || 'Failed to disconnect');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isDisconnecting = false;
		}
	}

	// Fetch general settings
	async function fetchGeneralSettings() {
		isLoadingSettings = true;
		settingsError = null;

		try {
			const data = await adminApi.get<{ settings: GeneralSettings }>('/api/admin/settings');
			if (data.settings) {
				generalSettings = { ...data.settings };
				originalSettings = { ...data.settings };
				settingsChanged = false;
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				settingsError = error.message;
			} else {
				settingsError = 'Failed to load settings';
			}
			console.error('Failed to fetch settings:', error);
		} finally {
			isLoadingSettings = false;
		}
	}

	// Save general settings
	async function saveGeneralSettings() {
		isSavingSettings = true;
		settingsError = null;

		try {
			const data = await adminApi.put('/api/admin/settings', generalSettings);
			if (data.success) {
				originalSettings = { ...generalSettings };
				settingsChanged = false;
				toastStore.success('Settings saved successfully');
			} else {
				settingsError = data.error || 'Failed to save settings';
				toastStore.error(settingsError ?? 'Failed to save settings');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				settingsError = error.message;
				toastStore.error(error.message);
			} else {
				settingsError = 'Network error. Please try again.';
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isSavingSettings = false;
		}
	}

	// Reset settings to original values
	function resetSettings() {
		if (originalSettings) {
			generalSettings = { ...originalSettings };
			settingsChanged = false;
		}
	}

	// Save notification settings
	async function saveNotificationSettings() {
		isSavingNotifications = true;
		try {
			const data = await adminApi.put('/api/admin/settings/notifications', notificationSettings);
			if (data.success) {
				notificationsChanged = false;
				toastStore.success('Notification settings saved successfully');
			} else {
				toastStore.error(data.error || 'Failed to save notification settings');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isSavingNotifications = false;
		}
	}

	// Save email settings
	async function saveEmailSettings() {
		isSavingEmail = true;
		try {
			const data = await adminApi.put('/api/admin/settings/email', emailSettings);
			if (data.success) {
				emailChanged = false;
				toastStore.success('Email settings saved successfully');
			} else {
				toastStore.error(data.error || 'Failed to save email settings');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isSavingEmail = false;
		}
	}

	// Test email configuration
	async function testEmailSettings() {
		isTestingEmail = true;
		try {
			const data = await adminApi.post('/api/admin/settings/email/test', emailSettings);
			if (data.success) {
				toastStore.success('Test email sent successfully! Check your inbox.');
			} else {
				toastStore.error(data.error || 'Failed to send test email');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isTestingEmail = false;
		}
	}

	// Save backup settings
	async function saveBackupSettings() {
		isSavingBackup = true;
		try {
			const data = await adminApi.put('/api/admin/settings/backup', backupSettings);
			if (data.success) {
				backupChanged = false;
				toastStore.success('Backup settings saved successfully');
			} else {
				toastStore.error(data.error || 'Failed to save backup settings');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isSavingBackup = false;
		}
	}

	// Create manual backup
	async function createManualBackup() {
		isCreatingBackup = true;
		try {
			const data = await adminApi.post('/api/admin/settings/backup/create');
			if (data.success) {
				toastStore.success('Backup created successfully');
				backupSettings.last_backup_at = new Date().toISOString();
			} else {
				toastStore.error(data.error || 'Failed to create backup');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isCreatingBackup = false;
		}
	}

	// Save performance settings
	async function savePerformanceSettings() {
		isSavingPerformance = true;
		try {
			const data = await adminApi.put('/api/admin/settings/performance', performanceSettings);
			if (data.success) {
				performanceChanged = false;
				toastStore.success('Performance settings saved successfully');
			} else {
				toastStore.error(data.error || 'Failed to save performance settings');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		} finally {
			isSavingPerformance = false;
		}
	}

	// Clear cache
	async function clearCache() {
		try {
			const data = await adminApi.post('/api/admin/settings/cache/clear');
			if (data.success) {
				toastStore.success('Cache cleared successfully');
			} else {
				toastStore.error(data.error || 'Failed to clear cache');
			}
		} catch (error) {
			if (error instanceof AdminApiError) {
				toastStore.error(error.message);
			} else {
				toastStore.error('Network error. Please try again.');
			}
		}
	}

	// Mark settings as changed when any value is modified
	function markSettingsChanged() {
		settingsChanged = true;
	}

	// Toggle boolean setting - uses type-safe approach
	function toggleSetting(key: keyof GeneralSettings) {
		const value = generalSettings[key];
		if (typeof value === 'boolean') {
			// Type-safe toggle using object spread
			generalSettings = { ...generalSettings, [key]: !value };
			markSettingsChanged();
		}
	}

	// Open connect modal
	function openConnectModal(service: Service) {
		selectedService = service;
		credentialValues = {};
		testResult = null;
		selectedEnvironment = 'production';
		showConnectModal = true;
	}

	// Open disconnect confirm
	function openDisconnectConfirm(service: Service) {
		disconnectingService = service;
		showDisconnectConfirm = true;
	}

	// Helper functions
	function getCategoryName(category: string): string {
		const names: Record<string, string> = {
			analytics: 'Analytics & Tracking',
			payments: 'Payments & Billing',
			email: 'Email Marketing',
			crm: 'CRM & Sales',
			social: 'Social Media',
			ai: 'AI & Machine Learning',
			monitoring: 'Monitoring & Errors',
			storage: 'Storage & CDN',
			communication: 'Communication'
		};
		return names[category] || category.charAt(0).toUpperCase() + category.slice(1);
	}

	function getCategoryIcon(category: string): string {
		const icons: Record<string, string> = {
			analytics: '📊',
			payments: '💳',
			email: '✉️',
			crm: '👥',
			social: '🔗',
			ai: '🤖',
			monitoring: '🔍',
			storage: '☁️',
			communication: '💬'
		};
		return icons[category] || '📦';
	}

	function getStatusConfig(status: string): { bg: string; text: string; label: string; dot: string } {
		switch (status) {
			case 'connected':
				return { bg: 'bg-emerald-500/15', text: 'text-emerald-400', label: 'Connected', dot: 'bg-emerald-400' };
			case 'error':
				return { bg: 'bg-red-500/15', text: 'text-red-400', label: 'Error', dot: 'bg-red-400' };
			case 'expired':
				return { bg: 'bg-amber-500/15', text: 'text-amber-400', label: 'Expired', dot: 'bg-amber-400' };
			case 'pending':
				return { bg: 'bg-blue-500/15', text: 'text-blue-400', label: 'Pending', dot: 'bg-blue-400' };
			case 'connecting':
				return { bg: 'bg-amber-500/15', text: 'text-amber-400', label: 'Connecting...', dot: 'bg-purple-400' };
			default:
				return { bg: 'bg-slate-500/15', text: 'text-slate-400', label: 'Not Connected', dot: 'bg-slate-400' };
		}
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getHealthColor(score: number): string {
		if (score >= 90) return 'text-emerald-400';
		if (score >= 70) return 'text-amber-400';
		if (score >= 50) return 'text-orange-400';
		return 'text-red-400';
	}

	onMount(() => {
		// Fetch both integrations and general settings
		fetchServices();
		fetchGeneralSettings();
		// Auto-refresh integrations every 30 seconds
		refreshInterval = setInterval(fetchServices, 30000);
	});

	onDestroy(() => {
		if (refreshInterval) clearInterval(refreshInterval);
	});
</script>

<svelte:head>
	<title>API Settings & Integrations | Admin</title>
</svelte:head>

<!-- Apple-grade Settings Dashboard -->
<div class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 text-white">
	<!-- Ambient Background Effects -->
	<div class="fixed inset-0 overflow-hidden pointer-events-none">
		<div class="absolute -top-40 -right-40 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl"></div>
		<div class="absolute -bottom-40 -left-40 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl"></div>
		<div class="absolute top-1/3 left-1/2 w-64 h-64 bg-emerald-500/5 rounded-full blur-3xl"></div>
	</div>

	<div class="relative z-10 p-6 lg:p-8 max-w-[1800px] mx-auto">
		<!-- Apple ICT7 Grade Header -->
		<header class="mb-8" in:fly={{ y: -20, duration: 600, easing: quintOut }}>
			<div class="flex flex-col gap-6">
				<div class="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4">
					<div>
						<!-- Grade Badge -->
						<div class="inline-flex items-center gap-2 px-3 py-1.5 mb-3 rounded-full text-xs font-semibold bg-gradient-to-r from-purple-500/20 to-blue-500/20 border border-purple-500/30 text-purple-300" in:scale={{ duration: 400, delay: 200 }}>
							<svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
								<circle cx="12" cy="12" r="3" />
							</svg>
							<span>Apple ICT 7 Grade</span>
						</div>
						<h1 class="text-3xl lg:text-4xl font-bold bg-gradient-to-r from-white via-slate-200 to-slate-400 bg-clip-text text-transparent">
							Settings & Integrations
						</h1>
						<p class="mt-2 text-slate-400 text-lg">
							Manage your API connections and configure platform settings
						</p>
					</div>
				</div>

				<!-- Tab Switcher - Always full width below header -->
				<div class="flex overflow-x-auto bg-white/5 backdrop-blur-xl rounded-2xl p-1 border border-white/10 gap-1 scrollbar-hide">
					<button
						onclick={() => activeTab = 'integrations'}
						class="px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 whitespace-nowrap {activeTab === 'integrations'
							? 'bg-white text-slate-900 shadow-lg'
							: 'text-slate-400 hover:text-white'}"
					>
						Integrations
					</button>
					<button
						onclick={() => activeTab = 'general'}
						class="px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 whitespace-nowrap {activeTab === 'general'
							? 'bg-white text-slate-900 shadow-lg'
							: 'text-slate-400 hover:text-white'}"
					>
						General
					</button>
					<button
						onclick={() => activeTab = 'notifications'}
						class="px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 whitespace-nowrap {activeTab === 'notifications'
							? 'bg-white text-slate-900 shadow-lg'
							: 'text-slate-400 hover:text-white'}"
					>
						Notifications
					</button>
					<button
						onclick={() => activeTab = 'email'}
						class="px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 whitespace-nowrap {activeTab === 'email'
							? 'bg-white text-slate-900 shadow-lg'
							: 'text-slate-400 hover:text-white'}"
					>
						Email
					</button>
					<button
						onclick={() => activeTab = 'backup'}
						class="px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 whitespace-nowrap {activeTab === 'backup'
							? 'bg-white text-slate-900 shadow-lg'
							: 'text-slate-400 hover:text-white'}"
					>
						Backup
					</button>
					<button
						onclick={() => activeTab = 'performance'}
						class="px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 whitespace-nowrap {activeTab === 'performance'
							? 'bg-white text-slate-900 shadow-lg'
							: 'text-slate-400 hover:text-white'}"
					>
						Performance
					</button>
				</div>
			</div>
		</header>

		{#if activeTab === 'integrations'}
			<!-- Connection Status Overview -->
			{#if !isLoading}
				<div class="grid grid-cols-2 lg:grid-cols-5 gap-4 mb-8" in:fly={{ y: 20, duration: 600, delay: 100, easing: quintOut }}>
					<!-- Total Available -->
					<div class="group relative">
						<div class="absolute inset-0 bg-gradient-to-br from-slate-500/20 to-slate-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-slate-400/30 transition-all">
							<div class="text-3xl font-bold text-white">{summary.total_available}</div>
							<div class="text-sm text-slate-400 mt-1">Available</div>
						</div>
					</div>

					<!-- Connected -->
					<div class="group relative">
						<div class="absolute inset-0 bg-gradient-to-br from-emerald-500/20 to-emerald-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-emerald-400/30 transition-all">
							<div class="text-3xl font-bold text-emerald-400">{summary.total_connected}</div>
							<div class="text-sm text-slate-400 mt-1">Connected</div>
						</div>
					</div>

					<!-- Disconnected -->
					<div class="group relative">
						<div class="absolute inset-0 bg-gradient-to-br from-slate-500/20 to-slate-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-slate-400/30 transition-all">
							<div class="text-3xl font-bold text-slate-400">{summary.total_disconnected}</div>
							<div class="text-sm text-slate-400 mt-1">Not Connected</div>
						</div>
					</div>

					<!-- Errors -->
					<div class="group relative">
						<div class="absolute inset-0 bg-gradient-to-br from-red-500/20 to-red-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-red-400/30 transition-all">
							<div class="text-3xl font-bold text-red-400">{summary.total_errors}</div>
							<div class="text-sm text-slate-400 mt-1">Errors</div>
						</div>
					</div>

					<!-- Needs Attention -->
					<div class="group relative col-span-2 lg:col-span-1">
						<div class="absolute inset-0 bg-gradient-to-br from-amber-500/20 to-amber-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-amber-400/30 transition-all">
							<div class="text-3xl font-bold text-amber-400">{summary.needs_attention}</div>
							<div class="text-sm text-slate-400 mt-1">Needs Attention</div>
						</div>
					</div>
				</div>
			{/if}

			<!-- Google Services Quick Access -->
			<div class="mb-8" in:fly={{ y: 20, duration: 600, delay: 150, easing: quintOut }}>
				<h2 class="text-xl font-semibold text-white mb-4 flex items-center gap-2">
					<span class="text-2xl">🔍</span>
					Google Services
				</h2>
				<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
					{#each googleServices as service, i}
						<div
							class="group relative"
							in:fly={{ y: 20, duration: 400, delay: 200 + i * 50, easing: quintOut }}
						>
							<div class="absolute inset-0 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-all duration-500"
								style="background: linear-gradient(135deg, {service.color}30, {service.color}10);"></div>

							<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-white/20 transition-all hover:-translate-y-1">
								<div class="flex items-start justify-between mb-3">
									<div class="flex items-center gap-3">
										<div class="w-10 h-10 rounded-xl flex items-center justify-center text-lg font-bold"
											style="background: {service.color}20; color: {service.color};">
											G
										</div>
										<div>
											<h3 class="font-semibold text-white text-sm">{service.name}</h3>
											<p class="text-xs text-slate-500">{service.category}</p>
										</div>
									</div>
									
									<span class="flex items-center gap-1.5 px-2 py-1 rounded-full text-xs font-medium {getStatusConfig(service.status).bg} {getStatusConfig(service.status).text}">
										<span class="w-1.5 h-1.5 rounded-full {getStatusConfig(service.status).dot}"></span>
										{getStatusConfig(service.status).label}
									</span>
								</div>

								<p class="text-xs text-slate-400 mb-4 line-clamp-2">{service.description}</p>

								{#if service.is_connected}
									<button
										onclick={() => openDisconnectConfirm(service)}
										class="w-full px-4 py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 rounded-xl text-sm font-medium transition-all"
									>
										Disconnect
									</button>
								{:else}
									<button
										onclick={() => openConnectModal(service)}
										class="w-full px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 text-white rounded-xl text-sm font-medium transition-all shadow-lg shadow-amber-500/20"
									>
										Connect
									</button>
								{/if}
							</div>
						</div>
					{/each}
				</div>
			</div>

			<!-- Search and Category Filter -->
			<div class="flex flex-col lg:flex-row gap-4 mb-6" in:fly={{ y: 20, duration: 600, delay: 200, easing: quintOut }}>
				<!-- Search -->
				<div class="relative flex-1 max-w-md">
					<input
						type="text"
						placeholder="Search integrations..."
						bind:value={searchQuery}
						class="w-full px-5 py-3 pl-12 bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-amber-500/50 focus:border-amber-500/50 transition-all"
					/>
					<svg class="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
					</svg>
				</div>

				<!-- Category Filter -->
				<div class="flex gap-2 overflow-x-auto pb-2">
					<button
						onclick={() => selectedCategory = null}
						class="flex items-center gap-2 px-4 py-2.5 rounded-xl font-medium transition-all whitespace-nowrap {selectedCategory === null
							? 'bg-white text-slate-900 shadow-lg'
							: 'bg-white/5 text-slate-400 hover:bg-white/10 hover:text-white border border-white/10'}"
					>
						All
					</button>
					{#each categoryList as [key, category]}
						<button
							onclick={() => selectedCategory = key}
							class="flex items-center gap-2 px-4 py-2.5 rounded-xl font-medium transition-all whitespace-nowrap {selectedCategory === key
								? 'bg-white text-slate-900 shadow-lg'
								: 'bg-white/5 text-slate-400 hover:bg-white/10 hover:text-white border border-white/10'}"
						>
							<span>{category.icon}</span>
							{category.name}
						</button>
					{/each}
				</div>
			</div>

			<!-- Loading State -->
			{#if isLoading}
				<div class="flex items-center justify-center h-64">
					<div class="relative">
						<div class="w-16 h-16 border-4 border-amber-500/20 rounded-full"></div>
						<div class="absolute top-0 left-0 w-16 h-16 border-4 border-amber-500 rounded-full animate-spin border-t-transparent"></div>
					</div>
				</div>
			{:else}
				<!-- Services Grid -->
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5">
					{#each filteredServices as service, i}
						<div
							class="group relative"
							in:fly={{ y: 30, duration: 400, delay: 250 + i * 30, easing: quintOut }}
						>
							<!-- Glow Effect -->
							<div
								class="absolute inset-0 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-all duration-500"
								style="background: linear-gradient(135deg, {service.color}25, {service.color}10);"
							></div>

							<!-- Card -->
							<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-white/20 transition-all hover:-translate-y-1">
								<!-- Header -->
								<div class="flex items-start justify-between mb-3">
									<div class="flex items-center gap-3">
										<div
											class="w-11 h-11 rounded-xl flex items-center justify-center text-xl font-bold shadow-lg"
											style="background: {service.color}20; color: {service.color};"
										>
											{service.name.charAt(0)}
										</div>
										<div>
											<h3 class="font-semibold text-white">{service.name}</h3>
											<p class="text-xs text-slate-500 capitalize">{service.category}</p>
										</div>
									</div>
								</div>

								<!-- Status Badge -->
								
								<div class="flex items-center gap-2 mb-3">
									<span class="flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium {getStatusConfig(service.status).bg} {getStatusConfig(service.status).text}">
										<span class="w-1.5 h-1.5 rounded-full {getStatusConfig(service.status).dot} {service.status === 'connected' ? 'animate-pulse' : ''}"></span>
										{getStatusConfig(service.status).label}
									</span>
									{#if service.connection?.health_score}
										<span class="text-xs {getHealthColor(service.connection.health_score)}">
											{service.connection.health_score}% health
										</span>
									{/if}
								</div>

								<!-- Description -->
								<p class="text-sm text-slate-400 mb-4 line-clamp-2">{service.description}</p>

								<!-- Connection Info -->
								{#if service.is_connected && service.connection}
									<div class="space-y-2 mb-4 p-3 bg-black/20 rounded-xl text-xs">
										<div class="flex justify-between">
											<span class="text-slate-500">API Calls Today</span>
											<span class="text-white font-medium">{service.connection.api_calls_today?.toLocaleString() || 0}</span>
										</div>
										<div class="flex justify-between">
											<span class="text-slate-500">Last Verified</span>
											<span class="text-white">{formatDate(service.connection.last_verified_at)}</span>
										</div>
									</div>
								{/if}

								<!-- Actions -->
								<div class="flex gap-2">
									{#if service.is_connected}
										<button
											onclick={() => openDisconnectConfirm(service)}
											class="flex-1 px-4 py-2.5 bg-red-500/10 hover:bg-red-500/20 text-red-400 rounded-xl text-sm font-medium transition-all"
										>
											Disconnect
										</button>
										<button
											onclick={() => openConnectModal(service)}
											class="px-4 py-2.5 bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white rounded-xl text-sm font-medium transition-all"
										>
											Configure
										</button>
									{:else}
										<button
											onclick={() => openConnectModal(service)}
											class="flex-1 px-4 py-2.5 bg-gradient-to-r from-amber-500 to-yellow-600 hover:from-amber-400 hover:to-yellow-500 text-slate-900 text-white rounded-xl text-sm font-medium transition-all shadow-lg shadow-amber-500/20 hover:shadow-purple-500/40"
										>
											Connect
										</button>
									{/if}

									{#if service.docs_url}
										<a
											href={service.docs_url}
											target="_blank"
											rel="noopener noreferrer"
											class="px-3 py-2.5 bg-white/5 hover:bg-white/10 text-slate-400 hover:text-white rounded-xl transition-all"
											title="View documentation"
										>
											<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
												<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
											</svg>
										</a>
									{/if}
								</div>
							</div>
						</div>
					{/each}
				</div>

				{#if filteredServices.length === 0}
					<div class="text-center py-16">
						<div class="text-6xl mb-4">🔌</div>
						<h3 class="text-xl font-semibold text-white mb-2">No integrations found</h3>
						<p class="text-slate-400">Try adjusting your search or filter criteria</p>
					</div>
				{/if}
			{/if}

		{:else if activeTab === 'general'}
			<div class="max-w-4xl space-y-6" in:fly={{ y: 20, duration: 600, easing: quintOut }}>
				<!-- Unsaved Changes Banner -->
				{#if settingsChanged}
					<div class="bg-amber-500/10 border border-amber-500/20 rounded-2xl p-4 flex items-center justify-between" transition:slide>
						<div class="flex items-center gap-3">
							<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
							</svg>
							<span class="text-amber-400 font-medium">You have unsaved changes</span>
						</div>
						<div class="flex gap-2">
							<button
								onclick={resetSettings}
								class="px-4 py-2 bg-white/5 hover:bg-white/10 text-slate-300 rounded-xl text-sm font-medium transition-all"
							>
								Discard
							</button>
							<button
								onclick={saveGeneralSettings}
								disabled={isSavingSettings}
								class="px-4 py-2 bg-amber-500 hover:bg-amber-400 text-white rounded-xl text-sm font-medium transition-all disabled:opacity-50"
							>
								{isSavingSettings ? 'Saving...' : 'Save Changes'}
							</button>
						</div>
					</div>
				{/if}

				<!-- Loading State -->
				{#if isLoadingSettings}
					<div class="flex items-center justify-center h-64">
						<div class="relative">
							<div class="w-16 h-16 border-4 border-purple-500/20 rounded-full"></div>
							<div class="absolute top-0 left-0 w-16 h-16 border-4 border-purple-500 rounded-full animate-spin border-t-transparent"></div>
						</div>
					</div>
				{:else}
					<!-- Site Information Section -->
					<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
						<div class="flex items-center gap-3 mb-6">
							<div class="w-10 h-10 bg-purple-500/20 rounded-xl flex items-center justify-center">
								<svg class="w-5 h-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
								</svg>
							</div>
							<div>
								<h2 class="text-xl font-semibold text-white">Site Information</h2>
								<p class="text-sm text-slate-400">Basic configuration for your website</p>
							</div>
						</div>

						<div class="grid gap-5">
							<!-- Site Name -->
							<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
								<div>
									<label for="site-name" class="font-medium text-white">Site Name</label>
									<p class="text-sm text-slate-400">Your website display name</p>
								</div>
								<div class="md:col-span-2">
									<input
										id="site-name"
										type="text"
										bind:value={generalSettings.site_name}
										oninput={markSettingsChanged}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all"
									/>
								</div>
							</div>

							<!-- Site URL -->
							<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
								<div>
									<label for="site-url" class="font-medium text-white">Site URL</label>
									<p class="text-sm text-slate-400">Primary website address</p>
								</div>
								<div class="md:col-span-2">
									<input
										id="site-url"
										type="url"
										bind:value={generalSettings.site_url}
										oninput={markSettingsChanged}
										placeholder="https://yoursite.com"
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all"
									/>
								</div>
							</div>

							<!-- Admin Email -->
							<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
								<div>
									<label for="admin-email" class="font-medium text-white">Admin Email</label>
									<p class="text-sm text-slate-400">System notification recipient</p>
								</div>
								<div class="md:col-span-2">
									<input
										id="admin-email"
										type="email"
										bind:value={generalSettings.admin_email}
										oninput={markSettingsChanged}
										placeholder="admin@yoursite.com"
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all"
									/>
								</div>
							</div>

							<!-- Timezone -->
							<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
								<div>
									<label for="timezone" class="font-medium text-white">Timezone</label>
									<p class="text-sm text-slate-400">Default system timezone</p>
								</div>
								<div class="md:col-span-2">
									<select
										id="timezone"
										bind:value={generalSettings.timezone}
										onchange={markSettingsChanged}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all"
									>
										{#each timezones as tz}
											<option value={tz} class="bg-slate-900">{tz}</option>
										{/each}
									</select>
								</div>
							</div>

							<!-- Date Format -->
							<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
								<div>
									<label for="date-format" class="font-medium text-white">Date Format</label>
									<p class="text-sm text-slate-400">Display format for dates</p>
								</div>
								<div class="md:col-span-2">
									<select
										id="date-format"
										bind:value={generalSettings.date_format}
										onchange={markSettingsChanged}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all"
									>
										{#each dateFormats as df}
											<option value={df.value} class="bg-slate-900">{df.label}</option>
										{/each}
									</select>
								</div>
							</div>
						</div>
					</div>

					<!-- System Controls Section -->
					<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
						<div class="flex items-center gap-3 mb-6">
							<div class="w-10 h-10 bg-amber-500/20 rounded-xl flex items-center justify-center">
								<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
								</svg>
							</div>
							<div>
								<h2 class="text-xl font-semibold text-white">System Controls</h2>
								<p class="text-sm text-slate-400">Operational mode toggles</p>
							</div>
						</div>

						<div class="grid gap-4">
							<!-- Maintenance Mode -->
							<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
								<div>
									<h3 class="font-medium text-white">Maintenance Mode</h3>
									<p class="text-sm text-slate-400">Temporarily disable public access to the site</p>
								</div>
								<button
									type="button"
									onclick={() => toggleSetting('maintenance_mode')}
									aria-label="Toggle maintenance mode"
									title="Toggle maintenance mode"
									class="relative w-14 h-8 rounded-full transition-colors {generalSettings.maintenance_mode ? 'bg-amber-500' : 'bg-slate-700'}"
								>
									<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {generalSettings.maintenance_mode ? 'left-7' : 'left-1'}"></span>
								</button>
							</div>

							<!-- Debug Mode -->
							<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
								<div>
									<h3 class="font-medium text-white">Debug Mode</h3>
									<p class="text-sm text-slate-400">Enable detailed error logging and debugging tools</p>
								</div>
								<button
									type="button"
									onclick={() => toggleSetting('debug_mode')}
									aria-label="Toggle debug mode"
									title="Toggle debug mode"
									class="relative w-14 h-8 rounded-full transition-colors {generalSettings.debug_mode ? 'bg-purple-500' : 'bg-slate-700'}"
								>
									<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {generalSettings.debug_mode ? 'left-7' : 'left-1'}"></span>
								</button>
							</div>
						</div>
					</div>

					<!-- Security & Access Section -->
					<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
						<div class="flex items-center gap-3 mb-6">
							<div class="w-10 h-10 bg-emerald-500/20 rounded-xl flex items-center justify-center">
								<svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
								</svg>
							</div>
							<div>
								<h2 class="text-xl font-semibold text-white">Security & Access</h2>
								<p class="text-sm text-slate-400">User registration and authentication settings</p>
							</div>
						</div>

						<div class="grid gap-4">
							<!-- Allow Registration -->
							<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
								<div>
									<h3 class="font-medium text-white">Allow Registration</h3>
									<p class="text-sm text-slate-400">Enable new user registrations</p>
								</div>
								<button
									type="button"
									onclick={() => toggleSetting('allow_registration')}
									aria-label="Toggle user registration"
									title="Toggle user registration"
									class="relative w-14 h-8 rounded-full transition-colors {generalSettings.allow_registration ? 'bg-emerald-500' : 'bg-slate-700'}"
								>
									<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {generalSettings.allow_registration ? 'left-7' : 'left-1'}"></span>
								</button>
							</div>

							<!-- Email Verification -->
							<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
								<div>
									<h3 class="font-medium text-white">Require Email Verification</h3>
									<p class="text-sm text-slate-400">New users must verify their email address</p>
								</div>
								<button
									type="button"
									onclick={() => toggleSetting('require_email_verification')}
									aria-label="Toggle email verification"
									title="Toggle email verification"
									class="relative w-14 h-8 rounded-full transition-colors {generalSettings.require_email_verification ? 'bg-emerald-500' : 'bg-slate-700'}"
								>
									<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {generalSettings.require_email_verification ? 'left-7' : 'left-1'}"></span>
								</button>
							</div>

							<!-- Session Lifetime -->
							<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
								<div>
									<label for="session-lifetime" class="font-medium text-white">Session Lifetime</label>
									<p class="text-sm text-slate-400">Minutes before auto-logout</p>
								</div>
								<div class="md:col-span-2">
									<input
										id="session-lifetime"
										type="number"
										min="5"
										max="1440"
										bind:value={generalSettings.session_lifetime}
										oninput={markSettingsChanged}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all"
									/>
								</div>
							</div>

							<!-- API Rate Limit -->
							<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
								<div>
									<label for="api-rate-limit" class="font-medium text-white">API Rate Limit</label>
									<p class="text-sm text-slate-400">Max requests per minute per user</p>
								</div>
								<div class="md:col-span-2">
									<input
										id="api-rate-limit"
										type="number"
										min="10"
										max="10000"
										bind:value={generalSettings.api_rate_limit}
										oninput={markSettingsChanged}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all"
									/>
								</div>
							</div>
						</div>
					</div>

					<!-- Save Button -->
					<div class="flex justify-end gap-3">
						{#if settingsChanged}
							<button
								onclick={resetSettings}
								class="px-6 py-3 bg-white/5 hover:bg-white/10 text-slate-300 rounded-xl font-medium transition-all"
							>
								Discard Changes
							</button>
						{/if}
						<button
							onclick={saveGeneralSettings}
							disabled={isSavingSettings || !settingsChanged}
							class="px-6 py-3 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-500 hover:to-blue-500 text-white rounded-xl font-medium transition-all shadow-lg shadow-purple-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
						>
							{#if isSavingSettings}
								<span class="flex items-center gap-2">
									<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
									Saving...
								</span>
							{:else}
								Save Settings
							{/if}
						</button>
					</div>
				{/if}
			</div>

		{:else if activeTab === 'notifications'}
			<!-- Notifications Settings Tab -->
			<div class="max-w-4xl space-y-6" in:fly={{ y: 20, duration: 600, easing: quintOut }}>
				<!-- Unsaved Changes Banner -->
				{#if notificationsChanged}
					<div class="bg-amber-500/10 border border-amber-500/20 rounded-2xl p-4 flex items-center justify-between" transition:slide>
						<div class="flex items-center gap-3">
							<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
							</svg>
							<span class="text-amber-400 font-medium">You have unsaved changes</span>
						</div>
						<button
							onclick={saveNotificationSettings}
							disabled={isSavingNotifications}
							class="px-4 py-2 bg-amber-500 hover:bg-amber-400 text-white rounded-xl text-sm font-medium transition-all disabled:opacity-50"
						>
							{isSavingNotifications ? 'Saving...' : 'Save Changes'}
						</button>
					</div>
				{/if}

				<!-- Notification Channels -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-blue-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Notification Channels</h2>
							<p class="text-sm text-slate-400">Choose how you want to receive alerts</p>
						</div>
					</div>

					<div class="grid gap-4">
						<!-- Email Alerts -->
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Email Alerts</h3>
								<p class="text-sm text-slate-400">Receive notifications via email</p>
							</div>
							<button
								type="button"
								aria-label="Toggle email alerts"
								onclick={() => { notificationSettings.email_alerts = !notificationSettings.email_alerts; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.email_alerts ? 'bg-blue-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.email_alerts ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<!-- Push Notifications -->
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Push Notifications</h3>
								<p class="text-sm text-slate-400">Browser push notifications</p>
							</div>
							<button
								type="button"
								aria-label="Toggle push notifications"
								onclick={() => { notificationSettings.push_notifications = !notificationSettings.push_notifications; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.push_notifications ? 'bg-blue-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.push_notifications ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<!-- SMS Notifications -->
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">SMS Notifications</h3>
								<p class="text-sm text-slate-400">Receive critical alerts via SMS</p>
							</div>
							<button
								type="button"
								aria-label="Toggle SMS notifications"
								onclick={() => { notificationSettings.sms_notifications = !notificationSettings.sms_notifications; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.sms_notifications ? 'bg-blue-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.sms_notifications ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<!-- Digest Frequency -->
						<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
							<div>
								<label for="digest-frequency" class="font-medium text-white">Digest Frequency</label>
								<p class="text-sm text-slate-400">How often to send summary emails</p>
							</div>
							<div class="md:col-span-2">
								<select
									id="digest-frequency"
									bind:value={notificationSettings.digest_frequency}
									onchange={() => notificationsChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all"
								>
									<option value="instant" class="bg-slate-900">Instant</option>
									<option value="hourly" class="bg-slate-900">Hourly</option>
									<option value="daily" class="bg-slate-900">Daily</option>
									<option value="weekly" class="bg-slate-900">Weekly</option>
								</select>
							</div>
						</div>
					</div>
				</div>

				<!-- Alert Types -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-amber-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Alert Types</h2>
							<p class="text-sm text-slate-400">Configure which events trigger notifications</p>
						</div>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">New User Registrations</h3>
								<p class="text-sm text-slate-400">Alert when new users sign up</p>
							</div>
							<button
								type="button"
								aria-label="Toggle new user registration alerts"
								onclick={() => { notificationSettings.alert_on_new_users = !notificationSettings.alert_on_new_users; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.alert_on_new_users ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.alert_on_new_users ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Payment Events</h3>
								<p class="text-sm text-slate-400">Alert on payments and subscriptions</p>
							</div>
							<button
								type="button"
								aria-label="Toggle payment event alerts"
								onclick={() => { notificationSettings.alert_on_payments = !notificationSettings.alert_on_payments; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.alert_on_payments ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.alert_on_payments ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Error Alerts</h3>
								<p class="text-sm text-slate-400">Alert on application errors</p>
							</div>
							<button
								type="button"
								aria-label="Toggle error alerts"
								onclick={() => { notificationSettings.alert_on_errors = !notificationSettings.alert_on_errors; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.alert_on_errors ? 'bg-red-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.alert_on_errors ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Security Alerts</h3>
								<p class="text-sm text-slate-400">Alert on suspicious activity</p>
							</div>
							<button
								type="button"
								aria-label="Toggle security alerts"
								onclick={() => { notificationSettings.alert_on_security = !notificationSettings.alert_on_security; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.alert_on_security ? 'bg-red-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.alert_on_security ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>
					</div>
				</div>

				<!-- Quiet Hours -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-purple-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Quiet Hours</h2>
							<p class="text-sm text-slate-400">Pause non-critical notifications during specified hours</p>
						</div>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Enable Quiet Hours</h3>
								<p class="text-sm text-slate-400">Only critical alerts during quiet hours</p>
							</div>
							<button
								type="button"
								aria-label="Toggle quiet hours"
								onclick={() => { notificationSettings.quiet_hours_enabled = !notificationSettings.quiet_hours_enabled; notificationsChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {notificationSettings.quiet_hours_enabled ? 'bg-purple-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {notificationSettings.quiet_hours_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						{#if notificationSettings.quiet_hours_enabled}
							<div class="grid md:grid-cols-2 gap-4 p-4 bg-black/20 rounded-xl" transition:slide>
								<div>
									<label for="quiet-start" class="block text-sm font-medium text-slate-300 mb-2">Start Time</label>
									<input
										id="quiet-start"
										type="time"
										bind:value={notificationSettings.quiet_hours_start}
										onchange={() => notificationsChanged = true}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50"
									/>
								</div>
								<div>
									<label for="quiet-end" class="block text-sm font-medium text-slate-300 mb-2">End Time</label>
									<input
										id="quiet-end"
										type="time"
										bind:value={notificationSettings.quiet_hours_end}
										onchange={() => notificationsChanged = true}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50"
									/>
								</div>
							</div>
						{/if}
					</div>
				</div>

				<!-- Save Button -->
				<div class="flex justify-end">
					<button
						onclick={saveNotificationSettings}
						disabled={isSavingNotifications || !notificationsChanged}
						class="px-6 py-3 bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500 text-white rounded-xl font-medium transition-all shadow-lg shadow-blue-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{#if isSavingNotifications}
							<span class="flex items-center gap-2">
								<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
								Saving...
							</span>
						{:else}
							Save Notification Settings
						{/if}
					</button>
				</div>
			</div>

		{:else if activeTab === 'email'}
			<!-- Email Settings Tab -->
			<div class="max-w-4xl space-y-6" in:fly={{ y: 20, duration: 600, easing: quintOut }}>
				<!-- Unsaved Changes Banner -->
				{#if emailChanged}
					<div class="bg-amber-500/10 border border-amber-500/20 rounded-2xl p-4 flex items-center justify-between" transition:slide>
						<div class="flex items-center gap-3">
							<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
							</svg>
							<span class="text-amber-400 font-medium">You have unsaved changes</span>
						</div>
						<button
							onclick={saveEmailSettings}
							disabled={isSavingEmail}
							class="px-4 py-2 bg-amber-500 hover:bg-amber-400 text-white rounded-xl text-sm font-medium transition-all disabled:opacity-50"
						>
							{isSavingEmail ? 'Saving...' : 'Save Changes'}
						</button>
					</div>
				{/if}

				<!-- SMTP Configuration -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-cyan-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-cyan-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">SMTP Configuration</h2>
							<p class="text-sm text-slate-400">Configure your outgoing email server</p>
						</div>
					</div>

					<div class="grid gap-5">
						<div class="grid md:grid-cols-2 gap-4">
							<div class="p-4 bg-black/20 rounded-xl">
								<label for="smtp-host" class="block text-sm font-medium text-slate-300 mb-2">SMTP Host</label>
								<input
									id="smtp-host"
									type="text"
									placeholder="smtp.example.com"
									bind:value={emailSettings.smtp_host}
									oninput={() => emailChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/50"
								/>
							</div>
							<div class="p-4 bg-black/20 rounded-xl">
								<label for="smtp-port" class="block text-sm font-medium text-slate-300 mb-2">SMTP Port</label>
								<input
									id="smtp-port"
									type="number"
									placeholder="587"
									bind:value={emailSettings.smtp_port}
									oninput={() => emailChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/50"
								/>
							</div>
						</div>

						<div class="grid md:grid-cols-2 gap-4">
							<div class="p-4 bg-black/20 rounded-xl">
								<label for="smtp-username" class="block text-sm font-medium text-slate-300 mb-2">Username</label>
								<input
									id="smtp-username"
									type="text"
									placeholder="your@email.com"
									bind:value={emailSettings.smtp_username}
									oninput={() => emailChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/50"
								/>
							</div>
							<div class="p-4 bg-black/20 rounded-xl">
								<label for="smtp-password" class="block text-sm font-medium text-slate-300 mb-2">Password</label>
								<input
									id="smtp-password"
									type="password"
									placeholder="Enter password"
									bind:value={emailSettings.smtp_password}
									oninput={() => emailChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-cyan-500/50"
								/>
							</div>
						</div>

						<div class="p-4 bg-black/20 rounded-xl">
							<label for="smtp-encryption" class="block text-sm font-medium text-slate-300 mb-2">Encryption</label>
							<select
								id="smtp-encryption"
								bind:value={emailSettings.smtp_encryption}
								onchange={() => emailChanged = true}
								class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-cyan-500/50"
							>
								<option value="none" class="bg-slate-900">None</option>
								<option value="tls" class="bg-slate-900">TLS</option>
								<option value="ssl" class="bg-slate-900">SSL</option>
							</select>
						</div>
					</div>
				</div>

				<!-- Sender Information -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-emerald-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Sender Information</h2>
							<p class="text-sm text-slate-400">Configure how your emails appear to recipients</p>
						</div>
					</div>

					<div class="grid gap-5">
						<div class="grid md:grid-cols-2 gap-4">
							<div class="p-4 bg-black/20 rounded-xl">
								<label for="from-name" class="block text-sm font-medium text-slate-300 mb-2">From Name</label>
								<input
									id="from-name"
									type="text"
									placeholder="Your Company"
									bind:value={emailSettings.from_name}
									oninput={() => emailChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
								/>
							</div>
							<div class="p-4 bg-black/20 rounded-xl">
								<label for="from-email" class="block text-sm font-medium text-slate-300 mb-2">From Email</label>
								<input
									id="from-email"
									type="email"
									placeholder="noreply@yoursite.com"
									bind:value={emailSettings.from_email}
									oninput={() => emailChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
								/>
							</div>
						</div>

						<div class="p-4 bg-black/20 rounded-xl">
							<label for="reply-to" class="block text-sm font-medium text-slate-300 mb-2">Reply-To Address</label>
							<input
								id="reply-to"
								type="email"
								placeholder="support@yoursite.com"
								bind:value={emailSettings.reply_to}
								oninput={() => emailChanged = true}
								class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
							/>
						</div>
					</div>
				</div>

				<!-- Email Options -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-purple-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
								<circle cx="12" cy="12" r="3" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Email Options</h2>
							<p class="text-sm text-slate-400">Additional email configuration options</p>
						</div>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Email Queue</h3>
								<p class="text-sm text-slate-400">Queue emails for background processing</p>
							</div>
							<button
								type="button"
								aria-label="Toggle email queue"
								onclick={() => { emailSettings.email_queue_enabled = !emailSettings.email_queue_enabled; emailChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {emailSettings.email_queue_enabled ? 'bg-purple-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {emailSettings.email_queue_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Email Tracking</h3>
								<p class="text-sm text-slate-400">Track email opens and clicks</p>
							</div>
							<button
								type="button"
								aria-label="Toggle email tracking"
								onclick={() => { emailSettings.email_tracking_enabled = !emailSettings.email_tracking_enabled; emailChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {emailSettings.email_tracking_enabled ? 'bg-purple-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {emailSettings.email_tracking_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>
					</div>
				</div>

				<!-- Save & Test Buttons -->
				<div class="flex justify-end gap-3">
					<button
						onclick={testEmailSettings}
						disabled={isTestingEmail || !emailSettings.smtp_host}
						class="px-6 py-3 bg-white/5 hover:bg-white/10 text-white rounded-xl font-medium transition-all disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{#if isTestingEmail}
							<span class="flex items-center gap-2">
								<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
								Sending Test...
							</span>
						{:else}
							Send Test Email
						{/if}
					</button>
					<button
						onclick={saveEmailSettings}
						disabled={isSavingEmail || !emailChanged}
						class="px-6 py-3 bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white rounded-xl font-medium transition-all shadow-lg shadow-cyan-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{#if isSavingEmail}
							<span class="flex items-center gap-2">
								<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
								Saving...
							</span>
						{:else}
							Save Email Settings
						{/if}
					</button>
				</div>
			</div>

		{:else if activeTab === 'backup'}
			<!-- Backup Settings Tab -->
			<div class="max-w-4xl space-y-6" in:fly={{ y: 20, duration: 600, easing: quintOut }}>
				<!-- Unsaved Changes Banner -->
				{#if backupChanged}
					<div class="bg-amber-500/10 border border-amber-500/20 rounded-2xl p-4 flex items-center justify-between" transition:slide>
						<div class="flex items-center gap-3">
							<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
							</svg>
							<span class="text-amber-400 font-medium">You have unsaved changes</span>
						</div>
						<button
							onclick={saveBackupSettings}
							disabled={isSavingBackup}
							class="px-4 py-2 bg-amber-500 hover:bg-amber-400 text-white rounded-xl text-sm font-medium transition-all disabled:opacity-50"
						>
							{isSavingBackup ? 'Saving...' : 'Save Changes'}
						</button>
					</div>
				{/if}

				<!-- Backup Status Card -->
				<div class="bg-gradient-to-r from-emerald-500/10 to-cyan-500/10 border border-emerald-500/20 rounded-2xl p-6">
					<div class="flex items-center justify-between">
						<div class="flex items-center gap-4">
							<div class="w-14 h-14 bg-emerald-500/20 rounded-2xl flex items-center justify-center">
								<svg class="w-7 h-7 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4" />
								</svg>
							</div>
							<div>
								<h3 class="text-lg font-semibold text-white">Last Backup</h3>
								<p class="text-emerald-400">{backupSettings.last_backup_at ? formatDate(backupSettings.last_backup_at) : 'Never'}</p>
							</div>
						</div>
						<button
							onclick={createManualBackup}
							disabled={isCreatingBackup}
							class="px-6 py-3 bg-emerald-600 hover:bg-emerald-500 text-white rounded-xl font-medium transition-all shadow-lg shadow-emerald-500/20 disabled:opacity-50"
						>
							{#if isCreatingBackup}
								<span class="flex items-center gap-2">
									<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
									Creating...
								</span>
							{:else}
								Create Backup Now
							{/if}
						</button>
					</div>
				</div>

				<!-- Automated Backups -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-blue-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Automated Backups</h2>
							<p class="text-sm text-slate-400">Configure automatic backup schedule</p>
						</div>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Enable Auto Backup</h3>
								<p class="text-sm text-slate-400">Automatically backup your data</p>
							</div>
							<button
								type="button"
								aria-label="Toggle auto backup"
								onclick={() => { backupSettings.auto_backup_enabled = !backupSettings.auto_backup_enabled; backupChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {backupSettings.auto_backup_enabled ? 'bg-blue-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {backupSettings.auto_backup_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						{#if backupSettings.auto_backup_enabled}
							<div class="grid md:grid-cols-2 gap-4" transition:slide>
								<div class="p-4 bg-black/20 rounded-xl">
									<label for="backup-frequency" class="block text-sm font-medium text-slate-300 mb-2">Backup Frequency</label>
									<select
										id="backup-frequency"
										bind:value={backupSettings.backup_frequency}
										onchange={() => backupChanged = true}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-blue-500/50"
									>
										<option value="hourly" class="bg-slate-900">Hourly</option>
										<option value="daily" class="bg-slate-900">Daily</option>
										<option value="weekly" class="bg-slate-900">Weekly</option>
										<option value="monthly" class="bg-slate-900">Monthly</option>
									</select>
								</div>

								<div class="p-4 bg-black/20 rounded-xl">
									<label for="retention-days" class="block text-sm font-medium text-slate-300 mb-2">Retention (Days)</label>
									<input
										id="retention-days"
										type="number"
										min="1"
										max="365"
										bind:value={backupSettings.backup_retention_days}
										oninput={() => backupChanged = true}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-blue-500/50"
									/>
								</div>
							</div>

							<div class="p-4 bg-black/20 rounded-xl">
								<label for="backup-storage" class="block text-sm font-medium text-slate-300 mb-2">Storage Location</label>
								<select
									id="backup-storage"
									bind:value={backupSettings.backup_storage}
									onchange={() => backupChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-blue-500/50"
								>
									<option value="local" class="bg-slate-900">Local Storage</option>
									<option value="s3" class="bg-slate-900">Amazon S3</option>
									<option value="gcs" class="bg-slate-900">Google Cloud Storage</option>
									<option value="azure" class="bg-slate-900">Azure Blob Storage</option>
								</select>
							</div>
						{/if}
					</div>
				</div>

				<!-- Backup Content -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-purple-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Backup Content</h2>
							<p class="text-sm text-slate-400">Select what to include in backups</p>
						</div>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Database</h3>
								<p class="text-sm text-slate-400">Full database backup</p>
							</div>
							<button
								type="button"
								aria-label="Toggle database backup"
								onclick={() => { backupSettings.include_database = !backupSettings.include_database; backupChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {backupSettings.include_database ? 'bg-purple-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {backupSettings.include_database ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Uploaded Files</h3>
								<p class="text-sm text-slate-400">Media and user uploads</p>
							</div>
							<button
								type="button"
								aria-label="Toggle uploaded files backup"
								onclick={() => { backupSettings.include_uploads = !backupSettings.include_uploads; backupChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {backupSettings.include_uploads ? 'bg-purple-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {backupSettings.include_uploads ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Log Files</h3>
								<p class="text-sm text-slate-400">Application and error logs</p>
							</div>
							<button
								type="button"
								aria-label="Toggle log files backup"
								onclick={() => { backupSettings.include_logs = !backupSettings.include_logs; backupChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {backupSettings.include_logs ? 'bg-purple-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {backupSettings.include_logs ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Compression</h3>
								<p class="text-sm text-slate-400">Compress backups to save space</p>
							</div>
							<button
								type="button"
								aria-label="Toggle backup compression"
								onclick={() => { backupSettings.compression_enabled = !backupSettings.compression_enabled; backupChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {backupSettings.compression_enabled ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {backupSettings.compression_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Encryption</h3>
								<p class="text-sm text-slate-400">Encrypt backup files</p>
							</div>
							<button
								type="button"
								aria-label="Toggle backup encryption"
								onclick={() => { backupSettings.encryption_enabled = !backupSettings.encryption_enabled; backupChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {backupSettings.encryption_enabled ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {backupSettings.encryption_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>
					</div>
				</div>

				<!-- Save Button -->
				<div class="flex justify-end">
					<button
						onclick={saveBackupSettings}
						disabled={isSavingBackup || !backupChanged}
						class="px-6 py-3 bg-gradient-to-r from-emerald-600 to-cyan-600 hover:from-emerald-500 hover:to-cyan-500 text-white rounded-xl font-medium transition-all shadow-lg shadow-emerald-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{#if isSavingBackup}
							<span class="flex items-center gap-2">
								<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
								Saving...
							</span>
						{:else}
							Save Backup Settings
						{/if}
					</button>
				</div>
			</div>

		{:else if activeTab === 'performance'}
			<!-- Performance Settings Tab -->
			<div class="max-w-4xl space-y-6" in:fly={{ y: 20, duration: 600, easing: quintOut }}>
				<!-- Unsaved Changes Banner -->
				{#if performanceChanged}
					<div class="bg-amber-500/10 border border-amber-500/20 rounded-2xl p-4 flex items-center justify-between" transition:slide>
						<div class="flex items-center gap-3">
							<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
							</svg>
							<span class="text-amber-400 font-medium">You have unsaved changes</span>
						</div>
						<button
							onclick={savePerformanceSettings}
							disabled={isSavingPerformance}
							class="px-4 py-2 bg-amber-500 hover:bg-amber-400 text-white rounded-xl text-sm font-medium transition-all disabled:opacity-50"
						>
							{isSavingPerformance ? 'Saving...' : 'Save Changes'}
						</button>
					</div>
				{/if}

				<!-- Cache Configuration -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center justify-between mb-6">
						<div class="flex items-center gap-3">
							<div class="w-10 h-10 bg-orange-500/20 rounded-xl flex items-center justify-center">
								<svg class="w-5 h-5 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
								</svg>
							</div>
							<div>
								<h2 class="text-xl font-semibold text-white">Cache Configuration</h2>
								<p class="text-sm text-slate-400">Optimize performance with caching</p>
							</div>
						</div>
						<button
							onclick={clearCache}
							class="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 text-red-400 rounded-xl text-sm font-medium transition-all"
						>
							Clear Cache
						</button>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Enable Cache</h3>
								<p class="text-sm text-slate-400">Cache responses for faster loading</p>
							</div>
							<button
								type="button"
								aria-label="Toggle cache"
								onclick={() => { performanceSettings.cache_enabled = !performanceSettings.cache_enabled; performanceChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {performanceSettings.cache_enabled ? 'bg-orange-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {performanceSettings.cache_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						{#if performanceSettings.cache_enabled}
							<div class="grid md:grid-cols-2 gap-4" transition:slide>
								<div class="p-4 bg-black/20 rounded-xl">
									<label for="cache-driver" class="block text-sm font-medium text-slate-300 mb-2">Cache Driver</label>
									<select
										id="cache-driver"
										bind:value={performanceSettings.cache_driver}
										onchange={() => performanceChanged = true}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-orange-500/50"
									>
										<option value="file" class="bg-slate-900">File System</option>
										<option value="redis" class="bg-slate-900">Redis</option>
										<option value="memcached" class="bg-slate-900">Memcached</option>
									</select>
								</div>

								<div class="p-4 bg-black/20 rounded-xl">
									<label for="cache-ttl" class="block text-sm font-medium text-slate-300 mb-2">TTL (Seconds)</label>
									<input
										id="cache-ttl"
										type="number"
										min="60"
										max="86400"
										bind:value={performanceSettings.cache_ttl}
										oninput={() => performanceChanged = true}
										class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-orange-500/50"
									/>
								</div>
							</div>
						{/if}
					</div>
				</div>

				<!-- CDN Settings -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-blue-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Content Delivery Network</h2>
							<p class="text-sm text-slate-400">Serve static assets from edge locations</p>
						</div>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Enable CDN</h3>
								<p class="text-sm text-slate-400">Serve assets via CDN</p>
							</div>
							<button
								type="button"
								aria-label="Toggle CDN"
								onclick={() => { performanceSettings.cdn_enabled = !performanceSettings.cdn_enabled; performanceChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {performanceSettings.cdn_enabled ? 'bg-blue-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {performanceSettings.cdn_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						{#if performanceSettings.cdn_enabled}
							<div class="p-4 bg-black/20 rounded-xl" transition:slide>
								<label for="cdn-url" class="block text-sm font-medium text-slate-300 mb-2">CDN URL</label>
								<input
									id="cdn-url"
									type="url"
									placeholder="https://cdn.yoursite.com"
									bind:value={performanceSettings.cdn_url}
									oninput={() => performanceChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500/50"
								/>
							</div>
						{/if}
					</div>
				</div>

				<!-- Optimization Options -->
				<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6">
					<div class="flex items-center gap-3 mb-6">
						<div class="w-10 h-10 bg-emerald-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
							</svg>
						</div>
						<div>
							<h2 class="text-xl font-semibold text-white">Optimization</h2>
							<p class="text-sm text-slate-400">Enable performance optimizations</p>
						</div>
					</div>

					<div class="grid gap-4">
						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Image Optimization</h3>
								<p class="text-sm text-slate-400">Automatically optimize images</p>
							</div>
							<button
								type="button"
								aria-label="Toggle image optimization"
								onclick={() => { performanceSettings.image_optimization = !performanceSettings.image_optimization; performanceChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {performanceSettings.image_optimization ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {performanceSettings.image_optimization ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Lazy Loading</h3>
								<p class="text-sm text-slate-400">Load images on scroll</p>
							</div>
							<button
								type="button"
								aria-label="Toggle lazy loading"
								onclick={() => { performanceSettings.lazy_loading = !performanceSettings.lazy_loading; performanceChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {performanceSettings.lazy_loading ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {performanceSettings.lazy_loading ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Minify CSS</h3>
								<p class="text-sm text-slate-400">Compress CSS files</p>
							</div>
							<button
								type="button"
								aria-label="Toggle CSS minification"
								onclick={() => { performanceSettings.minify_css = !performanceSettings.minify_css; performanceChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {performanceSettings.minify_css ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {performanceSettings.minify_css ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Minify JavaScript</h3>
								<p class="text-sm text-slate-400">Compress JS files</p>
							</div>
							<button
								type="button"
								aria-label="Toggle JavaScript minification"
								onclick={() => { performanceSettings.minify_js = !performanceSettings.minify_js; performanceChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {performanceSettings.minify_js ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {performanceSettings.minify_js ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="flex items-center justify-between p-4 bg-black/20 rounded-xl">
							<div>
								<h3 class="font-medium text-white">Gzip Compression</h3>
								<p class="text-sm text-slate-400">Compress responses</p>
							</div>
							<button
								type="button"
								aria-label="Toggle gzip compression"
								onclick={() => { performanceSettings.gzip_enabled = !performanceSettings.gzip_enabled; performanceChanged = true; }}
								class="relative w-14 h-8 rounded-full transition-colors {performanceSettings.gzip_enabled ? 'bg-emerald-500' : 'bg-slate-700'}"
							>
								<span class="absolute top-1 w-6 h-6 bg-white rounded-full transition-transform shadow-lg {performanceSettings.gzip_enabled ? 'left-7' : 'left-1'}"></span>
							</button>
						</div>

						<div class="grid md:grid-cols-3 gap-4 items-center p-4 bg-black/20 rounded-xl">
							<div>
								<label for="browser-cache" class="font-medium text-white">Browser Cache</label>
								<p class="text-sm text-slate-400">Days to cache in browser</p>
							</div>
							<div class="md:col-span-2">
								<input
									id="browser-cache"
									type="number"
									min="1"
									max="365"
									bind:value={performanceSettings.browser_cache_days}
									oninput={() => performanceChanged = true}
									class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
								/>
							</div>
						</div>
					</div>
				</div>

				<!-- Save Button -->
				<div class="flex justify-end">
					<button
						onclick={savePerformanceSettings}
						disabled={isSavingPerformance || !performanceChanged}
						class="px-6 py-3 bg-gradient-to-r from-orange-600 to-amber-600 hover:from-orange-500 hover:to-amber-500 text-white rounded-xl font-medium transition-all shadow-lg shadow-orange-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
					>
						{#if isSavingPerformance}
							<span class="flex items-center gap-2">
								<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
								Saving...
							</span>
						{:else}
							Save Performance Settings
						{/if}
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>

<!-- Connect Modal -->
{#if showConnectModal && selectedService}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 200 }}
	>
		<!-- Backdrop -->
		<button
			type="button"
			class="absolute inset-0 bg-black/80 backdrop-blur-sm cursor-default"
			onclick={() => showConnectModal = false}
			aria-label="Close modal"
		></button>

		<!-- Modal -->
		<div
			class="relative w-full max-w-lg bg-slate-900 border border-white/10 rounded-3xl shadow-2xl overflow-hidden"
			transition:scale={{ duration: 300, easing: backOut }}
		>
			<!-- Header -->
			<div class="relative p-6 border-b border-white/10">
				<div class="absolute inset-0 bg-gradient-to-r opacity-50" style="background: linear-gradient(135deg, {selectedService.color}15, transparent);"></div>
				<div class="relative flex items-center gap-4">
					<div
						class="w-14 h-14 rounded-xl flex items-center justify-center text-2xl font-bold shadow-lg"
						style="background: {selectedService.color}20; color: {selectedService.color};"
					>
						{selectedService.name.charAt(0)}
					</div>
					<div>
						<h2 class="text-xl font-bold text-white">Connect {selectedService.name}</h2>
						<p class="text-sm text-slate-400">{selectedService.description}</p>
					</div>
				</div>
				<button
					type="button"
					onclick={() => showConnectModal = false}
					aria-label="Close connection modal"
					title="Close"
					class="absolute top-6 right-6 text-slate-400 hover:text-white transition-colors"
				>
					<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Form -->
			<div class="p-6 space-y-5 max-h-[60vh] overflow-y-auto">
				<!-- Environment Selector -->
				{#if selectedService.environments && selectedService.environments.length > 1}
					<div>
						<span id="environment-label" class="block text-sm font-medium text-slate-300 mb-2">Environment</span>
						<div class="flex gap-2" role="group" aria-labelledby="environment-label">
							{#each selectedService.environments as env}
								<button
									onclick={() => selectedEnvironment = env}
									aria-pressed={selectedEnvironment === env}
									class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium transition-all {selectedEnvironment === env
										? 'bg-white text-slate-900'
										: 'bg-white/5 text-slate-400 hover:bg-white/10 hover:text-white border border-white/10'}"
								>
									{env.charAt(0).toUpperCase() + env.slice(1)}
								</button>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Credential Fields -->
				{#each selectedService.fields as field}
					<div>
						<label for="field-{field.key}" class="block text-sm font-medium text-slate-300 mb-2">
							{field.label}
							{#if field.required}<span class="text-red-400">*</span>{/if}
						</label>
						<input
							id="field-{field.key}"
							type={field.type === 'password' ? 'password' : 'text'}
							placeholder={field.placeholder || `Enter ${field.label.toLowerCase()}`}
							bind:value={credentialValues[field.key]}
							class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-amber-500/50 focus:border-amber-500/50 transition-all"
						/>
					</div>
				{/each}

				<!-- Test Result -->
				{#if testResult}
					<div
						class="p-4 rounded-xl {testResult.success
							? 'bg-emerald-500/10 border border-emerald-500/20'
							: 'bg-red-500/10 border border-red-500/20'}"
						transition:slide
					>
						<div class="flex items-center gap-2">
							{#if testResult.success}
								<svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
								<span class="text-emerald-400 font-medium">{testResult.message || 'Connection successful!'}</span>
							{:else}
								<svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
								</svg>
								<span class="text-red-400 font-medium">{testResult.error || 'Connection failed'}</span>
							{/if}
						</div>
						{#if testResult.latency}
							<p class="text-xs text-slate-400 mt-1">Response time: {testResult.latency}ms</p>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="p-6 border-t border-white/10 bg-black/20 flex gap-3">
				<button
					onclick={testConnection}
					disabled={isTesting}
					class="flex-1 px-4 py-3 bg-white/5 hover:bg-white/10 text-white rounded-xl font-medium transition-all disabled:opacity-50"
				>
					{#if isTesting}
						<span class="flex items-center justify-center gap-2">
							<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
							Testing...
						</span>
					{:else}
						Test Connection
					{/if}
				</button>
				<button
					onclick={connectService}
					disabled={isConnecting}
					class="flex-1 px-4 py-3 bg-gradient-to-r from-amber-500 to-yellow-600 hover:from-amber-400 hover:to-yellow-500 text-slate-900 text-white rounded-xl font-medium transition-all shadow-lg shadow-amber-500/20 disabled:opacity-50"
				>
					{#if isConnecting}
						<span class="flex items-center justify-center gap-2">
							<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
							Connecting...
						</span>
					{:else}
						Connect
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Disconnect Confirmation Modal -->
{#if showDisconnectConfirm && disconnectingService}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 200 }}
	>
		<button
			type="button"
			class="absolute inset-0 bg-black/80 backdrop-blur-sm cursor-default"
			onclick={() => showDisconnectConfirm = false}
			aria-label="Close"
		></button>

		<div
			class="relative w-full max-w-md bg-slate-900 border border-white/10 rounded-3xl shadow-2xl p-6"
			transition:scale={{ duration: 300, easing: backOut }}
		>
			<div class="text-center">
				<div class="w-16 h-16 mx-auto mb-4 bg-red-500/10 rounded-full flex items-center justify-center">
					<svg class="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
				</div>
				<h3 class="text-xl font-bold text-white mb-2">Disconnect {disconnectingService.name}?</h3>
				<p class="text-slate-400 mb-6">
					This will remove all stored credentials and disable the integration. Dashboard metrics using this service will show "Not Connected".
				</p>
				<div class="flex gap-3">
					<button
						onclick={() => showDisconnectConfirm = false}
						class="flex-1 px-4 py-3 bg-white/5 hover:bg-white/10 text-white rounded-xl font-medium transition-all"
					>
						Cancel
					</button>
					<button
						onclick={disconnectService}
						disabled={isDisconnecting}
						class="flex-1 px-4 py-3 bg-red-600 hover:bg-red-500 text-white rounded-xl font-medium transition-all disabled:opacity-50"
					>
						{#if isDisconnecting}
							<span class="flex items-center justify-center gap-2">
								<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
								Disconnecting...
							</span>
						{:else}
							Disconnect
						{/if}
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * ADMIN SETTINGS - Design System Override
	 * ICT 7 Apple Principal Engineer Grade - Uses admin color hierarchy
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Main container override - use design system colors */
	:global(.min-h-screen.bg-gradient-to-br) {
		background: var(--bg-base, #0D1117) !important;
	}

	/* Card backgrounds - use elevated surface */
	:global(.bg-white\/5),
	:global(.bg-black\/20),
	:global(.bg-slate-800\/50),
	:global(.bg-slate-900) {
		background: var(--bg-elevated, #161B22) !important;
		border-color: var(--border-default, #30363D) !important;
	}

	/* Text colors - use design system hierarchy */
	:global(.text-white) {
		color: var(--text-primary, #F0F6FC) !important;
	}

	:global(.text-slate-400),
	:global(.text-slate-300) {
		color: var(--text-secondary, #8B949E) !important;
	}

	:global(.text-slate-500) {
		color: var(--text-tertiary, #6E7681) !important;
	}

	/* Primary accent colors - use gold/amber brand color */
	:global(.from-amber-500),
	:global(.to-yellow-600),
	:global(.bg-amber-500),
	:global(.text-amber-400) {
		--tw-gradient-from: var(--primary-500, #E6B800) !important;
		--tw-gradient-to: var(--primary-600, #B38F00) !important;
		background-color: var(--primary-500, #E6B800) !important;
		color: var(--primary-400, #FFD11A) !important;
	}

	/* Border colors - use design system borders */
	:global(.border-white\/10),
	:global(.border-white\/20) {
		border-color: var(--border-default, #30363D) !important;
	}

	/* Tab switcher background */
	:global(.bg-white\/5.backdrop-blur-xl) {
		background: var(--bg-surface, #1C2128) !important;
		border: 1px solid var(--border-default, #30363D) !important;
	}

	/* Active tab styling */
	:global(.bg-white.text-slate-900) {
		background: var(--primary-500, #E6B800) !important;
		color: var(--bg-base, #0D1117) !important;
	}

	/* Card/Section containers */
	:global(.rounded-2xl),
	:global(.rounded-3xl) {
		border: 1px solid var(--border-default, #30363D) !important;
	}

	/* Success states */
	:global(.text-emerald-400),
	:global(.bg-emerald-500) {
		color: var(--success-emphasis, #3FB950) !important;
	}

	:global(.bg-emerald-500\/10),
	:global(.bg-emerald-500\/15) {
		background: var(--success-soft, rgba(46, 160, 67, 0.15)) !important;
	}

	/* Error states */
	:global(.text-red-400),
	:global(.bg-red-500) {
		color: var(--error-emphasis, #F85149) !important;
	}

	:global(.bg-red-500\/10),
	:global(.bg-red-500\/15) {
		background: var(--error-soft, rgba(218, 54, 51, 0.15)) !important;
	}

	/* Warning states */
	:global(.text-amber-400) {
		color: var(--warning-emphasis, #D29922) !important;
	}

	:global(.bg-amber-500\/10),
	:global(.bg-amber-500\/15) {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15)) !important;
	}

	/* Input fields */
	:global(input),
	:global(select),
	:global(textarea) {
		background: var(--bg-surface, #1C2128) !important;
		border: 1px solid var(--border-default, #30363D) !important;
		color: var(--text-primary, #F0F6FC) !important;
	}

	:global(input:focus),
	:global(select:focus),
	:global(textarea:focus) {
		border-color: var(--primary-500, #E6B800) !important;
		box-shadow: 0 0 0 2px rgba(230, 184, 0, 0.2) !important;
	}

	:global(input::placeholder) {
		color: var(--text-muted, #484F58) !important;
	}

	/* Toggle switches - connected state */
	:global(.bg-emerald-500) {
		background: var(--success-base, #2EA043) !important;
	}

	/* Toggle switches - disconnected state */
	:global(.bg-slate-700) {
		background: var(--bg-active, #2D333B) !important;
	}

	/* Primary buttons */
	:global(.bg-gradient-to-r.from-amber-500),
	:global(.from-orange-600) {
		background: linear-gradient(135deg, var(--primary-500, #E6B800), var(--primary-600, #B38F00)) !important;
		border: none !important;
		color: var(--bg-base, #0D1117) !important;
	}

	:global(.bg-gradient-to-r.from-amber-500:hover),
	:global(.from-orange-600:hover) {
		background: linear-gradient(135deg, var(--primary-400, #FFD11A), var(--primary-500, #E6B800)) !important;
	}

	/* Secondary/Ghost buttons */
	:global(.bg-white\/5:hover),
	:global(.hover\:bg-white\/10:hover) {
		background: var(--bg-hover, #252B33) !important;
	}

	/* Modal backgrounds */
	:global(.bg-slate-900.border) {
		background: var(--bg-elevated, #161B22) !important;
		border: 1px solid var(--border-default, #30363D) !important;
	}

	/* Ambient blob effects - use brand colors */
	:global(.bg-purple-500\/10) {
		background: rgba(99, 102, 241, 0.08) !important;
	}

	:global(.bg-blue-500\/10) {
		background: rgba(59, 130, 246, 0.08) !important;
	}

	:global(.bg-emerald-500\/5) {
		background: rgba(16, 185, 129, 0.05) !important;
	}

	/* Grade badge */
	:global(.from-purple-500\/20) {
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.15), rgba(59, 130, 246, 0.15)) !important;
		border-color: rgba(99, 102, 241, 0.25) !important;
	}

	/* Section headers */
	:global(h1),
	:global(h2),
	:global(h3) {
		color: var(--text-primary, #F0F6FC) !important;
	}

	/* Service cards */
	:global(.group:hover) {
		border-color: var(--primary-500, #E6B800) !important;
	}

	/* Status badges */
	:global(.bg-slate-500\/15) {
		background: rgba(100, 116, 139, 0.15) !important;
	}

	/* Scrollbar styling */
	:global(::-webkit-scrollbar) {
		width: 8px;
		height: 8px;
	}

	:global(::-webkit-scrollbar-track) {
		background: var(--bg-base, #0D1117);
	}

	:global(::-webkit-scrollbar-thumb) {
		background: var(--border-default, #30363D);
		border-radius: 4px;
	}

	:global(::-webkit-scrollbar-thumb:hover) {
		background: var(--border-emphasis, #8B949E);
	}

	/* Hide scrollbar for tab switcher on mobile */
	:global(.scrollbar-hide) {
		-ms-overflow-style: none;
		scrollbar-width: none;
	}

	:global(.scrollbar-hide::-webkit-scrollbar) {
		display: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Apple ICT7 Principal Engineer Grade
	 * Mobile-first approach with progressive enhancement
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Extra Small Mobile (< 380px) - iPhone SE, Galaxy Fold */
	@media (max-width: 380px) {
		:global(.p-6) {
			padding: 0.75rem !important;
		}

		:global(.p-8) {
			padding: 1rem !important;
		}

		:global(.gap-6) {
			gap: 0.75rem !important;
		}

		:global(.mb-8) {
			margin-bottom: 1rem !important;
		}

		:global(.text-3xl) {
			font-size: 1.5rem !important;
		}

		:global(.text-xl) {
			font-size: 1rem !important;
		}

		:global(.rounded-2xl) {
			border-radius: 0.75rem !important;
		}

		:global(.rounded-3xl) {
			border-radius: 1rem !important;
		}

		/* Tab switcher - horizontal scroll on tiny screens */
		:global(.flex.gap-1.p-1) {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
			scrollbar-width: none;
		}

		:global(.flex.gap-1.p-1::-webkit-scrollbar) {
			display: none;
		}

		/* Reduce button padding */
		:global(.px-4.py-2) {
			padding: 0.375rem 0.625rem !important;
			font-size: 0.75rem !important;
		}

		/* Grid layouts - single column */
		:global(.grid.grid-cols-2),
		:global(.grid.lg\:grid-cols-5),
		:global(.grid.md\:grid-cols-2),
		:global(.grid.xl\:grid-cols-4) {
			grid-template-columns: 1fr !important;
		}

		/* Modal sizing */
		:global(.max-w-lg),
		:global(.max-w-md) {
			max-width: calc(100vw - 1rem) !important;
			margin: 0.5rem !important;
		}
	}

	/* Small Mobile (381px - 480px) */
	@media (min-width: 381px) and (max-width: 480px) {
		:global(.p-6) {
			padding: 1rem !important;
		}

		:global(.p-8) {
			padding: 1.25rem !important;
		}

		:global(.gap-6) {
			gap: 1rem !important;
		}

		:global(.text-3xl) {
			font-size: 1.75rem !important;
		}

		/* Grid - 1 column on small mobile */
		:global(.grid.grid-cols-2),
		:global(.grid.lg\:grid-cols-5) {
			grid-template-columns: 1fr !important;
		}

		/* Settings form rows - stack vertically */
		:global(.grid.md\:grid-cols-3) {
			grid-template-columns: 1fr !important;
		}

		:global(.md\:col-span-2) {
			grid-column: span 1 !important;
		}
	}

	/* Mobile Landscape / Large Mobile (481px - 640px) */
	@media (min-width: 481px) and (max-width: 640px) {
		:global(.p-6) {
			padding: 1.25rem !important;
		}

		/* Grid - 2 columns for cards */
		:global(.grid.lg\:grid-cols-5) {
			grid-template-columns: repeat(2, 1fr) !important;
		}

		:global(.grid.xl\:grid-cols-4) {
			grid-template-columns: repeat(2, 1fr) !important;
		}

		/* Settings rows still stack */
		:global(.grid.md\:grid-cols-3) {
			grid-template-columns: 1fr !important;
		}
	}

	/* Tablet Portrait (641px - 768px) */
	@media (min-width: 641px) and (max-width: 768px) {
		/* Grid - 2-3 columns */
		:global(.grid.lg\:grid-cols-5) {
			grid-template-columns: repeat(3, 1fr) !important;
		}

		:global(.grid.xl\:grid-cols-4) {
			grid-template-columns: repeat(2, 1fr) !important;
		}

		/* Settings form - side by side */
		:global(.grid.md\:grid-cols-3) {
			grid-template-columns: 1fr 2fr !important;
		}
	}

	/* Tablet Landscape (769px - 1024px) */
	@media (min-width: 769px) and (max-width: 1024px) {
		:global(.grid.lg\:grid-cols-5) {
			grid-template-columns: repeat(4, 1fr) !important;
		}

		:global(.grid.xl\:grid-cols-4) {
			grid-template-columns: repeat(3, 1fr) !important;
		}
	}

	/* Touch Device Optimizations - Apple HIG 44pt minimum */
	@media (hover: none) and (pointer: coarse) {
		/* Toggle switches - larger touch targets */
		:global(.w-14.h-8) {
			min-width: 56px !important;
			min-height: 32px !important;
		}

		/* Buttons - 44px minimum */
		:global(button) {
			min-height: 44px;
		}

		/* Input fields - comfortable touch */
		:global(input),
		:global(select),
		:global(textarea) {
			min-height: 48px !important;
			font-size: 16px !important; /* Prevents iOS zoom */
		}

		/* Tab buttons */
		:global(.px-4.py-2\.5) {
			padding: 0.75rem 1rem !important;
		}

		/* Card tap targets */
		:global(.rounded-2xl.p-5),
		:global(.rounded-2xl.p-6) {
			padding: 1rem !important;
		}

		/* Adequate spacing between interactive elements */
		:global(.grid.gap-4) {
			gap: 0.75rem !important;
		}

		:global(.grid.gap-5) {
			gap: 1rem !important;
		}
	}

	/* Reduced Motion - Accessibility */
	@media (prefers-reduced-motion: reduce) {
		:global(*) {
			animation-duration: 0.01ms !important;
			animation-iteration-count: 1 !important;
			transition-duration: 0.01ms !important;
		}

		:global(.hover\:-translate-y-1:hover) {
			transform: none !important;
		}
	}

	/* High Contrast Mode - Accessibility */
	@media (prefers-contrast: high) {
		:global(.border-white\/10),
		:global(.border-white\/20) {
			border-width: 2px !important;
			border-color: currentColor !important;
		}

		:global(.text-slate-400),
		:global(.text-slate-500) {
			color: var(--text-secondary) !important;
			opacity: 1 !important;
		}

		:global(button),
		:global(input),
		:global(select) {
			border-width: 2px !important;
		}

		:global(.font-semibold),
		:global(.font-bold) {
			font-weight: 800 !important;
		}
	}

	/* Print Styles */
	@media print {
		:global(.fixed) {
			display: none !important;
		}

		:global(.bg-gradient-to-br) {
			background: white !important;
			color: black !important;
		}

		:global(.rounded-2xl),
		:global(.rounded-3xl) {
			box-shadow: none !important;
			border: 1px solid #ccc !important;
			break-inside: avoid;
		}

		:global(button:not([type="submit"])) {
			display: none !important;
		}
	}

	/* Landscape Mode - Short viewport */
	@media (max-height: 500px) and (orientation: landscape) {
		:global(.p-6) {
			padding: 0.75rem 1rem !important;
		}

		:global(.mb-8) {
			margin-bottom: 0.75rem !important;
		}

		:global(.gap-6) {
			gap: 0.5rem !important;
		}

		/* Reduce vertical spacing */
		:global(.space-y-6 > * + *) {
			margin-top: 0.75rem !important;
		}
	}

	/* Large Desktop (1440px+) - Enhanced spacing */
	@media (min-width: 1440px) {
		:global(.max-w-4xl) {
			max-width: 56rem !important;
		}

		:global(.max-w-\[1800px\]) {
			max-width: 2000px !important;
		}
	}
</style>

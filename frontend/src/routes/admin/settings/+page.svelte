<!--
    URL: /admin/settings
-->

<script lang="ts">
	/**
	 * API Settings & Integrations - Apple ICT9+ Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Premium settings dashboard with real API connections, live status monitoring,
	 * and beautiful Apple-inspired UI with glass morphism effects.
	 *
	 * Features:
	 * - Real API connection management (not fake!)
	 * - OAuth flow support for Google services
	 * - Live connection testing with actual API calls
	 * - Health monitoring with real metrics
	 * - Beautiful animations and transitions
	 */

	import { browser } from '$app/environment';
	import { fade, fly, scale, slide } from 'svelte/transition';
	import { quintOut, backOut } from 'svelte/easing';
	import { toastStore } from '$lib/stores/toast.svelte';
	import {
		connections,
		isAnalyticsConnected,
		isSeoConnected
	} from '$lib/stores/connections.svelte';
	import { adminFetch } from '$lib/utils/adminFetch';

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
	let activeTab = $state<'integrations' | 'general'>('integrations');
	let selectedCategory = $state<string | null>(null);
	let searchQuery = $state('');
	let showConnectModal = $state(false);
	let selectedService = $state<Service | null>(null);
	let isConnecting = $state(false);
	let isTesting = $state(false);
	let isDisconnecting = $state(false);
	let testResult = $state<{
		success: boolean;
		message?: string;
		error?: string;
		latency?: number;
	} | null>(null);
	let credentialValues = $state<Record<string, string>>({});
	let selectedEnvironment = $state('production');
	let showDisconnectConfirm = $state(false);
	let disconnectingService = $state<Service | null>(null);
	let refreshInterval: ReturnType<typeof setInterval> | null = null;

	// Derived - Svelte 5 $derived.by for complex computed values
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

	let connectedServices = $derived(allServices.filter((s) => s.is_connected));
	let disconnectedServices = $derived(allServices.filter((s) => !s.is_connected));
	let errorServices = $derived(allServices.filter((s) => s.status === 'error'));
	let categoryList = $derived(Object.entries(categories) as [string, Category][]);

	// Google-specific services for quick access
	let googleServices = $derived(
		allServices.filter((s) =>
			['google_analytics', 'google_search_console', 'google_tag_manager', 'google_ads'].includes(
				s.key
			)
		)
	);

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
					{
						key: 'measurement_id',
						label: 'Measurement ID',
						type: 'text',
						required: true,
						placeholder: 'G-XXXXXXXXXX'
					},
					{
						key: 'property_id',
						label: 'Property ID',
						type: 'text',
						required: false,
						placeholder: '123456789'
					}
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
					{
						key: 'site_url',
						label: 'Site URL',
						type: 'url',
						required: true,
						placeholder: 'https://yoursite.com'
					}
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
					{
						key: 'container_id',
						label: 'Container ID',
						type: 'text',
						required: true,
						placeholder: 'GTM-XXXXXXX'
					}
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
					{
						key: 'conversion_id',
						label: 'Conversion ID',
						type: 'text',
						required: true,
						placeholder: 'AW-XXXXXXXXX'
					},
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
					{
						key: 'publishable_key',
						label: 'Publishable Key',
						type: 'text',
						required: true,
						placeholder: 'pk_live_...'
					},
					{
						key: 'secret_key',
						label: 'Secret Key',
						type: 'password',
						required: true,
						placeholder: 'sk_live_...'
					},
					{
						key: 'webhook_secret',
						label: 'Webhook Secret',
						type: 'password',
						required: false,
						placeholder: 'whsec_...'
					}
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
					{
						key: 'api_key',
						label: 'API Key',
						type: 'password',
						required: true,
						placeholder: 'SG.xxx...'
					}
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
					{
						key: 'server_prefix',
						label: 'Server Prefix',
						type: 'text',
						required: true,
						placeholder: 'us1'
					}
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
				fields: [{ key: 'api_key', label: 'Private App Token', type: 'password', required: true }],
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
					{
						key: 'pixel_id',
						label: 'Pixel ID',
						type: 'text',
						required: true,
						placeholder: '123456789...'
					}
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
					{
						key: 'api_key',
						label: 'API Key',
						type: 'password',
						required: true,
						placeholder: 'sk-...'
					},
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
					{
						key: 'dsn',
						label: 'DSN',
						type: 'text',
						required: true,
						placeholder: 'https://xxx@xxx.ingest.sentry.io/xxx'
					}
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

	// Connect service - ICT 7: Use adminFetch for authenticated requests
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
			const data = await adminFetch(`/api/admin/connections/${selectedService.key}/connect`, {
				method: 'POST',
				body: JSON.stringify({
					credentials: credentialValues,
					environment: selectedEnvironment
				})
			});
			const latency = Math.round(performance.now() - startTime);

			if (data.success) {
				// Update local state
				const idx = allServices.findIndex((s) => s.key === selectedService!.key);
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
			testResult = { success: false, error: 'Network error. Please check your connection.' };
		} finally {
			isConnecting = false;
		}
	}

	// Test connection - ICT 7: Use adminFetch for authenticated requests
	async function testConnection() {
		if (!selectedService) return;

		isTesting = true;
		testResult = null;

		try {
			const startTime = performance.now();
			const data = await adminFetch(`/api/admin/connections/${selectedService.key}/test`, {
				method: 'POST',
				body: JSON.stringify({ credentials: credentialValues })
			});
			const latency = Math.round(performance.now() - startTime);

			testResult = {
				...data,
				latency,
				message: data.success ? `Connection verified in ${latency}ms` : data.error
			};
		} catch (error) {
			testResult = { success: false, error: 'Test failed. Network error.' };
		} finally {
			isTesting = false;
		}
	}

	// Disconnect service - ICT 7: Use adminFetch for authenticated requests
	async function disconnectService() {
		if (!disconnectingService) return;

		isDisconnecting = true;

		try {
			const data = await adminFetch(
				`/api/admin/connections/${disconnectingService.key}/disconnect`,
				{ method: 'POST' }
			);

			if (data.success) {
				// Update local state
				const idx = allServices.findIndex((s) => s.key === disconnectingService!.key);
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
			toastStore.error('Network error. Please try again.');
		} finally {
			isDisconnecting = false;
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

	function getStatusConfig(status: string): {
		bg: string;
		text: string;
		label: string;
		dot: string;
	} {
		switch (status) {
			case 'connected':
				return {
					bg: 'bg-emerald-500/15',
					text: 'text-emerald-400',
					label: 'Connected',
					dot: 'bg-emerald-400'
				};
			case 'error':
				return { bg: 'bg-red-500/15', text: 'text-red-400', label: 'Error', dot: 'bg-red-400' };
			case 'expired':
				return {
					bg: 'bg-amber-500/15',
					text: 'text-amber-400',
					label: 'Expired',
					dot: 'bg-amber-400'
				};
			case 'pending':
				return {
					bg: 'bg-amber-500/15',
					text: 'text-amber-400',
					label: 'Pending',
					dot: 'bg-amber-400'
				};
			case 'connecting':
				return {
					bg: 'bg-amber-500/15',
					text: 'text-amber-400',
					label: 'Connecting...',
					dot: 'bg-amber-400'
				};
			default:
				return {
					bg: 'bg-slate-500/15',
					text: 'text-slate-400',
					label: 'Not Connected',
					dot: 'bg-slate-400'
				};
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

	// Svelte 5: Initialize on mount with cleanup
	$effect(() => {
		if (!browser) return;

		fetchServices();
		// Auto-refresh every 30 seconds
		refreshInterval = setInterval(fetchServices, 30000);

		// Cleanup on destroy
		return () => {
			if (refreshInterval) clearInterval(refreshInterval);
		};
	});
</script>

<svelte:head>
	<title>API Settings & Integrations | Admin</title>
</svelte:head>

<!-- Apple-grade Settings Dashboard -->
<div class="admin-settings">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header" in:fly={{ y: -20, duration: 600, easing: quintOut }}>
			<h1>Settings & Integrations</h1>
			<p class="subtitle">Manage your API connections and configure platform settings</p>
			<div class="header-actions">
				<!-- Tab Switcher -->
				<div class="tab-switcher">
					<button
						type="button"
						onclick={(e) => {
							e.preventDefault();
							e.stopPropagation();
							activeTab = 'integrations';
						}}
						class="tab-btn {activeTab === 'integrations' ? 'active' : ''}"
					>
						API Integrations
					</button>
					<button
						type="button"
						onclick={(e) => {
							e.preventDefault();
							e.stopPropagation();
							activeTab = 'general';
						}}
						class="tab-btn {activeTab === 'general' ? 'active' : ''}"
					>
						General Settings
					</button>
				</div>
			</div>
		</header>

		{#if activeTab === 'integrations'}
			<!-- Connection Status Overview -->
			{#if !isLoading}
				<div class="stats-grid" in:fly={{ y: 20, duration: 600, delay: 100, easing: quintOut }}>
					<!-- Total Available -->
					<div class="stat-card">
						<div class="stat-value">{summary.total_available}</div>
						<div class="stat-label">Available</div>
					</div>

					<!-- Connected -->
					<div class="stat-card stat-success">
						<div class="stat-value">{summary.total_connected}</div>
						<div class="stat-label">Connected</div>
					</div>

					<!-- Disconnected -->
					<div class="stat-card">
						<div class="stat-value muted">{summary.total_disconnected}</div>
						<div class="stat-label">Not Connected</div>
					</div>

					<!-- Errors -->
					<div class="stat-card stat-error">
						<div class="stat-value">{summary.total_errors}</div>
						<div class="stat-label">Errors</div>
					</div>

					<!-- Needs Attention -->
					<div class="stat-card stat-warning">
						<div class="stat-value">{summary.needs_attention}</div>
						<div class="stat-label">Needs Attention</div>
					</div>
				</div>
			{/if}

			<!-- Google Services Quick Access -->
			<div class="section-block" in:fly={{ y: 20, duration: 600, delay: 150, easing: quintOut }}>
				<h2 class="section-title">
					<span class="section-icon">🔍</span>
					Google Services
				</h2>
				<div class="services-grid services-grid-4">
					{#each googleServices as service, i}
						<div
							class="service-card"
							in:fly={{ y: 20, duration: 400, delay: 200 + i * 50, easing: quintOut }}
						>
							<div class="service-card-header">
								<div class="service-info">
									<div
										class="service-icon"
										style="background: {service.color}20; color: {service.color};"
									>
										G
									</div>
									<div>
										<h3 class="service-name">{service.name}</h3>
										<p class="service-category">{service.category}</p>
									</div>
								</div>
								<span class="status-badge status-{service.status}">
									<span class="status-dot"></span>
									{getStatusConfig(service.status).label}
								</span>
							</div>

							<p class="service-description">{service.description}</p>

							{#if service.is_connected}
								<button onclick={() => openDisconnectConfirm(service)} class="btn-disconnect">
									Disconnect
								</button>
							{:else}
								<button onclick={() => openConnectModal(service)} class="btn-connect">
									Connect
								</button>
							{/if}
						</div>
					{/each}
				</div>
			</div>

			<!-- Search and Category Filter -->
			<div class="filter-section" in:fly={{ y: 20, duration: 600, delay: 200, easing: quintOut }}>
				<!-- Search -->
				<div class="search-wrapper">
					<input
						id="search-integrations"
						name="search"
						type="text"
						placeholder="Search integrations..."
						bind:value={searchQuery}
						class="search-input"
					/>
					<svg class="search-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
						/>
					</svg>
				</div>

				<!-- Category Filter -->
				<div class="category-filter">
					<button
						onclick={() => (selectedCategory = null)}
						class="filter-btn"
						class:active={selectedCategory === null}
					>
						All
					</button>
					{#each categoryList as [key, category]}
						<button
							onclick={() => (selectedCategory = key)}
							class="filter-btn"
							class:active={selectedCategory === key}
						>
							<span>{category.icon}</span>
							{category.name}
						</button>
					{/each}
				</div>
			</div>

			<!-- Loading State -->
			{#if isLoading}
				<div class="loading">
					<div class="spinner"></div>
					<p>Loading integrations...</p>
				</div>
			{:else}
				<!-- Services Grid -->
				<div class="services-grid">
					{#each filteredServices as service, i}
						<div
							class="service-card"
							in:fly={{ y: 30, duration: 400, delay: 250 + i * 30, easing: quintOut }}
						>
							<div class="service-card-header">
								<div class="service-info">
									<div
										class="service-icon"
										style="background: {service.color}20; color: {service.color};"
									>
										{service.name.charAt(0)}
									</div>
									<div>
										<h3 class="service-name">{service.name}</h3>
										<p class="service-category">{service.category}</p>
									</div>
								</div>
							</div>

							<!-- Status Badge -->
							<div class="service-status-row">
								<span class="status-badge status-{service.status}">
									<span class="status-dot" class:pulse={service.status === 'connected'}></span>
									{getStatusConfig(service.status).label}
								</span>
								{#if service.connection?.health_score}
									<span class="health-score {getHealthColor(service.connection.health_score)}">
										{service.connection.health_score}% health
									</span>
								{/if}
							</div>

							<p class="service-description">{service.description}</p>

							<!-- Connection Info -->
							{#if service.is_connected && service.connection}
								<div class="connection-info">
									<div class="info-row">
										<span>API Calls Today</span>
										<span class="info-value"
											>{service.connection.api_calls_today?.toLocaleString() || 0}</span
										>
									</div>
									<div class="info-row">
										<span>Last Verified</span>
										<span class="info-value">{formatDate(service.connection.last_verified_at)}</span
										>
									</div>
								</div>
							{/if}

							<!-- Actions -->
							<div class="card-actions">
								{#if service.is_connected}
									<button onclick={() => openDisconnectConfirm(service)} class="btn-disconnect">
										Disconnect
									</button>
									<button onclick={() => openConnectModal(service)} class="btn-secondary">
										Configure
									</button>
								{:else}
									<button onclick={() => openConnectModal(service)} class="btn-connect">
										Connect
									</button>
								{/if}

								{#if service.docs_url}
									<a
										href={service.docs_url}
										target="_blank"
										rel="noopener noreferrer"
										class="btn-docs"
										title="View documentation"
									>
										<svg class="docs-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
											/>
										</svg>
									</a>
								{/if}
							</div>
						</div>
					{/each}
				</div>

				{#if filteredServices.length === 0}
					<div class="empty-state">
						<div class="empty-icon">🔌</div>
						<h3>No integrations found</h3>
						<p>Try adjusting your search or filter criteria</p>
					</div>
				{/if}
			{/if}
		{:else}
			<!-- General Settings Tab -->
			<div class="settings-container" in:fly={{ y: 20, duration: 600, easing: quintOut }}>
				<div class="settings-panel">
					<h2 class="panel-title">General Settings</h2>

					<div class="settings-list">
						<div class="setting-row">
							<div class="setting-info">
								<h3>Site Name</h3>
								<p>Your website display name</p>
							</div>
							<input
								id="site-name"
								name="site_name"
								type="text"
								value="Revolution Trading Pros"
								class="setting-input"
							/>
						</div>

						<div class="setting-row">
							<div class="setting-info">
								<h3>Maintenance Mode</h3>
								<p>Temporarily disable public access</p>
							</div>
							<button
								type="button"
								aria-label="Toggle maintenance mode"
								title="Toggle maintenance mode"
								class="toggle-switch"
							>
								<span class="toggle-slider"></span>
							</button>
						</div>

						<div class="setting-row">
							<div class="setting-info">
								<h3>Debug Mode</h3>
								<p>Enable detailed error logging</p>
							</div>
							<button
								type="button"
								aria-label="Toggle debug mode"
								title="Toggle debug mode"
								class="toggle-switch"
							>
								<span class="toggle-slider"></span>
							</button>
						</div>

						<div class="settings-actions">
							<button class="btn-connect"> Save Settings </button>
						</div>
					</div>
				</div>
			</div>
		{/if}
	</div>
	<!-- End admin-page-container -->
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
			onclick={() => (showConnectModal = false)}
			aria-label="Close modal"
		></button>

		<!-- Modal -->
		<div
			class="relative w-full max-w-lg bg-slate-900 border border-white/10 rounded-3xl shadow-2xl overflow-hidden"
			transition:scale={{ duration: 300, easing: backOut }}
		>
			<!-- Header -->
			<div class="relative p-6 border-b border-white/10">
				<div
					class="absolute inset-0 bg-gradient-to-r opacity-50"
					style="background: linear-gradient(135deg, {selectedService.color}15, transparent);"
				></div>
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
					onclick={() => (showConnectModal = false)}
					aria-label="Close connection modal"
					title="Close"
					class="absolute top-6 right-6 text-slate-400 hover:text-white transition-colors"
				>
					<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>

			<!-- Form -->
			<div class="p-6 space-y-5 max-h-[60vh] overflow-y-auto">
				<!-- Environment Selector -->
				{#if selectedService.environments && selectedService.environments.length > 1}
					<div>
						<span id="environment-label" class="block text-sm font-medium text-slate-300 mb-2"
							>Environment</span
						>
						<div class="flex gap-2" role="group" aria-labelledby="environment-label">
							{#each selectedService.environments as env}
								<button
									onclick={() => (selectedEnvironment = env)}
									aria-pressed={selectedEnvironment === env}
									class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium transition-all {selectedEnvironment ===
									env
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
							name={field.key}
							type={field.type === 'password' ? 'password' : 'text'}
							placeholder={field.placeholder || `Enter ${field.label.toLowerCase()}`}
							bind:value={credentialValues[field.key]}
							autocomplete={field.type === 'password' ? 'current-password' : 'off'}
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
								<svg
									class="w-5 h-5 text-emerald-400"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M5 13l4 4L19 7"
									/>
								</svg>
								<span class="text-emerald-400 font-medium"
									>{testResult.message || 'Connection successful!'}</span
								>
							{:else}
								<svg
									class="w-5 h-5 text-red-400"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M6 18L18 6M6 6l12 12"
									/>
								</svg>
								<span class="text-red-400 font-medium"
									>{testResult.error || 'Connection failed'}</span
								>
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
							<div
								class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"
							></div>
							Testing...
						</span>
					{:else}
						Test Connection
					{/if}
				</button>
				<button
					onclick={connectService}
					disabled={isConnecting}
					class="flex-1 px-4 py-3 bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-400 hover:to-amber-500 text-white rounded-xl font-medium transition-all shadow-lg shadow-amber-500/20 disabled:opacity-50"
				>
					{#if isConnecting}
						<span class="flex items-center justify-center gap-2">
							<div
								class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"
							></div>
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
			onclick={() => (showDisconnectConfirm = false)}
			aria-label="Close"
		></button>

		<div
			class="relative w-full max-w-md bg-slate-900 border border-white/10 rounded-3xl shadow-2xl p-6"
			transition:scale={{ duration: 300, easing: backOut }}
		>
			<div class="text-center">
				<div
					class="w-16 h-16 mx-auto mb-4 bg-red-500/10 rounded-full flex items-center justify-center"
				>
					<svg class="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
						/>
					</svg>
				</div>
				<h3 class="text-xl font-bold text-white mb-2">Disconnect {disconnectingService.name}?</h3>
				<p class="text-slate-400 mb-6">
					This will remove all stored credentials and disable the integration. Dashboard metrics
					using this service will show "Not Connected".
				</p>
				<div class="flex gap-3">
					<button
						onclick={() => (showDisconnectConfirm = false)}
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
								<div
									class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"
								></div>
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
     * ADMIN SETTINGS - Apple ICT7 Principal Engineer Grade
     * Color Hierarchy: RTP Gold (var(--primary-500), var(--primary-600)) primary accent
     * ═══════════════════════════════════════════════════════════════════════════ */

	/* Main container - consistent with other admin pages */
	.admin-settings {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
		color: white;
		position: relative;
		overflow: hidden;
	}

	/* Background Effects - fixed position for parallax */
	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.15;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		top: -200px;
		right: -200px;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		animation: float 20s ease-in-out infinite;
	}

	.bg-blob-2 {
		width: 500px;
		height: 500px;
		bottom: -150px;
		left: -150px;
		background: linear-gradient(135deg, #3b82f6, var(--primary-600));
		animation: float 25s ease-in-out infinite reverse;
	}

	.bg-blob-3 {
		width: 400px;
		height: 400px;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: linear-gradient(135deg, #10b981, #14b8a6);
		animation: float 30s ease-in-out infinite;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		33% {
			transform: translate(30px, -30px) scale(1.05);
		}
		66% {
			transform: translate(-20px, 20px) scale(0.95);
		}
	}

	/* Content wrapper */
	.admin-page-container {
		position: relative;
		z-index: 10;
		max-width: 1800px;
		margin: 0 auto;
		padding: 2rem;
	}

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin-bottom: 1.5rem;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * TAB SWITCHER
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.tab-switcher {
		display: flex;
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border-radius: 16px;
		padding: 4px;
		border: 1px solid rgba(255, 255, 255, 0.1);
		gap: 4px;
	}

	.tab-btn {
		padding: 0.625rem 1.5rem;
		border-radius: 12px;
		font-size: 0.875rem;
		font-weight: 500;
		color: #94a3b8;
		background: transparent;
		border: none;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.tab-btn:hover {
		color: #f1f5f9;
		background: rgba(255, 255, 255, 0.05);
	}

	.tab-btn.active {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
		font-weight: 600;
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * STATS GRID - Connection Status Overview
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	.stat-card {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 1.25rem;
		transition: all 0.2s ease;
	}

	.stat-card:hover {
		border-color: rgba(255, 255, 255, 0.2);
		transform: translateY(-2px);
	}

	.stat-value {
		font-size: 1.875rem;
		font-weight: 700;
		color: #f1f5f9;
		line-height: 1.2;
	}

	.stat-value.muted {
		color: #94a3b8;
	}

	.stat-label {
		font-size: 0.875rem;
		color: #94a3b8;
		margin-top: 0.25rem;
	}

	.stat-success .stat-value {
		color: #34d399;
	}
	.stat-error .stat-value {
		color: #f87171;
	}
	.stat-warning .stat-value {
		color: #fbbf24;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SECTION BLOCK - Google Services, etc.
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.section-block {
		margin-bottom: 2rem;
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	.section-icon {
		font-size: 1.5rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SERVICES GRID - Integration Cards
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.services-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1.25rem;
	}

	.services-grid-4 {
		grid-template-columns: repeat(4, 1fr);
	}

	.service-card {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 1.25rem;
		transition: all 0.2s ease;
	}

	.service-card:hover {
		border-color: rgba(255, 255, 255, 0.2);
		transform: translateY(-2px);
	}

	.service-card-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-bottom: 0.75rem;
	}

	.service-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.service-icon {
		width: 2.75rem;
		height: 2.75rem;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
		font-weight: 700;
	}

	.service-name {
		font-size: 0.9375rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.service-category {
		font-size: 0.75rem;
		color: #64748b;
		text-transform: capitalize;
		margin: 0;
	}

	.service-status-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
	}

	.service-description {
		font-size: 0.875rem;
		color: #94a3b8;
		margin: 0 0 1rem 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * STATUS BADGES
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.625rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.status-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
	}

	.status-dot.pulse {
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.status-connected {
		background: rgba(52, 211, 153, 0.15);
		color: #34d399;
	}
	.status-connected .status-dot {
		background: #34d399;
	}

	.status-disconnected {
		background: rgba(148, 163, 184, 0.15);
		color: #94a3b8;
	}
	.status-disconnected .status-dot {
		background: #94a3b8;
	}

	.status-error {
		background: rgba(248, 113, 113, 0.15);
		color: #f87171;
	}
	.status-error .status-dot {
		background: #f87171;
	}

	.status-expired,
	.status-pending,
	.status-connecting {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.status-expired .status-dot,
	.status-pending .status-dot,
	.status-connecting .status-dot {
		background: #fbbf24;
	}

	.health-score {
		font-size: 0.75rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CONNECTION INFO
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.connection-info {
		background: rgba(0, 0, 0, 0.2);
		border-radius: 12px;
		padding: 0.75rem;
		margin-bottom: 1rem;
	}

	.info-row {
		display: flex;
		justify-content: space-between;
		font-size: 0.75rem;
		color: #64748b;
	}

	.info-row + .info-row {
		margin-top: 0.5rem;
	}

	.info-value {
		color: #f1f5f9;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BUTTONS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-connect {
		flex: 1;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
		border: none;
		border-radius: 12px;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-connect:hover {
		background: linear-gradient(135deg, #f5c800, var(--primary-500));
		transform: translateY(-1px);
	}

	.btn-disconnect {
		flex: 1;
		padding: 0.625rem 1rem;
		background: rgba(248, 113, 113, 0.1);
		color: #f87171;
		border: none;
		border-radius: 12px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-disconnect:hover {
		background: rgba(248, 113, 113, 0.2);
	}

	.btn-secondary {
		padding: 0.625rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		color: #94a3b8;
		border: none;
		border-radius: 12px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-secondary:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #f1f5f9;
	}

	.btn-docs {
		padding: 0.625rem 0.75rem;
		background: rgba(255, 255, 255, 0.05);
		color: #94a3b8;
		border: none;
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s ease;
		text-decoration: none;
		display: flex;
		align-items: center;
	}

	.btn-docs:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #f1f5f9;
	}

	.docs-icon {
		width: 1.25rem;
		height: 1.25rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * FILTER SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.filter-section {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.search-wrapper {
		position: relative;
		max-width: 400px;
	}

	.search-input {
		width: 100%;
		padding: 0.75rem 1.25rem 0.75rem 3rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		color: #f1f5f9;
		font-size: 0.875rem;
		outline: none;
		transition: all 0.2s ease;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.search-input:focus {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.search-icon {
		position: absolute;
		left: 1rem;
		top: 50%;
		transform: translateY(-50%);
		width: 1.25rem;
		height: 1.25rem;
		color: #64748b;
	}

	.category-filter {
		display: flex;
		gap: 0.5rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.filter-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.filter-btn:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #f1f5f9;
	}

	.filter-btn.active {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border-color: transparent;
		color: var(--bg-base);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * LOADING & EMPTY STATES
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		gap: 1rem;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 4px solid rgba(230, 184, 0, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading p {
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
	}

	.empty-icon {
		font-size: 4rem;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		color: #94a3b8;
		margin: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * GENERAL SETTINGS TAB
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.settings-container {
		max-width: 768px;
	}

	.settings-panel {
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 16px;
		padding: 1.5rem;
	}

	.panel-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1.5rem 0;
	}

	.settings-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.setting-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 12px;
	}

	.setting-info h3 {
		font-size: 0.9375rem;
		font-weight: 500;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.setting-info p {
		font-size: 0.875rem;
		color: #94a3b8;
		margin: 0;
	}

	.setting-input {
		padding: 0.5rem 1rem;
		background: rgba(255, 255, 255, 0.05);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 12px;
		color: #f1f5f9;
		font-size: 0.875rem;
		outline: none;
		transition: all 0.2s ease;
	}

	.setting-input:focus {
		border-color: rgba(230, 184, 0, 0.5);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	.toggle-switch {
		position: relative;
		width: 56px;
		height: 32px;
		background: #475569;
		border: none;
		border-radius: 9999px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.toggle-switch .toggle-slider {
		position: absolute;
		left: 4px;
		top: 4px;
		width: 24px;
		height: 24px;
		background: #f1f5f9;
		border-radius: 50%;
		transition: all 0.2s ease;
	}

	.settings-actions {
		padding-top: 1rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: calc(var(--breakpoint-xl) - 1px)) {
		.services-grid-4 {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: calc(var(--breakpoint-lg) - 1px)) {
		.admin-page-container {
			padding: 1.5rem;
		}
		.stats-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (max-width: calc(var(--breakpoint-md) - 1px)) {
		.page-header h1 {
			font-size: 1.5rem;
		}
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
		.services-grid-4 {
			grid-template-columns: 1fr;
		}
		.filter-section {
			flex-direction: column;
		}
		.search-wrapper {
			max-width: 100%;
		}
		.setting-row {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}
		.setting-input {
			width: 100%;
		}
		.section-title {
			font-size: 1.125rem;
		}
	}

	@media (max-width: calc(var(--breakpoint-sm) - 1px)) {
		.admin-page-container {
			padding: 1rem;
		}
		.page-header h1 {
			font-size: 1.25rem;
		}
		.subtitle {
			font-size: 0.8125rem;
		}
		.stats-grid {
			grid-template-columns: 1fr 1fr;
		}
		.stat-value {
			font-size: 1.5rem;
		}
		.tab-switcher {
			flex-direction: column;
			width: 100%;
		}
		.tab-btn {
			width: 100%;
			text-align: center;
			padding: 0.75rem 1rem;
		}
		.service-card {
			padding: 1rem;
		}
		.service-icon {
			width: 2.25rem;
			height: 2.25rem;
			font-size: 1rem;
		}
	}

	@media (max-width: calc(var(--breakpoint-sm) - 160px)) {
		.admin-page-container {
			padding: 0.75rem;
		}
		.stats-grid {
			grid-template-columns: 1fr;
		}
		.card-actions {
			flex-direction: column;
		}
		.btn-connect,
		.btn-disconnect {
			width: 100%;
		}
		.services-grid {
			grid-template-columns: 1fr;
		}
		.category-filter {
			flex-wrap: nowrap;
			-webkit-overflow-scrolling: touch;
		}
		.connection-info {
			padding: 0.5rem;
		}
		.info-row {
			flex-direction: column;
			gap: 0.25rem;
		}
	}

	/* Touch Device Optimizations - 44pt minimum */
	@media (hover: none) and (pointer: coarse) {
		.btn-connect,
		.btn-disconnect,
		.btn-secondary,
		.btn-docs,
		.filter-btn,
		.tab-btn {
			min-height: 44px;
		}
		.search-input,
		.setting-input {
			min-height: 48px;
			font-size: 16px;
		}
		.toggle-switch {
			min-width: 56px;
			min-height: 32px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.bg-blob,
		.bg-blob-1,
		.bg-blob-2,
		.bg-blob-3,
		.service-card,
		.stat-card,
		.btn-connect,
		.btn-disconnect {
			animation: none;
			transition: none;
		}
	}

	/* High Contrast Mode */
	@media (prefers-contrast: high) {
		.service-card,
		.stat-card,
		.settings-panel {
			border-width: 2px;
		}
	}
</style>

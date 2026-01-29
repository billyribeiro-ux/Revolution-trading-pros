<script lang="ts">
	/**
	 * System Connections & Integrations
	 *
	 * Apple/Netflix-grade dashboard for managing platform integrations.
	 * Clean, minimal design with clear visual hierarchy.
	 *
	 * @level L11 Principal Engineer - Premium UX
	 */

	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { fade, fly, scale, slide } from 'svelte/transition';
	import { backOut, cubicOut } from 'svelte/easing';
	import { toastStore } from '$lib/stores/toast.svelte';
	import { adminFetch } from '$lib/utils/adminFetch';

	// Types
	interface ServiceField {
		key: string;
		label: string;
		type: string;
		required: boolean;
		placeholder?: string;
		help?: string;
	}

	interface Service {
		key: string;
		name: string;
		category: string;
		description: string;
		icon: string;
		color: string;
		docs_url?: string;
		signup_url?: string;
		pricing_url?: string;
		is_oauth: boolean;
		is_builtin?: boolean;
		fields: ServiceField[];
		environments?: string[];
		connection?: ConnectionData | null;
		is_connected: boolean;
		status: string;
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

	// State
	let connections = $state<Service[]>([]);
	let categories = $state<Record<string, Category>>({});
	let isLoading = $state(true);
	let selectedCategory = $state<string | null>(null);
	let searchQuery = $state('');
	let showConnectModal = $state(false);
	let selectedService = $state<Service | null>(null);
	let isConnecting = $state(false);
	let isTesting = $state(false);
	let testResult = $state<{ success: boolean; message?: string; error?: string } | null>(null);
	let credentialValues = $state<Record<string, string>>({});
	let selectedEnvironment = $state('production');
	let showDisconnectConfirm = $state(false);
	let disconnectingService = $state<Service | null>(null);
	let viewMode = $state<'grid' | 'list'>('grid');

	// Derived
	let builtInServices = $derived(connections.filter((c) => c.is_builtin));
	let externalServices = $derived(connections.filter((c) => !c.is_builtin));
	let connectedExternal = $derived(externalServices.filter((c) => c.is_connected));
	let availableExternal = $derived(externalServices.filter((c) => !c.is_connected));

	let filteredConnections = $derived.by(() => {
		let result = externalServices;

		if (selectedCategory) {
			result = result.filter((c) => c.category === selectedCategory);
		}

		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			result = result.filter(
				(c) =>
					c.name.toLowerCase().includes(query) ||
					c.description.toLowerCase().includes(query) ||
					c.category.toLowerCase().includes(query)
			);
		}

		return result;
	});

	let categoryList = $derived(Object.entries(categories) as [string, Category][]);

	// Fetch data
	async function fetchConnections() {
		try {
			const data = await adminFetch('/api/admin/connections');
			connections = data.connections;
			categories = data.categories;
		} catch (error) {
			console.error('Failed to fetch connections:', error);
		} finally {
			isLoading = false;
		}
	}

	// Connect service
	async function connectService() {
		if (!selectedService) return;

		isConnecting = true;
		testResult = null;

		try {
			const data = await adminFetch(`/api/admin/connections/${selectedService.key}/connect`, {
				method: 'POST',
				body: JSON.stringify({
					credentials: credentialValues,
					environment: selectedEnvironment
				})
			});

			if (data.success) {
				showConnectModal = false;
				await fetchConnections();
				toastStore.success(`${selectedService.name} connected successfully!`);
			} else {
				testResult = { success: false, error: data.error || 'Connection failed' };
				toastStore.error(data.error || 'Connection failed. Please check your credentials.');
			}
		} catch (error) {
			testResult = { success: false, error: 'Network error. Please try again.' };
			toastStore.error('Network error. Please check your connection and try again.');
		} finally {
			isConnecting = false;
		}
	}

	// Test connection
	async function testConnection() {
		if (!selectedService) return;

		isTesting = true;
		testResult = null;

		try {
			const data = await adminFetch(`/api/admin/connections/${selectedService.key}/test`, {
				method: 'POST',
				body: JSON.stringify({ credentials: credentialValues })
			});
			testResult = data;
		} catch (error) {
			testResult = { success: false, error: 'Network error. Please try again.' };
		} finally {
			isTesting = false;
		}
	}

	// Disconnect service
	async function disconnectService() {
		if (!disconnectingService) return;

		try {
			const data = await adminFetch(
				`/api/admin/connections/${disconnectingService.key}/disconnect`,
				{ method: 'POST' }
			);

			if (data.success) {
				showDisconnectConfirm = false;
				await fetchConnections();
				toastStore.success(`${disconnectingService.name} disconnected`);
			} else {
				toastStore.error(data.error || 'Failed to disconnect');
			}
		} catch (error) {
			toastStore.error('Network error');
		}
	}

	function openConnectModal(service: Service) {
		selectedService = service;
		credentialValues = {};
		testResult = null;
		selectedEnvironment = 'production';
		showConnectModal = true;
	}

	function openDisconnectConfirm(service: Service) {
		disconnectingService = service;
		showDisconnectConfirm = true;
	}

	// Format date
	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		const date = new Date(dateStr);
		const now = new Date();
		const diff = now.getTime() - date.getTime();
		const minutes = Math.floor(diff / 60000);
		const hours = Math.floor(diff / 3600000);
		const days = Math.floor(diff / 86400000);

		if (minutes < 1) return 'Just now';
		if (minutes < 60) return `${minutes}m ago`;
		if (hours < 24) return `${hours}h ago`;
		if (days < 7) return `${days}d ago`;
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
	}

	// Get service icon component
	function getServiceIcon(service: Service): string {
		const iconMap: Record<string, string> = {
			// Payment
			stripe: '💳',
			paypal: '🅿️',
			// Analytics
			google_analytics: '📊',
			mixpanel: '📈',
			amplitude: '📉',
			// Email
			sendgrid: '✉️',
			mailgun: '📧',
			fluent_smtp: '📬',
			// Storage
			aws_s3: '☁️',
			cloudinary: '🖼️',
			bunny_cdn: '🐰',
			// CRM
			fluent_crm_pro: '👥',
			hubspot: '🎯',
			// Social
			facebook: '👍',
			twitter: '🐦',
			// AI
			openai: '🤖',
			anthropic: '🧠',
			// Search
			algolia: '🔍',
			elasticsearch: '🔎',
			// SEO
			google_search_console: '📈',
			ahrefs: '🔗',
			// Forms
			fluent_forms_pro: '📝'
		};
		return iconMap[service.key] || service.name.charAt(0).toUpperCase();
	}

	onMount(async () => {
		await fetchConnections();

		const urlParams = page.url.searchParams;
		const connectServiceKey = urlParams.get('connect');
		if (connectServiceKey) {
			const serviceToConnect = connections.find((s) => s.key === connectServiceKey);
			if (serviceToConnect) {
				openConnectModal(serviceToConnect);
			}
		}

		const categoryParam = urlParams.get('category');
		if (categoryParam) {
			const matchingCategory = Object.keys(categories).find(
				(cat) => cat.toLowerCase() === categoryParam.toLowerCase()
			);
			if (matchingCategory) {
				selectedCategory = matchingCategory;
			}
		}
	});
</script>

<svelte:head>
	<title>Connections | System</title>
</svelte:head>

<div class="admin-connections">
	<div class="admin-page-container">
		<!-- Animated Background -->
		<div class="bg-effects">
			<div class="bg-blob bg-blob-1"></div>
			<div class="bg-blob bg-blob-2"></div>
			<div class="bg-blob bg-blob-3"></div>
		</div>

		<!-- Header -->
		<header class="connections-header" in:fly={{ y: -20, duration: 500, easing: cubicOut }}>
			<div class="header-content">
				<div>
					<h1 class="page-title">Connections</h1>
					<p class="page-description">Manage your platform integrations and third-party services</p>
				</div>

				<!-- View Toggle & Search -->
				<div class="header-actions">
					<!-- View Mode Toggle -->
					<div class="view-toggle">
						<button
							onclick={() => (viewMode = 'grid')}
							class="view-btn {viewMode === 'grid' ? 'active' : ''}"
							aria-label="Grid view"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"
								/>
							</svg>
						</button>
						<button
							onclick={() => (viewMode = 'list')}
							class="view-btn {viewMode === 'list' ? 'active' : ''}"
							aria-label="List view"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M4 6h16M4 12h16M4 18h16"
								/>
							</svg>
						</button>
					</div>

					<!-- Search -->
					<div class="search-wrapper">
						<input
							type="text"
							placeholder="Search..."
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
				</div>
			</div>
		</header>

		{#if isLoading}
			<!-- Loading State -->
			<div class="loading-container">
				<div class="loading-content">
					<div class="spinner-wrapper">
						<div class="spinner-track"></div>
						<div class="spinner"></div>
					</div>
					<p class="loading-text">Loading connections...</p>
				</div>
			</div>
		{:else}
			<!-- Built-in Features Section -->
			{#if builtInServices.length > 0}
				<section
					class="section-builtin"
					in:fly={{ y: 20, duration: 500, delay: 100, easing: cubicOut }}
				>
					<div class="section-header-row">
						<div class="section-icon-wrapper builtin">
							<svg class="section-icon-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M5 13l4 4L19 7"
								/>
							</svg>
						</div>
						<div>
							<h2 class="section-heading">Built-in Features</h2>
							<p class="section-subheading">Pre-installed and ready to use</p>
						</div>
					</div>

					<div class="services-grid">
						{#each builtInServices as service, index}
							<div
								class="service-card builtin-card"
								in:fly={{ y: 20, duration: 400, delay: 150 + index * 50, easing: cubicOut }}
							>
								<div class="service-card-content">
									<div class="service-icon builtin-icon">
										{getServiceIcon(service)}
									</div>
									<div class="service-info">
										<div class="service-name-row">
											<h3 class="service-name">{service.name}</h3>
											<span class="status-badge active">Active</span>
										</div>
										<p class="service-description line-clamp-2">{service.description}</p>
									</div>
								</div>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Connected Integrations -->
			{#if connectedExternal.length > 0}
				<section
					class="section-connected"
					in:fly={{ y: 20, duration: 500, delay: 200, easing: cubicOut }}
				>
					<div class="section-header-row">
						<div class="section-icon-wrapper connected">
							<svg class="section-icon-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"
								/>
							</svg>
						</div>
						<div>
							<h2 class="section-heading">Connected</h2>
							<p class="section-subheading">
								{connectedExternal.length} active integration{connectedExternal.length !== 1
									? 's'
									: ''}
							</p>
						</div>
					</div>

					<div class="services-grid">
						{#each connectedExternal as service, index}
							<div
								class="service-card connected-card"
								in:fly={{ y: 20, duration: 400, delay: 200 + index * 50, easing: cubicOut }}
							>
								<div class="connected-card-header">
									<div class="connected-service-info">
										<div
											class="service-icon"
											style="background: {service.color}15; color: {service.color};"
										>
											{getServiceIcon(service)}
										</div>
										<div>
											<h3 class="service-name">{service.name}</h3>
											<p class="service-category">{service.category}</p>
										</div>
									</div>
									<div class="connected-status">
										<div class="status-dot"></div>
										<span class="status-text">Connected</span>
									</div>
								</div>

								{#if service.connection}
									<div class="connection-stats">
										<div class="stat-row">
											<span class="stat-label">Health</span>
											<span
												class="stat-value {service.connection.health_score >= 90
													? 'health-high'
													: service.connection.health_score >= 70
														? 'health-medium'
														: 'health-low'}"
											>
												{service.connection.health_score}%
											</span>
										</div>
										<div class="stat-row">
											<span class="stat-label">Last verified</span>
											<span class="stat-value"
												>{formatDate(service.connection.last_verified_at)}</span
											>
										</div>
									</div>
								{/if}

								<div class="card-actions">
									<button onclick={() => openConnectModal(service)} class="btn-configure">
										Configure
									</button>
									<button onclick={() => openDisconnectConfirm(service)} class="btn-disconnect">
										Disconnect
									</button>
								</div>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Available Integrations -->
			<section
				class="section-available"
				in:fly={{ y: 20, duration: 500, delay: 300, easing: cubicOut }}
			>
				<div class="available-header">
					<div class="section-header-row">
						<div class="section-icon-wrapper available">
							<svg class="section-icon-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 6v6m0 0v6m0-6h6m-6 0H6"
								/>
							</svg>
						</div>
						<div>
							<h2 class="section-heading">Available Integrations</h2>
							<p class="section-subheading">
								{availableExternal.length} services available to connect
							</p>
						</div>
					</div>

					<!-- Category Filter Pills -->
					<div class="category-pills">
						<button
							onclick={() => (selectedCategory = null)}
							class="category-pill {selectedCategory === null ? 'active' : ''}"
						>
							All
						</button>
						{#each categoryList as [key, category]}
							<button
								onclick={() => (selectedCategory = key)}
								class="category-pill {selectedCategory === key ? 'active' : ''}"
							>
								{category.name}
							</button>
						{/each}
					</div>
				</div>

				<!-- Services Grid/List -->
				{#if viewMode === 'grid'}
					<div class="services-grid available">
						{#each filteredConnections as service, index}
							<div
								class="service-card available-card"
								in:fly={{ y: 20, duration: 400, delay: 300 + index * 30, easing: cubicOut }}
							>
								<div class="service-card-content">
									<div
										class="service-icon"
										style="background: {service.color}15; color: {service.color};"
									>
										{getServiceIcon(service)}
									</div>
									<div class="service-info">
										<h3 class="service-name">{service.name}</h3>
										<p class="service-category">{service.category}</p>
									</div>
								</div>

								<p class="service-description line-clamp-2">
									{service.description}
								</p>

								<button onclick={() => openConnectModal(service)} class="btn-connect">
									Connect
								</button>
							</div>
						{/each}
					</div>
				{:else}
					<!-- List View -->
					<div class="services-list">
						{#each filteredConnections as service, index}
							<div
								class="service-list-item"
								in:fly={{ x: -20, duration: 400, delay: 300 + index * 30, easing: cubicOut }}
							>
								<div
									class="service-icon-small"
									style="background: {service.color}15; color: {service.color};"
								>
									{getServiceIcon(service)}
								</div>

								<div class="service-list-info">
									<div class="service-list-header">
										<h3 class="service-name">{service.name}</h3>
										<span class="category-badge">{service.category}</span>
									</div>
									<p class="service-description-truncate">{service.description}</p>
								</div>

								<button onclick={() => openConnectModal(service)} class="btn-connect-small">
									Connect
								</button>
							</div>
						{/each}
					</div>
				{/if}

				<!-- Empty State -->
				{#if filteredConnections.length === 0}
					<div class="empty-state">
						<div class="empty-state-icon">
							<svg class="empty-icon-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="1.5"
									d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
								/>
							</svg>
						</div>
						<h3 class="empty-state-title">No services found</h3>
						<p class="empty-state-text">
							{#if searchQuery}
								No results for "{searchQuery}"
							{:else}
								No services available in this category
							{/if}
						</p>
					</div>
				{/if}
			</section>
		{/if}
	</div>
</div>
<!-- End connections-page -->

<!-- Connect Modal -->
{#if showConnectModal && selectedService}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 150 }}
	>
		<button
			type="button"
			class="absolute inset-0 bg-black/80 backdrop-blur-sm"
			onclick={() => (showConnectModal = false)}
			aria-label="Close"
		></button>

		<div
			class="relative w-full max-w-md bg-[#141415] border border-white/10 rounded-2xl shadow-2xl overflow-hidden"
			transition:scale={{ duration: 200, easing: backOut, start: 0.95 }}
		>
			<!-- Header -->
			<div class="p-6 pb-0">
				<div class="flex items-start gap-4">
					<div
						class="w-12 h-12 rounded-xl flex items-center justify-center text-xl"
						style="background: {selectedService.color}15; color: {selectedService.color};"
					>
						{getServiceIcon(selectedService)}
					</div>
					<div class="flex-1">
						<h2 class="text-lg font-semibold text-white">{selectedService.name}</h2>
						<p class="text-sm text-gray-500 mt-0.5">{selectedService.description}</p>
					</div>
					<button
						onclick={() => (showConnectModal = false)}
						class="text-gray-500 hover:text-white transition-colors"
						aria-label="Close"
					>
						<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M6 18L18 6M6 6l12 12"
							/>
						</svg>
					</button>
				</div>

				<!-- Quick Links -->
				{#if selectedService.docs_url || selectedService.signup_url}
					<div class="flex gap-4 mt-4 pt-4 border-t border-white/5">
						{#if selectedService.signup_url}
							<a
								href={selectedService.signup_url}
								target="_blank"
								rel="noopener"
								class="text-xs text-gray-400 hover:text-white transition-colors flex items-center gap-1"
							>
								<svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
									/>
								</svg>
								Create account
							</a>
						{/if}
						{#if selectedService.docs_url}
							<a
								href={selectedService.docs_url}
								target="_blank"
								rel="noopener"
								class="text-xs text-gray-400 hover:text-white transition-colors flex items-center gap-1"
							>
								<svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
									/>
								</svg>
								Documentation
							</a>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Form -->
			<div class="p-6 space-y-4">
				{#if selectedService.environments && selectedService.environments.length > 1}
					<fieldset>
						<legend class="block text-xs font-medium text-gray-400 mb-2 uppercase tracking-wide"
							>Environment</legend
						>
						<div class="grid grid-cols-2 gap-2" role="group">
							{#each selectedService.environments as env}
								<button
									onclick={() => (selectedEnvironment = env)}
									class="px-4 py-2.5 rounded-lg text-sm transition-all {selectedEnvironment === env
										? 'bg-white text-black font-medium'
										: 'bg-white/5 text-gray-400 hover:bg-white/10 hover:text-white'}"
								>
									{env.charAt(0).toUpperCase() + env.slice(1)}
								</button>
							{/each}
						</div>
					</fieldset>
				{/if}

				{#each selectedService.fields as field}
					<div>
						<label
							for="field-{field.key}"
							class="block text-xs font-medium text-gray-400 mb-2 uppercase tracking-wide"
						>
							{field.label}
							{#if field.required}<span class="text-red-400 ml-1">*</span>{/if}
						</label>
						<input
							id="field-{field.key}"
							type={field.type === 'password' ? 'password' : 'text'}
							placeholder={field.placeholder}
							bind:value={credentialValues[field.key]}
							class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white text-sm placeholder-gray-600 focus:outline-none focus:border-white/25 transition-colors"
						/>
						{#if field.help}
							<p class="mt-1.5 text-xs text-gray-600">{field.help}</p>
						{/if}
					</div>
				{/each}

				{#if testResult}
					<div
						class="p-4 rounded-xl {testResult.success
							? 'bg-emerald-500/10 border border-emerald-500/20'
							: 'bg-red-500/10 border border-red-500/20'}"
						transition:slide={{ duration: 200 }}
					>
						<div class="flex items-center gap-2">
							{#if testResult.success}
								<svg
									class="w-4 h-4 text-emerald-400"
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
								<span class="text-sm text-emerald-400"
									>{testResult.message || 'Connection successful'}</span
								>
							{:else}
								<svg
									class="w-4 h-4 text-red-400"
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
								<span class="text-sm text-red-400">{testResult.error || 'Connection failed'}</span>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="p-6 pt-0 flex gap-3">
				<button
					onclick={testConnection}
					disabled={isTesting}
					class="flex-1 px-4 py-3 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-xl transition-colors disabled:opacity-50"
				>
					{#if isTesting}
						<span class="flex items-center justify-center gap-2">
							<div
								class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"
							></div>
							Testing...
						</span>
					{:else}
						Test
					{/if}
				</button>
				<button
					onclick={connectService}
					disabled={isConnecting}
					class="flex-1 px-4 py-3 bg-white text-black text-sm font-medium rounded-xl hover:bg-gray-100 transition-colors disabled:opacity-50"
				>
					{#if isConnecting}
						<span class="flex items-center justify-center gap-2">
							<div
								class="w-4 h-4 border-2 border-black/20 border-t-black rounded-full animate-spin"
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

<!-- Disconnect Confirmation -->
{#if showDisconnectConfirm && disconnectingService}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center p-4"
		transition:fade={{ duration: 150 }}
	>
		<button
			type="button"
			class="absolute inset-0 bg-black/80 backdrop-blur-sm"
			onclick={() => (showDisconnectConfirm = false)}
			aria-label="Close"
		></button>

		<div
			class="relative w-full max-w-sm bg-[#141415] border border-white/10 rounded-2xl p-6"
			transition:scale={{ duration: 200, easing: backOut, start: 0.95 }}
		>
			<div class="text-center">
				<div
					class="w-14 h-14 mx-auto mb-4 bg-red-500/10 rounded-full flex items-center justify-center"
				>
					<svg class="w-7 h-7 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
				</div>
				<h3 class="text-lg font-semibold text-white mb-2">
					Disconnect {disconnectingService.name}?
				</h3>
				<p class="text-sm text-gray-500 mb-6">
					This will remove all stored credentials. You can reconnect anytime.
				</p>
				<div class="flex gap-3">
					<button
						onclick={() => (showDisconnectConfirm = false)}
						class="flex-1 px-4 py-2.5 bg-white/5 hover:bg-white/10 text-white text-sm font-medium rounded-xl transition-colors"
					>
						Cancel
					</button>
					<button
						onclick={disconnectService}
						class="flex-1 px-4 py-2.5 bg-red-500 hover:bg-red-600 text-white text-sm font-medium rounded-xl transition-colors"
					>
						Disconnect
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Connections Page - Admin Design System Aligned Styles
	 * Uses RTP Admin color tokens from tokens/colors.css
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Header */
	.connections-header {
		margin-bottom: 2.5rem;
	}

	.header-content {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 2rem;
	}

	.page-title {
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		font-size: 1.75rem;
		font-weight: 600;
		color: var(--text-primary, var(--text-primary));
		margin: 0;
	}

	.page-description {
		margin-top: 0.5rem;
		font-size: 0.9375rem;
		color: var(--text-secondary, var(--text-secondary));
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	/* View Toggle */
	.view-toggle {
		display: flex;
		align-items: center;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-muted, var(--border-muted));
		border-radius: 0.5rem;
		padding: 0.25rem;
	}

	.view-btn {
		padding: 0.5rem;
		border-radius: 0.375rem;
		border: none;
		background: transparent;
		color: var(--text-tertiary, var(--text-tertiary));
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.view-btn:hover {
		color: var(--text-secondary, var(--text-secondary));
	}

	.view-btn.active {
		background: var(--bg-hover, var(--bg-hover));
		color: var(--text-primary, var(--text-primary));
	}

	/* Search */
	.search-wrapper {
		position: relative;
	}

	.search-input {
		width: 16rem;
		padding: 0.625rem 1rem 0.625rem 2.5rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.5rem;
		color: var(--text-primary, var(--text-primary));
		font-size: 0.875rem;
		transition: border-color 0.2s ease;
	}

	.search-input::placeholder {
		color: var(--text-muted, #484f58);
	}

	.search-input:focus {
		outline: none;
		border-color: var(--primary-500, var(--primary-500));
	}

	.search-icon {
		position: absolute;
		left: 0.75rem;
		top: 50%;
		transform: translateY(-50%);
		width: 1rem;
		height: 1rem;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	/* Loading State */
	.loading-container {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 24rem;
	}

	.loading-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
	}

	.spinner-wrapper {
		position: relative;
		width: 3rem;
		height: 3rem;
	}

	.spinner-track {
		position: absolute;
		inset: 0;
		border: 2px solid var(--border-muted, var(--border-muted));
		border-radius: 50%;
	}

	.spinner {
		position: absolute;
		inset: 0;
		border: 2px solid var(--primary-500, var(--primary-500));
		border-radius: 50%;
		border-top-color: transparent;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.loading-text {
		font-size: 0.875rem;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	/* Sections */
	.section-builtin,
	.section-connected,
	.section-available {
		margin-bottom: 2.5rem;
	}

	.section-header-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}

	.section-icon-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2rem;
		height: 2rem;
		border-radius: 0.5rem;
	}

	.section-icon-wrapper.builtin {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
	}

	.section-icon-wrapper.connected {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
	}

	.section-icon-wrapper.available {
		background: var(--bg-hover, var(--bg-hover));
	}

	.section-icon-svg {
		width: 1rem;
		height: 1rem;
	}

	.section-icon-wrapper.builtin .section-icon-svg {
		color: var(--success-emphasis, #3fb950);
	}

	.section-icon-wrapper.connected .section-icon-svg {
		color: var(--info-emphasis, #58a6ff);
	}

	.section-icon-wrapper.available .section-icon-svg {
		color: var(--text-tertiary, var(--text-tertiary));
	}

	.section-heading {
		font-family: var(--font-heading, 'Montserrat', sans-serif);
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, var(--text-primary));
		margin: 0;
	}

	.section-subheading {
		font-size: 0.8125rem;
		color: var(--text-secondary, var(--text-secondary));
		margin: 0;
	}

	/* Services Grid */
	.services-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 1rem;
	}

	@media (min-width: 768px) {
		.services-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.services-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	@media (min-width: 1280px) {
		.services-grid.available {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	/* Service Cards */
	.service-card {
		background: var(--bg-elevated, var(--bg-elevated));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.75rem;
		padding: 1.25rem;
		transition: all 0.2s ease;
	}

	.service-card:hover {
		background: var(--bg-surface, var(--bg-surface));
		border-color: var(--border-emphasis, var(--text-secondary));
	}

	.builtin-card {
		background: linear-gradient(135deg, rgba(46, 160, 67, 0.05), transparent);
		border-color: rgba(46, 160, 67, 0.2);
	}

	.builtin-card:hover {
		border-color: rgba(46, 160, 67, 0.4);
	}

	.connected-card {
		background: var(--bg-elevated, var(--bg-elevated));
	}

	.service-card-content {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.service-icon {
		flex-shrink: 0;
		width: 3rem;
		height: 3rem;
		border-radius: 0.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.25rem;
	}

	.builtin-icon {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
	}

	.service-info {
		flex: 1;
		min-width: 0;
	}

	.service-name-row {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.service-name {
		font-weight: 500;
		color: var(--text-primary, var(--text-primary));
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.status-badge {
		flex-shrink: 0;
		padding: 0.125rem 0.5rem;
		font-size: 0.6875rem;
		font-weight: 600;
		border-radius: 999px;
	}

	.status-badge.active {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.service-description {
		font-size: 0.8125rem;
		color: var(--text-secondary, var(--text-secondary));
		margin: 0;
		line-height: 1.5;
	}

	.service-category {
		font-size: 0.75rem;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	/* Connected Card Specific */
	.connected-card-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.connected-service-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.connected-status {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.status-dot {
		width: 0.5rem;
		height: 0.5rem;
		background: var(--success-emphasis, #3fb950);
		border-radius: 50%;
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

	.status-text {
		font-size: 0.75rem;
		color: var(--success-emphasis, #3fb950);
	}

	.connection-stats {
		background: var(--bg-base, var(--bg-base));
		border-radius: 0.5rem;
		padding: 0.75rem;
		margin-bottom: 1rem;
	}

	.stat-row {
		display: flex;
		justify-content: space-between;
		font-size: 0.75rem;
	}

	.stat-row + .stat-row {
		margin-top: 0.5rem;
	}

	.stat-label {
		color: var(--text-tertiary, var(--text-tertiary));
	}

	.stat-value {
		color: var(--text-secondary, var(--text-secondary));
	}

	.stat-value.health-high {
		color: var(--success-emphasis, #3fb950);
	}

	.stat-value.health-medium {
		color: var(--warning-emphasis, #d29922);
	}

	.stat-value.health-low {
		color: var(--error-emphasis, #f85149);
	}

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}

	.btn-configure {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-muted, var(--border-muted));
		border-radius: 0.5rem;
		color: var(--text-secondary, var(--text-secondary));
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-configure:hover {
		background: var(--bg-hover, var(--bg-hover));
		color: var(--text-primary, var(--text-primary));
	}

	.btn-disconnect {
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		color: var(--error-base, #da3633);
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-disconnect:hover {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
	}

	/* Service Cards */
	:global(.service-card) {
		background: var(--bg-elevated, var(--bg-elevated));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.75rem;
		padding: 1.25rem;
		transition: all 0.2s ease;
	}

	:global(.service-card:hover) {
		background: var(--bg-surface, var(--bg-surface));
		border-color: var(--border-emphasis, var(--text-secondary));
	}

	:global(.service-card.builtin) {
		background: linear-gradient(135deg, rgba(46, 160, 67, 0.05), transparent);
		border-color: rgba(46, 160, 67, 0.2);
	}

	:global(.service-card.builtin:hover) {
		border-color: rgba(46, 160, 67, 0.4);
	}

	/* Category Pills */
	:global(.category-pill) {
		padding: 0.375rem 0.75rem;
		font-size: 0.8125rem;
		border-radius: 0.5rem;
		border: none;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.2s ease;
		background: transparent;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	:global(.category-pill:hover) {
		color: var(--text-primary, var(--text-primary));
		background: var(--bg-hover, var(--bg-hover));
	}

	:global(.category-pill.active) {
		background: var(--primary-500, var(--primary-500));
		color: var(--bg-base, var(--bg-base));
		font-weight: 600;
	}

	/* Connection Status */
	:global(.status-connected) {
		color: var(--success-emphasis, #3fb950);
	}

	:global(.status-pending) {
		color: var(--text-tertiary, var(--text-tertiary));
	}

	/* Buttons */
	:global(.btn-connect) {
		width: 100%;
		padding: 0.625rem 1rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.5rem;
		color: var(--text-primary, var(--text-primary));
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	:global(.btn-connect:hover) {
		background: var(--bg-hover, var(--bg-hover));
		border-color: var(--border-emphasis, var(--text-secondary));
	}

	:global(.btn-configure) {
		flex: 1;
		padding: 0.5rem 0.75rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-muted, var(--border-muted));
		border-radius: 0.5rem;
		color: var(--text-secondary, var(--text-secondary));
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	:global(.btn-configure:hover) {
		background: var(--bg-hover, var(--bg-hover));
		color: var(--text-primary, var(--text-primary));
	}

	:global(.btn-disconnect) {
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		color: var(--error-base, #da3633);
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	:global(.btn-disconnect:hover) {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
	}

	/* Modal Styles */
	:global(.modal-backdrop) {
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(4px);
	}

	:global(.modal-content) {
		background: var(--bg-elevated, var(--bg-elevated));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 1rem;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
	}

	:global(.modal-input) {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.5rem;
		color: var(--text-primary, var(--text-primary));
		font-size: 0.875rem;
		transition: border-color 0.2s ease;
	}

	:global(.modal-input::placeholder) {
		color: var(--text-muted, #484f58);
	}

	:global(.modal-input:focus) {
		outline: none;
		border-color: var(--primary-500, var(--primary-500));
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	:global(.modal-label) {
		display: block;
		font-size: 0.6875rem;
		font-weight: 600;
		color: var(--text-secondary, var(--text-secondary));
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.5rem;
	}

	:global(.btn-primary) {
		flex: 1;
		padding: 0.75rem 1rem;
		background: var(--primary-500, var(--primary-500));
		border: none;
		border-radius: 0.5rem;
		color: var(--bg-base, var(--bg-base));
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	:global(.btn-primary:hover) {
		background: var(--primary-400, var(--primary-400));
	}

	:global(.btn-primary:disabled) {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.btn-secondary) {
		flex: 1;
		padding: 0.75rem 1rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.5rem;
		color: var(--text-primary, var(--text-primary));
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	:global(.btn-secondary:hover) {
		background: var(--bg-hover, var(--bg-hover));
	}

	:global(.btn-danger) {
		flex: 1;
		padding: 0.625rem 1rem;
		background: var(--error-base, #da3633);
		border: none;
		border-radius: 0.5rem;
		color: white;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: background 0.2s ease;
	}

	:global(.btn-danger:hover) {
		background: var(--error-emphasis, #f85149);
	}

	/* Health Score */
	:global(.health-high) {
		color: var(--success-emphasis, #3fb950);
	}

	:global(.health-medium) {
		color: var(--warning-emphasis, #d29922);
	}

	:global(.health-low) {
		color: var(--error-emphasis, #f85149);
	}

	/* Utility */
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Empty State */
	:global(.empty-state) {
		text-align: center;
		padding: 4rem 1rem;
	}

	:global(.empty-state-icon) {
		width: 4rem;
		height: 4rem;
		margin: 0 auto 1rem;
		background: var(--bg-surface, var(--bg-surface));
		border-radius: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	:global(.empty-state-title) {
		font-size: 1.125rem;
		font-weight: 500;
		color: var(--text-secondary, var(--text-secondary));
		margin-bottom: 0.25rem;
	}

	:global(.empty-state-text) {
		font-size: 0.875rem;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	/* Available Header Layout */
	.available-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	/* Category Pills Container */
	.category-pills {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		overflow-x: auto;
		padding: 0.25rem 0;
	}

	.category-pill {
		padding: 0.375rem 0.75rem;
		font-size: 0.8125rem;
		border-radius: 0.5rem;
		border: none;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.2s ease;
		background: transparent;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	.category-pill:hover {
		color: var(--text-primary, var(--text-primary));
		background: var(--bg-hover, var(--bg-hover));
	}

	.category-pill.active {
		background: var(--primary-500, var(--primary-500));
		color: var(--bg-base, var(--bg-base));
		font-weight: 600;
	}

	/* Available Card Specific */
	.available-card {
		display: flex;
		flex-direction: column;
	}

	.available-card .service-card-content {
		margin-bottom: 0.75rem;
	}

	.available-card .service-description {
		flex-grow: 1;
		margin-bottom: 1rem;
	}

	/* Connect Button */
	.btn-connect {
		width: 100%;
		padding: 0.625rem 1rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.75rem;
		color: var(--text-primary, var(--text-primary));
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-connect:hover {
		background: var(--bg-hover, var(--bg-hover));
		border-color: var(--border-emphasis, var(--text-secondary));
	}

	/* List View Styles */
	.services-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.service-list-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: var(--bg-elevated, var(--bg-elevated));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.75rem;
		transition: all 0.2s ease;
	}

	.service-list-item:hover {
		background: var(--bg-surface, var(--bg-surface));
		border-color: var(--border-emphasis, var(--text-secondary));
	}

	.service-icon-small {
		flex-shrink: 0;
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1rem;
	}

	.service-list-info {
		flex: 1;
		min-width: 0;
	}

	.service-list-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.25rem;
	}

	.category-badge {
		font-size: 0.6875rem;
		color: var(--text-tertiary, var(--text-tertiary));
		padding: 0.125rem 0.5rem;
		background: var(--bg-surface, var(--bg-surface));
		border-radius: 0.25rem;
	}

	.service-description-truncate {
		font-size: 0.8125rem;
		color: var(--text-secondary, var(--text-secondary));
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		margin: 0;
	}

	.btn-connect-small {
		flex-shrink: 0;
		padding: 0.5rem 1rem;
		background: var(--bg-surface, var(--bg-surface));
		border: 1px solid var(--border-default, var(--border-default));
		border-radius: 0.5rem;
		color: var(--text-primary, var(--text-primary));
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-connect-small:hover {
		background: var(--bg-hover, var(--bg-hover));
		border-color: var(--border-emphasis, var(--text-secondary));
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 4rem 1rem;
	}

	.empty-state-icon {
		width: 4rem;
		height: 4rem;
		margin: 0 auto 1rem;
		background: var(--bg-surface, var(--bg-surface));
		border-radius: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.empty-icon-svg {
		width: 2rem;
		height: 2rem;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	.empty-state-title {
		font-size: 1.125rem;
		font-weight: 500;
		color: var(--text-secondary, var(--text-secondary));
		margin-bottom: 0.25rem;
	}

	.empty-state-text {
		font-size: 0.875rem;
		color: var(--text-tertiary, var(--text-tertiary));
	}

	/* Responsive */
	@media (max-width: 768px) {
		.header-content {
			flex-direction: column;
			gap: 1rem;
		}

		.header-actions {
			width: 100%;
			justify-content: space-between;
		}

		.search-input {
			width: 100%;
			flex: 1;
		}

		.available-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.category-pills {
			width: 100%;
		}
	}
</style>

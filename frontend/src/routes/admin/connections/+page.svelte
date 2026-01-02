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
	import { toastStore } from '$lib/stores/toast';
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
	let builtInServices = $derived(connections.filter(c => c.is_builtin));
	let externalServices = $derived(connections.filter(c => !c.is_builtin));
	let connectedExternal = $derived(externalServices.filter(c => c.is_connected));
	let availableExternal = $derived(externalServices.filter(c => !c.is_connected));

	let filteredConnections = $derived(() => {
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
			fluent_forms_pro: '📝',
		};
		return iconMap[service.key] || service.name.charAt(0).toUpperCase();
	}

	onMount(async () => {
		await fetchConnections();

		const urlParams = page.url.searchParams;
		const connectServiceKey = urlParams.get('connect');
		if (connectServiceKey) {
			const serviceToConnect = connections.find(s => s.key === connectServiceKey);
			if (serviceToConnect) {
				openConnectModal(serviceToConnect);
			}
		}

		const categoryParam = urlParams.get('category');
		if (categoryParam) {
			const matchingCategory = Object.keys(categories).find(
				cat => cat.toLowerCase() === categoryParam.toLowerCase()
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

<div class="min-h-screen bg-[#0a0a0b]">
	<!-- Subtle gradient background -->
	<div class="fixed inset-0 bg-gradient-to-b from-gray-900/50 via-transparent to-transparent pointer-events-none"></div>

	<div class="relative max-w-7xl mx-auto px-6 py-8">
		<!-- Header -->
		<header class="mb-12" in:fly={{ y: -20, duration: 500, easing: cubicOut }}>
			<div class="flex items-start justify-between">
				<div>
					<h1 class="text-3xl font-semibold text-white tracking-tight">
						Connections
					</h1>
					<p class="mt-2 text-gray-500 text-base max-w-xl">
						Manage your platform integrations and third-party services
					</p>
				</div>

				<!-- View Toggle & Search -->
				<div class="flex items-center gap-4">
					<!-- View Mode Toggle -->
					<div class="flex items-center bg-white/5 rounded-lg p-1">
						<button
							onclick={() => viewMode = 'grid'}
							class="p-2 rounded-md transition-all {viewMode === 'grid' ? 'bg-white/10 text-white' : 'text-gray-500 hover:text-gray-300'}"
							aria-label="Grid view"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
							</svg>
						</button>
						<button
							onclick={() => viewMode = 'list'}
							class="p-2 rounded-md transition-all {viewMode === 'list' ? 'bg-white/10 text-white' : 'text-gray-500 hover:text-gray-300'}"
							aria-label="List view"
						>
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
							</svg>
						</button>
					</div>

					<!-- Search -->
					<div class="relative">
						<input
							type="text"
							placeholder="Search..."
							bind:value={searchQuery}
							class="w-64 px-4 py-2.5 pl-10 bg-white/5 border border-white/10 rounded-xl text-white text-sm placeholder-gray-500 focus:outline-none focus:border-white/20 transition-colors"
						/>
						<svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
						</svg>
					</div>
				</div>
			</div>
		</header>

		{#if isLoading}
			<!-- Loading State -->
			<div class="flex items-center justify-center h-96">
				<div class="flex flex-col items-center gap-4">
					<div class="relative w-12 h-12">
						<div class="absolute inset-0 border-2 border-white/10 rounded-full"></div>
						<div class="absolute inset-0 border-2 border-white rounded-full animate-spin border-t-transparent"></div>
					</div>
					<p class="text-gray-500 text-sm">Loading connections...</p>
				</div>
			</div>
		{:else}
			<!-- Built-in Features Section -->
			{#if builtInServices.length > 0}
				<section class="mb-12" in:fly={{ y: 20, duration: 500, delay: 100, easing: cubicOut }}>
					<div class="flex items-center gap-3 mb-6">
						<div class="flex items-center justify-center w-8 h-8 bg-emerald-500/10 rounded-lg">
							<svg class="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
							</svg>
						</div>
						<div>
							<h2 class="text-lg font-medium text-white">Built-in Features</h2>
							<p class="text-sm text-gray-500">Pre-installed and ready to use</p>
						</div>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
						{#each builtInServices as service, index}
							<div
								class="group relative bg-gradient-to-br from-emerald-500/5 to-transparent border border-emerald-500/20 rounded-2xl p-5 hover:border-emerald-500/30 transition-all duration-300"
								in:fly={{ y: 20, duration: 400, delay: 150 + index * 50, easing: cubicOut }}
							>
								<div class="flex items-start gap-4">
									<div class="flex-shrink-0 w-12 h-12 bg-emerald-500/10 rounded-xl flex items-center justify-center text-xl">
										{getServiceIcon(service)}
									</div>
									<div class="flex-1 min-w-0">
										<div class="flex items-center gap-2">
											<h3 class="font-medium text-white truncate">{service.name}</h3>
											<span class="flex-shrink-0 px-2 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs font-medium rounded-full">
												Active
											</span>
										</div>
										<p class="mt-1 text-sm text-gray-500 line-clamp-2">{service.description}</p>
									</div>
								</div>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Connected Integrations -->
			{#if connectedExternal.length > 0}
				<section class="mb-12" in:fly={{ y: 20, duration: 500, delay: 200, easing: cubicOut }}>
					<div class="flex items-center justify-between mb-6">
						<div class="flex items-center gap-3">
							<div class="flex items-center justify-center w-8 h-8 bg-blue-500/10 rounded-lg">
								<svg class="w-4 h-4 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
								</svg>
							</div>
							<div>
								<h2 class="text-lg font-medium text-white">Connected</h2>
								<p class="text-sm text-gray-500">{connectedExternal.length} active integration{connectedExternal.length !== 1 ? 's' : ''}</p>
							</div>
						</div>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
						{#each connectedExternal as service, index}
							<div
								class="group relative bg-white/[0.02] border border-white/10 rounded-2xl p-5 hover:bg-white/[0.04] hover:border-white/15 transition-all duration-300"
								in:fly={{ y: 20, duration: 400, delay: 200 + index * 50, easing: cubicOut }}
							>
								<div class="flex items-start justify-between mb-4">
									<div class="flex items-center gap-3">
										<div
											class="w-10 h-10 rounded-xl flex items-center justify-center text-lg"
											style="background: {service.color}15; color: {service.color};"
										>
											{getServiceIcon(service)}
										</div>
										<div>
											<h3 class="font-medium text-white">{service.name}</h3>
											<p class="text-xs text-gray-500">{service.category}</p>
										</div>
									</div>
									<div class="flex items-center gap-1.5">
										<div class="w-2 h-2 bg-emerald-400 rounded-full animate-pulse"></div>
										<span class="text-xs text-emerald-400">Connected</span>
									</div>
								</div>

								{#if service.connection}
									<div class="space-y-2 mb-4 p-3 bg-black/20 rounded-xl">
										<div class="flex justify-between text-xs">
											<span class="text-gray-500">Health</span>
											<span class="{service.connection.health_score >= 90 ? 'text-emerald-400' : service.connection.health_score >= 70 ? 'text-yellow-400' : 'text-red-400'}">
												{service.connection.health_score}%
											</span>
										</div>
										<div class="flex justify-between text-xs">
											<span class="text-gray-500">Last verified</span>
											<span class="text-gray-300">{formatDate(service.connection.last_verified_at)}</span>
										</div>
									</div>
								{/if}

								<div class="flex gap-2">
									<button
										onclick={() => openConnectModal(service)}
										class="flex-1 px-3 py-2 bg-white/5 hover:bg-white/10 text-gray-300 text-sm rounded-lg transition-colors"
									>
										Configure
									</button>
									<button
										onclick={() => openDisconnectConfirm(service)}
										class="px-3 py-2 text-red-400 hover:bg-red-500/10 text-sm rounded-lg transition-colors"
									>
										Disconnect
									</button>
								</div>
							</div>
						{/each}
					</div>
				</section>
			{/if}

			<!-- Available Integrations -->
			<section in:fly={{ y: 20, duration: 500, delay: 300, easing: cubicOut }}>
				<div class="flex items-center justify-between mb-6">
					<div class="flex items-center gap-3">
						<div class="flex items-center justify-center w-8 h-8 bg-gray-500/10 rounded-lg">
							<svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
							</svg>
						</div>
						<div>
							<h2 class="text-lg font-medium text-white">Available Integrations</h2>
							<p class="text-sm text-gray-500">{availableExternal.length} services available to connect</p>
						</div>
					</div>

					<!-- Category Filter Pills -->
					<div class="flex items-center gap-2 overflow-x-auto">
						<button
							onclick={() => (selectedCategory = null)}
							class="px-3 py-1.5 text-sm rounded-lg whitespace-nowrap transition-all {selectedCategory === null
								? 'bg-white text-black font-medium'
								: 'text-gray-400 hover:text-white hover:bg-white/5'}"
						>
							All
						</button>
						{#each categoryList as [key, category]}
							<button
								onclick={() => (selectedCategory = key)}
								class="px-3 py-1.5 text-sm rounded-lg whitespace-nowrap transition-all {selectedCategory === key
									? 'bg-white text-black font-medium'
									: 'text-gray-400 hover:text-white hover:bg-white/5'}"
							>
								{category.name}
							</button>
						{/each}
					</div>
				</div>

				<!-- Services Grid/List -->
				{#if viewMode === 'grid'}
					<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
						{#each filteredConnections() as service, index}
							<div
								class="group relative bg-white/[0.02] border border-white/10 rounded-2xl p-5 hover:bg-white/[0.04] hover:border-white/15 transition-all duration-300 flex flex-col"
								in:fly={{ y: 20, duration: 400, delay: 300 + index * 30, easing: cubicOut }}
							>
								<div class="flex items-start gap-3 mb-3">
									<div
										class="flex-shrink-0 w-11 h-11 rounded-xl flex items-center justify-center text-lg font-semibold"
										style="background: {service.color}12; color: {service.color};"
									>
										{getServiceIcon(service)}
									</div>
									<div class="flex-1 min-w-0">
										<h3 class="font-medium text-white truncate">{service.name}</h3>
										<p class="text-xs text-gray-500">{service.category}</p>
									</div>
								</div>

								<p class="text-sm text-gray-500 line-clamp-2 mb-4 flex-grow">{service.description}</p>

								<button
									onclick={() => openConnectModal(service)}
									class="w-full px-4 py-2.5 bg-white/5 hover:bg-white/10 border border-white/10 hover:border-white/20 text-white text-sm font-medium rounded-xl transition-all duration-200 group-hover:bg-white/10"
								>
									Connect
								</button>
							</div>
						{/each}
					</div>
				{:else}
					<!-- List View -->
					<div class="space-y-2">
						{#each filteredConnections() as service, index}
							<div
								class="group flex items-center gap-4 p-4 bg-white/[0.02] border border-white/10 rounded-xl hover:bg-white/[0.04] hover:border-white/15 transition-all duration-300"
								in:fly={{ x: -20, duration: 400, delay: 300 + index * 30, easing: cubicOut }}
							>
								<div
									class="flex-shrink-0 w-10 h-10 rounded-lg flex items-center justify-center text-base"
									style="background: {service.color}12; color: {service.color};"
								>
									{getServiceIcon(service)}
								</div>

								<div class="flex-1 min-w-0">
									<div class="flex items-center gap-2">
										<h3 class="font-medium text-white">{service.name}</h3>
										<span class="text-xs text-gray-500 px-2 py-0.5 bg-white/5 rounded">{service.category}</span>
									</div>
									<p class="text-sm text-gray-500 truncate">{service.description}</p>
								</div>

								<button
									onclick={() => openConnectModal(service)}
									class="flex-shrink-0 px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 text-white text-sm rounded-lg transition-colors"
								>
									Connect
								</button>
							</div>
						{/each}
					</div>
				{/if}

				<!-- Empty State -->
				{#if filteredConnections().length === 0}
					<div class="text-center py-16">
						<div class="w-16 h-16 mx-auto mb-4 bg-white/5 rounded-2xl flex items-center justify-center">
							<svg class="w-8 h-8 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
							</svg>
						</div>
						<h3 class="text-lg font-medium text-gray-400 mb-1">No services found</h3>
						<p class="text-sm text-gray-600">
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
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
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
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
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
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
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
						<legend class="block text-xs font-medium text-gray-400 mb-2 uppercase tracking-wide">Environment</legend>
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
						<label for="field-{field.key}" class="block text-xs font-medium text-gray-400 mb-2 uppercase tracking-wide">
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
						class="p-4 rounded-xl {testResult.success ? 'bg-emerald-500/10 border border-emerald-500/20' : 'bg-red-500/10 border border-red-500/20'}"
						transition:slide={{ duration: 200 }}
					>
						<div class="flex items-center gap-2">
							{#if testResult.success}
								<svg class="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
								<span class="text-sm text-emerald-400">{testResult.message || 'Connection successful'}</span>
							{:else}
								<svg class="w-4 h-4 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
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
							<div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
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
							<div class="w-4 h-4 border-2 border-black/20 border-t-black rounded-full animate-spin"></div>
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
				<div class="w-14 h-14 mx-auto mb-4 bg-red-500/10 rounded-full flex items-center justify-center">
					<svg class="w-7 h-7 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
				</div>
				<h3 class="text-lg font-semibold text-white mb-2">Disconnect {disconnectingService.name}?</h3>
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
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>

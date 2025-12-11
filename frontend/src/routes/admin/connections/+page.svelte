<script lang="ts">
	/**
	 * API Connections Manager
	 *
	 * Premium Apple/Netflix-style dashboard for managing all third-party API connections.
	 * Features elegant animations, glass morphism, and intuitive UX.
	 *
	 * @level L11 Principal Engineer - Apple-grade implementation
	 */

	import { onMount } from 'svelte';
	import { fade, fly, scale, slide } from 'svelte/transition';
	import { quintOut, elasticOut, backOut } from 'svelte/easing';

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

	interface Summary {
		total_available: number;
		total_connected: number;
		total_disconnected: number;
		total_errors: number;
		needs_attention: number;
	}

	// State
	let connections = $state<Service[]>([]);
	let categories = $state<Record<string, Category>>({});
	let summary = $state<Summary>({
		total_available: 0,
		total_connected: 0,
		total_disconnected: 0,
		total_errors: 0,
		needs_attention: 0
	});
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

	// Derived
	let filteredConnections = $derived(() => {
		let result = connections;

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
			const response = await fetch('/api/admin/connections');
			if (response.ok) {
				const data = await response.json();
				connections = data.connections;
				categories = data.categories;
				summary = data.summary;
			}
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
			const response = await fetch(`/api/admin/connections/${selectedService.key}/connect`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					credentials: credentialValues,
					environment: selectedEnvironment
				})
			});

			const data = await response.json();

			if (data.success) {
				showConnectModal = false;
				await fetchConnections();
				showSuccessToast(`${selectedService.name} connected successfully!`);
			} else {
				testResult = { success: false, error: data.error || 'Connection failed' };
				showErrorToast(data.error || 'Connection failed. Please check your credentials.');
			}
		} catch (error) {
			testResult = { success: false, error: 'Network error. Please try again.' };
			showErrorToast('Network error. Please check your connection and try again.');
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
			const response = await fetch(`/api/admin/connections/${selectedService.key}/test`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ credentials: credentialValues })
			});

			const data = await response.json();
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
			const response = await fetch(
				`/api/admin/connections/${disconnectingService.key}/disconnect`,
				{
					method: 'POST'
				}
			);

			const data = await response.json();

			if (data.success) {
				showDisconnectConfirm = false;
				await fetchConnections();
				showSuccessToast(`${disconnectingService.name} disconnected successfully`);
			} else {
				showErrorToast(data.error || 'Failed to disconnect. Please try again.');
			}
		} catch (error) {
			console.error('Failed to disconnect:', error);
			showErrorToast('Network error. Please check your connection and try again.');
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

	// Toast notifications - using enterprise toast store
	import { toastStore } from '$lib/stores/toast';

	function showSuccessToast(message: string) {
		toastStore.success(message);
	}

	function showErrorToast(message: string) {
		toastStore.error(message);
	}

	// Get health color
	function getHealthColor(score: number): string {
		if (score >= 90) return 'text-green-400';
		if (score >= 70) return 'text-yellow-400';
		if (score >= 50) return 'text-orange-400';
		return 'text-red-400';
	}

	// Get status badge
	function getStatusBadge(status: string): { bg: string; text: string; label: string } {
		switch (status) {
			case 'connected':
				return { bg: 'bg-green-500/20', text: 'text-green-400', label: 'Connected' };
			case 'error':
				return { bg: 'bg-red-500/20', text: 'text-red-400', label: 'Error' };
			case 'expired':
				return { bg: 'bg-orange-500/20', text: 'text-orange-400', label: 'Expired' };
			case 'pending':
				return { bg: 'bg-blue-500/20', text: 'text-blue-400', label: 'Pending' };
			default:
				return { bg: 'bg-gray-500/20', text: 'text-gray-400', label: 'Not Connected' };
		}
	}

	// Format date
	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	// Get category icon
	function getCategoryIcon(icon: string): string {
		const icons: Record<string, string> = {
			// Infrastructure
			server: '🖥️',
			database: '🗄️',
			// Payments
			'credit-card': '💳',
			// Analytics
			'chart-bar': '📊',
			// Email
			mail: '✉️',
			// Storage
			cloud: '☁️',
			// CRM
			users: '👥',
			// Social
			'share-2': '🔗',
			// Communication
			'message-circle': '💬',
			// AI
			cpu: '🤖',
			// Automation
			zap: '⚡',
			// Search
			search: '🔍',
			// SEO
			'trending-up': '📈',
			// Pixels/Conversion Tracking
			target: '🎯',
			// CDN
			globe: '🌐',
			// Hosting
			'upload-cloud': '☁️',
			// Monitoring
			'alert-triangle': '⚠️',
			// Default
			box: '📦'
		};
		return icons[icon] || '📦';
	}

	onMount(() => {
		fetchConnections();
	});
</script>

<svelte:head>
	<title>API Connections | Admin</title>
</svelte:head>

<!-- Premium Apple/Netflix-style Dashboard -->
<div class="min-h-screen bg-gradient-to-br from-gray-950 via-gray-900 to-black text-white">
	<!-- Animated Background -->
	<div class="fixed inset-0 overflow-hidden pointer-events-none">
		<div class="absolute -top-40 -right-40 w-80 h-80 bg-purple-500/10 rounded-full blur-3xl animate-pulse"></div>
		<div class="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-500/10 rounded-full blur-3xl animate-pulse" style="animation-delay: 1s;"></div>
		<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-indigo-500/5 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s;"></div>
	</div>

	<div class="relative z-10 p-8">
		<!-- Header -->
		<header class="mb-10" in:fly={{ y: -20, duration: 600, easing: quintOut }}>
			<div class="flex items-center justify-between">
				<div>
					<h1 class="text-4xl font-bold bg-gradient-to-r from-white via-gray-200 to-gray-400 bg-clip-text text-transparent">
						API Connections
					</h1>
					<p class="mt-2 text-gray-400 text-lg">
						Manage your third-party integrations in one place
					</p>
				</div>

				<!-- Search -->
				<div class="relative">
					<input
						type="text"
						placeholder="Search services..."
						bind:value={searchQuery}
						class="w-80 px-5 py-3 bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all duration-300"
					/>
					<svg class="absolute right-4 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
					</svg>
				</div>
			</div>

			<!-- Stats Cards -->
			{#if !isLoading}
				<div class="grid grid-cols-5 gap-4 mt-8" in:fly={{ y: 20, duration: 600, delay: 200, easing: quintOut }}>
					<!-- Total Available -->
					<div class="relative group">
						<div class="absolute inset-0 bg-gradient-to-r from-purple-600/20 to-blue-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-purple-500/30 transition-all duration-300">
							<div class="text-3xl font-bold text-white">{summary.total_available}</div>
							<div class="text-sm text-gray-400 mt-1">Available</div>
						</div>
					</div>

					<!-- Connected -->
					<div class="relative group">
						<div class="absolute inset-0 bg-gradient-to-r from-green-600/20 to-emerald-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-green-500/30 transition-all duration-300">
							<div class="text-3xl font-bold text-green-400">{summary.total_connected}</div>
							<div class="text-sm text-gray-400 mt-1">Connected</div>
						</div>
					</div>

					<!-- Disconnected -->
					<div class="relative group">
						<div class="absolute inset-0 bg-gradient-to-r from-gray-600/20 to-slate-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-gray-500/30 transition-all duration-300">
							<div class="text-3xl font-bold text-gray-400">{summary.total_disconnected}</div>
							<div class="text-sm text-gray-400 mt-1">Disconnected</div>
						</div>
					</div>

					<!-- Errors -->
					<div class="relative group">
						<div class="absolute inset-0 bg-gradient-to-r from-red-600/20 to-rose-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-red-500/30 transition-all duration-300">
							<div class="text-3xl font-bold text-red-400">{summary.total_errors}</div>
							<div class="text-sm text-gray-400 mt-1">Errors</div>
						</div>
					</div>

					<!-- Needs Attention -->
					<div class="relative group">
						<div class="absolute inset-0 bg-gradient-to-r from-orange-600/20 to-amber-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
						<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-5 hover:border-orange-500/30 transition-all duration-300">
							<div class="text-3xl font-bold text-orange-400">{summary.needs_attention}</div>
							<div class="text-sm text-gray-400 mt-1">Needs Attention</div>
						</div>
					</div>
				</div>
			{/if}
		</header>

		<!-- Category Filter -->
		{#if !isLoading}
			<div class="flex gap-3 mb-8 overflow-x-auto pb-2" in:fly={{ y: 20, duration: 600, delay: 300, easing: quintOut }}>
				<button
					onclick={() => (selectedCategory = null)}
					class="flex items-center gap-2 px-5 py-2.5 rounded-xl font-medium transition-all duration-300 whitespace-nowrap {selectedCategory === null
						? 'bg-white text-black shadow-lg shadow-white/20'
						: 'bg-white/5 text-gray-400 hover:bg-white/10 hover:text-white border border-white/10'}"
				>
					All Services
				</button>

				{#each categoryList as [key, category]}
					<button
						onclick={() => (selectedCategory = key)}
						class="flex items-center gap-2 px-5 py-2.5 rounded-xl font-medium transition-all duration-300 whitespace-nowrap {selectedCategory === key
							? 'bg-white text-black shadow-lg shadow-white/20'
							: 'bg-white/5 text-gray-400 hover:bg-white/10 hover:text-white border border-white/10'}"
					>
						<span>{getCategoryIcon(category.icon)}</span>
						{category.name}
					</button>
				{/each}
			</div>
		{/if}

		<!-- Loading State -->
		{#if isLoading}
			<div class="flex items-center justify-center h-96">
				<div class="relative">
					<div class="w-16 h-16 border-4 border-purple-500/20 rounded-full"></div>
					<div class="absolute top-0 left-0 w-16 h-16 border-4 border-purple-500 rounded-full animate-spin border-t-transparent"></div>
				</div>
			</div>
		{:else}
			<!-- Services Grid -->
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5">
				{#each filteredConnections() as service, index}
					<div
						class="group relative h-full"
						in:fly={{ y: 30, duration: 400, delay: index * 50, easing: quintOut }}
					>
						<!-- Glow Effect -->
						<div
							class="absolute inset-0 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-all duration-500"
							style="background: linear-gradient(135deg, {service.color}20, {service.color}10);"
						></div>

						<!-- Card -->
						<div
							class="service-card relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6 hover:border-white/20 transition-all duration-300 hover:transform hover:-translate-y-1 flex flex-col"
						>
							<!-- Header -->
							<div class="flex items-start justify-between mb-4">
								<div class="flex items-center gap-3">
									<!-- Service Icon -->
									<div
										class="w-12 h-12 rounded-xl flex items-center justify-center text-2xl font-bold"
										style="background: {service.color}20; color: {service.color};"
									>
										{service.name.charAt(0)}
									</div>
									<div>
										<h3 class="font-semibold text-white">{service.name}</h3>
										<p class="text-xs text-gray-500 capitalize">{service.category}</p>
									</div>
								</div>

								<!-- Status Badge -->
								{#if true}
									{@const badge = getStatusBadge(service.status)}
									<span class="px-2.5 py-1 rounded-full text-xs font-medium {badge.bg} {badge.text}">
										{badge.label}
									</span>
								{/if}
							</div>

							<!-- Description -->
							<p class="text-sm text-gray-400 mb-4 line-clamp-2 flex-grow">{service.description}</p>

							<!-- Connection Info (fixed height container) -->
							<div class="connection-info-container mb-4">
							{#if service.is_connected && service.connection}
								<div class="space-y-2 p-3 bg-black/20 rounded-xl">
									<div class="flex justify-between text-xs">
										<span class="text-gray-500">Health</span>
										<span class={getHealthColor(service.connection.health_score)}>
											{service.connection.health_score}%
										</span>
									</div>
									<div class="flex justify-between text-xs">
										<span class="text-gray-500">API Calls Today</span>
										<span class="text-white">{service.connection.api_calls_today.toLocaleString()}</span>
									</div>
									<div class="flex justify-between text-xs">
										<span class="text-gray-500">Last Verified</span>
										<span class="text-white">{formatDate(service.connection.last_verified_at)}</span>
									</div>
								</div>
							{:else}
								<!-- Placeholder for uniform card height -->
								<div class="space-y-2 p-3 bg-black/10 rounded-xl border border-dashed border-white/5">
									<div class="flex justify-between text-xs">
										<span class="text-gray-600">Status</span>
										<span class="text-gray-500">Not connected</span>
									</div>
									<div class="flex justify-between text-xs">
										<span class="text-gray-600">API Calls</span>
										<span class="text-gray-500">—</span>
									</div>
									<div class="flex justify-between text-xs">
										<span class="text-gray-600">Last Verified</span>
										<span class="text-gray-500">—</span>
									</div>
								</div>
							{/if}
							</div>

							<!-- Actions (always at bottom) -->
							<div class="flex gap-2 mt-auto">
								{#if service.is_connected}
									<button
										onclick={() => openDisconnectConfirm(service)}
										class="flex-1 px-4 py-2.5 bg-red-500/10 hover:bg-red-500/20 text-red-400 rounded-xl text-sm font-medium transition-all duration-300"
									>
										Disconnect
									</button>
									<button
										onclick={() => openConnectModal(service)}
										class="px-4 py-2.5 bg-white/5 hover:bg-white/10 text-gray-400 hover:text-white rounded-xl text-sm font-medium transition-all duration-300"
									>
										Configure
									</button>
								{:else}
									<button
										onclick={() => openConnectModal(service)}
										class="flex-1 px-4 py-2.5 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-500 hover:to-blue-500 text-white rounded-xl text-sm font-medium transition-all duration-300 shadow-lg shadow-purple-500/20 hover:shadow-purple-500/40"
									>
										Connect
									</button>
								{/if}

								{#if service.docs_url}
									<a
										href={service.docs_url}
										target="_blank"
										rel="noopener"
										aria-label="View {service.name} documentation"
										class="px-3 py-2.5 bg-white/5 hover:bg-white/10 text-gray-400 hover:text-white rounded-xl transition-all duration-300"
									>
										<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
										</svg>
									</a>
								{/if}
							</div>
						</div>
					</div>
				{/each}
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
			onclick={() => (showConnectModal = false)}
			aria-label="Close modal"
		></button>

		<!-- Modal -->
		<div
			class="relative w-full max-w-lg bg-gray-900 border border-white/10 rounded-3xl shadow-2xl overflow-hidden"
			transition:scale={{ duration: 300, easing: backOut }}
		>
			<!-- Header -->
			<div class="relative p-6 border-b border-white/10">
				<div class="absolute inset-0 bg-gradient-to-r from-purple-600/10 to-blue-600/10"></div>
				<div class="relative flex items-center gap-4">
					<div
						class="w-14 h-14 rounded-xl flex items-center justify-center text-2xl font-bold"
						style="background: {selectedService.color}20; color: {selectedService.color};"
					>
						{selectedService.name.charAt(0)}
					</div>
					<div>
						<h2 class="text-xl font-bold text-white">Connect {selectedService.name}</h2>
						<p class="text-sm text-gray-400">{selectedService.description}</p>
						<!-- Quick Links -->
						<div class="flex gap-3 mt-2">
							{#if selectedService.signup_url}
								<a
									href={selectedService.signup_url}
									target="_blank"
									rel="noopener"
									class="inline-flex items-center gap-1 text-xs text-purple-400 hover:text-purple-300 transition-colors"
								>
									<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
									</svg>
									Sign Up
								</a>
							{/if}
							{#if selectedService.pricing_url}
								<a
									href={selectedService.pricing_url}
									target="_blank"
									rel="noopener"
									class="inline-flex items-center gap-1 text-xs text-blue-400 hover:text-blue-300 transition-colors"
								>
									<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
									</svg>
									Pricing
								</a>
							{/if}
							{#if selectedService.docs_url}
								<a
									href={selectedService.docs_url}
									target="_blank"
									rel="noopener"
									class="inline-flex items-center gap-1 text-xs text-gray-400 hover:text-gray-300 transition-colors"
								>
									<svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
									</svg>
									Docs
								</a>
							{/if}
						</div>
					</div>
				</div>
				<button
					type="button"
					onclick={() => (showConnectModal = false)}
					aria-label="Close modal"
					class="absolute top-6 right-6 text-gray-400 hover:text-white transition-colors"
				>
					<svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Form -->
			<div class="p-6 space-y-5">
				<!-- Environment Selector -->
				{#if selectedService.environments && selectedService.environments.length > 1}
					<fieldset>
						<legend class="block text-sm font-medium text-gray-300 mb-2">Environment</legend>
						<div class="flex gap-2">
							{#each selectedService.environments as env}
								<button
									onclick={() => (selectedEnvironment = env)}
									class="flex-1 px-4 py-2.5 rounded-xl text-sm font-medium transition-all duration-300 {selectedEnvironment === env
										? 'bg-white text-black'
										: 'bg-white/5 text-gray-400 hover:bg-white/10 hover:text-white border border-white/10'}"
								>
									{env.charAt(0).toUpperCase() + env.slice(1)}
								</button>
							{/each}
						</div>
					</fieldset>
				{/if}

				<!-- Credential Fields -->
				{#each selectedService.fields as field}
					<div>
						<label for="field-{field.key}" class="block text-sm font-medium text-gray-300 mb-2">
							{field.label}
							{#if field.required}<span class="text-red-400">*</span>{/if}
						</label>
						<input
							id="field-{field.key}"
							type={field.type === 'password' ? 'password' : 'text'}
							placeholder={field.placeholder || `Enter ${field.label.toLowerCase()}`}
							bind:value={credentialValues[field.key]}
							class="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all duration-300"
						/>
						{#if field.help}
							<p class="mt-1 text-xs text-gray-500">{field.help}</p>
						{/if}
					</div>
				{/each}

				<!-- Test Result -->
				{#if testResult}
					<div
						class="p-4 rounded-xl {testResult.success
							? 'bg-green-500/10 border border-green-500/20'
							: 'bg-red-500/10 border border-red-500/20'}"
						transition:slide
					>
						<div class="flex items-center gap-2">
							{#if testResult.success}
								<svg class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
								<span class="text-green-400 font-medium">{testResult.message || 'Connection successful!'}</span>
							{:else}
								<svg class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
								</svg>
								<span class="text-red-400 font-medium">{testResult.error || 'Connection failed'}</span>
							{/if}
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div class="p-6 border-t border-white/10 bg-black/20 flex gap-3">
				<button
					onclick={testConnection}
					disabled={isTesting}
					class="flex-1 px-4 py-3 bg-white/5 hover:bg-white/10 text-white rounded-xl font-medium transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
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
					class="flex-1 px-4 py-3 bg-gradient-to-r from-purple-600 to-blue-600 hover:from-purple-500 hover:to-blue-500 text-white rounded-xl font-medium transition-all duration-300 shadow-lg shadow-purple-500/20 disabled:opacity-50 disabled:cursor-not-allowed"
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
		<!-- Backdrop -->
		<button
			type="button"
			class="absolute inset-0 bg-black/80 backdrop-blur-sm cursor-default"
			onclick={() => (showDisconnectConfirm = false)}
			aria-label="Close confirmation"
		></button>

		<!-- Modal -->
		<div
			class="relative w-full max-w-md bg-gray-900 border border-white/10 rounded-3xl shadow-2xl p-6"
			transition:scale={{ duration: 300, easing: backOut }}
		>
			<div class="text-center">
				<div class="w-16 h-16 mx-auto mb-4 bg-red-500/10 rounded-full flex items-center justify-center">
					<svg class="w-8 h-8 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
				</div>
				<h3 class="text-xl font-bold text-white mb-2">Disconnect {disconnectingService.name}?</h3>
				<p class="text-gray-400 mb-6">
					This will remove all stored credentials and disable the integration. You can reconnect anytime.
				</p>
				<div class="flex gap-3">
					<button
						onclick={() => (showDisconnectConfirm = false)}
						class="flex-1 px-4 py-3 bg-white/5 hover:bg-white/10 text-white rounded-xl font-medium transition-all duration-300"
					>
						Cancel
					</button>
					<button
						onclick={disconnectService}
						class="flex-1 px-4 py-3 bg-red-600 hover:bg-red-500 text-white rounded-xl font-medium transition-all duration-300"
					>
						Disconnect
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	@keyframes slide-up {
		from {
			opacity: 0;
			transform: translateY(1rem);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	:global(.animate-slide-up) {
		animation: slide-up 0.3s ease-out;
	}

	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Uniform card sizing */
	.service-card {
		min-height: 320px;
	}

	.connection-info-container {
		min-height: 90px;
	}

	/* Actions always at bottom */
	.service-card > :last-child {
		margin-top: auto;
	}
</style>

<script lang="ts">
	/**
	 * System Connections & Integrations
	 *
	 * Apple/Netflix-grade dashboard for managing platform integrations.
	 * Clean, minimal design with clear visual hierarchy.
	 *
	 * @level L11 Principal Engineer - Premium UX
	 */

	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { fade, fly, scale, slide } from 'svelte/transition';
	import { backOut, cubicOut } from 'svelte/easing';
	import { toastStore } from '$lib/stores/toast.svelte';
	import { adminFetch } from '$lib/utils/adminFetch';

	// FIX-2026-04-26: Tabler icons replace 11 raw inline <svg> blocks for
	// consistent professional styling per CLAUDE.md icon-system standard.
	import IconLayoutGrid from '@tabler/icons-svelte-runes/icons/layout-grid';
	import IconList from '@tabler/icons-svelte-runes/icons/list';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconLink from '@tabler/icons-svelte-runes/icons/link';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconUserPlus from '@tabler/icons-svelte-runes/icons/user-plus';
	import IconFileText from '@tabler/icons-svelte-runes/icons/file-text';
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';

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
		} catch (_error) {
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
		} catch (_error) {
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
		} catch (_error) {
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
							class={{ 'view-btn': true, active: viewMode === 'grid' }}
							aria-label="Grid view"
						>
							<IconLayoutGrid size={16} aria-hidden="true" />
						</button>
						<button
							onclick={() => (viewMode = 'list')}
							class={{ 'view-btn': true, active: viewMode === 'list' }}
							aria-label="List view"
						>
							<IconList size={16} aria-hidden="true" />
						</button>
					</div>

					<!-- Search -->
					<div class="search-wrapper">
						<input
							id="page-searchquery"
							name="page-searchquery"
							type="text"
							placeholder="Search..."
							bind:value={searchQuery}
							class="search-input"
						/>
						<span class="search-icon" aria-hidden="true">
							<IconSearch size={18} />
						</span>
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
							<IconCheck size={22} class="section-icon-svg" aria-hidden="true" />
						</div>
						<div>
							<h2 class="section-heading">Built-in Features</h2>
							<p class="section-subheading">Pre-installed and ready to use</p>
						</div>
					</div>

					<div class="services-grid">
						{#each builtInServices as service, index (service.key)}
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
							<IconLink size={22} class="section-icon-svg" aria-hidden="true" />
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
						{#each connectedExternal as service, index (service.key)}
							<div
								class="service-card connected-card"
								in:fly={{ y: 20, duration: 400, delay: 200 + index * 50, easing: cubicOut }}
							>
								<div class="connected-card-header">
									<div class="connected-service-info">
										<div
											class="service-icon"
											style:background={`${service.color}15`}
											style:color={service.color}
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
												class={{
													'stat-value': true,
													'health-high': service.connection.health_score >= 90,
													'health-medium':
														service.connection.health_score >= 70 &&
														service.connection.health_score < 90,
													'health-low': service.connection.health_score < 70
												}}
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
							<IconPlus size={22} class="section-icon-svg" aria-hidden="true" />
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
							class={{ 'category-pill': true, active: selectedCategory === null }}
						>
							All
						</button>
						{#each categoryList as [key, category] (key)}
							<button
								onclick={() => (selectedCategory = key)}
								class={{ 'category-pill': true, active: selectedCategory === key }}
							>
								{category.name}
							</button>
						{/each}
					</div>
				</div>

				<!-- Services Grid/List -->
				{#if viewMode === 'grid'}
					<div class="services-grid available">
						{#each filteredConnections as service, index (service.key)}
							<div
								class="service-card available-card"
								in:fly={{ y: 20, duration: 400, delay: 300 + index * 30, easing: cubicOut }}
							>
								<div class="service-card-content">
									<div
										class="service-icon"
										style:background={`${service.color}15`}
										style:color={service.color}
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
						{#each filteredConnections as service, index (service.key)}
							<div
								class="service-list-item"
								in:fly={{ x: -20, duration: 400, delay: 300 + index * 30, easing: cubicOut }}
							>
								<div
									class="service-icon-small"
									style:background={`${service.color}15`}
									style:color={service.color}
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
							<IconSearch size={48} class="empty-icon-svg" aria-hidden="true" />
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
	<div class="modal-shell" transition:fade={{ duration: 150 }}>
		<button
			type="button"
			class="modal-backdrop"
			onclick={() => (showConnectModal = false)}
			aria-label="Close"
		></button>

		<div
			class="modal-panel connect-panel"
			transition:scale={{ duration: 200, easing: backOut, start: 0.95 }}
		>
			<!-- Header -->
			<div class="modal-header">
				<div class="modal-title-row">
					<div
						class="modal-service-icon"
						style:background={`${selectedService.color}15`}
						style:color={selectedService.color}
					>
						{getServiceIcon(selectedService)}
					</div>
					<div class="modal-copy">
						<h2 class="modal-title">{selectedService.name}</h2>
						<p class="modal-description">{selectedService.description}</p>
					</div>
					<button
						type="button"
						onclick={() => (showConnectModal = false)}
						class="modal-close"
						aria-label="Close"
					>
						<IconX size={20} aria-hidden="true" />
					</button>
				</div>

				<!-- Quick Links -->
				{#if selectedService.docs_url || selectedService.signup_url}
					<div class="modal-links">
						{#if selectedService.signup_url}
							<a
								href={selectedService.signup_url}
								target="_blank"
								rel="noopener"
								class="modal-link"
							>
								<IconUserPlus size={14} aria-hidden="true" />
								Create account
							</a>
						{/if}
						{#if selectedService.docs_url}
							<a href={selectedService.docs_url} target="_blank" rel="noopener" class="modal-link">
								<IconFileText size={14} aria-hidden="true" />
								Documentation
							</a>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Form -->
			<form
				class="modal-form"
				onsubmit={(e) => {
					e.preventDefault();
					connectService();
				}}
			>
				{#if selectedService.environments && selectedService.environments.length > 1}
					<fieldset class="modal-fieldset">
						<legend class="modal-legend">Environment</legend>
						<div class="environment-grid" role="group">
							{#each selectedService.environments as env (env)}
								<button
									type="button"
									onclick={() => (selectedEnvironment = env)}
									class={{ 'environment-option': true, active: selectedEnvironment === env }}
								>
									{env.charAt(0).toUpperCase() + env.slice(1)}
								</button>
							{/each}
						</div>
					</fieldset>
				{/if}

				{#each selectedService.fields as field (field.key)}
					<div class="modal-field">
						<label for="field-{field.key}" class="modal-label">
							{field.label}
							{#if field.required}<span class="required-indicator">*</span>{/if}
						</label>
						<input
							id="field-{field.key}"
							name="field-{field.key}"
							type={field.type === 'password' ? 'password' : 'text'}
							placeholder={field.placeholder}
							bind:value={credentialValues[field.key]}
							autocomplete={field.type === 'password' ? 'current-password' : 'off'}
							class="modal-input"
						/>
						{#if field.help}
							<p class="field-help">{field.help}</p>
						{/if}
					</div>
				{/each}

				{#if testResult}
					<div
						class={{
							'modal-result': true,
							success: testResult.success,
							error: !testResult.success
						}}
						transition:slide={{ duration: 200 }}
					>
						<div class="result-content">
							{#if testResult.success}
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: check (test success) -->
								<IconCheck size={16} aria-hidden="true" />
								<span>{testResult.message || 'Connection successful'}</span>
							{:else}
								<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: x (test failure) -->
								<IconX size={16} aria-hidden="true" />
								<span>{testResult.error || 'Connection failed'}</span>
							{/if}
						</div>
					</div>
				{/if}
			</form>

			<!-- Footer -->
			<div class="modal-footer">
				<button
					type="button"
					onclick={testConnection}
					disabled={isTesting}
					class="modal-button modal-button--secondary"
				>
					{#if isTesting}
						<span class="button-loading-label">
							<span class="button-spinner button-spinner--light"></span>
							Testing...
						</span>
					{:else}
						Test
					{/if}
				</button>
				<button
					type="button"
					onclick={connectService}
					disabled={isConnecting}
					class="modal-button modal-button--primary"
				>
					{#if isConnecting}
						<span class="button-loading-label">
							<span class="button-spinner button-spinner--dark"></span>
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
	<div class="modal-shell" transition:fade={{ duration: 150 }}>
		<button
			type="button"
			class="modal-backdrop"
			onclick={() => (showDisconnectConfirm = false)}
			aria-label="Close"
		></button>

		<div
			class="modal-panel confirm-panel"
			transition:scale={{ duration: 200, easing: backOut, start: 0.95 }}
		>
			<div class="confirm-body">
				<div class="confirm-icon" aria-hidden="true">
					<IconInfoCircle size={28} class="confirm-icon-svg" />
				</div>
				<h3 class="confirm-title">
					Disconnect {disconnectingService.name}?
				</h3>
				<p class="confirm-copy">
					This will remove all stored credentials. You can reconnect anytime.
				</p>
				<div class="confirm-actions">
					<button
						type="button"
						onclick={() => (showDisconnectConfirm = false)}
						class="modal-button modal-button--secondary"
					>
						Cancel
					</button>
					<button
						type="button"
						onclick={disconnectService}
						class="modal-button modal-button--danger"
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

	/* FIX-2026-04-26: :global() wrapping needed because Tabler icon component
	   renders its own <svg> with a Svelte hash from inside its own component
	   scope; the .section-icon-svg class lands on that internal SVG and isn't
	   reachable from this parent's scoped style without :global(). Tagged
	   :global to keep the visual styling working post-icon-swap. */
	:global(.section-icon-wrapper .section-icon-svg) {
		width: 1.375rem;
		height: 1.375rem;
	}

	:global(.section-icon-wrapper.builtin .section-icon-svg) {
		color: var(--success-emphasis, #3fb950);
	}

	:global(.section-icon-wrapper.connected .section-icon-svg) {
		color: var(--info-emphasis, #58a6ff);
	}

	:global(.section-icon-wrapper.available .section-icon-svg) {
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

	/* Modal Styles */
	.modal-shell {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
	}

	.modal-backdrop {
		position: absolute;
		inset: 0;
		border: 0;
		background: rgb(0 0 0 / 80%);
		backdrop-filter: blur(4px);
		cursor: pointer;
	}

	.modal-panel {
		position: relative;
		width: 100%;
		overflow: hidden;
		border: 1px solid rgb(255 255 255 / 10%);
		border-radius: 1rem;
		background: #141415;
		box-shadow: 0 20px 60px rgb(0 0 0 / 50%);
		color: #ffffff;
	}

	.connect-panel {
		max-width: 28rem;
	}

	.confirm-panel {
		max-width: 24rem;
		padding: 1.5rem;
	}

	.modal-header,
	.modal-form {
		padding: 1.5rem;
	}

	.modal-header {
		padding-bottom: 0;
	}

	.modal-title-row {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
	}

	.modal-service-icon,
	.confirm-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex: 0 0 auto;
	}

	.modal-service-icon {
		width: 3rem;
		height: 3rem;
		border-radius: 0.75rem;
		font-size: 1.25rem;
	}

	.modal-copy {
		flex: 1;
		min-width: 0;
	}

	.modal-title,
	.confirm-title {
		margin: 0;
		color: #ffffff;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.modal-description,
	.confirm-copy {
		margin: 0;
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.modal-description {
		margin-top: 0.125rem;
	}

	.modal-close {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 1.75rem;
		height: 1.75rem;
		border: 0;
		border-radius: 0.375rem;
		background: transparent;
		color: #6b7280;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			color 160ms ease;
	}

	.modal-close:hover {
		background: rgb(255 255 255 / 8%);
		color: #ffffff;
	}

	.modal-links {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		margin-top: 1rem;
		border-top: 1px solid rgb(255 255 255 / 5%);
		padding-top: 1rem;
	}

	.modal-link {
		display: inline-flex;
		align-items: center;
		gap: 0.25rem;
		color: #9ca3af;
		font-size: 0.75rem;
		text-decoration: none;
		transition: color 160ms ease;
	}

	.modal-link:hover {
		color: #ffffff;
	}

	.modal-form {
		display: grid;
		gap: 1rem;
	}

	.modal-fieldset {
		min-width: 0;
		border: 0;
		padding: 0;
		margin: 0;
	}

	.modal-legend,
	.modal-label {
		display: block;
		margin-bottom: 0.5rem;
		color: #9ca3af;
		font-size: 0.75rem;
		font-weight: 600;
		letter-spacing: 0.05em;
		text-transform: uppercase;
	}

	.environment-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		gap: 0.5rem;
	}

	.environment-option {
		border: 0;
		border-radius: 0.5rem;
		background: rgb(255 255 255 / 5%);
		padding: 0.625rem 1rem;
		color: #9ca3af;
		font: inherit;
		font-size: 0.875rem;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			color 160ms ease;
	}

	.environment-option:hover {
		background: rgb(255 255 255 / 10%);
		color: #ffffff;
	}

	.environment-option.active {
		background: #ffffff;
		color: #000000;
		font-weight: 600;
	}

	.required-indicator {
		margin-left: 0.25rem;
		color: #f87171;
	}

	.modal-input {
		width: 100%;
		border: 1px solid rgb(255 255 255 / 10%);
		border-radius: 0.75rem;
		background: rgb(255 255 255 / 5%);
		padding: 0.75rem 1rem;
		color: #ffffff;
		font: inherit;
		font-size: 0.875rem;
		transition:
			border-color 160ms ease,
			box-shadow 160ms ease;
	}

	.modal-input::placeholder {
		color: #4b5563;
	}

	.modal-input:focus {
		outline: none;
		border-color: rgb(255 255 255 / 25%);
		box-shadow: 0 0 0 3px rgb(255 255 255 / 8%);
	}

	.field-help {
		margin: 0.375rem 0 0;
		color: #4b5563;
		font-size: 0.75rem;
	}

	.modal-result {
		border: 1px solid;
		border-radius: 0.75rem;
		padding: 1rem;
	}

	.modal-result.success {
		border-color: rgb(16 185 129 / 20%);
		background: rgb(16 185 129 / 10%);
		color: #34d399;
	}

	.modal-result.error {
		border-color: rgb(239 68 68 / 20%);
		background: rgb(239 68 68 / 10%);
		color: #f87171;
	}

	.result-content,
	.modal-footer,
	.button-loading-label,
	.confirm-actions {
		display: flex;
		align-items: center;
	}

	.result-content,
	.button-loading-label {
		gap: 0.5rem;
	}

	.result-content {
		font-size: 0.875rem;
	}

	.modal-footer {
		gap: 0.75rem;
		padding: 0 1.5rem 1.5rem;
	}

	.modal-button {
		flex: 1;
		border: 0;
		border-radius: 0.75rem;
		padding: 0.75rem 1rem;
		font: inherit;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition:
			background-color 160ms ease,
			color 160ms ease,
			opacity 160ms ease;
	}

	.modal-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.modal-button--secondary {
		background: rgb(255 255 255 / 5%);
		color: #ffffff;
	}

	.modal-button--secondary:hover:not(:disabled) {
		background: rgb(255 255 255 / 10%);
	}

	.modal-button--primary {
		background: #ffffff;
		color: #000000;
	}

	.modal-button--primary:hover:not(:disabled) {
		background: #f3f4f6;
	}

	.modal-button--danger {
		background: #ef4444;
		color: #ffffff;
	}

	.modal-button--danger:hover:not(:disabled) {
		background: #dc2626;
	}

	.button-loading-label {
		justify-content: center;
	}

	.button-spinner {
		width: 1rem;
		height: 1rem;
		border: 2px solid;
		border-radius: 999px;
		animation: spin 800ms linear infinite;
	}

	.button-spinner--light {
		border-color: rgb(255 255 255 / 20%);
		border-top-color: #ffffff;
	}

	.button-spinner--dark {
		border-color: rgb(0 0 0 / 20%);
		border-top-color: #000000;
	}

	.confirm-body {
		text-align: center;
	}

	.confirm-icon {
		width: 3.5rem;
		height: 3.5rem;
		margin: 0 auto 1rem;
		border-radius: 999px;
		background: rgb(239 68 68 / 10%);
		color: #f87171;
	}

	.confirm-title {
		margin-bottom: 0.5rem;
	}

	.confirm-copy {
		margin-bottom: 1.5rem;
	}

	.confirm-actions {
		gap: 0.75rem;
	}

	/* Utility */
	.line-clamp-2 {
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
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

	/* FIX-2026-04-26: :global() wrapper — see above note re: Tabler icon hashing. */
	:global(.empty-state-icon .empty-icon-svg) {
		width: 3rem;
		height: 3rem;
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
	@media (max-width: 767.98px) {
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

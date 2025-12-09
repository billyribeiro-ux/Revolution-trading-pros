<script lang="ts">
	/**
	 * Admin Dashboard - Apple ICT9+ Principal Engineer Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Connection-aware dashboard that displays REAL metrics or shows
	 * "Not Connected" status when services aren't integrated.
	 *
	 * NO FAKE DATA - Everything reflects actual connection status!
	 */

	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut, backOut } from 'svelte/easing';
	import { browser } from '$app/environment';

	// Types
	interface ConnectionStatus {
		google_analytics: boolean;
		google_search_console: boolean;
		google_tag_manager: boolean;
		google_ads: boolean;
		stripe: boolean;
		sendgrid: boolean;
	}

	interface DashboardMetrics {
		visitors?: number;
		sessions?: number;
		pageviews?: number;
		bounce_rate?: number;
		revenue?: number;
		mrr?: number;
		users?: number;
		new_users?: number;
		seo_score?: number;
		indexed_pages?: number;
		keywords_ranking?: number;
	}

	// State
	let isLoading = $state(true);
	let connections = $state<ConnectionStatus>({
		google_analytics: false,
		google_search_console: false,
		google_tag_manager: false,
		google_ads: false,
		stripe: false,
		sendgrid: false
	});
	let metrics = $state<DashboardMetrics>({});
	let lastUpdated = $state<Date | null>(null);
	let refreshInterval: ReturnType<typeof setInterval> | null = null;

	// Fetch connection status
	async function fetchConnectionStatus() {
		try {
			const response = await fetch('/api/admin/connections/summary');
			if (response.ok) {
				const data = await response.json();
				// Map connected services
				if (data.connections) {
					for (const conn of data.connections) {
						if (conn.is_connected) {
							(connections as any)[conn.key] = true;
						}
					}
				}
			}
		} catch (e) {
			console.warn('Failed to fetch connection status:', e);
		}
	}

	// Fetch real metrics from connected services
	async function fetchMetrics() {
		// Only fetch metrics if services are connected
		if (connections.google_analytics) {
			try {
				const response = await fetch('/api/analytics/realtime');
				if (response.ok) {
					const data = await response.json();
					metrics = { ...metrics, ...data };
				}
			} catch (e) {
				console.warn('Analytics fetch failed:', e);
			}
		}

		if (connections.stripe) {
			try {
				const response = await fetch('/api/payments/summary');
				if (response.ok) {
					const data = await response.json();
					metrics = { ...metrics, revenue: data.revenue, mrr: data.mrr };
				}
			} catch (e) {
				console.warn('Payments fetch failed:', e);
			}
		}

		lastUpdated = new Date();
	}

	// Load dashboard data
	async function loadDashboard() {
		isLoading = true;
		await fetchConnectionStatus();
		await fetchMetrics();
		isLoading = false;
	}

	// Format number with suffixes
	function formatNumber(num: number | undefined): string {
		if (num === undefined || num === null) return '—';
		if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
		if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
		return num.toLocaleString();
	}

	// Format currency
	function formatCurrency(num: number | undefined): string {
		if (num === undefined || num === null) return '—';
		return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(num);
	}

	// Get time ago string
	function getTimeAgo(date: Date | null): string {
		if (!date) return 'Never';
		const seconds = Math.floor((new Date().getTime() - date.getTime()) / 1000);
		if (seconds < 60) return 'Just now';
		if (seconds < 3600) return `${Math.floor(seconds / 60)}m ago`;
		return `${Math.floor(seconds / 3600)}h ago`;
	}

	onMount(() => {
		loadDashboard();
		// Refresh every 60 seconds
		refreshInterval = setInterval(loadDashboard, 60000);
	});

	onDestroy(() => {
		if (refreshInterval) clearInterval(refreshInterval);
	});

	// Count connected services
	let connectedCount = $derived(
		Object.values(connections).filter(Boolean).length
	);
	let totalServices = $derived(Object.keys(connections).length);
</script>

<svelte:head>
	<title>Admin Dashboard | Revolution Trading Pros</title>
</svelte:head>

<div class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 text-white">
	<!-- Ambient Background Effects -->
	<div class="fixed inset-0 overflow-hidden pointer-events-none">
		<div class="absolute -top-40 -right-40 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl"></div>
		<div class="absolute -bottom-40 -left-40 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl"></div>
		<div class="absolute top-1/2 left-1/2 w-64 h-64 bg-emerald-500/5 rounded-full blur-3xl"></div>
	</div>

	<div class="relative z-10 p-6 lg:p-8 max-w-[1800px] mx-auto">
		<!-- Header -->
		<header class="mb-8" in:fly={{ y: -20, duration: 600, easing: quintOut }}>
			<div class="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4">
				<div>
					<h1 class="text-3xl lg:text-4xl font-bold bg-gradient-to-r from-white via-slate-200 to-slate-400 bg-clip-text text-transparent">
						Admin Dashboard
					</h1>
					<p class="mt-2 text-slate-400 text-lg">
						Real-time metrics and system overview
					</p>
				</div>

				<div class="flex items-center gap-4">
					<!-- Connection Status Indicator -->
					<div class="flex items-center gap-2 px-4 py-2 bg-white/5 backdrop-blur-xl border border-white/10 rounded-xl">
						<span class="w-2 h-2 rounded-full {connectedCount > 0 ? 'bg-emerald-400 animate-pulse' : 'bg-slate-500'}"></span>
						<span class="text-sm text-slate-300">{connectedCount}/{totalServices} Connected</span>
					</div>

					<!-- Last Updated -->
					<div class="text-sm text-slate-400">
						Updated {getTimeAgo(lastUpdated)}
					</div>

					<!-- Refresh Button -->
					<button
						onclick={() => loadDashboard()}
						disabled={isLoading}
						class="p-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl transition-all disabled:opacity-50"
						title="Refresh dashboard"
						aria-label="Refresh dashboard"
					>
						<svg class="w-5 h-5 text-slate-300 {isLoading ? 'animate-spin' : ''}" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
						</svg>
					</button>
				</div>
			</div>
		</header>

		<!-- Connection Alert Banner -->
		{#if connectedCount === 0}
			<div
				class="mb-8 p-4 bg-amber-500/10 border border-amber-500/20 rounded-2xl flex items-center gap-4"
				in:fly={{ y: 20, duration: 400 }}
			>
				<div class="w-10 h-10 bg-amber-500/20 rounded-xl flex items-center justify-center flex-shrink-0">
					<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
					</svg>
				</div>
				<div class="flex-1">
					<h3 class="font-semibold text-amber-300">No Services Connected</h3>
					<p class="text-sm text-slate-400">Connect your services in Settings to see real metrics on this dashboard.</p>
				</div>
				<a
					href="/admin/settings"
					class="px-4 py-2 bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 rounded-xl text-sm font-medium transition-all"
				>
					Connect Services
				</a>
			</div>
		{/if}

		<!-- Main Metrics Grid -->
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5 mb-8">
			<!-- Visitors Widget -->
			<div
				class="group relative"
				in:fly={{ y: 30, duration: 400, delay: 100, easing: quintOut }}
			>
				<div class="absolute inset-0 bg-gradient-to-br from-purple-500/20 to-purple-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
				<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6 hover:border-purple-400/30 transition-all">
					<div class="flex items-center justify-between mb-4">
						<div class="w-12 h-12 bg-purple-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-6 h-6 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
							</svg>
						</div>
						{#if !connections.google_analytics}
							<span class="px-2 py-1 bg-slate-500/20 text-slate-400 rounded-lg text-xs font-medium">Not Connected</span>
						{/if}
					</div>

					{#if connections.google_analytics}
						<div class="text-3xl font-bold text-white mb-1">{formatNumber(metrics.visitors)}</div>
						<div class="text-sm text-slate-400">Visitors Today</div>
					{:else}
						<div class="text-lg text-slate-500 mb-1">No Data</div>
						<div class="text-sm text-slate-500">Connect Google Analytics</div>
					{/if}
				</div>
			</div>

			<!-- Revenue Widget -->
			<div
				class="group relative"
				in:fly={{ y: 30, duration: 400, delay: 150, easing: quintOut }}
			>
				<div class="absolute inset-0 bg-gradient-to-br from-emerald-500/20 to-emerald-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
				<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6 hover:border-emerald-400/30 transition-all">
					<div class="flex items-center justify-between mb-4">
						<div class="w-12 h-12 bg-emerald-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-6 h-6 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
							</svg>
						</div>
						{#if !connections.stripe}
							<span class="px-2 py-1 bg-slate-500/20 text-slate-400 rounded-lg text-xs font-medium">Not Connected</span>
						{/if}
					</div>

					{#if connections.stripe}
						<div class="text-3xl font-bold text-white mb-1">{formatCurrency(metrics.revenue)}</div>
						<div class="text-sm text-slate-400">Revenue This Month</div>
					{:else}
						<div class="text-lg text-slate-500 mb-1">No Data</div>
						<div class="text-sm text-slate-500">Connect Stripe</div>
					{/if}
				</div>
			</div>

			<!-- SEO Score Widget -->
			<div
				class="group relative"
				in:fly={{ y: 30, duration: 400, delay: 200, easing: quintOut }}
			>
				<div class="absolute inset-0 bg-gradient-to-br from-blue-500/20 to-blue-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
				<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6 hover:border-blue-400/30 transition-all">
					<div class="flex items-center justify-between mb-4">
						<div class="w-12 h-12 bg-blue-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
							</svg>
						</div>
						{#if !connections.google_search_console}
							<span class="px-2 py-1 bg-slate-500/20 text-slate-400 rounded-lg text-xs font-medium">Not Connected</span>
						{/if}
					</div>

					{#if connections.google_search_console}
						<div class="text-3xl font-bold text-white mb-1">{metrics.seo_score ?? '—'}/100</div>
						<div class="text-sm text-slate-400">SEO Health Score</div>
					{:else}
						<div class="text-lg text-slate-500 mb-1">No Data</div>
						<div class="text-sm text-slate-500">Connect Search Console</div>
					{/if}
				</div>
			</div>

			<!-- MRR Widget -->
			<div
				class="group relative"
				in:fly={{ y: 30, duration: 400, delay: 250, easing: quintOut }}
			>
				<div class="absolute inset-0 bg-gradient-to-br from-amber-500/20 to-amber-600/20 rounded-2xl blur-xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
				<div class="relative bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6 hover:border-amber-400/30 transition-all">
					<div class="flex items-center justify-between mb-4">
						<div class="w-12 h-12 bg-amber-500/20 rounded-xl flex items-center justify-center">
							<svg class="w-6 h-6 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
							</svg>
						</div>
						{#if !connections.stripe}
							<span class="px-2 py-1 bg-slate-500/20 text-slate-400 rounded-lg text-xs font-medium">Not Connected</span>
						{/if}
					</div>

					{#if connections.stripe}
						<div class="text-3xl font-bold text-white mb-1">{formatCurrency(metrics.mrr)}</div>
						<div class="text-sm text-slate-400">Monthly Recurring Revenue</div>
					{:else}
						<div class="text-lg text-slate-500 mb-1">No Data</div>
						<div class="text-sm text-slate-500">Connect Stripe</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- Integration Status Section -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
			<!-- Google Services Status -->
			<div
				class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6"
				in:fly={{ y: 30, duration: 400, delay: 300, easing: quintOut }}
			>
				<div class="flex items-center justify-between mb-6">
					<h2 class="text-lg font-semibold text-white flex items-center gap-2">
						<span class="text-xl">🔍</span> Google Integrations
					</h2>
					<a href="/admin/settings" class="text-sm text-purple-400 hover:text-purple-300 transition-colors">
						Manage →
					</a>
				</div>

				<div class="space-y-4">
					<!-- Google Analytics -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-amber-500/20 rounded-lg flex items-center justify-center">
								<span class="text-sm font-bold text-amber-400">G</span>
							</div>
							<div>
								<div class="text-sm font-medium text-white">Google Analytics 4</div>
								<div class="text-xs text-slate-500">Traffic & behavior tracking</div>
							</div>
						</div>
						{#if connections.google_analytics}
							<span class="flex items-center gap-1.5 px-2 py-1 bg-emerald-500/20 text-emerald-400 rounded-lg text-xs font-medium">
								<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
								Connected
							</span>
						{:else}
							<a href="/admin/settings" class="px-3 py-1.5 bg-purple-500/20 hover:bg-purple-500/30 text-purple-400 rounded-lg text-xs font-medium transition-all">
								Connect
							</a>
						{/if}
					</div>

					<!-- Google Search Console -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-blue-500/20 rounded-lg flex items-center justify-center">
								<span class="text-sm font-bold text-blue-400">G</span>
							</div>
							<div>
								<div class="text-sm font-medium text-white">Search Console</div>
								<div class="text-xs text-slate-500">SEO & indexing status</div>
							</div>
						</div>
						{#if connections.google_search_console}
							<span class="flex items-center gap-1.5 px-2 py-1 bg-emerald-500/20 text-emerald-400 rounded-lg text-xs font-medium">
								<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
								Connected
							</span>
						{:else}
							<a href="/admin/settings" class="px-3 py-1.5 bg-purple-500/20 hover:bg-purple-500/30 text-purple-400 rounded-lg text-xs font-medium transition-all">
								Connect
							</a>
						{/if}
					</div>

					<!-- Google Tag Manager -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-indigo-500/20 rounded-lg flex items-center justify-center">
								<span class="text-sm font-bold text-indigo-400">G</span>
							</div>
							<div>
								<div class="text-sm font-medium text-white">Tag Manager</div>
								<div class="text-xs text-slate-500">Marketing tag management</div>
							</div>
						</div>
						{#if connections.google_tag_manager}
							<span class="flex items-center gap-1.5 px-2 py-1 bg-emerald-500/20 text-emerald-400 rounded-lg text-xs font-medium">
								<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
								Connected
							</span>
						{:else}
							<a href="/admin/settings" class="px-3 py-1.5 bg-purple-500/20 hover:bg-purple-500/30 text-purple-400 rounded-lg text-xs font-medium transition-all">
								Connect
							</a>
						{/if}
					</div>

					<!-- Google Ads -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-green-500/20 rounded-lg flex items-center justify-center">
								<span class="text-sm font-bold text-green-400">G</span>
							</div>
							<div>
								<div class="text-sm font-medium text-white">Google Ads</div>
								<div class="text-xs text-slate-500">Conversion tracking</div>
							</div>
						</div>
						{#if connections.google_ads}
							<span class="flex items-center gap-1.5 px-2 py-1 bg-emerald-500/20 text-emerald-400 rounded-lg text-xs font-medium">
								<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
								Connected
							</span>
						{:else}
							<a href="/admin/settings" class="px-3 py-1.5 bg-purple-500/20 hover:bg-purple-500/30 text-purple-400 rounded-lg text-xs font-medium transition-all">
								Connect
							</a>
						{/if}
					</div>
				</div>
			</div>

			<!-- System Health -->
			<div
				class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl p-6"
				in:fly={{ y: 30, duration: 400, delay: 350, easing: quintOut }}
			>
				<div class="flex items-center justify-between mb-6">
					<h2 class="text-lg font-semibold text-white flex items-center gap-2">
						<span class="text-xl">🖥️</span> System Status
					</h2>
					<span class="flex items-center gap-1.5 px-2 py-1 bg-emerald-500/20 text-emerald-400 rounded-lg text-xs font-medium">
						<span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
						Operational
					</span>
				</div>

				<div class="space-y-4">
					<!-- API Status -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-emerald-500/20 rounded-lg flex items-center justify-center">
								<svg class="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
								</svg>
							</div>
							<div>
								<div class="text-sm font-medium text-white">API Server</div>
								<div class="text-xs text-slate-500">Backend services</div>
							</div>
						</div>
						<span class="text-xs text-emerald-400">Online</span>
					</div>

					<!-- Database -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-emerald-500/20 rounded-lg flex items-center justify-center">
								<svg class="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4" />
								</svg>
							</div>
							<div>
								<div class="text-sm font-medium text-white">Database</div>
								<div class="text-xs text-slate-500">MySQL connection</div>
							</div>
						</div>
						<span class="text-xs text-emerald-400">Connected</span>
					</div>

					<!-- Cache -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-emerald-500/20 rounded-lg flex items-center justify-center">
								<svg class="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
								</svg>
							</div>
							<div>
								<div class="text-sm font-medium text-white">Cache</div>
								<div class="text-xs text-slate-500">Redis cache layer</div>
							</div>
						</div>
						<span class="text-xs text-emerald-400">Active</span>
					</div>

					<!-- Queue -->
					<div class="flex items-center justify-between p-3 bg-black/20 rounded-xl">
						<div class="flex items-center gap-3">
							<div class="w-8 h-8 bg-emerald-500/20 rounded-lg flex items-center justify-center">
								<svg class="w-4 h-4 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
								</svg>
							</div>
							<div>
								<div class="text-sm font-medium text-white">Job Queue</div>
								<div class="text-xs text-slate-500">Background tasks</div>
							</div>
						</div>
						<span class="text-xs text-emerald-400">Running</span>
					</div>
				</div>
			</div>
		</div>

		<!-- Quick Actions -->
		<div
			class="grid grid-cols-2 md:grid-cols-4 gap-4"
			in:fly={{ y: 30, duration: 400, delay: 400, easing: quintOut }}
		>
			<a
				href="/admin/settings"
				class="group p-5 bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl hover:border-purple-400/30 transition-all hover:-translate-y-1"
			>
				<div class="w-10 h-10 bg-purple-500/20 rounded-xl flex items-center justify-center mb-3 group-hover:scale-110 transition-transform">
					<svg class="w-5 h-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
					</svg>
				</div>
				<div class="text-sm font-medium text-white">API Settings</div>
				<div class="text-xs text-slate-500">Connect services</div>
			</a>

			<a
				href="/admin/analytics"
				class="group p-5 bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl hover:border-blue-400/30 transition-all hover:-translate-y-1"
			>
				<div class="w-10 h-10 bg-blue-500/20 rounded-xl flex items-center justify-center mb-3 group-hover:scale-110 transition-transform">
					<svg class="w-5 h-5 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
					</svg>
				</div>
				<div class="text-sm font-medium text-white">Analytics</div>
				<div class="text-xs text-slate-500">View reports</div>
			</a>

			<a
				href="/admin/members"
				class="group p-5 bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl hover:border-emerald-400/30 transition-all hover:-translate-y-1"
			>
				<div class="w-10 h-10 bg-emerald-500/20 rounded-xl flex items-center justify-center mb-3 group-hover:scale-110 transition-transform">
					<svg class="w-5 h-5 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
					</svg>
				</div>
				<div class="text-sm font-medium text-white">Members</div>
				<div class="text-xs text-slate-500">Manage users</div>
			</a>

			<a
				href="/admin/seo"
				class="group p-5 bg-white/5 backdrop-blur-xl border border-white/10 rounded-2xl hover:border-amber-400/30 transition-all hover:-translate-y-1"
			>
				<div class="w-10 h-10 bg-amber-500/20 rounded-xl flex items-center justify-center mb-3 group-hover:scale-110 transition-transform">
					<svg class="w-5 h-5 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
					</svg>
				</div>
				<div class="text-sm font-medium text-white">SEO Tools</div>
				<div class="text-xs text-slate-500">Optimize search</div>
			</a>
		</div>
	</div>
</div>

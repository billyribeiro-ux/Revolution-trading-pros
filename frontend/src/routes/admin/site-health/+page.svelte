<script lang="ts">
	/**
	 * Site Health Dashboard - Apple ICT9+ Enterprise Grade
	 *
	 * Comprehensive site health monitoring with:
	 * - Real-time system status checks
	 * - Security scan results
	 * - Performance monitoring
	 * - Database health metrics
	 * - API connection health
	 * - Apple-level polish and animations
	 *
	 */

	import { fade, fly, scale } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { adminFetch } from '#lib/utils/adminFetch.js';
	import IconHeartbeat from '@tabler/icons-svelte-runes/icons/heartbeat';
	import IconShieldCheck from '@tabler/icons-svelte-runes/icons/shield-check';
	import IconDatabase from '@tabler/icons-svelte-runes/icons/database';
	import IconServer from '@tabler/icons-svelte-runes/icons/server';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconPlugConnected from '@tabler/icons-svelte-runes/icons/plug-connected';
	import IconPlugConnectedX from '@tabler/icons-svelte-runes/icons/plug-connected-x';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconCircleCheck from '@tabler/icons-svelte-runes/icons/circle-check';
	import IconCircleX from '@tabler/icons-svelte-runes/icons/circle-x';
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';
	import IconSettings from '@tabler/icons-svelte-runes/icons/settings';
	import IconApiApp from '@tabler/icons-svelte-runes/icons/api-app';
	import IconCertificate from '@tabler/icons-svelte-runes/icons/certificate';
	import IconFolder from '@tabler/icons-svelte-runes/icons/folder';
	import IconCloud from '@tabler/icons-svelte-runes/icons/cloud';
	import IconCode from '@tabler/icons-svelte-runes/icons/code';
	import IconBug from '@tabler/icons-svelte-runes/icons/bug';
	import IconRocket from '@tabler/icons-svelte-runes/icons/rocket';
	import IconChartLine from '@tabler/icons-svelte-runes/icons/chart-line';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import IconExternalLink from '@tabler/icons-svelte-runes/icons/external-link';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import {
		connections,
		getConnectedCount,
		getOverallHealth,
		getServicesWithErrors
	} from '#lib/stores/connections.svelte.js';
	import { toastStore } from '#lib/stores/toast.svelte.js';

	// Types

	interface HealthCheck {
		id: string;
		name: string;
		description: string;
		status: 'good' | 'warning' | 'critical' | 'unknown' | 'checking';
		message: string;
		category: 'performance' | 'security' | 'database' | 'server' | 'api';
		lastChecked: Date | null;
		action?: {
			label: string;
			href?: string;
			onClick?: () => void;
		};
	}

	interface SiteHealthData {
		overallScore: number;
		checks: HealthCheck[];
		performance: {
			responseTime: number | null;
			memoryUsage: number | null;
			cpuUsage: number | null;
			diskUsage: number | null;
		};
		security: {
			sslValid: boolean | null;
			sslExpiry: Date | null;
			headersScore: number | null;
			vulnerabilities: number;
		};
		database: {
			connected: boolean;
			responseTime: number | null;
			size: string | null;
			tables: number | null;
		};
		server: {
			phpVersion: string | null;
			webServer: string | null;
			os: string | null;
			uptime: string | null;
		};
	}

	// State

	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let isRunningTests = $state(false);
	let healthData = $state<SiteHealthData | null>(null);
	let lastUpdated = $state<Date | null>(null);
	let activeTab = $state<'overview' | 'performance' | 'security' | 'database' | 'server'>(
		'overview'
	);

	// Local derived from getters
	const connectedCount = $derived(getConnectedCount());
	const overallHealth = $derived(getOverallHealth());
	const servicesWithErrors = $derived(getServicesWithErrors());

	// Animated score display
	const scoreDisplay = tweened(0, { duration: 1500, easing: cubicOut });

	// Lifecycle

	onMount(() => {
		let cancelled = false;

		const init = async () => {
			try {
				await connections.load();
				connections.startAutoRefresh();
				await loadHealthData();
			} catch (error) {
				console.error('Failed to initialize site health dashboard:', error);
				toastStore.error('Failed to initialize site health dashboard');
			} finally {
				if (!cancelled) {
					isLoading = false;
				}
			}
		};

		void init();

		return () => {
			cancelled = true;
			connections.stopAutoRefresh();
		};
	});

	// Data Loading

	async function loadHealthData() {
		try {
			const data = await adminFetch('/api/admin/site-health');
			healthData = transformHealthData(data);
			lastUpdated = new Date();
			scoreDisplay.set(healthData.overallScore);
		} catch (error) {
			console.error('Failed to load health data:', error);
			healthData = getDefaultHealthData();
			scoreDisplay.set(0);
		}
	}

	function transformHealthData(apiData: Record<string, unknown>): SiteHealthData {
		// Transform API response to our format
		return {
			overallScore: (apiData['overall_score'] as number) ?? 0,
			checks: ((apiData['checks'] as HealthCheck[]) ?? []).map((check) => ({
				...check,
				lastChecked: check.lastChecked ? new Date(check.lastChecked) : null
			})),
			performance: {
				responseTime:
					((apiData['performance'] as Record<string, unknown>)?.['response_time'] as
						number | null) ?? null,
				memoryUsage:
					((apiData['performance'] as Record<string, unknown>)?.['memory_usage'] as
						number | null) ?? null,
				cpuUsage:
					((apiData['performance'] as Record<string, unknown>)?.['cpu_usage'] as number | null) ??
					null,
				diskUsage:
					((apiData['performance'] as Record<string, unknown>)?.['disk_usage'] as number | null) ??
					null
			},
			security: {
				sslValid:
					((apiData['security'] as Record<string, unknown>)?.['ssl_valid'] as boolean | null) ??
					null,
				sslExpiry: (apiData['security'] as Record<string, unknown>)?.['ssl_expiry']
					? new Date((apiData['security'] as Record<string, unknown>)['ssl_expiry'] as string)
					: null,
				headersScore:
					((apiData['security'] as Record<string, unknown>)?.['headers_score'] as number | null) ??
					null,
				vulnerabilities:
					((apiData['security'] as Record<string, unknown>)?.['vulnerabilities'] as number) ?? 0
			},
			database: {
				connected:
					((apiData['database'] as Record<string, unknown>)?.['connected'] as boolean) ?? false,
				responseTime:
					((apiData['database'] as Record<string, unknown>)?.['response_time'] as number | null) ??
					null,
				size: ((apiData['database'] as Record<string, unknown>)?.['size'] as string | null) ?? null,
				tables:
					((apiData['database'] as Record<string, unknown>)?.['tables'] as number | null) ?? null
			},
			server: {
				phpVersion:
					((apiData['server'] as Record<string, unknown>)?.['php_version'] as string | null) ??
					null,
				webServer:
					((apiData['server'] as Record<string, unknown>)?.['web_server'] as string | null) ?? null,
				os: ((apiData['server'] as Record<string, unknown>)?.['os'] as string | null) ?? null,
				uptime:
					((apiData['server'] as Record<string, unknown>)?.['uptime'] as string | null) ?? null
			}
		};
	}

	function getDefaultHealthData(): SiteHealthData {
		return {
			overallScore: 0,
			checks: [],
			performance: {
				responseTime: null,
				memoryUsage: null,
				cpuUsage: null,
				diskUsage: null
			},
			security: {
				sslValid: null,
				sslExpiry: null,
				headersScore: null,
				vulnerabilities: 0
			},
			database: {
				connected: false,
				responseTime: null,
				size: null,
				tables: null
			},
			server: {
				phpVersion: null,
				webServer: null,
				os: null,
				uptime: null
			}
		};
	}

	async function runHealthTests() {
		isRunningTests = true;
		toastStore.info('Running site health tests...');

		try {
			const data = await adminFetch('/api/admin/site-health/run-tests', { method: 'POST' });
			healthData = transformHealthData(data);
			lastUpdated = new Date();
			scoreDisplay.set(healthData.overallScore);
			toastStore.success('Health tests completed');
		} catch (error) {
			console.error('Failed to run health tests:', error);
			toastStore.error('Failed to run health tests');
		} finally {
			isRunningTests = false;
		}
	}

	async function handleRefresh() {
		isRefreshing = true;
		try {
			await loadHealthData();
			toastStore.success('Health data refreshed');
		} finally {
			isRefreshing = false;
		}
	}

	// Helpers

	function getScoreColor(score: number): string {
		if (score >= 90) return '#10b981';
		if (score >= 70) return '#f59e0b';
		if (score >= 50) return '#f97316';
		return '#ef4444';
	}

	function getScoreLabel(score: number): string {
		if (score >= 90) return 'Excellent';
		if (score >= 70) return 'Good';
		if (score >= 50) return 'Needs Attention';
		return 'Critical';
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'good':
				return IconCircleCheck;
			case 'warning':
				return IconAlertTriangle;
			case 'critical':
				return IconCircleX;
			default:
				return IconInfoCircle;
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'good':
				return '#10b981';
			case 'warning':
				return '#f59e0b';
			case 'critical':
				return '#ef4444';
			default:
				return '#64748b';
		}
	}

	function formatMs(ms: number): string {
		if (ms < 1000) return `${Math.round(ms)}ms`;
		return `${(ms / 1000).toFixed(2)}s`;
	}

	const tabs = [
		{ id: 'overview', label: 'Overview', icon: IconHeartbeat },
		{ id: 'performance', label: 'Performance', icon: IconRocket },
		{ id: 'security', label: 'Security', icon: IconShieldCheck },
		{ id: 'database', label: 'Database', icon: IconDatabase },
		{ id: 'server', label: 'Server', icon: IconServer }
	] as const;

	function getConnectionIndicatorClass(healthy: boolean): Record<string, boolean> {
		return {
			'connection-indicator': true,
			healthy
		};
	}

	function getIconWrapperClass(spinning: boolean): Record<string, boolean> {
		return {
			'icon-wrapper': true,
			spinning
		};
	}

	function getTabClass(tabId: (typeof tabs)[number]['id']): Record<string, boolean> {
		return {
			tab: true,
			active: activeTab === tabId
		};
	}

	function getMetricValueClass(options: {
		good?: boolean;
		bad?: boolean;
		large?: boolean;
	}): Record<string, boolean> {
		return {
			'metric-value': true,
			large: options.large ?? false,
			good: options.good ?? false,
			bad: options.bad ?? false
		};
	}

	function getApiHealthCardClass(hasErrors: boolean): Record<string, boolean> {
		return {
			'api-health-card': true,
			'has-errors': hasErrors
		};
	}

	function getCheckItemClass(status: HealthCheck['status']): Record<string, boolean> {
		return {
			'check-item': true,
			good: status === 'good',
			warning: status === 'warning',
			critical: status === 'critical'
		};
	}

	function getMetricCardClass(modifiers: Record<string, boolean> = {}): Record<string, boolean> {
		return {
			'metric-card': true,
			...modifiers
		};
	}

	function getEmptyStateClass(error: boolean): Record<string, boolean> {
		return {
			'empty-state': true,
			error
		};
	}

	function getPercentWidth(value: number | null | undefined): string {
		return `${Math.max(0, Math.min(100, value ?? 0))}%`;
	}

	function getScoreDashOffset(score: number): number {
		return 339.3 - (339.3 * Math.max(0, Math.min(100, score))) / 100;
	}
</script>

<svelte:head>
	<title>Site Health | Admin</title>
</svelte:head>

<div class="admin-site-health">
	

	<div class="admin-page-container">
		<!-- Header Section - Centered -->
		<header class="page-header" in:fly={{ y: -20, duration: 500, delay: 100 }}>
			<h1>Site Health</h1>
			<p class="subtitle">Monitor your site's performance, security, and overall health</p>

			<div class="header-actions">
				<!-- API Connections Status -->
				<div class="connections-status">
					<div class={getConnectionIndicatorClass(connectedCount > 0)}>
						{#if connectedCount > 0}
							<IconPlugConnected size={14} />
						{:else}
							<IconPlugConnectedX size={14} />
						{/if}
						<span>{connectedCount} APIs Connected</span>
					</div>
					{#if servicesWithErrors.length > 0}
						<div class="error-indicator">
							<IconAlertTriangle size={14} />
							<span>{servicesWithErrors.length} with issues</span>
						</div>
					{/if}
				</div>

				<button class="btn-primary" onclick={runHealthTests} disabled={isRunningTests}>
					<span class={getIconWrapperClass(isRunningTests)}>
						<IconPlayerPlay size={18} />
					</span>
					<span>{isRunningTests ? 'Running...' : 'Run Tests'}</span>
				</button>

				<button class="btn-secondary" onclick={handleRefresh} disabled={isRefreshing}>
					<span class={getIconWrapperClass(isRefreshing)}>
						<IconRefresh size={18} />
					</span>
					<span>Refresh</span>
				</button>
			</div>
		</header>

		{#if isLoading}
			<!-- Loading State -->
			<div class="loading-state" in:fade>
				<div class="loading-spinner"></div>
				<p>Checking site health...</p>
			</div>
		{:else}
			<!-- Health Score Card -->
			<div class="score-section" in:scale={{ duration: 500, start: 0.9, delay: 100 }}>
				<div class="score-card">
					<div class="score-ring" style:--score-color={getScoreColor($scoreDisplay)}>
						<svg aria-hidden="true" viewBox="0 0 120 120">
							<circle class="ring-bg" cx="60" cy="60" r="54" />
							<circle
								class="ring-progress"
								cx="60"
								cy="60"
								r="54"
								style:stroke-dashoffset={getScoreDashOffset($scoreDisplay)}
							/>
						</svg>
						<div class="score-value">
							<span class="score-number">{Math.round($scoreDisplay)}</span>
							<span class="score-unit">%</span>
						</div>
					</div>
					<div class="score-info">
						<h2 class="score-label" style:color={getScoreColor($scoreDisplay)}>
							{getScoreLabel(Math.round($scoreDisplay))}
						</h2>
						<p class="score-description">
							{#if healthData && healthData.overallScore >= 90}
								Your site is in great shape! All systems are functioning optimally.
							{:else if healthData && healthData.overallScore >= 70}
								Your site is healthy with minor areas for improvement.
							{:else if healthData && healthData.overallScore >= 50}
								Several issues need attention to improve site health.
							{:else if healthData && healthData.overallScore > 0}
								Critical issues detected. Immediate action recommended.
							{:else}
								Run health tests to check your site's status.
							{/if}
						</p>
					</div>
				</div>

				<!-- Quick Stats -->
				<div class="quick-stats">
					{#if healthData}
						<div class="stat-item good">
							<IconCircleCheck size={20} />
							<span class="stat-value"
								>{healthData.checks.filter((c) => c.status === 'good').length}</span
							>
							<span class="stat-label">Passed</span>
						</div>
						<div class="stat-item warning">
							<IconAlertTriangle size={20} />
							<span class="stat-value"
								>{healthData.checks.filter((c) => c.status === 'warning').length}</span
							>
							<span class="stat-label">Warnings</span>
						</div>
						<div class="stat-item critical">
							<IconCircleX size={20} />
							<span class="stat-value"
								>{healthData.checks.filter((c) => c.status === 'critical').length}</span
							>
							<span class="stat-label">Critical</span>
						</div>
					{:else}
						<div class="stat-item unknown">
							<IconInfoCircle size={20} />
							<span class="stat-value">—</span>
							<span class="stat-label">No data</span>
						</div>
					{/if}
				</div>
			</div>

			<!-- Tabs -->
			<div class="tabs-section" in:fly={{ y: 20, duration: 500, delay: 200 }}>
				<div class="tabs">
					{#each tabs as tab (tab.id)}
						{@const TabIcon = tab.icon}
						<button class={getTabClass(tab.id)} onclick={() => (activeTab = tab.id)}>
							<TabIcon size={18} />
							<span>{tab.label}</span>
						</button>
					{/each}
				</div>
			</div>

			<!-- Tab Content -->
			<div class="tab-content" in:fade={{ duration: 200 }}>
				{#if activeTab === 'overview'}
					<!-- Overview Tab -->
					<div class="overview-grid">
						<!-- Performance Overview -->
						<div class="overview-card performance">
							<div class="card-header">
								<div class="card-icon">
									<IconRocket size={24} />
								</div>
								<div class="card-title">
									<h3>Performance</h3>
									<p>Response times and resource usage</p>
								</div>
							</div>
							<div class="card-metrics">
								{#if healthData?.performance.responseTime !== null}
									<div class="metric">
										<span class="metric-label">Response Time</span>
										<span class="metric-value"
											>{formatMs(healthData?.performance.responseTime ?? 0)}</span
										>
									</div>
								{/if}
								{#if healthData?.performance.memoryUsage !== null}
									<div class="metric">
										<span class="metric-label">Memory Usage</span>
										<span class="metric-value">{healthData?.performance.memoryUsage ?? 0}%</span>
									</div>
								{/if}
								{#if healthData?.performance.responseTime === null && healthData?.performance.memoryUsage === null}
									<p class="no-data">Run tests to see performance metrics</p>
								{/if}
							</div>
							<button class="card-action" onclick={() => (activeTab = 'performance')}>
								<span>View Details</span>
								<IconArrowRight size={16} />
							</button>
						</div>

						<!-- Security Overview -->
						<div class="overview-card security">
							<div class="card-header">
								<div class="card-icon">
									<IconShieldCheck size={24} />
								</div>
								<div class="card-title">
									<h3>Security</h3>
									<p>SSL, headers, and vulnerabilities</p>
								</div>
							</div>
							<div class="card-metrics">
								{#if healthData?.security.sslValid !== null}
									<div class="metric">
										<span class="metric-label">SSL Certificate</span>
										<span
											class={getMetricValueClass({
												good: healthData?.security.sslValid === true,
												bad: healthData?.security.sslValid === false
											})}
										>
											{healthData?.security.sslValid ? 'Valid' : 'Invalid'}
										</span>
									</div>
								{/if}
								{#if healthData?.security.vulnerabilities !== undefined}
									<div class="metric">
										<span class="metric-label">Vulnerabilities</span>
										<span
											class={getMetricValueClass({
												good: healthData.security.vulnerabilities === 0,
												bad: healthData.security.vulnerabilities > 0
											})}
										>
											{healthData.security.vulnerabilities}
										</span>
									</div>
								{/if}
								{#if healthData?.security.sslValid === null}
									<p class="no-data">Run tests to check security status</p>
								{/if}
							</div>
							<button class="card-action" onclick={() => (activeTab = 'security')}>
								<span>View Details</span>
								<IconArrowRight size={16} />
							</button>
						</div>

						<!-- Database Overview -->
						<div class="overview-card database">
							<div class="card-header">
								<div class="card-icon">
									<IconDatabase size={24} />
								</div>
								<div class="card-title">
									<h3>Database</h3>
									<p>Connection and query performance</p>
								</div>
							</div>
							<div class="card-metrics">
								{#if healthData?.database.connected !== undefined}
									<div class="metric">
										<span class="metric-label">Status</span>
										<span
											class={getMetricValueClass({
												good: healthData.database.connected,
												bad: !healthData.database.connected
											})}
										>
											{healthData.database.connected ? 'Connected' : 'Disconnected'}
										</span>
									</div>
								{/if}
								{#if healthData?.database.size}
									<div class="metric">
										<span class="metric-label">Database Size</span>
										<span class="metric-value">{healthData.database.size}</span>
									</div>
								{/if}
								{#if !healthData?.database.connected && healthData?.database.size === null}
									<p class="no-data">Run tests to check database health</p>
								{/if}
							</div>
							<button class="card-action" onclick={() => (activeTab = 'database')}>
								<span>View Details</span>
								<IconArrowRight size={16} />
							</button>
						</div>

						<!-- Server Overview -->
						<div class="overview-card server">
							<div class="card-header">
								<div class="card-icon">
									<IconServer size={24} />
								</div>
								<div class="card-title">
									<h3>Server</h3>
									<p>PHP, web server, and environment</p>
								</div>
							</div>
							<div class="card-metrics">
								{#if healthData?.server.phpVersion}
									<div class="metric">
										<span class="metric-label">PHP Version</span>
										<span class="metric-value">{healthData.server.phpVersion}</span>
									</div>
								{/if}
								{#if healthData?.server.webServer}
									<div class="metric">
										<span class="metric-label">Web Server</span>
										<span class="metric-value">{healthData.server.webServer}</span>
									</div>
								{/if}
								{#if !healthData?.server.phpVersion && !healthData?.server.webServer}
									<p class="no-data">Run tests to see server info</p>
								{/if}
							</div>
							<button class="card-action" onclick={() => (activeTab = 'server')}>
								<span>View Details</span>
								<IconArrowRight size={16} />
							</button>
						</div>
					</div>

					<!-- API Connections Health -->
					<div class="api-health-section" in:fly={{ y: 20, duration: 500, delay: 300 }}>
						<h3>API Connections Health</h3>
						<div class="api-health-grid">
							<div class="api-health-card">
								<div class="api-health-header">
									<IconApiApp size={20} />
									<span>Connected APIs</span>
								</div>
								<div class="api-health-value">{connectedCount}</div>
								<button class="api-health-action" onclick={() => goto('/admin/connections')}>
									Manage Connections
								</button>
							</div>
							<div class={getApiHealthCardClass(servicesWithErrors.length > 0)}>
								<div class="api-health-header">
									<IconAlertTriangle size={20} />
									<span>Connection Issues</span>
								</div>
								<div class="api-health-value">{servicesWithErrors.length}</div>
								{#if servicesWithErrors.length > 0}
									<button
										class="api-health-action error"
										onclick={() => goto('/admin/connections')}
									>
										View Issues
									</button>
								{:else}
									<span class="api-health-status">All connections healthy</span>
								{/if}
							</div>
							<div class="api-health-card">
								<div class="api-health-header">
									<IconHeartbeat size={20} />
									<span>Overall API Health</span>
								</div>
								<div class="api-health-value">{overallHealth}%</div>
								<div class="api-health-bar">
									<div
										class="api-health-progress"
										style:width={getPercentWidth(overallHealth)}
										style:background={getScoreColor(overallHealth)}
									></div>
								</div>
							</div>
						</div>
					</div>

					<!-- Health Checks List -->
					{#if healthData && healthData.checks.length > 0}
						<div class="checks-section" in:fly={{ y: 20, duration: 500, delay: 400 }}>
							<h3>All Health Checks</h3>
							<div class="checks-list">
								{#each healthData.checks as check, i (check.id)}
									{@const StatusIcon = getStatusIcon(check.status)}
									<div
										class={getCheckItemClass(check.status)}
										in:fly={{ y: 10, duration: 300, delay: 50 * i }}
									>
										<div class="check-icon" style:color={getStatusColor(check.status)}>
											<StatusIcon size={20} />
										</div>
										<div class="check-info">
											<h4>{check.name}</h4>
											<p>{check.message}</p>
										</div>
										{#if check.action}
											{#if check.action.href}
												<a href={check.action.href} class="check-action">
													{check.action.label}
													<IconExternalLink size={14} />
												</a>
											{:else if check.action.onClick}
												<button class="check-action" onclick={check.action.onClick}>
													{check.action.label}
												</button>
											{/if}
										{/if}
									</div>
								{/each}
							</div>
						</div>
					{/if}
				{:else if activeTab === 'performance'}
					<!-- Performance Tab -->
					<div class="detail-section">
						<h3>Performance Metrics</h3>
						{#if healthData?.performance.responseTime !== null || healthData?.performance.memoryUsage !== null}
							<div class="metrics-grid">
								{#if healthData?.performance.responseTime !== null}
									<div class="metric-card">
										<div class="metric-icon">
											<IconClock size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Response Time</span>
											<span class="metric-value large"
												>{formatMs(healthData?.performance.responseTime ?? 0)}</span
											>
										</div>
									</div>
								{/if}
								{#if healthData?.performance.memoryUsage !== null}
									<div class="metric-card">
										<div class="metric-icon">
											<IconServer size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Memory Usage</span>
											<span class="metric-value large"
												>{healthData?.performance.memoryUsage ?? 0}%</span
											>
											<div class="metric-bar">
												<div
													class="metric-progress"
													style:width={getPercentWidth(healthData?.performance.memoryUsage)}
												></div>
											</div>
										</div>
									</div>
								{/if}
								{#if healthData?.performance.cpuUsage !== null}
									<div class="metric-card">
										<div class="metric-icon">
											<IconChartLine size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">CPU Usage</span>
											<span class="metric-value large"
												>{healthData?.performance.cpuUsage ?? 0}%</span
											>
											<div class="metric-bar">
												<div
													class="metric-progress"
													style:width={getPercentWidth(healthData?.performance.cpuUsage)}
												></div>
											</div>
										</div>
									</div>
								{/if}
								{#if healthData?.performance.diskUsage !== null}
									<div class="metric-card">
										<div class="metric-icon">
											<IconFolder size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Disk Usage</span>
											<span class="metric-value large"
												>{healthData?.performance.diskUsage ?? 0}%</span
											>
											<div class="metric-bar">
												<div
													class="metric-progress"
													style:width={getPercentWidth(healthData?.performance.diskUsage)}
												></div>
											</div>
										</div>
									</div>
								{/if}
							</div>
						{:else}
							<div class="empty-state">
								<IconRocket size={48} />
								<h4>No Performance Data</h4>
								<p>Run health tests to collect performance metrics</p>
								<button class="run-tests-btn" onclick={runHealthTests} disabled={isRunningTests}>
									<IconPlayerPlay size={18} />
									<span>Run Tests</span>
								</button>
							</div>
						{/if}
					</div>
				{:else if activeTab === 'security'}
					<!-- Security Tab -->
					<div class="detail-section">
						<h3>Security Status</h3>
						{#if healthData?.security.sslValid !== null}
							<div class="metrics-grid">
								<div
									class={getMetricCardClass({
										ssl: true,
										valid: healthData?.security.sslValid === true,
										invalid: healthData?.security.sslValid === false
									})}
								>
									<div class="metric-icon">
										<IconCertificate size={24} />
									</div>
									<div class="metric-content">
										<span class="metric-label">SSL Certificate</span>
										<span class="metric-value large"
											>{healthData?.security.sslValid ? 'Valid' : 'Invalid'}</span
										>
										{#if healthData?.security.sslExpiry}
											<span class="metric-detail"
												>Expires: {healthData?.security.sslExpiry.toLocaleDateString()}</span
											>
										{/if}
									</div>
									<div class="metric-status">
										{#if healthData?.security.sslValid}
											<IconCheck size={24} />
										{:else}
											<IconX size={24} />
										{/if}
									</div>
								</div>
								{#if healthData?.security.headersScore !== null}
									<div class="metric-card">
										<div class="metric-icon">
											<IconShieldCheck size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Security Headers</span>
											<span class="metric-value large"
												>{healthData?.security.headersScore ?? 0}%</span
											>
											<div class="metric-bar">
												<div
													class="metric-progress"
													style:width={getPercentWidth(healthData?.security.headersScore)}
													style:background={getScoreColor(healthData?.security.headersScore ?? 0)}
												></div>
											</div>
										</div>
									</div>
								{/if}
								<div
									class={getMetricCardClass({
										critical: (healthData?.security.vulnerabilities ?? 0) > 0
									})}
								>
									<div class="metric-icon">
										<IconBug size={24} />
									</div>
									<div class="metric-content">
										<span class="metric-label">Vulnerabilities</span>
										<span
											class={getMetricValueClass({
												large: true,
												good: (healthData?.security.vulnerabilities ?? 0) === 0
											})}>{healthData?.security.vulnerabilities ?? 0}</span
										>
									</div>
									{#if (healthData?.security.vulnerabilities ?? 0) === 0}
										<div class="metric-status good">
											<IconCheck size={24} />
										</div>
									{/if}
								</div>
							</div>
						{:else}
							<div class="empty-state">
								<IconShieldCheck size={48} />
								<h4>No Security Data</h4>
								<p>Run health tests to check security status</p>
								<button class="run-tests-btn" onclick={runHealthTests} disabled={isRunningTests}>
									<IconPlayerPlay size={18} />
									<span>Run Tests</span>
								</button>
							</div>
						{/if}
					</div>
				{:else if activeTab === 'database'}
					<!-- Database Tab -->
					<div class="detail-section">
						<h3>Database Health</h3>
						{#if healthData?.database.connected}
							<div class="metrics-grid">
								<div
									class={getMetricCardClass({
										connection: true,
										connected: healthData.database.connected
									})}
								>
									<div class="metric-icon">
										<IconDatabase size={24} />
									</div>
									<div class="metric-content">
										<span class="metric-label">Connection Status</span>
										<span class="metric-value large">Connected</span>
									</div>
									<div class="metric-status good">
										<IconCheck size={24} />
									</div>
								</div>
								{#if healthData.database.responseTime !== null}
									<div class="metric-card">
										<div class="metric-icon">
											<IconClock size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Query Response</span>
											<span class="metric-value large"
												>{formatMs(healthData.database.responseTime)}</span
											>
										</div>
									</div>
								{/if}
								{#if healthData.database.size}
									<div class="metric-card">
										<div class="metric-icon">
											<IconFolder size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Database Size</span>
											<span class="metric-value large">{healthData.database.size}</span>
										</div>
									</div>
								{/if}
								{#if healthData.database.tables !== null}
									<div class="metric-card">
										<div class="metric-icon">
											<IconSettings size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Tables</span>
											<span class="metric-value large">{healthData.database.tables}</span>
										</div>
									</div>
								{/if}
							</div>
						{:else}
							<div class={getEmptyStateClass(healthData?.database.connected === false)}>
								<IconDatabase size={48} />
								<h4>
									{healthData?.database.connected === false
										? 'Database Disconnected'
										: 'No Database Data'}
								</h4>
								<p>
									{healthData?.database.connected === false
										? 'Unable to connect to database'
										: 'Run health tests to check database status'}
								</p>
								<button class="run-tests-btn" onclick={runHealthTests} disabled={isRunningTests}>
									<IconPlayerPlay size={18} />
									<span>Run Tests</span>
								</button>
							</div>
						{/if}
					</div>
				{:else if activeTab === 'server'}
					<!-- Server Tab -->
					<div class="detail-section">
						<h3>Server Information</h3>
						{#if healthData?.server.phpVersion || healthData?.server.webServer}
							<div class="metrics-grid">
								{#if healthData.server.phpVersion}
									<div class="metric-card">
										<div class="metric-icon">
											<IconCode size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">PHP Version</span>
											<span class="metric-value large">{healthData.server.phpVersion}</span>
										</div>
									</div>
								{/if}
								{#if healthData.server.webServer}
									<div class="metric-card">
										<div class="metric-icon">
											<IconCloud size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Web Server</span>
											<span class="metric-value large">{healthData.server.webServer}</span>
										</div>
									</div>
								{/if}
								{#if healthData.server.os}
									<div class="metric-card">
										<div class="metric-icon">
											<IconServer size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Operating System</span>
											<span class="metric-value large">{healthData.server.os}</span>
										</div>
									</div>
								{/if}
								{#if healthData.server.uptime}
									<div class="metric-card">
										<div class="metric-icon">
											<IconClock size={24} />
										</div>
										<div class="metric-content">
											<span class="metric-label">Uptime</span>
											<span class="metric-value large">{healthData.server.uptime}</span>
										</div>
									</div>
								{/if}
							</div>
						{:else}
							<div class="empty-state">
								<IconServer size={48} />
								<h4>No Server Data</h4>
								<p>Run health tests to collect server information</p>
								<button class="run-tests-btn" onclick={runHealthTests} disabled={isRunningTests}>
									<IconPlayerPlay size={18} />
									<span>Run Tests</span>
								</button>
							</div>
						{/if}
					</div>
				{/if}
			</div>

			<!-- Last Updated Footer -->
			{#if lastUpdated}
				<div class="last-updated" in:fade={{ delay: 500 }}>
					<IconClock size={14} />
					<span>Last updated: {lastUpdated.toLocaleString()}</span>
				</div>
			{/if}
		{/if}
	</div>
	<!-- End admin-page-container -->
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	   Base Layout - Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.admin-site-health {
		background: var(--bg-base);
		position: relative;
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Header - CENTERED
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.page-header {
		text-align: center;
		margin-bottom: 2rem;
		padding: 1.5rem 2rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 20px;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.subtitle {
		color: var(--text-tertiary, #6e7681);
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.header-actions {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.connections-status {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.connection-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 0.875rem;
		background: rgba(248, 81, 73, 0.15);
		border: 1px solid rgba(248, 81, 73, 0.3);
		border-radius: 6px;
		color: var(--error-emphasis, #f85149);
		font-size: 0.75rem;
		font-weight: 500;
	}

	.connection-indicator.healthy {
		background: rgba(63, 185, 80, 0.15);
		border-color: rgba(63, 185, 80, 0.3);
		color: #6ee7b7;
	}

	.error-indicator {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: rgba(210, 153, 34, 0.15);
		border: 1px solid rgba(210, 153, 34, 0.3);
		border-radius: 6px;
		color: var(--warning-emphasis, #d29922);
		font-size: 0.75rem;
		font-weight: 500;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Buttons - Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--primary-500);
		color: var(--bg-base, #0d1117);
		border: none;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		background: var(--bg-hover, #252b33);
		color: var(--text-secondary, #8b949e);
		border: 1px solid var(--border-default, #30363d);
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--bg-hover, #252b33);
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	:global(.spinning) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Loading State
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		color: var(--text-secondary, #8b949e);
	}

	.loading-spinner {
		width: 48px;
		height: 48px;
		border: 3px solid var(--border-default, #30363d);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Score Section - Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.score-section {
		display: flex;
		align-items: center;
		gap: 2rem;
		padding: 1.5rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
		margin-bottom: 1.5rem;
	}

	.score-card {
		display: flex;
		align-items: center;
		gap: 2rem;
	}

	.score-ring {
		position: relative;
		width: 140px;
		height: 140px;
	}

	.score-ring svg {
		width: 100%;
		height: 100%;
		transform: rotate(-90deg);
	}

	.ring-bg {
		fill: none;
		stroke: rgba(148, 163, 184, 0.1);
		stroke-width: 8;
	}

	.ring-progress {
		fill: none;
		stroke: var(--score-color);
		stroke-width: 8;
		stroke-linecap: round;
		stroke-dasharray: 339.3;
		transition: stroke-dashoffset 1.5s ease-out;
	}

	.score-value {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		display: flex;
		align-items: baseline;
		gap: 0.125rem;
	}

	.score-number {
		font-size: 2.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.score-unit {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-secondary, #8b949e);
	}

	.score-info {
		flex: 1;
	}

	.score-label {
		font-size: 1.5rem;
		font-weight: 700;
		margin: 0 0 0.5rem 0;
	}

	.score-description {
		font-size: 0.9rem;
		color: var(--text-secondary, #8b949e);
		margin: 0;
		line-height: 1.6;
		max-width: 400px;
	}

	.quick-stats {
		display: flex;
		gap: 1rem;
		margin-left: auto;
	}

	.stat-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem 1rem;
		background: var(--bg-elevated, #161b22);
		border-radius: 8px;
		min-width: 80px;
	}

	.stat-item.good {
		color: var(--success-emphasis, #3fb950);
	}

	.stat-item.warning {
		color: var(--warning-emphasis, #d29922);
	}

	.stat-item.critical {
		color: var(--error-emphasis, #f85149);
	}

	.stat-item.unknown {
		color: var(--text-tertiary, #6e7681);
	}

	.stat-item .stat-value {
		font-size: 1.75rem;
		font-weight: 700;
	}

	.stat-item .stat-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--text-secondary, #8b949e);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Tabs - Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.tabs-section {
		margin-bottom: 1.5rem;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
		padding: 0.375rem;
		background: var(--bg-elevated, #161b22);
		border-radius: 8px;
		overflow-x: auto;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.tab:hover {
		background: var(--bg-surface, #1c2128);
		color: var(--text-primary, #f0f6fc);
	}

	.tab.active {
		background: rgba(230, 184, 0, 0.2);
		color: #f9a8d4;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Tab Content - Email Templates Style
	   ═══════════════════════════════════════════════════════════════════════════════ */

	/* Overview Grid */
	.overview-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.overview-card {
		padding: 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
	}

	.card-header {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		margin-bottom: 1.25rem;
	}

	.card-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: 8px;
		color: #ffffff;
	}

	.overview-card.performance .card-icon {
		background: var(--primary-500, #e6b800);
	}

	.overview-card.security .card-icon {
		background: var(--success-base, #2ea043);
	}

	.overview-card.database .card-icon {
		background: var(--primary-500);
	}

	.overview-card.server .card-icon {
		background: var(--warning-emphasis, #d29922);
	}

	.card-title h3 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.card-title p {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
		margin: 0;
	}

	.card-metrics {
		margin-bottom: 1rem;
	}

	.metric {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.5rem 0;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	.metric:last-child {
		border-bottom: none;
	}

	.metric-label {
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
	}

	.metric-value {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.metric-value.good {
		color: var(--success-emphasis, #3fb950);
	}

	.metric-value.bad {
		color: var(--error-emphasis, #f85149);
	}

	.no-data {
		font-size: 0.8125rem;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
		padding: 1rem 0;
	}

	.card-action {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.5rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.card-action:hover {
		background: var(--bg-hover, #252b33);
	}

	/* API Health Section */
	.api-health-section {
		margin-bottom: 2rem;
	}

	.api-health-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 1rem 0;
	}

	.api-health-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 1rem;
	}

	.api-health-card {
		padding: 1rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
	}

	.api-health-card.has-errors {
		border-color: rgba(248, 81, 73, 0.3);
	}

	.api-health-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.75rem;
	}

	.api-health-value {
		font-size: 2rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
		margin-bottom: 0.75rem;
	}

	.api-health-action {
		display: inline-block;
		padding: 0.375rem 0.75rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.api-health-action:hover {
		background: var(--bg-hover, #252b33);
	}

	.api-health-action.error {
		background: rgba(248, 81, 73, 0.15);
		border-color: rgba(248, 81, 73, 0.3);
		color: var(--error-emphasis, #f85149);
	}

	.api-health-status {
		font-size: 0.75rem;
		color: var(--success-emphasis, #3fb950);
	}

	.api-health-bar {
		height: 6px;
		background: var(--bg-surface, #1c2128);
		border-radius: 3px;
		overflow: hidden;
	}

	.api-health-progress {
		height: 100%;
		border-radius: 3px;
		transition: width 0.5s ease-out;
	}

	/* Checks Section */
	.checks-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 1rem 0;
	}

	.checks-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.check-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.875rem 1rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
		transition: all 0.2s ease;
	}

	.check-item:hover {
		border-color: var(--border-default, #30363d);
	}

	.check-item.good {
		border-left: 3px solid var(--success-base, #2ea043);
	}

	.check-item.warning {
		border-left: 3px solid var(--warning-emphasis, #d29922);
	}

	.check-item.critical {
		border-left: 3px solid var(--error-base, #da3633);
	}

	.check-icon {
		flex-shrink: 0;
	}

	.check-info {
		flex: 1;
	}

	.check-info h4 {
		font-size: 0.9rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.check-info p {
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		margin: 0;
	}

	.check-action {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 6px;
		color: var(--text-primary, #f0f6fc);
		font-size: 0.75rem;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.check-action:hover {
		background: var(--bg-hover, #252b33);
	}

	/* Detail Section */
	.detail-section {
		margin-bottom: 2rem;
	}

	.detail-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 1.25rem 0;
	}

	.metrics-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
		gap: 1.25rem;
	}

	.metric-card {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1.25rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
	}

	.metric-card .metric-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		background: var(--bg-surface, #1c2128);
		border-radius: 8px;
		color: var(--text-secondary, #8b949e);
	}

	.metric-card .metric-content {
		flex: 1;
	}

	.metric-card .metric-label {
		display: block;
		font-size: 0.8125rem;
		color: var(--text-secondary, #8b949e);
		margin-bottom: 0.375rem;
	}

	.metric-card .metric-value.large {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.metric-card .metric-detail {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
		margin-top: 0.375rem;
	}

	.metric-bar {
		height: 6px;
		background: var(--bg-surface, #1c2128);
		border-radius: 3px;
		margin-top: 0.75rem;
		overflow: hidden;
	}

	.metric-progress {
		height: 100%;
		background: var(--primary-500);
		border-radius: 3px;
		transition: width 0.5s ease-out;
	}

	.metric-status {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: var(--bg-surface, #1c2128);
		color: var(--text-tertiary, #6e7681);
	}

	.metric-status.good {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}

	.metric-card.ssl.valid .metric-icon {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}

	.metric-card.ssl.invalid .metric-icon {
		background: rgba(248, 81, 73, 0.15);
		color: var(--error-emphasis, #f85149);
	}

	.metric-card.critical .metric-icon {
		background: rgba(248, 81, 73, 0.15);
		color: var(--error-emphasis, #f85149);
	}

	.metric-card.connection.connected .metric-icon {
		background: rgba(63, 185, 80, 0.15);
		color: var(--success-emphasis, #3fb950);
	}

	.metric-value.large.good {
		color: var(--success-emphasis, #3fb950);
	}

	/* Empty State - Email Templates Style */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-muted, #21262d);
		border-radius: 8px;
		text-align: center;
		color: var(--text-tertiary, #6e7681);
	}

	.empty-state.error {
		border-color: rgba(248, 81, 73, 0.3);
	}

	.empty-state h4 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 1rem 0 0.5rem;
	}

	.empty-state p {
		font-size: 0.9rem;
		margin: 0 0 1.5rem;
	}

	.run-tests-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: var(--primary-500);
		border: none;
		border-radius: 6px;
		color: var(--bg-base, #0d1117);
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.run-tests-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.run-tests-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Last Updated */
	.last-updated {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1.5rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Responsive
	   ═══════════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1023.98px) {
		.score-section {
			flex-direction: column;
			text-align: center;
		}

		.score-card {
			flex-direction: column;
		}

		.quick-stats {
			margin-left: 0;
			justify-content: center;
		}
	}

	@media (max-width: 767.98px) {
		.admin-site-health {
			padding: 1rem;
		}

		.page-header h1 {
			font-size: 1.5rem;
		}

		.header-actions {
			flex-direction: column;
			gap: 0.5rem;
		}

		.connections-status {
			flex-direction: column;
			gap: 0.5rem;
		}

		.overview-grid {
			grid-template-columns: 1fr;
		}

		.metrics-grid {
			grid-template-columns: 1fr;
		}

		.api-health-grid {
			grid-template-columns: 1fr;
		}

		.quick-stats {
			flex-wrap: wrap;
		}

		.stat-item {
			min-width: 70px;
			padding: 0.5rem 0.75rem;
		}
	}
</style>

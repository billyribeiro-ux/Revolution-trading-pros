<script lang="ts">
	/**
	 * Site Health Dashboard - Apple ICT9+ Enterprise Grade
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Comprehensive site health monitoring with:
	 * - Real-time system status checks
	 * - Security scan results
	 * - Performance monitoring
	 * - Database health metrics
	 * - API connection health
	 * - Apple-level polish and animations
	 *
	 * @version 1.0.0 - Enterprise Edition
	 */

	import { onMount, onDestroy } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { goto } from '$app/navigation';
	import { adminFetch } from '$lib/utils/adminFetch';
	import IconHeartbeat from '@tabler/icons-svelte/icons/heartbeat';
	import IconShieldCheck from '@tabler/icons-svelte/icons/shield-check';
	import IconDatabase from '@tabler/icons-svelte/icons/database';
	import IconServer from '@tabler/icons-svelte/icons/server';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconPlugConnected from '@tabler/icons-svelte/icons/plug-connected';
	import IconPlugConnectedX from '@tabler/icons-svelte/icons/plug-connected-x';
	import IconAlertTriangle from '@tabler/icons-svelte/icons/alert-triangle';
	import IconCircleCheck from '@tabler/icons-svelte/icons/circle-check';
	import IconCircleX from '@tabler/icons-svelte/icons/circle-x';
	import IconInfoCircle from '@tabler/icons-svelte/icons/info-circle';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconApiApp from '@tabler/icons-svelte/icons/api-app';
	import IconCertificate from '@tabler/icons-svelte/icons/certificate';
	import IconFolder from '@tabler/icons-svelte/icons/folder';
	import IconCloud from '@tabler/icons-svelte/icons/cloud';
	import IconCode from '@tabler/icons-svelte/icons/code';
	import IconBug from '@tabler/icons-svelte/icons/bug';
	import IconRocket from '@tabler/icons-svelte/icons/rocket';
	import IconChartLine from '@tabler/icons-svelte/icons/chart-line';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconExternalLink from '@tabler/icons-svelte/icons/external-link';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import {
		connections,
		connectedCount,
		overallHealth,
		servicesWithErrors
	} from '$lib/stores/connections';
	import { toastStore } from '$lib/stores/toast';

	// ═══════════════════════════════════════════════════════════════════════════════
	// Types
	// ═══════════════════════════════════════════════════════════════════════════════

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

	// ═══════════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════════

	let isLoading = $state(true);
	let isRefreshing = $state(false);
	let isRunningTests = $state(false);
	let healthData = $state<SiteHealthData | null>(null);
	let lastUpdated = $state<Date | null>(null);
	let activeTab = $state<'overview' | 'performance' | 'security' | 'database' | 'server'>('overview');

	// Animated score display
	const scoreDisplay = tweened(0, { duration: 1500, easing: cubicOut });

	// ═══════════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════════

	onMount(async () => {
		await connections.load();
		connections.startAutoRefresh();
		await loadHealthData();

		isLoading = false;
	});

	onDestroy(() => {
		connections.stopAutoRefresh();
	});

	// ═══════════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════════

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
				responseTime: (apiData['performance'] as Record<string, unknown>)?.['response_time'] as number | null ?? null,
				memoryUsage: (apiData['performance'] as Record<string, unknown>)?.['memory_usage'] as number | null ?? null,
				cpuUsage: (apiData['performance'] as Record<string, unknown>)?.['cpu_usage'] as number | null ?? null,
				diskUsage: (apiData['performance'] as Record<string, unknown>)?.['disk_usage'] as number | null ?? null
			},
			security: {
				sslValid: (apiData['security'] as Record<string, unknown>)?.['ssl_valid'] as boolean | null ?? null,
				sslExpiry: (apiData['security'] as Record<string, unknown>)?.['ssl_expiry'] ? new Date((apiData['security'] as Record<string, unknown>)['ssl_expiry'] as string) : null,
				headersScore: (apiData['security'] as Record<string, unknown>)?.['headers_score'] as number | null ?? null,
				vulnerabilities: (apiData['security'] as Record<string, unknown>)?.['vulnerabilities'] as number ?? 0
			},
			database: {
				connected: (apiData['database'] as Record<string, unknown>)?.['connected'] as boolean ?? false,
				responseTime: (apiData['database'] as Record<string, unknown>)?.['response_time'] as number | null ?? null,
				size: (apiData['database'] as Record<string, unknown>)?.['size'] as string | null ?? null,
				tables: (apiData['database'] as Record<string, unknown>)?.['tables'] as number | null ?? null
			},
			server: {
				phpVersion: (apiData['server'] as Record<string, unknown>)?.['php_version'] as string | null ?? null,
				webServer: (apiData['server'] as Record<string, unknown>)?.['web_server'] as string | null ?? null,
				os: (apiData['server'] as Record<string, unknown>)?.['os'] as string | null ?? null,
				uptime: (apiData['server'] as Record<string, unknown>)?.['uptime'] as string | null ?? null
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

	// ═══════════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════════

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
</script>

<svelte:head>
	<title>Site Health | Admin</title>
</svelte:head>

<div class="site-health-dashboard">
	<!-- Animated Background -->
	<div class="animated-bg">
		<div class="blob blob-1"></div>
		<div class="blob blob-2"></div>
		<div class="blob blob-3"></div>
	</div>

	<!-- Header Section -->
	<header class="dashboard-header" in:fly={{ y: -20, duration: 500, delay: 100 }}>
		<div class="header-content">
			<div class="header-icon">
				<IconHeartbeat size={32} />
			</div>
			<div class="header-text">
				<h1>Site Health</h1>
				<p>Monitor your site's performance, security, and overall health</p>
			</div>
		</div>

		<div class="header-actions">
			<!-- API Connections Status -->
			<div class="connections-status">
				<div class="connection-indicator" class:healthy={$connectedCount > 0}>
					{#if $connectedCount > 0}
						<IconPlugConnected size={14} />
					{:else}
						<IconPlugConnectedX size={14} />
					{/if}
					<span>{$connectedCount} APIs Connected</span>
				</div>
				{#if $servicesWithErrors.length > 0}
					<div class="error-indicator">
						<IconAlertTriangle size={14} />
						<span>{$servicesWithErrors.length} with issues</span>
					</div>
				{/if}
			</div>

			<button class="action-btn run-tests" onclick={runHealthTests} disabled={isRunningTests}>
				<span class="icon-wrapper" class:spinning={isRunningTests}>
					<IconPlayerPlay size={18} />
				</span>
				<span>{isRunningTests ? 'Running...' : 'Run Tests'}</span>
			</button>

			<button class="action-btn refresh" onclick={handleRefresh} disabled={isRefreshing}>
				<span class="icon-wrapper" class:spinning={isRefreshing}>
					<IconRefresh size={18} />
				</span>
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
				<div class="score-ring" style="--score-color: {getScoreColor($scoreDisplay)}">
					<svg viewBox="0 0 120 120">
						<circle class="ring-bg" cx="60" cy="60" r="54" />
						<circle
							class="ring-progress"
							cx="60"
							cy="60"
							r="54"
							style="stroke-dashoffset: {339.3 - (339.3 * $scoreDisplay) / 100}"
						/>
					</svg>
					<div class="score-value">
						<span class="score-number">{Math.round($scoreDisplay)}</span>
						<span class="score-unit">%</span>
					</div>
				</div>
				<div class="score-info">
					<h2 class="score-label" style="color: {getScoreColor($scoreDisplay)}">
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
						<span class="stat-value">{healthData.checks.filter((c) => c.status === 'good').length}</span>
						<span class="stat-label">Passed</span>
					</div>
					<div class="stat-item warning">
						<IconAlertTriangle size={20} />
						<span class="stat-value">{healthData.checks.filter((c) => c.status === 'warning').length}</span>
						<span class="stat-label">Warnings</span>
					</div>
					<div class="stat-item critical">
						<IconCircleX size={20} />
						<span class="stat-value">{healthData.checks.filter((c) => c.status === 'critical').length}</span>
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
				{#each tabs as tab}
					{@const TabIcon = tab.icon}
					<button
						class="tab"
						class:active={activeTab === tab.id}
						onclick={() => (activeTab = tab.id)}
					>
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
									<span class="metric-value">{formatMs(healthData?.performance.responseTime ?? 0)}</span>
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
									<span class="metric-value" class:good={healthData?.security.sslValid} class:bad={!healthData?.security.sslValid}>
										{healthData?.security.sslValid ? 'Valid' : 'Invalid'}
									</span>
								</div>
							{/if}
							{#if healthData?.security.vulnerabilities !== undefined}
								<div class="metric">
									<span class="metric-label">Vulnerabilities</span>
									<span class="metric-value" class:good={healthData.security.vulnerabilities === 0} class:bad={healthData.security.vulnerabilities > 0}>
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
									<span class="metric-value" class:good={healthData.database.connected} class:bad={!healthData.database.connected}>
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
							<div class="api-health-value">{$connectedCount}</div>
							<button class="api-health-action" onclick={() => goto('/admin/connections')}>
								Manage Connections
							</button>
						</div>
						<div class="api-health-card" class:has-errors={$servicesWithErrors.length > 0}>
							<div class="api-health-header">
								<IconAlertTriangle size={20} />
								<span>Connection Issues</span>
							</div>
							<div class="api-health-value">{$servicesWithErrors.length}</div>
							{#if $servicesWithErrors.length > 0}
								<button class="api-health-action error" onclick={() => goto('/admin/connections')}>
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
							<div class="api-health-value">{$overallHealth}%</div>
							<div class="api-health-bar">
								<div class="api-health-progress" style="width: {$overallHealth}%; background: {getScoreColor($overallHealth)}"></div>
							</div>
						</div>
					</div>
				</div>

				<!-- Health Checks List -->
				{#if healthData && healthData.checks.length > 0}
					<div class="checks-section" in:fly={{ y: 20, duration: 500, delay: 400 }}>
						<h3>All Health Checks</h3>
						<div class="checks-list">
							{#each healthData.checks as check, i}
								{@const StatusIcon = getStatusIcon(check.status)}
								<div
									class="check-item"
									class:good={check.status === 'good'}
									class:warning={check.status === 'warning'}
									class:critical={check.status === 'critical'}
									in:fly={{ y: 10, duration: 300, delay: 50 * i }}
								>
									<div class="check-icon" style="color: {getStatusColor(check.status)}">
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
										<span class="metric-value large">{formatMs(healthData?.performance.responseTime ?? 0)}</span>
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
										<span class="metric-value large">{healthData?.performance.memoryUsage ?? 0}%</span>
										<div class="metric-bar">
											<div class="metric-progress" style="width: {healthData?.performance.memoryUsage ?? 0}%"></div>
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
										<span class="metric-value large">{healthData?.performance.cpuUsage ?? 0}%</span>
										<div class="metric-bar">
											<div class="metric-progress" style="width: {healthData?.performance.cpuUsage ?? 0}%"></div>
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
										<span class="metric-value large">{healthData?.performance.diskUsage ?? 0}%</span>
										<div class="metric-bar">
											<div class="metric-progress" style="width: {healthData?.performance.diskUsage ?? 0}%"></div>
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
							<div class="metric-card ssl" class:valid={healthData?.security.sslValid} class:invalid={!healthData?.security.sslValid}>
								<div class="metric-icon">
									<IconCertificate size={24} />
								</div>
								<div class="metric-content">
									<span class="metric-label">SSL Certificate</span>
									<span class="metric-value large">{healthData?.security.sslValid ? 'Valid' : 'Invalid'}</span>
									{#if healthData?.security.sslExpiry}
										<span class="metric-detail">Expires: {healthData?.security.sslExpiry.toLocaleDateString()}</span>
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
										<span class="metric-value large">{healthData?.security.headersScore ?? 0}%</span>
										<div class="metric-bar">
											<div class="metric-progress" style="width: {healthData?.security.headersScore ?? 0}%; background: {getScoreColor(healthData?.security.headersScore ?? 0)}"></div>
										</div>
									</div>
								</div>
							{/if}
							<div class="metric-card" class:critical={(healthData?.security.vulnerabilities ?? 0) > 0}>
								<div class="metric-icon">
									<IconBug size={24} />
								</div>
								<div class="metric-content">
									<span class="metric-label">Vulnerabilities</span>
									<span class="metric-value large" class:good={(healthData?.security.vulnerabilities ?? 0) === 0}>{healthData?.security.vulnerabilities ?? 0}</span>
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
							<div class="metric-card connection" class:connected={healthData.database.connected}>
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
										<span class="metric-value large">{formatMs(healthData.database.responseTime)}</span>
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
						<div class="empty-state" class:error={healthData?.database.connected === false}>
							<IconDatabase size={48} />
							<h4>{healthData?.database.connected === false ? 'Database Disconnected' : 'No Database Data'}</h4>
							<p>{healthData?.database.connected === false ? 'Unable to connect to database' : 'Run health tests to check database status'}</p>
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

<style>
	/* ═══════════════════════════════════════════════════════════════════════════════
	   Base Layout
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.site-health-dashboard {
		position: relative;
		min-height: 100vh;
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		overflow: hidden;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Animated Background
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.animated-bg {
		position: absolute;
		inset: 0;
		overflow: hidden;
		pointer-events: none;
	}

	.blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(80px);
		opacity: 0.4;
		animation: float 20s ease-in-out infinite;
	}

	.blob-1 {
		width: 400px;
		height: 400px;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		top: -100px;
		right: -100px;
		animation-delay: 0s;
	}

	.blob-2 {
		width: 300px;
		height: 300px;
		background: linear-gradient(135deg, #10b981, #059669);
		bottom: 20%;
		left: -50px;
		animation-delay: -7s;
	}

	.blob-3 {
		width: 350px;
		height: 350px;
		background: linear-gradient(135deg, #3b82f6, #1d4ed8);
		top: 50%;
		right: 20%;
		animation-delay: -14s;
	}

	@keyframes float {
		0%,
		100% {
			transform: translate(0, 0) scale(1);
		}
		25% {
			transform: translate(30px, -30px) scale(1.05);
		}
		50% {
			transform: translate(-20px, 20px) scale(0.95);
		}
		75% {
			transform: translate(20px, 30px) scale(1.02);
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Header
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.dashboard-header {
		position: relative;
		z-index: 10;
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
		padding: 1.5rem 2rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.9));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 20px;
		backdrop-filter: blur(20px);
	}

	.header-content {
		display: flex;
		align-items: center;
		gap: 1.25rem;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 56px;
		height: 56px;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		border-radius: 16px;
		color: white;
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.3);
	}

	.header-text h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
		letter-spacing: -0.02em;
	}

	.header-text p {
		font-size: 0.9rem;
		color: #94a3b8;
		margin: 0;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 1rem;
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
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 10px;
		color: #fca5a5;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.connection-indicator.healthy {
		background: rgba(16, 185, 129, 0.15);
		border-color: rgba(16, 185, 129, 0.3);
		color: #6ee7b7;
	}

	.error-indicator {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: rgba(245, 158, 11, 0.15);
		border: 1px solid rgba(245, 158, 11, 0.3);
		border-radius: 10px;
		color: #fbbf24;
		font-size: 0.75rem;
		font-weight: 500;
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		border-radius: 12px;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.action-btn.run-tests {
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		border: none;
		color: white;
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.action-btn.run-tests:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 6px 16px rgba(230, 184, 0, 0.4);
	}

	.action-btn.refresh {
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		color: #e2e8f0;
		padding: 0.625rem;
	}

	.action-btn.refresh:hover:not(:disabled) {
		background: rgba(148, 163, 184, 0.2);
	}

	.action-btn:disabled {
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
		padding: 4rem;
		color: #94a3b8;
	}

	.loading-spinner {
		width: 48px;
		height: 48px;
		border: 3px solid rgba(148, 163, 184, 0.2);
		border-top-color: #E6B800;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Score Section
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.score-section {
		position: relative;
		z-index: 10;
		display: flex;
		align-items: center;
		gap: 3rem;
		padding: 2rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.9));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 24px;
		backdrop-filter: blur(20px);
		margin-bottom: 2rem;
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
		color: #f1f5f9;
	}

	.score-unit {
		font-size: 1.25rem;
		font-weight: 600;
		color: #94a3b8;
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
		color: #94a3b8;
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
		padding: 1rem 1.5rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 14px;
		min-width: 100px;
	}

	.stat-item.good {
		color: #10b981;
	}

	.stat-item.warning {
		color: #f59e0b;
	}

	.stat-item.critical {
		color: #ef4444;
	}

	.stat-item.unknown {
		color: #64748b;
	}

	.stat-item .stat-value {
		font-size: 1.75rem;
		font-weight: 700;
	}

	.stat-item .stat-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #94a3b8;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Tabs
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.tabs-section {
		position: relative;
		z-index: 10;
		margin-bottom: 2rem;
	}

	.tabs {
		display: flex;
		gap: 0.5rem;
		padding: 0.5rem;
		background: rgba(30, 41, 59, 0.6);
		border-radius: 16px;
		backdrop-filter: blur(10px);
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: transparent;
		border: none;
		border-radius: 12px;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.tab:hover {
		background: rgba(148, 163, 184, 0.1);
		color: #e2e8f0;
	}

	.tab.active {
		background: rgba(230, 184, 0, 0.2);
		color: #f9a8d4;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Tab Content
	   ═══════════════════════════════════════════════════════════════════════════════ */

	.tab-content {
		position: relative;
		z-index: 10;
	}

	/* Overview Grid */
	.overview-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 1.25rem;
		margin-bottom: 2rem;
	}

	.overview-card {
		padding: 1.5rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.9), rgba(15, 23, 42, 0.95));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 20px;
		backdrop-filter: blur(10px);
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
		width: 44px;
		height: 44px;
		border-radius: 12px;
		color: white;
	}

	.overview-card.performance .card-icon {
		background: linear-gradient(135deg, #3b82f6, #1d4ed8);
	}

	.overview-card.security .card-icon {
		background: linear-gradient(135deg, #10b981, #059669);
	}

	.overview-card.database .card-icon {
		background: linear-gradient(135deg, #8b5cf6, #7c3aed);
	}

	.overview-card.server .card-icon {
		background: linear-gradient(135deg, #f59e0b, #d97706);
	}

	.card-title h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.card-title p {
		font-size: 0.75rem;
		color: #64748b;
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
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.metric:last-child {
		border-bottom: none;
	}

	.metric-label {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.metric-value {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.metric-value.good {
		color: #10b981;
	}

	.metric-value.bad {
		color: #ef4444;
	}

	.no-data {
		font-size: 0.8125rem;
		color: #64748b;
		text-align: center;
		padding: 1rem 0;
	}

	.card-action {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.625rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.8125rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.card-action:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	/* API Health Section */
	.api-health-section {
		margin-bottom: 2rem;
	}

	.api-health-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 1rem 0;
	}

	.api-health-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 1rem;
	}

	.api-health-card {
		padding: 1.25rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.9), rgba(15, 23, 42, 0.95));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 16px;
	}

	.api-health-card.has-errors {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.api-health-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem;
		color: #94a3b8;
		margin-bottom: 0.75rem;
	}

	.api-health-value {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.api-health-action {
		display: inline-block;
		padding: 0.375rem 0.75rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.75rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.api-health-action:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	.api-health-action.error {
		background: rgba(239, 68, 68, 0.15);
		border-color: rgba(239, 68, 68, 0.3);
		color: #fca5a5;
	}

	.api-health-status {
		font-size: 0.75rem;
		color: #10b981;
	}

	.api-health-bar {
		height: 6px;
		background: rgba(148, 163, 184, 0.1);
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
		color: #f1f5f9;
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
		padding: 1rem 1.25rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.9), rgba(15, 23, 42, 0.95));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 14px;
		transition: all 0.2s ease;
	}

	.check-item:hover {
		border-color: rgba(148, 163, 184, 0.2);
	}

	.check-item.good {
		border-left: 3px solid #10b981;
	}

	.check-item.warning {
		border-left: 3px solid #f59e0b;
	}

	.check-item.critical {
		border-left: 3px solid #ef4444;
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
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.check-info p {
		font-size: 0.8125rem;
		color: #94a3b8;
		margin: 0;
	}

	.check-action {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.875rem;
		background: rgba(148, 163, 184, 0.1);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 8px;
		color: #e2e8f0;
		font-size: 0.75rem;
		font-weight: 500;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.check-action:hover {
		background: rgba(148, 163, 184, 0.2);
	}

	/* Detail Section */
	.detail-section {
		margin-bottom: 2rem;
	}

	.detail-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
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
		padding: 1.5rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.9), rgba(15, 23, 42, 0.95));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 18px;
	}

	.metric-card .metric-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 14px;
		color: #94a3b8;
	}

	.metric-card .metric-content {
		flex: 1;
	}

	.metric-card .metric-label {
		display: block;
		font-size: 0.8125rem;
		color: #94a3b8;
		margin-bottom: 0.375rem;
	}

	.metric-card .metric-value.large {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.metric-card .metric-detail {
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.375rem;
	}

	.metric-bar {
		height: 6px;
		background: rgba(148, 163, 184, 0.1);
		border-radius: 3px;
		margin-top: 0.75rem;
		overflow: hidden;
	}

	.metric-progress {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #8b5cf6);
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
		background: rgba(148, 163, 184, 0.1);
		color: #64748b;
	}

	.metric-status.good {
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
	}

	.metric-card.ssl.valid .metric-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
	}

	.metric-card.ssl.invalid .metric-icon {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.metric-card.critical .metric-icon {
		background: rgba(239, 68, 68, 0.15);
		color: #ef4444;
	}

	.metric-card.connection.connected .metric-icon {
		background: rgba(16, 185, 129, 0.15);
		color: #10b981;
	}

	.metric-value.large.good {
		color: #10b981;
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.9));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 20px;
		text-align: center;
		color: #64748b;
	}

	.empty-state.error {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.empty-state h4 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
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
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		border: none;
		border-radius: 12px;
		color: white;
		font-size: 0.9rem;
		font-weight: 600;
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
		position: relative;
		z-index: 10;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 2rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	/* ═══════════════════════════════════════════════════════════════════════════════
	   Responsive
	   ═══════════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 1024px) {
		.dashboard-header {
			flex-direction: column;
			gap: 1.5rem;
			text-align: center;
		}

		.header-content {
			flex-direction: column;
		}

		.header-actions {
			width: 100%;
			justify-content: center;
			flex-wrap: wrap;
		}

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

	@media (max-width: 768px) {
		.site-health-dashboard {
			padding: 1rem;
		}

		.dashboard-header {
			padding: 1.25rem;
			border-radius: 16px;
		}

		.header-text h1 {
			font-size: 1.5rem;
		}

		.tabs {
			overflow-x: auto;
			-webkit-overflow-scrolling: touch;
		}

		.tab {
			white-space: nowrap;
			padding: 0.625rem 1rem;
			font-size: 0.8125rem;
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
			min-width: 80px;
			padding: 0.75rem 1rem;
		}

		.connections-status {
			flex-direction: column;
		}
	}
</style>

<script lang="ts">
	import { browser } from '$app/environment';
	import IconActivityHeartbeat from '@tabler/icons-svelte-runes/icons/activity-heartbeat';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconAlertTriangle from '@tabler/icons-svelte-runes/icons/alert-triangle';
	import IconInfoCircle from '@tabler/icons-svelte-runes/icons/info-circle';
	import IconBug from '@tabler/icons-svelte-runes/icons/bug';
	import IconExclamationCircle from '@tabler/icons-svelte-runes/icons/exclamation-circle';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconRoute from '@tabler/icons-svelte-runes/icons/route';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconApi from '@tabler/icons-svelte-runes/icons/api';
	import IconWebhook from '@tabler/icons-svelte-runes/icons/webhook';
	import IconSettings from '@tabler/icons-svelte-runes/icons/settings';
	import { crmAPI } from '$lib/api/crm';
	import type { SystemLog, LogLevel, LogCategory } from '$lib/crm/types';

	let logs = $state<SystemLog[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let searchQuery = $state('');
	let levelFilter = $state<LogLevel | ''>('');
	let categoryFilter = $state<LogCategory | ''>('');

	let stats = $state({
		total: 0,
		by_level: {} as Record<string, number>,
		by_category: {} as Record<string, number>
	});

	async function loadLogs() {
		isLoading = true;
		error = '';

		try {
			const [logsResponse, statsResponse] = await Promise.all([
				crmAPI.getSystemLogs({
					level: levelFilter || undefined,
					category: categoryFilter || undefined,
					search: searchQuery || undefined
				}),
				crmAPI.getLogStats()
			]);

			logs = logsResponse.data || [];
			stats = statsResponse;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load logs';
		} finally {
			isLoading = false;
		}
	}

	async function clearLogs() {
		if (!confirm('Are you sure you want to clear system logs? This action cannot be undone.'))
			return;

		try {
			await crmAPI.clearSystemLogs({
				level: levelFilter || undefined,
				category: categoryFilter || undefined
			});
			await loadLogs();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to clear logs';
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		});
	}

	function getLevelIcon(level: LogLevel) {
		const icons = {
			debug: IconBug,
			info: IconInfoCircle,
			warning: IconAlertTriangle,
			error: IconAlertCircle,
			critical: IconExclamationCircle
		};
		return icons[level] || IconInfoCircle;
	}

	function getLevelColor(level: LogLevel): string {
		const colors = {
			debug: 'gray',
			info: 'blue',
			warning: 'yellow',
			error: 'red',
			critical: 'purple'
		};
		return colors[level] || 'gray';
	}

	function getCategoryIcon(category: LogCategory) {
		const icons = {
			email: IconMail,
			automation: IconRoute,
			import: IconUpload,
			api: IconApi,
			webhook: IconWebhook,
			system: IconSettings
		};
		return icons[category] || IconSettings;
	}

	let filteredLogs = $derived(
		logs.filter((log) => {
			if (levelFilter && log.level !== levelFilter) return false;
			if (categoryFilter && log.category !== categoryFilter) return false;
			if (searchQuery) {
				return log.message.toLowerCase().includes(searchQuery.toLowerCase());
			}
			return true;
		})
	);

	$effect(() => {
		if (browser) {
			loadLogs();
		}
	});
</script>

<svelte:head>
	<title>System Logs - FluentCRM Pro</title>
</svelte:head>

<div class="logs-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>System Logs</h1>
			<p class="page-description">Monitor and debug system activity</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadLogs()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
			<button class="btn-danger" onclick={clearLogs}>
				<IconTrash size={18} />
				Clear Logs
			</button>
		</div>
	</div>

	<!-- Stats Cards -->
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon blue">
				<IconActivityHeartbeat size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{stats.total.toLocaleString()}</span>
				<span class="stat-label">Total Logs</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon red">
				<IconAlertCircle size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{(stats.by_level?.error || 0).toLocaleString()}</span>
				<span class="stat-label">Errors</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon yellow">
				<IconAlertTriangle size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{(stats.by_level?.warning || 0).toLocaleString()}</span>
				<span class="stat-label">Warnings</span>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon purple">
				<IconExclamationCircle size={24} />
			</div>
			<div class="stat-content">
				<span class="stat-value">{(stats.by_level?.critical || 0).toLocaleString()}</span>
				<span class="stat-label">Critical</span>
			</div>
		</div>
	</div>

	<!-- Filters -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				id="page-searchquery" name="page-searchquery" type="text"
				placeholder="Search logs..."
				bind:value={searchQuery}
				onchange={loadLogs}
			/>
		</div>
		<select bind:value={levelFilter} class="filter-select" onchange={loadLogs}>
			<option value="">All Levels</option>
			<option value="debug">Debug</option>
			<option value="info">Info</option>
			<option value="warning">Warning</option>
			<option value="error">Error</option>
			<option value="critical">Critical</option>
		</select>
		<select bind:value={categoryFilter} class="filter-select" onchange={loadLogs}>
			<option value="">All Categories</option>
			<option value="email">Email</option>
			<option value="automation">Automation</option>
			<option value="import">Import</option>
			<option value="api">API</option>
			<option value="webhook">Webhook</option>
			<option value="system">System</option>
		</select>
	</div>

	<!-- Logs List -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading logs...</p>
		</div>
	{:else if error}
		<div class="error-state">
			<p>{error}</p>
			<button onclick={() => loadLogs()}>Try Again</button>
		</div>
	{:else if filteredLogs.length === 0}
		<div class="empty-state">
			<IconActivityHeartbeat size={48} />
			<h3>No logs found</h3>
			<p>System logs will appear here as activity occurs</p>
		</div>
	{:else}
		<div class="logs-list">
			{#each filteredLogs as log}
				{@const LevelIcon = getLevelIcon(log.level)}
				{@const CategoryIcon = getCategoryIcon(log.category)}
				<div class="log-entry {getLevelColor(log.level)}">
					<div class="log-header">
						<div class="log-level">
							<LevelIcon size={16} />
							<span>{log.level}</span>
						</div>
						<div class="log-category">
							<CategoryIcon size={14} />
							<span>{log.category}</span>
						</div>
						<span class="log-time">{formatDate(log.created_at)}</span>
					</div>
					<div class="log-message">{log.message}</div>
					{#if log.context && Object.keys(log.context).length > 0}
						<div class="log-context">
							<details>
								<summary>Context</summary>
								<pre>{JSON.stringify(log.context, null, 2)}</pre>
							</details>
						</div>
					{/if}
					{#if log.contact_id}
						<div class="log-meta">
							<a href="/admin/crm/contacts/{log.contact_id}" class="contact-link"> View Contact </a>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.logs-page {
		max-width: 1200px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.page-header h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.page-description {
		color: #64748b;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
	}

	.btn-refresh :global(.spinning) {
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

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: rgba(239, 68, 68, 0.2);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		margin-bottom: 2rem;
	}

	@media (max-width: 1024px) {
		.stats-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}

	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 14px;
	}

	.stat-icon {
		width: 52px;
		height: 52px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.stat-icon.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.stat-icon.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.stat-icon.yellow {
		background: rgba(251, 191, 36, 0.15);
		color: #fbbf24;
	}
	.stat-icon.purple {
		background: rgba(139, 92, 246, 0.15);
		color: #a78bfa;
	}

	.stat-content {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
	}

	.stat-label {
		font-size: 0.8rem;
		color: #64748b;
	}

	.filters-bar {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
		flex-wrap: wrap;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		flex: 1;
		max-width: 400px;
	}

	.search-box :global(svg) {
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		padding: 0.75rem 0;
		background: transparent;
		border: none;
		color: #e2e8f0;
		font-size: 0.9rem;
		outline: none;
	}

	.search-box input::placeholder {
		color: #64748b;
	}

	.filter-select {
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		color: #e2e8f0;
		font-size: 0.9rem;
		cursor: pointer;
	}

	.logs-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.log-entry {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(99, 102, 241, 0.1);
		border-radius: 10px;
		padding: 1rem 1.25rem;
		border-left: 3px solid;
	}

	.log-entry.gray {
		border-left-color: #64748b;
	}
	.log-entry.blue {
		border-left-color: #3b82f6;
	}
	.log-entry.yellow {
		border-left-color: #fbbf24;
	}
	.log-entry.red {
		border-left-color: #ef4444;
	}
	.log-entry.purple {
		border-left-color: #8b5cf6;
	}

	.log-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		margin-bottom: 0.5rem;
	}

	.log-level {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.log-entry.gray .log-level {
		color: #64748b;
	}
	.log-entry.blue .log-level {
		color: #60a5fa;
	}
	.log-entry.yellow .log-level {
		color: #fbbf24;
	}
	.log-entry.red .log-level {
		color: #f87171;
	}
	.log-entry.purple .log-level {
		color: #a78bfa;
	}

	.log-category {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 4px;
		font-size: 0.7rem;
		color: #818cf8;
		text-transform: capitalize;
	}

	.log-time {
		font-size: 0.8rem;
		color: #64748b;
		margin-left: auto;
	}

	.log-message {
		color: #e2e8f0;
		font-size: 0.9rem;
		line-height: 1.5;
	}

	.log-context {
		margin-top: 0.75rem;
	}

	.log-context summary {
		font-size: 0.8rem;
		color: #818cf8;
		cursor: pointer;
	}

	.log-context pre {
		margin: 0.5rem 0 0 0;
		padding: 0.75rem;
		background: rgba(0, 0, 0, 0.3);
		border-radius: 6px;
		font-size: 0.8rem;
		color: #94a3b8;
		overflow-x: auto;
	}

	.log-meta {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
	}

	.contact-link {
		font-size: 0.85rem;
		color: #818cf8;
		text-decoration: none;
	}

	.contact-link:hover {
		text-decoration: underline;
	}

	.loading-state,
	.error-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
		color: #64748b;
	}

	.empty-state :global(svg) {
		color: #475569;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		color: #e2e8f0;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		margin: 0;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(99, 102, 241, 0.2);
		border-top-color: var(--primary-500);
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 1rem;
	}
</style>

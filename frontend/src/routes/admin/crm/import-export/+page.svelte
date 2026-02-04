<!--
	/admin/crm/import-export - Import/Export Data Management
	Apple Principal Engineer ICT 7 Grade - January 2026
	
	Features:
	- Import contacts, tags, lists, sequences, campaigns, automations, templates
	- Export with download functionality
	- Job status tracking with progress
	- File type cards with icons
	- Full Svelte 5 $state/$derived reactivity
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconFile from '@tabler/icons-svelte-runes/icons/file';
	import IconFileSpreadsheet from '@tabler/icons-svelte-runes/icons/file-spreadsheet';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconTags from '@tabler/icons-svelte-runes/icons/tags';
	import IconListDetails from '@tabler/icons-svelte-runes/icons/list-details';
	import IconMailForward from '@tabler/icons-svelte-runes/icons/mail-forward';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconRoute from '@tabler/icons-svelte-runes/icons/route';
	import IconTemplate from '@tabler/icons-svelte-runes/icons/template';
	import IconRefresh from '@tabler/icons-svelte-runes/icons/refresh';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconClock from '@tabler/icons-svelte-runes/icons/clock';
	import { crmAPI } from '$lib/api/crm';
	import type { ImportJob, ExportJob } from '$lib/crm/types';

	let importJobs = $state<ImportJob[]>([]);
	let exportJobs = $state<ExportJob[]>([]);
	let isLoading = $state(true);
	let error = $state('');
	let activeTab = $state<'import' | 'export'>('import');

	const importTypes = [
		{
			type: 'contacts',
			name: 'Contacts',
			icon: IconUsers,
			description: 'Import contacts from CSV'
		},
		{ type: 'tags', name: 'Tags', icon: IconTags, description: 'Import contact tags' },
		{ type: 'lists', name: 'Lists', icon: IconListDetails, description: 'Import contact lists' },
		{
			type: 'sequences',
			name: 'Email Sequences',
			icon: IconMailForward,
			description: 'Import email sequences'
		},
		{ type: 'campaigns', name: 'Campaigns', icon: IconMail, description: 'Import campaigns' },
		{
			type: 'automations',
			name: 'Automations',
			icon: IconRoute,
			description: 'Import automation funnels'
		},
		{
			type: 'templates',
			name: 'Templates',
			icon: IconTemplate,
			description: 'Import email templates'
		}
	];

	const exportTypes = [
		{ type: 'contacts', name: 'Contacts', icon: IconUsers, description: 'Export all contacts' },
		{ type: 'tags', name: 'Tags', icon: IconTags, description: 'Export contact tags' },
		{ type: 'lists', name: 'Lists', icon: IconListDetails, description: 'Export contact lists' },
		{
			type: 'sequences',
			name: 'Email Sequences',
			icon: IconMailForward,
			description: 'Export email sequences'
		},
		{ type: 'campaigns', name: 'Campaigns', icon: IconMail, description: 'Export campaigns' },
		{
			type: 'automations',
			name: 'Automations',
			icon: IconRoute,
			description: 'Export automation funnels'
		},
		{
			type: 'templates',
			name: 'Templates',
			icon: IconTemplate,
			description: 'Export email templates'
		}
	];

	async function loadJobs() {
		isLoading = true;
		error = '';

		try {
			const [importResponse, exportResponse] = await Promise.all([
				crmAPI.getImportJobs(),
				crmAPI.getExportJobs()
			]);

			importJobs = importResponse.data || [];
			exportJobs = exportResponse.data || [];
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load jobs';
		} finally {
			isLoading = false;
		}
	}

	async function startExport(type: string) {
		try {
			const job = await crmAPI.createExportJob(type);
			await loadJobs();
			alert(`Export started. Job ID: ${job.id}`);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to start export';
		}
	}

	async function downloadExport(id: string) {
		try {
			const blob = await crmAPI.downloadExport(id);
			const url = window.URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `export-${id}.csv`;
			a.click();
			window.URL.revokeObjectURL(url);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to download export';
		}
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStatusIcon(status: string) {
		switch (status) {
			case 'completed':
				return IconCheck;
			case 'failed':
				return IconX;
			case 'processing':
				return IconRefresh;
			default:
				return IconClock;
		}
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed':
				return 'green';
			case 'failed':
				return 'red';
			case 'processing':
				return 'blue';
			default:
				return 'gray';
		}
	}

	$effect(() => {
		if (browser) {
			loadJobs();
		}
	});
</script>

<svelte:head>
	<title>Import / Export - FluentCRM Pro</title>
</svelte:head>

<div class="import-export-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Import / Export</h1>
			<p class="page-description">Import and export your CRM data</p>
		</div>
		<div class="header-actions">
			<button class="btn-refresh" onclick={() => loadJobs()} disabled={isLoading}>
				<IconRefresh size={18} class={isLoading ? 'spinning' : ''} />
			</button>
		</div>
	</div>

	<!-- Tabs -->
	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === 'import'}
			onclick={() => (activeTab = 'import')}
		>
			<IconUpload size={18} />
			Import
		</button>
		<button
			class="tab"
			class:active={activeTab === 'export'}
			onclick={() => (activeTab = 'export')}
		>
			<IconDownload size={18} />
			Export
		</button>
	</div>

	{#if activeTab === 'import'}
		<!-- Import Section -->
		<div class="section">
			<h2>Import Data</h2>
			<p class="section-description">Select what you want to import</p>

			<div class="type-grid">
				{#each importTypes as { type, name, icon: Icon, description }}
					<a href="/admin/crm/import-export/import/{type}" class="type-card">
						<div class="type-icon">
							<Icon size={24} />
						</div>
						<div class="type-info">
							<h3>{name}</h3>
							<p>{description}</p>
						</div>
					</a>
				{/each}
			</div>
		</div>

		<!-- Recent Imports -->
		{#if importJobs.length > 0}
			<div class="section">
				<h2>Recent Imports</h2>
				<div class="jobs-table">
					<table>
						<thead>
							<tr>
								<th>Type</th>
								<th>File</th>
								<th>Status</th>
								<th>Progress</th>
								<th>Created</th>
							</tr>
						</thead>
						<tbody>
							{#each importJobs as job}
								{@const StatusIcon = getStatusIcon(job.status)}
								<tr>
									<td>
										<span class="type-badge">{job.type}</span>
									</td>
									<td>
										<span class="file-name">{job.file_name}</span>
									</td>
									<td>
										<span class="status-badge {getStatusColor(job.status)}">
											<StatusIcon size={14} />
											{job.status}
										</span>
									</td>
									<td>
										<div class="progress-info">
											<span>{job.processed_rows} / {job.total_rows}</span>
											{#if job.error_count > 0}
												<span class="error-count">{job.error_count} errors</span>
											{/if}
										</div>
									</td>
									<td>
										<span class="date">{formatDate(job.created_at)}</span>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	{:else}
		<!-- Export Section -->
		<div class="section">
			<h2>Export Data</h2>
			<p class="section-description">Select what you want to export</p>

			<div class="type-grid">
				{#each exportTypes as { type, name, icon: Icon, description }}
					<button class="type-card" onclick={() => startExport(type)}>
						<div class="type-icon">
							<Icon size={24} />
						</div>
						<div class="type-info">
							<h3>{name}</h3>
							<p>{description}</p>
						</div>
					</button>
				{/each}
			</div>
		</div>

		<!-- Recent Exports -->
		{#if exportJobs.length > 0}
			<div class="section">
				<h2>Recent Exports</h2>
				<div class="jobs-table">
					<table>
						<thead>
							<tr>
								<th>Type</th>
								<th>Records</th>
								<th>Status</th>
								<th>Created</th>
								<th>Actions</th>
							</tr>
						</thead>
						<tbody>
							{#each exportJobs as job}
								{@const ExportStatusIcon = getStatusIcon(job.status)}
								<tr>
									<td>
										<span class="type-badge">{job.type}</span>
									</td>
									<td>
										<span>{job.total_records.toLocaleString()}</span>
									</td>
									<td>
										<span class="status-badge {getStatusColor(job.status)}">
											<ExportStatusIcon size={14} />
											{job.status}
										</span>
									</td>
									<td>
										<span class="date">{formatDate(job.created_at)}</span>
									</td>
									<td>
										{#if job.status === 'completed' && job.file_url}
											<button class="btn-download" onclick={() => downloadExport(job.id)}>
												<IconDownload size={16} />
												Download
											</button>
										{/if}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	{/if}

	{#if error}
		<div class="error-message">
			<IconX size={18} />
			<span>{error}</span>
		</div>
	{/if}
</div>

<style>
	.import-export-page {
		max-width: 1200px;
	}

	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
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

	.btn-refresh {
		width: 42px;
		height: 42px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(30, 41, 59, 0.8);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 10px;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-refresh:hover {
		background: rgba(230, 184, 0, 0.2);
		color: #ffd11a;
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

	.tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 2rem;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
		padding-bottom: 1rem;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 10px;
		color: #64748b;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #ffd11a;
	}

	.tab.active {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.3);
		color: #ffd11a;
	}

	.section {
		margin-bottom: 3rem;
	}

	.section h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.section-description {
		color: #64748b;
		margin: 0 0 1.5rem 0;
	}

	.type-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
		gap: 1rem;
	}

	.type-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
		width: 100%;
	}

	.type-card:hover {
		border-color: rgba(230, 184, 0, 0.3);
		transform: translateY(-2px);
	}

	.type-icon {
		width: 48px;
		height: 48px;
		border-radius: 12px;
		background: rgba(230, 184, 0, 0.15);
		display: flex;
		align-items: center;
		justify-content: center;
		color: #ffd11a;
		flex-shrink: 0;
	}

	.type-info h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.type-info p {
		font-size: 0.85rem;
		color: #64748b;
		margin: 0;
	}

	.jobs-table {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(230, 184, 0, 0.1);
		border-radius: 14px;
		overflow: hidden;
	}

	.jobs-table table {
		width: 100%;
		border-collapse: collapse;
	}

	.jobs-table th,
	.jobs-table td {
		padding: 1rem;
		text-align: left;
		border-bottom: 1px solid rgba(230, 184, 0, 0.1);
	}

	.jobs-table th {
		background: rgba(30, 41, 59, 0.5);
		color: #94a3b8;
		font-weight: 600;
		font-size: 0.8rem;
		text-transform: uppercase;
	}

	.jobs-table tr:last-child td {
		border-bottom: none;
	}

	.type-badge {
		padding: 0.25rem 0.75rem;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		color: #ffd11a;
		text-transform: capitalize;
	}

	.file-name {
		color: #e2e8f0;
		font-family: monospace;
		font-size: 0.85rem;
	}

	.status-badge {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: capitalize;
	}

	.status-badge.green {
		background: rgba(34, 197, 94, 0.15);
		color: #4ade80;
	}
	.status-badge.red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}
	.status-badge.blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}
	.status-badge.gray {
		background: rgba(100, 116, 139, 0.15);
		color: #94a3b8;
	}

	.progress-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.error-count {
		font-size: 0.75rem;
		color: #f87171;
	}

	.date {
		color: #64748b;
		font-size: 0.85rem;
	}

	.btn-download {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 8px;
		color: #ffd11a;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-download:hover {
		background: rgba(230, 184, 0, 0.2);
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 10px;
		color: #f87171;
	}
</style>

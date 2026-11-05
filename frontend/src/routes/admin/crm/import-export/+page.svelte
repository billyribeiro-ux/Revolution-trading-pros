<script lang="ts">
	import { onMount } from 'svelte';
	import { useCrmAccess } from '#lib/crm/access.svelte.js';
	const access = useCrmAccess();
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';
	import IconUsers from '@tabler/icons-svelte-runes/icons/users';
	import IconTags from '@tabler/icons-svelte-runes/icons/tags';
	import IconListDetails from '@tabler/icons-svelte-runes/icons/list-details';
	import IconMailForward from '@tabler/icons-svelte-runes/icons/mail-forward';
	import IconMail from '@tabler/icons-svelte-runes/icons/mail';
	import IconRoute from '@tabler/icons-svelte-runes/icons/route';
	import IconTemplate from '@tabler/icons-svelte-runes/icons/template';
	import { crmAPI } from '#lib/api/crm.js';
	import { contactExportFields } from '#lib/api/crm-transfer-contract.js';
	import { apiErrorMessage } from '#lib/utils/api-error-message.js';
	import type { ImportJob, ExportJob } from '#lib/crm/types.js';
	let importJobs = $state<ImportJob[]>([]),
		exportJobs = $state<ExportJob[]>([]);
	let importPage = $state(1),
		exportPage = $state(1),
		importPages = $state(1),
		exportPages = $state(1);
	let importTotal = $state<number | null>(null),
		exportTotal = $state<number | null>(null);
	let isLoading = $state(true),
		busy = $state(false),
		error = $state(''),
		notice = $state('');
	let activeTab = $state<'import' | 'export'>('import'),
		format = $state<'csv' | 'json'>('csv');
	let search = $state(''),
		status = $state('');
	let fields = $state<string[]>([
		'email',
		'first_name',
		'last_name',
		'status',
		'tags',
		'custom_fields'
	]);
	let disposed = false,
		generation = 0;
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

	async function loadJobs(force = false) {
		if (busy && !force) return;
		const current = ++generation;
		isLoading = true;
		error = '';
		try {
			const results = await Promise.allSettled([
				access.can('GET', '/imports')
					? crmAPI.getImportJobs({ page: importPage, per_page: 20 })
					: Promise.resolve(null),
				access.can('GET', '/exports')
					? crmAPI.getExportJobs({ page: exportPage, per_page: 20 })
					: Promise.resolve(null)
			]);
			if (disposed || current !== generation) return;
			const errors: string[] = [];
			const imported = results[0],
				exported = results[1];
			if (imported.status === 'fulfilled' && imported.value) {
				importJobs = imported.value.data;
				importTotal = imported.value.meta.total;
				importPages = imported.value.meta.total_pages ?? 1;
			} else if (imported.status === 'rejected') {
				importJobs = [];
				importTotal = null;
				errors.push(apiErrorMessage(imported.reason, 'Import history could not be loaded'));
			} else {
				importJobs = [];
				importTotal = null;
			}
			if (exported.status === 'fulfilled' && exported.value) {
				exportJobs = exported.value.data;
				exportTotal = exported.value.meta.total;
				exportPages = exported.value.meta.total_pages ?? 1;
			} else if (exported.status === 'rejected') {
				exportJobs = [];
				exportTotal = null;
				errors.push(apiErrorMessage(exported.reason, 'Export history could not be loaded'));
			} else {
				exportJobs = [];
				exportTotal = null;
			}
			error = errors.join(' ');
		} finally {
			if (!disposed && current === generation) isLoading = false;
		}
	}
	async function changePage(direction: 'import' | 'export', by: number) {
		if (busy || isLoading) return;
		if (direction === 'import') importPage += by;
		else exportPage += by;
		await loadJobs();
	}
	async function startExport(type: string) {
		if (!access.can('POST', '/exports')) {
			error = 'Your role cannot create exports.';
			return;
		}
		if (busy) return;
		busy = true;
		error = '';
		notice = '';
		try {
			if (!fields.length) throw new Error('Select at least one export field');
			const saved = await crmAPI.createExportJob(
				type,
				{ ...(search ? { search } : {}), ...(status ? { status } : {}) },
				fields,
				format
			);
			if (disposed) return;
			exportJobs = [saved, ...exportJobs.filter((job) => job.id !== saved.id)].slice(0, 20);
			exportPage = 1;
			notice = `Export completed: ${saved.total_records} records saved as ${saved.file_type.toUpperCase()}.`;
			await loadJobs(true);
		} catch (cause) {
			if (!disposed) error = apiErrorMessage(cause, 'Export creation was not confirmed');
		} finally {
			if (!disposed) busy = false;
		}
	}
	async function downloadExport(job: ExportJob) {
		if (!access.can('GET', '/exports/{id}/download')) {
			error = 'Your role cannot download exports.';
			return;
		}
		if (busy) return;
		busy = true;
		error = '';
		try {
			const blob = await crmAPI.downloadExport(job.id);
			if (disposed) return;
			const url = URL.createObjectURL(blob),
				link = document.createElement('a');
			link.href = url;
			link.download = job.file_name;
			link.click();
			setTimeout(() => URL.revokeObjectURL(url), 0);
		} catch (cause) {
			if (!disposed) error = apiErrorMessage(cause, 'Unable to download the saved export');
		} finally {
			if (!disposed) busy = false;
		}
	}
	function at(value: string) {
		return new Date(value).toLocaleString();
	}
	onMount(() => {
		void loadJobs();
		return () => {
			disposed = true;
			generation += 1;
		};
	});
</script>

<div class="import-export-page">
	<div class="page-header">
		<div>
			<h1>Import / Export</h1>
			<p>Upload and validate CRM data or download a recorded export snapshot.</p>
		</div>
		<button type="button" onclick={() => loadJobs()} disabled={isLoading || busy}
			>Refresh jobs</button
		>
	</div>
	<div class="tabs">
		<button
			class={{ active: activeTab === 'import' }}
			type="button"
			onclick={() => (activeTab = 'import')}><IconUpload size={18} />Import</button
		><button
			class={{ active: activeTab === 'export' }}
			type="button"
			onclick={() => (activeTab = 'export')}><IconDownload size={18} />Export</button
		>
	</div>
	{#if error}<div role="alert">
			{error}<button type="button" onclick={() => loadJobs()} disabled={isLoading || busy}
				>Retry history</button
			>
		</div>{/if}
	{#if notice}<p role="status">{notice}</p>{/if}
	<p class="scope">
		Contact transfers are implemented. Tags, lists, sequences, campaigns, automations and templates
		still require their transfer workflows; those requests return a service error.
	</p>
	{#if activeTab === 'import'}
		<section>
			<h2>Import Data</h2>
			<div class="type-grid">
				{#each importTypes as item (item.type)}{#if access.page(`/admin/crm/import-export/import/${item.type}`)}<a
							class="type-card"
							href={`/admin/crm/import-export/import/${item.type}`}
							><item.icon size={24} /><span
								><strong>{item.name}</strong><small>{item.description}</small></span
							></a
						>{/if}{/each}
			</div>
		</section>
		<section>
			<h2>Recent Imports</h2>
			{#if access.can('GET', '/imports')}{#if isLoading}<p>
						Loading import history…
					</p>{:else if importTotal === 0}<p>No saved imports.</p>{/if}
				<div class="jobs-table">
					<table>
						<thead
							><tr
								><th>Type / file</th><th>Status</th><th>Recorded results</th><th>Created</th><th
									>Action</th
								></tr
							></thead
						><tbody
							>{#each importJobs as job (job.id)}<tr data-import-id={job.id}
									><td>{job.type}<br /><strong>{job.file_name}</strong></td><td>{job.status}</td><td
										>{job.processed_rows} / {job.total_rows} rows · {job.created_count} created · {job.updated_count}
										updated · {job.skipped_count} skipped · {job.error_count} errors</td
									><td>{at(job.created_at)}</td><td
										><a href={`/admin/crm/import-export/new?job=${encodeURIComponent(job.id)}`}
											>{job.status === 'pending' ? 'Review and apply' : 'View import'}</a
										></td
									></tr
								>{/each}</tbody
						>
					</table>
				</div>
				<nav aria-label="Import history pagination">
					<button
						type="button"
						onclick={() => changePage('import', -1)}
						disabled={busy || isLoading || importPage <= 1}>Previous imports</button
					><span>Page{importPage} of{importPages} · {importTotal ?? '—'} imports</span><button
						type="button"
						onclick={() => changePage('import', 1)}
						disabled={busy || isLoading || importPage >= importPages}>Next imports</button
					>
				</nav>{:else}<p>Your role does not include import history.</p>{/if}
		</section>
	{:else}
		<section>
			<h2>Export Data</h2>
			<div class="export-controls">
				<label
					>File format<select bind:value={format} disabled={busy}
						><option value="csv">CSV</option><option value="json">JSON</option></select
					></label
				><label>Contact search<input bind:value={search} maxlength="500" disabled={busy} /></label
				><label
					>Contact status<select bind:value={status} disabled={busy}
						><option value="">All statuses</option
						>{#each ['lead', 'prospect', 'customer', 'churned', 'unqualified', 'new', 'contacted', 'qualified', 'proposal', 'negotiation', 'won', 'lost'] as value (value)}<option
								{value}>{value}</option
							>{/each}</select
					></label
				>
			</div>
			<fieldset disabled={busy}>
				<legend>Contact fields</legend>{#each contactExportFields as field (field)}<label
						class="checkbox"
						><input type="checkbox" value={field} bind:group={fields} />{field.replaceAll(
							'_',
							' '
						)}</label
					>{/each}
			</fieldset>
			<p>
				Exports contain at most1000matching contacts and8MiB. Narrow the search when the selection
				exceeds either limit.
			</p>
			<div class="type-grid">
				{#each importTypes as item (item.type)}<button
						class="type-card"
						type="button"
						onclick={() => startExport(item.type)}
						disabled={busy || !access.can('POST', '/exports')}
						><item.icon size={24} /><span
							><strong>{item.name}</strong><small>Export {item.name.toLowerCase()}</small></span
						></button
					>{/each}
			</div>
		</section>
		<section>
			<h2>Recent Exports</h2>
			{#if access.can('GET', '/exports')}{#if isLoading}<p>
						Loading export history…
					</p>{:else if exportTotal === 0}<p>No recorded exports.</p>{/if}
				<div class="jobs-table">
					<table>
						<thead
							><tr
								><th>Type / format</th><th>Records</th><th>Status</th><th>Created</th><th>Action</th
								></tr
							></thead
						><tbody
							>{#each exportJobs as job (job.id)}<tr data-export-id={job.id}
									><td>{job.type} / {job.file_type.toUpperCase()}</td><td>{job.total_records}</td
									><td>{job.status}</td><td>{at(job.created_at)}</td><td
										><button
											type="button"
											onclick={() => downloadExport(job)}
											disabled={busy || !access.can('GET', '/exports/{id}/download')}
											>Download {job.file_type.toUpperCase()}</button
										></td
									></tr
								>{/each}</tbody
						>
					</table>
				</div>
				<nav aria-label="Export history pagination">
					<button
						type="button"
						onclick={() => changePage('export', -1)}
						disabled={busy || isLoading || exportPage <= 1}>Previous exports</button
					><span>Page{exportPage} of{exportPages} · {exportTotal ?? '—'} exports</span><button
						type="button"
						onclick={() => changePage('export', 1)}
						disabled={busy || isLoading || exportPage >= exportPages}>Next exports</button
					>
				</nav>{:else}<p>Your role does not include export history.</p>{/if}
		</section>
	{/if}
</div>

<style>
	.import-export-page {
		max-width: 1200px;
		color: var(--text-primary, #f0f6fc);
	}
	h1 {
		font-size: 1.8rem;
		margin: 0;
	}
	h2 {
		font-size: 1.25rem;
	}
	p,
	small {
		color: var(--text-secondary, #b1bac4);
		line-height: 1.6;
	}
	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}
	.tabs {
		display: flex;
		gap: 0.5rem;
		border-bottom: 1px solid #30363d;
		padding: 1rem 0;
	}
	.tabs button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
	.active {
		border-color: #e6b800;
		color: #e6b800;
	}
	section {
		margin: 1.5rem 0;
	}
	.type-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(230px, 1fr));
		gap: 1rem;
	}
	.type-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.2rem;
		text-decoration: none;
		text-align: left;
		background: var(--bg-elevated, #161b22);
		border: 1px solid #30363d;
		border-radius: 10px;
		color: inherit;
	}
	.type-card:hover {
		border-color: #e6b800;
	}
	.type-card span {
		display: grid;
		gap: 0.3rem;
	}
	button,
	input,
	select {
		font: inherit;
		padding: 0.6rem;
		border: 1px solid #30363d;
		border-radius: 7px;
		background: #0d1117;
		color: #f0f6fc;
	}
	button {
		cursor: pointer;
	}
	button:disabled {
		opacity: 0.6;
		cursor: wait;
	}
	a {
		color: #e6b800;
	}
	.jobs-table {
		overflow: auto;
	}
	table {
		width: 100%;
		border-collapse: collapse;
		background: #161b22;
	}
	th,
	td {
		padding: 0.9rem;
		text-align: left;
		border-bottom: 1px solid #30363d;
		vertical-align: top;
	}
	nav {
		display: flex;
		align-items: center;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 1rem;
	}
	.export-controls {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}
	label {
		display: grid;
		gap: 0.4rem;
	}
	fieldset {
		border: 1px solid #30363d;
		border-radius: 9px;
		margin-top: 1rem;
	}
	.checkbox {
		display: inline-flex;
		align-items: center;
		margin: 0.5rem;
	}
	.scope {
		font-size: 0.9rem;
	}
	[role='alert'] {
		border: 1px solid #f85149;
		padding: 1rem;
		margin: 1rem 0;
		border-radius: 8px;
	}
	[role='status'] {
		color: #e6b800;
	}
	@media (max-width: 650px) {
		.page-header,
		nav {
			flex-wrap: wrap;
			justify-content: flex-start;
		}
	}
</style>

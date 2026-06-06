<!--
	URL: /admin/forms/entries
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Table, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import {
		getForms,
		getSubmissions,
		exportSubmissions,
		type Form,
		type FormEntry
	} from '$lib/api/forms';
	import { IconDownload, IconEye } from '$lib/icons';

	let forms = $state<Form[]>([]);
	let entries = $state<FormEntry[]>([]);
	let selectedFormId = $state<number | null>(null);
	let loading = $state(true);

	onMount(async () => {
		await loadForms();
	});

	async function loadForms() {
		try {
			const response = await getForms();
			forms = response.forms || [];
			const firstForm = forms[0];
			if (forms.length > 0 && firstForm?.id) {
				selectedFormId = firstForm.id;
				await loadEntries(firstForm.id);
			}
		} catch (_error) {
			addToast({ type: 'error', message: 'Failed to load forms' });
		} finally {
			loading = false;
		}
	}

	async function loadEntries(formId: number) {
		try {
			loading = true;
			const response = await getSubmissions(formId);
			// Map submissions to entries format
			entries = (response.submissions || []).map((s) => ({
				id: s.id ?? 0,
				form_id: s.form_id ?? formId,
				data: s.data || {},
				created_at: s.created_at ?? ''
			}));
		} catch (_error) {
			addToast({ type: 'error', message: 'Failed to load entries' });
			entries = [];
		} finally {
			loading = false;
		}
	}

	async function handleFormChange() {
		if (selectedFormId) {
			await loadEntries(selectedFormId);
		}
	}

	async function handleExport() {
		if (!selectedFormId) return;

		try {
			const blob = await exportSubmissions(selectedFormId, 'csv');
			// Create download link
			const url = window.URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `form-entries-${selectedFormId}.csv`;
			a.click();
			window.URL.revokeObjectURL(url);
			addToast({ type: 'success', message: 'Entries exported successfully' });
		} catch (_error) {
			addToast({ type: 'error', message: 'Failed to export entries' });
		}
	}

	let formOptions = $derived(
		forms.map((form) => ({
			value: form.id,
			label: form.title
		}))
	);
</script>

<svelte:head>
	<title>Form Entries | Revolution Admin</title>
</svelte:head>

<div class="form-entries-page">
	<!-- Header -->
	<div class="page-header">
		<div>
			<h1>Form Entries</h1>
			<p>View and manage form submissions</p>
		</div>
		<Button onclick={handleExport} disabled={!selectedFormId || entries.length === 0}>
			<IconDownload size={20} />
			Export CSV
		</Button>
	</div>

	<!-- Form Selector -->
	<div class="form-selector">
		<Select
			label="Select Form"
			options={formOptions}
			bind:value={selectedFormId}
			onchange={handleFormChange}
			placeholder="Choose a form..."
		/>
	</div>

	<!-- Entries List -->
	{#if loading}
		<Card>
			<div class="empty-state">
				<div class="loading-spinner"></div>
				<p>Loading entries...</p>
			</div>
		</Card>
	{:else if !selectedFormId}
		<Card>
			<div class="empty-state">
				<p>Please select a form to view entries</p>
			</div>
		</Card>
	{:else if entries.length === 0}
		<Card>
			<div class="empty-state">
				<p>No entries found for this form</p>
			</div>
		</Card>
	{:else}
		<Card padding={false}>
			<Table headers={['ID', 'Status', 'Submitted', 'Preview', 'Actions']}>
				{#each entries as entry (entry.id)}
					<tr>
						<td class="entry-id">#{entry.id}</td>
						<td>
							<Badge variant={entry.status === 'complete' ? 'success' : 'warning'}>
								{entry.status}
							</Badge>
						</td>
						<td>{new Date(entry.created_at).toLocaleString()}</td>
						<td class="entry-preview">
							{JSON.stringify(entry.data).substring(0, 100)}...
						</td>
						<td>
							<a
								class="entry-action"
								href="/admin/forms/{selectedFormId}/entries/{entry.id}"
								title="View Entry"
							>
								<IconEye size={18} />
							</a>
						</td>
					</tr>
				{/each}
			</Table>
		</Card>
	{/if}
</div>

<style>
	.form-entries-page {
		color: #111827;
	}

	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.page-header h1 {
		margin: 0;
		color: #111827;
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.2;
	}

	.page-header p {
		margin: 0.25rem 0 0;
		color: #4b5563;
	}

	.form-selector {
		max-width: 28rem;
		margin-bottom: 1.5rem;
	}

	.empty-state {
		padding: 3rem 1rem;
		text-align: center;
		color: #6b7280;
	}

	.empty-state p {
		margin: 0;
	}

	.empty-state .loading-spinner + p {
		margin-top: 1rem;
		color: #4b5563;
	}

	.loading-spinner {
		width: 3rem;
		height: 3rem;
		margin: 0 auto;
		border: 2px solid transparent;
		border-bottom-color: #2563eb;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.entry-id {
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
	}

	.entry-preview {
		max-width: 20rem;
		overflow: hidden;
		color: #4b5563;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.entry-action {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.5rem;
		border-radius: 0.25rem;
		color: #2563eb;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
	}

	.entry-action:hover {
		background: #eff6ff;
	}

	.entry-action:focus-visible {
		outline: 2px solid #2563eb;
		outline-offset: 2px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 640px) {
		.page-header {
			align-items: flex-start;
			flex-direction: column;
		}
	}
</style>

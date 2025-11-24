<script lang="ts">
	import { onMount } from 'svelte';
	import { Card, Button, Badge, Table, Select } from '$lib/components/ui';
	import { addToast } from '$lib/utils/toast';
	import { getForms, getSubmissions, exportSubmissions, type Form, type FormEntry } from '$lib/api/forms';
	import { IconDownload, IconEye } from '@tabler/icons-svelte';

	let forms: Form[] = [];
	let entries: FormSubmission[] = [];
	let selectedFormId: number | null = null;
	let loading = true;

	onMount(async () => {
		await loadForms();
	});

	async function loadForms() {
		try {
			const response = await getForms();
			forms = response.forms || [];
			if (forms.length > 0 && forms[0].id) {
				selectedFormId = forms[0].id;
				await loadEntries(forms[0].id);
			}
		} catch (error) {
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
		} catch (error) {
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
		} catch (error) {
			addToast({ type: 'error', message: 'Failed to export entries' });
		}
	}

	$: formOptions = forms.map((form) => ({
		value: form.id,
		label: form.title
	}));
</script>

<svelte:head>
	<title>Form Entries | Revolution Admin</title>
</svelte:head>

<div>
	<!-- Header -->
	<div class="flex items-center justify-between mb-6">
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Form Entries</h1>
			<p class="text-gray-600 mt-1">View and manage form submissions</p>
		</div>
		<Button on:click={handleExport} disabled={!selectedFormId || entries.length === 0}>
			<IconDownload size={20} />
			Export CSV
		</Button>
	</div>

	<!-- Form Selector -->
	<div class="mb-6 max-w-md">
		<Select
			label="Select Form"
			options={formOptions}
			bind:value={selectedFormId}
			on:change={handleFormChange}
			placeholder="Choose a form..."
		/>
	</div>

	<!-- Entries List -->
	{#if loading}
		<Card>
			<div class="text-center py-12">
				<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
				<p class="mt-4 text-gray-600">Loading entries...</p>
			</div>
		</Card>
	{:else if !selectedFormId}
		<Card>
			<div class="text-center py-12">
				<p class="text-gray-500">Please select a form to view entries</p>
			</div>
		</Card>
	{:else if entries.length === 0}
		<Card>
			<div class="text-center py-12">
				<p class="text-gray-500">No entries found for this form</p>
			</div>
		</Card>
	{:else}
		<Card padding={false}>
			<Table headers={['ID', 'Status', 'Submitted', 'Preview', 'Actions']}>
				{#each entries as entry}
					<tr>
						<td class="font-mono">#{entry.id}</td>
						<td>
							<Badge variant={entry.status === 'complete' ? 'success' : 'warning'}>
								{entry.status}
							</Badge>
						</td>
						<td>{new Date(entry.created_at).toLocaleString()}</td>
						<td class="max-w-xs truncate text-gray-600">
							{JSON.stringify(entry.data).substring(0, 100)}...
						</td>
						<td>
							<button
								class="p-2 text-blue-600 hover:bg-blue-50 rounded transition-colors"
								title="View Entry"
							>
								<a href="/admin/forms/{selectedFormId}/entries/{entry.id}">
									<IconEye size={18} />
								</a>
							</button>
						</td>
					</tr>
				{/each}
			</Table>
		</Card>
	{/if}
</div>

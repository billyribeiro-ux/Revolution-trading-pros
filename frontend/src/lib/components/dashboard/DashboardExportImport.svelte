<script lang="ts">
	import { apiClient } from '$lib/api/client';

	interface Props {
		dashboardId: string;
		onexported?: (data: { filename: string }) => void;
		onimported?: (data: { dashboardId: string }) => void;
	}

	let { dashboardId, onexported, onimported }: Props = $props();

	let isExporting = $state(false);
	let isImporting = $state(false);
	let exportFormat: 'json' | 'pdf' | 'csv' = $state('json');
	let includeData = $state(true);
	let importFile: File | null = $state(null);
	let overwrite = $state(false);
	let error: string | null = $state(null);
	let success: string | null = $state(null);

	async function handleExport() {
		isExporting = true;
		error = null;
		success = null;

		try {
			const response = (await apiClient.post(`/dashboards/${dashboardId}/export`, {
				format: exportFormat,
				include_data: includeData
			})) as { data: { filename: string; url: string } };

			// Download file
			const link = document.createElement('a');
			link.href = response.data.url;
			link.download = response.data.filename;
			link.click();

			success = 'Dashboard exported successfully!';
			onexported?.({ filename: response.data.filename });
		} catch (err: any) {
			error = err.message || 'Failed to export dashboard';
		} finally {
			isExporting = false;
		}
	}

	async function handleImport() {
		if (!importFile) {
			error = 'Please select a file to import';
			return;
		}

		isImporting = true;
		error = null;
		success = null;

		try {
			const formData = new FormData();
			formData.append('file', importFile);
			formData.append('overwrite', overwrite.toString());

			const response = (await apiClient.post('/dashboards/import', formData, {
				headers: {
					'Content-Type': 'multipart/form-data'
				}
			})) as { data: { dashboard_id: string } };

			success = 'Dashboard imported successfully!';
			onimported?.({ dashboardId: response.data.dashboard_id });
		} catch (err: any) {
			error = err.message || 'Failed to import dashboard';
		} finally {
			isImporting = false;
		}
	}

	function handleFileSelect(event: Event) {
		const target = event.target as HTMLInputElement;
		importFile = target.files?.[0] || null;
	}
</script>

<div class="export-import-panel">
	<div class="section">
		<h3>Export Dashboard</h3>
		<p class="description">Download your dashboard configuration</p>

		<div class="form-group">
			<label for="export-format">Format</label>
			<select id="export-format" bind:value={exportFormat}>
				<option value="json">JSON</option>
				<option value="pdf">PDF</option>
				<option value="csv">CSV</option>
			</select>
		</div>

		<div class="form-group">
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={includeData} />
				<span>Include widget data</span>
			</label>
		</div>

		<button class="btn-primary" onclick={handleExport} disabled={isExporting}>
			{isExporting ? 'Exporting...' : 'Export Dashboard'}
		</button>
	</div>

	<div class="divider"></div>

	<div class="section">
		<h3>Import Dashboard</h3>
		<p class="description">Upload a dashboard configuration file</p>

		<div class="form-group">
			<label for="import-file">Select File</label>
			<input id="import-file" type="file" accept=".json" onchange={handleFileSelect} />
			{#if importFile}
				<p class="file-info">{importFile.name} ({(importFile.size / 1024).toFixed(2)} KB)</p>
			{/if}
		</div>

		<div class="form-group">
			<label class="checkbox-label">
				<input type="checkbox" bind:checked={overwrite} />
				<span>Overwrite existing dashboard</span>
			</label>
		</div>

		<button class="btn-primary" onclick={handleImport} disabled={isImporting || !importFile}>
			{isImporting ? 'Importing...' : 'Import Dashboard'}
		</button>
	</div>

	{#if error}
		<div class="alert alert-error">
			{error}
		</div>
	{/if}

	{#if success}
		<div class="alert alert-success">
			{success}
		</div>
	{/if}
</div>

<style>
	.export-import-panel {
		background: white;
		border-radius: 12px;
		padding: 2rem;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.section {
		margin-bottom: 2rem;
	}

	.section:last-of-type {
		margin-bottom: 0;
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1f2937;
		margin: 0 0 0.5rem 0;
	}

	.description {
		color: #6b7280;
		font-size: 0.875rem;
		margin: 0 0 1.5rem 0;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.5rem;
	}

	select,
	input[type='file'] {
		width: 100%;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: auto;
	}

	.file-info {
		margin-top: 0.5rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.btn-primary {
		width: 100%;
		padding: 0.75rem 1.5rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.divider {
		height: 1px;
		background: #e5e7eb;
		margin: 2rem 0;
	}

	.alert {
		padding: 1rem;
		border-radius: 8px;
		margin-top: 1.5rem;
		font-size: 0.875rem;
	}

	.alert-error {
		background: #fee2e2;
		color: #991b1b;
		border: 1px solid #fecaca;
	}

	.alert-success {
		background: #d1fae5;
		color: #065f46;
		border: 1px solid #a7f3d0;
	}
</style>

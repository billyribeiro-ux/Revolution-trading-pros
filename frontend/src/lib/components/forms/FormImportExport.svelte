<script lang="ts">
	/**
	 * Form Import/Export - Multi-format data interchange UI
	 *
	 * Features:
	 * - JSON/CSV export
	 * - Form structure export
	 * - Submissions export
	 * - Import from various sources
	 * - Template save/load
	 *
	 * @version 1.0.0
	 */

	import { getAuthToken } from '$lib/stores/auth';

	interface Props {
		formId?: number;
		formTitle?: string;
		onImportComplete?: (result: any) => void;
	}

	let { formId, formTitle, onImportComplete }: Props = $props();

	// State
	let activeTab = $state<'export' | 'import'>('export');
	let exportFormat = $state('json');
	let exportType = $state('form');
	let includeSubmissions = $state(false);
	let exporting = $state(false);
	let importing = $state(false);
	let importSource = $state('json');
	let importFile = $state<File | null>(null);
	let importResult = $state<{ success: boolean; message: string } | null>(null);
	let dateFrom = $state('');
	let dateTo = $state('');

	// Export formats
	const exportFormats = [
		{ value: 'json', label: 'JSON', description: 'Full form structure', icon: '{ }' },
		{ value: 'csv', label: 'CSV', description: 'Submissions data', icon: '📊' },
		{ value: 'xlsx', label: 'Excel', description: 'Spreadsheet format', icon: '📗' }
	];

	// Import sources
	const importSources = [
		{ value: 'json', label: 'JSON File', description: 'Revolution Forms format' },
		{ value: 'gravity', label: 'Gravity Forms', description: 'WordPress plugin export' },
		{ value: 'typeform', label: 'Typeform', description: 'Typeform JSON export' },
		{ value: 'template', label: 'Template', description: 'Load from template library' }
	];

	// Handle file selection
	function handleFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files?.length) {
			importFile = input.files[0];
		}
	}

	// Export form/submissions
	async function handleExport() {
		if (!formId && exportType !== 'template') return;

		exporting = true;

		try {
			const token = getAuthToken();
			const params = new URLSearchParams({
				format: exportFormat,
				type: exportType,
				include_submissions: includeSubmissions.toString()
			});

			if (dateFrom) params.append('date_from', dateFrom);
			if (dateTo) params.append('date_to', dateTo);

			const response = await fetch(`/api/forms/${formId}/export?${params}`, {
				headers: { Authorization: `Bearer ${token}` }
			});

			if (response.ok) {
				const blob = await response.blob();
				const filename = response.headers.get('Content-Disposition')?.match(/filename="(.+)"/)?.[1] || `form-export.${exportFormat}`;

				// Download file
				const url = window.URL.createObjectURL(blob);
				const a = document.createElement('a');
				a.href = url;
				a.download = filename;
				document.body.appendChild(a);
				a.click();
				document.body.removeChild(a);
				window.URL.revokeObjectURL(url);
			}
		} catch (error) {
			console.error('Export failed:', error);
		}

		exporting = false;
	}

	// Import form
	async function handleImport() {
		if (!importFile) return;

		importing = true;
		importResult = null;

		try {
			const token = getAuthToken();
			const formData = new FormData();
			formData.append('file', importFile);
			formData.append('source', importSource);

			const response = await fetch('/api/forms/import', {
				method: 'POST',
				headers: { Authorization: `Bearer ${token}` },
				body: formData
			});

			const data = await response.json();

			if (response.ok) {
				importResult = {
					success: true,
					message: `Successfully imported form with ${data.fields_created || 0} fields`
				};
				onImportComplete?.(data);
			} else {
				importResult = {
					success: false,
					message: data.error || 'Import failed'
				};
			}
		} catch (error) {
			importResult = {
				success: false,
				message: 'Failed to process import'
			};
			console.error('Import failed:', error);
		}

		importing = false;
	}

	// Save as template
	async function saveAsTemplate() {
		if (!formId) return;

		const templateName = prompt('Enter template name:');
		if (!templateName) return;

		try {
			const token = getAuthToken();
			const response = await fetch(`/api/forms/${formId}/export?format=template`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Authorization: `Bearer ${token}`
				},
				body: JSON.stringify({
					name: templateName,
					category: 'custom'
				})
			});

			if (response.ok) {
				alert('Template saved successfully!');
			}
		} catch (error) {
			console.error('Failed to save template:', error);
		}
	}
</script>

<div class="import-export">
	<div class="panel-header">
		<h3>Import & Export</h3>
	</div>

	<!-- Tabs -->
	<div class="tabs">
		<button class="tab" class:active={activeTab === 'export'} onclick={() => (activeTab = 'export')}>
			Export
		</button>
		<button class="tab" class:active={activeTab === 'import'} onclick={() => (activeTab = 'import')}>
			Import
		</button>
	</div>

	<div class="panel-content">
		<!-- Export Tab -->
		{#if activeTab === 'export'}
			<div class="export-section">
				<!-- Export Type -->
				<div class="option-group">
					<label class="option-label">What to export</label>
					<div class="radio-group">
						<label class="radio-option">
							<input type="radio" bind:group={exportType} value="form" />
							<span class="radio-content">
								<span class="radio-title">Form Structure</span>
								<span class="radio-desc">Export form fields and settings</span>
							</span>
						</label>
						<label class="radio-option">
							<input type="radio" bind:group={exportType} value="submissions" />
							<span class="radio-content">
								<span class="radio-title">Submissions</span>
								<span class="radio-desc">Export all form submissions</span>
							</span>
						</label>
						<label class="radio-option">
							<input type="radio" bind:group={exportType} value="all" />
							<span class="radio-content">
								<span class="radio-title">Everything</span>
								<span class="radio-desc">Form structure + all submissions</span>
							</span>
						</label>
					</div>
				</div>

				<!-- Export Format -->
				<div class="option-group">
					<label class="option-label">Format</label>
					<div class="format-options">
						{#each exportFormats as format}
							<button
								class="format-btn"
								class:selected={exportFormat === format.value}
								onclick={() => (exportFormat = format.value)}
								disabled={exportType !== 'form' && format.value === 'json'}
							>
								<span class="format-icon">{format.icon}</span>
								<span class="format-label">{format.label}</span>
								<span class="format-desc">{format.description}</span>
							</button>
						{/each}
					</div>
				</div>

				<!-- Date Range (for submissions) -->
				{#if exportType === 'submissions' || exportType === 'all'}
					<div class="option-group">
						<label class="option-label">Date Range (optional)</label>
						<div class="date-range">
							<input type="date" bind:value={dateFrom} class="date-input" placeholder="From" />
							<span class="date-separator">to</span>
							<input type="date" bind:value={dateTo} class="date-input" placeholder="To" />
						</div>
					</div>
				{/if}

				<!-- Export Button -->
				<button class="btn-primary" onclick={handleExport} disabled={exporting || !formId}>
					{#if exporting}
						<span class="spinner"></span>
						Exporting...
					{:else}
						Export {exportType === 'form' ? 'Form' : exportType === 'submissions' ? 'Submissions' : 'All'}
					{/if}
				</button>

				<!-- Save as Template -->
				{#if formId}
					<div class="template-section">
						<p class="template-hint">Save this form as a reusable template</p>
						<button class="btn-secondary" onclick={saveAsTemplate}> Save as Template </button>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Import Tab -->
		{#if activeTab === 'import'}
			<div class="import-section">
				<!-- Import Source -->
				<div class="option-group">
					<label class="option-label">Import from</label>
					<div class="source-options">
						{#each importSources as source}
							<button
								class="source-btn"
								class:selected={importSource === source.value}
								onclick={() => (importSource = source.value)}
							>
								<span class="source-label">{source.label}</span>
								<span class="source-desc">{source.description}</span>
							</button>
						{/each}
					</div>
				</div>

				<!-- File Upload -->
				{#if importSource !== 'template'}
					<div class="option-group">
						<label class="option-label">Select File</label>
						<div class="file-upload">
							<input
								type="file"
								accept={importSource === 'csv' ? '.csv' : '.json'}
								onchange={handleFileSelect}
								id="import-file"
								class="file-input"
							/>
							<label for="import-file" class="file-label">
								{#if importFile}
									<span class="file-name">{importFile.name}</span>
									<span class="file-size">({(importFile.size / 1024).toFixed(1)} KB)</span>
								{:else}
									<span class="file-placeholder">Click to select or drag file here</span>
								{/if}
							</label>
						</div>
					</div>
				{/if}

				<!-- Import Result -->
				{#if importResult}
					<div class="import-result" class:success={importResult.success} class:error={!importResult.success}>
						{#if importResult.success}
							<span class="result-icon">✓</span>
						{:else}
							<span class="result-icon">✕</span>
						{/if}
						<span class="result-message">{importResult.message}</span>
					</div>
				{/if}

				<!-- Import Button -->
				<button class="btn-primary" onclick={handleImport} disabled={importing || (!importFile && importSource !== 'template')}>
					{#if importing}
						<span class="spinner"></span>
						Importing...
					{:else}
						Import Form
					{/if}
				</button>

				<!-- Import Help -->
				<div class="import-help">
					<h4>Supported formats:</h4>
					<ul>
						<li><strong>JSON:</strong> Revolution Forms native format</li>
						<li><strong>Gravity Forms:</strong> Export from WordPress</li>
						<li><strong>Typeform:</strong> Typeform JSON export</li>
					</ul>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.import-export {
		background: white;
		border-radius: 0.75rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		overflow: hidden;
	}

	.panel-header {
		padding: 1.25rem;
		border-bottom: 1px solid #e5e7eb;
	}

	.panel-header h3 {
		margin: 0;
		font-size: 1.125rem;
		font-weight: 600;
		color: #111827;
	}

	.tabs {
		display: flex;
		border-bottom: 1px solid #e5e7eb;
	}

	.tab {
		flex: 1;
		padding: 0.75rem;
		background: none;
		border: none;
		font-size: 0.875rem;
		font-weight: 500;
		color: #6b7280;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #374151;
		background: #f9fafb;
	}

	.tab.active {
		color: #3b82f6;
		border-bottom-color: #3b82f6;
	}

	.panel-content {
		padding: 1.25rem;
	}

	.option-group {
		margin-bottom: 1.5rem;
	}

	.option-label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		margin-bottom: 0.75rem;
	}

	.radio-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.radio-option {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 0.75rem;
		background: #f9fafb;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.radio-option:hover {
		background: #f3f4f6;
	}

	.radio-option input[type='radio'] {
		margin-top: 0.25rem;
	}

	.radio-content {
		display: flex;
		flex-direction: column;
	}

	.radio-title {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.radio-desc {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.format-options {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 0.75rem;
	}

	.format-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1rem;
		background: #f9fafb;
		border: 2px solid #e5e7eb;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.format-btn:hover:not(:disabled) {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.format-btn.selected {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.format-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.format-icon {
		font-size: 1.5rem;
		margin-bottom: 0.5rem;
	}

	.format-label {
		font-size: 0.875rem;
		font-weight: 600;
		color: #374151;
	}

	.format-desc {
		font-size: 0.625rem;
		color: #6b7280;
		margin-top: 0.25rem;
	}

	.source-options {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 0.75rem;
	}

	.source-btn {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		padding: 0.75rem;
		background: #f9fafb;
		border: 2px solid #e5e7eb;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
		text-align: left;
	}

	.source-btn:hover {
		border-color: #3b82f6;
	}

	.source-btn.selected {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.source-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.source-desc {
		font-size: 0.6875rem;
		color: #6b7280;
		margin-top: 0.125rem;
	}

	.date-range {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.date-input {
		flex: 1;
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 0.375rem;
		font-size: 0.875rem;
	}

	.date-separator {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.file-upload {
		position: relative;
	}

	.file-input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	.file-label {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		border: 2px dashed #d1d5db;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.file-label:hover {
		border-color: #3b82f6;
		background: #f9fafb;
	}

	.file-name {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.file-size {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.file-placeholder {
		font-size: 0.875rem;
		color: #6b7280;
	}

	.btn-primary {
		width: 100%;
		padding: 0.75rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		transition: background 0.2s;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		background: #9ca3af;
		cursor: not-allowed;
	}

	.btn-secondary {
		width: 100%;
		padding: 0.5rem 1rem;
		background: white;
		color: #3b82f6;
		border: 1px solid #3b82f6;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-secondary:hover {
		background: #eff6ff;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.template-section {
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid #e5e7eb;
	}

	.template-hint {
		font-size: 0.8125rem;
		color: #6b7280;
		margin: 0 0 0.75rem 0;
	}

	.import-result {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem;
		border-radius: 0.375rem;
		margin-bottom: 1rem;
	}

	.import-result.success {
		background: #dcfce7;
		color: #166534;
	}

	.import-result.error {
		background: #fee2e2;
		color: #991b1b;
	}

	.result-icon {
		font-weight: bold;
	}

	.result-message {
		font-size: 0.875rem;
	}

	.import-help {
		margin-top: 1.5rem;
		padding: 1rem;
		background: #f9fafb;
		border-radius: 0.5rem;
	}

	.import-help h4 {
		margin: 0 0 0.5rem 0;
		font-size: 0.8125rem;
		font-weight: 600;
		color: #374151;
	}

	.import-help ul {
		margin: 0;
		padding-left: 1.25rem;
		font-size: 0.75rem;
		color: #6b7280;
	}

	.import-help li {
		margin-bottom: 0.25rem;
	}

	.import-help strong {
		color: #374151;
	}

	@media (max-width: 640px) {
		.format-options,
		.source-options {
			grid-template-columns: 1fr;
		}
	}
</style>

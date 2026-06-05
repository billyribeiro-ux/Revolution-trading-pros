<script lang="ts">
	import { onDestroy } from 'svelte';
	import { boardsAPI } from '$lib/api/boards';
	import type { ImportJob, ImportSource } from '$lib/boards/types';
	import {
		IconUpload,
		IconArrowLeft,
		IconBrandTrello,
		IconBrandAsana,
		IconFileSpreadsheet,
		IconCheck,
		IconX,
		IconLoader,
		IconAlertCircle
	} from '$lib/icons';

	// State
	let selectedSource = $state<ImportSource | null>(null);
	let file = $state<File | null>(null);
	let importJob = $state<ImportJob | null>(null);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let dragOver = $state(false);

	const sources = [
		{
			id: 'trello' as ImportSource,
			name: 'Trello',
			description: 'Import boards, lists, and cards from Trello JSON export',
			icon: IconBrandTrello,
			tone: 'blue',
			accept: '.json'
		},
		{
			id: 'asana' as ImportSource,
			name: 'Asana',
			description: 'Import projects and tasks from Asana CSV export',
			icon: IconBrandAsana,
			tone: 'orange',
			accept: '.csv'
		},
		{
			id: 'csv' as ImportSource,
			name: 'CSV File',
			description: 'Import tasks from a CSV file with custom mapping',
			icon: IconFileSpreadsheet,
			tone: 'green',
			accept: '.csv'
		}
	];

	function handleFileSelect(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files[0]) {
			file = input.files[0];
		}
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		dragOver = false;
		if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
			file = event.dataTransfer.files[0];
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		dragOver = true;
	}

	function handleDragLeave() {
		dragOver = false;
	}

	async function startImport() {
		if (!selectedSource || !file) return;

		loading = true;
		error = null;

		try {
			switch (selectedSource) {
				case 'trello':
					importJob = await boardsAPI.importFromTrello(file);
					break;
				case 'asana':
					importJob = await boardsAPI.importFromAsana(file);
					break;
				case 'csv':
					importJob = await boardsAPI.importFromCsv(file);
					break;
			}

			// Poll for status updates
			pollImportStatus();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to start import';
		} finally {
			loading = false;
		}
	}

	// FIX-2026-04-26 (P1-5): track the in-flight setTimeout so we can cancel on
	// unmount. Previously, navigating away during an import left poll() recursing
	// forever, calling `importJob = job` against a defunct component (Svelte 5
	// raises `state_unsafe_mutation` warnings).
	let pollTimer: ReturnType<typeof setTimeout> | null = null;
	let pollCancelled = false;

	async function pollImportStatus() {
		if (!importJob) return;
		pollCancelled = false;

		const poll = async () => {
			if (pollCancelled || !importJob) return;
			try {
				const job = await boardsAPI.getImportJob(importJob.id);
				if (pollCancelled) return;
				importJob = job;

				if (job.status === 'processing' || job.status === 'pending') {
					pollTimer = setTimeout(poll, 2000);
				}
			} catch (err) {
				console.error('Failed to poll import status:', err);
			}
		};

		poll();
	}

	onDestroy(() => {
		pollCancelled = true;
		if (pollTimer) {
			clearTimeout(pollTimer);
			pollTimer = null;
		}
	});

	function resetImport() {
		selectedSource = null;
		file = null;
		importJob = null;
		error = null;
	}

	function getStatusClass(status: string): Record<string, boolean> {
		return {
			'status-pill': true,
			'status-pill--success': status === 'completed',
			'status-pill--danger': status === 'failed',
			'status-pill--info': status === 'processing',
			'status-pill--neutral':
				status !== 'completed' && status !== 'failed' && status !== 'processing'
		};
	}

	function getProgressPercent(job: ImportJob): number {
		return job.total_items > 0 ? (job.processed_items / job.total_items) * 100 : 0;
	}
</script>

<svelte:head>
	<title>Import Board | Project Boards</title>
</svelte:head>

<div class="import-page">
	<!-- Header -->
	<div class="page-header">
		<div class="page-container page-container--header">
			<div class="header-row">
				<a href="/admin/boards" class="back-link" aria-label="Back to boards">
					<IconArrowLeft size={20} aria-hidden="true" />
				</a>
				<div class="title-row">
					<div class="title-icon">
						<IconUpload size={24} aria-hidden="true" />
					</div>
					<div>
						<h1>Import Board</h1>
						<p>Import from Trello, Asana, or CSV</p>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="page-container page-container--body">
		{#if importJob}
			<!-- Import Progress -->
			<div class="panel">
				<div class="panel-header">
					<h2>Import Progress</h2>
					<span class={getStatusClass(importJob.status)}>
						{importJob.status}
					</span>
				</div>

				{#if importJob.status === 'processing'}
					<div class="progress-section">
						<div class="progress-header">
							<span>Processing...</span>
							<span>
								{importJob.processed_items} / {importJob.total_items}
							</span>
						</div>
						<div class="progress-track">
							<div
								class="progress-bar"
								style:width="{getProgressPercent(importJob).toFixed(1)}%"
							></div>
						</div>
					</div>
				{/if}

				{#if importJob.status === 'completed'}
					<div class="result-state">
						<div class="result-icon result-icon--success">
							<IconCheck size={32} aria-hidden="true" />
						</div>
						<h3>Import Complete!</h3>
						<p>
							Successfully imported {importJob.processed_items} items
							{#if importJob.failed_items > 0}
								({importJob.failed_items} failed)
							{/if}
						</p>
						<div class="result-actions">
							<a href="/admin/boards" class="primary-action"> View Boards </a>
							<button type="button" onclick={resetImport} class="secondary-action">
								Import Another
							</button>
						</div>
					</div>
				{/if}

				{#if importJob.status === 'failed'}
					<div class="result-state">
						<div class="result-icon result-icon--danger">
							<IconX size={32} aria-hidden="true" />
						</div>
						<h3>Import Failed</h3>
						<p>
							{importJob.errors?.[0]?.message || 'An error occurred during import'}
						</p>
						<button type="button" onclick={resetImport} class="primary-action"> Try Again </button>
					</div>
				{/if}

				{#if importJob.errors && importJob.errors.length > 0 && importJob.status === 'completed'}
					<div class="warning-callout">
						<h4>
							{importJob.errors.length} items could not be imported:
						</h4>
						<ul>
							{#each importJob.errors.slice(0, 5) as err, i (i)}
								<li>• {err.message}</li>
							{/each}
							{#if importJob.errors.length > 5}
								<li class="warning-more">...and {importJob.errors.length - 5} more</li>
							{/if}
						</ul>
					</div>
				{/if}
			</div>
		{:else if selectedSource}
			<!-- File Upload -->
			<div class="panel">
				<button type="button" onclick={() => (selectedSource = null)} class="inline-back">
					<IconArrowLeft size={16} aria-hidden="true" />
					Back to source selection
				</button>

				<h2 class="panel-title">
					Upload {sources.find((s) => s.id === selectedSource)?.name} File
				</h2>
				<p class="panel-copy">
					{sources.find((s) => s.id === selectedSource)?.description}
				</p>

				<!-- Drop Zone -->
				<div
					role="region"
					aria-label="File drop zone"
					class={{ 'drop-zone': true, 'drop-zone--active': dragOver }}
					ondrop={handleDrop}
					ondragover={handleDragOver}
					ondragleave={handleDragLeave}
				>
					{#if file}
						<div class="selected-file">
							<IconFileSpreadsheet size={40} aria-hidden="true" />
							<div class="selected-file-details">
								<div class="selected-file-name">{file.name}</div>
								<div class="selected-file-size">{(file.size / 1024).toFixed(1)} KB</div>
							</div>
							<button
								type="button"
								onclick={() => (file = null)}
								class="clear-file"
								aria-label="Remove selected file"
							>
								<IconX size={20} aria-hidden="true" />
							</button>
						</div>
					{:else}
						<IconUpload class="drop-icon" size={40} aria-hidden="true" />
						<p class="drop-copy">Drag and drop your file here, or</p>
					{/if}

					<label class="file-picker">
						<span>
							{file ? 'Choose Different File' : 'Browse Files'}
						</span>
						<input
							type="file"
							accept={sources.find((s) => s.id === selectedSource)?.accept}
							onchange={handleFileSelect}
							class="visually-hidden"
						/>
					</label>
				</div>

				{#if error}
					<div class="error-callout">
						<IconAlertCircle size={20} aria-hidden="true" />
						<span>{error}</span>
					</div>
				{/if}

				<div class="upload-actions">
					<button
						type="button"
						onclick={startImport}
						disabled={!file || loading}
						class="primary-action primary-action--wide"
					>
						{#if loading}
							<IconLoader class="spin-icon" size={16} aria-hidden="true" />
							Importing...
						{:else}
							<IconUpload size={16} aria-hidden="true" />
							Start Import
						{/if}
					</button>
				</div>
			</div>
		{:else}
			<!-- Source Selection -->
			<div class="source-section">
				<h2>Choose Import Source</h2>

				{#each sources as source (source.id)}
					<button type="button" onclick={() => (selectedSource = source.id)} class="source-card">
						<div
							class={{
								'source-icon': true,
								'source-icon--blue': source.tone === 'blue',
								'source-icon--orange': source.tone === 'orange',
								'source-icon--green': source.tone === 'green'
							}}
						>
							<source.icon size={32} aria-hidden="true" />
						</div>
						<div>
							<h3>{source.name}</h3>
							<p>{source.description}</p>
						</div>
					</button>
				{/each}
			</div>

			<!-- Help Section -->
			<div class="help-panel">
				<h3>How to export from other tools:</h3>
				<ul>
					<li>
						<strong>Trello:</strong> Open board menu → More → Print and export → Export to JSON
					</li>
					<li><strong>Asana:</strong> Open project → ••• menu → Export/Print → CSV</li>
					<li>
						<strong>CSV:</strong> Create a CSV with columns: title, description, status, due_date, priority
					</li>
				</ul>
			</div>
		{/if}
	</div>
</div>

<style>
	.import-page {
		min-height: 100vh;
		background: #f9fafb;
		color: #111827;
	}

	.page-header {
		border-bottom: 1px solid #e5e7eb;
		background: #ffffff;
	}

	.page-container {
		max-width: 48rem;
		margin: 0 auto;
		padding-inline: 1rem;
	}

	.page-container--header {
		padding-block: 1.5rem;
	}

	.page-container--body {
		padding-block: 2rem;
	}

	.header-row,
	.title-row,
	.panel-header,
	.progress-header,
	.selected-file,
	.error-callout,
	.primary-action,
	.inline-back {
		display: flex;
		align-items: center;
	}

	.header-row {
		gap: 1rem;
	}

	.title-row {
		gap: 0.75rem;
	}

	.back-link,
	.title-icon,
	.result-icon,
	.source-icon {
		display: grid;
		flex: 0 0 auto;
		place-items: center;
	}

	.back-link {
		width: 2.25rem;
		height: 2.25rem;
		border-radius: 8px;
		color: #6b7280;
		text-decoration: none;
		transition:
			background-color 0.2s ease,
			color 0.2s ease;
	}

	.back-link:hover {
		background: #f3f4f6;
		color: #374151;
	}

	.title-icon {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 8px;
		background: #e0e7ff;
		color: #4f46e5;
	}

	h1,
	h2,
	h3,
	h4,
	p {
		margin-top: 0;
	}

	h1 {
		margin-bottom: 0;
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.25;
	}

	.title-row p,
	.panel-copy,
	.result-state p,
	.drop-copy,
	.selected-file-size,
	.source-card p {
		color: #6b7280;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.title-row p {
		margin-bottom: 0;
	}

	.panel {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		background: #ffffff;
		padding: 1.5rem;
	}

	.panel-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.panel-header h2,
	.panel-title,
	.source-section h2 {
		margin-bottom: 0;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.panel-copy {
		margin-bottom: 1.5rem;
	}

	.status-pill {
		border-radius: 999px;
		padding: 0.25rem 0.75rem;
		font-size: 0.875rem;
		line-height: 1.5;
		text-transform: capitalize;
	}

	.status-pill--success {
		background: #dcfce7;
		color: #16a34a;
	}

	.status-pill--danger {
		background: #fee2e2;
		color: #dc2626;
	}

	.status-pill--info {
		background: #dbeafe;
		color: #2563eb;
	}

	.status-pill--neutral {
		background: #f3f4f6;
		color: #4b5563;
	}

	.progress-section {
		margin-bottom: 1.5rem;
	}

	.progress-header {
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.5rem;
		color: #4b5563;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.progress-track {
		height: 0.5rem;
		overflow: hidden;
		border-radius: 999px;
		background: #e5e7eb;
	}

	.progress-bar {
		height: 100%;
		border-radius: inherit;
		background: #4f46e5;
		transition: width 0.2s ease;
	}

	.result-state {
		padding-block: 2rem;
		text-align: center;
	}

	.result-icon {
		width: 4rem;
		height: 4rem;
		margin: 0 auto 1rem;
		border-radius: 999px;
	}

	.result-icon--success {
		background: #dcfce7;
		color: #16a34a;
	}

	.result-icon--danger {
		background: #fee2e2;
		color: #dc2626;
	}

	.result-state h3 {
		margin-bottom: 0.5rem;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.result-state p {
		margin-bottom: 1.5rem;
	}

	.result-actions {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		gap: 1rem;
	}

	.primary-action,
	.secondary-action,
	.file-picker span {
		border: 0;
		border-radius: 8px;
		padding: 0.5rem 1rem;
		font: inherit;
		text-decoration: none;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			opacity 0.2s ease;
	}

	.primary-action,
	.file-picker span {
		justify-content: center;
		gap: 0.5rem;
		background: #4f46e5;
		color: #ffffff;
	}

	.primary-action:hover,
	.file-picker span:hover {
		background: #4338ca;
	}

	.primary-action:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.primary-action--wide {
		padding-inline: 1.5rem;
	}

	.secondary-action {
		background: transparent;
		color: #374151;
	}

	.secondary-action:hover {
		background: #f3f4f6;
	}

	.warning-callout,
	.error-callout,
	.help-panel {
		border: 1px solid #fde68a;
		border-radius: 8px;
		background: #fffbeb;
		padding: 1rem;
	}

	.warning-callout {
		margin-top: 1.5rem;
	}

	.warning-callout h4,
	.help-panel h3 {
		margin-bottom: 0.5rem;
		color: #854d0e;
		font-size: 0.875rem;
		font-weight: 700;
		line-height: 1.4;
	}

	.warning-callout ul,
	.help-panel ul {
		display: grid;
		gap: 0.25rem;
		margin: 0;
		padding: 0;
		color: #a16207;
		font-size: 0.875rem;
		line-height: 1.6;
		list-style: none;
	}

	.warning-more {
		color: #ca8a04;
	}

	.inline-back {
		gap: 0.25rem;
		margin-bottom: 1rem;
		border: 0;
		background: transparent;
		color: #4f46e5;
		font: inherit;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.inline-back:hover {
		color: #4338ca;
	}

	.drop-zone {
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		padding: 2rem;
		text-align: center;
		transition:
			background-color 0.2s ease,
			border-color 0.2s ease;
	}

	.drop-zone--active {
		border-color: #6366f1;
		background: #eef2ff;
	}

	.selected-file {
		justify-content: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
		color: #4f46e5;
	}

	.selected-file-details {
		text-align: left;
	}

	.selected-file-name {
		color: #111827;
		font-weight: 600;
		line-height: 1.4;
	}

	.clear-file {
		display: grid;
		border: 0;
		background: transparent;
		color: #9ca3af;
		cursor: pointer;
		place-items: center;
	}

	.clear-file:hover {
		color: #4b5563;
	}

	.drop-icon {
		margin: 0 auto 1rem;
		color: #9ca3af;
	}

	.drop-copy {
		margin-bottom: 0.5rem;
	}

	.file-picker {
		display: inline-block;
	}

	.file-picker span {
		display: inline-flex;
	}

	.visually-hidden {
		position: absolute;
		width: 1px;
		height: 1px;
		overflow: hidden;
		clip: rect(0 0 0 0);
		clip-path: inset(50%);
		white-space: nowrap;
	}

	.error-callout {
		gap: 0.75rem;
		margin-top: 1rem;
		border-color: #fecaca;
		background: #fef2f2;
		color: #b91c1c;
	}

	.error-callout span {
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.upload-actions {
		display: flex;
		justify-content: flex-end;
		margin-top: 1.5rem;
	}

	.spin-icon {
		animation: spin 0.8s linear infinite;
	}

	.source-section {
		display: grid;
		gap: 1rem;
	}

	.source-section h2 {
		margin-bottom: 0.5rem;
	}

	.source-card {
		display: flex;
		width: 100%;
		align-items: center;
		gap: 1rem;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		background: #ffffff;
		padding: 1.5rem;
		text-align: left;
		cursor: pointer;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.source-card:hover {
		border-color: #a5b4fc;
		box-shadow: 0 10px 20px rgba(15, 23, 42, 0.08);
	}

	.source-icon {
		width: 3.5rem;
		height: 3.5rem;
		border-radius: 12px;
		color: #ffffff;
	}

	.source-icon--blue {
		background: #3b82f6;
	}

	.source-icon--orange {
		background: #f97316;
	}

	.source-icon--green {
		background: #22c55e;
	}

	.source-card h3 {
		margin-bottom: 0.125rem;
		color: #111827;
		font-size: 1.125rem;
		font-weight: 600;
		line-height: 1.4;
	}

	.source-card p {
		margin-bottom: 0;
	}

	.help-panel {
		margin-top: 2rem;
		border-color: #bfdbfe;
		background: #eff6ff;
	}

	.help-panel h3,
	.help-panel ul {
		color: #1e40af;
	}

	@media (min-width: 640px) {
		.page-container {
			padding-inline: 1.5rem;
		}
	}

	@media (min-width: 1024px) {
		.page-container {
			padding-inline: 2rem;
		}
	}

	@media (prefers-color-scheme: dark) {
		.import-page {
			background: #111827;
			color: #f9fafb;
		}

		.page-header,
		.panel,
		.source-card {
			border-color: #374151;
			background: #1f2937;
		}

		.back-link {
			color: #9ca3af;
		}

		.back-link:hover,
		.secondary-action:hover {
			background: #374151;
			color: #e5e7eb;
		}

		h1,
		.panel-header h2,
		.panel-title,
		.source-section h2,
		.result-state h3,
		.selected-file-name,
		.source-card h3 {
			color: #ffffff;
		}

		.title-row p,
		.panel-copy,
		.result-state p,
		.drop-copy,
		.selected-file-size,
		.source-card p,
		.progress-header {
			color: #9ca3af;
		}

		.progress-track {
			background: #374151;
		}

		.status-pill--success,
		.result-icon--success {
			background: rgba(22, 163, 74, 0.2);
		}

		.status-pill--danger,
		.result-icon--danger,
		.error-callout {
			background: rgba(220, 38, 38, 0.16);
		}

		.status-pill--info,
		.title-icon,
		.drop-zone--active {
			background: rgba(79, 70, 229, 0.24);
		}

		.status-pill--neutral {
			background: #374151;
			color: #d1d5db;
		}

		.secondary-action {
			color: #d1d5db;
		}

		.warning-callout {
			border-color: #854d0e;
			background: rgba(133, 77, 14, 0.18);
		}

		.warning-callout h4,
		.warning-callout ul {
			color: #fde68a;
		}

		.drop-zone {
			border-color: #4b5563;
		}

		.help-panel {
			border-color: #1e3a8a;
			background: rgba(30, 58, 138, 0.22);
		}

		.help-panel h3,
		.help-panel ul {
			color: #bfdbfe;
		}
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>

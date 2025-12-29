<script lang="ts">
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
			color: 'bg-blue-500',
			accept: '.json'
		},
		{
			id: 'asana' as ImportSource,
			name: 'Asana',
			description: 'Import projects and tasks from Asana CSV export',
			icon: IconBrandAsana,
			color: 'bg-orange-500',
			accept: '.csv'
		},
		{
			id: 'csv' as ImportSource,
			name: 'CSV File',
			description: 'Import tasks from a CSV file with custom mapping',
			icon: IconFileSpreadsheet,
			color: 'bg-green-500',
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
		} catch (err: any) {
			error = err.message || 'Failed to start import';
		} finally {
			loading = false;
		}
	}

	async function pollImportStatus() {
		if (!importJob) return;

		const poll = async () => {
			try {
				const job = await boardsAPI.getImportJob(importJob!.id);
				importJob = job;

				if (job.status === 'processing' || job.status === 'pending') {
					setTimeout(poll, 2000);
				}
			} catch (err) {
				console.error('Failed to poll import status:', err);
			}
		};

		poll();
	}

	function resetImport() {
		selectedSource = null;
		file = null;
		importJob = null;
		error = null;
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'completed': return 'text-green-600 bg-green-100 dark:bg-green-900/30';
			case 'failed': return 'text-red-600 bg-red-100 dark:bg-red-900/30';
			case 'processing': return 'text-blue-600 bg-blue-100 dark:bg-blue-900/30';
			default: return 'text-gray-600 bg-gray-100 dark:bg-gray-700';
		}
	}
</script>

<svelte:head>
	<title>Import Board | Project Boards</title>
</svelte:head>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<!-- Header -->
	<div class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
		<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="flex items-center gap-4">
				<a
					href="/admin/boards"
					class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
				>
					<IconArrowLeft class="w-5 h-5" />
				</a>
				<div class="flex items-center gap-3">
					<div class="p-2 bg-indigo-100 dark:bg-indigo-900/30 rounded-lg">
						<IconUpload class="w-6 h-6 text-indigo-600 dark:text-indigo-400" />
					</div>
					<div>
						<h1 class="text-2xl font-bold text-gray-900 dark:text-white">Import Board</h1>
						<p class="text-sm text-gray-500 dark:text-gray-400">Import from Trello, Asana, or CSV</p>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		{#if importJob}
			<!-- Import Progress -->
			<div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6">
				<div class="flex items-center justify-between mb-6">
					<h2 class="text-lg font-semibold text-gray-900 dark:text-white">Import Progress</h2>
					<span class="px-3 py-1 text-sm rounded-full {getStatusColor(importJob.status)} capitalize">
						{importJob.status}
					</span>
				</div>

				{#if importJob.status === 'processing'}
					<div class="mb-6">
						<div class="flex items-center justify-between mb-2">
							<span class="text-sm text-gray-600 dark:text-gray-400">Processing...</span>
							<span class="text-sm text-gray-600 dark:text-gray-400">
								{importJob.processed_items} / {importJob.total_items}
							</span>
						</div>
						<div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
							<div
								class="bg-indigo-600 h-2 rounded-full transition-all"
								style="width: {(importJob.processed_items / importJob.total_items) * 100}%"
							></div>
						</div>
					</div>
				{/if}

				{#if importJob.status === 'completed'}
					<div class="text-center py-8">
						<div class="w-16 h-16 bg-green-100 dark:bg-green-900/30 rounded-full flex items-center justify-center mx-auto mb-4">
							<IconCheck class="w-8 h-8 text-green-600" />
						</div>
						<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">Import Complete!</h3>
						<p class="text-gray-500 dark:text-gray-400 mb-6">
							Successfully imported {importJob.processed_items} items
							{#if importJob.failed_items > 0}
								({importJob.failed_items} failed)
							{/if}
						</p>
						<div class="flex items-center justify-center gap-4">
							<a
								href="/admin/boards"
								class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg"
							>
								View Boards
							</a>
							<button
								onclick={resetImport}
								class="px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
							>
								Import Another
							</button>
						</div>
					</div>
				{/if}

				{#if importJob.status === 'failed'}
					<div class="text-center py-8">
						<div class="w-16 h-16 bg-red-100 dark:bg-red-900/30 rounded-full flex items-center justify-center mx-auto mb-4">
							<IconX class="w-8 h-8 text-red-600" />
						</div>
						<h3 class="text-lg font-medium text-gray-900 dark:text-white mb-2">Import Failed</h3>
						<p class="text-gray-500 dark:text-gray-400 mb-6">
							{importJob.errors?.[0]?.message || 'An error occurred during import'}
						</p>
						<button
							onclick={resetImport}
							class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg"
						>
							Try Again
						</button>
					</div>
				{/if}

				{#if importJob.errors && importJob.errors.length > 0 && importJob.status === 'completed'}
					<div class="mt-6 p-4 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg">
						<h4 class="text-sm font-medium text-yellow-800 dark:text-yellow-200 mb-2">
							{importJob.errors.length} items could not be imported:
						</h4>
						<ul class="text-sm text-yellow-700 dark:text-yellow-300 space-y-1">
							{#each importJob.errors.slice(0, 5) as err}
								<li>• {err.message}</li>
							{/each}
							{#if importJob.errors.length > 5}
								<li class="text-yellow-600">...and {importJob.errors.length - 5} more</li>
							{/if}
						</ul>
					</div>
				{/if}
			</div>
		{:else if selectedSource}
			<!-- File Upload -->
			<div class="bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6">
				<button
					onclick={() => selectedSource = null}
					class="text-sm text-indigo-600 hover:text-indigo-700 mb-4 flex items-center gap-1"
				>
					<IconArrowLeft class="w-4 h-4" />
					Back to source selection
				</button>

				<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
					Upload {sources.find(s => s.id === selectedSource)?.name} File
				</h2>
				<p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
					{sources.find(s => s.id === selectedSource)?.description}
				</p>

				<!-- Drop Zone -->
				<div
					role="button"
					tabindex="0"
					class="border-2 border-dashed rounded-xl p-8 text-center transition-colors {dragOver ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-900/20' : 'border-gray-300 dark:border-gray-600'}"
					ondrop={handleDrop}
					ondragover={handleDragOver}
					ondragleave={handleDragLeave}
				>
					{#if file}
						<div class="flex items-center justify-center gap-3 mb-4">
							<IconFileSpreadsheet class="w-10 h-10 text-indigo-600" />
							<div class="text-left">
								<div class="font-medium text-gray-900 dark:text-white">{file.name}</div>
								<div class="text-sm text-gray-500">{(file.size / 1024).toFixed(1)} KB</div>
							</div>
							<button
								onclick={() => file = null}
								class="p-1 text-gray-400 hover:text-gray-600"
							>
								<IconX class="w-5 h-5" />
							</button>
						</div>
					{:else}
						<IconUpload class="w-10 h-10 text-gray-400 mx-auto mb-4" />
						<p class="text-gray-600 dark:text-gray-400 mb-2">
							Drag and drop your file here, or
						</p>
					{/if}

					<label class="inline-block">
						<span class="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg cursor-pointer">
							{file ? 'Choose Different File' : 'Browse Files'}
						</span>
						<input
							type="file"
							accept={sources.find(s => s.id === selectedSource)?.accept}
							onchange={handleFileSelect}
							class="hidden"
						/>
					</label>
				</div>

				{#if error}
					<div class="mt-4 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg flex items-center gap-3">
						<IconAlertCircle class="w-5 h-5 text-red-600" />
						<span class="text-sm text-red-700 dark:text-red-300">{error}</span>
					</div>
				{/if}

				<div class="mt-6 flex justify-end">
					<button
						onclick={startImport}
						disabled={!file || loading}
						class="px-6 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
					>
						{#if loading}
							<IconLoader class="w-4 h-4 animate-spin" />
							Importing...
						{:else}
							<IconUpload class="w-4 h-4" />
							Start Import
						{/if}
					</button>
				</div>
			</div>
		{:else}
			<!-- Source Selection -->
			<div class="space-y-4">
				<h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-6">Choose Import Source</h2>

				{#each sources as source}
					<button
						onclick={() => selectedSource = source.id}
						class="w-full bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 p-6 text-left hover:border-indigo-300 dark:hover:border-indigo-700 hover:shadow-md transition-all flex items-center gap-4"
					>
						<div class="p-3 {source.color} rounded-xl">
							<source.icon class="w-8 h-8 text-white" />
						</div>
						<div>
							<h3 class="text-lg font-medium text-gray-900 dark:text-white">{source.name}</h3>
							<p class="text-sm text-gray-500 dark:text-gray-400">{source.description}</p>
						</div>
					</button>
				{/each}
			</div>

			<!-- Help Section -->
			<div class="mt-8 p-6 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-xl">
				<h3 class="text-sm font-semibold text-blue-800 dark:text-blue-200 mb-2">How to export from other tools:</h3>
				<ul class="text-sm text-blue-700 dark:text-blue-300 space-y-2">
					<li><strong>Trello:</strong> Open board menu → More → Print and export → Export to JSON</li>
					<li><strong>Asana:</strong> Open project → ••• menu → Export/Print → CSV</li>
					<li><strong>CSV:</strong> Create a CSV with columns: title, description, status, due_date, priority</li>
				</ul>
			</div>
		{/if}
	</div>
</div>

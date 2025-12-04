<script lang="ts">
	/**
	 * Duplicate Image Detector UI
	 *
	 * Finds and manages duplicate images in the media library.
	 * Allows merging duplicates to save storage space.
	 */

	import { onMount } from 'svelte';

	interface DuplicateFile {
		id: number;
		filename: string;
		path: string;
		size: number;
		created_at: string;
		usage_count: number;
	}

	interface DuplicateGroup {
		hash: string;
		count: number;
		total_size: number;
		potential_savings: number;
		files: DuplicateFile[];
	}

	interface Props {
		apiEndpoint?: string;
	}

	let { apiEndpoint = '/api/admin/media/duplicates' }: Props = $props();

	let duplicates: DuplicateGroup[] = $state([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedGroup = $state<DuplicateGroup | null>(null);
	let selectedKeepId = $state<number | null>(null);
	let isMerging = $state(false);
	let scanProgress = $state(0);
	let isScanning = $state(false);

	// Summary stats
	let totalDuplicates = $derived(duplicates.reduce((sum, g) => sum + g.count - 1, 0));
	let totalSavings = $derived(duplicates.reduce((sum, g) => sum + g.potential_savings, 0));

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	async function loadDuplicates() {
		isLoading = true;
		error = null;

		try {
			const response = await fetch(apiEndpoint, {
				headers: {
					Accept: 'application/json',
					Authorization: `Bearer ${localStorage.getItem('auth_token')}`
				}
			});

			if (!response.ok) {
				throw new Error('Failed to load duplicates');
			}

			const data = await response.json();
			duplicates = data.data || data;
		} catch (e) {
			error = e instanceof Error ? e.message : 'An error occurred';
		} finally {
			isLoading = false;
		}
	}

	async function scanForDuplicates() {
		isScanning = true;
		scanProgress = 0;

		try {
			// Simulate progress during scan
			const progressInterval = setInterval(() => {
				scanProgress = Math.min(scanProgress + Math.random() * 15, 90);
			}, 500);

			const response = await fetch(`${apiEndpoint}/scan`, {
				method: 'POST',
				headers: {
					Accept: 'application/json',
					Authorization: `Bearer ${localStorage.getItem('auth_token')}`
				}
			});

			clearInterval(progressInterval);
			scanProgress = 100;

			if (!response.ok) {
				throw new Error('Scan failed');
			}

			// Reload duplicates
			await loadDuplicates();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Scan failed';
		} finally {
			isScanning = false;
			scanProgress = 0;
		}
	}

	async function mergeDuplicates() {
		if (!selectedGroup || !selectedKeepId) return;

		isMerging = true;
		const idsToMerge = selectedGroup.files.map((f) => f.id);

		try {
			const response = await fetch(`${apiEndpoint}/merge`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
					Accept: 'application/json',
					Authorization: `Bearer ${localStorage.getItem('auth_token')}`
				},
				body: JSON.stringify({
					keep_id: selectedKeepId,
					merge_ids: idsToMerge
				})
			});

			if (!response.ok) {
				throw new Error('Merge failed');
			}

			// Remove merged group from list
			duplicates = duplicates.filter((g) => g.hash !== selectedGroup!.hash);
			closeModal();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Merge failed';
		} finally {
			isMerging = false;
		}
	}

	function openMergeModal(group: DuplicateGroup) {
		selectedGroup = group;
		// Default to keeping the one with highest usage
		const maxUsage = Math.max(...group.files.map((f) => f.usage_count));
		const fileWithMaxUsage = group.files.find((f) => f.usage_count === maxUsage);
		selectedKeepId = fileWithMaxUsage?.id || group.files[0].id;
	}

	function closeModal() {
		selectedGroup = null;
		selectedKeepId = null;
	}

	onMount(() => {
		loadDuplicates();
	});
</script>

<div class="duplicate-detector">
	<!-- Header -->
	<div class="header">
		<div class="header-content">
			<h2>Duplicate Image Detector</h2>
			<p>Find and remove duplicate images to save storage space</p>
		</div>
		<button class="scan-button" onclick={scanForDuplicates} disabled={isScanning}>
			{#if isScanning}
				<span class="spinner"></span>
				Scanning... {Math.round(scanProgress)}%
			{:else}
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
					<path
						d="M15.312 11.424a5.5 5.5 0 01-9.201 2.466l-.312-.311h2.433a.75.75 0 000-1.5H3.989a.75.75 0 00-.75.75v4.242a.75.75 0 001.5 0v-2.43l.31.31a7 7 0 0011.712-3.138.75.75 0 00-1.449-.39zm1.23-3.723a.75.75 0 00.219-.53V2.929a.75.75 0 00-1.5 0v2.43l-.31-.31A7 7 0 003.239 8.188a.75.75 0 101.448.389A5.5 5.5 0 0113.89 6.11l.311.31h-2.432a.75.75 0 000 1.5h4.243a.75.75 0 00.53-.219z"
					/>
				</svg>
				Scan for Duplicates
			{/if}
		</button>
	</div>

	<!-- Scan progress bar -->
	{#if isScanning}
		<div class="progress-bar-container">
			<div class="progress-bar" style="width: {scanProgress}%"></div>
		</div>
	{/if}

	<!-- Summary cards -->
	{#if !isLoading && duplicates.length > 0}
		<div class="summary-cards">
			<div class="summary-card">
				<div class="card-icon groups">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
						<path
							d="M3.5 2A1.5 1.5 0 002 3.5V5c0 1.149.15 2.263.43 3.326a13.022 13.022 0 009.244 9.244c1.063.28 2.177.43 3.326.43h1.5a1.5 1.5 0 001.5-1.5v-1.148a1.5 1.5 0 00-1.175-1.465l-3.223-.716a1.5 1.5 0 00-1.767 1.052l-.267.933c-.117.41-.555.643-.95.48a11.542 11.542 0 01-6.254-6.254c-.163-.395.07-.833.48-.95l.933-.267a1.5 1.5 0 001.052-1.767l-.716-3.223A1.5 1.5 0 004.648 2H3.5z"
						/>
					</svg>
				</div>
				<div class="card-content">
					<span class="card-value">{duplicates.length}</span>
					<span class="card-label">Duplicate Groups</span>
				</div>
			</div>

			<div class="summary-card">
				<div class="card-icon files">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
						<path
							fill-rule="evenodd"
							d="M15.988 3.012A2.25 2.25 0 0118 5.25v6.5A2.25 2.25 0 0115.75 14H13.5v-3.379a3 3 0 00-.879-2.121l-3.12-3.121a3 3 0 00-1.402-.791 2.252 2.252 0 011.913-1.576A2.25 2.25 0 0112.25 1h1.5a2.25 2.25 0 012.238 2.012zM11.5 3.25a.75.75 0 01.75-.75h1.5a.75.75 0 01.75.75v.25h-3v-.25z"
							clip-rule="evenodd"
						/>
						<path
							d="M3.5 6A1.5 1.5 0 002 7.5v9A1.5 1.5 0 003.5 18h7a1.5 1.5 0 001.5-1.5v-5.879a1.5 1.5 0 00-.44-1.06L8.44 6.439A1.5 1.5 0 007.378 6H3.5z"
						/>
					</svg>
				</div>
				<div class="card-content">
					<span class="card-value">{totalDuplicates}</span>
					<span class="card-label">Duplicate Files</span>
				</div>
			</div>

			<div class="summary-card highlight">
				<div class="card-icon savings">
					<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
						<path
							fill-rule="evenodd"
							d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
							clip-rule="evenodd"
						/>
					</svg>
				</div>
				<div class="card-content">
					<span class="card-value">{formatBytes(totalSavings)}</span>
					<span class="card-label">Potential Savings</span>
				</div>
			</div>
		</div>
	{/if}

	<!-- Error message -->
	{#if error}
		<div class="error-message">
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
				<path
					fill-rule="evenodd"
					d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zm0 10a1 1 0 100-2 1 1 0 000 2z"
					clip-rule="evenodd"
				/>
			</svg>
			<span>{error}</span>
			<button onclick={() => (error = null)}>Dismiss</button>
		</div>
	{/if}

	<!-- Loading state -->
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner large"></div>
			<p>Searching for duplicates...</p>
		</div>
	{:else if duplicates.length === 0}
		<!-- Empty state -->
		<div class="empty-state">
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="1.5"
					d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
				/>
			</svg>
			<h3>No Duplicates Found</h3>
			<p>Your media library is clean! No duplicate images were detected.</p>
		</div>
	{:else}
		<!-- Duplicate groups list -->
		<div class="duplicates-list">
			{#each duplicates as group (group.hash)}
				<div class="duplicate-group">
					<div class="group-header">
						<div class="group-info">
							<span class="group-count">{group.count} copies</span>
							<span class="group-savings">Save {formatBytes(group.potential_savings)}</span>
						</div>
						<button class="merge-button" onclick={() => openMergeModal(group)}>
							Merge Duplicates
						</button>
					</div>

					<div class="files-preview">
						{#each group.files.slice(0, 4) as file (file.id)}
							<div class="file-card">
								<div class="file-thumbnail">
									<img src="/storage/{file.path}" alt={file.filename} loading="lazy" />
								</div>
								<div class="file-info">
									<span class="file-name" title={file.filename}>
										{file.filename.length > 20
											? file.filename.slice(0, 20) + '...'
											: file.filename}
									</span>
									<span class="file-meta">
										{formatBytes(file.size)} · {formatDate(file.created_at)}
									</span>
									{#if file.usage_count > 0}
										<span class="usage-badge">Used {file.usage_count}x</span>
									{/if}
								</div>
							</div>
						{/each}
						{#if group.files.length > 4}
							<div class="more-files">+{group.files.length - 4} more</div>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Merge Modal -->
	{#if selectedGroup}
		<div class="modal-overlay" onclick={closeModal} role="presentation">
			<div
				class="modal"
				onclick={(e) => e.stopPropagation()}
				role="dialog"
				aria-modal="true"
				aria-labelledby="modal-title"
			>
				<div class="modal-header">
					<h3 id="modal-title">Merge Duplicates</h3>
					<button class="close-button" onclick={closeModal} aria-label="Close">
						<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
							<path
								d="M6.28 5.22a.75.75 0 00-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 101.06 1.06L10 11.06l3.72 3.72a.75.75 0 101.06-1.06L11.06 10l3.72-3.72a.75.75 0 00-1.06-1.06L10 8.94 6.28 5.22z"
							/>
						</svg>
					</button>
				</div>

				<div class="modal-body">
					<p class="modal-description">
						Select which file to keep. All other files will be deleted and their references will be
						updated to point to the kept file.
					</p>

					<div class="files-selection">
						{#each selectedGroup.files as file (file.id)}
							<label class="file-option" class:selected={selectedKeepId === file.id}>
								<input
									type="radio"
									name="keep-file"
									value={file.id}
									checked={selectedKeepId === file.id}
									onchange={() => (selectedKeepId = file.id)}
								/>
								<div class="file-option-content">
									<div class="file-option-thumbnail">
										<img src="/storage/{file.path}" alt={file.filename} />
									</div>
									<div class="file-option-info">
										<span class="file-option-name">{file.filename}</span>
										<span class="file-option-meta">
											{formatBytes(file.size)} · Created {formatDate(file.created_at)}
										</span>
										<span class="file-option-usage">
											{file.usage_count > 0
												? `Used in ${file.usage_count} places`
												: 'Not used anywhere'}
										</span>
									</div>
									{#if selectedKeepId === file.id}
										<span class="keep-badge">Keep</span>
									{/if}
								</div>
							</label>
						{/each}
					</div>

					<div class="merge-summary">
						<p>
							<strong>{selectedGroup.files.length - 1}</strong> files will be deleted, saving
							<strong>{formatBytes(selectedGroup.potential_savings)}</strong>
						</p>
					</div>
				</div>

				<div class="modal-footer">
					<button class="cancel-button" onclick={closeModal} disabled={isMerging}>Cancel</button>
					<button class="confirm-button" onclick={mergeDuplicates} disabled={isMerging}>
						{#if isMerging}
							<span class="spinner small"></span>
							Merging...
						{:else}
							Merge & Delete Duplicates
						{/if}
					</button>
				</div>
			</div>
		</div>
	{/if}
</div>

<style>
	.duplicate-detector {
		padding: 1.5rem;
		background: white;
		border-radius: 12px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
	}

	.header-content h2 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 0.25rem 0;
	}

	.header-content p {
		color: #64748b;
		margin: 0;
	}

	.scan-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.scan-button:hover:not(:disabled) {
		background: #2563eb;
	}

	.scan-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.scan-button svg {
		width: 1.25rem;
		height: 1.25rem;
	}

	.progress-bar-container {
		height: 4px;
		background: #e2e8f0;
		border-radius: 2px;
		margin-bottom: 1.5rem;
		overflow: hidden;
	}

	.progress-bar {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #8b5cf6);
		transition: width 0.3s ease;
	}

	.summary-cards {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.summary-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: #f8fafc;
		border-radius: 8px;
	}

	.summary-card.highlight {
		background: linear-gradient(135deg, #dcfce7, #d1fae5);
	}

	.card-icon {
		width: 3rem;
		height: 3rem;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
	}

	.card-icon.groups {
		background: #dbeafe;
		color: #3b82f6;
	}

	.card-icon.files {
		background: #fef3c7;
		color: #f59e0b;
	}

	.card-icon.savings {
		background: #dcfce7;
		color: #22c55e;
	}

	.card-icon svg {
		width: 1.5rem;
		height: 1.5rem;
	}

	.card-content {
		display: flex;
		flex-direction: column;
	}

	.card-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: #1e293b;
	}

	.card-label {
		font-size: 0.75rem;
		color: #64748b;
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 8px;
		color: #dc2626;
		margin-bottom: 1.5rem;
	}

	.error-message svg {
		width: 1.25rem;
		height: 1.25rem;
		flex-shrink: 0;
	}

	.error-message span {
		flex: 1;
	}

	.error-message button {
		padding: 0.25rem 0.75rem;
		background: white;
		border: 1px solid #fecaca;
		border-radius: 4px;
		color: #dc2626;
		cursor: pointer;
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		text-align: center;
	}

	.empty-state svg {
		width: 4rem;
		height: 4rem;
		color: #22c55e;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1e293b;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		color: #64748b;
		margin: 0;
	}

	.spinner {
		width: 1.25rem;
		height: 1.25rem;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.spinner.large {
		width: 3rem;
		height: 3rem;
		border-width: 3px;
		border-color: rgba(59, 130, 246, 0.2);
		border-top-color: #3b82f6;
		margin-bottom: 1rem;
	}

	.spinner.small {
		width: 1rem;
		height: 1rem;
		border-width: 2px;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.duplicates-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.duplicate-group {
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		overflow: hidden;
	}

	.group-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: #f8fafc;
		border-bottom: 1px solid #e2e8f0;
	}

	.group-info {
		display: flex;
		gap: 1rem;
	}

	.group-count {
		font-weight: 600;
		color: #1e293b;
	}

	.group-savings {
		color: #22c55e;
		font-weight: 500;
	}

	.merge-button {
		padding: 0.5rem 1rem;
		background: #f59e0b;
		color: white;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.merge-button:hover {
		background: #d97706;
	}

	.files-preview {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		overflow-x: auto;
	}

	.file-card {
		flex-shrink: 0;
		width: 160px;
	}

	.file-thumbnail {
		width: 100%;
		aspect-ratio: 1;
		background: #f1f5f9;
		border-radius: 6px;
		overflow: hidden;
		margin-bottom: 0.5rem;
	}

	.file-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file-info {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.file-name {
		font-size: 0.875rem;
		font-weight: 500;
		color: #1e293b;
	}

	.file-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	.usage-badge {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		background: #dbeafe;
		color: #3b82f6;
		font-size: 0.625rem;
		font-weight: 500;
		border-radius: 9999px;
		margin-top: 0.25rem;
	}

	.more-files {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		background: #f1f5f9;
		border-radius: 6px;
		color: #64748b;
		font-size: 0.875rem;
	}

	/* Modal styles */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		padding: 1rem;
	}

	.modal {
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		background: white;
		border-radius: 12px;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		border-bottom: 1px solid #e2e8f0;
	}

	.modal-header h3 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0;
	}

	.close-button {
		padding: 0.5rem;
		background: none;
		border: none;
		color: #64748b;
		cursor: pointer;
		border-radius: 6px;
	}

	.close-button:hover {
		background: #f1f5f9;
	}

	.close-button svg {
		width: 1.25rem;
		height: 1.25rem;
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-description {
		color: #64748b;
		margin: 0 0 1.5rem 0;
	}

	.files-selection {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.file-option {
		display: block;
		cursor: pointer;
	}

	.file-option input {
		position: absolute;
		opacity: 0;
	}

	.file-option-content {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem;
		border: 2px solid #e2e8f0;
		border-radius: 8px;
		transition:
			border-color 0.2s,
			background 0.2s;
	}

	.file-option.selected .file-option-content {
		border-color: #3b82f6;
		background: #eff6ff;
	}

	.file-option-thumbnail {
		width: 4rem;
		height: 4rem;
		flex-shrink: 0;
		background: #f1f5f9;
		border-radius: 6px;
		overflow: hidden;
	}

	.file-option-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.file-option-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.file-option-name {
		font-weight: 500;
		color: #1e293b;
	}

	.file-option-meta {
		font-size: 0.75rem;
		color: #64748b;
	}

	.file-option-usage {
		font-size: 0.75rem;
		color: #3b82f6;
	}

	.keep-badge {
		padding: 0.25rem 0.75rem;
		background: #22c55e;
		color: white;
		font-size: 0.75rem;
		font-weight: 500;
		border-radius: 9999px;
	}

	.merge-summary {
		margin-top: 1.5rem;
		padding: 1rem;
		background: #fef3c7;
		border-radius: 8px;
		color: #92400e;
	}

	.merge-summary p {
		margin: 0;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid #e2e8f0;
	}

	.cancel-button {
		padding: 0.75rem 1.5rem;
		background: white;
		border: 1px solid #e2e8f0;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
	}

	.cancel-button:hover:not(:disabled) {
		background: #f8fafc;
	}

	.confirm-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: #ef4444;
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
	}

	.confirm-button:hover:not(:disabled) {
		background: #dc2626;
	}

	.confirm-button:disabled,
	.cancel-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}
</style>

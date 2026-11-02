<script lang="ts">
	/**
	 * Duplicate Image Detector UI
	 *
	 * Finds and manages duplicate images in the media library.
	 * Allows merging duplicates to save storage space.
	 */

	import { onMount } from 'svelte';
	import Icon from '#lib/components/Icon.svelte';

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
	const scanProgressWidth = $derived(`${scanProgress}%`);

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
		let progressInterval: ReturnType<typeof setInterval> | undefined;

		try {
			// Simulate progress during scan
			progressInterval = setInterval(() => {
				scanProgress = Math.min(scanProgress + Math.random() * 15, 90);
			}, 500);

			const response = await fetch(`${apiEndpoint}/scan`, {
				method: 'POST',
				headers: {
					Accept: 'application/json',
					Authorization: `Bearer ${localStorage.getItem('auth_token')}`
				}
			});

			scanProgress = 100;

			if (!response.ok) {
				throw new Error('Scan failed');
			}

			// Reload duplicates
			await loadDuplicates();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Scan failed';
		} finally {
			if (progressInterval) {
				clearInterval(progressInterval);
			}
			isScanning = false;
			scanProgress = 0;
		}
	}

	async function mergeDuplicates() {
		if (!selectedGroup || !selectedKeepId) return;
		const group = selectedGroup;

		isMerging = true;
		const idsToMerge = group.files.map((f) => f.id);

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
			duplicates = duplicates.filter((g) => g.hash !== group.hash);
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
		selectedKeepId = fileWithMaxUsage?.id || group.files[0]?.id || null;
	}

	function closeModal() {
		selectedGroup = null;
		selectedKeepId = null;
	}

	onMount(() => {
		void loadDuplicates();
	});

	function getFileOptionClass(fileId: number) {
		return ['file-option', selectedKeepId === fileId && 'file-option--selected'];
	}
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
				<Icon name="IconRefresh" size={20} />
				Scan for Duplicates
			{/if}
		</button>
	</div>

	<!-- Scan progress bar -->
	{#if isScanning}
		<div class="progress-bar-container">
			<div class="progress-bar" style:width={scanProgressWidth}></div>
		</div>
	{/if}

	<!-- Summary cards -->
	{#if !isLoading && duplicates.length > 0}
		<div class="summary-cards">
			<div class="summary-card">
				<div class="card-icon groups">
					<Icon name="IconPhone" size={24} />
				</div>
				<div class="card-content">
					<span class="card-value">{duplicates.length}</span>
					<span class="card-label">Duplicate Groups</span>
				</div>
			</div>

			<div class="summary-card">
				<div class="card-icon files">
					<Icon name="IconFile" size={24} />
				</div>
				<div class="card-content">
					<span class="card-value">{totalDuplicates}</span>
					<span class="card-label">Duplicate Files</span>
				</div>
			</div>

			<div class="summary-card highlight">
				<div class="card-icon savings">
					<Icon name="IconCircleCheckFilled" size={24} />
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
			<Icon name="IconAlertCircle" size={20} />
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
			<Icon name="IconCircleCheck" size={64} class="empty-state__icon" />
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
									<img
										src="/storage/{file.path}"
										alt={file.filename}
										loading="lazy"
										width="160"
										height="160"
									/>
								</div>
								<div class="file-info">
									<span class="file-name" title={file.filename}>
										{file.filename.length > 20 ? file.filename.slice(0, 20) + '...' : file.filename}
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
		<div
			class="modal-overlay"
			onclick={closeModal}
			onkeydown={(e: KeyboardEvent) => {
				if (e.key === 'Escape') closeModal();
				if (e.key === 'Enter' || e.key === ' ') closeModal();
			}}
			role="button"
			tabindex="0"
			aria-label="Close modal"
		>
			<div
				class="modal"
				onclick={(e: MouseEvent) => e.stopPropagation()}
				onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
				role="dialog"
				aria-modal="true"
				aria-labelledby="modal-title"
				tabindex="-1"
			>
				<div class="modal-header">
					<h3 id="modal-title">Merge Duplicates</h3>
					<button class="close-button" onclick={closeModal} aria-label="Close">
						<Icon name="IconX" size={20} />
					</button>
				</div>

				<div class="modal-body">
					<p class="modal-description">
						Select which file to keep. All other files will be deleted and their references will be
						updated to point to the kept file.
					</p>

					<div class="files-selection">
						{#each selectedGroup.files as file (file.id)}
							<label class={getFileOptionClass(file.id)}>
								<input
									type="radio"
									name="keep-file"
									value={file.id}
									checked={selectedKeepId === file.id}
									onchange={() => (selectedKeepId = file.id)}
								/>
								<div class="file-option-content">
									<div class="file-option-thumbnail">
										<img
											src="/storage/{file.path}"
											alt={file.filename}
											width="64"
											height="64"
											loading="lazy"
										/>
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
		background: var(--bg-elevated, #161b22);
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
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.25rem 0;
	}

	.header-content p {
		color: var(--text-tertiary, #6e7681);
		margin: 0;
	}

	.scan-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
		border: none;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.scan-button:hover:not(:disabled) {
		background: var(--primary-500, #e6b800);
	}

	.scan-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.progress-bar-container {
		height: 4px;
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		border-radius: 2px;
		margin-bottom: 1.5rem;
		overflow: hidden;
	}

	.progress-bar {
		height: 100%;
		background: var(--primary-500, #e6b800);
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
		background: var(--bg-elevated, #161b22);
		border-radius: 8px;
	}

	.summary-card.highlight {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
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
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--info-emphasis, #58a6ff);
	}

	.card-icon.files {
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		color: var(--warning-emphasis, #d29922);
	}

	.card-icon.savings {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}

	.card-content {
		display: flex;
		flex-direction: column;
	}

	.card-value {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary, #f0f6fc);
	}

	.card-label {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.error-message {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		color: var(--error-emphasis, #f85149);
		margin-bottom: 1.5rem;
	}

	.error-message span {
		flex: 1;
	}

	.error-message button {
		padding: 0.25rem 0.75rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 4px;
		color: var(--error-emphasis, #f85149);
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

	.empty-state h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0 0 0.5rem 0;
	}

	.empty-state p {
		color: var(--text-tertiary, #6e7681);
		margin: 0;
	}

	.empty-state :global(.empty-state__icon) {
		color: var(--success-emphasis, #3fb950);
		margin-bottom: 1rem;
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
		border-color: rgba(88, 166, 255, 0.2);
		border-top-color: var(--primary-500, #e6b800);
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
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		overflow: hidden;
	}

	.group-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
	}

	.group-info {
		display: flex;
		gap: 1rem;
	}

	.group-count {
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
	}

	.group-savings {
		color: var(--success-emphasis, #3fb950);
		font-weight: 500;
	}

	.merge-button {
		padding: 0.5rem 1rem;
		background: var(--warning-emphasis, #d29922);
		color: #ffffff;
		border: none;
		border-radius: 6px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.2s;
	}

	.merge-button:hover {
		background: var(--warning-emphasis, #d29922);
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
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
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
		color: var(--text-primary, #f0f6fc);
	}

	.file-meta {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.usage-badge {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		color: var(--info-emphasis, #58a6ff);
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
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
		border-radius: 6px;
		color: var(--text-tertiary, #6e7681);
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
		background: var(--bg-elevated, #161b22);
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
		border-bottom: 1px solid var(--border-default, #30363d);
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
		color: var(--text-tertiary, #6e7681);
		cursor: pointer;
		border-radius: 6px;
	}

	.close-button:hover {
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
	}

	.modal-description {
		color: var(--text-tertiary, #6e7681);
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
		border: 2px solid var(--border-default, #30363d);
		border-radius: 8px;
		transition:
			border-color 0.2s,
			background 0.2s;
	}

	.file-option--selected .file-option-content {
		border-color: var(--primary-500, #e6b800);
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
	}

	.file-option-thumbnail {
		width: 4rem;
		height: 4rem;
		flex-shrink: 0;
		background: var(--info-soft, rgba(56, 139, 253, 0.15));
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
		color: var(--text-primary, #f0f6fc);
	}

	.file-option-meta {
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.file-option-usage {
		font-size: 0.75rem;
		color: var(--info-emphasis, #58a6ff);
	}

	.keep-badge {
		padding: 0.25rem 0.75rem;
		background: var(--success-base, #2ea043);
		color: #ffffff;
		font-size: 0.75rem;
		font-weight: 500;
		border-radius: 9999px;
	}

	.merge-summary {
		margin-top: 1.5rem;
		padding: 1rem;
		background: var(--warning-soft, rgba(187, 128, 9, 0.15));
		border-radius: 8px;
		color: var(--warning-emphasis, #d29922);
	}

	.merge-summary p {
		margin: 0;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		border-top: 1px solid var(--border-default, #30363d);
	}

	.cancel-button {
		padding: 0.75rem 1.5rem;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
	}

	.cancel-button:hover:not(:disabled) {
		background: var(--bg-elevated, #161b22);
	}

	.confirm-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: var(--error-base, #da3633);
		color: #ffffff;
		border: none;
		border-radius: 8px;
		font-weight: 500;
		cursor: pointer;
	}

	.confirm-button:hover:not(:disabled) {
		background: var(--error-base, #da3633);
	}

	.confirm-button:disabled,
	.cancel-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}
</style>

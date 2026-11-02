<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';

	interface IndicatorFile {
		id: number;
		indicator_id: number;
		platform: string;
		file_name: string;
		display_name?: string;
		file_path?: string;
		file_size_bytes?: number;
		version?: string;
		is_current_version?: boolean;
		is_active?: boolean;
		display_order?: number;
		download_count?: number;
		created_at?: string;
	}

	interface Props {
		files: IndicatorFile[];
		loading: boolean;
		onUploadClick: () => void;
		onToggleStatus: (fileId: number) => void;
		onDelete: (fileId: number) => void;
	}

	let { files, loading, onUploadClick, onToggleStatus, onDelete }: Props = $props();

	const formatFileSize = (bytes?: number): string => {
		if (!bytes) return 'Unknown';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
	};
</script>

<div class="form-section">
	<div class="section-header">
		<h2>Indicator Files</h2>
		<button class="btn-primary" onclick={onUploadClick}>Upload File</button>
	</div>

	{#if loading}
		<div class="loading-inline">
			<div class="spinner-small"></div>
			<span>Loading files...</span>
		</div>
	{:else if files.length === 0}
		<div class="empty-state">
			<p>No files uploaded yet</p>
			<p class="hint">Upload indicator files for different trading platforms</p>
		</div>
	{:else}
		<div class="files-table">
			<table>
				<thead>
					<tr>
						<th>Platform</th>
						<th>File</th>
						<th>Version</th>
						<th>Size</th>
						<th>Downloads</th>
						<th>Status</th>
						<th>Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each files as file (file.id)}
						<tr>
							<td class="platform">{file.platform}</td>
							<td class="file-name">{file.display_name || file.file_name}</td>
							<td>{file.version || '1.0'}</td>
							<td>{formatFileSize(file.file_size_bytes)}</td>
							<td>{file.download_count || 0}</td>
							<td>
								<button
									class={['file-status', { active: file.is_active }]}
									onclick={() => onToggleStatus(file.id)}
								>
									{file.is_active ? 'Active' : 'Inactive'}
								</button>
							</td>
							<td>
								<button
									class="btn-icon btn-danger"
									onclick={() => onDelete(file.id)}
									title="Delete file"
									aria-label="Delete file"
								>
									<IconX size={16} aria-hidden="true" />
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<style>
	.form-section {
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: 12px;
		padding: 24px;
		margin-bottom: 24px;
	}
	.form-section h2 {
		font-size: 16px;
		font-weight: 600;
		color: var(--text-primary, #f0f6fc);
		margin: 0;
	}
	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		border: none;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		background: var(--primary-500, #e6b800);
		color: var(--bg-base, #0d1117);
	}
	.btn-primary:hover {
		background: var(--primary-500, #e6b800);
	}

	.spinner-small {
		width: 20px;
		height: 20px;
		border: 2px solid #e5e7eb;
		border-top-color: var(--primary-500, #e6b800);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
	.loading-inline {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 24px;
		color: var(--text-tertiary, #6e7681);
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state {
		text-align: center;
		padding: 40px;
		color: var(--text-tertiary, #6e7681);
	}
	.hint {
		font-size: 13px;
		color: var(--text-secondary, #8b949e);
	}

	.files-table {
		overflow-x: auto;
	}
	.files-table table {
		width: 100%;
		border-collapse: collapse;
		min-width: 600px;
	}
	.files-table th {
		text-align: left;
		padding: 12px;
		font-size: 12px;
		font-weight: 600;
		color: var(--text-tertiary, #6e7681);
		text-transform: uppercase;
		border-bottom: 1px solid var(--border-default, #30363d);
	}
	.files-table td {
		padding: 12px;
		border-bottom: 1px solid var(--border-default, #30363d);
	}
	.platform {
		font-weight: 500;
		text-transform: capitalize;
	}
	.file-name {
		color: var(--text-primary, #f0f6fc);
		font-size: 14px;
	}
	.file-status {
		font-size: 12px;
		padding: 4px 12px;
		border-radius: 4px;
		background: var(--bg-surface, #1c2128);
		border: none;
		cursor: pointer;
	}
	.file-status.active {
		background: var(--success-soft, rgba(46, 160, 67, 0.15));
		color: var(--success-emphasis, #3fb950);
	}
	.file-status:hover {
		opacity: 0.8;
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: var(--bg-surface, #1c2128);
		border-radius: 6px;
		color: var(--text-tertiary, #6e7681);
		cursor: pointer;
		min-height: 44px;
		min-width: 44px;
		font-weight: bold;
	}
	.btn-danger:hover {
		background: var(--error-soft, rgba(218, 54, 51, 0.15));
		color: var(--error-emphasis, #f85149);
	}

	@media (max-width: 639px) {
		.form-section {
			padding: 16px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.spinner-small {
			animation: none;
		}
	}
</style>

<script lang="ts">
	/**
	 * R19-C extraction (2026-05-20): Downloads tab — file upload + list +
	 * delete. Parent owns the upload pipeline (Bunny storage), child surfaces
	 * the file picker via `onFileSelect`.
	 */
	import IconTrash from '@tabler/icons-svelte-runes/icons/trash';
	import IconUpload from '@tabler/icons-svelte-runes/icons/upload';
	import IconDownload from '@tabler/icons-svelte-runes/icons/download';

	interface Download {
		id: number;
		title: string;
		file_name: string;
		file_size_bytes?: number;
		download_url?: string;
		category?: string;
	}

	interface Props {
		downloads: Download[];
		uploading: boolean;
		onFileSelect: (event: Event) => void;
		onDeleteDownload: (id: number) => void;
	}

	let { downloads, uploading, onFileSelect, onDeleteDownload }: Props = $props();

	let fileInput: HTMLInputElement | null = $state(null);

	const formatFileSize = (bytes?: number) => {
		if (!bytes) return '-';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / 1048576).toFixed(1)} MB`;
	};
</script>

<div class="downloads-section">
	<div class="content-header">
		<h2>Course Downloads</h2>
		<input
			id="page-file"
			name="page-file"
			type="file"
			bind:this={fileInput}
			onchange={onFileSelect}
			style="display: none;"
		/>
		<button class="btn-secondary" onclick={() => fileInput?.click()} disabled={uploading}>
			{#if uploading}
				<span class="spinner-small"></span>
				Uploading...
			{:else}
				<IconUpload size={16} aria-hidden="true" />
				Upload File
			{/if}
		</button>
	</div>

	{#if downloads.length === 0}
		<div class="empty-downloads">
			<IconDownload size={48} aria-hidden="true" />
			<p>No downloads yet. Upload files to make them available to enrolled students.</p>
		</div>
	{:else}
		<ul class="downloads-list">
			{#each downloads as dl (dl.id)}
				<li>
					<span class="dl-title">{dl.title}</span>
					<span class="dl-meta">
						<span class="dl-file">{dl.file_name}</span>
						<span class="dl-size">{formatFileSize(dl.file_size_bytes)}</span>
					</span>
					<button onclick={() => onDeleteDownload(dl.id)} aria-label="Delete download">
						<IconTrash size={16} aria-hidden="true" />
					</button>
				</li>
			{/each}
		</ul>
	{/if}
</div>

<style>
	.downloads-section {
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		padding: 24px;
	}
	.content-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
	}
	.content-header h2 {
		font-size: 16px;
		font-weight: 600;
		color: #1f2937;
		margin: 0;
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 16px;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
		text-decoration: none;
		border: none;
		background: #f3f4f6;
		color: #1f2937;
	}
	.btn-secondary:hover {
		background: #e5e7eb;
	}
	.btn-secondary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.empty-downloads {
		text-align: center;
		padding: 48px;
		color: #6b7280;
	}
	.empty-downloads :global(svg) {
		margin-bottom: 16px;
		opacity: 0.5;
	}
	.downloads-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}
	.downloads-list li {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px;
		background: #f9fafb;
		border-radius: 6px;
		margin-bottom: 8px;
	}
	.dl-title {
		font-weight: 500;
		flex: 1;
	}
	.dl-meta {
		display: flex;
		gap: 16px;
		align-items: center;
	}
	.dl-file {
		color: #6b7280;
		font-size: 13px;
	}
	.dl-size {
		color: #9ca3af;
		font-size: 12px;
	}
	.downloads-list button {
		background: none;
		border: none;
		color: #9ca3af;
		cursor: pointer;
	}
	.downloads-list button:hover {
		color: #dc2626;
	}
	.spinner-small {
		width: 16px;
		height: 16px;
		border: 2px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
		display: inline-block;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.btn-secondary {
			transition: none;
		}
		.spinner-small {
			animation: none;
		}
	}
</style>

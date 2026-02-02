<!--
/**
 * File Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Downloadable file attachment with icon and metadata
 */
-->

<script lang="ts">
	import { IconFile, IconDownload, IconFileTypePdf, IconFileTypeDoc, IconFileSpreadsheet, IconFileZip } from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();

	let fileUrl = $derived(props.block.content.fileUrl || '');
	let fileName = $derived(props.block.content.fileName || 'Document');
	let fileSize = $derived(props.block.content.fileSize || 0);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function getFileIcon(name: string) {
		const ext = name.split('.').pop()?.toLowerCase() || '';
		switch (ext) {
			case 'pdf': return IconFileTypePdf;
			case 'doc':
			case 'docx': return IconFileTypeDoc;
			case 'xls':
			case 'xlsx':
			case 'csv': return IconFileSpreadsheet;
			case 'zip':
			case 'rar':
			case '7z': return IconFileZip;
			default: return IconFile;
		}
	}

	function formatFileSize(bytes: number): string {
		if (bytes === 0) return '';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
	}

	function getFileExtension(name: string): string {
		return name.split('.').pop()?.toUpperCase() || 'FILE';
	}

	let FileIcon = $derived(getFileIcon(fileName));
</script>

<div class="file-block" role="article" aria-label="File download">
	{#if props.isEditing}
		<div class="file-edit">
			<div class="file-icon-wrapper">
				<FileIcon size={28} aria-hidden="true" />
			</div>
			<div class="file-inputs">
				<input
					type="text"
					placeholder="File name (e.g., document.pdf)"
					value={fileName}
					oninput={(e) => updateContent({ fileName: (e.target as HTMLInputElement).value })}
				/>
				<input
					type="url"
					placeholder="File URL"
					value={fileUrl}
					oninput={(e) => updateContent({ fileUrl: (e.target as HTMLInputElement).value })}
				/>
				<input
					type="number"
					placeholder="File size (bytes)"
					value={fileSize || ''}
					oninput={(e) => updateContent({ fileSize: parseInt((e.target as HTMLInputElement).value) || 0 })}
				/>
			</div>
		</div>
	{:else}
		<a
			href={sanitizeURL(fileUrl) || '#'}
			class="file-link"
			download={fileName}
			aria-label="Download {fileName}"
		>
			<div class="file-icon-wrapper">
				<FileIcon size={28} aria-hidden="true" />
			</div>
			<div class="file-info">
				<span class="file-name">{fileName}</span>
				<span class="file-meta">
					<span class="file-type">{getFileExtension(fileName)}</span>
					{#if fileSize > 0}
						<span class="file-size">{formatFileSize(fileSize)}</span>
					{/if}
				</span>
			</div>
			<div class="file-download">
				<IconDownload size={20} aria-hidden="true" />
			</div>
		</a>
	{/if}
</div>

<style>
	.file-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}

	.file-link {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: #f8fafc;
		text-decoration: none;
		color: inherit;
		transition: all 0.15s;
	}

	.file-link:hover {
		background: #f1f5f9;
	}

	.file-link:hover .file-download {
		background: #3b82f6;
		color: white;
	}

	.file-icon-wrapper {
		width: 48px;
		height: 48px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: white;
		border: 1px solid #e5e7eb;
		border-radius: 10px;
		color: #3b82f6;
		flex-shrink: 0;
	}

	.file-info {
		flex: 1;
		min-width: 0;
	}

	.file-name {
		display: block;
		font-weight: 600;
		color: #1e293b;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.file-meta {
		display: flex;
		gap: 0.75rem;
		margin-top: 0.25rem;
		font-size: 0.8125rem;
		color: #64748b;
	}

	.file-type {
		font-weight: 500;
		color: #3b82f6;
	}

	.file-download {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e2e8f0;
		border-radius: 8px;
		color: #64748b;
		transition: all 0.15s;
		flex-shrink: 0;
	}

	.file-edit {
		display: flex;
		align-items: flex-start;
		gap: 1rem;
		padding: 1rem 1.25rem;
	}

	.file-inputs {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.file-inputs input {
		padding: 0.625rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 8px;
		font-size: 0.9375rem;
	}

	.file-inputs input:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	:global(.dark) .file-block { border-color: #334155; }
	:global(.dark) .file-link { background: #1e293b; }
	:global(.dark) .file-link:hover { background: #334155; }
	:global(.dark) .file-icon-wrapper { background: #0f172a; border-color: #334155; }
	:global(.dark) .file-name { color: #f1f5f9; }
	:global(.dark) .file-meta { color: #94a3b8; }
	:global(.dark) .file-download { background: #334155; color: #94a3b8; }
	:global(.dark) .file-inputs input { background: #0f172a; border-color: #475569; color: #e2e8f0; }
</style>

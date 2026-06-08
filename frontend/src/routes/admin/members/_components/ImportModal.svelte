<script lang="ts">
	import { IconUpload, IconX } from '$lib/icons';

	interface Props {
		file: File | null;
		importing: boolean;
		onClose: () => void;
		onFileSelect: (event: Event) => void;
		onRemoveFile: () => void;
		onImport: () => void;
	}

	let { file, importing, onClose, onFileSelect, onRemoveFile, onImport }: Props = $props();
</script>

<div
	class="modal-overlay"
	onclick={(e: MouseEvent) => {
		if (e.target === e.currentTarget) onClose();
	}}
	onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && onClose()}
	role="dialog"
	tabindex="-1"
	aria-modal="true"
>
	<div class="modal-content" role="document">
		<div class="modal-header">
			<h2>Import Members</h2>
			<button class="close-btn" onclick={onClose}>
				<IconX size={20} />
			</button>
		</div>

		<div class="modal-body">
			<div class="import-instructions">
				<h4>CSV Format Requirements</h4>
				<ul>
					<li>First row must contain column headers</li>
					<li>Required columns: <code>email</code>, <code>name</code></li>
					<li>
						Optional: <code>first_name</code>, <code>last_name</code>, <code>phone</code>,
						<code>tags</code>
					</li>
					<li>Maximum file size: 10MB</li>
				</ul>
			</div>

			<div class="file-upload">
				<label for="import-file" class={['upload-zone', { 'has-file': file }]}>
					<IconUpload size={32} />
					{#if file}
						<span class="file-name">{file.name}</span>
						<span class="file-size">{(file.size / 1024).toFixed(1)} KB</span>
					{:else}
						<span>Click to select CSV file</span>
						<span class="upload-hint">or drag and drop</span>
					{/if}
				</label>
				<input
					id="import-file"
					name="import-file"
					type="file"
					accept=".csv"
					onchange={onFileSelect}
					style="display: none"
				/>
			</div>

			{#if file}
				<button class="btn-secondary" style="margin-top: 1rem" onclick={onRemoveFile}>
					<IconX size={16} />
					Remove File
				</button>
			{/if}
		</div>

		<div class="modal-footer">
			<button class="btn-secondary" onclick={onClose}>Cancel</button>
			<button class="btn-primary" onclick={onImport} disabled={!file || importing}>
				<IconUpload size={18} />
				{importing ? 'Importing...' : 'Import Members'}
			</button>
		</div>
	</div>
</div>

<style>
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.85);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 50;
		padding: 2rem;
	}

	.modal-content {
		background: linear-gradient(145deg, var(--bg-elevated) 0%, var(--bg-base) 100%);
		border: 1px solid rgba(230, 184, 0, 0.2);
		border-radius: 16px;
		width: 100%;
		max-width: 600px;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow:
			0 25px 60px -15px rgba(0, 0, 0, 0.7),
			0 0 40px -10px rgba(230, 184, 0, 0.1);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.5);
	}

	.modal-header h2 {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(100, 116, 139, 0.2);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 10px;
		color: var(--text-secondary);
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: rgba(100, 116, 139, 0.3);
		color: var(--text-primary);
	}

	.modal-body {
		padding: 1.5rem;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		padding: 1.5rem;
		border-top: 1px solid var(--border-muted);
		background: rgba(13, 17, 23, 0.3);
	}

	.import-instructions {
		background: rgba(22, 27, 34, 0.6);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		padding: 1rem 1.25rem;
		margin-bottom: 1.5rem;
	}

	.import-instructions h4 {
		color: var(--text-primary);
		font-size: 0.875rem;
		font-weight: 600;
		margin: 0 0 0.75rem;
	}

	.import-instructions ul {
		margin: 0;
		padding-left: 1.25rem;
		color: var(--text-secondary);
		font-size: 0.8125rem;
		line-height: 1.6;
	}

	.import-instructions code {
		background: rgba(230, 184, 0, 0.2);
		color: var(--primary-500);
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-size: 0.75rem;
	}

	.file-upload {
		margin-top: 1rem;
	}

	.upload-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		background: rgba(22, 27, 34, 0.6);
		border: 2px dashed var(--border-default);
		border-radius: 12px;
		cursor: pointer;
		color: var(--text-secondary);
		text-align: center;
		transition: all 0.2s;
	}

	.upload-zone:hover {
		background: rgba(230, 184, 0, 0.1);
		border-color: rgba(230, 184, 0, 0.4);
		color: var(--primary-500);
	}

	.upload-zone.has-file {
		background: rgba(16, 185, 129, 0.1);
		border-color: rgba(16, 185, 129, 0.4);
		color: var(--success-emphasis);
	}

	.upload-zone .file-name {
		font-weight: 600;
		color: var(--text-primary);
		margin-top: 0.75rem;
	}

	.upload-zone .file-size {
		font-size: 0.75rem;
		color: var(--text-tertiary);
	}

	.upload-hint {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		margin-top: 0.25rem;
	}

	.btn-secondary,
	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		border-radius: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: var(--text-primary);
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		color: var(--text-primary);
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: var(--bg-base);
		box-shadow: 0 4px 14px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:hover:not(:disabled) {
		background: linear-gradient(135deg, var(--primary-400), var(--primary-500));
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>

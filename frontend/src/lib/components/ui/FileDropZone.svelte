<!--
	FileDropZone.svelte
	═══════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT Level 7 - January 2026
	
	Drag-and-drop file upload zone with validation.
	Supports file type and size restrictions.
	
	@version 1.0.0
-->
<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';

	interface Props {
		accept?: string[];
		maxSizeBytes?: number;
		onfile: (file: File) => void;
		onerror?: (message: string) => void;
		disabled?: boolean;
		hint?: string;
	}

	let props: Props = $props();
	let accept = $derived(
		props.accept ?? [
			'video/mp4',
			'video/webm',
			'video/quicktime',
			'video/x-msvideo',
			'video/x-matroska'
		]
	);
	let maxSizeBytes = $derived(props.maxSizeBytes ?? 5 * 1024 * 1024 * 1024);
	let onfile = $derived(props.onfile);
	let onerror = $derived(props.onerror);
	let disabled = $derived(props.disabled ?? false);
	let hint = $derived(props.hint ?? 'or use the button below');

	// Local state
	let isDragOver = $state(false);
	let fileInput: HTMLInputElement | null = null;

	// Derived
	const acceptString = $derived(accept.join(','));
	const maxSizeDisplay = $derived(formatFileSize(maxSizeBytes));
	const acceptedFormatsDisplay = $derived(
		accept.map((t) => t.split('/')[1]?.toUpperCase() || t).join(', ')
	);

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
		return (bytes / (1024 * 1024 * 1024)).toFixed(0) + 'GB';
	}

	function validateFile(file: File): string | null {
		if (!accept.includes(file.type)) {
			return `Invalid file type. Accepted: ${acceptedFormatsDisplay}`;
		}
		if (file.size > maxSizeBytes) {
			return `File too large. Maximum ${maxSizeDisplay}.`;
		}
		if (file.size === 0) {
			return 'File is empty.';
		}
		return null;
	}

	function handleFile(file: File) {
		const error = validateFile(file);
		if (error) {
			onerror?.(error);
			return;
		}
		onfile(file);
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		if (!disabled) isDragOver = true;
	}

	function handleDragLeave() {
		isDragOver = false;
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		e.stopPropagation();
		isDragOver = false;

		if (disabled) return;

		const files = e.dataTransfer?.files;
		if (files && files.length > 0) {
			handleFile(files[0]);
		}
	}

	function handleFileInputChange(e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) handleFile(file);
		// Reset input so same file can be selected again
		if (fileInput) fileInput.value = '';
	}

	function openFileBrowser() {
		if (!disabled) fileInput?.click();
	}

	function trackFileInput(element: HTMLInputElement) {
		fileInput = element;

		return () => {
			if (fileInput === element) {
				fileInput = null;
			}
		};
	}
</script>

<div
	class={{ 'drop-zone': true, 'drag-over': isDragOver, disabled }}
	role="region"
	aria-label="File upload area"
	ondragover={handleDragOver}
	ondragleave={handleDragLeave}
	ondrop={handleDrop}
>
	<div class="drop-zone-content">
		<div class="upload-icon">
			<Icon name="IconUpload" size={56} />
		</div>
		<p class="drop-text">Drag and drop file here</p>
		<p class="drop-hint">{hint}</p>
		<span class="supported-formats">{acceptedFormatsDisplay} (max {maxSizeDisplay})</span>

		<button type="button" class="btn-browse" onclick={openFileBrowser} {disabled}>
			<Icon name="IconFolder" size={18} />
			Browse Files & Folders
		</button>
	</div>
</div>

<input
	{@attach trackFileInput}
	type="file"
	accept={acceptString}
	hidden
	onchange={handleFileInputChange}
/>

<style>
	.drop-zone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 24px 20px;
		border: 2px dashed #cbd5e1;
		border-radius: 16px;
		background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
		transition: all 0.2s ease;
		color: #64748b;
		cursor: pointer;
	}

	.drop-zone:hover:not(.disabled) {
		border-color: #94a3b8;
		background: linear-gradient(135deg, #f1f5f9 0%, #e2e8f0 100%);
	}

	.drop-zone.drag-over {
		border-color: #143e59;
		background: rgba(20, 62, 89, 0.08);
		border-style: solid;
		transform: scale(1.01);
	}

	.drop-zone.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.drop-zone-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.upload-icon {
		width: 56px;
		height: 56px;
		color: #143e59;
		margin-bottom: 0.5rem;
	}

	.upload-icon :global(svg) {
		width: 100%;
		height: 100%;
	}

	.drop-text {
		margin: 12px 0 4px;
		font-size: 15px;
		font-weight: 500;
		color: #334155;
	}

	.drop-hint {
		margin: 0;
		font-size: 13px;
		color: #94a3b8;
	}

	.supported-formats {
		font-size: 0.75rem;
		color: #94a3b8;
		margin-top: 0.5rem;
	}

	.btn-browse {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		margin-top: 16px;
		padding: 12px 24px;
		background: linear-gradient(135deg, #143e59 0%, #1a5a7e 100%);
		border: none;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 600;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 2px 8px rgba(20, 62, 89, 0.3);
	}

	.btn-browse:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(20, 62, 89, 0.4);
		background: linear-gradient(135deg, #0f2d42 0%, #143e59 100%);
	}

	.btn-browse:active:not(:disabled) {
		transform: translateY(0);
	}

	.btn-browse:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}
</style>

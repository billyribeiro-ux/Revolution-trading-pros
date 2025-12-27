<script lang="ts">
	/**
	 * FileUploadField Component (FluentForms Pro)
	 *
	 * Advanced file upload with drag-drop, preview, and validation.
	 */

	interface UploadedFile {
		id: string;
		name: string;
		size: number;
		type: string;
		url?: string;
		progress?: number;
		status: 'pending' | 'uploading' | 'complete' | 'error';
		error?: string;
	}

	interface Props {
		name: string;
		value?: UploadedFile[];
		label?: string;
		required?: boolean;
		disabled?: boolean;
		multiple?: boolean;
		maxFiles?: number;
		maxFileSize?: number; // in bytes
		acceptedTypes?: string[]; // e.g., ['image/*', '.pdf']
		showPreview?: boolean;
		uploadEndpoint?: string;
		error?: string;
		helpText?: string;
		onchange?: (files: UploadedFile[]) => void;
		onupload?: (file: File) => Promise<{ url: string; id: string }>;
	}

	let {
		name,
		value = [],
		label = 'Upload Files',
		required = false,
		disabled = false,
		multiple = true,
		maxFiles = 10,
		maxFileSize = 10 * 1024 * 1024, // 10MB
		acceptedTypes = ['image/*', '.pdf', '.doc', '.docx'],
		showPreview = true,
		error = '',
		helpText = '',
		onchange,
		onupload
	}: Props = $props();

	let files = $state<UploadedFile[]>([]);
	let isDragging = $state(false);
	let fileInput = $state<HTMLInputElement>();

	// Sync files with value prop
	$effect(() => {
		if (value !== undefined) files = [...value];
	});

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
	}

	function getFileIcon(type: string): string {
		if (type.startsWith('image/')) return '🖼️';
		if (type.includes('pdf')) return '📄';
		if (type.includes('word') || type.includes('document')) return '📝';
		if (type.includes('excel') || type.includes('spreadsheet')) return '📊';
		if (type.includes('video')) return '🎬';
		if (type.includes('audio')) return '🎵';
		return '📁';
	}

	function validateFile(file: File): string | null {
		if (file.size > maxFileSize) {
			return `File too large. Maximum size is ${formatFileSize(maxFileSize)}`;
		}

		if (acceptedTypes.length > 0) {
			const isAccepted = acceptedTypes.some((type) => {
				if (type.endsWith('/*')) {
					return file.type.startsWith(type.replace('/*', '/'));
				}
				if (type.startsWith('.')) {
					return file.name.toLowerCase().endsWith(type.toLowerCase());
				}
				return file.type === type;
			});

			if (!isAccepted) {
				return `File type not accepted. Allowed: ${acceptedTypes.join(', ')}`;
			}
		}

		return null;
	}

	async function handleFiles(fileList: FileList) {
		if (disabled) return;

		const newFiles: UploadedFile[] = [];

		for (const file of Array.from(fileList)) {
			if (!multiple && files.length + newFiles.length >= 1) break;
			if (files.length + newFiles.length >= maxFiles) break;

			const validationError = validateFile(file);
			const uploadedFile: UploadedFile = {
				id: crypto.randomUUID(),
				name: file.name,
				size: file.size,
				type: file.type,
				status: validationError ? 'error' : 'pending',
				error: validationError || undefined,
				progress: 0
			};

			newFiles.push(uploadedFile);

			if (!validationError && onupload) {
				uploadedFile.status = 'uploading';
				try {
					const result = await onupload(file);
					uploadedFile.url = result.url;
					uploadedFile.id = result.id;
					uploadedFile.status = 'complete';
					uploadedFile.progress = 100;
				} catch (err) {
					uploadedFile.status = 'error';
					uploadedFile.error = err instanceof Error ? err.message : 'Upload failed';
				}
			} else if (!validationError) {
				// Create local preview URL
				uploadedFile.url = URL.createObjectURL(file);
				uploadedFile.status = 'complete';
				uploadedFile.progress = 100;
			}
		}

		files = [...files, ...newFiles];
		if (onchange) onchange(files);
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
		if (e.dataTransfer?.files) {
			handleFiles(e.dataTransfer.files);
		}
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	function handleDragLeave() {
		isDragging = false;
	}

	function handleInputChange(e: Event) {
		const target = e.target as HTMLInputElement;
		if (target.files) {
			handleFiles(target.files);
		}
	}

	function removeFile(fileId: string) {
		files = files.filter((f) => f.id !== fileId);
		if (onchange) onchange(files);
	}

	function openFilePicker() {
		fileInput?.click();
	}

	const canAddMore = $derived(!multiple ? files.length < 1 : files.length < maxFiles);
	const acceptString = $derived(acceptedTypes.join(','));
</script>

<div class="file-upload-field" class:disabled class:has-error={error}>
	{#if label}
		<label class="field-label" for="{name}-upload-area">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</label>
	{/if}

	{#if canAddMore}
		<div
			id="{name}-upload-area"
			class="dropzone"
			class:dragging={isDragging}
			ondrop={handleDrop}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			onclick={openFilePicker}
			onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && openFilePicker()}
			role="button"
			tabindex="0"
			aria-label={label || 'Upload files'}
		>
			<input
				bind:this={fileInput}
				type="file"
				accept={acceptString}
				{multiple}
				onchange={handleInputChange}
				{disabled}
				class="file-input"
			/>

			<div class="dropzone-content">
				<svg class="upload-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
					<polyline points="17 8 12 3 7 8"></polyline>
					<line x1="12" y1="3" x2="12" y2="15"></line>
				</svg>
				<p class="dropzone-text">
					<span class="dropzone-link">Click to upload</span> or drag and drop
				</p>
				<p class="dropzone-hint">
					{acceptedTypes.join(', ')} up to {formatFileSize(maxFileSize)}
				</p>
			</div>
		</div>
	{/if}

	{#if files.length > 0}
		<ul class="file-list">
			{#each files as file (file.id)}
				<li class="file-item" class:error={file.status === 'error'}>
					{#if showPreview && file.type.startsWith('image/') && file.url}
						<img src={file.url} alt={file.name} class="file-preview" />
					{:else}
						<span class="file-icon">{getFileIcon(file.type)}</span>
					{/if}

					<div class="file-info">
						<span class="file-name">{file.name}</span>
						<span class="file-size">{formatFileSize(file.size)}</span>
						{#if file.status === 'uploading'}
							<div class="progress-bar">
								<div class="progress-fill" style="width: {file.progress}%"></div>
							</div>
						{/if}
						{#if file.error}
							<span class="file-error">{file.error}</span>
						{/if}
					</div>

					<button
						type="button"
						class="remove-btn"
						onclick={() => removeFile(file.id)}
						title="Remove file"
					>
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="18" y1="6" x2="6" y2="18"></line>
							<line x1="6" y1="6" x2="18" y2="18"></line>
						</svg>
					</button>

					<input type="hidden" name="{name}[]" value={file.id} />
				</li>
			{/each}
		</ul>
	{/if}

	{#if helpText && !error}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}
</div>

<style>
	.file-upload-field {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.field-label {
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
	}

	.required {
		color: #ef4444;
		margin-left: 0.25rem;
	}

	.dropzone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		border: 2px dashed #d1d5db;
		border-radius: 0.75rem;
		background-color: #f9fafb;
		cursor: pointer;
		transition: all 0.2s;
	}

	.dropzone:hover {
		border-color: #3b82f6;
		background-color: #eff6ff;
	}

	.dropzone.dragging {
		border-color: #3b82f6;
		background-color: #dbeafe;
	}

	.file-input {
		display: none;
	}

	.dropzone-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
	}

	.upload-icon {
		width: 40px;
		height: 40px;
		color: #9ca3af;
	}

	.dropzone-text {
		font-size: 0.9375rem;
		color: #4b5563;
		margin: 0;
	}

	.dropzone-link {
		color: #3b82f6;
		font-weight: 500;
	}

	.dropzone-hint {
		font-size: 0.75rem;
		color: #9ca3af;
		margin: 0;
	}

	.file-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.file-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background-color: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
	}

	.file-item.error {
		background-color: #fef2f2;
		border-color: #fecaca;
	}

	.file-preview {
		width: 48px;
		height: 48px;
		object-fit: cover;
		border-radius: 0.375rem;
	}

	.file-icon {
		font-size: 2rem;
		width: 48px;
		text-align: center;
	}

	.file-info {
		flex: 1;
		min-width: 0;
	}

	.file-name {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: #374151;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.file-size {
		font-size: 0.75rem;
		color: #6b7280;
	}

	.file-error {
		display: block;
		font-size: 0.75rem;
		color: #ef4444;
	}

	.progress-bar {
		height: 4px;
		background-color: #e5e7eb;
		border-radius: 2px;
		overflow: hidden;
		margin-top: 0.25rem;
	}

	.progress-fill {
		height: 100%;
		background-color: #3b82f6;
		transition: width 0.2s;
	}

	.remove-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background-color: transparent;
		color: #9ca3af;
		cursor: pointer;
		border-radius: 0.375rem;
		transition: all 0.15s;
	}

	.remove-btn:hover {
		background-color: #fee2e2;
		color: #ef4444;
	}

	.remove-btn svg {
		width: 16px;
		height: 16px;
	}

	.help-text {
		font-size: 0.75rem;
		color: #6b7280;
		margin: 0;
	}

	.error-text {
		font-size: 0.75rem;
		color: #ef4444;
		margin: 0;
	}

	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	.has-error .dropzone {
		border-color: #fca5a5;
	}
</style>

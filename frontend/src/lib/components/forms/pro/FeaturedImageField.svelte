<script lang="ts">
	/**
	 * FeaturedImageField Component (FluentForms Pro)
	 *
	 * Image upload field for post featured images with preview.
	 */

	interface Props {
		name?: string;
		value?: string;
		label?: string;
		required?: boolean;
		disabled?: boolean;
		maxFileSize?: number;
		acceptedTypes?: string[];
		aspectRatio?: '16:9' | '4:3' | '1:1' | 'free';
		error?: string;
		helpText?: string;
		onchange?: (file: File | null, url: string | null) => void;
		onupload?: (file: File) => Promise<{ url: string; id: string }>;
	}

	let {
		name = 'featured_image',
		value = '',
		label = 'Featured Image',
		required = false,
		disabled = false,
		maxFileSize = 5 * 1024 * 1024, // 5MB
		acceptedTypes = ['image/jpeg', 'image/png', 'image/gif', 'image/webp'],
		aspectRatio = 'free',
		error = '',
		helpText = '',
		onchange,
		onupload
	}: Props = $props();

	let imageUrl = $state('');
	let imageId = $state('');
	let isDragging = $state(false);
	let uploading = $state(false);
	let uploadError = $state('');
	let fileInput: HTMLInputElement;

	// Sync imageUrl with value prop
	$effect(() => {
		if (value !== undefined) imageUrl = value;
	});

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
	}

	function validateFile(file: File): string | null {
		if (file.size > maxFileSize) {
			return `File too large. Maximum size is ${formatFileSize(maxFileSize)}`;
		}
		if (!acceptedTypes.includes(file.type)) {
			return 'Invalid file type. Please upload an image (JPEG, PNG, GIF, or WebP)';
		}
		return null;
	}

	async function handleFile(file: File) {
		const validationError = validateFile(file);
		if (validationError) {
			uploadError = validationError;
			return;
		}

		uploadError = '';
		uploading = true;

		try {
			if (onupload) {
				const result = await onupload(file);
				imageUrl = result.url;
				imageId = result.id;
			} else {
				// Create local preview
				imageUrl = URL.createObjectURL(file);
				imageId = '';
			}

			if (onchange) onchange(file, imageUrl);
		} catch (err) {
			uploadError = err instanceof Error ? err.message : 'Upload failed';
			imageUrl = '';
		} finally {
			uploading = false;
		}
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;

		if (disabled) return;

		const file = e.dataTransfer?.files[0];
		if (file) {
			handleFile(file);
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
		const file = target.files?.[0];
		if (file) {
			handleFile(file);
		}
	}

	function removeImage() {
		imageUrl = '';
		imageId = '';
		uploadError = '';
		if (fileInput) fileInput.value = '';
		if (onchange) onchange(null, null);
	}

	function openFilePicker() {
		fileInput?.click();
	}

	const aspectRatioClass = $derived({
		'16:9': 'aspect-video',
		'4:3': 'aspect-4-3',
		'1:1': 'aspect-square',
		free: 'aspect-free'
	}[aspectRatio]);
</script>

<div class="featured-image-field" class:disabled class:has-error={error || uploadError}>
	{#if label}
		<label for="featured-image-{name}" class="field-label">
			{label}
			{#if required}
				<span class="required">*</span>
			{/if}
		</label>
	{/if}

	{#if imageUrl}
		<div class="image-preview {aspectRatioClass}">
			<img src={imageUrl} alt="Featured preview" />
			<div class="preview-overlay">
				<button type="button" class="change-btn" onclick={openFilePicker} {disabled}>
					Change
				</button>
				<button type="button" class="remove-btn" onclick={removeImage} {disabled}>
					Remove
				</button>
			</div>
		</div>
	{:else}
		<div
			class="dropzone {aspectRatioClass}"
			class:dragging={isDragging}
			class:uploading
			ondrop={handleDrop}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			onclick={openFilePicker}
			onkeydown={(e) => e.key === 'Enter' && openFilePicker()}
			role="button"
			tabindex="0"
		>
			{#if uploading}
				<div class="upload-progress">
					<svg class="spinner" viewBox="0 0 24 24">
						<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" />
					</svg>
					<span>Uploading...</span>
				</div>
			{:else}
				<svg class="upload-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
					<circle cx="8.5" cy="8.5" r="1.5"></circle>
					<polyline points="21 15 16 10 5 21"></polyline>
				</svg>
				<p class="dropzone-text">
					<span class="dropzone-link">Click to upload</span> or drag and drop
				</p>
				<p class="dropzone-hint">
					{acceptedTypes.map((t) => t.replace('image/', '')).join(', ')} up to {formatFileSize(maxFileSize)}
				</p>
			{/if}
		</div>
	{/if}

	<input
		id="featured-image-{name}"
		bind:this={fileInput}
		type="file"
		accept={acceptedTypes.join(',')}
		onchange={handleInputChange}
		{disabled}
		class="file-input"
	/>

	{#if uploadError}
		<p class="error-text">{uploadError}</p>
	{/if}

	{#if helpText && !error && !uploadError}
		<p class="help-text">{helpText}</p>
	{/if}

	{#if error}
		<p class="error-text">{error}</p>
	{/if}

	<!-- Hidden inputs for form submission -->
	<input type="hidden" name={name} value={imageUrl} />
	{#if imageId}
		<input type="hidden" name="{name}_id" value={imageId} />
	{/if}
</div>

<style>
	.featured-image-field {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
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

	.dropzone.uploading {
		pointer-events: none;
	}

	.aspect-video {
		aspect-ratio: 16 / 9;
	}

	.aspect-4-3 {
		aspect-ratio: 4 / 3;
	}

	.aspect-square {
		aspect-ratio: 1;
	}

	.aspect-free {
		min-height: 200px;
	}

	.upload-icon {
		width: 48px;
		height: 48px;
		color: #9ca3af;
		margin-bottom: 0.75rem;
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
		margin: 0.5rem 0 0 0;
	}

	.upload-progress {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		color: #3b82f6;
	}

	.spinner {
		width: 32px;
		height: 32px;
		animation: spin 1s linear infinite;
	}

	.spinner circle {
		stroke-dasharray: 60;
		stroke-dashoffset: 45;
		stroke-linecap: round;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.image-preview {
		position: relative;
		border-radius: 0.75rem;
		overflow: hidden;
		background-color: #f3f4f6;
	}

	.image-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.preview-overlay {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		background-color: rgba(0, 0, 0, 0.5);
		opacity: 0;
		transition: opacity 0.2s;
	}

	.image-preview:hover .preview-overlay {
		opacity: 1;
	}

	.change-btn,
	.remove-btn {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.change-btn {
		background-color: white;
		color: #374151;
	}

	.change-btn:hover {
		background-color: #f3f4f6;
	}

	.remove-btn {
		background-color: #ef4444;
		color: white;
	}

	.remove-btn:hover {
		background-color: #dc2626;
	}

	.file-input {
		display: none;
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

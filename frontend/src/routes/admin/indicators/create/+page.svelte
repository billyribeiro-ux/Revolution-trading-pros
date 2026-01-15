<!--
	URL: /admin/indicators/create
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import { IconPhoto, IconX, IconSparkles, IconCheck } from '$lib/icons';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ICT 7 FIX: Match backend CreateIndicatorRequest schema
	interface IndicatorForm {
		name: string;
		slug: string;
		description: string;
		long_description: string;
		price: number | null;
		thumbnail: string;
		platform: string;
		version: string;
		download_url: string;
		documentation_url: string;
		is_active: boolean;
	}

	let indicator = $state<IndicatorForm>({
		name: '',
		slug: '',
		description: '',
		long_description: '',
		price: null,
		thumbnail: '',
		platform: '',
		version: '1.0',
		download_url: '',
		documentation_url: '',
		is_active: true
	});

	let uploading = $state(false);
	let saving = $state(false);
	let uploadError = $state('');
	let formError = $state('');
	let successMessage = $state('');

	function generateSlug() {
		if (!indicator.slug && indicator.name) {
			indicator.slug = indicator.name
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
	}

	/**
	 * Resize image to max dimensions while maintaining aspect ratio
	 * Converts to optimized format for faster uploads
	 */
	async function resizeImage(file: File, maxWidth = 1200, maxHeight = 1200, quality = 0.85): Promise<Blob> {
		return new Promise((resolve, reject) => {
			const img = new Image();
			const canvas = document.createElement('canvas');
			const ctx = canvas.getContext('2d');

			img.onload = () => {
				let { width, height } = img;

				// Calculate new dimensions maintaining aspect ratio
				if (width > maxWidth || height > maxHeight) {
					const ratio = Math.min(maxWidth / width, maxHeight / height);
					width = Math.round(width * ratio);
					height = Math.round(height * ratio);
				}

				canvas.width = width;
				canvas.height = height;

				if (ctx) {
					// Use high-quality image smoothing
					ctx.imageSmoothingEnabled = true;
					ctx.imageSmoothingQuality = 'high';
					ctx.drawImage(img, 0, 0, width, height);

					canvas.toBlob(
						(blob) => {
							if (blob) {
								resolve(blob);
							} else {
								reject(new Error('Failed to create blob from canvas'));
							}
						},
						'image/jpeg',
						quality
					);
				} else {
					reject(new Error('Could not get canvas context'));
				}
			};

			img.onerror = () => reject(new Error('Failed to load image'));
			img.src = URL.createObjectURL(file);
		});
	}

	async function handleImageUpload(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];

		if (!file) return;

		uploadError = '';

		// Validate file type
		if (!file.type.startsWith('image/')) {
			uploadError = 'Please select an image file (JPEG, PNG, WebP, etc.)';
			return;
		}

		// Validate file size (max 50MB before resize)
		if (file.size > 50 * 1024 * 1024) {
			uploadError = 'Image must be less than 50MB';
			return;
		}

		uploading = true;
		try {
			// Resize image before upload (max 1200px, converts to JPEG)
			const resizedBlob = await resizeImage(file, 1200, 1200, 0.85);
			const resizedFile = new File([resizedBlob], file.name.replace(/\.[^.]+$/, '.jpg'), {
				type: 'image/jpeg'
			});

			const formData = new FormData();
			formData.append('file', resizedFile);

			const response = await adminFetch('/api/admin/media/upload', {
				method: 'POST',
				body: formData
			});

			// The response contains an array of uploaded files
			if (response.success && response.data && response.data.length > 0) {
				indicator.thumbnail = response.data[0].url;
				uploadError = '';
			} else {
				throw new Error(response.message || 'Upload failed - no URL returned');
			}
		} catch (error: any) {
			console.error('Failed to upload image:', error);
			uploadError = error.message || 'Failed to upload image. Please try again.';

			// Fallback: Use local preview if server upload fails
			indicator.thumbnail = URL.createObjectURL(file);
		} finally {
			uploading = false;
		}
	}

	async function saveIndicator() {
		formError = '';
		successMessage = '';

		if (!indicator.name) {
			formError = 'Please fill in the required field: Name';
			return;
		}

		saving = true;
		try {
			// ICT 7 FIX: Use correct indicators API endpoint
			const payload = {
				name: indicator.name,
				slug: indicator.slug || undefined,
				description: indicator.description || undefined,
				long_description: indicator.long_description || undefined,
				price: indicator.price ?? 0,
				thumbnail: indicator.thumbnail || undefined,
				platform: indicator.platform || undefined,
				version: indicator.version || undefined,
				download_url: indicator.download_url || undefined,
				documentation_url: indicator.documentation_url || undefined
			};

			const response = await adminFetch('/api/admin/indicators', {
				method: 'POST',
				body: JSON.stringify(payload)
			});

			if (response.success) {
				successMessage = 'Indicator created successfully! Redirecting...';
				setTimeout(() => goto('/admin/indicators'), 1500);
			} else {
				formError = response.error || 'Failed to create indicator';
			}
		} catch (error: any) {
			console.error('Failed to save indicator:', error);
			formError = error.message || 'Failed to save indicator. Please try again.';
		} finally {
			saving = false;
		}
	}
</script>

<svelte:head>
	<title>Add New Indicator | Admin</title>
</svelte:head>

<div class="create-page">
	<div class="page-header">
		<div>
			<h1><IconSparkles size={32} /> Add New Indicator</h1>
			<p>Create a new trading indicator for your platform</p>
		</div>
	</div>

	<div class="content-container">
		<!-- Preview Card -->
		<div class="preview-section">
			<h3>Preview</h3>
			<div class="product-card">
				<div class="card-image">
					{#if indicator.thumbnail}
						<img src={indicator.thumbnail} alt={indicator.name || 'Indicator'} />
					{:else}
						<div class="placeholder-image">
							<IconPhoto size={48} />
							<span>No image</span>
						</div>
					{/if}
				</div>
				<div class="card-content">
					<h4>{indicator.name || 'Indicator Name'}</h4>
					<p>{indicator.description || 'Add a description...'}</p>
					<div class="card-footer">
						<div class="price">${indicator.price || '0.00'}</div>
						<button class="buy-btn" disabled>
							<IconCheck size={16} />
							Purchase
						</button>
					</div>
				</div>
			</div>
		</div>

		<!-- Form Section -->
		<div class="form-section">
			<div class="form-card">
				<h3>Indicator Details</h3>

				<div class="form-group">
					<label for="name">Indicator Name *</label>
					<input
						id="name"
						type="text"
						bind:value={indicator.name}
						onblur={generateSlug}
						placeholder="e.g., Advanced RSI Indicator"
					/>
				</div>

				<div class="form-group">
					<label for="slug">URL Slug</label>
					<input
						id="slug"
						type="text"
						bind:value={indicator.slug}
						placeholder="advanced-rsi-indicator"
					/>
				</div>

				<div class="form-group">
					<label for="description">Short Description</label>
					<textarea
						id="description"
						bind:value={indicator.description}
						placeholder="Brief description for listings..."
						rows="2"
					></textarea>
				</div>

				<div class="form-group">
					<label for="long_description">Full Description</label>
					<textarea
						id="long_description"
						bind:value={indicator.long_description}
						placeholder="Detailed description of the indicator, features, usage..."
						rows="6"
					></textarea>
				</div>

				<div class="form-row">
					<div class="form-group">
						<label for="platform">Platform</label>
						<select id="platform" bind:value={indicator.platform}>
							<option value="">Select Platform</option>
							<option value="thinkorswim">thinkorswim</option>
							<option value="tradingview">TradingView</option>
							<option value="metatrader4">MetaTrader 4</option>
							<option value="metatrader5">MetaTrader 5</option>
							<option value="ninjatrader">NinjaTrader</option>
							<option value="tradestation">TradeStation</option>
						</select>
					</div>

					<div class="form-group">
						<label for="version">Version</label>
						<input
							id="version"
							type="text"
							bind:value={indicator.version}
							placeholder="1.0"
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="download_url">Download URL</label>
					<input
						id="download_url"
						type="url"
						bind:value={indicator.download_url}
						placeholder="https://..."
					/>
				</div>

				<div class="form-group">
					<label for="documentation_url">Documentation URL</label>
					<input
						id="documentation_url"
						type="url"
						bind:value={indicator.documentation_url}
						placeholder="https://docs.example.com/..."
					/>
				</div>

				<div class="form-group">
					<label for="price">Price (USD)</label>
					<div class="price-input">
						<span class="currency">$</span>
						<input
							id="price"
							type="number"
							bind:value={indicator.price}
							placeholder="99.00"
							step="0.01"
							min="0"
						/>
					</div>
				</div>

				<div class="form-group">
					<label for="thumbnail-upload">Thumbnail Image</label>
					{#if indicator.thumbnail}
						<div class="image-preview">
							<img src={indicator.thumbnail} alt="Thumbnail" />
							<button type="button" class="remove-btn" onclick={() => (indicator.thumbnail = '')}>
								<IconX size={16} />
							</button>
						</div>
					{/if}
					<label for="thumbnail-upload" class="upload-btn" class:uploading>
						<IconPhoto size={20} />
						{uploading ? 'Uploading & Resizing...' : 'Upload Image'}
						<input
							id="thumbnail-upload"
							type="file"
							accept="image/*"
							onchange={handleImageUpload}
							disabled={uploading}
						/>
					</label>
					{#if uploadError}
						<div class="upload-error">{uploadError}</div>
					{/if}
					<div class="upload-hint">Images will be automatically resized to max 1200px</div>
				</div>

				<div class="form-group">
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={indicator.is_active} />
						<span>Active (visible to customers)</span>
					</label>
				</div>

				{#if formError}
					<div class="form-error">{formError}</div>
				{/if}

				{#if successMessage}
					<div class="form-success">{successMessage}</div>
				{/if}

				<div class="form-actions">
					<button class="btn-secondary" onclick={() => goto('/admin/indicators')}> Cancel </button>
					<button class="btn-primary" onclick={saveIndicator} disabled={saving}>
						{saving ? 'Creating...' : 'Create Indicator'}
					</button>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.create-page {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.page-header {
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.page-header p {
		color: #94a3b8;
		font-size: 1rem;
	}

	.content-container {
		display: grid;
		grid-template-columns: 400px 1fr;
		gap: 2rem;
		align-items: start;
	}

	.preview-section h3,
	.form-section h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #f1f5f9;
		margin-bottom: 1rem;
	}

	/* Product Card Preview */
	.product-card {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border-radius: 20px;
		overflow: hidden;
		border: 2px solid transparent;
		background-clip: padding-box;
		position: relative;
		transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
		position: sticky;
		top: 2rem;
	}

	.product-card::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: 20px;
		padding: 2px;
		background: linear-gradient(135deg, #E6B800, #B38F00, #FFD11A);
		-webkit-mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		mask:
			linear-gradient(#fff 0 0) content-box,
			linear-gradient(#fff 0 0);
		-webkit-mask-composite: xor;
		mask-composite: exclude;
		opacity: 0.5;
		transition: opacity 0.4s;
	}

	.product-card:hover {
		transform: translateY(-8px) scale(1.02);
		box-shadow:
			0 25px 50px -12px rgba(230, 184, 0, 0.4),
			0 0 0 1px rgba(230, 184, 0, 0.1),
			inset 0 1px 0 rgba(255, 255, 255, 0.1);
	}

	.product-card:hover::before {
		opacity: 1;
	}

	.card-image {
		width: 100%;
		height: 250px;
		background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		overflow: hidden;
	}

	.card-image::after {
		content: '';
		position: absolute;
		inset: 0;
		background:
			radial-gradient(circle at 20% 50%, rgba(230, 184, 0, 0.3) 0%, transparent 50%),
			radial-gradient(circle at 80% 80%, rgba(230, 184, 0, 0.3) 0%, transparent 50%);
		animation: shimmer 3s ease-in-out infinite;
	}

	@keyframes shimmer {
		0%,
		100% {
			opacity: 0.5;
		}
		50% {
			opacity: 1;
		}
	}

	.card-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		position: relative;
		z-index: 1;
	}

	.placeholder-image {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5rem;
		color: #64748b;
	}

	.card-content {
		padding: 1.5rem;
	}

	.card-content h4 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.card-content p {
		color: #94a3b8;
		font-size: 0.9375rem;
		line-height: 1.6;
		margin-bottom: 1.5rem;
		min-height: 3rem;
	}

	.card-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.price {
		font-size: 2rem;
		font-weight: 700;
		background: linear-gradient(135deg, #E6B800, #B38F00, #FFD11A);
		background-size: 200% 200%;
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		animation: gradient-shift 3s ease infinite;
		position: relative;
	}

	.price::before {
		content: attr(data-price);
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		filter: blur(8px);
		opacity: 0.5;
	}

	@keyframes gradient-shift {
		0%,
		100% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
	}

	.buy-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: #0D1117;
		border: none;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
		overflow: hidden;
	}

	.buy-btn::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #B38F00, #FFD11A);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.buy-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow:
			0 12px 24px rgba(230, 184, 0, 0.4),
			0 0 20px rgba(230, 184, 0, 0.3);
	}

	.buy-btn:hover:not(:disabled)::before {
		opacity: 1;
	}

	.buy-btn :global(svg) {
		position: relative;
		z-index: 1;
	}

	.buy-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	/* Form Section */
	.form-card {
		background: linear-gradient(135deg, #1e293b 0%, #1a2332 100%);
		border-radius: 20px;
		padding: 2rem;
		border: 1px solid rgba(230, 184, 0, 0.15);
		box-shadow:
			0 4px 6px rgba(0, 0, 0, 0.1),
			inset 0 1px 0 rgba(255, 255, 255, 0.05);
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.form-row .form-group {
		margin-bottom: 0;
	}

	.form-group label {
		display: block;
		font-weight: 500;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
		font-size: 0.9375rem;
	}

	.form-group input[type='text'],
	.form-group input[type='number'],
	.form-group input[type='url'],
	.form-group select,
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.form-group select {
		cursor: pointer;
		appearance: none;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='24' height='24' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: right 0.75rem center;
		background-size: 1rem;
		padding-right: 2.5rem;
	}

	.form-group select option {
		background: #1e293b;
		color: #f1f5f9;
	}

	.form-group input::placeholder,
	.form-group textarea::placeholder {
		color: #64748b;
		opacity: 1;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #E6B800;
		background: rgba(15, 23, 42, 0.8);
		box-shadow:
			0 0 0 3px rgba(230, 184, 0, 0.15),
			0 4px 12px rgba(230, 184, 0, 0.1);
		transform: translateY(-1px);
	}

	.form-group textarea {
		resize: vertical;
		font-family: inherit;
	}

	.price-input {
		position: relative;
		display: flex;
		align-items: center;
	}

	.price-input .currency {
		position: absolute;
		left: 1rem;
		color: #E6B800;
		font-weight: 600;
		font-size: 1.125rem;
	}

	.price-input input {
		padding-left: 2.5rem;
	}

	.image-preview {
		position: relative;
		width: 100%;
		height: 200px;
		border-radius: 8px;
		overflow: hidden;
		margin-bottom: 1rem;
	}

	.image-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.remove-btn {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		background: rgba(239, 68, 68, 0.9);
		color: white;
		border: none;
		border-radius: 6px;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: all 0.2s;
	}

	.remove-btn:hover {
		background: rgba(239, 68, 68, 1);
		transform: scale(1.1);
	}

	.upload-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: rgba(230, 184, 0, 0.1);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 8px;
		color: #FFD11A;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.upload-btn:hover {
		background: rgba(230, 184, 0, 0.2);
		border-color: rgba(230, 184, 0, 0.5);
	}

	.upload-btn.uploading {
		opacity: 0.6;
		cursor: wait;
	}

	.upload-btn input {
		display: none;
	}

	.upload-error,
	.form-error {
		margin-top: 0.5rem;
		padding: 0.75rem 1rem;
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 8px;
		color: #fca5a5;
		font-size: 0.875rem;
	}

	.form-error {
		margin-bottom: 1rem;
	}

	.form-success {
		margin-top: 0.5rem;
		margin-bottom: 1rem;
		padding: 0.75rem 1rem;
		background: rgba(34, 197, 94, 0.15);
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 8px;
		color: #86efac;
		font-size: 0.875rem;
	}

	.upload-hint {
		margin-top: 0.5rem;
		color: #64748b;
		font-size: 0.8125rem;
		font-style: italic;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		cursor: pointer;
		color: #f1f5f9;
	}

	.checkbox-label input[type='checkbox'] {
		width: 20px;
		height: 20px;
		cursor: pointer;
	}

	.form-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 2rem;
		padding-top: 2rem;
		border-top: 1px solid rgba(230, 184, 0, 0.1);
	}

	.btn-secondary,
	.btn-primary {
		padding: 0.75rem 1.5rem;
		border-radius: 10px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		border: none;
		position: relative;
		overflow: hidden;
	}

	.btn-secondary {
		background: rgba(230, 184, 0, 0.1);
		color: #FFD11A;
		border: 1px solid rgba(230, 184, 0, 0.3);
	}

	.btn-secondary::before {
		content: '';
		position: absolute;
		inset: 0;
		background: rgba(230, 184, 0, 0.2);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.btn-secondary:hover::before {
		opacity: 1;
	}

	.btn-secondary:hover {
		border-color: rgba(230, 184, 0, 0.5);
		transform: translateY(-1px);
	}

	.btn-primary {
		background: linear-gradient(135deg, #E6B800 0%, #B38F00 100%);
		color: #0D1117;
	}

	.btn-primary::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #B38F00, #FFD11A);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow:
			0 12px 24px rgba(230, 184, 0, 0.4),
			0 0 20px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:hover:not(:disabled)::before {
		opacity: 1;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	@media (max-width: 1024px) {
		.content-container {
			grid-template-columns: 1fr;
		}

		.product-card {
			position: static;
		}
	}
</style>

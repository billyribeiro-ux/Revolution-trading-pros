<script lang="ts">
	import { goto } from '$app/navigation';
	import { IconPhoto, IconX, IconSparkles, IconCheck } from '@tabler/icons-svelte';

	let indicator = {
		name: '',
		slug: '',
		description: '',
		price: '',
		thumbnail: '',
		type: 'indicator',
		is_active: true
	};

	let uploading = false;
	let saving = false;

	function generateSlug() {
		if (!indicator.slug && indicator.name) {
			indicator.slug = indicator.name
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
	}

	async function handleImageUpload(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];

		if (!file) return;

		uploading = true;
		try {
			const formData = new FormData();
			formData.append('file', file);

			const response = await fetch('/api/uploads/file', {
				method: 'POST',
				body: formData
			});

			if (response.ok) {
				const data = await response.json();
				indicator.thumbnail = data.url;
			}
		} catch (error) {
			console.error('Failed to upload image:', error);
		} finally {
			uploading = false;
		}
	}

	async function saveIndicator() {
		if (!indicator.name || !indicator.price) {
			alert('Please fill in name and price');
			return;
		}

		saving = true;
		try {
			const response = await fetch('/api/admin/products', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(indicator)
			});

			if (response.ok) {
				goto('/admin/indicators');
			} else {
				const error = await response.json();
				alert(error.message || 'Failed to create indicator');
			}
		} catch (error) {
			console.error('Failed to save indicator:', error);
			alert('Failed to save indicator');
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
						on:blur={generateSlug}
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
					<label for="description">Description</label>
					<textarea
						id="description"
						bind:value={indicator.description}
						placeholder="Describe what makes this indicator special..."
						rows="4"
					></textarea>
				</div>

				<div class="form-group">
					<label for="price">Price (USD) *</label>
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
							<button type="button" class="remove-btn" on:click={() => (indicator.thumbnail = '')}>
								<IconX size={16} />
							</button>
						</div>
					{/if}
					<label for="thumbnail-upload" class="upload-btn" class:uploading>
						<IconPhoto size={20} />
						{uploading ? 'Uploading...' : 'Upload Image'}
						<input
							id="thumbnail-upload"
							type="file"
							accept="image/*"
							on:change={handleImageUpload}
							disabled={uploading}
						/>
					</label>
				</div>

				<div class="form-group">
					<label class="checkbox-label">
						<input type="checkbox" bind:checked={indicator.is_active} />
						<span>Active (visible to customers)</span>
					</label>
				</div>

				<div class="form-actions">
					<button class="btn-secondary" on:click={() => goto('/admin/indicators')}> Cancel </button>
					<button class="btn-primary" on:click={saveIndicator} disabled={saving}>
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6, #a855f7);
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
			0 25px 50px -12px rgba(99, 102, 241, 0.4),
			0 0 0 1px rgba(99, 102, 241, 0.1),
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
			radial-gradient(circle at 20% 50%, rgba(99, 102, 241, 0.3) 0%, transparent 50%),
			radial-gradient(circle at 80% 80%, rgba(139, 92, 246, 0.3) 0%, transparent 50%);
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6, #a855f7);
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
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
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
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
		background: linear-gradient(135deg, #8b5cf6, #a855f7);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.buy-btn:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow:
			0 12px 24px rgba(99, 102, 241, 0.4),
			0 0 20px rgba(139, 92, 246, 0.3);
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
		border: 1px solid rgba(99, 102, 241, 0.15);
		box-shadow:
			0 4px 6px rgba(0, 0, 0, 0.1),
			inset 0 1px 0 rgba(255, 255, 255, 0.05);
	}

	.form-group {
		margin-bottom: 1.5rem;
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
	.form-group textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		backdrop-filter: blur(10px);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.form-group input:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: #6366f1;
		background: rgba(15, 23, 42, 0.8);
		box-shadow:
			0 0 0 3px rgba(99, 102, 241, 0.15),
			0 4px 12px rgba(99, 102, 241, 0.1);
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
		color: #6366f1;
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
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.3);
		border-radius: 8px;
		color: #a5b4fc;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.upload-btn:hover {
		background: rgba(99, 102, 241, 0.2);
		border-color: rgba(99, 102, 241, 0.5);
	}

	.upload-btn.uploading {
		opacity: 0.6;
		cursor: wait;
	}

	.upload-btn input {
		display: none;
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
		border-top: 1px solid rgba(99, 102, 241, 0.1);
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
		background: rgba(99, 102, 241, 0.1);
		color: #a5b4fc;
		border: 1px solid rgba(99, 102, 241, 0.3);
	}

	.btn-secondary::before {
		content: '';
		position: absolute;
		inset: 0;
		background: rgba(99, 102, 241, 0.2);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.btn-secondary:hover::before {
		opacity: 1;
	}

	.btn-secondary:hover {
		border-color: rgba(99, 102, 241, 0.5);
		transform: translateY(-1px);
	}

	.btn-primary {
		background: linear-gradient(135deg, #6366f1, #8b5cf6);
		color: white;
	}

	.btn-primary::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #8b5cf6, #a855f7);
		opacity: 0;
		transition: opacity 0.3s;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow:
			0 12px 24px rgba(99, 102, 241, 0.4),
			0 0 20px rgba(139, 92, 246, 0.3);
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

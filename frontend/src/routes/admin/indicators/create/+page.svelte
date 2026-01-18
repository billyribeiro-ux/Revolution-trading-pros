<!--
	URL: /admin/indicators/create
	Apple Principal Engineer ICT Level 7 - January 2026
	
	Creates indicator product pages for member download portal.
	Members who purchase indicators can download platform-specific files.
-->

<script lang="ts">
	import { goto } from '$app/navigation';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ═══════════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════════

	interface Platform {
		id: string;
		name: string;
		displayName: string;
		icon: string;
		extension: string;
	}

	interface UploadedFile {
		name: string;
		url: string;
		size: number;
		type: string;
	}

	interface IndicatorForm {
		name: string;
		slug: string;
		subtitle: string;
		short_description: string;
		description: string;
		description_html: string;
		price_cents: number;
		is_free: boolean;
		thumbnail_url: string;
		preview_image_url: string;
		platforms: string[];
		version: string;
		version_notes: string;
		category: string;
		tags: string[];
		is_published: boolean;
		is_featured: boolean;
		has_tradingview_access: boolean;
		tradingview_invite_only: boolean;
	}

	interface PlatformFile {
		platform_id: string;
		file: UploadedFile | null;
		version: string;
		installation_notes: string;
	}

	interface DocumentationFile {
		title: string;
		file: UploadedFile | null;
		doc_type: string;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// AVAILABLE PLATFORMS
	// ═══════════════════════════════════════════════════════════════════════════

	const PLATFORMS: Platform[] = [
		{ id: 'thinkorswim', name: 'thinkorswim', displayName: 'thinkorswim', icon: '📊', extension: '.ts' },
		{ id: 'tradingview', name: 'tradingview', displayName: 'TradingView', icon: '📈', extension: '.pine' },
		{ id: 'metatrader4', name: 'metatrader4', displayName: 'MetaTrader 4', icon: '💹', extension: '.mq4' },
		{ id: 'metatrader5', name: 'metatrader5', displayName: 'MetaTrader 5', icon: '💹', extension: '.mq5' },
		{ id: 'ninjatrader', name: 'ninjatrader', displayName: 'NinjaTrader', icon: '🥷', extension: '.cs' },
		{ id: 'tradestation', name: 'tradestation', displayName: 'TradeStation', icon: '🚂', extension: '.eld' },
		{ id: 'multicharts', name: 'multicharts', displayName: 'MultiCharts', icon: '📉', extension: '.pla' },
		{ id: 'esignal', name: 'esignal', displayName: 'eSignal', icon: '📡', extension: '.efs' }
	];

	const CATEGORIES = [
		'Momentum',
		'Trend',
		'Volume',
		'Volatility',
		'Support/Resistance',
		'Pattern Recognition',
		'Multi-Timeframe',
		'Options',
		'Scalping',
		'Swing Trading'
	];

	const DOC_TYPES = [
		{ id: 'pdf', name: 'PDF Guide', icon: '📄' },
		{ id: 'video', name: 'Video Tutorial', icon: '🎬' },
		{ id: 'quickstart', name: 'Quick Start', icon: '🚀' },
		{ id: 'faq', name: 'FAQ', icon: '❓' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let indicator = $state<IndicatorForm>({
		name: '',
		slug: '',
		subtitle: '',
		short_description: '',
		description: '',
		description_html: '',
		price_cents: 0,
		is_free: false,
		thumbnail_url: '',
		preview_image_url: '',
		platforms: [],
		version: '1.0.0',
		version_notes: '',
		category: '',
		tags: [],
		is_published: false,
		is_featured: false,
		has_tradingview_access: false,
		tradingview_invite_only: true
	});

	let platformFiles = $state<PlatformFile[]>([]);
	let documentationFiles = $state<DocumentationFile[]>([]);
	let tagInput = $state('');

	// Upload states
	let thumbnailUploading = $state(false);
	let thumbnailError = $state('');
	let thumbnailDragOver = $state(false);

	let saving = $state(false);
	let formError = $state('');
	let successMessage = $state('');

	// ═══════════════════════════════════════════════════════════════════════════
	// REACTIVE SLUG GENERATION - Instant as you type
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (indicator.name) {
			indicator.slug = indicator.name
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '')
				.substring(0, 100);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// PLATFORM SELECTION
	// ═══════════════════════════════════════════════════════════════════════════

	function togglePlatform(platformId: string) {
		const index = indicator.platforms.indexOf(platformId);
		if (index === -1) {
			indicator.platforms = [...indicator.platforms, platformId];
			// Add platform file entry
			platformFiles = [...platformFiles, {
				platform_id: platformId,
				file: null,
				version: indicator.version,
				installation_notes: ''
			}];
		} else {
			indicator.platforms = indicator.platforms.filter(p => p !== platformId);
			platformFiles = platformFiles.filter(pf => pf.platform_id !== platformId);
		}
	}

	function getPlatformById(id: string): Platform | undefined {
		return PLATFORMS.find(p => p.id === id);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// TAG MANAGEMENT
	// ═══════════════════════════════════════════════════════════════════════════

	function addTag() {
		const tag = tagInput.trim().toLowerCase();
		if (tag && !indicator.tags.includes(tag)) {
			indicator.tags = [...indicator.tags, tag];
			tagInput = '';
		}
	}

	function removeTag(tag: string) {
		indicator.tags = indicator.tags.filter(t => t !== tag);
	}

	function handleTagKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ',') {
			e.preventDefault();
			addTag();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// FILE UPLOAD - Bunny Storage Integration
	// ═══════════════════════════════════════════════════════════════════════════

	async function uploadToBunny(file: File, folder: string): Promise<UploadedFile> {
		const formData = new FormData();
		formData.append('file', file);
		formData.append('folder', folder);

		try {
			// Try the media upload endpoint first
			const response = await adminFetch('/api/admin/media/upload', {
				method: 'POST',
				body: formData
			});

			if (response.success && response.data && response.data.length > 0) {
				return {
					name: file.name,
					url: response.data[0].url,
					size: file.size,
					type: file.type
				};
			}
			throw new Error(response.message || 'Upload failed');
		} catch (error: any) {
			// Fallback: Create local blob URL for preview (will need proper upload later)
			console.warn('Server upload failed, using local preview:', error.message);
			return {
				name: file.name,
				url: URL.createObjectURL(file),
				size: file.size,
				type: file.type
			};
		}
	}

	async function resizeImage(file: File, maxWidth = 1200, maxHeight = 1200, quality = 0.85): Promise<Blob> {
		return new Promise((resolve, reject) => {
			const img = new Image();
			const canvas = document.createElement('canvas');
			const ctx = canvas.getContext('2d');

			img.onload = () => {
				let { width, height } = img;
				if (width > maxWidth || height > maxHeight) {
					const ratio = Math.min(maxWidth / width, maxHeight / height);
					width = Math.round(width * ratio);
					height = Math.round(height * ratio);
				}
				canvas.width = width;
				canvas.height = height;

				if (ctx) {
					ctx.imageSmoothingEnabled = true;
					ctx.imageSmoothingQuality = 'high';
					ctx.drawImage(img, 0, 0, width, height);
					canvas.toBlob(
						(blob) => blob ? resolve(blob) : reject(new Error('Failed to create blob')),
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

	// ═══════════════════════════════════════════════════════════════════════════
	// THUMBNAIL DRAG & DROP
	// ═══════════════════════════════════════════════════════════════════════════

	function handleThumbnailDragOver(e: DragEvent) {
		e.preventDefault();
		thumbnailDragOver = true;
	}

	function handleThumbnailDragLeave(e: DragEvent) {
		e.preventDefault();
		thumbnailDragOver = false;
	}

	async function handleThumbnailDrop(e: DragEvent) {
		e.preventDefault();
		thumbnailDragOver = false;
		
		const file = e.dataTransfer?.files[0];
		if (file && file.type.startsWith('image/')) {
			await uploadThumbnail(file);
		} else {
			thumbnailError = 'Please drop an image file';
		}
	}

	async function handleThumbnailSelect(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			await uploadThumbnail(file);
		}
	}

	async function uploadThumbnail(file: File) {
		thumbnailError = '';
		thumbnailUploading = true;

		try {
			if (!file.type.startsWith('image/')) {
				throw new Error('Please select an image file');
			}
			if (file.size > 50 * 1024 * 1024) {
				throw new Error('Image must be less than 50MB');
			}

			const resizedBlob = await resizeImage(file, 1200, 1200, 0.85);
			const resizedFile = new File([resizedBlob], file.name.replace(/\.[^.]+$/, '.jpg'), {
				type: 'image/jpeg'
			});

			const uploaded = await uploadToBunny(resizedFile, 'indicators/thumbnails');
			indicator.thumbnail_url = uploaded.url;
		} catch (error: any) {
			thumbnailError = error.message || 'Upload failed';
		} finally {
			thumbnailUploading = false;
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// PLATFORM FILE UPLOAD
	// ═══════════════════════════════════════════════════════════════════════════

	let platformFileUploading = $state<Record<string, boolean>>({});
	let platformFileErrors = $state<Record<string, string>>({});
	let platformFileDragOver = $state<Record<string, boolean>>({});

	function handlePlatformFileDragOver(e: DragEvent, platformId: string) {
		e.preventDefault();
		platformFileDragOver = { ...platformFileDragOver, [platformId]: true };
	}

	function handlePlatformFileDragLeave(e: DragEvent, platformId: string) {
		e.preventDefault();
		platformFileDragOver = { ...platformFileDragOver, [platformId]: false };
	}

	async function handlePlatformFileDrop(e: DragEvent, platformId: string) {
		e.preventDefault();
		platformFileDragOver = { ...platformFileDragOver, [platformId]: false };
		
		const file = e.dataTransfer?.files[0];
		if (file) {
			await uploadPlatformFile(file, platformId);
		}
	}

	async function handlePlatformFileSelect(e: Event, platformId: string) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			await uploadPlatformFile(file, platformId);
		}
	}

	async function uploadPlatformFile(file: File, platformId: string) {
		platformFileErrors = { ...platformFileErrors, [platformId]: '' };
		platformFileUploading = { ...platformFileUploading, [platformId]: true };

		try {
			const uploaded = await uploadToBunny(file, `indicators/files/${platformId}`);
			
			platformFiles = platformFiles.map(pf => {
				if (pf.platform_id === platformId) {
					return { ...pf, file: uploaded };
				}
				return pf;
			});
		} catch (error: any) {
			platformFileErrors = { ...platformFileErrors, [platformId]: error.message || 'Upload failed' };
		} finally {
			platformFileUploading = { ...platformFileUploading, [platformId]: false };
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// DOCUMENTATION UPLOAD
	// ═══════════════════════════════════════════════════════════════════════════

	let docUploading = $state<Record<number, boolean>>({});
	let docErrors = $state<Record<number, string>>({});
	let docDragOver = $state<Record<number, boolean>>({});

	function addDocumentation() {
		documentationFiles = [...documentationFiles, {
			title: '',
			file: null,
			doc_type: 'pdf'
		}];
	}

	function removeDocumentation(index: number) {
		documentationFiles = documentationFiles.filter((_, i) => i !== index);
	}

	function handleDocDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		docDragOver = { ...docDragOver, [index]: true };
	}

	function handleDocDragLeave(e: DragEvent, index: number) {
		e.preventDefault();
		docDragOver = { ...docDragOver, [index]: false };
	}

	async function handleDocDrop(e: DragEvent, index: number) {
		e.preventDefault();
		docDragOver = { ...docDragOver, [index]: false };
		
		const file = e.dataTransfer?.files[0];
		if (file) {
			await uploadDocFile(file, index);
		}
	}

	async function handleDocSelect(e: Event, index: number) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			await uploadDocFile(file, index);
		}
	}

	async function uploadDocFile(file: File, index: number) {
		docErrors = { ...docErrors, [index]: '' };
		docUploading = { ...docUploading, [index]: true };

		try {
			const uploaded = await uploadToBunny(file, 'indicators/docs');
			
			documentationFiles = documentationFiles.map((doc, i) => {
				if (i === index) {
					return { ...doc, file: uploaded, title: doc.title || file.name.replace(/\.[^.]+$/, '') };
				}
				return doc;
			});
		} catch (error: any) {
			docErrors = { ...docErrors, [index]: error.message || 'Upload failed' };
		} finally {
			docUploading = { ...docUploading, [index]: false };
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// FORM SUBMISSION
	// ═══════════════════════════════════════════════════════════════════════════

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
	}

	async function saveIndicator() {
		formError = '';
		successMessage = '';

		if (!indicator.name.trim()) {
			formError = 'Indicator name is required';
			return;
		}

		if (indicator.platforms.length === 0) {
			formError = 'Please select at least one platform';
			return;
		}

		saving = true;
		try {
			const payload = {
				name: indicator.name,
				slug: indicator.slug || undefined,
				subtitle: indicator.subtitle || undefined,
				short_description: indicator.short_description || undefined,
				description: indicator.description || undefined,
				description_html: indicator.description_html || undefined,
				thumbnail_url: indicator.thumbnail_url || undefined,
				preview_image_url: indicator.preview_image_url || undefined,
				category: indicator.category || undefined,
				tags: indicator.tags.length > 0 ? indicator.tags : undefined,
				version: indicator.version || '1.0.0',
				version_notes: indicator.version_notes || undefined,
				is_published: indicator.is_published,
				is_featured: indicator.is_featured,
				is_free: indicator.is_free,
				price_cents: indicator.is_free ? 0 : indicator.price_cents,
				has_tradingview_access: indicator.has_tradingview_access,
				tradingview_invite_only: indicator.tradingview_invite_only
			};

			const response = await adminFetch('/api/admin/indicators', {
				method: 'POST',
				body: JSON.stringify(payload)
			});

			if (response.success || response.indicator) {
				const indicatorId = response.indicator?.id;

				// Upload platform files if indicator was created
				if (indicatorId) {
					for (const pf of platformFiles) {
						if (pf.file) {
							await adminFetch(`/api/admin/indicators/${indicatorId}/files`, {
								method: 'POST',
								body: JSON.stringify({
									platform_id: PLATFORMS.findIndex(p => p.id === pf.platform_id) + 1,
									file_url: pf.file.url,
									file_name: pf.file.name,
									file_size_bytes: pf.file.size,
									version: pf.version || indicator.version,
									installation_notes: pf.installation_notes || undefined,
									is_latest: true
								})
							}).catch(e => console.warn('Failed to save platform file:', e));
						}
					}

					// Upload documentation
					for (const doc of documentationFiles) {
						if (doc.file && doc.title) {
							await adminFetch(`/api/admin/indicators/${indicatorId}/documentation`, {
								method: 'POST',
								body: JSON.stringify({
									title: doc.title,
									doc_type: doc.doc_type,
									file_url: doc.file.url,
									file_name: doc.file.name,
									is_published: true
								})
							}).catch(e => console.warn('Failed to save documentation:', e));
						}
					}
				}

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
	<title>Create Indicator | Admin | Revolution Trading Pros</title>
</svelte:head>

<div class="admin-indicators-create">
	<!-- Animated Background -->
	<div class="bg-effects">
		<div class="bg-blob bg-blob-1"></div>
		<div class="bg-blob bg-blob-2"></div>
		<div class="bg-blob bg-blob-3"></div>
	</div>

	<div class="admin-page-container">
		<!-- Header -->
		<header class="page-header">
			<div class="header-icon">✨</div>
			<h1>Create New Indicator</h1>
			<p class="subtitle">Build a downloadable indicator product for your members</p>
		</header>

		<!-- Main Content Grid -->
		<div class="content-grid">
			<!-- Left Column: Form -->
			<div class="form-column">
				<!-- Basic Information Card -->
				<section class="form-card">
					<h2 class="card-title">
						<span class="title-icon">📝</span>
						Basic Information
					</h2>

					<div class="form-group">
						<label for="name">Indicator Name <span class="required">*</span></label>
						<input
							id="name"
							type="text"
							bind:value={indicator.name}
							placeholder="e.g., Volume Max Indicator"
							class="input-lg"
						/>
						{#if indicator.slug}
							<div class="slug-preview">
								<span class="slug-label">URL:</span>
								<span class="slug-value">/indicators/{indicator.slug}</span>
							</div>
						{/if}
					</div>

					<div class="form-group">
						<label for="subtitle">Subtitle / Tagline</label>
						<input
							id="subtitle"
							type="text"
							bind:value={indicator.subtitle}
							placeholder="e.g., Advanced volume analysis for professional traders"
						/>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="category">Category</label>
							<select id="category" bind:value={indicator.category}>
								<option value="">Select Category</option>
								{#each CATEGORIES as cat}
									<option value={cat}>{cat}</option>
								{/each}
							</select>
						</div>

						<div class="form-group">
							<label for="version">Version</label>
							<input
								id="version"
								type="text"
								bind:value={indicator.version}
								placeholder="1.0.0"
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="short_description">Short Description</label>
						<textarea
							id="short_description"
							bind:value={indicator.short_description}
							placeholder="Brief summary for listings and cards..."
							rows="2"
						></textarea>
					</div>

					<div class="form-group">
						<label for="description">Full Description</label>
						<textarea
							id="description"
							bind:value={indicator.description}
							placeholder="Detailed description with features, benefits, and usage..."
							rows="6"
						></textarea>
					</div>

					<!-- Tags -->
					<div class="form-group">
						<label>Tags</label>
						<div class="tags-container">
							{#each indicator.tags as tag}
								<span class="tag">
									{tag}
									<button type="button" class="tag-remove" onclick={() => removeTag(tag)}>×</button>
								</span>
							{/each}
							<input
								type="text"
								class="tag-input"
								bind:value={tagInput}
								onkeydown={handleTagKeydown}
								placeholder="Add tag..."
							/>
						</div>
						<div class="form-hint">Press Enter or comma to add</div>
					</div>
				</section>

				<!-- Platforms Card -->
				<section class="form-card">
					<h2 class="card-title">
						<span class="title-icon">🖥️</span>
						Supported Platforms
						<span class="required">*</span>
					</h2>
					<p class="card-description">Select all platforms this indicator supports. You can upload platform-specific files below.</p>

					<div class="platforms-grid">
						{#each PLATFORMS as platform}
							<button
								type="button"
								class="platform-chip"
								class:selected={indicator.platforms.includes(platform.id)}
								onclick={() => togglePlatform(platform.id)}
							>
								<span class="platform-icon">{platform.icon}</span>
								<span class="platform-name">{platform.displayName}</span>
								{#if indicator.platforms.includes(platform.id)}
									<span class="check-icon">✓</span>
								{/if}
							</button>
						{/each}
					</div>

					{#if indicator.platforms.length > 0}
						<div class="selected-count">
							{indicator.platforms.length} platform{indicator.platforms.length !== 1 ? 's' : ''} selected
						</div>
					{/if}
				</section>

				<!-- Platform Files Card -->
				{#if platformFiles.length > 0}
					<section class="form-card">
						<h2 class="card-title">
							<span class="title-icon">📦</span>
							Platform Files
						</h2>
						<p class="card-description">Upload indicator files for each selected platform. Drag and drop or click to upload.</p>

						<div class="platform-files-list">
							{#each platformFiles as pf, index}
								{@const platform = getPlatformById(pf.platform_id)}
								{#if platform}
									<div class="platform-file-item">
										<div class="platform-file-header">
											<span class="platform-icon">{platform.icon}</span>
											<span class="platform-name">{platform.displayName}</span>
											<span class="file-ext">{platform.extension}</span>
										</div>

										<!-- Drop Zone -->
										<div
											class="drop-zone"
											class:drag-over={platformFileDragOver[pf.platform_id]}
											class:has-file={pf.file}
											class:uploading={platformFileUploading[pf.platform_id]}
											ondragover={(e) => handlePlatformFileDragOver(e, pf.platform_id)}
											ondragleave={(e) => handlePlatformFileDragLeave(e, pf.platform_id)}
											ondrop={(e) => handlePlatformFileDrop(e, pf.platform_id)}
											role="button"
											tabindex="0"
										>
											{#if platformFileUploading[pf.platform_id]}
												<div class="upload-spinner"></div>
												<span>Uploading...</span>
											{:else if pf.file}
												<div class="file-info">
													<span class="file-icon">📄</span>
													<div class="file-details">
														<span class="file-name">{pf.file.name}</span>
														<span class="file-size">{formatFileSize(pf.file.size)}</span>
													</div>
													<button
														type="button"
														class="file-remove"
														onclick={() => {
															platformFiles = platformFiles.map((f, i) => 
																i === index ? { ...f, file: null } : f
															);
														}}
													>×</button>
												</div>
											{:else}
												<div class="drop-content">
													<span class="drop-icon">📤</span>
													<span class="drop-text">Drop file here or click to browse</span>
												</div>
											{/if}
											<input
												type="file"
												class="file-input"
												onchange={(e) => handlePlatformFileSelect(e, pf.platform_id)}
											/>
										</div>

										{#if platformFileErrors[pf.platform_id]}
											<div class="file-error">{platformFileErrors[pf.platform_id]}</div>
										{/if}

										<!-- Installation Notes -->
										<div class="form-group compact">
											<input
												type="text"
												bind:value={pf.installation_notes}
												placeholder="Installation notes (optional)..."
											/>
										</div>
									</div>
								{/if}
							{/each}
						</div>
					</section>
				{/if}

				<!-- Documentation Card -->
				<section class="form-card">
					<h2 class="card-title">
						<span class="title-icon">📚</span>
						Documentation
					</h2>
					<p class="card-description">Upload PDF guides, quick start docs, or link to video tutorials.</p>

					{#if documentationFiles.length > 0}
						<div class="documentation-list">
							{#each documentationFiles as doc, index}
								<div class="doc-item">
									<div class="doc-header">
										<select bind:value={doc.doc_type} class="doc-type-select">
											{#each DOC_TYPES as dt}
												<option value={dt.id}>{dt.icon} {dt.name}</option>
											{/each}
										</select>
										<button
											type="button"
											class="doc-remove"
											onclick={() => removeDocumentation(index)}
										>×</button>
									</div>

									<input
										type="text"
										bind:value={doc.title}
										placeholder="Document title..."
										class="doc-title"
									/>

									<!-- Drop Zone -->
									<div
										class="drop-zone compact"
										class:drag-over={docDragOver[index]}
										class:has-file={doc.file}
										class:uploading={docUploading[index]}
										ondragover={(e) => handleDocDragOver(e, index)}
										ondragleave={(e) => handleDocDragLeave(e, index)}
										ondrop={(e) => handleDocDrop(e, index)}
										role="button"
										tabindex="0"
									>
										{#if docUploading[index]}
											<div class="upload-spinner small"></div>
											<span>Uploading...</span>
										{:else if doc.file}
											<div class="file-info compact">
												<span class="file-name">{doc.file.name}</span>
												<span class="file-size">{formatFileSize(doc.file.size)}</span>
											</div>
										{:else}
											<span class="drop-text">Drop file or click</span>
										{/if}
										<input
											type="file"
											class="file-input"
											accept=".pdf,.doc,.docx,.txt,.md"
											onchange={(e) => handleDocSelect(e, index)}
										/>
									</div>

									{#if docErrors[index]}
										<div class="file-error">{docErrors[index]}</div>
									{/if}
								</div>
							{/each}
						</div>
					{/if}

					<button type="button" class="btn-add-doc" onclick={addDocumentation}>
						<span>+</span> Add Documentation
					</button>
				</section>

				<!-- Pricing Card -->
				<section class="form-card">
					<h2 class="card-title">
						<span class="title-icon">💰</span>
						Pricing
					</h2>

					<label class="toggle-row">
						<input type="checkbox" bind:checked={indicator.is_free} />
						<span class="toggle-label">Free Indicator</span>
					</label>

					{#if !indicator.is_free}
						<div class="form-group">
							<label for="price">Price (USD)</label>
							<div class="price-input">
								<span class="currency">$</span>
								<input
									id="price"
									type="number"
									bind:value={indicator.price_cents}
									placeholder="9900"
									step="100"
									min="0"
								/>
								<span class="price-hint">in cents (9900 = $99.00)</span>
							</div>
						</div>
					{/if}
				</section>

				<!-- TradingView Access -->
				{#if indicator.platforms.includes('tradingview')}
					<section class="form-card">
						<h2 class="card-title">
							<span class="title-icon">📈</span>
							TradingView Access
						</h2>

						<label class="toggle-row">
							<input type="checkbox" bind:checked={indicator.has_tradingview_access} />
							<span class="toggle-label">Enable TradingView Access Management</span>
						</label>

						{#if indicator.has_tradingview_access}
							<label class="toggle-row">
								<input type="checkbox" bind:checked={indicator.tradingview_invite_only} />
								<span class="toggle-label">Invite-only (manual approval required)</span>
							</label>
						{/if}
					</section>
				{/if}

				<!-- Publishing Options -->
				<section class="form-card">
					<h2 class="card-title">
						<span class="title-icon">🚀</span>
						Publishing
					</h2>

					<div class="toggle-group">
						<label class="toggle-row">
							<input type="checkbox" bind:checked={indicator.is_published} />
							<span class="toggle-label">Publish immediately</span>
						</label>

						<label class="toggle-row">
							<input type="checkbox" bind:checked={indicator.is_featured} />
							<span class="toggle-label">Featured indicator</span>
						</label>
					</div>

					<div class="form-group">
						<label for="version_notes">Version Notes</label>
						<textarea
							id="version_notes"
							bind:value={indicator.version_notes}
							placeholder="What's new in this version..."
							rows="3"
						></textarea>
					</div>
				</section>
			</div>

			<!-- Right Column: Preview & Thumbnail -->
			<div class="preview-column">
				<!-- Thumbnail Upload -->
				<section class="form-card sticky">
					<h2 class="card-title">
						<span class="title-icon">🖼️</span>
						Thumbnail
					</h2>

					<div
						class="thumbnail-drop-zone"
						class:drag-over={thumbnailDragOver}
						class:has-image={indicator.thumbnail_url}
						class:uploading={thumbnailUploading}
						ondragover={handleThumbnailDragOver}
						ondragleave={handleThumbnailDragLeave}
						ondrop={handleThumbnailDrop}
						role="button"
						tabindex="0"
					>
						{#if thumbnailUploading}
							<div class="upload-spinner large"></div>
							<span class="upload-text">Uploading & Optimizing...</span>
						{:else if indicator.thumbnail_url}
							<img src={indicator.thumbnail_url} alt="Thumbnail preview" class="thumbnail-preview" />
							<button
								type="button"
								class="thumbnail-remove"
								onclick={() => indicator.thumbnail_url = ''}
							>×</button>
						{:else}
							<div class="drop-placeholder">
								<span class="drop-icon-large">📷</span>
								<span class="drop-title">Drop image here</span>
								<span class="drop-subtitle">or click to browse</span>
							</div>
						{/if}
						<input
							type="file"
							class="file-input"
							accept="image/*"
							onchange={handleThumbnailSelect}
						/>
					</div>

					{#if thumbnailError}
						<div class="file-error">{thumbnailError}</div>
					{/if}

					<div class="form-hint center">Auto-resized to 1200px max, JPEG optimized</div>
				</section>

				<!-- Live Preview Card -->
				<section class="form-card sticky-preview">
					<h2 class="card-title">
						<span class="title-icon">👁️</span>
						Live Preview
					</h2>

					<div class="preview-card">
						<div class="preview-image">
							{#if indicator.thumbnail_url}
								<img src={indicator.thumbnail_url} alt="Preview" />
							{:else}
								<div class="preview-placeholder">
									<span>📊</span>
								</div>
							{/if}
							{#if indicator.is_featured}
								<span class="featured-badge">⭐ Featured</span>
							{/if}
						</div>
						<div class="preview-content">
							<h3 class="preview-title">{indicator.name || 'Indicator Name'}</h3>
							{#if indicator.subtitle}
								<p class="preview-subtitle">{indicator.subtitle}</p>
							{/if}
							<p class="preview-description">
								{indicator.short_description || 'Add a short description...'}
							</p>
							<div class="preview-meta">
								{#if indicator.category}
									<span class="meta-tag">{indicator.category}</span>
								{/if}
								{#each indicator.platforms.slice(0, 3) as pid}
									{@const p = getPlatformById(pid)}
									{#if p}
										<span class="meta-platform">{p.icon}</span>
									{/if}
								{/each}
								{#if indicator.platforms.length > 3}
									<span class="meta-more">+{indicator.platforms.length - 3}</span>
								{/if}
							</div>
							<div class="preview-footer">
								<span class="preview-price">
									{indicator.is_free ? 'FREE' : `$${(indicator.price_cents / 100).toFixed(2)}`}
								</span>
								<span class="preview-version">v{indicator.version}</span>
							</div>
						</div>
					</div>
				</section>
			</div>
		</div>

		<!-- Error / Success Messages -->
		{#if formError}
			<div class="form-message error">
				<span class="message-icon">⚠️</span>
				{formError}
			</div>
		{/if}

		{#if successMessage}
			<div class="form-message success">
				<span class="message-icon">✅</span>
				{successMessage}
			</div>
		{/if}

		<!-- Form Actions -->
		<div class="form-actions">
			<button type="button" class="btn-secondary" onclick={() => goto('/admin/indicators')}>
				Cancel
			</button>
			<button
				type="button"
				class="btn-primary"
				onclick={saveIndicator}
				disabled={saving}
			>
				{#if saving}
					<span class="btn-spinner"></span>
					Creating...
				{:else}
					Create Indicator
				{/if}
			</button>
		</div>
	</div>
</div>

<style>
	/* Page wrapper - Email Templates Style */
	.admin-indicators-create {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Header CENTERED */
	.page-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.page-header h1 {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		font-size: 1.75rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
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
		background: linear-gradient(135deg, #e6b800, #b38f00, #ffd11a);
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
		background: linear-gradient(135deg, #e6b800, #b38f00, #ffd11a);
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
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
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
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		color: #0d1117;
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
		background: linear-gradient(135deg, #b38f00, #ffd11a);
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
		border-color: #e6b800;
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
		color: #e6b800;
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
		color: #ffd11a;
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
		color: #ffd11a;
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
		background: linear-gradient(135deg, #e6b800 0%, #b38f00 100%);
		color: #0d1117;
	}

	.btn-primary::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(135deg, #b38f00, #ffd11a);
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

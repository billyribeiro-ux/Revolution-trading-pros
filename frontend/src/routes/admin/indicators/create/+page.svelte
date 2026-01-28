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
		{ id: 'thinkorswim', name: 'thinkorswim', displayName: 'Thinkorswim', icon: '', extension: '.ts' },
		{ id: 'tradingview', name: 'tradingview', displayName: 'TradingView', icon: '', extension: '.pine' },
		{ id: 'trendspider', name: 'trendspider', displayName: 'TrendSpider', icon: '', extension: '.tsp' }
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
		{ id: 'pdf', name: 'PDF Guide', icon: 'pdf' },
		{ id: 'video', name: 'Video Tutorial', icon: 'video' },
		{ id: 'quickstart', name: 'Quick Start', icon: 'quickstart' },
		{ id: 'faq', name: 'FAQ', icon: 'faq' }
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
			<div class="header-icon">
				<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v18h18"/><path d="m19 9-5 5-4-4-3 3"/></svg>
			</div>
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
						<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><line x1="16" x2="8" y1="13" y2="13"/><line x1="16" x2="8" y1="17" y2="17"/><line x1="10" x2="8" y1="9" y2="9"/></svg></span>
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
						<label for="tags-input">Tags</label>
						<div class="tags-container">
							{#each indicator.tags as tag}
								<span class="tag">
									{tag}
									<button type="button" class="tag-remove" onclick={() => removeTag(tag)}>×</button>
								</span>
							{/each}
							<input
								id="tags-input"
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
					<p class="card-description">Select all platforms this indicator supports.</p>

					<div class="platforms-toggle-list">
						{#each PLATFORMS as platform}
							<label class="platform-toggle">
								<input
									type="checkbox"
									checked={indicator.platforms.includes(platform.id)}
									onchange={() => togglePlatform(platform.id)}
								/>
								<span class="toggle-slider"></span>
								<span class="platform-name">{platform.displayName}</span>
							</label>
						{/each}
					</div>
				</section>

				<!-- Platform Files Card -->
				{#if platformFiles.length > 0}
					<section class="form-card">
						<h2 class="card-title">
							<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/></svg></span>
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
													<span class="file-icon"><svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/></svg></span>
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
													<span class="drop-icon"><svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/></svg></span>
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
						<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1 0-5H20"/></svg></span>
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
												<option value={dt.id}>{dt.name}</option>
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
						<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="12" x2="12" y1="2" y2="22"/><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"/></svg></span>
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
							<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg></span>
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
						<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg></span>
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
						<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/></svg></span>
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
								<span class="drop-icon-large"><svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/></svg></span>
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
						<span class="title-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg></span>
						Live Preview
					</h2>

					<div class="preview-card">
						<div class="preview-image">
							{#if indicator.thumbnail_url}
								<img src={indicator.thumbnail_url} alt="Preview" />
							{:else}
								<div class="preview-placeholder">
									<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M3 3v18h18"/><path d="m19 9-5 5-4-4-3 3"/></svg>
								</div>
							{/if}
							{#if indicator.is_featured}
								<span class="featured-badge"><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg> Featured</span>
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
				<span class="message-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg></span>
				{formError}
			</div>
		{/if}

		{#if successMessage}
			<div class="form-message success">
				<span class="message-icon"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg></span>
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
	/* ═══════════════════════════════════════════════════════════════════════════
	   ICT Level 7 Apple Principal Engineer Styles
	   Create Indicator Page - Revolution Trading Pros
	   ═══════════════════════════════════════════════════════════════════════════ */

	.admin-indicators-create {
		min-height: 100vh;
		position: relative;
	}

	.admin-page-container {
		max-width: 1400px;
		margin: 0 auto;
		padding: 2rem;
	}

	/* Background Effects */
	.bg-effects {
		position: fixed;
		inset: 0;
		pointer-events: none;
		overflow: hidden;
		z-index: -1;
	}

	.bg-blob {
		position: absolute;
		border-radius: 50%;
		filter: blur(100px);
		opacity: 0.15;
		animation: float 20s ease-in-out infinite;
	}

	.bg-blob-1 {
		width: 600px;
		height: 600px;
		background: var(--primary-500);
		top: -200px;
		right: -200px;
	}

	.bg-blob-2 {
		width: 400px;
		height: 400px;
		background: #143E59;
		bottom: -100px;
		left: -100px;
		animation-delay: -7s;
	}

	.bg-blob-3 {
		width: 300px;
		height: 300px;
		background: var(--primary-500);
		top: 50%;
		left: 50%;
		animation-delay: -14s;
	}

	@keyframes float {
		0%, 100% { transform: translate(0, 0) scale(1); }
		33% { transform: translate(30px, -30px) scale(1.05); }
		66% { transform: translate(-20px, 20px) scale(0.95); }
	}

	/* Header */
	.page-header {
		text-align: center;
		margin-bottom: 2.5rem;
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		margin: 0 auto 1rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		border-radius: 20px;
		color: #fff;
		box-shadow: 0 8px 32px rgba(230, 184, 0, 0.3);
	}

	.header-icon svg {
		width: 48px;
		height: 48px;
	}

	.page-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 1rem;
		margin: 0;
	}

	/* Content Grid */
	.content-grid {
		display: grid;
		grid-template-columns: 1fr 380px;
		gap: 2rem;
		align-items: start;
	}

	.form-column {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.preview-column {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* Form Cards */
	.form-card {
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.95) 0%, rgba(15, 23, 42, 0.95) 100%);
		backdrop-filter: blur(20px);
		border-radius: 16px;
		padding: 1.5rem;
		border: 1px solid rgba(230, 184, 0, 0.15);
		box-shadow: 0 4px 24px rgba(0, 0, 0, 0.2);
	}

	.form-card.sticky {
		position: sticky;
		top: 1rem;
	}

	.form-card.sticky-preview {
		position: sticky;
		top: 320px;
	}

	.card-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.125rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.5rem 0;
	}

	.title-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 8px;
		color: var(--primary-500);
	}

	.title-icon svg {
		width: 18px;
		height: 18px;
	}

	.card-description {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0 0 1rem 0;
		line-height: 1.5;
	}

	.required {
		color: #ef4444;
	}

	/* Form Groups */
	.form-group {
		margin-bottom: 1.25rem;
	}

	.form-group:last-child {
		margin-bottom: 0;
	}

	.form-group.compact {
		margin-bottom: 0.75rem;
	}

	.form-group label {
		display: block;
		font-weight: 500;
		color: #e2e8f0;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-row .form-group {
		margin-bottom: 0;
	}

	/* Inputs */
	input[type='text'],
	input[type='number'],
	select,
	textarea {
		width: 100%;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 10px;
		color: #f1f5f9;
		font-size: 0.9375rem;
		font-family: inherit;
		transition: all 0.2s ease;
	}

	input.input-lg {
		padding: 1rem 1.25rem;
		font-size: 1.125rem;
		font-weight: 500;
	}

	input::placeholder,
	textarea::placeholder {
		color: #64748b;
	}

	input:focus,
	select:focus,
	textarea:focus {
		outline: none;
		border-color: var(--primary-500);
		box-shadow: 0 0 0 3px rgba(230, 184, 0, 0.15);
	}

	select {
		cursor: pointer;
		appearance: none;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='20' height='20' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: right 0.75rem center;
		padding-right: 2.5rem;
	}

	select option {
		background: #1e293b;
		color: #f1f5f9;
	}

	textarea {
		resize: vertical;
		min-height: 80px;
	}

	.form-hint {
		color: #64748b;
		font-size: 0.75rem;
		margin-top: 0.375rem;
	}

	.form-hint.center {
		text-align: center;
	}

	/* Slug Preview */
	.slug-preview {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(230, 184, 0, 0.1);
		border-radius: 6px;
		font-size: 0.8125rem;
	}

	.slug-label {
		color: #64748b;
	}

	.slug-value {
		color: var(--primary-500);
		font-family: monospace;
	}

	/* Tags */
	.tags-container {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.5rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 10px;
		min-height: 44px;
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.625rem;
		background: rgba(230, 184, 0, 0.2);
		border: 1px solid rgba(230, 184, 0, 0.4);
		border-radius: 20px;
		color: var(--primary-400);
		font-size: 0.8125rem;
	}

	.tag-remove {
		background: none;
		border: none;
		color: var(--primary-400);
		cursor: pointer;
		font-size: 1rem;
		line-height: 1;
		padding: 0;
		opacity: 0.7;
	}

	.tag-remove:hover {
		opacity: 1;
	}

	.tag-input {
		flex: 1;
		min-width: 100px;
		background: none !important;
		border: none !important;
		padding: 0.25rem !important;
		box-shadow: none !important;
	}

	/* Platform Toggle List */
	.platforms-toggle-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.platform-toggle {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(100, 116, 139, 0.3);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.platform-toggle:hover {
		border-color: rgba(230, 184, 0, 0.4);
		background: rgba(230, 184, 0, 0.05);
	}

	.platform-toggle input[type="checkbox"] {
		display: none;
	}

	.toggle-slider {
		position: relative;
		width: 48px;
		height: 26px;
		background: rgba(100, 116, 139, 0.4);
		border-radius: 13px;
		transition: all 0.3s ease;
		flex-shrink: 0;
	}

	.toggle-slider::after {
		content: '';
		position: absolute;
		top: 3px;
		left: 3px;
		width: 20px;
		height: 20px;
		background: #94a3b8;
		border-radius: 50%;
		transition: all 0.3s ease;
	}

	.platform-toggle input:checked + .toggle-slider {
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
	}

	.platform-toggle input:checked + .toggle-slider::after {
		left: 25px;
		background: #fff;
	}

	.platform-name {
		flex: 1;
		font-size: 1rem;
		font-weight: 500;
		color: #f1f5f9;
	}

	/* Platform Files */
	.platform-files-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.platform-file-item {
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 12px;
		border: 1px solid rgba(100, 116, 139, 0.2);
	}

	.platform-file-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.75rem;
		font-weight: 500;
		color: #e2e8f0;
	}

	.file-ext {
		margin-left: auto;
		padding: 0.125rem 0.5rem;
		background: rgba(100, 116, 139, 0.3);
		border-radius: 4px;
		font-size: 0.75rem;
		color: #94a3b8;
		font-family: monospace;
	}

	/* Drop Zones */
	.drop-zone {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 80px;
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border: 2px dashed rgba(100, 116, 139, 0.4);
		border-radius: 10px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.drop-zone.compact {
		min-height: 48px;
		padding: 0.75rem;
	}

	.drop-zone:hover,
	.drop-zone.drag-over {
		border-color: var(--primary-500);
		background: rgba(230, 184, 0, 0.05);
	}

	.drop-zone.has-file {
		border-style: solid;
		border-color: #22c55e;
		background: rgba(34, 197, 94, 0.05);
	}

	.drop-zone.uploading {
		border-color: #3b82f6;
		background: rgba(59, 130, 246, 0.05);
	}

	.file-input {
		position: absolute;
		inset: 0;
		opacity: 0;
		cursor: pointer;
	}

	.drop-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.375rem;
		color: #64748b;
	}

	.drop-icon {
		font-size: 1.5rem;
	}

	.drop-text {
		font-size: 0.8125rem;
	}

	.file-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
	}

	.file-info.compact {
		gap: 0.5rem;
	}

	.file-icon {
		font-size: 1.5rem;
	}

	.file-details {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.file-name {
		color: #e2e8f0;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.file-size {
		color: #64748b;
		font-size: 0.75rem;
	}

	.file-remove {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(239, 68, 68, 0.2);
		border: none;
		border-radius: 6px;
		color: #ef4444;
		cursor: pointer;
		font-size: 1rem;
	}

	.file-remove:hover {
		background: rgba(239, 68, 68, 0.3);
	}

	.file-error {
		margin-top: 0.5rem;
		padding: 0.5rem 0.75rem;
		background: rgba(239, 68, 68, 0.1);
		border-radius: 6px;
		color: #f87171;
		font-size: 0.8125rem;
	}

	/* Upload Spinner */
	.upload-spinner {
		width: 24px;
		height: 24px;
		border: 3px solid rgba(59, 130, 246, 0.3);
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.upload-spinner.small {
		width: 16px;
		height: 16px;
		border-width: 2px;
	}

	.upload-spinner.large {
		width: 40px;
		height: 40px;
		border-width: 4px;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Documentation */
	.documentation-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-bottom: 1rem;
	}

	.doc-item {
		padding: 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 10px;
		border: 1px solid rgba(100, 116, 139, 0.2);
	}

	.doc-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.75rem;
	}

	.doc-type-select {
		flex: 1;
		padding: 0.5rem 0.75rem;
		font-size: 0.8125rem;
	}

	.doc-remove {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(239, 68, 68, 0.2);
		border: none;
		border-radius: 6px;
		color: #ef4444;
		cursor: pointer;
		font-size: 1.125rem;
	}

	.doc-title {
		margin-bottom: 0.75rem;
	}

	.btn-add-doc {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.75rem;
		background: rgba(230, 184, 0, 0.1);
		border: 2px dashed rgba(230, 184, 0, 0.3);
		border-radius: 10px;
		color: var(--primary-500);
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-add-doc:hover {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.5);
	}

	/* Pricing */
	.price-input {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.price-input .currency {
		color: var(--primary-500);
		font-weight: 600;
		font-size: 1.25rem;
	}

	.price-input input {
		flex: 1;
	}

	.price-hint {
		color: #64748b;
		font-size: 0.75rem;
		white-space: nowrap;
	}

	/* Toggle Rows */
	.toggle-group {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		margin-bottom: 1rem;
	}

	.toggle-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: rgba(15, 23, 42, 0.4);
		border-radius: 10px;
		cursor: pointer;
		transition: background 0.2s;
	}

	.toggle-row:hover {
		background: rgba(15, 23, 42, 0.6);
	}

	.toggle-row input[type='checkbox'] {
		width: 18px;
		height: 18px;
		accent-color: var(--primary-500);
		cursor: pointer;
	}

	.toggle-label {
		color: #e2e8f0;
		font-size: 0.9375rem;
	}

	/* Thumbnail Drop Zone */
	.thumbnail-drop-zone {
		position: relative;
		aspect-ratio: 16/10;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		background: rgba(15, 23, 42, 0.4);
		border: 2px dashed rgba(100, 116, 139, 0.4);
		border-radius: 12px;
		cursor: pointer;
		overflow: hidden;
		transition: all 0.2s ease;
	}

	.thumbnail-drop-zone:hover,
	.thumbnail-drop-zone.drag-over {
		border-color: var(--primary-500);
		background: rgba(230, 184, 0, 0.05);
	}

	.thumbnail-drop-zone.has-image {
		border-style: solid;
		border-color: #22c55e;
	}

	.thumbnail-preview {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-remove {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(239, 68, 68, 0.9);
		border: none;
		border-radius: 8px;
		color: white;
		font-size: 1.25rem;
		cursor: pointer;
		z-index: 2;
	}

	.drop-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.375rem;
		color: #64748b;
	}

	.drop-icon-large {
		font-size: 2.5rem;
	}

	.drop-title {
		font-size: 0.9375rem;
		font-weight: 500;
		color: #94a3b8;
	}

	.drop-subtitle {
		font-size: 0.8125rem;
	}

	.upload-text {
		color: #3b82f6;
		font-size: 0.875rem;
	}

	/* Preview Card */
	.preview-card {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border-radius: 16px;
		overflow: hidden;
		border: 1px solid rgba(230, 184, 0, 0.2);
	}

	.preview-image {
		position: relative;
		aspect-ratio: 16/10;
		background: linear-gradient(135deg, #1e3a8a 0%, #1e40af 100%);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.preview-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.preview-placeholder {
		font-size: 3rem;
		opacity: 0.5;
	}

	.featured-badge {
		position: absolute;
		top: 0.75rem;
		left: 0.75rem;
		padding: 0.375rem 0.75rem;
		background: rgba(230, 184, 0, 0.9);
		border-radius: 20px;
		color: #0f172a;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.preview-content {
		padding: 1.25rem;
	}

	.preview-title {
		font-size: 1.125rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.25rem 0;
	}

	.preview-subtitle {
		color: #94a3b8;
		font-size: 0.8125rem;
		margin: 0 0 0.5rem 0;
	}

	.preview-description {
		color: #64748b;
		font-size: 0.875rem;
		line-height: 1.5;
		margin: 0 0 0.75rem 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.preview-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.meta-tag {
		padding: 0.25rem 0.625rem;
		background: rgba(100, 116, 139, 0.3);
		border-radius: 20px;
		color: #94a3b8;
		font-size: 0.75rem;
	}

	.meta-platform {
		font-size: 1rem;
	}

	.meta-more {
		padding: 0.125rem 0.375rem;
		background: rgba(230, 184, 0, 0.2);
		border-radius: 4px;
		color: var(--primary-500);
		font-size: 0.75rem;
	}

	.preview-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(100, 116, 139, 0.2);
	}

	.preview-price {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--primary-500);
	}

	.preview-version {
		color: #64748b;
		font-size: 0.75rem;
	}

	/* Form Messages */
	.form-message {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 12px;
		margin-top: 1.5rem;
		font-size: 0.9375rem;
	}

	.form-message.error {
		background: rgba(239, 68, 68, 0.15);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #fca5a5;
	}

	.form-message.success {
		background: rgba(34, 197, 94, 0.15);
		border: 1px solid rgba(34, 197, 94, 0.3);
		color: #86efac;
	}

	.message-icon {
		font-size: 1.25rem;
	}

	/* Form Actions */
	.form-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
		margin-top: 2rem;
		padding: 1.5rem 0;
	}

	.btn-secondary,
	.btn-primary {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.875rem 2rem;
		border-radius: 12px;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-secondary {
		background: rgba(100, 116, 139, 0.2);
		color: #94a3b8;
		border: 1px solid rgba(100, 116, 139, 0.3);
	}

	.btn-secondary:hover {
		background: rgba(100, 116, 139, 0.3);
		color: #e2e8f0;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--primary-500) 0%, var(--primary-600) 100%);
		color: #0f172a;
		min-width: 180px;
	}

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(230, 184, 0, 0.4);
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.btn-spinner {
		width: 18px;
		height: 18px;
		border: 2px solid rgba(15, 23, 42, 0.3);
		border-top-color: #0f172a;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.content-grid {
			grid-template-columns: 1fr;
		}

		.preview-column {
			order: -1;
		}

		.form-card.sticky,
		.form-card.sticky-preview {
			position: static;
		}
	}

	@media (max-width: 640px) {
		.admin-page-container {
			padding: 1rem;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-actions {
			flex-direction: column;
		}

		.btn-secondary,
		.btn-primary {
			width: 100%;
		}
	}
</style>

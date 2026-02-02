<script lang="ts">
	/**
	 * Indicator Builder - Revolution Trading Pros
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 *
	 * Complete indicator management with:
	 * - Platform-specific files (ThinkorSwim, TradingView, etc.)
	 * - Video tutorials
	 * - Documentation
	 * - TradingView Access Management
	 */

	import {
		indicatorsApi,
		platformsApi,
		indicatorVideosApi,
		platformFilesApi,
		documentationApi,
		tradingViewAccessApi,
		type Indicator,
		type Platform,
		type PlatformFile,
		type IndicatorVideo,
		type Documentation,
		type TradingViewAccess,
		type CreateIndicatorRequest,
		formatFileSize,
		validateTradingViewUsername
	} from '$lib/api/indicators-enhanced';

	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconTrash from '@tabler/icons-svelte/icons/trash';
	import IconEdit from '@tabler/icons-svelte/icons/edit';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconUpload from '@tabler/icons-svelte/icons/upload';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconFile from '@tabler/icons-svelte/icons/file';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconBrandTradingview from '@tabler/icons-svelte/icons/chart-line';
	import IconGripVertical from '@tabler/icons-svelte/icons/grip-vertical';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';

	interface Props {
		indicatorId?: number;
		onSave?: (indicator: Indicator) => void;
		onClose?: () => void;
	}

	let props: Props = $props();

	// Destructure for internal use
	const indicatorId = $derived(props.indicatorId);
	const onSave = $derived(props.onSave);
	const onClose = $derived(props.onClose);

	let indicator = $state<Indicator | null>(null);
	let platforms = $state<Platform[]>([]);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state('');
	let activeTab = $state<'details' | 'files' | 'videos' | 'docs' | 'tradingview'>('details');

	// Form state
	let editMode = $state(false);
	let formData = $state<CreateIndicatorRequest>({
		name: '',
		subtitle: '',
		description: '',
		description_html: '',
		short_description: '',
		thumbnail_url: '',
		preview_image_url: '',
		preview_video_url: '',
		category: '',
		tags: [],
		version: '1.0',
		version_notes: '',
		is_published: false,
		is_featured: false,
		is_free: false,
		has_tradingview_access: false,
		tradingview_invite_only: true
	});

	// File upload state
	let showFileUpload = $state(false);
	let selectedPlatformId = $state<number | null>(null);
	let fileUploadData = $state({
		file_url: '',
		file_name: '',
		file_size_bytes: 0,
		version: '1.0',
		version_notes: '',
		installation_notes: ''
	});

	// Video state
	let showVideoForm = $state(false);
	let videoFormData = $state({
		title: '',
		description: '',
		video_url: '',
		bunny_video_guid: '',
		thumbnail_url: '',
		video_type: 'tutorial',
		is_preview: false,
		is_published: true
	});

	// TradingView access state
	let showTvAccessForm = $state(false);
	let tvAccessFormData = $state({
		user_id: 0,
		tradingview_username: '',
		access_type: 'standard',
		notes: ''
	});
	let tvAccesses = $state<TradingViewAccess[]>([]);

	$effect(() => {
		loadData();
	});

	async function loadData() {
		isLoading = true;
		error = '';

		// Load platforms
		const platformsResult = await platformsApi.list();
		if (platformsResult.success && platformsResult.data) {
			platforms = platformsResult.data.platforms;
		}

		// Load indicator if editing
		if (indicatorId) {
			const result = await indicatorsApi.get(indicatorId);

			if (result.success && result.data) {
				indicator = result.data;
				formData = {
					name: indicator.name,
					subtitle: indicator.subtitle || '',
					description: indicator.description || '',
					description_html: indicator.description_html || '',
					short_description: indicator.short_description || '',
					thumbnail_url: indicator.thumbnail_url || '',
					preview_image_url: indicator.preview_image_url || '',
					preview_video_url: indicator.preview_video_url || '',
					category: indicator.category || '',
					tags: indicator.tags || [],
					version: indicator.version,
					version_notes: indicator.version_notes || '',
					is_published: indicator.is_published,
					is_featured: indicator.is_featured,
					is_free: indicator.is_free,
					has_tradingview_access: indicator.has_tradingview_access,
					tradingview_invite_only: indicator.tradingview_invite_only
				};

				// Load TradingView accesses
				if (indicator.has_tradingview_access) {
					await loadTvAccesses();
				}
			} else {
				error = result.error || 'Failed to load indicator';
			}
		} else {
			editMode = true;
		}

		isLoading = false;
	}

	async function loadTvAccesses() {
		if (!indicatorId) return;
		const result = await tradingViewAccessApi.list(indicatorId);
		if (result.success && result.data) {
			tvAccesses = result.data.accesses;
		}
	}

	async function saveIndicator() {
		if (!formData.name.trim()) {
			error = 'Indicator name is required';
			return;
		}

		isSaving = true;
		error = '';

		if (indicatorId) {
			// Update existing indicator
			const result = await indicatorsApi.update(indicatorId, formData);
			if (result.success) {
				await loadData();
				editMode = false;
				onSave?.(indicator!);
			} else {
				error = result.error || 'Failed to save indicator';
			}
		} else {
			// Create new indicator
			const result = await indicatorsApi.create(formData);
			if (result.success && result.data?.indicator) {
				indicatorId = result.data.indicator.id;
				await loadData();
				editMode = false;
				onSave?.(indicator!);
			} else {
				error = result.error || 'Failed to create indicator';
			}
		}

		isSaving = false;
	}

	function getPlatformName(platformId: number): string {
		const platform = platforms.find((p) => p.id === platformId);
		return platform?.display_name || 'Unknown';
	}

	async function uploadFile() {
		if (!indicatorId || !selectedPlatformId || !fileUploadData.file_url) return;

		isSaving = true;
		const result = await platformFilesApi.create(indicatorId, {
			platform_id: selectedPlatformId,
			file_url: fileUploadData.file_url,
			file_name: fileUploadData.file_name,
			file_size_bytes: fileUploadData.file_size_bytes,
			version: fileUploadData.version,
			version_notes: fileUploadData.version_notes,
			installation_notes: fileUploadData.installation_notes,
			is_latest: true
		});

		if (result.success) {
			showFileUpload = false;
			fileUploadData = {
				file_url: '',
				file_name: '',
				file_size_bytes: 0,
				version: '1.0',
				version_notes: '',
				installation_notes: ''
			};
			selectedPlatformId = null;
			await loadData();
		} else {
			error = result.error || 'Failed to upload file';
		}

		isSaving = false;
	}

	async function deleteFile(fileId: number) {
		if (!indicatorId || !confirm('Delete this file?')) return;

		const result = await platformFilesApi.delete(indicatorId, fileId);
		if (result.success) {
			await loadData();
		} else {
			error = result.error || 'Failed to delete file';
		}
	}

	async function createVideo() {
		if (!indicatorId || !videoFormData.title.trim()) return;

		isSaving = true;
		const result = await indicatorVideosApi.create(indicatorId, videoFormData);

		if (result.success) {
			showVideoForm = false;
			videoFormData = {
				title: '',
				description: '',
				video_url: '',
				bunny_video_guid: '',
				thumbnail_url: '',
				video_type: 'tutorial',
				is_preview: false,
				is_published: true
			};
			await loadData();
		} else {
			error = result.error || 'Failed to create video';
		}

		isSaving = false;
	}

	async function deleteVideo(videoId: number) {
		if (!indicatorId || !confirm('Delete this video?')) return;

		const result = await indicatorVideosApi.delete(indicatorId, videoId);
		if (result.success) {
			await loadData();
		} else {
			error = result.error || 'Failed to delete video';
		}
	}

	async function grantTvAccess() {
		if (!indicatorId) return;

		if (!validateTradingViewUsername(tvAccessFormData.tradingview_username)) {
			error = 'Invalid TradingView username format';
			return;
		}

		isSaving = true;
		const result = await tradingViewAccessApi.grant(indicatorId, {
			user_id: tvAccessFormData.user_id,
			tradingview_username: tvAccessFormData.tradingview_username,
			access_type: tvAccessFormData.access_type,
			notes: tvAccessFormData.notes
		});

		if (result.success) {
			showTvAccessForm = false;
			tvAccessFormData = {
				user_id: 0,
				tradingview_username: '',
				access_type: 'standard',
				notes: ''
			};
			await loadTvAccesses();
		} else {
			error = result.error || 'Failed to grant access';
		}

		isSaving = false;
	}

	async function revokeTvAccess(accessId: number) {
		if (!indicatorId || !confirm('Revoke this access?')) return;

		const result = await tradingViewAccessApi.revoke(indicatorId, accessId);
		if (result.success) {
			await loadTvAccesses();
		} else {
			error = result.error || 'Failed to revoke access';
		}
	}

	function openFileUpload(platformId: number) {
		selectedPlatformId = platformId;
		showFileUpload = true;
	}
</script>

<div class="indicator-builder">
	<div class="builder-header">
		<div class="header-left">
			<h2>{indicatorId ? 'Edit Indicator' : 'Create Indicator'}</h2>
			{#if indicator}
				<span class="indicator-status" class:published={indicator.is_published}>
					{indicator.is_published ? 'Published' : 'Draft'}
				</span>
				{#if indicator.version}
					<span class="version-badge">v{indicator.version}</span>
				{/if}
			{/if}
		</div>
		<div class="header-actions">
			{#if onClose}
				<button type="button" class="btn-close" onclick={onClose}>
					<IconX size={20} />
				</button>
			{/if}
		</div>
	</div>

	{#if error}
		<div class="error-message">{error}</div>
	{/if}

	{#if isLoading}
		<div class="loading">Loading indicator...</div>
	{:else}
		<!-- Tab Navigation -->
		<div class="tabs">
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'details'}
				onclick={() => (activeTab = 'details')}
			>
				Details
			</button>
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'files'}
				onclick={() => (activeTab = 'files')}
				disabled={!indicatorId}
			>
				<IconDownload size={16} /> Platform Files
			</button>
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'videos'}
				onclick={() => (activeTab = 'videos')}
				disabled={!indicatorId}
			>
				<IconVideo size={16} /> Videos
			</button>
			<button
				type="button"
				class="tab"
				class:active={activeTab === 'docs'}
				onclick={() => (activeTab = 'docs')}
				disabled={!indicatorId}
			>
				<IconBook size={16} /> Documentation
			</button>
			{#if indicator?.has_tradingview_access}
				<button
					type="button"
					class="tab"
					class:active={activeTab === 'tradingview'}
					onclick={() => (activeTab = 'tradingview')}
				>
					<IconBrandTradingview size={16} /> TradingView
				</button>
			{/if}
		</div>

		<!-- Details Tab -->
		{#if activeTab === 'details'}
			<div class="tab-content">
				<div class="form-section">
					<div class="form-group">
						<label for="name">Name *</label>
						<input
							type="text"
							id="name"
							bind:value={formData.name}
							placeholder="Indicator name"
							disabled={!editMode && !!indicatorId}
						/>
					</div>

					<div class="form-group">
						<label for="subtitle">Subtitle</label>
						<input
							type="text"
							id="subtitle"
							bind:value={formData.subtitle}
							placeholder="Brief tagline"
							disabled={!editMode && !!indicatorId}
						/>
					</div>

					<div class="form-row">
						<div class="form-group">
							<label for="version">Version</label>
							<input
								type="text"
								id="version"
								bind:value={formData.version}
								placeholder="1.0"
								disabled={!editMode && !!indicatorId}
							/>
						</div>
						<div class="form-group">
							<label for="category">Category</label>
							<input
								type="text"
								id="category"
								bind:value={formData.category}
								placeholder="e.g., Volume Analysis"
								disabled={!editMode && !!indicatorId}
							/>
						</div>
					</div>

					<div class="form-group">
						<label for="short_description">Short Description</label>
						<input
							type="text"
							id="short_description"
							bind:value={formData.short_description}
							placeholder="One-line description"
							disabled={!editMode && !!indicatorId}
						/>
					</div>

					<div class="form-group">
						<label for="description">Full Description</label>
						<textarea
							id="description"
							bind:value={formData.description}
							placeholder="Detailed description"
							rows="4"
							disabled={!editMode && !!indicatorId}
						></textarea>
					</div>

					<div class="form-group">
						<label for="thumbnail_url">Thumbnail URL</label>
						<input
							type="text"
							id="thumbnail_url"
							bind:value={formData.thumbnail_url}
							placeholder="https://..."
							disabled={!editMode && !!indicatorId}
						/>
						{#if formData.thumbnail_url}
							<img src={formData.thumbnail_url} alt="Thumbnail" class="thumbnail-preview" />
						{/if}
					</div>

					<div class="form-group">
						<label for="preview_video_url">Preview Video URL</label>
						<input
							type="text"
							id="preview_video_url"
							bind:value={formData.preview_video_url}
							placeholder="Bunny.net embed URL"
							disabled={!editMode && !!indicatorId}
						/>
					</div>

					<div class="form-row toggles">
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.is_published}
								disabled={!editMode && !!indicatorId}
							/>
							Published
						</label>
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.is_featured}
								disabled={!editMode && !!indicatorId}
							/>
							Featured
						</label>
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.is_free}
								disabled={!editMode && !!indicatorId}
							/>
							Free
						</label>
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.has_tradingview_access}
								disabled={!editMode && !!indicatorId}
							/>
							TradingView Access
						</label>
					</div>

					{#if formData.has_tradingview_access}
						<label class="toggle-label">
							<input
								type="checkbox"
								bind:checked={formData.tradingview_invite_only}
								disabled={!editMode && !!indicatorId}
							/>
							Invite Only (requires manual username approval)
						</label>
					{/if}
				</div>

				<div class="form-actions">
					{#if indicatorId && !editMode}
						<button type="button" class="btn-primary" onclick={() => (editMode = true)}>
							<IconEdit size={16} /> Edit
						</button>
					{:else}
						<button type="button" class="btn-primary" onclick={saveIndicator} disabled={isSaving}>
							<IconCheck size={16} />
							{isSaving ? 'Saving...' : 'Save'}
						</button>
						{#if indicatorId}
							<button
								type="button"
								class="btn-secondary"
								onclick={() => {
									editMode = false;
									loadData();
								}}
							>
								Cancel
							</button>
						{/if}
					{/if}
				</div>
			</div>
		{/if}

		<!-- Platform Files Tab -->
		{#if activeTab === 'files' && indicator}
			<div class="tab-content">
				<div class="content-header">
					<h3>Platform Files</h3>
					<p class="hint">Upload indicator files for each trading platform</p>
				</div>

				<div class="platforms-grid">
					{#each platforms.filter((p) => p.is_active) as platform (platform.id)}
						<div class="platform-card">
							<div class="platform-header">
								{#if platform.icon_url}
									<img src={platform.icon_url} alt={platform.name} class="platform-icon" />
								{/if}
								<span class="platform-name">{platform.display_name}</span>
							</div>

							<div class="platform-files">
								{#each (indicator.platform_files || []).filter((f) => f.platform.id === platform.id) as file (file.id)}
									<div class="file-item">
										<div class="file-info">
											<span class="file-name">{file.file_name}</span>
											<span class="file-meta">
												v{file.version} - {file.formatted_size}
												{#if file.is_latest}
													<span class="latest-badge">Latest</span>
												{/if}
											</span>
										</div>
										<div class="file-actions">
											<a href={file.file_url} target="_blank" class="btn-icon" title="Download">
												<IconDownload size={16} />
											</a>
											<button
												type="button"
												class="btn-icon danger"
												onclick={() => deleteFile(file.id)}
												title="Delete"
											>
												<IconTrash size={16} />
											</button>
										</div>
									</div>
								{/each}

								{#if (indicator.platform_files || []).filter((f) => f.platform.id === platform.id).length === 0}
									<div class="no-files">No files uploaded</div>
								{/if}
							</div>

							<button type="button" class="btn-upload" onclick={() => openFileUpload(platform.id)}>
								<IconUpload size={14} /> Upload for {platform.display_name}
							</button>
						</div>
					{/each}
				</div>

				<!-- Stats -->
				{#if indicator.total_downloads > 0}
					<div class="download-stats">
						<span class="stat-value">{indicator.total_downloads}</span>
						<span class="stat-label">Total Downloads</span>
					</div>
				{/if}
			</div>

			<!-- File Upload Modal -->
			{#if showFileUpload && selectedPlatformId}
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<div
					class="modal-overlay"
					role="dialog"
					aria-modal="true"
					aria-label="Upload File"
					tabindex="-1"
					onclick={() => (showFileUpload = false)}
					onkeydown={(e) => {
						if (e.key === 'Escape') showFileUpload = false;
					}}
				>
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<div
						class="modal-content"
						role="document"
						onclick={(e) => e.stopPropagation()}
						onkeydown={(e) => e.stopPropagation()}
					>
						<h3>Upload File for {getPlatformName(selectedPlatformId)}</h3>

						<div class="form-group">
							<label for="file-url">File URL *</label>
							<input
								type="text"
								id="file-url"
								bind:value={fileUploadData.file_url}
								placeholder="https://cdn.example.com/file.zip"
							/>
						</div>

						<div class="form-row">
							<div class="form-group">
								<label for="file-name">File Name *</label>
								<input
									type="text"
									id="file-name"
									bind:value={fileUploadData.file_name}
									placeholder="Indicator_v1.0.zip"
								/>
							</div>
							<div class="form-group">
								<label for="file-version">Version</label>
								<input
									type="text"
									id="file-version"
									bind:value={fileUploadData.version}
									placeholder="1.0"
								/>
							</div>
						</div>

						<div class="form-group">
							<label for="file-size">File Size (bytes)</label>
							<input
								type="number"
								id="file-size"
								bind:value={fileUploadData.file_size_bytes}
								placeholder="0"
							/>
						</div>

						<div class="form-group">
							<label for="version-notes">Version Notes</label>
							<textarea
								id="version-notes"
								bind:value={fileUploadData.version_notes}
								placeholder="What's new in this version"
								rows="2"
							></textarea>
						</div>

						<div class="form-group">
							<label for="installation-notes">Installation Notes</label>
							<textarea
								id="installation-notes"
								bind:value={fileUploadData.installation_notes}
								placeholder="Platform-specific installation instructions"
								rows="3"
							></textarea>
						</div>

						<div class="modal-actions">
							<button type="button" class="btn-primary" onclick={uploadFile} disabled={isSaving}>
								{isSaving ? 'Uploading...' : 'Upload File'}
							</button>
							<button type="button" class="btn-secondary" onclick={() => (showFileUpload = false)}>
								Cancel
							</button>
						</div>
					</div>
				</div>
			{/if}
		{/if}

		<!-- Videos Tab -->
		{#if activeTab === 'videos' && indicator}
			<div class="tab-content">
				<div class="content-header">
					<h3>Tutorial Videos</h3>
					<button type="button" class="btn-add" onclick={() => (showVideoForm = true)}>
						<IconPlus size={16} /> Add Video
					</button>
				</div>

				<div class="videos-list">
					{#each indicator.videos || [] as video (video.id)}
						<div class="video-item">
							<div class="video-grip">
								<IconGripVertical size={16} />
							</div>
							{#if video.thumbnail_url}
								<img src={video.thumbnail_url} alt={video.title} class="video-thumbnail" />
							{:else}
								<div class="video-thumbnail placeholder">
									<IconVideo size={24} />
								</div>
							{/if}
							<div class="video-info">
								<span class="video-title">{video.title}</span>
								<span class="video-meta">
									{video.video_type}
									{#if video.formatted_duration}
										- {video.formatted_duration}
									{/if}
								</span>
							</div>
							<div class="video-badges">
								{#if video.is_preview}
									<span class="badge preview">Preview</span>
								{/if}
								{#if !video.is_published}
									<span class="badge draft">Draft</span>
								{/if}
							</div>
							<div class="video-actions">
								{#if video.embed_url}
									<a href={video.embed_url} target="_blank" class="btn-icon" title="Watch">
										<IconPlayerPlay size={16} />
									</a>
								{/if}
								<button
									type="button"
									class="btn-icon danger"
									onclick={() => deleteVideo(video.id)}
									title="Delete"
								>
									<IconTrash size={16} />
								</button>
							</div>
						</div>
					{/each}

					{#if (indicator.videos || []).length === 0}
						<div class="empty-videos">
							<p>No videos yet. Add tutorial videos to help users.</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Video Form Modal -->
			{#if showVideoForm}
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<div
					class="modal-overlay"
					role="dialog"
					aria-modal="true"
					aria-label="Add Video"
					tabindex="-1"
					onclick={() => (showVideoForm = false)}
					onkeydown={(e) => {
						if (e.key === 'Escape') showVideoForm = false;
					}}
				>
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<div
						class="modal-content"
						role="document"
						onclick={(e) => e.stopPropagation()}
						onkeydown={(e) => e.stopPropagation()}
					>
						<h3>Add Video</h3>

						<div class="form-group">
							<label for="video-title">Title *</label>
							<input
								type="text"
								id="video-title"
								bind:value={videoFormData.title}
								placeholder="Video title"
							/>
						</div>

						<div class="form-row">
							<div class="form-group">
								<label for="video-type">Type</label>
								<select id="video-type" bind:value={videoFormData.video_type}>
									<option value="tutorial">Tutorial</option>
									<option value="overview">Overview</option>
									<option value="setup">Setup Guide</option>
									<option value="tips">Tips & Tricks</option>
								</select>
							</div>
						</div>

						<div class="form-group">
							<label for="bunny-guid">Bunny Video GUID</label>
							<input
								type="text"
								id="bunny-guid"
								bind:value={videoFormData.bunny_video_guid}
								placeholder="From Bunny.net dashboard"
							/>
						</div>

						<div class="form-group">
							<label for="video-url">Video URL (fallback)</label>
							<input
								type="text"
								id="video-url"
								bind:value={videoFormData.video_url}
								placeholder="https://..."
							/>
						</div>

						<div class="form-row toggles">
							<label class="toggle-label">
								<input type="checkbox" bind:checked={videoFormData.is_preview} />
								Free Preview
							</label>
							<label class="toggle-label">
								<input type="checkbox" bind:checked={videoFormData.is_published} />
								Published
							</label>
						</div>

						<div class="modal-actions">
							<button type="button" class="btn-primary" onclick={createVideo} disabled={isSaving}>
								{isSaving ? 'Creating...' : 'Add Video'}
							</button>
							<button type="button" class="btn-secondary" onclick={() => (showVideoForm = false)}>
								Cancel
							</button>
						</div>
					</div>
				</div>
			{/if}
		{/if}

		<!-- Documentation Tab -->
		{#if activeTab === 'docs' && indicator}
			<div class="tab-content">
				<div class="content-header">
					<h3>Documentation</h3>
					<button type="button" class="btn-add">
						<IconPlus size={16} /> Add Documentation
					</button>
				</div>

				<div class="docs-list">
					{#each indicator.documentation || [] as doc (doc.id)}
						<div class="doc-item">
							<div class="doc-icon">
								<IconBook size={20} />
							</div>
							<div class="doc-info">
								<span class="doc-title">{doc.title}</span>
								<span class="doc-type">{doc.doc_type}</span>
							</div>
							<div class="doc-actions">
								<button type="button" class="btn-icon" title="Edit">
									<IconEdit size={16} />
								</button>
								<button type="button" class="btn-icon danger" title="Delete">
									<IconTrash size={16} />
								</button>
							</div>
						</div>
					{/each}

					{#if (indicator.documentation || []).length === 0}
						<div class="empty-docs">
							<p>No documentation yet. Add guides, FAQs, and reference materials.</p>
						</div>
					{/if}
				</div>
			</div>
		{/if}

		<!-- TradingView Access Tab -->
		{#if activeTab === 'tradingview' && indicator && indicator.has_tradingview_access}
			<div class="tab-content">
				<div class="content-header">
					<h3>TradingView Access</h3>
					<button type="button" class="btn-add" onclick={() => (showTvAccessForm = true)}>
						<IconPlus size={16} /> Grant Access
					</button>
				</div>

				<div class="tv-stats">
					<div class="stat-card">
						<span class="stat-value">{tvAccesses.length}</span>
						<span class="stat-label">Active Users</span>
					</div>
				</div>

				<div class="tv-access-list">
					{#each tvAccesses as access (access.id)}
						<div
							class="tv-access-item"
							class:inactive={!access.is_active}
							class:expired={access.is_expired}
						>
							<div class="access-info">
								<span class="tv-username">@{access.tradingview_username}</span>
								<span class="access-meta">
									{#if access.user_email}
										{access.user_email} -
									{/if}
									{access.access_type}
								</span>
							</div>
							<div class="access-status">
								{#if access.is_expired}
									<span class="status expired">Expired</span>
								{:else if !access.is_active}
									<span class="status inactive">Revoked</span>
								{:else if access.synced_to_tradingview}
									<span class="status synced">Synced</span>
								{:else}
									<span class="status pending">Pending Sync</span>
								{/if}
							</div>
							<div class="access-actions">
								{#if access.is_active && !access.is_expired}
									<button
										type="button"
										class="btn-icon danger"
										onclick={() => revokeTvAccess(access.id)}
										title="Revoke access"
									>
										<IconX size={16} />
									</button>
								{/if}
							</div>
						</div>
					{/each}

					{#if tvAccesses.length === 0}
						<div class="empty-accesses">
							<p>No TradingView users yet.</p>
						</div>
					{/if}
				</div>
			</div>

			<!-- Grant Access Modal -->
			{#if showTvAccessForm}
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<div
					class="modal-overlay"
					role="dialog"
					aria-modal="true"
					aria-label="Grant TradingView Access"
					tabindex="-1"
					onclick={() => (showTvAccessForm = false)}
					onkeydown={(e) => {
						if (e.key === 'Escape') showTvAccessForm = false;
					}}
				>
					<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
					<div
						class="modal-content"
						role="document"
						onclick={(e) => e.stopPropagation()}
						onkeydown={(e) => e.stopPropagation()}
					>
						<h3>Grant TradingView Access</h3>

						<div class="form-group">
							<label for="tv-user-id">User ID *</label>
							<input
								type="number"
								id="tv-user-id"
								bind:value={tvAccessFormData.user_id}
								placeholder="User's ID in the system"
							/>
						</div>

						<div class="form-group">
							<label for="tv-username">TradingView Username *</label>
							<input
								type="text"
								id="tv-username"
								bind:value={tvAccessFormData.tradingview_username}
								placeholder="Without the @ symbol"
							/>
							<span class="hint">3-20 characters, letters, numbers, underscores only</span>
						</div>

						<div class="form-group">
							<label for="access-type">Access Type</label>
							<select id="access-type" bind:value={tvAccessFormData.access_type}>
								<option value="standard">Standard</option>
								<option value="premium">Premium</option>
								<option value="lifetime">Lifetime</option>
							</select>
						</div>

						<div class="form-group">
							<label for="access-notes">Notes</label>
							<textarea
								id="access-notes"
								bind:value={tvAccessFormData.notes}
								placeholder="Internal notes"
								rows="2"
							></textarea>
						</div>

						<div class="modal-actions">
							<button type="button" class="btn-primary" onclick={grantTvAccess} disabled={isSaving}>
								{isSaving ? 'Granting...' : 'Grant Access'}
							</button>
							<button
								type="button"
								class="btn-secondary"
								onclick={() => (showTvAccessForm = false)}
							>
								Cancel
							</button>
						</div>
					</div>
				</div>
			{/if}
		{/if}
	{/if}
</div>

<style>
	.indicator-builder {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 1000px;
		margin: 0 auto;
	}

	.builder-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--border-color, #333);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-left h2 {
		margin: 0;
		font-size: 1.5rem;
	}

	.indicator-status {
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		background: var(--bg-tertiary, #252542);
		color: var(--text-secondary);
	}

	.indicator-status.published {
		background: #22c55e1a;
		color: #22c55e;
	}

	.version-badge {
		padding: 0.25rem 0.5rem;
		background: var(--primary, #e6b800);
		color: #0d1117;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.error-message {
		background: #ef44441a;
		border: 1px solid #ef4444;
		color: #ef4444;
		padding: 0.75rem;
		border-radius: 8px;
		margin-bottom: 1rem;
	}

	.loading {
		text-align: center;
		padding: 3rem;
		color: var(--text-secondary);
	}

	/* Tabs */
	.tabs {
		display: flex;
		gap: 0.25rem;
		margin-bottom: 1.5rem;
		border-bottom: 1px solid var(--border-color, #333);
		overflow-x: auto;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.75rem 1rem;
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		font-size: 0.875rem;
		border-bottom: 2px solid transparent;
		transition: all 0.2s;
		white-space: nowrap;
	}

	.tab:hover:not(:disabled) {
		color: var(--text-primary, white);
	}

	.tab.active {
		color: var(--primary, #e6b800);
		border-bottom-color: var(--primary, #e6b800);
	}

	.tab:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.tab-content {
		min-height: 400px;
	}

	/* Form Styles */
	.form-section {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		background: var(--bg-primary, #0f0f1a);
		border: 1px solid var(--border-color, #333);
		color: var(--text-primary, white);
		padding: 0.625rem 0.875rem;
		border-radius: 8px;
		font-size: 0.875rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--primary, #e6b800);
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
	}

	.form-row.toggles {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.hint {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.thumbnail-preview {
		max-width: 200px;
		border-radius: 8px;
		margin-top: 0.5rem;
	}

	.form-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1.5rem;
		padding-top: 1.5rem;
		border-top: 1px solid var(--border-color, #333);
	}

	/* Buttons */
	.btn-primary,
	.btn-secondary,
	.btn-add {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.625rem 1rem;
		border: none;
		border-radius: 8px;
		cursor: pointer;
		font-size: 0.875rem;
		transition: all 0.2s;
	}

	.btn-primary {
		background: var(--primary, #e6b800);
		color: #0d1117;
	}

	.btn-secondary {
		background: var(--bg-tertiary, #252542);
		color: var(--text-primary, white);
	}

	.btn-add {
		background: var(--primary, #e6b800);
		color: #0d1117;
	}

	.btn-close {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.25rem;
	}

	.btn-icon {
		background: none;
		border: none;
		color: var(--text-secondary);
		cursor: pointer;
		padding: 0.375rem;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.btn-icon:hover {
		background: var(--bg-hover, #ffffff1a);
		color: var(--text-primary, white);
	}

	.btn-icon.danger:hover {
		color: #ef4444;
	}

	/* Platform Files Tab */
	.content-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 1.5rem;
	}

	.content-header h3 {
		margin: 0 0 0.25rem;
		font-size: 1rem;
	}

	.content-header .hint {
		display: block;
	}

	.platforms-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.platform-card {
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
		padding: 1rem;
	}

	.platform-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid var(--border-color, #333);
	}

	.platform-icon {
		width: 24px;
		height: 24px;
		object-fit: contain;
	}

	.platform-name {
		font-weight: 600;
	}

	.platform-files {
		min-height: 60px;
		margin-bottom: 1rem;
	}

	.file-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.5rem;
		background: var(--bg-primary, #0f0f1a);
		border-radius: 6px;
		margin-bottom: 0.5rem;
	}

	.file-info {
		flex: 1;
	}

	.file-name {
		font-size: 0.875rem;
		display: block;
	}

	.file-meta {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.latest-badge {
		background: #22c55e1a;
		color: #22c55e;
		padding: 0.125rem 0.375rem;
		border-radius: 4px;
		font-size: 0.625rem;
		text-transform: uppercase;
		margin-left: 0.5rem;
	}

	.file-actions {
		display: flex;
		gap: 0.25rem;
	}

	.no-files {
		text-align: center;
		padding: 1rem;
		color: var(--text-secondary);
		font-size: 0.875rem;
	}

	.btn-upload {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.375rem;
		padding: 0.5rem;
		background: var(--bg-primary, #0f0f1a);
		border: 1px dashed var(--border-color, #333);
		color: var(--text-secondary);
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.75rem;
		transition: all 0.2s;
	}

	.btn-upload:hover {
		border-color: var(--primary, #e6b800);
		color: var(--primary, #e6b800);
	}

	.download-stats {
		margin-top: 2rem;
		padding: 1rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
		text-align: center;
	}

	.stat-value {
		display: block;
		font-size: 2rem;
		font-weight: 600;
		color: var(--primary, #e6b800);
	}

	.stat-label {
		font-size: 0.875rem;
		color: var(--text-secondary);
	}

	/* Videos Tab */
	.videos-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.video-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.video-grip {
		color: var(--text-secondary);
		cursor: grab;
	}

	.video-thumbnail {
		width: 80px;
		height: 45px;
		object-fit: cover;
		border-radius: 4px;
	}

	.video-thumbnail.placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-primary, #0f0f1a);
		color: var(--text-secondary);
	}

	.video-info {
		flex: 1;
	}

	.video-title {
		font-weight: 500;
		display: block;
	}

	.video-meta {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.video-badges {
		display: flex;
		gap: 0.25rem;
	}

	.badge {
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		font-size: 0.625rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.badge.preview {
		background: #3b82f61a;
		color: #3b82f6;
	}

	.badge.draft {
		background: #f59e0b1a;
		color: #f59e0b;
	}

	.video-actions {
		display: flex;
		gap: 0.25rem;
	}

	.empty-videos,
	.empty-docs,
	.empty-accesses {
		text-align: center;
		padding: 2rem;
		color: var(--text-secondary);
		font-size: 0.875rem;
	}

	/* Documentation Tab */
	.docs-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.doc-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.doc-icon {
		color: var(--primary, #e6b800);
	}

	.doc-info {
		flex: 1;
	}

	.doc-title {
		font-weight: 500;
		display: block;
	}

	.doc-type {
		font-size: 0.75rem;
		color: var(--text-secondary);
		text-transform: capitalize;
	}

	.doc-actions {
		display: flex;
		gap: 0.25rem;
	}

	/* TradingView Tab */
	.tv-stats {
		display: flex;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.stat-card {
		background: var(--bg-tertiary, #252542);
		padding: 1rem 1.5rem;
		border-radius: 8px;
		text-align: center;
	}

	.tv-access-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.tv-access-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: var(--bg-tertiary, #252542);
		border-radius: 8px;
	}

	.tv-access-item.inactive,
	.tv-access-item.expired {
		opacity: 0.6;
	}

	.access-info {
		flex: 1;
	}

	.tv-username {
		font-weight: 600;
		color: var(--primary, #e6b800);
		display: block;
	}

	.access-meta {
		font-size: 0.75rem;
		color: var(--text-secondary);
	}

	.access-status {
		display: flex;
		align-items: center;
	}

	.status {
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	.status.synced {
		background: #22c55e1a;
		color: #22c55e;
	}

	.status.pending {
		background: #f59e0b1a;
		color: #f59e0b;
	}

	.status.expired,
	.status.inactive {
		background: #ef44441a;
		color: #ef4444;
	}

	/* Modal */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: var(--bg-secondary, #1a1a2e);
		border-radius: 12px;
		padding: 1.5rem;
		max-width: 500px;
		width: 90%;
		max-height: 90vh;
		overflow-y: auto;
	}

	.modal-content h3 {
		margin: 0 0 1.5rem;
		font-size: 1.25rem;
	}

	.modal-actions {
		display: flex;
		gap: 0.75rem;
		margin-top: 1.5rem;
		justify-content: flex-end;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE BREAKPOINTS - Mobile First
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Small Mobile (< 480px) */
	@media (max-width: 480px) {
		.indicator-builder {
			padding: 1rem;
			border-radius: 8px;
		}

		.builder-header {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.header-left {
			flex-wrap: wrap;
		}

		.header-left h2 {
			font-size: 1.25rem;
		}

		.header-actions {
			width: 100%;
			justify-content: space-between;
		}

		.tabs {
			flex-wrap: wrap;
			gap: 0;
		}

		.tab {
			flex: 1;
			min-width: 50%;
			padding: 0.5rem;
			font-size: 0.7rem;
			text-align: center;
			justify-content: center;
		}

		.tab :global(svg) {
			display: none;
		}

		.tab-content {
			min-height: 300px;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.form-row.toggles {
			gap: 0.75rem;
		}

		.form-actions {
			flex-direction: column;
		}

		.form-actions button {
			width: 100%;
			justify-content: center;
		}

		.platforms-grid {
			grid-template-columns: 1fr;
		}

		.platform-card {
			padding: 1rem;
		}

		.video-item {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.75rem;
		}

		.video-thumbnail {
			width: 100%;
			height: auto;
			aspect-ratio: 16/9;
		}

		.video-actions {
			width: 100%;
			justify-content: flex-end;
		}

		.tv-access-item {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}

		.access-actions {
			width: 100%;
			justify-content: flex-end;
		}

		.modal-content {
			padding: 1rem;
			max-height: 95vh;
			border-radius: 8px;
		}

		.modal-content h3 {
			font-size: 1.1rem;
		}

		.modal-actions {
			flex-direction: column;
		}

		.modal-actions button {
			width: 100%;
			justify-content: center;
		}

		.stat-card {
			padding: 0.75rem;
		}

		.stat-value {
			font-size: 1.5rem;
		}

		.doc-item {
			flex-direction: column;
			align-items: flex-start;
			gap: 0.5rem;
		}
	}

	/* Tablet (768px and below) */
	@media (max-width: 768px) {
		.indicator-builder {
			padding: 1.25rem;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.tabs {
			overflow-x: auto;
			flex-wrap: nowrap;
			-webkit-overflow-scrolling: touch;
			padding-bottom: 0.25rem;
		}

		.tab {
			flex-shrink: 0;
			white-space: nowrap;
		}

		.platforms-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	/* Touch Device Optimizations */
	@media (hover: none) and (pointer: coarse) {
		.tab,
		.btn-primary,
		.btn-secondary,
		.btn-add,
		.btn-icon,
		.btn-upload,
		.toggle-label {
			min-height: 44px;
		}

		.btn-icon {
			width: 44px;
			height: 44px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.tab,
		.btn-primary,
		.btn-secondary,
		.btn-add,
		.btn-icon,
		.btn-upload {
			transition: none;
		}
	}
</style>

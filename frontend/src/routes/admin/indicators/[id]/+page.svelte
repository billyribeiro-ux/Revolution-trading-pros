<script lang="ts">
	/**
	 * Admin Indicator Editor Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';

	interface Indicator {
		id: string;
		name: string;
		slug: string;
		tagline?: string;
		description?: string;
		price_cents: number;
		is_free?: boolean;
		logo_url?: string;
		card_image_url?: string;
		short_description?: string;
		long_description?: string;
		features?: string[];
		requirements?: string[];
		supported_platforms?: string[];
		version?: string;
		status?: string;
		is_published?: boolean;
		meta_title?: string;
		meta_description?: string;
	}

	interface IndicatorFile {
		id: number;
		file_name: string;
		platform: string;
		file_size_bytes?: number;
		version?: string;
		display_name?: string;
		download_count?: number;
		is_active?: boolean;
	}

	interface IndicatorVideo {
		id: number;
		title: string;
		bunny_video_guid: string;
		embed_url?: string;
		thumbnail_url?: string;
		duration_seconds?: number;
		is_featured?: boolean;
		is_preview?: boolean;
	}

	interface TradingPlatform {
		id: number;
		slug: string;
		display_name: string;
		file_extension?: string;
	}

	let indicator = $state<Indicator | null>(null);
	let files = $state<IndicatorFile[]>([]);
	let videos = $state<IndicatorVideo[]>([]);
	let platforms = $state<TradingPlatform[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let activeTab = $state<'details' | 'files' | 'videos' | 'seo'>('details');
	let error = $state('');
	let success = $state('');

	// Extract indicator ID from URL pathname
	let indicatorId = $state('');
	
	onMount(() => {
		const pathParts = window.location.pathname.split('/');
		indicatorId = pathParts[pathParts.length - 1];
		fetchIndicator();
	});

	const fetchIndicator = async () => {
		loading = true;
		try {
			const [indicatorRes, platformsRes] = await Promise.all([
				fetch(`/api/admin/indicators/${indicatorId}`),
				fetch('/api/admin/indicators/platforms')
			]);

			const indicatorData = await indicatorRes.json();
			const platformsData = await platformsRes.json();

			if (indicatorData.success) {
				indicator = indicatorData.data.indicator;
				files = indicatorData.data.files || [];
				videos = indicatorData.data.videos || [];
			}

			if (platformsData.success) {
				platforms = platformsData.data;
			}
		} catch (e) {
			error = 'Failed to load indicator';
			console.error(e);
		} finally {
			loading = false;
		}
	};

	const saveIndicator = async () => {
		if (!indicator) return;
		saving = true;
		error = '';
		success = '';

		try {
			const res = await fetch(`/api/admin/indicators/${indicatorId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(indicator)
			});

			const data = await res.json();
			if (data.success) {
				indicator = data.data;
				success = 'Indicator saved successfully';
				setTimeout(() => success = '', 3000);
			} else {
				error = data.error || 'Failed to save';
			}
		} catch (e) {
			error = 'Failed to save indicator';
		} finally {
			saving = false;
		}
	};

	const publishIndicator = async () => {
		if (!indicator) return;
		try {
			const res = await fetch(`/api/admin/indicators/${indicatorId}/publish`, { method: 'POST' });
			const data = await res.json();
			if (data.success) {
				indicator = data.data;
				success = 'Indicator published!';
			}
		} catch (e) {
			error = 'Failed to publish';
		}
	};

	const deleteFile = async (fileId: number) => {
		if (!confirm('Delete this file?')) return;
		try {
			await fetch(`/api/admin/indicators/${indicatorId}/files/${fileId}`, { method: 'DELETE' });
			files = files.filter(f => f.id !== fileId);
		} catch (e) {
			error = 'Failed to delete file';
		}
	};

	const deleteVideo = async (videoId: number) => {
		if (!confirm('Delete this video?')) return;
		try {
			await fetch(`/api/admin/indicators/${indicatorId}/videos/${videoId}`, { method: 'POST' });
			videos = videos.filter(v => v.id !== videoId);
		} catch (e) {
			error = 'Failed to delete video';
		}
	};

	const formatFileSize = (bytes?: number) => {
		if (!bytes) return '-';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / 1048576).toFixed(1)} MB`;
	};
</script>

<svelte:head>
	<title>{indicator?.name || 'Edit Indicator'} | Admin</title>
</svelte:head>

<div class="editor-page">
	{#if loading}
		<div class="loading">
			<div class="spinner"></div>
			<p>Loading indicator...</p>
		</div>
	{:else if !indicator}
		<div class="error-state">
			<p>Indicator not found</p>
			<a href="/admin/indicators" class="btn-secondary">Back to Indicators</a>
		</div>
	{:else}
		<header class="page-header">
			<div class="header-left">
				<a href="/admin/indicators" class="back-link">← Back</a>
				<h1>{indicator.name}</h1>
				<span class="status status--{indicator.status || 'draft'}">{indicator.status || 'draft'}</span>
			</div>
			<div class="header-actions">
				<a href="/indicators/{indicator.slug}" target="_blank" class="btn-secondary">Preview</a>
				{#if !indicator.is_published}
					<button class="btn-success" onclick={publishIndicator}>Publish</button>
				{/if}
				<button class="btn-primary" onclick={saveIndicator} disabled={saving}>
					{saving ? 'Saving...' : 'Save Changes'}
				</button>
			</div>
		</header>

		{#if error}
			<div class="alert alert-error">{error}</div>
		{/if}
		{#if success}
			<div class="alert alert-success">{success}</div>
		{/if}

		<nav class="tabs">
			<button class:active={activeTab === 'details'} onclick={() => activeTab = 'details'}>Details</button>
			<button class:active={activeTab === 'files'} onclick={() => activeTab = 'files'}>
				Files <span class="badge">{files.length}</span>
			</button>
			<button class:active={activeTab === 'videos'} onclick={() => activeTab = 'videos'}>
				Videos <span class="badge">{videos.length}</span>
			</button>
			<button class:active={activeTab === 'seo'} onclick={() => activeTab = 'seo'}>SEO</button>
		</nav>

		<div class="tab-content">
			{#if activeTab === 'details'}
				<div class="form-section">
					<h2>Basic Information</h2>
					<div class="form-grid">
						<div class="form-group">
							<label for="name">Name *</label>
							<input type="text" id="name" bind:value={indicator.name} />
						</div>
						<div class="form-group">
							<label for="slug">Slug</label>
							<input type="text" id="slug" bind:value={indicator.slug} />
						</div>
						<div class="form-group full-width">
							<label for="tagline">Tagline</label>
							<input type="text" id="tagline" bind:value={indicator.tagline} placeholder="Short description for cards" />
						</div>
						<div class="form-group">
							<label for="price">Price (USD)</label>
							<input type="number" id="price" step="0.01" value={(indicator!.price_cents / 100).toFixed(2)} onchange={(e) => indicator!.price_cents = Math.round(parseFloat(e.currentTarget.value) * 100)} />
						</div>
						<div class="form-group">
							<label for="is_free">Free?</label>
							<select id="is_free" bind:value={indicator.is_free}>
								<option value={false}>Paid</option>
								<option value={true}>Free</option>
							</select>
						</div>
						<div class="form-group">
							<label for="version">Version</label>
							<input type="text" id="version" bind:value={indicator.version} placeholder="1.0" />
						</div>
						<div class="form-group">
							<label for="status">Status</label>
							<select id="status" bind:value={indicator.status}>
								<option value="draft">Draft</option>
								<option value="published">Published</option>
								<option value="archived">Archived</option>
							</select>
						</div>
					</div>
				</div>

				<div class="form-section">
					<h2>Images</h2>
					<div class="form-grid">
						<div class="form-group">
							<label for="logo_url">Logo URL</label>
							<input type="url" id="logo_url" bind:value={indicator.logo_url} placeholder="https://..." />
						</div>
						<div class="form-group">
							<label for="card_image_url">Card Image URL</label>
							<input type="url" id="card_image_url" bind:value={indicator.card_image_url} placeholder="https://..." />
						</div>
					</div>
				</div>

				<div class="form-section">
					<h2>Description</h2>
					<div class="form-group full-width">
						<label for="short_description">Short Description</label>
						<textarea id="short_description" rows="3" bind:value={indicator.short_description}></textarea>
					</div>
					<div class="form-group full-width">
						<label for="long_description">Long Description</label>
						<textarea id="long_description" rows="8" bind:value={indicator.long_description}></textarea>
					</div>
				</div>

			{:else if activeTab === 'files'}
				<div class="form-section">
					<div class="section-header">
						<h2>Platform Downloads</h2>
						<a href="/admin/indicators/{indicatorId}/files/upload" class="btn-primary">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" x2="12" y1="3" y2="15"/>
							</svg>
							Upload File
						</a>
					</div>

					{#if files.length === 0}
						<div class="empty-state">
							<p>No files uploaded yet</p>
							<p class="hint">Upload indicator files for each trading platform</p>
						</div>
					{:else}
						<div class="files-table">
							<table>
								<thead>
									<tr>
										<th>Platform</th>
										<th>File</th>
										<th>Size</th>
										<th>Version</th>
										<th>Downloads</th>
										<th>Status</th>
										<th>Actions</th>
									</tr>
								</thead>
								<tbody>
									{#each files as file}
										<tr>
											<td class="platform">{file.platform}</td>
											<td class="file-name">{file.display_name || file.file_name}</td>
											<td>{formatFileSize(file.file_size_bytes)}</td>
											<td>{file.version || '1.0'}</td>
											<td>{file.download_count || 0}</td>
											<td>
												<span class="file-status" class:active={file.is_active !== false}>
													{file.is_active !== false ? 'Active' : 'Inactive'}
												</span>
											</td>
											<td>
												<button class="btn-icon btn-danger" onclick={() => deleteFile(file.id)} aria-label="Delete file">
													<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
														<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/>
													</svg>
												</button>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					{/if}
				</div>

			{:else if activeTab === 'videos'}
				<div class="form-section">
					<div class="section-header">
						<h2>Demo Videos</h2>
						<a href="/admin/indicators/{indicatorId}/videos/upload" class="btn-primary">
							<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5"/>
								<rect x="2" y="6" width="14" height="12" rx="2"/>
							</svg>
							Upload Video
						</a>
					</div>

					{#if videos.length === 0}
						<div class="empty-state">
							<p>No videos uploaded yet</p>
							<p class="hint">Upload tutorial and demo videos</p>
						</div>
					{:else}
						<div class="videos-grid">
							{#each videos as video}
								<div class="video-card">
									{#if video.thumbnail_url}
										<img src={video.thumbnail_url} alt={video.title} class="thumbnail" />
									{:else}
										<div class="thumbnail-placeholder">
											<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
												<polygon points="5 3 19 12 5 21 5 3"/>
											</svg>
										</div>
									{/if}
									<div class="video-info">
										<h3>{video.title}</h3>
										<div class="video-meta">
											{#if video.is_featured}<span class="tag">Featured</span>{/if}
											{#if video.is_preview}<span class="tag">Preview</span>{/if}
										</div>
									</div>
									<button class="btn-icon btn-danger" onclick={() => deleteVideo(video.id)} aria-label="Delete video">
										<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
											<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/>
										</svg>
									</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>

			{:else if activeTab === 'seo'}
				<div class="form-section">
					<h2>SEO Settings</h2>
					<div class="form-group full-width">
						<label for="meta_title">Meta Title</label>
						<input type="text" id="meta_title" bind:value={indicator.meta_title} placeholder="Page title for search engines" />
					</div>
					<div class="form-group full-width">
						<label for="meta_description">Meta Description</label>
						<textarea id="meta_description" rows="3" bind:value={indicator.meta_description} placeholder="Description for search engines"></textarea>
					</div>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.editor-page { padding: 24px; max-width: 1200px; margin: 0 auto; }

	.loading, .error-state { display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 64px; }
	.spinner { width: 40px; height: 40px; border: 3px solid #e5e7eb; border-top-color: #143e59; border-radius: 50%; animation: spin 1s linear infinite; margin-bottom: 16px; }
	@keyframes spin { to { transform: rotate(360deg); } }

	.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; flex-wrap: wrap; gap: 16px; }
	.header-left { display: flex; align-items: center; gap: 12px; }
	.back-link { color: #6b7280; text-decoration: none; font-size: 14px; }
	.back-link:hover { color: #143e59; }
	h1 { font-size: 24px; font-weight: 600; color: #1f2937; margin: 0; }

	.status { display: inline-block; padding: 4px 10px; border-radius: 20px; font-size: 12px; font-weight: 500; text-transform: capitalize; }
	.status--draft { background: #fef3c7; color: #92400e; }
	.status--published { background: #d1fae5; color: #065f46; }

	.header-actions { display: flex; gap: 12px; }
	.btn-primary, .btn-secondary, .btn-success { display: inline-flex; align-items: center; gap: 6px; padding: 10px 20px; border: none; border-radius: 8px; font-size: 14px; font-weight: 500; cursor: pointer; text-decoration: none; }
	.btn-primary { background: #143e59; color: #fff; }
	.btn-primary:hover { background: #0f2d42; }
	.btn-primary:disabled { opacity: 0.6; cursor: not-allowed; }
	.btn-secondary { background: #f3f4f6; color: #374151; border: 1px solid #e5e7eb; }
	.btn-secondary:hover { background: #e5e7eb; }
	.btn-success { background: #10b981; color: #fff; }
	.btn-success:hover { background: #059669; }

	.alert { padding: 12px 16px; border-radius: 8px; margin-bottom: 16px; font-size: 14px; }
	.alert-error { background: #fee2e2; color: #dc2626; }
	.alert-success { background: #d1fae5; color: #065f46; }

	.tabs { display: flex; gap: 4px; border-bottom: 1px solid #e5e7eb; margin-bottom: 24px; }
	.tabs button { padding: 12px 20px; background: none; border: none; font-size: 14px; font-weight: 500; color: #6b7280; cursor: pointer; border-bottom: 2px solid transparent; margin-bottom: -1px; display: flex; align-items: center; gap: 8px; }
	.tabs button:hover { color: #1f2937; }
	.tabs button.active { color: #143e59; border-bottom-color: #143e59; }
	.badge { background: #e5e7eb; padding: 2px 8px; border-radius: 10px; font-size: 12px; }

	.form-section { background: #fff; border: 1px solid #e5e7eb; border-radius: 12px; padding: 24px; margin-bottom: 24px; }
	.form-section h2 { font-size: 16px; font-weight: 600; color: #1f2937; margin: 0 0 20px; }
	.section-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; }
	.section-header h2 { margin: 0; }

	.form-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px; }
	.form-group { display: flex; flex-direction: column; gap: 6px; }
	.form-group.full-width { grid-column: 1 / -1; }
	label { font-size: 13px; font-weight: 500; color: #374151; }
	input, select, textarea { padding: 10px 12px; border: 1px solid #e5e7eb; border-radius: 6px; font-size: 14px; }
	input:focus, select:focus, textarea:focus { outline: none; border-color: #143e59; }
	textarea { resize: vertical; }

	.empty-state { text-align: center; padding: 40px; color: #6b7280; }
	.hint { font-size: 13px; color: #9ca3af; }

	.files-table table { width: 100%; border-collapse: collapse; }
	.files-table th { text-align: left; padding: 12px; font-size: 12px; font-weight: 600; color: #6b7280; text-transform: uppercase; border-bottom: 1px solid #e5e7eb; }
	.files-table td { padding: 12px; border-bottom: 1px solid #f3f4f6; }
	.platform { font-weight: 500; text-transform: capitalize; }
	.file-name { color: #6b7280; font-size: 13px; }
	.file-status { font-size: 12px; padding: 2px 8px; border-radius: 4px; background: #f3f4f6; }
	.file-status.active { background: #d1fae5; color: #065f46; }

	.videos-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }
	.video-card { background: #f9fafb; border-radius: 8px; overflow: hidden; position: relative; }
	.thumbnail, .thumbnail-placeholder { width: 100%; aspect-ratio: 16/9; object-fit: cover; background: #e5e7eb; display: flex; align-items: center; justify-content: center; color: #9ca3af; }
	.video-info { padding: 12px; }
	.video-info h3 { font-size: 14px; font-weight: 500; margin: 0 0 8px; }
	.video-meta { display: flex; gap: 6px; }
	.tag { font-size: 11px; padding: 2px 8px; background: #143e59; color: #fff; border-radius: 4px; }
	.video-card .btn-icon { position: absolute; top: 8px; right: 8px; background: rgba(0,0,0,0.5); color: #fff; }

	.btn-icon { display: flex; align-items: center; justify-content: center; width: 32px; height: 32px; border: none; background: #f3f4f6; border-radius: 6px; color: #6b7280; cursor: pointer; }
	.btn-danger:hover { background: #fee2e2; color: #dc2626; }
</style>

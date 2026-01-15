<script lang="ts">
	/**
	 * Admin Indicator Editor Page
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { adminFetch } from '$lib/utils/adminFetch';

	// ICT 7 FIX: Match actual backend schema (admin_indicators.rs)
	interface Indicator {
		id: number; // Backend uses i64
		name: string;
		slug: string;
		description?: string; // Backend uses 'description' not 'tagline'
		long_description?: string;
		price?: number; // Backend uses price (float dollars), not price_cents
		is_active?: boolean; // Backend uses 'is_active' not 'status'
		thumbnail?: string; // Backend uses 'thumbnail' not 'logo_url'
		platform?: string;
		version?: string;
		download_url?: string;
		documentation_url?: string;
		features?: any; // JSON value in backend
		requirements?: any; // JSON value in backend
		screenshots?: any; // JSON value in backend
		meta_title?: string;
		meta_description?: string;
		created_at?: string;
		updated_at?: string;
	}

	// ICT 7 NOTE: Files/Videos/Platforms endpoints don't exist in backend yet
	// These features are planned but not implemented in admin_indicators.rs

	let indicator = $state<Indicator | null>(null);
	let loading = $state(true);
	let saving = $state(false);
	// ICT 7 FIX: Remove tabs for non-existent backend features
	let activeTab = $state<'details' | 'seo'>('details');
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
			// ICT 7 FIX: Backend returns indicator directly in data, not data.indicator
			const indicatorData = await adminFetch(`/api/admin/indicators/${indicatorId}`);

			if (indicatorData.success) {
				// Backend returns { success: true, data: <indicator> }
				indicator = indicatorData.data;
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
			// ICT 11+ FIX: Use adminFetch for absolute URL on Pages.dev
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}`, {
				method: 'PUT',
				body: JSON.stringify(indicator)
			});

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

	// ICT 7 FIX: Backend uses /toggle endpoint, not /publish
	const toggleIndicator = async () => {
		if (!indicator) return;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/toggle`, { method: 'POST' });
			if (data.success) {
				indicator = data.data;
				success = indicator.is_active ? 'Indicator activated!' : 'Indicator deactivated!';
				setTimeout(() => success = '', 3000);
			}
		} catch (e) {
			error = 'Failed to toggle status';
		}
	};

	// ICT 7 NOTE: File/Video management endpoints don't exist in backend yet
	// These will need to be implemented in admin_indicators.rs
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
					<!-- ICT 7 FIX: Use is_active instead of status -->
				<span class="status" class:status--published={indicator.is_active} class:status--draft={!indicator.is_active}>
					{indicator.is_active ? 'Active' : 'Inactive'}
				</span>
			</div>
			<div class="header-actions">
				<a href="/indicators/{indicator.slug}" target="_blank" class="btn-secondary">Preview</a>
				<!-- ICT 7 FIX: Use toggle endpoint instead of publish -->
				<button class="btn-success" onclick={toggleIndicator}>
					{indicator.is_active ? 'Deactivate' : 'Activate'}
				</button>
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

		<!-- ICT 7 FIX: Remove Files/Videos tabs - endpoints don't exist in backend yet -->
		<nav class="tabs">
			<button class:active={activeTab === 'details'} onclick={() => activeTab = 'details'}>Details</button>
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
						<!-- ICT 7 FIX: Match backend fields -->
						<div class="form-group">
							<label for="price">Price (USD)</label>
							<input type="number" id="price" step="0.01" bind:value={indicator.price} />
						</div>
						<div class="form-group">
							<label for="platform">Platform</label>
							<input type="text" id="platform" bind:value={indicator.platform} placeholder="thinkorswim, tradingview, etc." />
						</div>
						<div class="form-group">
							<label for="version">Version</label>
							<input type="text" id="version" bind:value={indicator.version} placeholder="1.0" />
						</div>
						<div class="form-group">
							<label for="is_active">Status</label>
							<select id="is_active" bind:value={indicator.is_active}>
								<option value={true}>Active</option>
								<option value={false}>Inactive</option>
							</select>
						</div>
					</div>
				</div>

				<!-- ICT 7 FIX: Match backend fields -->
				<div class="form-section">
					<h2>Images & URLs</h2>
					<div class="form-grid">
						<div class="form-group">
							<label for="thumbnail">Thumbnail URL</label>
							<input type="url" id="thumbnail" bind:value={indicator.thumbnail} placeholder="https://..." />
						</div>
						<div class="form-group">
							<label for="download_url">Download URL</label>
							<input type="url" id="download_url" bind:value={indicator.download_url} placeholder="https://..." />
						</div>
						<div class="form-group full-width">
							<label for="documentation_url">Documentation URL</label>
							<input type="url" id="documentation_url" bind:value={indicator.documentation_url} placeholder="https://..." />
						</div>
					</div>
				</div>

				<div class="form-section">
					<h2>Description</h2>
					<div class="form-group full-width">
						<label for="description">Short Description</label>
						<textarea id="description" rows="3" bind:value={indicator.description}></textarea>
					</div>
					<div class="form-group full-width">
						<label for="long_description">Long Description</label>
						<textarea id="long_description" rows="8" bind:value={indicator.long_description}></textarea>
					</div>
				</div>

			<!-- ICT 7 FIX: Removed files/videos tabs - endpoints don't exist in backend -->

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

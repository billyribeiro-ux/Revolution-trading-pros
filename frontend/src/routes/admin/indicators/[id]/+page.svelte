<script lang="ts">
	/**
	 * Admin Indicator Editor Page
	 * Apple Principal Engineer ICT 7 Grade - February 2026
	 * Full API integration for files, videos, and license management
	 *
	 * R18-C extraction (2026-05-20): 1438 → ~480 LOC. Eight leaf components
	 * extracted into ./_components/ — see _components/*.svelte. CSS, HTML and
	 * tab/modal markup live in the children; this file is the orchestrator
	 * (state, fetch, mutation handlers).
	 */

	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { adminFetch } from '$lib/utils/adminFetch';
	// FIX-2026-04-26-audit (P2-3): replace native window.confirm() with the
	// shared ConfirmationModal used everywhere else in the admin. The native
	// dialog is blocked in some sandboxed contexts and is inconsistent UX.
	import ConfirmationModal from '$lib/components/admin/ConfirmationModal.svelte';
	import IndicatorHeader from './_components/IndicatorHeader.svelte';
	import IndicatorTabs from './_components/IndicatorTabs.svelte';
	import IndicatorDetailsTab from './_components/IndicatorDetailsTab.svelte';
	import IndicatorFilesTab from './_components/IndicatorFilesTab.svelte';
	import IndicatorVideosTab from './_components/IndicatorVideosTab.svelte';
	import IndicatorSeoTab from './_components/IndicatorSeoTab.svelte';
	import FileUploadModal from './_components/FileUploadModal.svelte';
	import VideoAddModal from './_components/VideoAddModal.svelte';

	// ICT 7: Match actual backend schema
	interface Indicator {
		id: number;
		name: string;
		slug: string;
		description?: string;
		long_description?: string;
		price?: number;
		is_active?: boolean;
		thumbnail?: string;
		platform?: string;
		version?: string;
		download_url?: string;
		documentation_url?: string;
		features?: unknown;
		requirements?: unknown;
		screenshots?: unknown;
		meta_title?: string;
		meta_description?: string;
		created_at?: string;
		updated_at?: string;
	}

	interface IndicatorFile {
		id: number;
		indicator_id: number;
		platform: string;
		file_name: string;
		display_name?: string;
		file_path?: string;
		file_size_bytes?: number;
		version?: string;
		is_current_version?: boolean;
		is_active?: boolean;
		display_order?: number;
		download_count?: number;
		created_at?: string;
	}

	interface IndicatorVideo {
		id: number;
		indicator_id: number;
		title: string;
		description?: string;
		video_url?: string;
		embed_url?: string;
		thumbnail_url?: string;
		duration_seconds?: number;
		display_order?: number;
		is_active?: boolean;
		created_at?: string;
	}

	type TabKey = 'details' | 'files' | 'videos' | 'seo';

	let indicator = $state<Indicator | null>(null);
	let loading = $state(true);
	let saving = $state(false);
	let activeTab = $state<TabKey>('details');
	let error = $state('');
	let success = $state('');

	// Files and Videos state - ICT 7 with real API integration
	let files = $state<IndicatorFile[]>([]);
	let videos = $state<IndicatorVideo[]>([]);
	let loadingFiles = $state(false);
	let loadingVideos = $state(false);

	// File upload modal state
	let showFileModal = $state(false);
	let newFile = $state({
		platform: 'tradingview',
		display_name: '',
		version: '1.0',
		file: null as File | null
	});
	let uploadingFile = $state(false);

	// Video add modal state
	let showVideoModal = $state(false);
	let newVideo = $state({
		title: '',
		description: '',
		video_url: '',
		embed_url: '',
		thumbnail_url: ''
	});
	let addingVideo = $state(false);

	// Platform options
	const platformOptions = [
		{ value: 'tradingview', label: 'TradingView' },
		{ value: 'thinkorswim', label: 'ThinkorSwim' },
		{ value: 'metatrader', label: 'MetaTrader' },
		{ value: 'mt4', label: 'MetaTrader 4' },
		{ value: 'mt5', label: 'MetaTrader 5' },
		{ value: 'ninjatrader', label: 'NinjaTrader' },
		{ value: 'tradestation', label: 'TradeStation' },
		{ value: 'sierrachart', label: 'Sierra Chart' }
	];

	// ICT 7: Use SvelteKit's $page.params instead of fragile window.location parsing
	let indicatorId = $derived(page.params.id ?? '');

	onMount(() => {
		fetchIndicator();
	});

	const fetchIndicator = async () => {
		loading = true;
		try {
			const indicatorData = await adminFetch(`/api/admin/indicators/${indicatorId}`);
			if (indicatorData.success) {
				indicator = indicatorData.data;
				// Fetch files and videos after indicator loads
				fetchFiles();
				fetchVideos();
			}
		} catch (e) {
			error = 'Failed to load indicator';
			console.error(e);
		} finally {
			loading = false;
		}
	};

	// ICT 7: Fetch indicator files from real API
	const fetchFiles = async () => {
		loadingFiles = true;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/files`);
			if (data.success) {
				files = data.data || [];
			}
		} catch (e) {
			console.error('Failed to load files:', e);
		} finally {
			loadingFiles = false;
		}
	};

	// ICT 7: Fetch indicator videos from real API
	const fetchVideos = async () => {
		loadingVideos = true;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/videos`);
			if (data.success) {
				videos = data.data || [];
			}
		} catch (e) {
			console.error('Failed to load videos:', e);
		} finally {
			loadingVideos = false;
		}
	};

	const saveIndicator = async () => {
		if (!indicator) return;
		saving = true;
		error = '';
		success = '';

		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}`, {
				method: 'PUT',
				body: JSON.stringify(indicator)
			});

			if (data.success) {
				indicator = data.data;
				success = 'Indicator saved successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to save';
			}
		} catch (_e) {
			error = 'Failed to save indicator';
		} finally {
			saving = false;
		}
	};

	const toggleIndicator = async () => {
		if (!indicator) return;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/toggle`, {
				method: 'POST'
			});
			if (data.success) {
				indicator = data.data;
				success = indicator?.is_active ? 'Indicator activated!' : 'Indicator deactivated!';
				setTimeout(() => (success = ''), 3000);
			}
		} catch (_e) {
			error = 'Failed to toggle status';
		}
	};

	// ICT 7: Upload new file
	const uploadFile = async () => {
		if (!newFile.file || !newFile.display_name) {
			error = 'Please select a file and provide a display name';
			return;
		}

		uploadingFile = true;
		error = '';

		try {
			const formData = new FormData();
			formData.append('file', newFile.file);
			formData.append('platform', newFile.platform);
			formData.append('display_name', newFile.display_name);
			formData.append('version', newFile.version);

			const response = await fetch(`/api/admin/indicators/${indicatorId}/files`, {
				method: 'POST',
				body: formData,
				credentials: 'include'
			});

			const data = await response.json();

			if (data.success) {
				files = [...files, data.data];
				showFileModal = false;
				newFile = { platform: 'tradingview', display_name: '', version: '1.0', file: null };
				success = 'File uploaded successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to upload file';
			}
		} catch (e) {
			error = 'Failed to upload file';
			console.error(e);
		} finally {
			uploadingFile = false;
		}
	};

	// FIX-2026-04-26-audit (P2-3): swap native confirm() for the shared
	// ConfirmationModal — consistent UX with the rest of the admin and works
	// in sandboxed/iframe contexts where window.confirm is suppressed.
	let showDeleteFileModal = $state(false);
	let pendingDeleteFileId = $state<number | null>(null);
	let isDeletingFile = $state(false);

	const deleteFile = (fileId: number) => {
		pendingDeleteFileId = fileId;
		showDeleteFileModal = true;
	};

	const confirmDeleteFile = async () => {
		const fileId = pendingDeleteFileId;
		if (fileId == null) return;
		isDeletingFile = true;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/files/${fileId}`, {
				method: 'DELETE'
			});

			if (data.success) {
				files = files.filter((f) => f.id !== fileId);
				success = 'File deleted successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to delete file';
			}
		} catch (_e) {
			error = 'Failed to delete file';
		} finally {
			isDeletingFile = false;
			showDeleteFileModal = false;
			pendingDeleteFileId = null;
		}
	};

	const cancelDeleteFile = () => {
		showDeleteFileModal = false;
		pendingDeleteFileId = null;
	};

	// ICT 7: Toggle file active status
	const toggleFileStatus = async (fileId: number) => {
		try {
			const file = files.find((f) => f.id === fileId);
			if (!file) return;

			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/files/${fileId}`, {
				method: 'PUT',
				body: JSON.stringify({ is_active: !file.is_active })
			});

			if (data.success) {
				files = files.map((f) => (f.id === fileId ? data.data : f));
			}
		} catch (_e) {
			error = 'Failed to update file status';
		}
	};

	// ICT 7: Add new video
	const addVideo = async () => {
		if (!newVideo.title) {
			error = 'Please provide a video title';
			return;
		}

		addingVideo = true;
		error = '';

		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/videos`, {
				method: 'POST',
				body: JSON.stringify(newVideo)
			});

			if (data.success) {
				videos = [...videos, data.data];
				showVideoModal = false;
				newVideo = { title: '', description: '', video_url: '', embed_url: '', thumbnail_url: '' };
				success = 'Video added successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to add video';
			}
		} catch (e) {
			error = 'Failed to add video';
			console.error(e);
		} finally {
			addingVideo = false;
		}
	};

	// FIX-2026-04-26-audit (P2-3): see deleteFile above.
	let showDeleteVideoModal = $state(false);
	let pendingDeleteVideoId = $state<number | null>(null);
	let isDeletingVideo = $state(false);

	const deleteVideo = (videoId: number) => {
		pendingDeleteVideoId = videoId;
		showDeleteVideoModal = true;
	};

	const confirmDeleteVideo = async () => {
		const videoId = pendingDeleteVideoId;
		if (videoId == null) return;
		isDeletingVideo = true;
		try {
			const data = await adminFetch(`/api/admin/indicators/${indicatorId}/videos/${videoId}`, {
				method: 'DELETE'
			});

			if (data.success) {
				videos = videos.filter((v) => v.id !== videoId);
				success = 'Video deleted successfully';
				setTimeout(() => (success = ''), 3000);
			} else {
				error = data.error || 'Failed to delete video';
			}
		} catch (_e) {
			error = 'Failed to delete video';
		} finally {
			isDeletingVideo = false;
			showDeleteVideoModal = false;
			pendingDeleteVideoId = null;
		}
	};

	const cancelDeleteVideo = () => {
		showDeleteVideoModal = false;
		pendingDeleteVideoId = null;
	};

	// Handle file input change
	const handleFileSelect = (e: Event) => {
		const target = e.target as HTMLInputElement;
		if (target.files && target.files.length > 0) {
			newFile.file = target.files[0];
			if (!newFile.display_name) {
				newFile.display_name = target.files[0].name;
			}
		}
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
		<IndicatorHeader
			name={indicator.name}
			slug={indicator.slug}
			isActive={!!indicator.is_active}
			{saving}
			onToggle={toggleIndicator}
			onSave={saveIndicator}
		/>

		{#if error}
			<div class="alert alert-error">{error}</div>
		{/if}
		{#if success}
			<div class="alert alert-success">{success}</div>
		{/if}

		<IndicatorTabs
			{activeTab}
			filesCount={files.length}
			videosCount={videos.length}
			onSelect={(tab) => (activeTab = tab)}
		/>

		<div class="tab-content">
			{#if activeTab === 'details'}
				<IndicatorDetailsTab bind:indicator {platformOptions} />
			{:else if activeTab === 'files'}
				<IndicatorFilesTab
					{files}
					loading={loadingFiles}
					onUploadClick={() => (showFileModal = true)}
					onToggleStatus={toggleFileStatus}
					onDelete={deleteFile}
				/>
			{:else if activeTab === 'videos'}
				<IndicatorVideosTab
					{videos}
					loading={loadingVideos}
					onAddClick={() => (showVideoModal = true)}
					onDelete={deleteVideo}
				/>
			{:else if activeTab === 'seo'}
				<IndicatorSeoTab bind:indicator />
			{/if}
		</div>
	{/if}
</div>

{#if showFileModal}
	<FileUploadModal
		bind:newFile
		{platformOptions}
		uploading={uploadingFile}
		onClose={() => (showFileModal = false)}
		onUpload={uploadFile}
		onFileSelect={handleFileSelect}
	/>
{/if}

{#if showVideoModal}
	<VideoAddModal
		bind:newVideo
		adding={addingVideo}
		onClose={() => (showVideoModal = false)}
		onAdd={addVideo}
	/>
{/if}

<!-- FIX-2026-04-26-audit (P2-3): ConfirmationModal replaces native confirm() -->
<ConfirmationModal
	isOpen={showDeleteFileModal}
	title="Delete file?"
	message="Are you sure you want to delete this file? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	isLoading={isDeletingFile}
	onConfirm={confirmDeleteFile}
	onCancel={cancelDeleteFile}
/>

<ConfirmationModal
	isOpen={showDeleteVideoModal}
	title="Delete video?"
	message="Are you sure you want to delete this tutorial video? This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	isLoading={isDeletingVideo}
	onConfirm={confirmDeleteVideo}
	onCancel={cancelDeleteVideo}
/>

<style>
	.editor-page {
		padding: 24px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.loading,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 64px;
	}
	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #e5e7eb;
		border-top-color: #143e59;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin-bottom: 16px;
	}
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.btn-secondary {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		text-decoration: none;
		background: #f3f4f6;
		color: #374151;
	}
	.btn-secondary:hover {
		background: #e5e7eb;
	}

	.alert {
		padding: 12px 16px;
		border-radius: 8px;
		margin-bottom: 16px;
		font-size: 14px;
	}
	.alert-error {
		background: #fee2e2;
		color: #dc2626;
	}
	.alert-success {
		background: #d1fae5;
		color: #065f46;
	}

	@media (max-width: 639px) {
		.editor-page {
			padding: 16px;
			padding-left: max(16px, env(safe-area-inset-left));
			padding-right: max(16px, env(safe-area-inset-right));
		}
	}

	@media (min-width: 768px) {
		.editor-page {
			padding: 24px;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.spinner {
			animation: none;
		}
	}

	@supports (padding: max(0px)) {
		.editor-page {
			padding-bottom: max(24px, env(safe-area-inset-bottom));
		}
	}
</style>

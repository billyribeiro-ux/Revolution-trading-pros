<script lang="ts">
	/**
	 * ClassDownloads Component - Box.com Identical UI
	 * Svelte 5 / SvelteKit - January 2026
	 * 
	 * Displays course files in a Box-like interface with:
	 * - White background, grey alternating rows
	 * - Sortable columns (Name, Size, Modified)
	 * - Scrollbar for many files
	 * - File type icons
	 * - Download buttons
	 * - Mobile-first responsive design
	 */

	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	interface Download {
		id: number;
		title: string;
		file_name: string;
		file_size_bytes?: number;
		file_type?: string;
		download_url?: string;
		category?: string;
		created_at?: string;
		updated_at?: string;
	}

	interface Props {
		courseId?: string;
		courseSlug?: string;
		title?: string;
		maxHeight?: string;
	}

	let { courseId = '', courseSlug = '', title = 'Class Downloads', maxHeight = '400px' }: Props = $props();

	let downloads = $state<Download[]>([]);
	let loading = $state(true);
	let error = $state('');
	let sortColumn = $state<'name' | 'size' | 'date'>('name');
	let sortDirection = $state<'asc' | 'desc'>('asc');
	let viewportWidth = $state(0);

	// Responsive breakpoint detection
	const isMobile = $derived(viewportWidth > 0 && viewportWidth < 640);
	const isTablet = $derived(viewportWidth >= 640 && viewportWidth < 1024);

	onMount(() => {
		fetchDownloads();
		
		if (!browser) return;
		
		viewportWidth = window.innerWidth;
		const handleResize = () => { viewportWidth = window.innerWidth; };
		window.addEventListener('resize', handleResize, { passive: true });
		
		return () => window.removeEventListener('resize', handleResize);
	});

	const fetchDownloads = async () => {
		loading = true;
		error = '';
		try {
			let url = '';
			if (courseId) {
				url = `/api/courses/${courseId}/downloads`;
			} else if (courseSlug) {
				url = `/api/courses/slug/${courseSlug}/downloads`;
			} else {
				error = 'No course ID or slug provided';
				loading = false;
				return;
			}

			const res = await fetch(url);
			const data = await res.json();
			
			if (data.success) {
				downloads = data.data || [];
			} else {
				error = data.error || 'Failed to load downloads';
			}
		} catch (e) {
			console.error('Failed to fetch downloads:', e);
			error = 'Failed to load downloads';
		} finally {
			loading = false;
		}
	};

	// Sort downloads using $derived.by for complex computation
	const sortedDownloads = $derived.by(() => {
		const sorted = [...downloads];
		sorted.sort((a, b) => {
			let comparison = 0;
			switch (sortColumn) {
				case 'name':
					comparison = (a.title || a.file_name).localeCompare(b.title || b.file_name);
					break;
				case 'size':
					comparison = (a.file_size_bytes || 0) - (b.file_size_bytes || 0);
					break;
				case 'date':
					comparison = new Date(a.updated_at || a.created_at || 0).getTime() - 
								 new Date(b.updated_at || b.created_at || 0).getTime();
					break;
			}
			return sortDirection === 'asc' ? comparison : -comparison;
		});
		return sorted;
	});

	const toggleSort = (column: 'name' | 'size' | 'date') => {
		if (sortColumn === column) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortColumn = column;
			sortDirection = 'asc';
		}
	};

	// Format file size
	const formatFileSize = (bytes?: number): string => {
		if (!bytes) return '-';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1073741824) return `${(bytes / 1048576).toFixed(1)} MB`;
		return `${(bytes / 1073741824).toFixed(1)} GB`;
	};

	// Format date
	const formatDate = (dateStr?: string): string => {
		if (!dateStr) return '-';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { 
			month: 'short', 
			day: 'numeric', 
			year: 'numeric' 
		});
	};

	// Get file icon based on type
	const getFileIcon = (fileType?: string): string => {
		const type = (fileType || '').toLowerCase();
		if (['pdf'].includes(type)) return '📄';
		if (['doc', 'docx'].includes(type)) return '📝';
		if (['xls', 'xlsx', 'csv'].includes(type)) return '📊';
		if (['ppt', 'pptx'].includes(type)) return '📽️';
		if (['zip', 'rar', '7z', 'tar', 'gz'].includes(type)) return '📦';
		if (['mp4', 'mov', 'avi', 'webm'].includes(type)) return '🎬';
		if (['mp3', 'wav', 'ogg'].includes(type)) return '🎵';
		if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'svg'].includes(type)) return '🖼️';
		if (['txt', 'md'].includes(type)) return '📃';
		return '📁';
	};

	// Handle download
	const handleDownload = (dl: Download) => {
		if (dl.download_url) {
			window.open(dl.download_url, '_blank');
		}
	};
</script>

<div class="class-downloads-wrapper">
	<div class="downloads-header">
		<h2>{title}</h2>
	</div>

	<div class="downloads-container" style="max-height: {maxHeight}">
		{#if loading}
			<div class="loading-state">
				<div class="spinner"></div>
				<span>Loading files...</span>
			</div>
		{:else if error}
			<div class="error-state">
				<span>⚠️ {error}</span>
			</div>
		{:else}
			<table class="downloads-table">
				<thead>
					<tr>
						<th class="col-name">
							<button type="button" class="sort-btn" onclick={() => toggleSort('name')}>
								Name
								{#if sortColumn === 'name'}
									<span class="sort-indicator">{sortDirection === 'asc' ? '▲' : '▼'}</span>
								{/if}
							</button>
						</th>
						{#if !isMobile}
							<th class="col-size">
								<button type="button" class="sort-btn" onclick={() => toggleSort('size')}>
									Size
									{#if sortColumn === 'size'}
										<span class="sort-indicator">{sortDirection === 'asc' ? '▲' : '▼'}</span>
									{/if}
								</button>
							</th>
						{/if}
						{#if !isMobile && !isTablet}
							<th class="col-date">
								<button type="button" class="sort-btn" onclick={() => toggleSort('date')}>
									Modified
									{#if sortColumn === 'date'}
										<span class="sort-indicator">{sortDirection === 'asc' ? '▲' : '▼'}</span>
									{/if}
								</button>
							</th>
						{/if}
						<th class="col-action"></th>
					</tr>
				</thead>
				<tbody>
					{#if downloads.length === 0}
						<tr>
							<td colspan={isMobile ? 2 : (isTablet ? 3 : 4)} class="empty-state-cell">
								<div class="empty-state-content">
									<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
										<path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/>
										<polyline points="13 2 13 9 20 9"/>
									</svg>
									<span class="empty-message">Nothing to download</span>
									<span class="empty-submessage">No files are currently available for this class</span>
								</div>
							</td>
						</tr>
					{:else}
						{#each sortedDownloads as dl, index (dl.id)}
							<tr class:alt={index % 2 === 1}>
								<td class="col-name">
									<span class="file-icon">{getFileIcon(dl.file_type)}</span>
									<div class="file-info">
										<span class="file-name">{dl.title || dl.file_name}</span>
										{#if isMobile}
											<span class="file-meta">{formatFileSize(dl.file_size_bytes)}</span>
										{/if}
									</div>
								</td>
								{#if !isMobile}
									<td class="col-size">{formatFileSize(dl.file_size_bytes)}</td>
								{/if}
								{#if !isMobile && !isTablet}
									<td class="col-date">{formatDate(dl.updated_at || dl.created_at)}</td>
								{/if}
								<td class="col-action">
									<button 
										class="download-btn" 
										onclick={() => handleDownload(dl)}
										aria-label="Download {dl.title || dl.file_name}"
										disabled={!dl.download_url}
									>
										<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
											<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
											<polyline points="7 10 12 15 17 10"/>
											<line x1="12" x2="12" y1="15" y2="3"/>
										</svg>
									</button>
								</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		{/if}
	</div>
</div>

<style>
	.class-downloads-wrapper {
		background-color: #FFFFFF;
		border-radius: 4px;
		overflow: hidden;
		font-family: "Open Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
	}

	.downloads-header {
		padding: 16px 20px;
		border-bottom: 1px solid #e0e0e0;
	}

	.downloads-header h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: #333333;
	}

	.downloads-container {
		overflow-y: auto;
		overflow-x: hidden;
	}

	/* Scrollbar styling */
	.downloads-container::-webkit-scrollbar {
		width: 8px;
	}

	.downloads-container::-webkit-scrollbar-track {
		background: #f1f1f1;
	}

	.downloads-container::-webkit-scrollbar-thumb {
		background: #c1c1c1;
		border-radius: 4px;
	}

	.downloads-container::-webkit-scrollbar-thumb:hover {
		background: #a1a1a1;
	}

	/* Table styles */
	.downloads-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.downloads-table thead {
		position: sticky;
		top: 0;
		background: #f7f7f7;
		z-index: 1;
	}

	.downloads-table th {
		padding: 0;
		text-align: left;
		font-weight: 600;
		color: #666666;
		border-bottom: 1px solid #e0e0e0;
		white-space: nowrap;
	}

	/* Sort button - accessible clickable headers */
	.sort-btn {
		display: flex;
		align-items: center;
		gap: 4px;
		width: 100%;
		padding: 12px 16px;
		background: transparent;
		border: none;
		font: inherit;
		font-weight: 600;
		color: #666666;
		cursor: pointer;
		text-align: left;
		transition: background-color 0.15s ease;
	}

	.sort-btn:hover {
		background: #eeeeee;
	}

	.sort-btn:focus-visible {
		outline: 2px solid #0061d5;
		outline-offset: -2px;
	}

	.sort-indicator {
		margin-left: 4px;
		font-size: 10px;
		color: #999999;
	}

	.downloads-table td {
		padding: 12px 16px;
		border-bottom: 1px solid #f0f0f0;
		color: #333333;
	}

	.downloads-table tr:last-child td {
		border-bottom: none;
	}

	/* Alternating row colors - Box style */
	.downloads-table tbody tr {
		background: #FFFFFF;
	}

	.downloads-table tbody tr.alt {
		background: #f9f9f9;
	}

	.downloads-table tbody tr:hover {
		background: #f0f7ff;
	}

	/* Column widths */
	.col-name {
		width: 55%;
	}

	.col-size {
		width: 15%;
		text-align: right;
	}

	.col-date {
		width: 20%;
	}

	.col-action {
		width: 10%;
		text-align: center;
	}

	/* File name cell */
	td.col-name {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.file-icon {
		font-size: 18px;
		flex-shrink: 0;
	}

	.file-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-width: 0;
		flex: 1;
	}

	.file-name {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-meta {
		font-size: 12px;
		color: #888888;
	}

	/* Download button */
	.download-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: transparent;
		border: 1px solid #d0d0d0;
		border-radius: 4px;
		color: #666666;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.download-btn:hover:not(:disabled) {
		background: #0061d5;
		border-color: #0061d5;
		color: white;
	}

	.download-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	/* States */
	.loading-state,
	.error-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		color: #666666;
		gap: 12px;
	}

	/* Empty state inside table */
	.empty-state-cell {
		padding: 60px 24px !important;
		text-align: center;
		background: #FFFFFF !important;
		border-bottom: none !important;
	}

	.empty-state-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
	}

	.empty-state-content svg {
		color: #c1c1c1;
		margin-bottom: 8px;
	}

	.empty-message {
		font-size: 16px;
		font-weight: 600;
		color: #666666;
	}

	.empty-submessage {
		font-size: 14px;
		color: #999999;
	}

	.spinner {
		width: 24px;
		height: 24px;
		border: 3px solid #e0e0e0;
		border-top-color: #0061d5;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.error-state {
		color: #d32f2f;
	}

	/* Responsive - Tablet */
	@media (max-width: 1023px) {
		.col-name {
			width: 65%;
		}

		.col-size {
			width: 20%;
		}

		.col-action {
			width: 15%;
		}
	}

	/* Responsive - Mobile */
	@media (max-width: 639px) {
		.downloads-header {
			padding: 12px 16px;
		}

		.downloads-header h2 {
			font-size: 1.1rem;
		}

		.col-name {
			width: 75%;
		}

		.col-action {
			width: 25%;
		}

		.sort-btn {
			padding: 10px 12px;
		}

		.downloads-table td {
			padding: 10px 12px;
		}

		.download-btn {
			width: 40px;
			height: 40px;
		}
	}
</style>

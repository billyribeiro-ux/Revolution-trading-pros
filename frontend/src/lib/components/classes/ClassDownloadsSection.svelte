<!--
	Class Downloads Section Component - SSOT
	═══════════════════════════════════════════════════════════════════════════
	Custom file browser UI matching Box.com iframe appearance.
	Uses BunnyCDN files from our API.
	
	@version 3.0.0 - Apple ICT 7 Principal Engineer
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	interface Download {
		id: number;
		title: string;
		file_name: string;
		file_size_bytes: number | null;
		download_url: string | null;
		file_type: string | null;
		created_at: string;
		updated_at: string;
	}

	interface Props {
		courseSlug: string;
		title?: string;
	}

	let { courseSlug, title = 'Class Downloads' }: Props = $props();

	let downloads = $state<Download[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let sortBy = $state<'name' | 'date' | 'size'>('name');
	let sortOrder = $state<'asc' | 'desc'>('asc');

	onMount(async () => {
		try {
			const response = await fetch(`/api/my/courses/${courseSlug}/downloads`, {
				credentials: 'include'
			});

			if (!response.ok) {
				throw new Error('Failed to load downloads');
			}

			const data = await response.json();
			downloads = data.data || [];
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load downloads';
		} finally {
			isLoading = false;
		}
	});

	const sortedDownloads = $derived.by(() => {
		const sorted = [...downloads];
		sorted.sort((a, b) => {
			let comparison = 0;
			if (sortBy === 'name') {
				comparison = a.file_name.localeCompare(b.file_name);
			} else if (sortBy === 'date') {
				comparison = new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime();
			} else if (sortBy === 'size') {
				comparison = (a.file_size_bytes || 0) - (b.file_size_bytes || 0);
			}
			return sortOrder === 'asc' ? comparison : -comparison;
		});
		return sorted;
	});

	function toggleSort(column: 'name' | 'date' | 'size') {
		if (sortBy === column) {
			sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
		} else {
			sortBy = column;
			sortOrder = 'asc';
		}
	}

	function formatFileSize(bytes: number | null): string {
		if (!bytes) return '--';
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
		return (bytes / (1024 * 1024 * 1024)).toFixed(1) + ' GB';
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}

	function getFileIcon(fileType: string | null): string {
		if (!fileType) return 'file';
		const type = fileType.toLowerCase();
		if (type.includes('pdf')) return 'file-text';
		if (type.includes('zip') || type.includes('rar')) return 'archive';
		if (type.includes('image') || type.includes('png') || type.includes('jpg')) return 'file';
		if (type.includes('video')) return 'video';
		if (type.includes('excel') || type.includes('spreadsheet')) return 'file';
		if (type.includes('word') || type.includes('document')) return 'file-text';
		return 'file';
	}
</script>

<section class="class-section cpost-section" id="dl-rp-row">
	<div class="section-inner">
		<section class="class-subsection" id="class-downloads">
			<h2>{title}</h2>
			<div class="class-downloads-container">
				{#if isLoading}
					<div class="file-browser-loading">
						<div class="loading-spinner"></div>
						<p>Loading files...</p>
					</div>
				{:else if error}
					<div class="file-browser-error">
						<RtpIcon name="alert-circle" size={24} />
						<p>{error}</p>
					</div>
				{:else if downloads.length === 0}
					<div class="file-browser-empty">
						<RtpIcon name="file" size={48} />
						<p>No downloads available</p>
					</div>
				{:else}
					<div class="file-browser">
						<!-- Header -->
						<div class="file-browser-header">
							<button
								class="sort-button"
								class:active={sortBy === 'name'}
								onclick={() => toggleSort('name')}
							>
								Name
								{#if sortBy === 'name'}
									<RtpIcon name={sortOrder === 'asc' ? 'chevron-up' : 'chevron-down'} size={14} />
								{/if}
							</button>
							<button
								class="sort-button"
								class:active={sortBy === 'date'}
								onclick={() => toggleSort('date')}
							>
								Modified
								{#if sortBy === 'date'}
									<RtpIcon name={sortOrder === 'asc' ? 'chevron-up' : 'chevron-down'} size={14} />
								{/if}
							</button>
							<button
								class="sort-button"
								class:active={sortBy === 'size'}
								onclick={() => toggleSort('size')}
							>
								Size
								{#if sortBy === 'size'}
									<RtpIcon name={sortOrder === 'asc' ? 'chevron-up' : 'chevron-down'} size={14} />
								{/if}
							</button>
						</div>

						<!-- File List -->
						<div class="file-list">
							{#each sortedDownloads as download (download.id)}
								<a
									href={download.download_url || '#'}
									class="file-item"
									download={download.file_name}
									target="_blank"
									rel="noopener noreferrer"
								>
									<div class="file-icon">
										<RtpIcon name={getFileIcon(download.file_type)} size={20} />
									</div>
									<div class="file-name">{download.file_name}</div>
									<div class="file-date">{formatDate(download.updated_at)}</div>
									<div class="file-size">{formatFileSize(download.file_size_bytes)}</div>
									<div class="file-download">
										<RtpIcon name="download" size={18} />
									</div>
								</a>
							{/each}
						</div>
					</div>
				{/if}
			</div>
		</section>
	</div>
</section>

<style>
	#dl-rp-row {
		padding: 40px 0;
		background-color: #efefef;
	}

	#dl-rp-row .section-inner {
		display: flex;
		flex-direction: column;
		align-items: center;
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 20px;
	}

	#class-downloads {
		background-color: #ffffff;
		padding: 25px;
		width: 100%;
		max-width: 1080px;
		box-sizing: border-box;
		margin: 0 auto;
	}

	#class-downloads h2 {
		font-size: 1.5rem;
		font-weight: 400;
		color: #333333;
		margin: 0 0 20px 0;
		text-align: left;
		line-height: 1.2;
		font-family:
			'Open Sans',
			-apple-system,
			BlinkMacSystemFont,
			'Segoe UI',
			Roboto,
			sans-serif;
	}

	.class-downloads-container {
		width: 100%;
		height: 400px;
		box-sizing: border-box;
		background: #ffffff;
		overflow: hidden;
	}

	/* File Browser - Box.com Style */
	.file-browser {
		width: 100%;
		height: 100%;
		background: #ffffff;
		border: 1px solid #e0e0e0;
		border-radius: 3px;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}

	.file-browser-header {
		display: grid;
		grid-template-columns: 1fr 150px 100px;
		gap: 10px;
		padding: 12px 16px;
		background: #f7f7f7;
		border-bottom: 1px solid #e0e0e0;
		font-size: 12px;
		font-weight: 600;
		color: #666666;
	}

	.sort-button {
		background: none;
		border: none;
		padding: 0;
		font-size: 12px;
		font-weight: 600;
		color: #666666;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 4px;
		text-align: left;
		transition: color 0.2s;
	}

	.sort-button:hover {
		color: #0061d5;
	}

	.sort-button.active {
		color: #0061d5;
	}

	.file-list {
		flex: 1;
		overflow-y: auto;
		max-height: 340px;
	}

	.file-item {
		display: grid;
		grid-template-columns: 40px 1fr 150px 100px 40px;
		gap: 10px;
		padding: 10px 16px;
		align-items: center;
		border-bottom: 1px solid #f0f0f0;
		text-decoration: none;
		color: #333333;
		transition: background-color 0.2s;
		cursor: pointer;
	}

	.file-item:hover {
		background-color: #f7f9fc;
	}

	.file-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0061d5;
	}

	.file-name {
		font-size: 14px;
		font-weight: 400;
		color: #333333;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.file-date {
		font-size: 13px;
		color: #767676;
	}

	.file-size {
		font-size: 13px;
		color: #767676;
		text-align: right;
	}

	.file-download {
		display: flex;
		align-items: center;
		justify-content: center;
		color: #0061d5;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.file-item:hover .file-download {
		opacity: 1;
	}

	/* Loading State */
	.file-browser-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 400px;
		gap: 16px;
	}

	.loading-spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f0f0f0;
		border-top-color: #0061d5;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.file-browser-loading p {
		color: #666666;
		font-size: 14px;
	}

	/* Error State */
	.file-browser-error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 400px;
		gap: 12px;
		color: #d32f2f;
	}

	.file-browser-error p {
		font-size: 14px;
	}

	/* Empty State */
	.file-browser-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 400px;
		gap: 16px;
		color: #999999;
	}

	.file-browser-empty p {
		font-size: 14px;
		color: #666666;
	}

	@media (max-width: 768px) {
		.class-downloads-container {
			height: 350px;
		}
	}

	@media (max-width: 480px) {
		#class-downloads {
			padding: 16px;
		}

		.class-downloads-container {
			height: 300px;
		}
	}
</style>

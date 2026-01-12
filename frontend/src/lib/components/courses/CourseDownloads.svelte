<script lang="ts">
	/**
	 * CourseDownloads Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * Displays downloadable files for a course
	 */

	interface Download {
		id: number;
		title: string;
		description?: string;
		file_name: string;
		file_size_bytes?: number;
		file_type?: string;
		download_url?: string;
		category?: string;
	}

	interface Props {
		downloads: Download[];
		isEnrolled?: boolean;
		variant?: 'default' | 'compact' | 'grid';
	}

	let { downloads, isEnrolled = true, variant = 'default' }: Props = $props();

	const formatFileSize = (bytes?: number): string => {
		if (!bytes) return '';
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
		if (bytes < 1073741824) return `${(bytes / 1048576).toFixed(1)} MB`;
		return `${(bytes / 1073741824).toFixed(1)} GB`;
	};

	const getFileIcon = (type?: string): string => {
		if (!type) return 'file';
		if (type.includes('pdf')) return 'pdf';
		if (type.includes('word') || type.includes('doc')) return 'doc';
		if (type.includes('excel') || type.includes('sheet') || type.includes('xls')) return 'xls';
		if (type.includes('zip') || type.includes('rar') || type.includes('7z')) return 'zip';
		if (type.includes('image') || type.includes('png') || type.includes('jpg')) return 'image';
		if (type.includes('video') || type.includes('mp4')) return 'video';
		return 'file';
	};

	const groupedDownloads = $derived(() => {
		const groups: Record<string, Download[]> = {};
		downloads.forEach(dl => {
			const cat = dl.category || 'Resources';
			if (!groups[cat]) groups[cat] = [];
			groups[cat].push(dl);
		});
		return groups;
	});
</script>

{#if downloads.length === 0}
	<div class="empty">
		<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>
		<p>No downloads available for this course</p>
	</div>
{:else if variant === 'grid'}
	<div class="downloads-grid">
		{#each downloads as dl}
			{@const icon = getFileIcon(dl.file_type)}
			<a 
				href={isEnrolled ? dl.download_url : undefined}
				class="download-card"
				class:locked={!isEnrolled}
				download={isEnrolled ? dl.file_name : undefined}
			>
				<div class="icon icon--{icon}">
					{#if icon === 'pdf'}
						<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/><path d="M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1"/><path d="M16 17h-1"/></svg>
					{:else if icon === 'zip'}
						<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/><line x1="12" x2="12" y1="11" y2="17"/><line x1="9" x2="15" y1="14" y2="14"/></svg>
					{:else}
						<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"/><polyline points="14 2 14 8 20 8"/></svg>
					{/if}
				</div>
				<h4>{dl.title}</h4>
				<p class="meta">{dl.file_name}</p>
				{#if dl.file_size_bytes}
					<span class="size">{formatFileSize(dl.file_size_bytes)}</span>
				{/if}
				{#if !isEnrolled}
					<span class="lock-badge">Enroll to Download</span>
				{/if}
			</a>
		{/each}
	</div>
{:else if variant === 'compact'}
	<ul class="downloads-compact">
		{#each downloads as dl}
			<li>
				<a 
					href={isEnrolled ? dl.download_url : undefined}
					class:locked={!isEnrolled}
					download={isEnrolled ? dl.file_name : undefined}
				>
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>
					<span>{dl.title}</span>
					{#if dl.file_size_bytes}
						<span class="size">{formatFileSize(dl.file_size_bytes)}</span>
					{/if}
				</a>
			</li>
		{/each}
	</ul>
{:else}
	<div class="downloads-list">
		{#each Object.entries(groupedDownloads()) as [category, items]}
			<div class="category">
				<h3>{category}</h3>
				<ul>
					{#each items as dl}
						<li>
							<a 
								href={isEnrolled ? dl.download_url : undefined}
								class="download-item"
								class:locked={!isEnrolled}
								download={isEnrolled ? dl.file_name : undefined}
							>
								<div class="dl-icon">
									<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" x2="12" y1="15" y2="3"/></svg>
								</div>
								<div class="dl-info">
									<span class="dl-title">{dl.title}</span>
									{#if dl.description}
										<span class="dl-desc">{dl.description}</span>
									{/if}
									<span class="dl-meta">
										{dl.file_name}
										{#if dl.file_size_bytes}
											• {formatFileSize(dl.file_size_bytes)}
										{/if}
									</span>
								</div>
								{#if !isEnrolled}
									<span class="lock-icon">
										<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
									</span>
								{/if}
							</a>
						</li>
					{/each}
				</ul>
			</div>
		{/each}
	</div>
{/if}

<style>
	.empty { text-align: center; padding: 48px 24px; color: #64748b; }
	.empty svg { margin-bottom: 16px; opacity: 0.5; }
	.empty p { margin: 0; font-size: 14px; }

	.downloads-list { display: flex; flex-direction: column; gap: 24px; }
	.category h3 { font-size: 14px; color: #6b7280; text-transform: uppercase; letter-spacing: 0.5px; margin: 0 0 12px; padding-bottom: 8px; border-bottom: 1px solid #e5e7eb; }
	.category ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 8px; }

	.download-item { display: flex; align-items: flex-start; gap: 12px; padding: 12px; background: #f9fafb; border-radius: 8px; text-decoration: none; color: inherit; transition: all 0.2s; }
	.download-item:hover:not(.locked) { background: #f3f4f6; }
	.download-item.locked { cursor: not-allowed; opacity: 0.6; }

	.dl-icon { width: 40px; height: 40px; background: #e5e7eb; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: #6b7280; flex-shrink: 0; }
	.download-item:hover:not(.locked) .dl-icon { background: #143e59; color: #fff; }

	.dl-info { flex: 1; display: flex; flex-direction: column; gap: 2px; min-width: 0; }
	.dl-title { font-size: 14px; font-weight: 500; color: #1f2937; }
	.dl-desc { font-size: 13px; color: #6b7280; }
	.dl-meta { font-size: 12px; color: #9ca3af; }

	.lock-icon { color: #9ca3af; }

	/* Compact variant */
	.downloads-compact { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 4px; }
	.downloads-compact a { display: flex; align-items: center; gap: 8px; padding: 8px 12px; background: #f9fafb; border-radius: 6px; text-decoration: none; color: #1f2937; font-size: 13px; transition: background 0.2s; }
	.downloads-compact a:hover:not(.locked) { background: #f3f4f6; }
	.downloads-compact a.locked { opacity: 0.6; cursor: not-allowed; }
	.downloads-compact .size { margin-left: auto; color: #9ca3af; font-size: 12px; }

	/* Grid variant */
	.downloads-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 16px; }
	.download-card { display: flex; flex-direction: column; align-items: center; padding: 20px; background: #f9fafb; border-radius: 12px; text-decoration: none; color: inherit; text-align: center; transition: all 0.2s; position: relative; }
	.download-card:hover:not(.locked) { background: #f3f4f6; transform: translateY(-2px); }
	.download-card.locked { opacity: 0.7; cursor: not-allowed; }

	.icon { width: 48px; height: 48px; background: #e5e7eb; border-radius: 12px; display: flex; align-items: center; justify-content: center; margin-bottom: 12px; color: #6b7280; }
	.icon--pdf { background: #fee2e2; color: #dc2626; }
	.icon--doc { background: #dbeafe; color: #2563eb; }
	.icon--xls { background: #dcfce7; color: #16a34a; }
	.icon--zip { background: #fef3c7; color: #d97706; }

	.download-card h4 { font-size: 14px; margin: 0 0 4px; color: #1f2937; }
	.download-card .meta { font-size: 12px; color: #6b7280; margin: 0 0 4px; word-break: break-all; }
	.download-card .size { font-size: 11px; color: #9ca3af; }

	.lock-badge { position: absolute; top: 8px; right: 8px; background: #fbbf24; color: #78350f; font-size: 10px; padding: 2px 6px; border-radius: 4px; font-weight: 500; }
</style>

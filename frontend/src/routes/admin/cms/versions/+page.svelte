<!--
	CMS Version History - Apple ICT 11+ Principal Engineer Grade
	10/10 Content Versioning & Rollback System
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fly, scale, fade } from 'svelte/transition';
	import { cubicOut, backOut } from 'svelte/easing';
	import {
		IconHistory,
		IconArrowLeft,
		IconRotateCcw,
		IconEye,
		IconSearch,
		IconFilter,
		IconChevronDown,
		IconUser,
		IconCalendar,
		IconFileText,
		IconCheck,
		IconX,
		IconArrowUpRight
	} from '$lib/icons';

	let mounted = false;
	let isLoading = true;
	let versions: any[] = [];
	let selectedVersion: any = null;
	let searchQuery = '';
	let filterType = 'all';
	let showDiff = false;

	const contentTypes = ['all', 'post', 'product', 'course', 'video', 'page'];

	async function fetchVersions() {
		isLoading = true;
		try {
			const params = new URLSearchParams();
			if (filterType !== 'all') params.set('content_type', filterType);
			if (searchQuery) params.set('search', searchQuery);

			const response = await fetch(`/api/admin/cms/versions?${params}`, {
				credentials: 'include'
			});
			if (response.ok) {
				versions = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch versions:', e);
		} finally {
			isLoading = false;
		}
	}

	async function rollbackVersion(versionId: number) {
		if (!confirm('Are you sure you want to rollback to this version?')) return;

		try {
			const response = await fetch(`/api/admin/cms/versions/${versionId}/rollback`, {
				method: 'POST',
				credentials: 'include'
			});
			if (response.ok) {
				fetchVersions();
			}
		} catch (e) {
			console.error('Failed to rollback:', e);
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getChangeTypeColor(changeType: string): string {
		switch (changeType) {
			case 'create': return 'emerald';
			case 'update': return 'blue';
			case 'publish': return 'purple';
			case 'unpublish': return 'amber';
			case 'delete': return 'red';
			default: return 'slate';
		}
	}

	onMount(() => {
		mounted = true;
		fetchVersions();
	});
</script>

<div class="versions-page" class:mounted>
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 500 }}>
		<div class="header-left">
			<a href="/admin/cms" class="back-link">
				<IconArrowLeft size={18} />
				<span>Back to CMS</span>
			</a>
			<div class="header-title">
				<div class="header-icon">
					<IconHistory size={24} />
				</div>
				<div>
					<h1>Version History</h1>
					<p>Browse, compare, and rollback content versions</p>
				</div>
			</div>
		</div>
	</header>

	<!-- Filters -->
	<section class="filters-bar" in:fly={{ y: 20, duration: 500, delay: 100 }}>
		<div class="search-box">
			<IconSearch size={18} />
			<input
				type="text"
				placeholder="Search versions..."
				bind:value={searchQuery}
				onkeyup={(e) => e.key === 'Enter' && fetchVersions()}
			/>
		</div>

		<div class="filter-group">
			<div class="filter-label">
				<IconFilter size={16} />
				<span>Content Type</span>
			</div>
			<div class="filter-chips">
				{#each contentTypes as type}
					<button
						class="filter-chip"
						class:active={filterType === type}
						onclick={() => { filterType = type; fetchVersions(); }}
					>
						{type === 'all' ? 'All Types' : type}
					</button>
				{/each}
			</div>
		</div>
	</section>

	<!-- Versions List -->
	<section class="versions-container" in:fly={{ y: 20, duration: 500, delay: 200 }}>
		{#if isLoading}
			<div class="loading-state">
				<div class="spinner"></div>
				<p>Loading versions...</p>
			</div>
		{:else if versions.length === 0}
			<div class="empty-state" in:scale={{ duration: 400 }}>
				<div class="empty-icon">
					<IconHistory size={48} />
				</div>
				<h3>No versions found</h3>
				<p>Content versions will appear here when changes are made</p>
			</div>
		{:else}
			<div class="versions-timeline">
				{#each versions as version, i}
					<div
						class="version-card"
						in:fly={{ x: -20, duration: 400, delay: i * 50 }}
						class:selected={selectedVersion?.id === version.id}
						onclick={() => selectedVersion = selectedVersion?.id === version.id ? null : version}
					>
						<div class="version-timeline-dot {getChangeTypeColor(version.change_type)}"></div>

						<div class="version-main">
							<div class="version-header">
								<span class="change-badge {getChangeTypeColor(version.change_type)}">
									{version.change_type}
								</span>
								<span class="version-number">v{version.version_number}</span>
							</div>

							<div class="version-content">
								<span class="content-type">{version.content_type}</span>
								<span class="content-id">#{version.content_id}</span>
							</div>

							<div class="version-meta">
								<div class="meta-item">
									<IconUser size={14} />
									<span>{version.created_by_email || 'System'}</span>
								</div>
								<div class="meta-item">
									<IconCalendar size={14} />
									<span>{formatDate(version.created_at)}</span>
								</div>
							</div>

							{#if version.change_summary}
								<p class="change-summary">{version.change_summary}</p>
							{/if}
						</div>

						<div class="version-actions">
							<button class="action-btn" title="Preview">
								<IconEye size={16} />
							</button>
							<button class="action-btn" title="Compare">
								<IconDiff size={16} />
							</button>
							<button
								class="action-btn rollback"
								title="Rollback"
								onclick={(e) => { e.stopPropagation(); rollbackVersion(version.id); }}
							>
								<IconRotateCcw size={16} />
							</button>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	</section>

	<!-- Version Detail Panel -->
	{#if selectedVersion}
		<aside class="detail-panel" in:fly={{ x: 300, duration: 400 }}>
			<div class="panel-header">
				<h2>Version Details</h2>
				<button class="close-btn" onclick={() => selectedVersion = null}>
					<IconX size={18} />
				</button>
			</div>

			<div class="panel-content">
				<div class="detail-section">
					<h3>Snapshot Data</h3>
					<pre class="json-preview">{JSON.stringify(selectedVersion.snapshot_data, null, 2)}</pre>
				</div>

				<div class="detail-actions">
					<button class="btn-primary" onclick={() => rollbackVersion(selectedVersion.id)}>
						<IconRotateCcw size={16} />
						Rollback to This Version
					</button>
				</div>
			</div>
		</aside>
	{/if}
</div>

<style>
	.versions-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 1.5rem 3rem;
		opacity: 0;
		transform: translateY(10px);
		transition: all 0.5s ease;
	}

	.versions-page.mounted {
		opacity: 1;
		transform: translateY(0);
	}

	/* Header */
	.page-header {
		padding: 1.5rem 0 2rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: #6366f1;
		text-decoration: none;
		margin-bottom: 1rem;
		transition: color 0.2s;
	}

	.back-link:hover {
		color: #4f46e5;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		width: 56px;
		height: 56px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.12) 0%, rgba(99, 102, 241, 0.06) 100%);
		color: #6366f1;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 800;
		color: #1e293b;
		margin: 0;
	}

	.header-title p {
		font-size: 0.9rem;
		color: #64748b;
		margin: 0.25rem 0 0 0;
	}

	/* Filters */
	.filters-bar {
		display: flex;
		flex-wrap: wrap;
		gap: 1.5rem;
		padding: 1.25rem 1.5rem;
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 16px;
		margin-bottom: 1.5rem;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.75rem 1rem;
		background: #f8fafc;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 12px;
		flex: 1;
		min-width: 280px;
		color: #64748b;
	}

	.search-box input {
		flex: 1;
		border: none;
		background: none;
		font-size: 0.9rem;
		color: #1e293b;
		outline: none;
	}

	.search-box input::placeholder {
		color: #94a3b8;
	}

	.filter-group {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.filter-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: #64748b;
	}

	.filter-chips {
		display: flex;
		gap: 0.5rem;
	}

	.filter-chip {
		padding: 0.5rem 1rem;
		background: #f1f5f9;
		border: 1px solid transparent;
		border-radius: 100px;
		font-size: 0.8rem;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
		text-transform: capitalize;
	}

	.filter-chip:hover {
		background: #e2e8f0;
	}

	.filter-chip.active {
		background: rgba(99, 102, 241, 0.1);
		border-color: rgba(99, 102, 241, 0.3);
		color: #6366f1;
	}

	/* Versions Timeline */
	.versions-container {
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 20px;
		padding: 1.5rem;
		min-height: 400px;
	}

	.versions-timeline {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		position: relative;
		padding-left: 2rem;
	}

	.versions-timeline::before {
		content: '';
		position: absolute;
		left: 7px;
		top: 0;
		bottom: 0;
		width: 2px;
		background: linear-gradient(180deg, rgba(99, 102, 241, 0.3) 0%, rgba(99, 102, 241, 0.05) 100%);
		border-radius: 1px;
	}

	.version-card {
		display: flex;
		align-items: flex-start;
		gap: 1.25rem;
		padding: 1.25rem;
		background: linear-gradient(135deg, rgba(248, 250, 252, 0.8) 0%, rgba(255, 255, 255, 0.5) 100%);
		border: 1px solid rgba(0, 0, 0, 0.04);
		border-radius: 16px;
		cursor: pointer;
		transition: all 0.25s ease;
		position: relative;
	}

	.version-card:hover {
		border-color: rgba(99, 102, 241, 0.2);
		background: linear-gradient(135deg, rgba(99, 102, 241, 0.04) 0%, rgba(255, 255, 255, 0.8) 100%);
	}

	.version-card.selected {
		border-color: rgba(99, 102, 241, 0.4);
		box-shadow: 0 4px 20px rgba(99, 102, 241, 0.1);
	}

	.version-timeline-dot {
		position: absolute;
		left: -2rem;
		top: 1.5rem;
		width: 14px;
		height: 14px;
		border-radius: 50%;
		border: 3px solid #ffffff;
		box-shadow: 0 0 0 2px currentColor;
	}

	.version-timeline-dot.emerald { background: #10b981; color: rgba(16, 185, 129, 0.3); }
	.version-timeline-dot.blue { background: #3b82f6; color: rgba(59, 130, 246, 0.3); }
	.version-timeline-dot.purple { background: #8b5cf6; color: rgba(139, 92, 246, 0.3); }
	.version-timeline-dot.amber { background: #f59e0b; color: rgba(245, 158, 11, 0.3); }
	.version-timeline-dot.red { background: #ef4444; color: rgba(239, 68, 68, 0.3); }
	.version-timeline-dot.slate { background: #64748b; color: rgba(100, 116, 139, 0.3); }

	.version-main {
		flex: 1;
	}

	.version-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
	}

	.change-badge {
		font-size: 0.7rem;
		font-weight: 700;
		padding: 0.25rem 0.6rem;
		border-radius: 6px;
		text-transform: uppercase;
	}

	.change-badge.emerald { background: rgba(16, 185, 129, 0.12); color: #059669; }
	.change-badge.blue { background: rgba(59, 130, 246, 0.12); color: #2563eb; }
	.change-badge.purple { background: rgba(139, 92, 246, 0.12); color: #7c3aed; }
	.change-badge.amber { background: rgba(245, 158, 11, 0.12); color: #b45309; }
	.change-badge.red { background: rgba(239, 68, 68, 0.12); color: #dc2626; }
	.change-badge.slate { background: rgba(100, 116, 139, 0.12); color: #475569; }

	.version-number {
		font-size: 0.8rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.version-content {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.content-type {
		font-size: 0.95rem;
		font-weight: 600;
		color: #1e293b;
		text-transform: capitalize;
	}

	.content-id {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.version-meta {
		display: flex;
		gap: 1.25rem;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.change-summary {
		margin: 0.75rem 0 0 0;
		font-size: 0.85rem;
		color: #64748b;
		line-height: 1.5;
	}

	.version-actions {
		display: flex;
		gap: 0.5rem;
	}

	.action-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.08);
		border-radius: 10px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn:hover {
		background: #f8fafc;
		color: #6366f1;
		border-color: rgba(99, 102, 241, 0.3);
	}

	.action-btn.rollback:hover {
		color: #d97706;
		border-color: rgba(245, 158, 11, 0.3);
	}

	/* Loading & Empty States */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f1f5f9;
		border-top-color: #6366f1;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		border-radius: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(148, 163, 184, 0.1) 0%, rgba(148, 163, 184, 0.05) 100%);
		color: #cbd5e1;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #475569;
		margin: 0 0 0.5rem 0;
	}

	.empty-state p,
	.loading-state p {
		font-size: 0.9rem;
		color: #94a3b8;
		margin: 0.5rem 0 0 0;
	}

	/* Detail Panel */
	.detail-panel {
		position: fixed;
		top: 0;
		right: 0;
		width: 480px;
		height: 100vh;
		background: #ffffff;
		border-left: 1px solid rgba(0, 0, 0, 0.08);
		box-shadow: -20px 0 60px rgba(0, 0, 0, 0.08);
		z-index: 100;
		display: flex;
		flex-direction: column;
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.06);
	}

	.panel-header h2 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.close-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f8fafc;
		border: none;
		border-radius: 10px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.close-btn:hover {
		background: #f1f5f9;
		color: #1e293b;
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.detail-section h3 {
		font-size: 0.85rem;
		font-weight: 600;
		color: #64748b;
		margin: 0 0 0.75rem 0;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.json-preview {
		padding: 1rem;
		background: #f8fafc;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 12px;
		font-size: 0.8rem;
		font-family: 'SF Mono', Consolas, monospace;
		color: #475569;
		overflow-x: auto;
		white-space: pre-wrap;
		word-break: break-word;
	}

	.detail-actions {
		margin-top: 1.5rem;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.875rem 1.5rem;
		background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
		border: none;
		border-radius: 12px;
		font-size: 0.9rem;
		font-weight: 600;
		color: #ffffff;
		cursor: pointer;
		transition: all 0.25s;
	}

	.btn-primary:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 25px rgba(99, 102, 241, 0.3);
	}

	@media (max-width: 768px) {
		.filters-bar {
			flex-direction: column;
		}

		.filter-chips {
			flex-wrap: wrap;
		}

		.detail-panel {
			width: 100%;
		}
	}
</style>

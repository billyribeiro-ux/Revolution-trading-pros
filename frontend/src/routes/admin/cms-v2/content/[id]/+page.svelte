<!--
	CMS v2 Edit Content
	═══════════════════════════════════════════════════════════════════════════════

	Content editing page with:
	- Load existing content
	- Rich text editor
	- Status workflow
	- Revision history
	- Publishing controls

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { fly, scale } from 'svelte/transition';
	import {
		cmsApi,
		type CmsContent,
		type CmsRevision,
		type UpdateContentRequest
	} from '$lib/api/cms-v2';
	import type { CmsContentStatus } from '$lib/page-builder/types';
	import MultiModeEditor from '$lib/components/cms/MultiModeEditor.svelte';
	import {
		IconArrowLeft,
		IconDeviceFloppy,
		IconSend,
		IconEye,
		IconSettings,
		IconSearch,
		IconPhoto,
		IconHistory,
		IconTrash,
		IconCheck,
		IconClock,
		IconArchive
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	// Content data
	let content = $state<CmsContent | null>(null);
	let revisions = $state<CmsRevision[]>([]);

	// Editable fields
	let title = $state('');
	let slug = $state('');
	let subtitle = $state('');
	let excerpt = $state('');
	let body = $state('');
	let bodyFormat = $state<'html' | 'markdown' | 'raw'>('html');
	let metaTitle = $state('');
	let metaDescription = $state('');
	let canonicalUrl = $state('');
	let template = $state('');

	// UI State
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state<string | null>(null);
	let successMessage = $state<string | null>(null);
	let activeTab = $state<'content' | 'seo' | 'settings' | 'revisions'>('content');
	let showStatusModal = $state(false);
	let pendingStatus = $state<CmsContentStatus | null>(null);

	// Derived
	let contentId = $derived($page.params.id);
	let hasChanges = $derived(checkForChanges());
	let statusIcon = $derived(content ? getStatusIcon(content.status) : IconClock);
	let statusColor = $derived(content ? getStatusColor(content.status) : 'gray');

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (browser && contentId) {
			loadContent();
			loadRevisions();
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Data Loading
	// ═══════════════════════════════════════════════════════════════════════════

	async function loadContent() {
		if (!contentId) return;
		isLoading = true;
		try {
			content = await cmsApi.getContent(contentId);
			populateFields(content);
		} catch (e: any) {
			error = e.message || 'Failed to load content';
		} finally {
			isLoading = false;
		}
	}

	async function loadRevisions() {
		if (!contentId) return;
		try {
			revisions = await cmsApi.getContentRevisions(contentId);
		} catch (e) {
			console.error('Failed to load revisions:', e);
		}
	}

	function populateFields(c: CmsContent) {
		title = c.title;
		slug = c.slug;
		subtitle = c.subtitle || '';
		excerpt = c.excerpt || '';
		body = c.content || '';
		bodyFormat = (c.content_format as 'html' | 'markdown' | 'raw') || 'html';
		metaTitle = c.meta_title || '';
		metaDescription = c.meta_description || '';
		canonicalUrl = c.canonical_url || '';
		template = c.template || '';
	}

	function checkForChanges(): boolean {
		if (!content) return false;
		return (
			title !== content.title ||
			slug !== content.slug ||
			subtitle !== (content.subtitle || '') ||
			excerpt !== (content.excerpt || '') ||
			body !== (content.content || '') ||
			metaTitle !== (content.meta_title || '') ||
			metaDescription !== (content.meta_description || '') ||
			canonicalUrl !== (content.canonical_url || '') ||
			template !== (content.template || '')
		);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Actions
	// ═══════════════════════════════════════════════════════════════════════════

	async function save() {
		if (!title.trim()) {
			error = 'Title is required';
			return;
		}

		if (!contentId) return;

		error = null;
		successMessage = null;
		isSaving = true;

		try {
			const request: UpdateContentRequest = {
				title: title.trim(),
				slug: slug.trim(),
				subtitle: subtitle.trim() || undefined,
				excerpt: excerpt.trim() || undefined,
				content: body.trim(),
				contentFormat: bodyFormat,
				metaTitle: metaTitle.trim() || undefined,
				metaDescription: metaDescription.trim() || undefined,
				canonicalUrl: canonicalUrl.trim() || undefined
			};

			content = await cmsApi.updateContent(contentId, request);
			await loadRevisions();
			successMessage = 'Content saved successfully';
			setTimeout(() => (successMessage = null), 3000);
		} catch (e: any) {
			error = e.message || 'Failed to save content';
		} finally {
			isSaving = false;
		}
	}

	async function changeStatus(newStatus: CmsContentStatus) {
		if (!contentId) return;
		try {
			content = await cmsApi.transitionContentStatus(contentId, { status: newStatus });
			showStatusModal = false;
			pendingStatus = null;
			successMessage = `Status changed to ${newStatus.replace('_', ' ')}`;
			setTimeout(() => (successMessage = null), 3000);
		} catch (e: any) {
			error = e.message || 'Failed to change status';
		}
	}

	async function restoreRevision(revisionNumber: number) {
		if (!contentId || !confirm('Restore this revision? Current changes will be lost.')) return;

		try {
			content = await cmsApi.restoreRevision(contentId, revisionNumber);
			populateFields(content);
			await loadRevisions();
			successMessage = 'Revision restored successfully';
			setTimeout(() => (successMessage = null), 3000);
		} catch (e: any) {
			error = e.message || 'Failed to restore revision';
		}
	}

	async function deleteContent() {
		if (!contentId || !confirm('Delete this content? This cannot be undone.')) return;

		try {
			await cmsApi.deleteContent(contentId);
			goto('/admin/cms-v2/content');
		} catch (e: any) {
			error = e.message || 'Failed to delete content';
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

	function getStatusColor(status: CmsContentStatus): string {
		switch (status) {
			case 'published':
				return 'green';
			case 'draft':
				return 'gray';
			case 'in_review':
				return 'yellow';
			case 'scheduled':
				return 'blue';
			case 'archived':
				return 'red';
			default:
				return 'gray';
		}
	}

	function getStatusIcon(status: CmsContentStatus) {
		switch (status) {
			case 'published':
				return IconCheck;
			case 'draft':
				return IconDeviceFloppy;
			case 'in_review':
				return IconClock;
			case 'scheduled':
				return IconClock;
			case 'archived':
				return IconArchive;
			default:
				return IconDeviceFloppy;
		}
	}

	function formatDate(dateString: string | null): string {
		if (!dateString) return '-';
		return new Date(dateString).toLocaleString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	const statusOptions: { value: CmsContentStatus; label: string }[] = [
		{ value: 'draft', label: 'Draft' },
		{ value: 'in_review', label: 'In Review' },
		{ value: 'approved', label: 'Approved' },
		{ value: 'published', label: 'Published' },
		{ value: 'archived', label: 'Archived' }
	];
</script>

<svelte:head>
	<title>{title || 'Edit Content'} | CMS v2</title>
</svelte:head>

<div class="edit-page">
	{#if isLoading}
		<div class="loading-state">
			<div class="spinner"></div>
			<p>Loading content...</p>
		</div>
	{:else if content}
		<!-- Header -->
		<header class="page-header" in:fly={{ y: -20, duration: 300 }}>
			<div class="header-left">
				<a href="/admin/cms-v2/content" class="btn-back">
					<IconArrowLeft size={18} />
				</a>
				<div class="header-content">
					<h1 class="page-title">{content.title}</h1>
					<p class="page-subtitle">
						{content.content_type.replace('_', ' ')} · Last updated {formatDate(content.updated_at)}
					</p>
				</div>
			</div>

			<div class="header-actions">
				<!-- Status Badge -->
				{#if statusIcon}
					{@const StatusIcon = statusIcon}
					<button
						class="status-badge status-{statusColor}"
						onclick={() => (showStatusModal = true)}
					>
						<StatusIcon size={14} />
						{content.status.replace('_', ' ')}
					</button>
				{/if}

				<button
					class="btn-secondary"
					onclick={save}
					disabled={isSaving || !hasChanges}
				>
					<IconDeviceFloppy size={18} />
					{isSaving ? 'Saving...' : 'Save'}
				</button>

				{#if content.status === 'draft'}
					<button
						class="btn-primary"
						onclick={() => changeStatus('published')}
					>
						<IconSend size={18} />
						Publish
					</button>
				{/if}
			</div>
		</header>

		{#if error}
			<div class="error-banner" in:scale={{ duration: 200 }}>
				<span>{error}</span>
				<button onclick={() => (error = null)}>Dismiss</button>
			</div>
		{/if}

		{#if successMessage}
			<div class="success-banner" in:scale={{ duration: 200 }}>
				<IconCheck size={18} />
				<span>{successMessage}</span>
			</div>
		{/if}

		<div class="editor-layout">
			<!-- Main Editor -->
			<main class="editor-main" in:fly={{ y: 20, duration: 300, delay: 50 }}>
				<!-- Tabs -->
				<div class="editor-tabs">
					<button
						class="tab-btn"
						class:active={activeTab === 'content'}
						onclick={() => (activeTab = 'content')}
					>
						Content
					</button>
					<button
						class="tab-btn"
						class:active={activeTab === 'seo'}
						onclick={() => (activeTab = 'seo')}
					>
						<IconSearch size={14} />
						SEO
					</button>
					<button
						class="tab-btn"
						class:active={activeTab === 'settings'}
						onclick={() => (activeTab = 'settings')}
					>
						<IconSettings size={14} />
						Settings
					</button>
					<button
						class="tab-btn"
						class:active={activeTab === 'revisions'}
						onclick={() => (activeTab = 'revisions')}
					>
						<IconHistory size={14} />
						Revisions
						{#if revisions.length > 0}
							<span class="tab-badge">{revisions.length}</span>
						{/if}
					</button>
				</div>

				<!-- Content Tab -->
				{#if activeTab === 'content'}
					<div class="tab-content" in:fly={{ x: -10, duration: 200 }}>
						<div class="field-group">
							<label for="title" class="field-label required">Title</label>
							<input
								id="title"
								type="text"
								bind:value={title}
								placeholder="Enter title..."
								class="field-input title-input"
							/>
						</div>

						<div class="field-group">
							<label for="slug" class="field-label required">Slug</label>
							<div class="slug-input-group">
								<span class="slug-prefix">/</span>
								<input
									id="slug"
									type="text"
									bind:value={slug}
									placeholder="url-friendly-slug"
									class="field-input"
								/>
							</div>
						</div>

						<div class="field-group">
							<label for="subtitle" class="field-label">Subtitle</label>
							<input
								id="subtitle"
								type="text"
								bind:value={subtitle}
								placeholder="Optional subtitle..."
								class="field-input"
							/>
						</div>

						<div class="field-group">
							<label for="content-editor" class="field-label">Content</label>
							<MultiModeEditor
								id="content-editor"
								value={body}
								format={bodyFormat}
								onchange={(v, f) => {
									body = v;
									bodyFormat = f;
								}}
								placeholder="Write your content..."
								minHeight="400px"
							/>
						</div>

						<div class="field-group">
							<label for="excerpt" class="field-label">Excerpt</label>
							<textarea
								id="excerpt"
								bind:value={excerpt}
								placeholder="Brief summary..."
								class="field-textarea"
								rows="3"
							></textarea>
						</div>
					</div>
				{/if}

				<!-- SEO Tab -->
				{#if activeTab === 'seo'}
					<div class="tab-content" in:fly={{ x: -10, duration: 200 }}>
						<div class="seo-preview">
							<h3 class="seo-preview-title">Search Preview</h3>
							<div class="seo-preview-result">
								<div class="seo-preview-url">
									example.com/{content.content_type}/{slug}
								</div>
								<div class="seo-preview-headline">
									{metaTitle || title || 'Page Title'}
								</div>
								<div class="seo-preview-desc">
									{metaDescription || excerpt || 'Page description...'}
								</div>
							</div>
						</div>

						<div class="field-group">
							<label for="meta-title" class="field-label">Meta Title</label>
							<input
								id="meta-title"
								type="text"
								bind:value={metaTitle}
								placeholder={title}
								class="field-input"
							/>
							<span class="field-hint">{(metaTitle || title).length}/60 characters</span>
						</div>

						<div class="field-group">
							<label for="meta-desc" class="field-label">Meta Description</label>
							<textarea
								id="meta-desc"
								bind:value={metaDescription}
								placeholder="Enter description..."
								class="field-textarea"
								rows="3"
							></textarea>
							<span class="field-hint">{metaDescription.length}/160 characters</span>
						</div>

						<div class="field-group">
							<label for="canonical" class="field-label">Canonical URL</label>
							<input
								id="canonical"
								type="url"
								bind:value={canonicalUrl}
								placeholder="https://..."
								class="field-input"
							/>
						</div>
					</div>
				{/if}

				<!-- Settings Tab -->
				{#if activeTab === 'settings'}
					<div class="tab-content" in:fly={{ x: -10, duration: 200 }}>
						<div class="field-group">
							<span class="field-label">Content Type</span>
							<div class="field-value">{content.content_type.replace('_', ' ')}</div>
						</div>

						<div class="field-group">
							<label for="template" class="field-label">Template</label>
							<select id="template" bind:value={template} class="field-select">
								<option value="">Default Template</option>
								<option value="full-width">Full Width</option>
								<option value="sidebar">With Sidebar</option>
								<option value="landing">Landing Page</option>
							</select>
						</div>

						<div class="field-group">
							<span class="field-label">Featured Image</span>
							<button type="button" class="btn-select-image" aria-label="Select featured image">
								<IconPhoto size={20} />
								<span>Select Image</span>
							</button>
						</div>

						<hr class="field-divider" />

						<div class="danger-zone">
							<h3 class="danger-title">Danger Zone</h3>
							<button class="btn-danger" onclick={deleteContent}>
								<IconTrash size={16} />
								Delete Content
							</button>
						</div>
					</div>
				{/if}

				<!-- Revisions Tab -->
				{#if activeTab === 'revisions'}
					<div class="tab-content" in:fly={{ x: -10, duration: 200 }}>
						{#if revisions.length === 0}
							<div class="empty-revisions">
								<IconHistory size={48} />
								<h3>No Revisions</h3>
								<p>Revisions will appear here after saving changes.</p>
							</div>
						{:else}
							<div class="revisions-list">
								{#each revisions as revision (revision.id)}
									<div class="revision-item" class:current={revision.isCurrent}>
										<div class="revision-info">
											<span class="revision-number">
												v{revision.revisionNumber}
												{#if revision.isCurrent}
													<span class="current-badge">Current</span>
												{/if}
											</span>
											<span class="revision-date">
												{formatDate(revision.createdAt)}
											</span>
											{#if revision.changeSummary}
												<span class="revision-summary">{revision.changeSummary}</span>
											{/if}
										</div>
										{#if !revision.isCurrent}
											<button
												class="btn-restore"
												onclick={() => restoreRevision(revision.revisionNumber)}
											>
												Restore
											</button>
										{/if}
									</div>
								{/each}
							</div>
						{/if}
					</div>
				{/if}
			</main>

			<!-- Sidebar -->
			<aside class="editor-sidebar" in:fly={{ x: 20, duration: 300, delay: 100 }}>
				<div class="sidebar-card">
					<h3 class="sidebar-title">Publish</h3>
					<div class="publish-info">
						<div class="info-row">
							<span class="info-label">Status</span>
							<span class="info-value status-{getStatusColor(content.status)}">
								{content.status.replace('_', ' ')}
							</span>
						</div>
						<div class="info-row">
							<span class="info-label">Created</span>
							<span class="info-value">{formatDate(content.created_at)}</span>
						</div>
						{#if content.published_at}
							<div class="info-row">
								<span class="info-label">Published</span>
								<span class="info-value">{formatDate(content.published_at)}</span>
							</div>
						{/if}
						<div class="info-row">
							<span class="info-label">Version</span>
							<span class="info-value">v{content.version}</span>
						</div>
					</div>
				</div>

				<div class="sidebar-card">
					<h3 class="sidebar-title">Actions</h3>
					<div class="action-buttons">
						<button class="btn-action-full" onclick={save} disabled={!hasChanges}>
							<IconDeviceFloppy size={16} />
							Save Changes
						</button>
						<a
							href="/preview/{content.id}"
							target="_blank"
							class="btn-action-full secondary"
						>
							<IconEye size={16} />
							Preview
						</a>
					</div>
				</div>
			</aside>
		</div>
	{/if}
</div>

<!-- Status Modal -->
{#if showStatusModal && content}
	<div
		class="modal-backdrop"
		role="presentation"
		onclick={() => (showStatusModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showStatusModal = false)}
	>
		<div
			class="modal"
			role="dialog"
			aria-modal="true"
			aria-labelledby="status-modal-title"
			tabindex="-1"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.key === 'Escape' && (showStatusModal = false)}
			in:scale={{ duration: 200 }}
		>
			<div class="modal-header">
				<h3 id="status-modal-title">Change Status</h3>
				<button class="btn-close" onclick={() => (showStatusModal = false)}>×</button>
			</div>
			<div class="modal-body">
				<p class="modal-text">Current status: <strong>{content.status.replace('_', ' ')}</strong></p>
				<div class="status-options">
					{#each statusOptions as option}
						<button
							class="status-option"
							class:active={option.value === content.status}
							onclick={() => changeStatus(option.value)}
							disabled={option.value === content.status}
						>
							{option.label}
						</button>
					{/each}
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.edit-page {
		padding: 1.5rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	/* Loading */
	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		color: #64748b;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid rgba(230, 184, 0, 0.2);
		border-top-color: #e6b800;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Header */
	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1.5rem;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.btn-back {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		text-decoration: none;
		transition: all 0.15s;
	}

	.btn-back:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.page-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0;
	}

	.page-subtitle {
		font-size: 0.875rem;
		color: #64748b;
		margin: 0.25rem 0 0 0;
		text-transform: capitalize;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.status-badge {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		border: none;
		border-radius: 0.5rem;
		font-size: 0.8125rem;
		font-weight: 600;
		text-transform: capitalize;
		cursor: pointer;
		transition: all 0.15s;
	}

	.status-badge.status-green {
		background: rgba(16, 185, 129, 0.15);
		color: #34d399;
	}

	.status-badge.status-gray {
		background: rgba(100, 116, 139, 0.15);
		color: #94a3b8;
	}

	.status-badge.status-yellow {
		background: rgba(245, 158, 11, 0.15);
		color: #fbbf24;
	}

	.status-badge.status-blue {
		background: rgba(59, 130, 246, 0.15);
		color: #60a5fa;
	}

	.status-badge.status-red {
		background: rgba(239, 68, 68, 0.15);
		color: #f87171;
	}

	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-weight: 500;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-secondary:hover:not(:disabled) {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.btn-secondary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-primary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1rem;
		background: linear-gradient(135deg, #e6b800, #d4a600);
		border: none;
		border-radius: 0.5rem;
		color: #0f172a;
		font-weight: 600;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	/* Banners */
	.error-banner,
	.success-banner {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-radius: 0.5rem;
		margin-bottom: 1.5rem;
	}

	.error-banner {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.success-banner {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.error-banner button {
		margin-left: auto;
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.375rem;
		color: #f87171;
		font-size: 0.8125rem;
		cursor: pointer;
	}

	/* Layout */
	.editor-layout {
		display: grid;
		grid-template-columns: 1fr 300px;
		gap: 1.5rem;
	}

	/* Editor */
	.editor-main {
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		overflow: hidden;
	}

	.editor-tabs {
		display: flex;
		gap: 0.25rem;
		padding: 0.5rem;
		background: rgba(0, 0, 0, 0.2);
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.tab-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tab-btn:hover {
		background: rgba(51, 65, 85, 0.3);
		color: #f1f5f9;
	}

	.tab-btn.active {
		background: rgba(230, 184, 0, 0.15);
		color: #e6b800;
	}

	.tab-badge {
		padding: 0.125rem 0.375rem;
		background: rgba(230, 184, 0, 0.2);
		border-radius: 0.25rem;
		font-size: 0.6875rem;
		font-weight: 700;
	}

	.tab-content {
		padding: 1.5rem;
	}

	/* Fields */
	.field-group {
		margin-bottom: 1.5rem;
	}

	.field-label {
		display: block;
		font-size: 0.8125rem;
		font-weight: 600;
		color: #cbd5e1;
		margin-bottom: 0.5rem;
	}

	.field-label.required::after {
		content: ' *';
		color: #ef4444;
	}

	.field-input,
	.field-textarea,
	.field-select {
		width: 100%;
		padding: 0.625rem 0.875rem;
		background: rgba(0, 0, 0, 0.2);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #f1f5f9;
		font-size: 0.9375rem;
		transition: border-color 0.15s;
	}

	.field-input:focus,
	.field-textarea:focus {
		outline: none;
		border-color: #e6b800;
	}

	.title-input {
		font-size: 1.5rem;
		font-weight: 600;
	}

	.slug-input-group {
		display: flex;
		align-items: center;
	}

	.slug-prefix {
		padding: 0.625rem 0.75rem;
		background: rgba(0, 0, 0, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-right: none;
		border-radius: 0.5rem 0 0 0.5rem;
		color: #64748b;
	}

	.slug-input-group .field-input {
		border-radius: 0 0.5rem 0.5rem 0;
	}

	.field-hint {
		display: block;
		font-size: 0.75rem;
		color: #64748b;
		margin-top: 0.375rem;
	}

	.field-value {
		padding: 0.625rem 0;
		color: #f1f5f9;
		text-transform: capitalize;
	}

	.field-divider {
		border: none;
		border-top: 1px solid rgba(51, 65, 85, 0.5);
		margin: 2rem 0;
	}

	.btn-select-image {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 2rem;
		background: rgba(0, 0, 0, 0.2);
		border: 2px dashed rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-select-image:hover {
		border-color: #e6b800;
		color: #f1f5f9;
	}

	/* SEO Preview */
	.seo-preview {
		margin-bottom: 1.5rem;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.5rem;
	}

	.seo-preview-title {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		margin: 0 0 0.75rem 0;
		text-transform: uppercase;
	}

	.seo-preview-result {
		font-family: Arial, sans-serif;
	}

	.seo-preview-url {
		font-size: 0.8125rem;
		color: #059669;
		margin-bottom: 0.25rem;
	}

	.seo-preview-headline {
		font-size: 1.125rem;
		color: #1e40af;
		margin-bottom: 0.25rem;
	}

	.seo-preview-desc {
		font-size: 0.875rem;
		color: #4b5563;
		line-height: 1.4;
	}

	/* Danger Zone */
	.danger-zone {
		padding: 1rem;
		background: rgba(239, 68, 68, 0.05);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 0.5rem;
	}

	.danger-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f87171;
		margin: 0 0 0.75rem 0;
	}

	.btn-danger {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.5rem;
		color: #f87171;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-danger:hover {
		background: rgba(239, 68, 68, 0.2);
	}

	/* Revisions */
	.empty-revisions {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem;
		color: #64748b;
		text-align: center;
	}

	.empty-revisions h3 {
		font-size: 1rem;
		color: #f1f5f9;
		margin: 1rem 0 0.25rem 0;
	}

	.revisions-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.revision-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		background: rgba(0, 0, 0, 0.2);
		border-radius: 0.5rem;
	}

	.revision-item.current {
		border: 1px solid rgba(230, 184, 0, 0.3);
	}

	.revision-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.revision-number {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 600;
		color: #f1f5f9;
	}

	.current-badge {
		padding: 0.125rem 0.375rem;
		background: rgba(230, 184, 0, 0.2);
		border-radius: 0.25rem;
		font-size: 0.6875rem;
		color: #e6b800;
	}

	.revision-date {
		font-size: 0.75rem;
		color: #64748b;
	}

	.revision-summary {
		font-size: 0.8125rem;
		color: #94a3b8;
	}

	.btn-restore {
		padding: 0.375rem 0.75rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.375rem;
		color: #94a3b8;
		font-size: 0.8125rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-restore:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	/* Sidebar */
	.editor-sidebar {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.sidebar-card {
		background: rgba(30, 41, 59, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		padding: 1rem;
	}

	.sidebar-title {
		font-size: 0.875rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0 0 0.75rem 0;
	}

	.publish-info {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.info-row {
		display: flex;
		justify-content: space-between;
		font-size: 0.8125rem;
	}

	.info-label {
		color: #94a3b8;
	}

	.info-value {
		color: #f1f5f9;
		text-transform: capitalize;
	}

	.action-buttons {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.btn-action-full {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		width: 100%;
		padding: 0.625rem 1rem;
		background: rgba(230, 184, 0, 0.15);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 0.5rem;
		color: #e6b800;
		font-weight: 500;
		font-size: 0.875rem;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-action-full:hover:not(:disabled) {
		background: rgba(230, 184, 0, 0.25);
	}

	.btn-action-full:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn-action-full.secondary {
		background: rgba(51, 65, 85, 0.3);
		border-color: rgba(51, 65, 85, 0.5);
		color: #94a3b8;
	}

	.btn-action-full.secondary:hover {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	/* Modal */
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(4px);
	}

	.modal {
		width: 100%;
		max-width: 400px;
		background: #1e293b;
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.75rem;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(51, 65, 85, 0.5);
	}

	.modal-header h3 {
		font-size: 1.0625rem;
		font-weight: 600;
		color: #f1f5f9;
		margin: 0;
	}

	.btn-close {
		background: transparent;
		border: none;
		color: #64748b;
		font-size: 1.5rem;
		cursor: pointer;
	}

	.modal-body {
		padding: 1.25rem;
	}

	.modal-text {
		font-size: 0.9375rem;
		color: #94a3b8;
		margin: 0 0 1rem 0;
	}

	.modal-text strong {
		color: #f1f5f9;
		text-transform: capitalize;
	}

	.status-options {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.status-option {
		padding: 0.5rem 1rem;
		background: rgba(51, 65, 85, 0.3);
		border: 1px solid rgba(51, 65, 85, 0.5);
		border-radius: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		cursor: pointer;
		transition: all 0.15s;
	}

	.status-option:hover:not(:disabled) {
		background: rgba(51, 65, 85, 0.5);
		color: #f1f5f9;
	}

	.status-option.active {
		background: rgba(230, 184, 0, 0.15);
		border-color: rgba(230, 184, 0, 0.3);
		color: #e6b800;
	}

	.status-option:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Responsive */
	@media (max-width: 1024px) {
		.editor-layout {
			grid-template-columns: 1fr;
		}

		.editor-sidebar {
			order: -1;
			flex-direction: row;
			flex-wrap: wrap;
		}

		.sidebar-card {
			flex: 1 1 auto;
			min-width: 200px;
		}
	}

	@media (max-width: 768px) {
		.edit-page {
			padding: 1rem;
		}

		.header-actions {
			flex-wrap: wrap;
		}
	}
</style>

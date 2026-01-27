<!--
	CMS v2 Create Content
	═══════════════════════════════════════════════════════════════════════════════

	Content creation page with:
	- Rich text editor
	- Content type selection
	- SEO metadata
	- Publishing options
	- Save as draft or publish

	@version 1.0.0
	@author Revolution Trading Pros
	@since January 2026
-->

<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { fly, scale } from 'svelte/transition';
	import { cmsApi, type CreateContentRequest } from '$lib/api/cms-v2';
	import type { CmsContentType, CmsContentStatus } from '$lib/page-builder/types';
	import RichTextEditor from '$lib/components/cms/RichTextEditor.svelte';
	import {
		IconArrowLeft,
		IconDeviceFloppy,
		IconSend,
		IconEye,
		IconSettings,
		IconSearch,
		IconPhoto
	} from '$lib/icons';

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	// Content data
	let contentType = $state<CmsContentType>('page');
	let title = $state('');
	let slug = $state('');
	let subtitle = $state('');
	let excerpt = $state('');
	let content = $state('');

	// SEO
	let metaTitle = $state('');
	let metaDescription = $state('');
	let canonicalUrl = $state('');

	// Settings
	let template = $state('');
	let featuredImageId = $state('');

	// UI State
	let isSaving = $state(false);
	let isPublishing = $state(false);
	let error = $state<string | null>(null);
	let activeTab = $state<'content' | 'seo' | 'settings'>('content');
	let autoSlug = $state(true);

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		// Auto-generate slug from title
		if (autoSlug && title) {
			slug = title
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
	});

	$effect(() => {
		// Check for content type in URL
		const typeParam = $page.url.searchParams.get('type');
		if (typeParam && isValidContentType(typeParam)) {
			contentType = typeParam as CmsContentType;
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Actions
	// ═══════════════════════════════════════════════════════════════════════════

	async function save(publish: boolean = false) {
		if (!title.trim()) {
			error = 'Title is required';
			return;
		}

		if (!slug.trim()) {
			error = 'Slug is required';
			return;
		}

		error = null;
		isSaving = !publish;
		isPublishing = publish;

		try {
			const request: CreateContentRequest = {
				contentType,
				slug: slug.trim(),
				title: title.trim(),
				subtitle: subtitle.trim() || undefined,
				excerpt: excerpt.trim() || undefined,
				content: content || undefined,
				metaTitle: metaTitle.trim() || undefined,
				metaDescription: metaDescription.trim() || undefined,
				canonicalUrl: canonicalUrl.trim() || undefined,
				template: template || undefined,
				featuredImageId: featuredImageId || undefined
			};

			const created = await cmsApi.createContent(request);

			// If publishing, transition status
			if (publish) {
				await cmsApi.transitionContentStatus(created.id, {
					status: 'published'
				});
			}

			goto(`/admin/cms-v2/content/${created.id}`);
		} catch (e: any) {
			error = e.message || 'Failed to save content';
		} finally {
			isSaving = false;
			isPublishing = false;
		}
	}

	function isValidContentType(type: string): boolean {
		const validTypes: CmsContentType[] = [
			'page',
			'blog_post',
			'alert_service',
			'trading_room',
			'indicator',
			'course',
			'lesson',
			'testimonial',
			'faq',
			'author',
			'topic_cluster',
			'weekly_watchlist',
			'resource'
		];
		return validTypes.includes(type as CmsContentType);
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helpers
	// ═══════════════════════════════════════════════════════════════════════════

	const contentTypes: { value: CmsContentType; label: string; description: string }[] = [
		{ value: 'page', label: 'Page', description: 'Static page (About, Contact, etc.)' },
		{ value: 'blog_post', label: 'Blog Post', description: 'Blog article or news' },
		{ value: 'alert_service', label: 'Alert Service', description: 'Trading alert service' },
		{ value: 'trading_room', label: 'Trading Room', description: 'Live trading room' },
		{ value: 'indicator', label: 'Indicator', description: 'Trading indicator product' },
		{ value: 'course', label: 'Course', description: 'Educational course' },
		{ value: 'lesson', label: 'Lesson', description: 'Course lesson' },
		{ value: 'testimonial', label: 'Testimonial', description: 'Customer testimonial' },
		{ value: 'faq', label: 'FAQ', description: 'Frequently asked question' }
	];
</script>

<svelte:head>
	<title>Create Content | CMS v2</title>
</svelte:head>

<div class="create-page">
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 300 }}>
		<div class="header-left">
			<a href="/admin/cms-v2/content" class="btn-back">
				<IconArrowLeft size={18} />
			</a>
			<div class="header-content">
				<h1 class="page-title">Create Content</h1>
				<p class="page-subtitle">New {contentType.replace('_', ' ')}</p>
			</div>
		</div>

		<div class="header-actions">
			<button
				class="btn-secondary"
				onclick={() => save(false)}
				disabled={isSaving || isPublishing}
			>
				<IconDeviceFloppy size={18} />
				{isSaving ? 'Saving...' : 'Save Draft'}
			</button>
			<button
				class="btn-primary"
				onclick={() => save(true)}
				disabled={isSaving || isPublishing}
			>
				<IconSend size={18} />
				{isPublishing ? 'Publishing...' : 'Publish'}
			</button>
		</div>
	</header>

	{#if error}
		<div class="error-banner" in:scale={{ duration: 200 }}>
			<span>{error}</span>
			<button onclick={() => (error = null)}>Dismiss</button>
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
			</div>

			<!-- Content Tab -->
			{#if activeTab === 'content'}
				<div class="tab-content" in:fly={{ x: -10, duration: 200 }}>
					<!-- Title -->
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

					<!-- Slug -->
					<div class="field-group">
						<div class="field-label-row">
							<label for="slug" class="field-label required">Slug</label>
							<label class="auto-slug-toggle">
								<input type="checkbox" bind:checked={autoSlug} />
								Auto-generate
							</label>
						</div>
						<div class="slug-input-group">
							<span class="slug-prefix">/</span>
							<input
								id="slug"
								type="text"
								bind:value={slug}
								placeholder="url-friendly-slug"
								class="field-input"
								disabled={autoSlug}
							/>
						</div>
					</div>

					<!-- Subtitle -->
					<div class="field-group">
						<label for="subtitle" class="field-label">Subtitle</label>
						<input
							id="subtitle"
							type="text"
							bind:value={subtitle}
							placeholder="Optional subtitle or tagline..."
							class="field-input"
						/>
					</div>

					<!-- Content -->
					<div class="field-group">
						<label class="field-label">Content</label>
						<RichTextEditor
							value={content}
							onchange={(v) => (content = v)}
							placeholder="Write your content..."
							minHeight="400px"
						/>
					</div>

					<!-- Excerpt -->
					<div class="field-group">
						<label for="excerpt" class="field-label">Excerpt</label>
						<textarea
							id="excerpt"
							bind:value={excerpt}
							placeholder="Brief summary for listings and search results..."
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
								example.com/{contentType}/{slug || 'url-slug'}
							</div>
							<div class="seo-preview-headline">
								{metaTitle || title || 'Page Title'}
							</div>
							<div class="seo-preview-desc">
								{metaDescription || excerpt || 'Page description will appear here...'}
							</div>
						</div>
					</div>

					<div class="field-group">
						<label for="meta-title" class="field-label">Meta Title</label>
						<input
							id="meta-title"
							type="text"
							bind:value={metaTitle}
							placeholder={title || 'Leave blank to use page title'}
							class="field-input"
						/>
						<span class="field-hint">{(metaTitle || title).length}/60 characters</span>
					</div>

					<div class="field-group">
						<label for="meta-desc" class="field-label">Meta Description</label>
						<textarea
							id="meta-desc"
							bind:value={metaDescription}
							placeholder="Enter a description for search engines..."
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
							placeholder="https://example.com/canonical-page"
							class="field-input"
						/>
						<span class="field-hint">
							Leave blank to use default URL. Use if content appears elsewhere.
						</span>
					</div>
				</div>
			{/if}

			<!-- Settings Tab -->
			{#if activeTab === 'settings'}
				<div class="tab-content" in:fly={{ x: -10, duration: 200 }}>
					<div class="field-group">
						<label for="content-type" class="field-label required">Content Type</label>
						<select id="content-type" bind:value={contentType} class="field-select">
							{#each contentTypes as type}
								<option value={type.value}>{type.label}</option>
							{/each}
						</select>
						<span class="field-hint">
							{contentTypes.find((t) => t.value === contentType)?.description}
						</span>
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
						<label class="field-label">Featured Image</label>
						<button type="button" class="btn-select-image">
							<IconPhoto size={20} />
							<span>Select Image</span>
						</button>
						<span class="field-hint">
							Recommended size: 1200x630 pixels for social sharing
						</span>
					</div>
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
						<span class="info-value status-draft">Draft</span>
					</div>
					<div class="info-row">
						<span class="info-label">Visibility</span>
						<span class="info-value">Public</span>
					</div>
				</div>
			</div>

			<div class="sidebar-card">
				<h3 class="sidebar-title">Actions</h3>
				<div class="action-buttons">
					<button class="btn-action-full" onclick={() => save(false)}>
						<IconDeviceFloppy size={16} />
						Save Draft
					</button>
					<a href="/admin/cms-v2/content" class="btn-action-full secondary">
						<IconArrowLeft size={16} />
						Discard
					</a>
				</div>
			</div>
		</aside>
	</div>
</div>

<style>
	.create-page {
		padding: 1.5rem;
		max-width: 1400px;
		margin: 0 auto;
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

	.btn-primary:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.3);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Error Banner */
	.error-banner {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem 1.25rem;
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.5rem;
		color: #f87171;
		margin-bottom: 1.5rem;
	}

	.error-banner button {
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

	/* Main Editor */
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

	.field-label-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.5rem;
	}

	.auto-slug-toggle {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: #64748b;
		cursor: pointer;
	}

	.auto-slug-toggle input {
		accent-color: #e6b800;
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
	.field-textarea:focus,
	.field-select:focus {
		outline: none;
		border-color: #e6b800;
	}

	.field-input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.field-textarea {
		resize: vertical;
	}

	.field-select {
		cursor: pointer;
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
		font-size: 0.9375rem;
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
		font-size: 0.875rem;
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
		cursor: pointer;
	}

	.seo-preview-headline:hover {
		text-decoration: underline;
	}

	.seo-preview-desc {
		font-size: 0.875rem;
		color: #4b5563;
		line-height: 1.4;
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
	}

	.status-draft {
		color: #fbbf24;
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

	.btn-action-full:hover {
		background: rgba(230, 184, 0, 0.25);
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
		.create-page {
			padding: 1rem;
		}

		.header-actions {
			flex-direction: column;
			gap: 0.5rem;
		}

		.tab-content {
			padding: 1rem;
		}
	}
</style>

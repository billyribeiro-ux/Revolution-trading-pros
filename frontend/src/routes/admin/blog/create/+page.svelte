<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import IconDeviceFloppy from '@tabler/icons-svelte/icons/device-floppy';
	import IconEye from '@tabler/icons-svelte/icons/eye';
	import IconPhoto from '@tabler/icons-svelte/icons/photo';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconPlus from '@tabler/icons-svelte/icons/plus';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconLoader from '@tabler/icons-svelte/icons/loader';
	import IconCheck from '@tabler/icons-svelte/icons/check';
	import IconTags from '@tabler/icons-svelte/icons/tags';
	import RichTextEditor from '$lib/components/blog/RichTextEditor.svelte';
	import SeoMetaFields from '$lib/components/blog/SeoMetaFields.svelte';
	import { api } from '$lib/api/config';
	import { mediaApi } from '$lib/api/media';

	// ═══════════════════════════════════════════════════════════════════════════
	// PREDEFINED BLOG CATEGORIES (same system as videos)
	// ═══════════════════════════════════════════════════════════════════════════

	interface BlogCategory {
		id: string;
		name: string;
		color: string;
	}

	const predefinedCategories: BlogCategory[] = [
		{ id: 'market-analysis', name: 'Market Analysis', color: '#3b82f6' },
		{ id: 'trading-strategies', name: 'Trading Strategies', color: '#10b981' },
		{ id: 'risk-management', name: 'Risk Management', color: '#ef4444' },
		{ id: 'options-trading', name: 'Options Trading', color: '#f59e0b' },
		{ id: 'technical-analysis', name: 'Technical Analysis', color: '#6366f1' },
		{ id: 'fundamental-analysis', name: 'Fundamental Analysis', color: '#ec4899' },
		{ id: 'psychology', name: 'Psychology', color: '#8b5cf6' },
		{ id: 'education', name: 'Education', color: '#14b8a6' },
		{ id: 'news', name: 'News & Updates', color: '#06b6d4' },
		{ id: 'earnings', name: 'Earnings', color: '#f97316' },
		{ id: 'stocks', name: 'Stocks', color: '#84cc16' },
		{ id: 'futures', name: 'Futures', color: '#22c55e' },
		{ id: 'forex', name: 'Forex', color: '#0ea5e9' },
		{ id: 'crypto', name: 'Crypto', color: '#a855f7' },
		{ id: 'small-accounts', name: 'Small Accounts', color: '#eab308' },
		{ id: 'day-trading', name: 'Day Trading', color: '#d946ef' },
		{ id: 'swing-trading', name: 'Swing Trading', color: '#64748b' },
		{ id: 'beginners', name: 'Beginners Guide', color: '#fb7185' }
	];

	function getPredefinedCategoryById(id: string): BlogCategory | undefined {
		return predefinedCategories.find(c => c.id === id);
	}

	let post = $state({
		title: '',
		slug: '',
		excerpt: '',
		content_blocks: [] as any[],
		featured_image: '',
		featured_image_alt: '',
		featured_image_title: '',
		featured_image_caption: '',
		featured_image_description: '',
		featured_media_id: null as number | null,
		status: 'draft',
		published_at: '',
		allow_comments: true,
		meta_title: '',
		meta_description: '',
		meta_keywords: [] as string[],
		indexable: true,
		canonical_url: '',
		categories: [] as string[], // Using string IDs for predefined categories
		tags: [] as number[]
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// CATEGORY SELECTION FUNCTIONS
	// ═══════════════════════════════════════════════════════════════════════════

	function toggleCategorySelection(categoryId: string) {
		const index = post.categories.indexOf(categoryId);
		if (index === -1) {
			post.categories = [...post.categories, categoryId];
		} else {
			post.categories = post.categories.filter(c => c !== categoryId);
		}
	}

	function isCategorySelected(categoryId: string): boolean {
		return post.categories.includes(categoryId);
	}

	let content = $state('');
	let categories: any[] = $state([]);
	let tags: any[] = $state([]);
	let availableCategories: any[] = $state([]);
	let availableTags: any[] = $state([]);
	let saving = $state(false);
	let showFeaturedImage = $state(false);
	let newTag = $state('');
	let showSeoPanel = $state(false);
	let uploadingImage = $state(false);
	let uploadError = $state('');

	onMount(() => {
		loadCategories();
		loadTags();
		generateSlug();
	});

	async function loadCategories() {
		try {
			const data = await api.get('/api/admin/categories');
			availableCategories = data.data || data || [];
		} catch (error) {
			console.error('Failed to load categories:', error);
		}
	}

	async function loadTags() {
		try {
			const data = await api.get('/api/admin/tags');
			availableTags = data.data || data || [];
		} catch (error) {
			console.error('Failed to load tags:', error);
		}
	}

	function generateSlug() {
		if (!post.slug && post.title) {
			post.slug = post.title
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
	}

	function handleContentChange(newContent: string) {
		content = newContent;
	}

	async function createTag() {
		if (!newTag.trim()) return;

		try {
			const data = await api.post('/api/admin/tags', { name: newTag.trim() });
			const newTagData = data.data || data;
			availableTags = [...availableTags, newTagData];
			post.tags = [...post.tags, newTagData.id];
			newTag = '';
		} catch (error) {
			console.error('Failed to create tag:', error);
		}
	}

	async function savePost(status: string) {
		saving = true;

		try {
			const postData = {
				...post,
				status,
				content_blocks: [{ type: 'html', content }],
				published_at:
					status === 'published' && !post.published_at
						? new Date().toISOString()
						: post.published_at || null
			};

			await api.post('/api/admin/posts', postData);
			goto('/admin/blog');
		} catch (error) {
			console.error('Failed to save post:', error);
			alert('Failed to save post');
		} finally {
			saving = false;
		}
	}

	async function handleFeaturedImageUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files[0]) {
			const file = input.files[0];
			uploadingImage = true;
			uploadError = '';

			try {
				// Upload to server via mediaApi
				const result = await mediaApi.uploadFile(file, {
					title: post.featured_image_title || file.name.replace(/\.[^/.]+$/, ''),
					alt_text: post.featured_image_alt || '',
					optimize: true,
					generate_webp: true
				});

				// Store the uploaded media details
				post.featured_image = result.file.url;
				post.featured_media_id = parseInt(result.file.id);

				// Pre-fill title from filename if not set
				if (!post.featured_image_title) {
					post.featured_image_title = result.file.title || file.name.replace(/\.[^/.]+$/, '');
				}

				showFeaturedImage = true;
			} catch (error: any) {
				console.error('Failed to upload featured image:', error);
				uploadError = error.message || 'Failed to upload image';
			} finally {
				uploadingImage = false;
			}
		}
	}

	function removeFeaturedImage() {
		post.featured_image = '';
		post.featured_image_alt = '';
		post.featured_image_title = '';
		post.featured_image_caption = '';
		post.featured_image_description = '';
		post.featured_media_id = null;
		showFeaturedImage = false;
	}

	$effect(() => {
		if (post.title) {
			generateSlug();
		}
	});
</script>

<svelte:head>
	<title>Create Post | Blog</title>
</svelte:head>

<div class="post-editor">
	<div class="editor-header">
		<div class="header-left">
			<h1>Create New Post</h1>
		</div>
		<div class="header-actions">
			<button class="btn-secondary" onclick={() => goto('/admin/blog')}> Cancel </button>
			<button class="btn-secondary" onclick={() => savePost('draft')} disabled={saving}>
				<IconDeviceFloppy size={18} />
				Save Draft
			</button>
			<button class="btn-primary" onclick={() => savePost('published')} disabled={saving}>
				<IconEye size={18} />
				{saving ? 'Publishing...' : 'Publish'}
			</button>
		</div>
	</div>

	<div class="editor-layout">
		<!-- Main Content Area -->
		<div class="editor-main">
			<div class="form-group">
				<input type="text" bind:value={post.title} placeholder="Post Title" class="title-input" />
			</div>

			<div class="form-group">
				<label for="slug">URL Slug</label>
				<div class="slug-input">
					<span class="slug-prefix">your-site.com/blog/</span>
					<input id="slug" type="text" bind:value={post.slug} placeholder="post-url-slug" />
				</div>
			</div>

			<div class="form-group">
				<label for="blog-excerpt">Excerpt</label>
				<textarea
					id="excerpt"
					bind:value={post.excerpt}
					placeholder="Brief description of the post..."
					rows="3"
				></textarea>
			</div>

			<div class="form-group">
				<label for="blog-content">Content</label>
				<RichTextEditor bind:content onchange={handleContentChange} />
			</div>

			<!-- SEO Section -->
			<div class="panel">
				<button type="button" class="panel-header" onclick={() => (showSeoPanel = !showSeoPanel)}>
					<h3>SEO Settings</h3>
					<span>{showSeoPanel ? '−' : '+'}</span>
				</button>

				{#if showSeoPanel}
					<div class="panel-content">
						<SeoMetaFields bind:meta={post} />
					</div>
				{/if}
			</div>
		</div>

		<!-- Sidebar -->
		<div class="editor-sidebar">
			<!-- Publish Settings -->
			<div class="sidebar-panel">
				<h3>Publish</h3>

				<div class="form-group">
					<label for="publish-date">
						<IconCalendar size={16} />
						Publish Date
					</label>
					<input id="publish-date" type="datetime-local" bind:value={post.published_at} />
				</div>

				<div class="form-group">
					<label>
						<input type="checkbox" bind:checked={post.allow_comments} />
						Allow Comments
					</label>
				</div>
			</div>

			<!-- Featured Image -->
			<div class="sidebar-panel">
				<h3>Featured Image</h3>

				{#if uploadingImage}
					<div class="upload-loading">
						<IconLoader size={48} class="spin" />
						<span>Uploading image...</span>
					</div>
				{:else if post.featured_image}
					<div class="featured-image-preview">
						<img src={post.featured_image} alt={post.featured_image_alt || 'Featured'} />
						<button type="button" class="remove-image" onclick={removeFeaturedImage}>
							<IconX size={16} />
						</button>
					</div>

					<div class="form-group">
						<label for="img-title">Image Title</label>
						<input
							id="img-title"
							type="text"
							bind:value={post.featured_image_title}
							placeholder="Image title for SEO"
						/>
					</div>

					<div class="form-group">
						<label for="img-alt">Alt Text</label>
						<input
							id="img-alt"
							type="text"
							bind:value={post.featured_image_alt}
							placeholder="Describe the image for accessibility"
						/>
					</div>

					<div class="form-group">
						<label for="img-caption">Caption</label>
						<input
							id="img-caption"
							type="text"
							bind:value={post.featured_image_caption}
							placeholder="Image caption displayed below image"
						/>
					</div>

					<div class="form-group">
						<label for="img-description">Description</label>
						<textarea
							id="img-description"
							bind:value={post.featured_image_description}
							placeholder="Detailed description of the image"
							rows="3"
						></textarea>
					</div>
				{:else}
					<label class="upload-box" class:disabled={uploadingImage}>
						<input type="file" accept="image/*" onchange={handleFeaturedImageUpload} hidden disabled={uploadingImage} />
						<IconPhoto size={48} />
						<span>Click to upload featured image</span>
					</label>
					{#if uploadError}
						<p class="upload-error">{uploadError}</p>
					{/if}
				{/if}
			</div>

			<!-- Categories (Colorful Button Selection) -->
			<div class="sidebar-panel categories-panel">
				<h3>
					<IconTags size={16} />
					Categories
				</h3>
				<div class="categories-grid">
					{#each predefinedCategories as category}
						<button
							type="button"
							class="category-btn"
							class:selected={isCategorySelected(category.id)}
							style:--tag-color={category.color}
							onclick={() => toggleCategorySelection(category.id)}
						>
							{#if isCategorySelected(category.id)}
								<IconCheck size={14} />
							{/if}
							{category.name}
						</button>
					{/each}
				</div>

				{#if post.categories.length > 0}
					<div class="selected-categories">
						<span class="selected-count">{post.categories.length} selected:</span>
						{#each post.categories as categoryId}
							{@const category = getPredefinedCategoryById(categoryId)}
							{#if category}
								<span class="selected-tag" style:--tag-color={category.color}>
									{category.name}
									<button type="button" onclick={() => toggleCategorySelection(categoryId)} aria-label="Remove {category.name}">
										<IconX size={12} />
									</button>
								</span>
							{/if}
						{/each}
					</div>
				{/if}
			</div>

			<!-- Tags -->
			<div class="sidebar-panel">
				<h3>Tags</h3>

				<div class="tags-selected">
					{#each post.tags as tagId}
						{@const tag = availableTags.find((t) => t.id === tagId)}
						{#if tag}
							<span class="tag-badge" style="background: {tag.color}20; color: {tag.color}">
								{tag.name}
								<button
									type="button"
									onclick={() => (post.tags = post.tags.filter((id) => id !== tagId))}
								>
									<IconX size={14} />
								</button>
							</span>
						{/if}
					{/each}
				</div>

				<div class="tag-input-group">
					<input
						type="text"
						bind:value={newTag}
						placeholder="Add new tag..."
						onkeydown={(e) => e.key === 'Enter' && (e.preventDefault(), createTag())}
					/>
					<button type="button" class="btn-add-tag" onclick={createTag}>
						<IconPlus size={16} />
					</button>
				</div>

				<div class="checkbox-list">
					{#each availableTags as tag}
						{#if !post.tags.includes(tag.id)}
							<label class="checkbox-item">
								<input
									type="checkbox"
									value={tag.id}
									onchange={(e) => {
										if (e.currentTarget.checked) {
											post.tags = [...post.tags, tag.id];
										}
									}}
								/>
								<span style="color: {tag.color}">{tag.name}</span>
							</label>
						{/if}
					{/each}
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.post-editor {
		min-height: 100vh;
		background: #f8f9fa;
	}

	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.5rem 2rem;
		background: white;
		border-bottom: 1px solid #e5e5e5;
		position: sticky;
		top: 0;
		z-index: 100;
	}

	.editor-header h1 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.header-actions {
		display: flex;
		gap: 0.75rem;
	}

	.btn-primary,
	.btn-secondary {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		background: #2563eb;
	}

	.btn-primary:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-secondary {
		background: white;
		color: #666;
		border: 1px solid #e5e5e5;
	}

	.btn-secondary:hover {
		background: #f8f9fa;
	}

	.editor-layout {
		display: grid;
		grid-template-columns: 1fr 350px;
		gap: 2rem;
		padding: 2rem;
		max-width: 1600px;
		margin: 0 auto;
	}

	.editor-main {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 500;
		color: #1a1a1a;
		font-size: 0.95rem;
	}

	.title-input {
		width: 100%;
		padding: 1rem;
		font-size: 2rem;
		font-weight: 700;
		border: none;
		border-radius: 8px;
		background: white;
		color: #1a1a1a;
		outline: none;
	}

	.title-input:focus {
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.slug-input {
		display: flex;
		align-items: center;
		background: white;
		border-radius: 6px;
		overflow: hidden;
		border: 1px solid #e5e5e5;
	}

	.slug-prefix {
		padding: 0.75rem 1rem;
		background: #f8f9fa;
		color: #999;
		font-size: 0.9rem;
		white-space: nowrap;
	}

	.slug-input input {
		flex: 1;
		padding: 0.75rem 1rem;
		border: none;
		outline: none;
		font-size: 0.95rem;
		color: #1a1a1a;
		background: white;
	}

	textarea {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
		resize: vertical;
		background: white;
		color: #1a1a1a;
	}

	textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.panel {
		background: white;
		border-radius: 8px;
		overflow: hidden;
	}

	.panel-header {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 1.5rem;
		background: white;
		border: none;
		cursor: pointer;
		border-bottom: 1px solid #f0f0f0;
	}

	.panel-header h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.panel-content {
		padding: 1.5rem;
	}

	.editor-sidebar {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.sidebar-panel {
		background: white;
		padding: 1.5rem;
		border-radius: 8px;
		border: 1px solid #e5e5e5;
	}

	.sidebar-panel h3 {
		font-size: 1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0 0 1rem;
		padding-bottom: 0.75rem;
		border-bottom: 1px solid #f0f0f0;
	}

	.sidebar-panel input[type='text'],
	.sidebar-panel input[type='datetime-local'] {
		width: 100%;
		padding: 0.625rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.9rem;
		color: #1a1a1a;
		background: white;
	}

	.sidebar-panel input[type='checkbox'] {
		margin-right: 0.5rem;
	}

	.upload-box {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		border: 2px dashed #e5e5e5;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		color: #999;
	}

	.upload-box:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.upload-box span {
		margin-top: 0.5rem;
		font-size: 0.9rem;
	}

	.featured-image-preview {
		position: relative;
		margin-bottom: 1rem;
	}

	.featured-image-preview img {
		width: 100%;
		border-radius: 6px;
	}

	.remove-image {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.7);
		color: white;
		border: none;
		border-radius: 50%;
		cursor: pointer;
	}

	.checkbox-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		max-height: 200px;
		overflow-y: auto;
	}

	.checkbox-item {
		display: flex;
		align-items: center;
		padding: 0.5rem;
		border-radius: 4px;
		cursor: pointer;
	}

	.checkbox-item:hover {
		background: #f8f9fa;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COLORFUL CATEGORY SYSTEM (matching video management)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.categories-panel h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.categories-grid {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem;
		background: #f8f9fa;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		max-height: 280px;
		overflow-y: auto;
	}

	.category-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		background: rgba(100, 116, 139, 0.1);
		border: 1px solid rgba(100, 116, 139, 0.2);
		border-radius: 6px;
		font-size: 0.8rem;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.15s;
	}

	.category-btn:hover {
		background: color-mix(in srgb, var(--tag-color, #6366f1) 15%, transparent);
		border-color: color-mix(in srgb, var(--tag-color, #6366f1) 30%, transparent);
		color: var(--tag-color, #6366f1);
	}

	.category-btn.selected {
		background: color-mix(in srgb, var(--tag-color, #6366f1) 20%, transparent);
		border-color: var(--tag-color, #6366f1);
		color: var(--tag-color, #6366f1);
	}

	.selected-categories {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: rgba(59, 130, 246, 0.05);
		border: 1px solid rgba(59, 130, 246, 0.1);
		border-radius: 8px;
	}

	.selected-count {
		font-size: 0.75rem;
		color: #64748b;
		margin-right: 0.5rem;
		width: 100%;
	}

	.selected-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.625rem;
		background: color-mix(in srgb, var(--tag-color, #6366f1) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, #6366f1) 30%, transparent);
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--tag-color, #6366f1);
	}

	.selected-tag button {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		background: transparent;
		border: none;
		color: inherit;
		cursor: pointer;
		opacity: 0.7;
		transition: opacity 0.15s;
	}

	.selected-tag button:hover {
		opacity: 1;
	}

	.tags-selected {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.tag-badge {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		border-radius: 4px;
		font-size: 0.85rem;
		font-weight: 500;
	}

	.tag-badge button {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		display: flex;
		align-items: center;
	}

	.tag-input-group {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1rem;
	}

	.tag-input-group input {
		flex: 1;
	}

	.btn-add-tag {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #3b82f6;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
	}

	.btn-add-tag:hover {
		background: #2563eb;
	}

	.upload-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 1rem;
		color: #3b82f6;
	}

	.upload-loading span {
		margin-top: 0.5rem;
		font-size: 0.9rem;
		color: #666;
	}

	.upload-loading :global(.spin) {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.upload-box.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.upload-error {
		color: #ef4444;
		font-size: 0.85rem;
		margin-top: 0.5rem;
		text-align: center;
	}

	.sidebar-panel textarea {
		width: 100%;
		padding: 0.625rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.9rem;
		font-family: inherit;
		resize: vertical;
		color: #1a1a1a;
		background: white;
	}

	.sidebar-panel textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}

	@media (max-width: 1024px) {
		.editor-layout {
			grid-template-columns: 1fr;
		}
	}
</style>

<script lang="ts">
	import { page } from '$app/stores';
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
	import IconKeyboard from '@tabler/icons-svelte/icons/keyboard';
	import IconMaximize from '@tabler/icons-svelte/icons/maximize';
	import IconMinimize from '@tabler/icons-svelte/icons/minimize';
	import IconArrowLeft from '@tabler/icons-svelte/icons/arrow-left';
	import { BlockEditor, type Block } from '$lib/components/blog/BlockEditor';
	import SeoMetaFields from '$lib/components/blog/SeoMetaFields.svelte';
	import { api } from '$lib/api/config';
	import { mediaApi } from '$lib/api/media';
	import { adminFetch } from '$lib/utils/adminFetch';

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
		return predefinedCategories.find((c) => c.id === id);
	}

	// Get post ID from URL params
	const postId = $derived($page.params.id);

	let post = $state({
		id: 0,
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
		categories: [] as string[],
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
			post.categories = post.categories.filter((c) => c !== categoryId);
		}
	}

	function isCategorySelected(categoryId: string): boolean {
		return post.categories.includes(categoryId);
	}

	// Content blocks for the BlockEditor
	let contentBlocks = $state<Block[]>([]);

	let availableTags: any[] = $state([]);
	let loading = $state(true);
	let saving = $state(false);
	let saveError = $state('');
	let saveSuccess = $state('');
	let newTag = $state('');
	let showSeoPanel = $state(false);
	let uploadingImage = $state(false);
	let uploadError = $state('');
	let isFullscreen = $state(false);

	onMount(async () => {
		await Promise.all([loadPost(), loadTags()]);
	});

	async function loadPost() {
		loading = true;
		try {
			const data = await adminFetch(`/api/admin/posts/${postId}`);
			const postData = data.data || data;

			// Map post data to our state
			post = {
				id: postData.id,
				title: postData.title || '',
				slug: postData.slug || '',
				excerpt: postData.excerpt || '',
				content_blocks: postData.content_blocks || [],
				featured_image: postData.featured_image || '',
				featured_image_alt: postData.featured_image_alt || '',
				featured_image_title: postData.featured_image_title || '',
				featured_image_caption: postData.featured_image_caption || '',
				featured_image_description: postData.featured_image_description || '',
				featured_media_id: postData.featured_media_id || null,
				status: postData.status || 'draft',
				published_at: postData.published_at
					? new Date(postData.published_at).toISOString().slice(0, 16)
					: '',
				allow_comments: postData.allow_comments ?? true,
				meta_title: postData.meta_title || '',
				meta_description: postData.meta_description || '',
				meta_keywords: postData.meta_keywords || [],
				indexable: postData.indexable ?? true,
				canonical_url: postData.canonical_url || '',
				categories: postData.categories || [],
				tags: postData.tags?.map((t: any) => (typeof t === 'object' ? t.id : t)) || []
			};

			// Load blocks from the post data
			if (postData.blocks && Array.isArray(postData.blocks) && postData.blocks.length > 0) {
				contentBlocks = postData.blocks.map((b: any) => ({
					id: b.id || crypto.randomUUID(),
					type: b.type,
					content: b.content,
					settings: b.settings || {},
					metadata: b.metadata || {
						createdAt: new Date().toISOString(),
						updatedAt: new Date().toISOString(),
						version: 1
					}
				}));
			} else {
				// If no blocks, start with empty paragraph
				contentBlocks = [
					{
						id: crypto.randomUUID(),
						type: 'paragraph',
						content: { text: '' },
						settings: {},
						metadata: {
							createdAt: new Date().toISOString(),
							updatedAt: new Date().toISOString(),
							version: 1
						}
					}
				];
			}
		} catch (error) {
			console.error('Failed to load post:', error);
			saveError = 'Failed to load post';
		} finally {
			loading = false;
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
		if (post.title) {
			post.slug = post.title
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
		}
	}

	// Handle block editor changes
	function handleBlocksChange(blocks: Block[]) {
		contentBlocks = blocks;
		post.content_blocks = blocks.map((b) => ({
			type: b.type,
			content: b.content,
			settings: b.settings
		}));
	}

	// Handle block editor save
	function handleEditorSave(blocks: Block[]) {
		handleBlocksChange(blocks);
		// Auto-save as draft
		savePost(post.status);
	}

	// Convert blocks to HTML for backward compatibility
	function blocksToHtml(blocks: Block[]): string {
		return blocks
			.map((block) => {
				const content = block.content as any;
				switch (block.type) {
					case 'paragraph':
						return `<p>${content.text || ''}</p>`;
					case 'heading':
						const level = content.level || 2;
						return `<h${level}>${content.text || ''}</h${level}>`;
					case 'quote':
						return `<blockquote>${content.text || ''}</blockquote>`;
					case 'list':
						const items = content.items || [];
						const listType = content.listType === 'ordered' ? 'ol' : 'ul';
						return `<${listType}>${items.map((item: string) => `<li>${item}</li>`).join('')}</${listType}>`;
					case 'image':
						return `<figure><img src="${content.src || ''}" alt="${content.alt || ''}" />${content.caption ? `<figcaption>${content.caption}</figcaption>` : ''}</figure>`;
					case 'code':
						return `<pre><code class="language-${content.language || 'text'}">${content.code || ''}</code></pre>`;
					case 'separator':
						return '<hr />';
					case 'html':
						return content.html || '';
					default:
						return content.text ? `<p>${content.text}</p>` : '';
				}
			})
			.join('\n');
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
		saveError = '';
		saveSuccess = '';

		try {
			// Validate required fields
			if (!post.title.trim()) {
				saveError = 'Title is required';
				return;
			}

			// Convert blocks to both formats for backward compatibility
			const htmlContent = blocksToHtml(contentBlocks);

			const postData = {
				...post,
				status,
				content: htmlContent, // HTML for backward compatibility
				blocks: contentBlocks.map((b) => ({
					id: b.id,
					type: b.type,
					content: b.content,
					settings: b.settings,
					metadata: b.metadata
				})),
				published_at:
					status === 'published' && !post.published_at
						? new Date().toISOString()
						: post.published_at || null
			};

			// Update existing post
			const response = await fetch(`/api/admin/posts/${postId}`, {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(postData)
			});

			const result = await response.json();

			if (!response.ok) {
				throw new Error(result.message || 'Failed to save post');
			}

			saveSuccess = status === 'published' ? 'Post published!' : 'Changes saved!';

			// Clear success message after a delay
			setTimeout(() => {
				saveSuccess = '';
			}, 3000);
		} catch (error: any) {
			console.error('Failed to save post:', error);
			saveError = error.message || 'Failed to save post. Please try again.';
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
	}
</script>

<svelte:head>
	<title>Edit Post | Blog</title>
</svelte:head>

{#if loading}
	<div class="loading-state">
		<IconLoader size={48} class="spin" />
		<span>Loading post...</span>
	</div>
{:else}
	<div class="post-editor">
		<div class="editor-header">
			<div class="header-left">
				<button class="btn-back" onclick={() => goto('/admin/blog')}>
					<IconArrowLeft size={20} />
				</button>
				<h1>Edit Post</h1>
				{#if post.status === 'published'}
					<span class="status-badge published">Published</span>
				{:else if post.status === 'draft'}
					<span class="status-badge draft">Draft</span>
				{:else if post.status === 'scheduled'}
					<span class="status-badge scheduled">Scheduled</span>
				{/if}
			</div>
			<div class="header-actions">
				{#if saveError}
					<span class="save-error">{saveError}</span>
				{/if}
				{#if saveSuccess}
					<span class="save-success">{saveSuccess}</span>
				{/if}
				<button class="btn-secondary" onclick={() => goto('/admin/blog')}> Cancel </button>
				<button class="btn-secondary" onclick={() => savePost('draft')} disabled={saving}>
					<IconDeviceFloppy size={18} />
					{saving ? 'Saving...' : 'Save Draft'}
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
					<input
						type="text"
						bind:value={post.title}
						placeholder="Post Title"
						class="title-input"
					/>
				</div>

				<div class="form-group">
					<label for="slug">URL Slug</label>
					<div class="slug-input">
						<span class="slug-prefix">your-site.com/blog/</span>
						<input id="slug" type="text" bind:value={post.slug} placeholder="post-url-slug" />
						<button type="button" class="btn-regenerate" onclick={generateSlug}>Regenerate</button>
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

				<!-- Advanced Block Editor -->
				<div class="form-group editor-container" class:fullscreen={isFullscreen}>
					<div class="editor-toolbar">
						<h3 class="editor-label">Content</h3>
						<div class="editor-actions">
							<button
								type="button"
								class="toolbar-btn"
								title="Keyboard shortcuts"
								onclick={() => {
									/* Handled by BlockEditor */
								}}
							>
								<IconKeyboard size={18} />
							</button>
							<button
								type="button"
								class="toolbar-btn"
								title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}
								onclick={() => (isFullscreen = !isFullscreen)}
							>
								{#if isFullscreen}
									<IconMinimize size={18} />
								{:else}
									<IconMaximize size={18} />
								{/if}
							</button>
						</div>
					</div>
					<BlockEditor
						bind:blocks={contentBlocks}
						postTitle={post.title}
						postSlug={post.slug}
						metaDescription={post.meta_description}
						focusKeyword={post.meta_keywords?.[0] || ''}
						onsave={handleEditorSave}
						onchange={handleBlocksChange}
						autosaveInterval={30000}
					/>
				</div>

				<!-- SEO Section -->
				<div class="panel">
					<button
						type="button"
						class="panel-header"
						onclick={() => (showSeoPanel = !showSeoPanel)}
					>
						<h3>SEO Settings</h3>
						<span>{showSeoPanel ? '-' : '+'}</span>
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
							<input
								type="file"
								accept="image/*"
								onchange={handleFeaturedImageUpload}
								hidden
								disabled={uploadingImage}
							/>
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
										<button
											type="button"
											onclick={() => toggleCategorySelection(categoryId)}
											aria-label="Remove {category.name}"
										>
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
							onkeydown={(e: KeyboardEvent) =>
								e.key === 'Enter' && (e.preventDefault(), createTag())}
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
										onchange={(e: Event) => {
											if ((e.currentTarget as HTMLInputElement).checked) {
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
{/if}

<style>
	.post-editor {
		min-height: 100vh;
		background: #f8f9fa;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		color: #666;
		gap: 1rem;
	}

	.loading-state :global(.spin) {
		animation: spin 1s linear infinite;
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
		background: #f8f9fa;
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		cursor: pointer;
		color: #666;
		transition: all 0.2s;
	}

	.btn-back:hover {
		background: #e5e5e5;
		color: #1a1a1a;
	}

	.editor-header h1 {
		font-size: 1.5rem;
		font-weight: 600;
		color: #1a1a1a;
		margin: 0;
	}

	.status-badge {
		padding: 0.375rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
	}

	.status-badge.published {
		background: rgba(16, 185, 129, 0.1);
		color: #10b981;
	}

	.status-badge.draft {
		background: rgba(245, 158, 11, 0.1);
		color: #f59e0b;
	}

	.status-badge.scheduled {
		background: rgba(59, 130, 246, 0.1);
		color: #3b82f6;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.save-error {
		padding: 0.5rem 1rem;
		background: #fef2f2;
		border: 1px solid #fecaca;
		border-radius: 6px;
		color: #dc2626;
		font-size: 0.875rem;
		font-weight: 500;
	}

	.save-success {
		padding: 0.5rem 1rem;
		background: #f0fdf4;
		border: 1px solid #bbf7d0;
		border-radius: 6px;
		color: #16a34a;
		font-size: 0.875rem;
		font-weight: 500;
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

	.btn-regenerate {
		padding: 0.75rem 1rem;
		background: #f8f9fa;
		border: none;
		border-left: 1px solid #e5e5e5;
		color: #666;
		font-size: 0.85rem;
		cursor: pointer;
		white-space: nowrap;
	}

	.btn-regenerate:hover {
		background: #e5e5e5;
		color: #1a1a1a;
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

	/* Category styles */
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

	/* Block Editor styles */
	.editor-container {
		display: flex;
		flex-direction: column;
		background: white;
		border-radius: 8px;
		overflow: hidden;
		border: 1px solid #e5e5e5;
		min-height: 600px;
	}

	.editor-container.fullscreen {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		z-index: 1000;
		border-radius: 0;
		min-height: 100vh;
	}

	.editor-toolbar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid #e5e5e5;
		background: #f8f9fa;
	}

	.editor-label {
		font-weight: 600;
		color: #1a1a1a;
		font-size: 0.95rem;
		margin: 0;
	}

	.editor-actions {
		display: flex;
		gap: 0.5rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: white;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		cursor: pointer;
		color: #666;
		transition: all 0.2s;
	}

	.toolbar-btn:hover {
		background: #f0f0f0;
		color: #3b82f6;
	}

	/* Make BlockEditor take full height */
	.editor-container :global(.block-editor) {
		flex: 1;
		min-height: 500px;
	}

	.editor-container.fullscreen :global(.block-editor) {
		min-height: calc(100vh - 60px);
	}

	@media (max-width: 1024px) {
		.editor-layout {
			grid-template-columns: 1fr;
		}
	}
</style>

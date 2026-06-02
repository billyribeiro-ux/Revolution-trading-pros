<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import IconDeviceFloppy from '@tabler/icons-svelte-runes/icons/device-floppy';
	import IconEye from '@tabler/icons-svelte-runes/icons/eye';
	import IconPhoto from '@tabler/icons-svelte-runes/icons/photo';
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconLoader from '@tabler/icons-svelte-runes/icons/loader';
	import IconCheck from '@tabler/icons-svelte-runes/icons/check';
	import IconTags from '@tabler/icons-svelte-runes/icons/tags';
	import IconKeyboard from '@tabler/icons-svelte-runes/icons/keyboard';
	import IconMaximize from '@tabler/icons-svelte-runes/icons/maximize';
	import IconMinimize from '@tabler/icons-svelte-runes/icons/minimize';
	import { BlockEditor, type Block } from '$lib/components/blog/BlockEditor';
	import SeoMetaFields from '$lib/components/blog/SeoMetaFields.svelte';
	import { api } from '$lib/api/config';
	import { mediaApi } from '$lib/api/media';
	// FIX-2026-04-26 (P2-4): sanitize raw-HTML blog blocks before send.
	import { sanitizeBlogContent } from '$lib/utils/sanitize';
	import { logger } from '$lib/utils/logger';
	import {
		predefinedCategories,
		getPredefinedCategoryById
		// `type BlogCategory` commented out (audit 2026-05-16): traced — this
		// page models categories as string IDs (`post.categories: string[]`)
		// and never annotates with BlogCategory. Kept here (not deleted) as a
		// pointer in case typed categories get wired in later.
		// , type BlogCategory
	} from '$lib/data/predefined-categories';

	// Tag row as returned by /api/admin/tags. `color` is used for badge styling.
	interface TagRow {
		id: number;
		name?: string;
		color?: string;
	}

	let post = $state({
		title: '',
		slug: '',
		excerpt: '',
		content_blocks: [] as Pick<Block, 'type' | 'content' | 'settings'>[],
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

	// CATEGORY SELECTION FUNCTIONS

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
	let contentBlocks = $state<Block[]>([
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
	]);

	let availableTags: TagRow[] = $state([]);
	let saving = $state(false);
	let saveError = $state('');
	let saveSuccess = $state('');
	let newTag = $state('');
	let showSeoPanel = $state(false);
	let uploadingImage = $state(false);
	let uploadError = $state('');
	let isFullscreen = $state(false);

	onMount(() => {
		loadTags();
		generateSlug();
	});

	async function loadTags() {
		try {
			// Use same-origin fetch so the SvelteKit proxy attaches rtp_access_token
			// from the httpOnly cookie. api.get('/api/admin/...') would bypass the
			// proxy by routing directly to API_BASE_URL.
			const resp = await fetch('/api/admin/tags', { credentials: 'include' });
			if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
			const data = (await resp.json()) as { data?: TagRow[] } | TagRow[];
			availableTags = (Array.isArray(data) ? data : data.data) || [];
		} catch (error) {
			logger.error('Failed to load tags:', error);
		}
	}

	// FIX-2026-04-26 (P0-4): track manual slug edits — once true, never
	// auto-overwrite from the title.
	let slugEdited = $state(false);
	// FIX-2026-04-26 (P1-4): debounced slug uniqueness probe state.
	let slugCheckStatus = $state<'idle' | 'checking' | 'available' | 'taken' | 'error'>('idle');
	let slugCheckTimer: ReturnType<typeof setTimeout> | null = null;

	function handleSlugInput() {
		slugEdited = true;
		scheduleSlugUniquenessCheck();
	}

	function generateSlug() {
		if (!post.slug && post.title) {
			post.slug = post.title
				.toLowerCase()
				.replace(/[^a-z0-9]+/g, '-')
				.replace(/^-+|-+$/g, '');
			scheduleSlugUniquenessCheck();
		}
	}

	function scheduleSlugUniquenessCheck() {
		if (slugCheckTimer) clearTimeout(slugCheckTimer);
		const slug = post.slug?.trim();
		if (!slug) {
			slugCheckStatus = 'idle';
			return;
		}
		slugCheckStatus = 'checking';
		slugCheckTimer = setTimeout(async () => {
			try {
				const res = await fetch(`/api/admin/posts?slug=${encodeURIComponent(slug)}&per_page=1`);
				if (!res.ok) {
					slugCheckStatus = 'error';
					return;
				}
				const body = (await res.json()) as { data?: { slug?: string }[] };
				const items = body?.data ?? [];
				const taken = items.some((p) => p.slug === slug);
				slugCheckStatus = taken ? 'taken' : 'available';
			} catch {
				slugCheckStatus = 'error';
			}
		}, 400);
	}

	// Handle block editor changes
	function handleBlocksChange(blocks: Block[]) {
		contentBlocks = blocks;
		// Also update the post content_blocks
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
		savePost('draft');
	}

	// Convert blocks to HTML for backward compatibility.
	// FIX-2026-04-26 (P2-4): user-controlled `html` blocks are now passed through
	// sanitizeBlogContent (DOMPurify, rich profile) so an admin compromise cannot
	// inject `<script>` etc. into the public blog. Defense-in-depth — server
	// should also sanitize.
	function blocksToHtml(blocks: Block[]): string {
		return blocks
			.map((block) => {
				// BlockContent is a wide union; index dynamically as a generic record so
				// legacy fields (level/src/alt/items/caption) resolve at runtime.
				const content = block.content as Record<string, unknown>;
				switch (block.type) {
					case 'paragraph':
						return `<p>${content.text || ''}</p>`;
					case 'heading':
						const level = content.level || 2;
						return `<h${level}>${content.text || ''}</h${level}>`;
					case 'quote':
						return `<blockquote>${content.text || ''}</blockquote>`;
					case 'list':
						const items = (content.items as string[] | undefined) || [];
						const listType = content.listType === 'ordered' ? 'ol' : 'ul';
						return `<${listType}>${items.map((item: string) => `<li>${item}</li>`).join('')}</${listType}>`;
					case 'image':
						return `<figure><img src="${content.src || ''}" alt="${content.alt || ''}" />${content.caption ? `<figcaption>${content.caption}</figcaption>` : ''}</figure>`;
					case 'code':
						return `<pre><code class="language-${content.language || 'text'}">${content.code || ''}</code></pre>`;
					case 'separator':
						return '<hr />';
					case 'html':
						// FIX-2026-04-26 (P2-4): sanitize raw-HTML blocks.
						return sanitizeBlogContent(String(content.html || ''));
					default:
						return content.text ? `<p>${content.text}</p>` : '';
				}
			})
			.join('\n');
	}

	async function createTag() {
		if (!newTag.trim()) return;

		try {
			type TagRes = { data?: TagRow } | TagRow;
			const data = await api.post<TagRes>('/api/admin/tags', { name: newTag.trim() });
			const newTagData = (data && 'data' in data && data.data ? data.data : data) as TagRow;
			availableTags = [...availableTags, newTagData];
			post.tags = [...post.tags, newTagData.id];
			newTag = '';
		} catch (error) {
			logger.error('Failed to create tag:', error);
		}
	}

	// Normalize any date string to the "YYYY-MM-DDTHH:MM:SS" format the Rust
	// backend requires (NaiveDateTime — no Z, no ms, seconds always present).
	function toNaiveDateTime(value: string | null): string | null {
		if (!value) return null;
		const s = value.replace(/\.\d+Z?$/, '').replace('Z', '');
		// datetime-local gives "YYYY-MM-DDTHH:MM" — pad missing seconds
		return /T\d{2}:\d{2}$/.test(s) ? `${s}:00` : s;
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
				// Map tag IDs → names; backend expects Vec<String> (tag names), not IDs.
				tags: post.tags
					.map((id) => availableTags.find((t) => t.id === id)?.name)
					.filter((name): name is string => !!name),
				blocks: contentBlocks.map((b) => ({
					id: b.id,
					type: b.type,
					content: b.content,
					settings: b.settings,
					metadata: b.metadata
				})),
				// Backend expects NaiveDateTime "YYYY-MM-DDTHH:MM:SS" — no Z, no ms.
				// datetime-local inputs give "YYYY-MM-DDTHH:MM" (no seconds); pad them.
				published_at: toNaiveDateTime(
					status === 'published' && !post.published_at
						? new Date().toISOString()
						: post.published_at || null
				)
			};

			// Use SvelteKit proxy endpoint (relative URL)
			const response = await fetch('/api/admin/posts', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(postData)
			});

			const result = await response.json();

			if (!response.ok) {
				throw new Error(result.message || 'Failed to save post');
			}

			saveSuccess = status === 'published' ? 'Post published!' : 'Draft saved!';

			// Navigate after short delay to show success message
			setTimeout(() => goto('/admin/blog'), 1000);
		} catch (error) {
			logger.error('Failed to save post:', error);
			const err = error as { message?: string };
			saveError = err.message || 'Failed to save post. Please try again.';
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
			} catch (error) {
				logger.error('Failed to upload featured image:', error);
				const err = error as { message?: string };
				uploadError = err.message || 'Failed to upload image';
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

	// FIX-2026-04-26 (P0-4): only auto-generate slug if user hasn't manually
	// edited the slug field; previous code would clobber a deliberate slug on
	// every title keystroke if the user ever cleared the slug field.
	$effect(() => {
		if (post.title && !slugEdited) {
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
					id="post-title"
					name="title"
					type="text"
					bind:value={post.title}
					placeholder="Post Title"
					class="title-input"
					autocomplete="off"
				/>
			</div>

			<div class="form-group">
				<label for="slug">URL Slug</label>
				<div class="slug-input">
					<span class="slug-prefix">your-site.com/blog/</span>
					<!-- FIX-2026-04-26 (P0-4 / P1-4): mark slug as user-edited on input
					     and trigger debounced uniqueness probe against the API. -->
					<input
						id="slug"
						name="slug"
						type="text"
						bind:value={post.slug}
						oninput={handleSlugInput}
						placeholder="post-url-slug"
						autocomplete="off"
					/>
				</div>
				{#if slugCheckStatus === 'checking'}
					<p class="help-text">Checking availability…</p>
				{:else if slugCheckStatus === 'taken'}
					<p class="help-text" style="color:#dc2626">This slug is already taken</p>
				{:else if slugCheckStatus === 'available'}
					<p class="help-text" style="color:#059669">Slug is available</p>
				{:else if slugCheckStatus === 'error'}
					<p class="help-text" style="color:#92400e">Could not verify slug</p>
				{/if}
			</div>

			<div class="form-group">
				<label for="excerpt">Excerpt</label>
				<textarea
					id="excerpt"
					name="excerpt"
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
					<input
						id="publish-date"
						name="published_at"
						type="datetime-local"
						bind:value={post.published_at}
					/>
				</div>

				<div class="form-group">
					<label>
						<input
							id="allow-comments"
							name="allow_comments"
							type="checkbox"
							bind:checked={post.allow_comments}
						/>
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
						<!-- TODO(cls): width/height needed — user-uploaded blog featured image; capture intrinsic dims on upload and store alongside URL -->
						<img
							src={post.featured_image}
							alt={post.featured_image_alt || 'Featured'}
							loading="lazy"
						/>
						<button type="button" class="remove-image" onclick={removeFeaturedImage}>
							<IconX size={16} />
						</button>
					</div>

					<div class="form-group">
						<label for="img-title">Image Title</label>
						<input
							id="img-title"
							name="featured_image_title"
							type="text"
							bind:value={post.featured_image_title}
							placeholder="Image title for SEO"
						/>
					</div>

					<div class="form-group">
						<label for="img-alt">Alt Text</label>
						<input
							id="img-alt"
							name="featured_image_alt"
							type="text"
							bind:value={post.featured_image_alt}
							placeholder="Describe the image for accessibility"
						/>
					</div>

					<div class="form-group">
						<label for="img-caption">Caption</label>
						<input
							id="img-caption"
							name="featured_image_caption"
							type="text"
							bind:value={post.featured_image_caption}
							placeholder="Image caption displayed below image"
						/>
					</div>

					<div class="form-group">
						<label for="img-description">Description</label>
						<textarea
							id="img-description"
							name="featured_image_description"
							bind:value={post.featured_image_description}
							placeholder="Detailed description of the image"
							rows="3"
						></textarea>
					</div>
				{:else}
					<label class="upload-box" class:disabled={uploadingImage}>
						<input
							id="featured-image-upload"
							name="featured_image"
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
					{#each predefinedCategories as category (category.id)}
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
						{#each post.categories as categoryId (categoryId)}
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
					{#each post.tags as tagId (tagId)}
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
						id="new-tag-input"
						name="new_tag"
						type="text"
						bind:value={newTag}
						placeholder="Add new tag..."
						onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && (e.preventDefault(), createTag())}
						autocomplete="off"
					/>
					<button type="button" class="btn-add-tag" onclick={createTag}>
						<IconPlus size={16} />
					</button>
				</div>

				<div class="checkbox-list">
					{#each availableTags as tag (tag.id)}
						{#if !post.tags.includes(tag.id)}
							<label class="checkbox-item">
								<input
									id="tag-{tag.id}"
									name="tags[]"
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

<style>
	.post-editor {
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
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 15%, transparent);
		border-color: color-mix(in srgb, var(--tag-color, var(--primary-500)) 30%, transparent);
		color: var(--tag-color, var(--primary-500));
	}

	.category-btn.selected {
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 20%, transparent);
		border-color: var(--tag-color, var(--primary-500));
		color: var(--tag-color, var(--primary-500));
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
		background: color-mix(in srgb, var(--tag-color, var(--primary-500)) 15%, transparent);
		border: 1px solid color-mix(in srgb, var(--tag-color, var(--primary-500)) 30%, transparent);
		border-radius: 4px;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--tag-color, var(--primary-500));
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   BLOCK EDITOR STYLES
	   ═══════════════════════════════════════════════════════════════════════════ */

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

	@media (max-width: 1023.98px) {
		.editor-layout {
			grid-template-columns: 1fr;
		}
	}
</style>

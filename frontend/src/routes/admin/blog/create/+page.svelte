<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import {
		IconDeviceFloppy,
		IconEye,
		IconPhoto,
		IconX,
		IconPlus,
		IconCalendar
	} from '@tabler/icons-svelte';
	import RichTextEditor from '$lib/components/blog/RichTextEditor.svelte';
	import SeoMetaFields from '$lib/components/blog/SeoMetaFields.svelte';

	let post = {
		title: '',
		slug: '',
		excerpt: '',
		content_blocks: [],
		featured_image: '',
		featured_image_alt: '',
		featured_image_title: '',
		status: 'draft',
		published_at: '',
		allow_comments: true,
		meta_title: '',
		meta_description: '',
		indexable: true,
		canonical_url: '',
		categories: [] as number[],
		tags: [] as number[]
	};

	let content = '';
	let categories: any[] = [];
	let tags: any[] = [];
	let availableCategories: any[] = [];
	let availableTags: any[] = [];
	let saving = false;
	let showFeaturedImage = false;
	let newTag = '';
	let showSeoPanel = false;

	onMount(() => {
		loadCategories();
		loadTags();
		generateSlug();
	});

	async function loadCategories() {
		try {
			const response = await fetch('/api/admin/categories');
			const data = await response.json();
			availableCategories = data.data || [];
		} catch (error) {
			console.error('Failed to load categories:', error);
		}
	}

	async function loadTags() {
		try {
			const response = await fetch('/api/admin/tags');
			const data = await response.json();
			availableTags = data.data || [];
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

	function handleContentChange(event: CustomEvent) {
		content = event.detail;
	}

	async function createTag() {
		if (!newTag.trim()) return;

		try {
			const response = await fetch('/api/admin/tags', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ name: newTag.trim() })
			});

			if (response.ok) {
				const data = await response.json();
				availableTags = [...availableTags, data.data];
				post.tags = [...post.tags, data.data.id];
				newTag = '';
			}
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

			const response = await fetch('/api/admin/posts', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(postData)
			});

			if (response.ok) {
				const data = await response.json();
				goto('/admin/blog');
			} else {
				alert('Failed to save post');
			}
		} catch (error) {
			console.error('Failed to save post:', error);
			alert('Failed to save post');
		} finally {
			saving = false;
		}
	}

	function handleFeaturedImageUpload(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files[0]) {
			const file = input.files[0];
			// Here you would upload to your server
			// For now, create a local URL
			post.featured_image = URL.createObjectURL(file);
			showFeaturedImage = true;
		}
	}

	$: if (post.title) {
		generateSlug();
	}
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
			<button class="btn-secondary" on:click={() => goto('/admin/blog')}> Cancel </button>
			<button class="btn-secondary" on:click={() => savePost('draft')} disabled={saving}>
				<IconDeviceFloppy size={18} />
				Save Draft
			</button>
			<button class="btn-primary" on:click={() => savePost('published')} disabled={saving}>
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
				<RichTextEditor bind:content on:change={handleContentChange} />
			</div>

			<!-- SEO Section -->
			<div class="panel">
				<button type="button" class="panel-header" on:click={() => (showSeoPanel = !showSeoPanel)}>
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

				{#if post.featured_image}
					<div class="featured-image-preview">
						<img src={post.featured_image} alt={post.featured_image_alt || 'Featured'} />
						<button type="button" class="remove-image" on:click={() => (post.featured_image = '')}>
							<IconX size={16} />
						</button>
					</div>

					<div class="form-group">
						<label for="img-title">Image Title</label>
						<input
							id="img-title"
							type="text"
							bind:value={post.featured_image_title}
							placeholder="Image title"
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
				{:else}
					<label class="upload-box">
						<input type="file" accept="image/*" on:change={handleFeaturedImageUpload} hidden />
						<IconPhoto size={48} />
						<span>Click to upload featured image</span>
					</label>
				{/if}
			</div>

			<!-- Categories -->
			<div class="sidebar-panel">
				<h3>Categories</h3>
				<div class="checkbox-list">
					{#each availableCategories as category}
						<label class="checkbox-item">
							<input
								type="checkbox"
								value={category.id}
								checked={post.categories.includes(category.id)}
								on:change={(e) => {
									if (e.currentTarget.checked) {
										post.categories = [...post.categories, category.id];
									} else {
										post.categories = post.categories.filter((id) => id !== category.id);
									}
								}}
							/>
							<span style="color: {category.color}">{category.name}</span>
						</label>
					{/each}
				</div>

				{#if availableCategories.length === 0}
					<p class="empty-text">No categories. <a href="/admin/blog/categories">Create one</a></p>
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
									on:click={() => (post.tags = post.tags.filter((id) => id !== tagId))}
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
						on:keydown={(e) => e.key === 'Enter' && (e.preventDefault(), createTag())}
					/>
					<button type="button" class="btn-add-tag" on:click={createTag}>
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
									on:change={(e) => {
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

	.empty-text {
		color: #999;
		font-size: 0.9rem;
	}

	.empty-text a {
		color: #3b82f6;
		text-decoration: none;
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

	@media (max-width: 1024px) {
		.editor-layout {
			grid-template-columns: 1fr;
		}
	}
</style>

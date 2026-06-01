<script lang="ts">
	import { page } from '$app/state';
	import { goto, beforeNavigate } from '$app/navigation';
	import { onMount } from 'svelte';
	// FIX-2026-04-26 (P2-4): sanitize raw-HTML blog blocks before send.
	import { sanitizeBlogContent } from '$lib/utils/sanitize';
	import IconLoader from '@tabler/icons-svelte-runes/icons/loader';
	import { type Block } from '$lib/components/blog/BlockEditor';
	import SeoMetaFields from '$lib/components/blog/SeoMetaFields.svelte';
	import { api } from '$lib/api/config';
	import { mediaApi } from '$lib/api/media';
	import { logger } from '$lib/utils/logger';

	import EditorHeader from './_components/EditorHeader.svelte';
	import PostMainFields from './_components/PostMainFields.svelte';
	import ContentEditorPanel from './_components/ContentEditorPanel.svelte';
	import PublishSidebarPanel from './_components/PublishSidebarPanel.svelte';
	import FeaturedImagePanel from './_components/FeaturedImagePanel.svelte';
	import CategoriesPanel from './_components/CategoriesPanel.svelte';
	import TagsPanel from './_components/TagsPanel.svelte';
	import type { PostState, TagRow } from './_components/types';

	// Raw block shape as returned by the API (all fields optional / loosely typed).
	interface RawBlock {
		id?: string;
		type: Block['type'];
		content: Block['content'];
		settings?: Block['settings'];
		metadata?: Block['metadata'];
	}

	// Raw post shape as returned by the API. Tags may arrive as bare ids or
	// {id,...} objects; categories as string ids. Only the fields read below are
	// modelled here.
	interface RawPost {
		id: number;
		title?: string;
		slug?: string;
		excerpt?: string;
		content_blocks?: Pick<Block, 'type' | 'content' | 'settings'>[];
		featured_image?: string;
		featured_image_alt?: string;
		featured_image_title?: string;
		featured_image_caption?: string;
		featured_image_description?: string;
		featured_media_id?: number | null;
		status?: string;
		published_at?: string | null;
		allow_comments?: boolean;
		meta_title?: string;
		meta_description?: string;
		meta_keywords?: string[];
		indexable?: boolean;
		canonical_url?: string;
		categories?: string[];
		tags?: (number | { id: number })[];
		blocks?: RawBlock[];
	}

	// Get post ID from URL params
	const postId = $derived(page.params.id);

	let post = $state<PostState>({
		id: 0,
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
		categories: [] as string[],
		tags: [] as number[]
	});

	// Content blocks for the BlockEditor
	let contentBlocks = $state<Block[]>([]);

	let availableTags: TagRow[] = $state([]);
	// FIX-2026-04-26 (P3-4): explicit loading state for tags so the post-edit UI
	// can render a placeholder until tag names resolve, instead of silently
	// rendering raw IDs / no-ops on availableTags.find lookups.
	let tagsLoading = $state(true);
	let loading = $state(true);
	let saving = $state(false);
	let saveError = $state('');
	let saveSuccess = $state('');
	let newTag = $state('');
	let showSeoPanel = $state(false);
	let uploadingImage = $state(false);
	let uploadError = $state('');
	let isFullscreen = $state(false);

	// FIX-2026-04-26 (P1-6): unsaved-changes guard.
	let lastSavedSnapshot = $state<string>('');
	let hasUnsavedChanges = $derived(
		!loading && JSON.stringify({ post, contentBlocks }) !== lastSavedSnapshot
	);

	beforeNavigate((nav) => {
		// TODO(modal-confirm): SvelteKit's beforeNavigate is synchronous —
		// nav.cancel() must be called inline, so native confirm() is the only
		// portable option today. Migrating requires a state-driven flow that
		// intercepts navigation differently.
		if (
			hasUnsavedChanges &&
			!nav.willUnload &&
			!confirm('You have unsaved changes. Discard and leave?')
		) {
			nav.cancel();
		}
	});

	onMount(() => {
		void Promise.all([loadPost(), loadTags()]).then(() => {
			// Snapshot once both loads complete.
			lastSavedSnapshot = JSON.stringify({ post, contentBlocks });
		});
		const handler = (e: BeforeUnloadEvent) => {
			if (hasUnsavedChanges) {
				e.preventDefault();
			}
		};
		window.addEventListener('beforeunload', handler);
		return () => window.removeEventListener('beforeunload', handler);
	});

	async function loadPost() {
		loading = true;
		try {
			// Use same-origin fetch so the SvelteKit proxy attaches rtp_access_token
			// from the httpOnly cookie. adminFetch relies on in-memory token which may
			// be null on fresh page load before the auth store restores.
			const resp = await fetch(`/api/admin/posts/${postId}`, { credentials: 'include' });
			if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
			const data = (await resp.json()) as { data?: RawPost } | RawPost;
			const postData = ('data' in data && data.data ? data.data : data) as RawPost;

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
				// FIX-2026-04-26 (P3-11): backend returns either bare ids or {id,...}
				// objects; normalize to ids here. The polymorphism still papers over
				// a backend-shape mismatch — track in API contract notes.
				tags: postData.tags?.map((t) => (t && typeof t === 'object' && 'id' in t ? t.id : t)) || []
			};

			// Load blocks from the post data
			if (postData.blocks && Array.isArray(postData.blocks) && postData.blocks.length > 0) {
				contentBlocks = postData.blocks.map((b) => ({
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
			logger.error('Failed to load post:', error);
			saveError = 'Failed to load post';
		} finally {
			loading = false;
		}
	}

	async function loadTags() {
		tagsLoading = true;
		try {
			// Use same-origin fetch so the SvelteKit proxy attaches rtp_access_token
			// from the httpOnly cookie. api.get('/api/admin/...') would bypass the
			// proxy by routing directly to API_BASE_URL.
			const resp = await fetch('/api/admin/tags', { credentials: 'include' });
			if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
			const data = await resp.json();
			availableTags = data.data || data || [];
		} catch (error) {
			logger.error('Failed to load tags:', error);
		} finally {
			tagsLoading = false;
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
						// FIX-2026-04-26 (P2-4): sanitize raw-HTML blocks (DOMPurify).
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

	function toNaiveDateTime(value: string | null): string | null {
		if (!value) return null;
		const s = value.replace(/\.\d+Z?$/, '').replace('Z', '');
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
				published_at: toNaiveDateTime(
					status === 'published' && !post.published_at
						? new Date().toISOString()
						: post.published_at || null
				)
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
			// FIX-2026-04-26 (P1-6): refresh the unsaved-changes baseline.
			lastSavedSnapshot = JSON.stringify({ post, contentBlocks });

			// Clear success message after a delay
			setTimeout(() => {
				saveSuccess = '';
			}, 3000);
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
		<EditorHeader
			status={post.status}
			{saveError}
			{saveSuccess}
			{saving}
			onBack={() => goto('/admin/blog')}
			onSaveDraft={() => savePost('draft')}
			onPublish={() => savePost('published')}
		/>

		<div class="editor-layout">
			<!-- Main Content Area -->
			<div class="editor-main">
				<PostMainFields bind:post onGenerateSlug={generateSlug} />

				<!-- Advanced Block Editor -->
				<ContentEditorPanel
					bind:contentBlocks
					postTitle={post.title}
					postSlug={post.slug}
					metaDescription={post.meta_description}
					focusKeyword={post.meta_keywords?.[0] || ''}
					bind:isFullscreen
					onsave={handleEditorSave}
					onchange={handleBlocksChange}
				/>

				<!-- SEO Section -->
				<div class="panel">
					<button type="button" class="panel-header" onclick={() => (showSeoPanel = !showSeoPanel)}>
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
				<PublishSidebarPanel bind:post />
				<FeaturedImagePanel
					bind:post
					{uploadingImage}
					{uploadError}
					onUpload={handleFeaturedImageUpload}
					onRemove={removeFeaturedImage}
				/>
				<CategoriesPanel bind:post />
				<TagsPanel bind:post {availableTags} {tagsLoading} bind:newTag onCreateTag={createTag} />
			</div>
		</div>
	</div>
{/if}

<style>
	.post-editor {
		background: #f8f9fa;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		color: #666;
		gap: 1rem;
	}

	.loading-state :global(.spin) {
		animation: spin 1s linear infinite;
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

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@media (max-width: 1023.98px) {
		.editor-layout {
			grid-template-columns: 1fr;
		}
	}
</style>

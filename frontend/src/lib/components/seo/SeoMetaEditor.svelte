<script lang="ts">
	import { browser } from '$app/environment';
	import {
		IconFileText,
		IconPhoto,
		IconBrandTwitter,
		IconBrandFacebook,
		IconRobot,
		IconDeviceFloppy,
		IconRefresh,
		IconEye,
		IconCode
	} from '$lib/icons';
	import SeoAnalyzer from './SeoAnalyzer.svelte';
	import SeoPreview from './SeoPreview.svelte';

	interface Props {
		entity: any;
		onsaved?: () => void;
	}

	let { entity, onsaved }: Props = $props();

	let activeTab = $state('general');
	let meta: any = $state({
		title: '',
		description: '',
		canonical_url: '',
		focus_keyword: '',
		additional_keywords: [],
		og_title: '',
		og_description: '',
		og_image: '',
		og_type: 'website',
		twitter_title: '',
		twitter_description: '',
		twitter_image: '',
		twitter_card_type: 'summary_large_image',
		noindex: false,
		nofollow: false,
		noarchive: false,
		noimageindex: false,
		nosnippet: false,
		breadcrumb_title: ''
	});

	let content = $state('');
	let analyzing = $state(false);
	let saving = $state(false);

	const tabs = [
		{ id: 'general', label: 'General', icon: IconFileText },
		{ id: 'social', label: 'Social Media', icon: IconBrandFacebook },
		{ id: 'advanced', label: 'Advanced', icon: IconCode },
		{ id: 'analysis', label: 'Analysis', icon: IconEye }
	];

	$effect(() => {
		if (browser) {
			loadMeta();
		}
	});

	async function loadMeta() {
		// Load SEO meta from API
		try {
			const response = await fetch(`/api/seo/meta/${entity.type}/${entity.id}`);
			const data = await response.json();
			if (data.data) {
				meta = { ...meta, ...data.data };
			}
		} catch (error) {
			console.error('Failed to load SEO meta:', error);
		}
	}

	async function saveMeta() {
		saving = true;
		try {
			const response = await fetch('/api/seo/meta', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					entity_type: entity.type,
					entity_id: entity.id,
					...meta
				})
			});

			if (response.ok) {
				onsaved?.();
			}
		} catch (error) {
			console.error('Failed to save SEO meta:', error);
		} finally {
			saving = false;
		}
	}

	function addKeyword() {
		const input = prompt('Enter keyword:');
		if (input && input.trim()) {
			meta.additional_keywords = [...meta.additional_keywords, input.trim()];
		}
	}

	function removeKeyword(index: number) {
		meta.additional_keywords = meta.additional_keywords.filter(
			(_: string, i: number) => i !== index
		);
	}
</script>

<div class="seo-meta-editor">
	<div class="editor-header">
		<h2>{entity.title}</h2>
		<div class="header-actions">
			<button class="btn-secondary" onclick={loadMeta}>
				<IconRefresh size={18} />
				Refresh
			</button>
			<button class="btn-primary" onclick={saveMeta} disabled={saving}>
				<IconDeviceFloppy size={18} />
				{saving ? 'Saving...' : 'Save Changes'}
			</button>
		</div>
	</div>

	<div class="editor-tabs">
		{#each tabs as tab}
			{@const TabIcon = tab.icon}
			<button class="tab" class:active={activeTab === tab.id} onclick={() => (activeTab = tab.id)}>
				<TabIcon size={18} />
				{tab.label}
			</button>
		{/each}
	</div>

	<div class="editor-content">
		{#if activeTab === 'general'}
			<div class="tab-panel">
				<div class="form-group">
					<label for="title">
						SEO Title
						<span class="char-count">{meta.title.length}/60</span>
					</label>
					<input
						id="title"
						type="text"
						bind:value={meta.title}
						placeholder="Enter SEO title..."
						maxlength="70"
					/>
					<div class="hint">Recommended: 50-60 characters</div>
				</div>

				<div class="form-group">
					<label for="description">
						Meta Description
						<span class="char-count">{meta.description.length}/160</span>
					</label>
					<textarea
						id="description"
						bind:value={meta.description}
						placeholder="Enter meta description..."
						rows="3"
						maxlength="170"
					></textarea>
					<div class="hint">Recommended: 150-160 characters</div>
				</div>

				<div class="form-group">
					<label for="focus-keyword">Focus Keyword</label>
					<input
						id="focus-keyword"
						type="text"
						bind:value={meta.focus_keyword}
						placeholder="Enter your primary keyword..."
					/>
				</div>

				<div class="form-group">
					<label for="additional-keywords">Additional Keywords</label>
					<div class="keywords-list" id="additional-keywords">
						{#each meta.additional_keywords as keyword, i}
							<span class="keyword-tag">
								{keyword}
								<button type="button" onclick={() => removeKeyword(i)}>×</button>
							</span>
						{/each}
						<button type="button" class="add-keyword" onclick={addKeyword}>+ Add Keyword</button>
					</div>
				</div>

				<div class="form-group">
					<label for="canonical">Canonical URL</label>
					<input
						id="canonical"
						type="url"
						bind:value={meta.canonical_url}
						placeholder="https://example.com/page"
					/>
				</div>

				<SeoPreview
					title={meta.title || entity.title}
					description={meta.description}
					url={meta.canonical_url || entity.url}
				/>
			</div>
		{:else if activeTab === 'social'}
			<div class="tab-panel">
				<h3><IconBrandFacebook size={20} /> Open Graph (Facebook)</h3>

				<div class="form-group">
					<label for="og-title">OG Title</label>
					<input
						id="og-title"
						type="text"
						bind:value={meta.og_title}
						placeholder="Leave empty to use SEO title"
					/>
				</div>

				<div class="form-group">
					<label for="og-description">OG Description</label>
					<textarea
						id="og-description"
						bind:value={meta.og_description}
						placeholder="Leave empty to use meta description"
						rows="2"
					></textarea>
				</div>

				<div class="form-group">
					<label for="og-image">OG Image URL</label>
					<input
						id="og-image"
						type="url"
						bind:value={meta.og_image}
						placeholder="https://example.com/image.jpg"
					/>
					<div class="hint">Recommended: 1200x630px</div>
				</div>

				<div class="form-group">
					<label for="og-type">OG Type</label>
					<select id="og-type" bind:value={meta.og_type}>
						<option value="website">Website</option>
						<option value="article">Article</option>
						<option value="product">Product</option>
					</select>
				</div>

				<hr />

				<h3><IconBrandTwitter size={20} /> Twitter Cards</h3>

				<div class="form-group">
					<label for="twitter-card">Card Type</label>
					<select id="twitter-card" bind:value={meta.twitter_card_type}>
						<option value="summary">Summary</option>
						<option value="summary_large_image">Summary with Large Image</option>
					</select>
				</div>

				<div class="form-group">
					<label for="twitter-title">Twitter Title</label>
					<input
						id="twitter-title"
						type="text"
						bind:value={meta.twitter_title}
						placeholder="Leave empty to use OG title"
					/>
				</div>

				<div class="form-group">
					<label for="twitter-description">Twitter Description</label>
					<textarea
						id="twitter-description"
						bind:value={meta.twitter_description}
						placeholder="Leave empty to use OG description"
						rows="2"
					></textarea>
				</div>

				<div class="form-group">
					<label for="twitter-image">Twitter Image URL</label>
					<input
						id="twitter-image"
						type="url"
						bind:value={meta.twitter_image}
						placeholder="Leave empty to use OG image"
					/>
				</div>
			</div>
		{:else if activeTab === 'advanced'}
			<div class="tab-panel">
				<h3><IconRobot size={20} /> Robots Meta</h3>

				<div class="checkbox-group">
					<label>
						<input type="checkbox" bind:checked={meta.noindex} />
						<div>
							<strong>NoIndex</strong>
							<span>Prevent search engines from indexing this page</span>
						</div>
					</label>

					<label>
						<input type="checkbox" bind:checked={meta.nofollow} />
						<div>
							<strong>NoFollow</strong>
							<span>Tell search engines not to follow links on this page</span>
						</div>
					</label>

					<label>
						<input type="checkbox" bind:checked={meta.noarchive} />
						<div>
							<strong>NoArchive</strong>
							<span>Prevent search engines from caching this page</span>
						</div>
					</label>

					<label>
						<input type="checkbox" bind:checked={meta.noimageindex} />
						<div>
							<strong>NoImageIndex</strong>
							<span>Prevent images from being indexed</span>
						</div>
					</label>

					<label>
						<input type="checkbox" bind:checked={meta.nosnippet} />
						<div>
							<strong>NoSnippet</strong>
							<span>Prevent showing snippets in search results</span>
						</div>
					</label>
				</div>

				<hr />

				<div class="form-group">
					<label for="breadcrumb">Breadcrumb Title</label>
					<input
						id="breadcrumb"
						type="text"
						bind:value={meta.breadcrumb_title}
						placeholder="Override breadcrumb title"
					/>
				</div>
			</div>
		{:else if activeTab === 'analysis'}
			<div class="tab-panel">
				<SeoAnalyzer
					{content}
					title={meta.title}
					description={meta.description}
					focusKeyword={meta.focus_keyword}
					additionalKeywords={meta.additional_keywords}
				/>
			</div>
		{/if}
	</div>
</div>

<style>
	.seo-meta-editor {
		background: white;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.editor-header {
		padding: 1.5rem 2rem;
		border-bottom: 1px solid #e5e5e5;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.editor-header h2 {
		font-size: 1.25rem;
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
		font-size: 0.95rem;
		cursor: pointer;
		transition: all 0.2s;
		border: none;
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

	.editor-tabs {
		display: flex;
		gap: 0.5rem;
		padding: 1rem 2rem 0;
		border-bottom: 2px solid #f0f0f0;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: none;
		border: none;
		color: #666;
		font-weight: 500;
		font-size: 0.95rem;
		cursor: pointer;
		border-bottom: 2px solid transparent;
		margin-bottom: -2px;
		transition: all 0.2s;
	}

	.tab:hover {
		color: #1a1a1a;
	}

	.tab.active {
		color: #3b82f6;
		border-bottom-color: #3b82f6;
	}

	.editor-content {
		flex: 1;
		overflow-y: auto;
		padding: 2rem;
	}

	.tab-panel {
		max-width: 800px;
	}

	.tab-panel h3 {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1.1rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 1.5rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-weight: 500;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.char-count {
		font-size: 0.85rem;
		color: #999;
		font-weight: 400;
	}

	.form-group input,
	.form-group textarea,
	.form-group select {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		font-size: 0.95rem;
		font-family: inherit;
		transition: border-color 0.2s;
	}

	.form-group input:focus,
	.form-group textarea:focus,
	.form-group select:focus {
		outline: none;
		border-color: #3b82f6;
	}

	.form-group textarea {
		resize: vertical;
	}

	.hint {
		margin-top: 0.375rem;
		font-size: 0.85rem;
		color: #999;
	}

	.keywords-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		min-height: 48px;
	}

	.keyword-tag {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.375rem 0.75rem;
		background: #eff6ff;
		color: #3b82f6;
		border-radius: 4px;
		font-size: 0.9rem;
	}

	.keyword-tag button {
		background: none;
		border: none;
		color: #3b82f6;
		font-size: 1.25rem;
		line-height: 1;
		cursor: pointer;
		padding: 0;
	}

	.add-keyword {
		padding: 0.375rem 0.75rem;
		background: white;
		border: 1px dashed #ccc;
		border-radius: 4px;
		color: #666;
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s;
	}

	.add-keyword:hover {
		border-color: #3b82f6;
		color: #3b82f6;
	}

	.checkbox-group label {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
		padding: 1rem;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		margin-bottom: 0.75rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.checkbox-group label:hover {
		background: #f8f9fa;
	}

	.checkbox-group input[type='checkbox'] {
		margin-top: 0.25rem;
		width: auto;
		cursor: pointer;
	}

	.checkbox-group label div {
		flex: 1;
	}

	.checkbox-group strong {
		display: block;
		color: #1a1a1a;
		margin-bottom: 0.25rem;
	}

	.checkbox-group span {
		display: block;
		font-size: 0.85rem;
		color: #666;
	}

	hr {
		border: none;
		border-top: 1px solid #e5e5e5;
		margin: 2rem 0;
	}
</style>

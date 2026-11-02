<script lang="ts">
	import { IconSearch } from '#lib/icons/index.js';

	interface Props {
		title?: string;
		description?: string;
		url?: string;
	}

	let { title = '', description = '', url = '' }: Props = $props();

	let displayUrl = $derived(url ? new URL(url).hostname + ' › ...' : 'example.com › page');
	let displayTitle = $derived(title || 'Untitled Page');
	let displayDescription = $derived(description || 'No description provided');
</script>

<div class="seo-preview">
	<div class="preview-header">
		<IconSearch size={16} />
		<span>Search Preview</span>
	</div>

	<div class="preview-content">
		<div class="google-preview">
			<div class="preview-url">{displayUrl}</div>
			<div class="preview-title">{displayTitle}</div>
			<div class="preview-description">{displayDescription}</div>
		</div>
	</div>

	<div class="preview-footer">
		<small>This is how your page might appear in Google search results</small>
	</div>
</div>

<style>
	.seo-preview {
		margin-top: 2rem;
		border: 1px solid var(--border-default, #30363d);
		border-radius: 8px;
		overflow: hidden;
	}

	.preview-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: var(--bg-elevated, #161b22);
		border-bottom: 1px solid var(--border-default, #30363d);
		font-weight: 500;
		font-size: 0.9rem;
		color: var(--text-secondary, #8b949e);
	}

	.preview-content {
		padding: 1.5rem;
		background: var(--bg-elevated, #161b22);
	}

	.google-preview {
		max-width: 600px;
	}

	.preview-url {
		font-size: 0.85rem;
		color: var(--info-emphasis, #58a6ff);
		margin-bottom: 0.25rem;
	}

	.preview-title {
		font-size: 1.25rem;
		color: var(--info-emphasis, #58a6ff);
		font-weight: 400;
		margin-bottom: 0.25rem;
		line-height: 1.3;
	}

	.preview-title:hover {
		text-decoration: underline;
	}

	.preview-description {
		font-size: 0.9rem;
		color: var(--text-secondary, #8b949e);
		line-height: 1.5;
	}

	.preview-footer {
		padding: 0.5rem 1rem;
		background: var(--bg-elevated, #161b22);
		border-top: 1px solid var(--border-default, #30363d);
		font-size: 0.8rem;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
	}
</style>

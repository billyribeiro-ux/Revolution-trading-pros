<script lang="ts">
	import { IconSearch } from '$lib/icons';

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
		border: 1px solid #e5e5e5;
		border-radius: 8px;
		overflow: hidden;
	}

	.preview-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: #f8f9fa;
		border-bottom: 1px solid #e5e5e5;
		font-weight: 500;
		font-size: 0.9rem;
		color: #666;
	}

	.preview-content {
		padding: 1.5rem;
		background: white;
	}

	.google-preview {
		max-width: 600px;
	}

	.preview-url {
		font-size: 0.85rem;
		color: #1a0dab;
		margin-bottom: 0.25rem;
	}

	.preview-title {
		font-size: 1.25rem;
		color: #1a0dab;
		font-weight: 400;
		margin-bottom: 0.25rem;
		line-height: 1.3;
	}

	.preview-title:hover {
		text-decoration: underline;
	}

	.preview-description {
		font-size: 0.9rem;
		color: #545454;
		line-height: 1.5;
	}

	.preview-footer {
		padding: 0.5rem 1rem;
		background: #f8f9fa;
		border-top: 1px solid #e5e5e5;
		font-size: 0.8rem;
		color: #999;
		text-align: center;
	}
</style>

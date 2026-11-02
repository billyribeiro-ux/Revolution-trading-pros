<script lang="ts">
	import type { PostState } from './types';

	type Props = {
		post: PostState;
		onGenerateSlug: () => void;
	};

	let { post = $bindable(), onGenerateSlug }: Props = $props();
</script>

<div class="form-group">
	<input autocomplete="off"
		id="page-post-title"
		name="page-post-title"
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
		<input autocomplete="off" id="slug" name="slug" type="text" bind:value={post.slug} placeholder="post-url-slug" />
		<button type="button" class="btn-regenerate" onclick={onGenerateSlug}>Regenerate</button>
	</div>
</div>

<div class="form-group">
	<label for="blog-excerpt">Excerpt</label>
	<textarea
		id="excerpt"
		bind:value={post.excerpt}
		placeholder="Brief description of the post..."
		rows="3"></textarea>
</div>

<style>
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
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-primary, #f0f6fc);
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
		color: var(--text-primary, #f0f6fc);
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
		color: var(--bg-base, #0d1117);
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
		color: var(--text-primary, #f0f6fc);
	}

	textarea:focus {
		outline: none;
		border-color: #3b82f6;
	}
</style>

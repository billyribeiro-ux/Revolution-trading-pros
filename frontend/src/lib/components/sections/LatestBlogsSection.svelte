<script lang="ts">
	import { IconArrowRight, IconClock, IconCalendar } from '@tabler/icons-svelte';
	import type { Post } from '$lib/types/post';

	// ─────────────────────────────────────────────────────────────────────────
	// Props
	// ─────────────────────────────────────────────────────────────────────────
	export let posts: Post[] = [];

	/**
	 * Format date to readable string
	 */
	function formatDate(dateString: string): string {
		try {
			const date = new Date(dateString);
			return date.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				year: 'numeric'
			});
		} catch {
			return dateString;
		}
	}

	/**
	 * Get default image if featured image is null
	 */
	function getImageUrl(imageUrl: string | null): string {
		return imageUrl || '/images/blog-placeholder.jpg';
	}

	/**
	 * Get reading time from post
	 */
	function getReadingTime(post: Post): string {
		// Try reading_time_text first (from appends), then reading_time
		if ('reading_time_text' in post && post.reading_time_text) {
			return post.reading_time_text as string;
		}
		if ('reading_time' in post && post.reading_time) {
			return `${post.reading_time} min read`;
		}
		return '5 min read';
	}
</script>

<section class="latest-blogs-section">
	<div class="section-container">
		<!-- Section Header -->
		<div class="section-header">
			<h2 class="section-title">Latest Blog Posts</h2>
			<a href="/blog" class="see-all-button">
				<span>See All Blogs</span>
				<IconArrowRight size={20} />
			</a>
		</div>

		<!-- Blog Posts Grid -->
		{#if posts.length > 0}
			<div class="blog-grid">
				{#each posts as post}
					<article class="blog-card">
						<a href="/blog/{post.slug}" class="card-link">
							<!-- Featured Image -->
							<div class="card-image">
								<img src={getImageUrl(post.featured_image)} alt={post.title} loading="lazy" />
								<div class="image-overlay"></div>
							</div>

							<!-- Card Content -->
							<div class="card-content">
								<!-- Metadata -->
								<div class="card-meta">
									<span class="meta-item">
										<IconCalendar size={16} />
										{formatDate(post.published_at)}
									</span>
									<span class="meta-item">
										<IconClock size={16} />
										{getReadingTime(post)}
									</span>
								</div>

								<!-- Title -->
								<h3 class="card-title">{post.title}</h3>

								<!-- Excerpt -->
								<p class="card-excerpt">{post.excerpt || 'Read more to discover...'}</p>

								<!-- Author and Date -->
								<div class="card-footer">
									<div class="author-meta">
										{#if post.author}
											<div class="author-avatar-small">
												{post.author.name.charAt(0).toUpperCase()}
											</div>
											<span class="author-name-small">{post.author.name}</span>
										{/if}
									</div>
									<span class="publish-date">{formatDate(post.published_at)}</span>
								</div>

								<!-- Read More -->
								<div class="read-more">
									<span>Read More</span>
									<IconArrowRight size={18} />
								</div>
							</div>
						</a>
					</article>
				{/each}
			</div>
		{/if}

		<!-- Empty State -->
		{#if posts.length === 0}
			<div class="empty-state">
				<p>No blog posts available yet. Check back soon!</p>
			</div>
		{/if}
	</div>
</section>

<style>
	.latest-blogs-section {
		position: relative;
		padding: 6rem 1.5rem;
		background: linear-gradient(135deg, #0a1628 0%, #050e1f 100%);
	}

	.section-container {
		max-width: 1280px;
		margin: 0 auto;
	}

	/* Section Header */
	.section-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 3rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.section-title {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		color: #ffffff;
		margin: 0;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.see-all-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background:
			linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0) 30%),
			rgba(46, 142, 255, 0.2);
		color: #ffffff;
		font-weight: 600;
		font-size: 1rem;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.3s ease;
		border: 1px solid rgba(46, 142, 255, 0.3);
	}

	.see-all-button:hover {
		background:
			linear-gradient(to bottom right, #2e8eff 0%, rgba(46, 142, 255, 0.3) 30%),
			rgba(46, 142, 255, 0.4);
		border-color: rgba(46, 142, 255, 0.5);
		transform: translateY(-2px);
		box-shadow: 0 0 20px rgba(46, 142, 255, 0.3);
	}

	/* Blog Grid - 3 per row */
	.blog-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 2rem;
	}

	/* Blog Card */
	.blog-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(46, 142, 255, 0.15);
		border-radius: 16px;
		overflow: hidden;
		transition: all 0.3s ease;
		backdrop-filter: blur(10px);
	}

	.blog-card:hover {
		border-color: rgba(46, 142, 255, 0.4);
		transform: translateY(-4px);
		box-shadow: 0 10px 40px rgba(46, 142, 255, 0.2);
	}

	.card-link {
		text-decoration: none;
		color: inherit;
		display: block;
	}

	/* Card Image */
	.card-image {
		position: relative;
		width: 100%;
		height: 200px;
		overflow: hidden;
	}

	.card-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		transition: transform 0.3s ease;
	}

	.blog-card:hover .card-image img {
		transform: scale(1.05);
	}

	.image-overlay {
		position: absolute;
		inset: 0;
		background: linear-gradient(to bottom, transparent, rgba(15, 23, 42, 0.8));
	}

	/* Card Content */
	.card-content {
		padding: 1.5rem;
	}

	.card-meta {
		display: flex;
		gap: 1rem;
		margin-bottom: 1rem;
		flex-wrap: wrap;
	}

	.meta-item {
		display: inline-flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.85rem;
		color: #94a3b8;
	}

	.card-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: #ffffff;
		margin: 0 0 0.75rem 0;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-excerpt {
		font-size: 0.95rem;
		color: #cbd5e1;
		line-height: 1.6;
		margin: 0 0 1rem 0;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(148, 163, 184, 0.1);
	}

	.author-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.author-avatar-small {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		background: linear-gradient(135deg, #2e8eff, #60a5fa);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.875rem;
		font-weight: 700;
		color: white;
	}

	.author-name-small {
		font-size: 0.875rem;
		font-weight: 600;
		color: #94a3b8;
	}

	.publish-date {
		font-size: 0.8rem;
		color: #64748b;
	}

	.read-more {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		color: #2e8eff;
		font-weight: 600;
		font-size: 0.95rem;
		transition: all 0.2s ease;
	}

	.blog-card:hover .read-more {
		gap: 0.75rem;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #94a3b8;
		font-size: 1.1rem;
	}

	/* Tablet - 2 per row */
	@media (max-width: 1024px) {
		.blog-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 1.5rem;
		}
	}

	/* Mobile - 1 per row */
	@media (max-width: 768px) {
		.latest-blogs-section {
			padding: 4rem 1rem;
		}

		.section-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.see-all-button {
			width: 100%;
			justify-content: center;
		}

		.blog-grid {
			grid-template-columns: 1fr;
			gap: 1.5rem;
		}

		.card-image {
			height: 180px;
		}
	}
</style>

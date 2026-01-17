<!--
	Platform Tutorials Page
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	Pixel-perfect match: frontend/Implementation/tutorials
	
	Svelte 5 / SvelteKit 2.0 Best Practices:
	- $props() rune for component props
	- $derived() rune for reactive computed values
	- Type imports from $types
	
	@version 1.0.0
-->
<script lang="ts">
	import type { PageData } from './$types';
	import type { Tutorial } from './+page.server';

	// Svelte 5 props with SvelteKit typing
	interface Props {
		data: PageData;
	}

	let { data }: Props = $props();

	// Derived state
	let tosTutorials = $derived(data.tosTutorials ?? []);
	let tradestationTutorials = $derived(data.tradestationTutorials ?? []);
	let pagination = $derived(data.tosPagination);

	// Generate pagination URL
	function getPageUrl(page: number): string {
		const url = new URL(window.location.href);
		url.searchParams.set('page', page.toString());
		return url.toString();
	}

	// Scroll to section
	function scrollToSection(sectionId: string): void {
		const element = document.getElementById(sectionId);
		if (element) {
			element.scrollIntoView({ behavior: 'smooth' });
		}
	}
</script>

<svelte:head>
	<title>Platform Tutorials - Revolution Trading Pros</title>
	<meta
		name="description"
		content="Tutorials, Tips and Platform Features for ThinkorSwim and TradeStation trading platforms."
	/>
</svelte:head>

<div class="fl-builder-content">
	<!-- Header Section -->
	<div class="fl-row fl-row-full-width fl-row-bg-color">
		<div class="fl-row-content-wrap">
			<div class="fl-row-content fl-row-fixed-width">
				<div class="fl-rich-text">
					<h1 class="page-title"><strong>Tutorials, Tips and Platform Features.</strong></h1>
				</div>
			</div>
		</div>
	</div>

	<!-- Platform Links -->
	<div class="fl-row fl-row-full-width fl-row-bg-color">
		<div class="fl-row-content-wrap">
			<div class="fl-row-content fl-row-fixed-width">
				<div class="fl-rich-text platform-links">
					<p>
						<button type="button" class="link-btn" onclick={() => scrollToSection('think')}
							>ThinkorSwim</button
						>
						<span class="separator">|</span>
						<button type="button" class="link-btn" onclick={() => scrollToSection('trade')}
							>TradeStation</button
						>
					</p>
				</div>
			</div>
		</div>
	</div>

	<!-- ThinkorSwim Tutorials Section -->
	<div class="fl-row fl-row-full-width">
		<div class="fl-row-content-wrap">
			<div class="fl-row-content fl-row-fixed-width">
				<div class="fl-rich-text">
					<h1 class="section-title"><strong>ThinkorSwim Tutorials</strong></h1>
				</div>
				<div class="fl-separator"></div>

				<div id="think" class="fl-post-grid">
					{#each tosTutorials as tutorial (tutorial.id)}
						<div class="fl-post-grid-post">
							<div class="fl-post-image">
								<a href={tutorial.url} title={tutorial.title}>
									<img
										loading="lazy"
										decoding="async"
										src={tutorial.thumbnail}
										alt={tutorial.title}
										width="1024"
										height="576"
									/>
								</a>
							</div>
							<div class="fl-post-text">
								<h2 class="fl-post-title">
									<a href={tutorial.url} title={tutorial.title}>{tutorial.title}</a>
								</h2>
								<div class="fl-post-excerpt">
									<p>{tutorial.excerpt}</p>
								</div>
								<div class="fl-post-more-link">
									<a href={tutorial.url} title="Read More...">Read More...</a>
								</div>
							</div>
						</div>
					{/each}
					<div class="fl-post-grid-sizer"></div>
				</div>

				<!-- TOS Pagination -->
				{#if pagination.totalPages > 1}
					<div class="fl-builder-pagination">
						<ul class="page-numbers">
							{#each Array.from({ length: pagination.totalPages }, (_, i) => i + 1) as pageNum}
								{#if pageNum === pagination.page}
									<li><span aria-current="page" class="page-numbers current">{pageNum}</span></li>
								{:else}
									<li><a class="page-numbers" href={getPageUrl(pageNum)}>{pageNum}</a></li>
								{/if}
							{/each}
							{#if pagination.page < pagination.totalPages}
								<li>
									<a class="next page-numbers" href={getPageUrl(pagination.page + 1)}
										>Next &raquo;</a
									>
								</li>
							{/if}
						</ul>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- TradeStation Tutorials Section -->
	<div class="fl-row fl-row-full-width">
		<div class="fl-row-content-wrap">
			<div class="fl-row-content fl-row-fixed-width">
				<div class="fl-rich-text">
					<h1 class="section-title"><strong>TradeStation Tutorials</strong></h1>
				</div>
				<div class="fl-separator"></div>

				<div id="trade" class="fl-post-grid">
					{#each tradestationTutorials as tutorial (tutorial.id)}
						<div class="fl-post-grid-post">
							<div class="fl-post-image">
								<a href={tutorial.url} title={tutorial.title}>
									<img
										loading="lazy"
										decoding="async"
										src={tutorial.thumbnail}
										alt={tutorial.title}
										width="1024"
										height="576"
									/>
								</a>
							</div>
							<div class="fl-post-text">
								<h2 class="fl-post-title">
									<a href={tutorial.url} title={tutorial.title}>{tutorial.title}</a>
								</h2>
								<div class="fl-post-excerpt">
									<p>{tutorial.excerpt}</p>
								</div>
								<div class="fl-post-more-link">
									<a href={tutorial.url} title="Read More...">Read More...</a>
								</div>
							</div>
						</div>
					{/each}
					<div class="fl-post-grid-sizer"></div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * PLATFORM TUTORIALS - Pixel-Perfect WordPress Match
	 * Matches: frontend/Implementation/tutorials
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.fl-builder-content {
		max-width: 100%;
		margin: 0 auto;
	}

	/* Row Styles */
	.fl-row {
		margin-left: auto;
		margin-right: auto;
	}

	.fl-row-full-width {
		width: 100%;
	}

	.fl-row-bg-color {
		background-color: #f7f7f7;
	}

	.fl-row-content-wrap {
		padding: 20px 0;
	}

	.fl-row-content {
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.fl-row-fixed-width {
		width: 100%;
	}

	/* Page Title */
	.page-title {
		text-align: center;
		font-size: 30px;
		font-weight: 700;
		color: #333;
		margin: 0;
		padding: 20px 0;
	}

	/* Platform Links */
	.platform-links {
		text-align: center;
		margin-bottom: 20px;
	}

	.platform-links p {
		margin: 0;
		font-size: 16px;
	}

	.link-btn {
		background: none;
		border: none;
		color: #0984ae;
		font-size: 16px;
		cursor: pointer;
		padding: 0;
		text-decoration: none;
	}

	.link-btn:hover {
		text-decoration: underline;
	}

	.separator {
		margin: 0 10px;
		color: #666;
	}

	/* Section Title */
	.section-title {
		text-align: center;
		letter-spacing: 2px;
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 30px 0 10px;
	}

	/* Separator */
	.fl-separator {
		border-top: 1px solid #e0e0e0;
		margin: 15px auto 30px;
		max-width: 100px;
	}

	/* Post Grid */
	.fl-post-grid {
		display: grid;
		grid-template-columns: repeat(1, 1fr);
		gap: 30px;
		margin-bottom: 40px;
	}

	@media (min-width: 576px) {
		.fl-post-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 992px) {
		.fl-post-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.fl-post-grid-post {
		background: #fff;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		transition:
			box-shadow 0.3s ease,
			transform 0.2s ease;
	}

	.fl-post-grid-post:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.fl-post-grid-sizer {
		display: none;
	}

	/* Post Image */
	.fl-post-image {
		position: relative;
		overflow: hidden;
	}

	.fl-post-image a {
		display: block;
	}

	.fl-post-image img {
		width: 100%;
		height: auto;
		display: block;
		aspect-ratio: 16 / 9;
		object-fit: cover;
		transition: transform 0.3s ease;
	}

	.fl-post-grid-post:hover .fl-post-image img {
		transform: scale(1.05);
	}

	/* Post Text */
	.fl-post-text {
		padding: 20px;
	}

	.fl-post-title {
		font-size: 18px;
		font-weight: 600;
		line-height: 1.4;
		margin: 0 0 12px;
	}

	.fl-post-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.fl-post-title a:hover {
		color: #0984ae;
	}

	.fl-post-excerpt {
		margin-bottom: 15px;
	}

	.fl-post-excerpt p {
		font-size: 14px;
		line-height: 1.6;
		color: #666;
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.fl-post-more-link a {
		font-size: 14px;
		color: #0984ae;
		text-decoration: none;
		font-weight: 500;
	}

	.fl-post-more-link a:hover {
		text-decoration: underline;
	}

	/* Pagination */
	.fl-builder-pagination {
		text-align: center;
		margin: 30px 0 40px;
	}

	.page-numbers {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		justify-content: center;
		gap: 5px;
		flex-wrap: wrap;
	}

	.page-numbers li {
		display: inline-block;
	}

	.page-numbers a,
	.page-numbers span {
		display: inline-block;
		padding: 10px 15px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.page-numbers a {
		background: #f4f4f4;
		color: #666;
	}

	.page-numbers a:hover {
		background: #0984ae;
		color: #fff;
	}

	.page-numbers .current {
		background: #0984ae;
		color: #fff;
	}

	.page-numbers .next {
		background: #0984ae;
		color: #fff;
	}

	.page-numbers .next:hover {
		background: #076a8a;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.page-title {
			font-size: 24px;
		}

		.section-title {
			font-size: 22px;
		}

		.fl-post-title {
			font-size: 16px;
		}
	}
</style>

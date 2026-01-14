<!--
	LatestUpdates Component - Reusable Dashboard Section
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation - Svelte 5
	
	Displays a grid of article/video cards for any trading room dashboard.
	Supports both static data (items prop) and dynamic API fetching (apiEndpoint prop).
	
	@version 2.0.0 - January 2026 - Svelte 5 + API-driven
	@author Revolution Trading Pros
	@svelte5 Fully compliant with Nov/Dec 2025 best practices
-->
<script lang="ts">
	import { browser } from '$app/environment';

	/**
	 * Article/Update item structure
	 */
	export interface UpdateItem {
		id: number | string;
		type: string;
		title: string;
		date: string;
		excerpt: string;
		href: string;
		image: string;
		isVideo?: boolean;
	}

	interface Props {
		/** Array of update items to display (for static data) */
		items?: UpdateItem[];
		/** API endpoint to fetch updates from (for dynamic data) */
		apiEndpoint?: string;
		/** Room slug for the API endpoint */
		roomSlug?: string;
		/** Section title - defaults to "Latest Updates" */
		title?: string;
		/** Button text for cards - defaults to "Watch Now" */
		buttonText?: string;
		/** Maximum number of items to display */
		maxItems?: number;
	}

	let { 
		items = [], 
		apiEndpoint,
		roomSlug = '',
		title = 'Latest Updates',
		buttonText = 'Watch Now',
		maxItems = 6
	}: Props = $props();

	// Svelte 5 $state for dynamic data
	let dynamicItems = $state<UpdateItem[]>([]);
	let isLoading = $state(false);
	let error = $state<string | null>(null);

	// Svelte 5 $derived - compute final items to display
	let displayItems = $derived(
		apiEndpoint && dynamicItems.length > 0 
			? dynamicItems.slice(0, maxItems)
			: items.slice(0, maxItems)
	);

	// Svelte 5 $effect - fetch from API if endpoint provided
	$effect(() => {
		if (browser && apiEndpoint && roomSlug) {
			isLoading = true;
			error = null;

			fetch(apiEndpoint)
				.then(res => res.ok ? res.json() : Promise.reject('Failed to fetch'))
				.then(data => {
					dynamicItems = data.data || data.items || [];
					isLoading = false;
				})
				.catch(err => {
					console.error('LatestUpdates fetch error:', err);
					error = 'Failed to load updates';
					isLoading = false;
				});
		}
	});
</script>

<section class="latest-updates">
	<h2 class="section-title">{title}</h2>
	
	{#if isLoading}
		<div class="empty-state">
			<p>Loading updates...</p>
		</div>
	{:else if error}
		<div class="empty-state error">
			<p>{error}</p>
		</div>
	{:else if displayItems.length === 0}
		<div class="empty-state">
			<p>No updates available at this time.</p>
		</div>
	{:else}
		<div class="updates-grid">
			{#each displayItems as item (item.id)}
				<div class="updates-grid__item">
					<article class="article-card">
						<figure class="article-card__image" style="background-image: url({item.image});">
							<img src={item.image} alt={item.title} loading="lazy" />
						</figure>
						
						{#if item.isVideo && item.type}
							<div class="article-card__type">
								<span class="label label--info">{item.type}</span>
							</div>
						{/if}
						
						<h4 class="h5 article-card__title">
							<a href={item.href}>{item.title}</a>
						</h4>
						
						<span class="article-card__meta">
							<small>{item.date}</small>
						</span>
						
						<div class="article-card__excerpt u--hide-read-more">
							<p>{item.excerpt}</p>
						</div>
						
						<a href={item.href} class="btn btn-tiny btn-default">{buttonText}</a>
					</article>
				</div>
			{/each}
		</div>
	{/if}
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * LATEST UPDATES SECTION - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.latest-updates {
		padding: 30px 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	@media screen and (min-width: 1280px) {
		.latest-updates {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.latest-updates {
			padding: 40px;
		}
	}

	.section-title {
		color: #333;
		font-weight: 700;
		font-size: 20px;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * UPDATES GRID - Responsive 3-Column Layout
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.updates-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 1.25rem;
	}

	@media screen and (min-width: 768px) {
		.updates-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media screen and (min-width: 1024px) {
		.updates-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.updates-grid__item {
		display: flex;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * ARTICLE CARD - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.article-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08), 0 4px 20px rgba(0, 0, 0, 0.06);
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		border: 1px solid rgba(0, 0, 0, 0.04);
		will-change: transform, box-shadow;
	}

	.article-card:hover {
		box-shadow: 
			0 10px 20px rgba(0, 0, 0, 0.12),
			0 20px 40px rgba(0, 0, 0, 0.08),
			0 30px 60px rgba(0, 0, 0, 0.05);
		transform: translateY(-4px) scale(1.02);
		border-color: rgba(20, 62, 89, 0.15);
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		margin: 0;
		overflow: hidden;
		transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1);
		will-change: transform;
	}

	.article-card:hover .article-card__image {
		transform: scale(1.08);
	}

	.article-card__image::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: linear-gradient(to bottom, transparent 50%, rgba(0, 0, 0, 0.1) 100%);
		opacity: 0;
		transition: opacity 0.4s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.article-card:hover .article-card__image::after {
		opacity: 1;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__type {
		padding: 12px 20px 0;
	}

	.label {
		display: inline-block;
		padding: 4px 12px;
		border-radius: 3px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.label--info {
		background: #143E59;
		color: #fff;
	}

	.article-card__title {
		padding: 12px 20px 0;
		margin: 0;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		font-size: 18px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.4;
		transition: color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.article-card:hover .article-card__title a {
		color: #143E59;
	}

	.article-card__title a:hover {
		color: #0c2638;
	}

	.h5 {
		font-size: 18px;
		font-weight: 600;
	}

	.article-card__meta {
		display: block;
		padding: 8px 20px 0;
		color: #999;
		font-size: 13px;
	}

	.article-card__excerpt {
		padding: 12px 20px;
		color: #666;
		font-size: 14px;
		line-height: 1.6;
		flex-grow: 1;
	}

	.article-card__excerpt p {
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.u--hide-read-more {
		display: block;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BUTTON STYLES - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.article-card .btn {
		margin: 0 20px 20px;
		display: block;
		width: calc(100% - 40px);
		text-align: center;
	}

	.btn {
		display: inline-block;
		text-decoration: none;
		border-radius: 5px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease-in-out;
		text-align: center;
		cursor: pointer;
		border: none;
	}

	.btn-tiny {
		padding: 10px 24px;
		font-size: 13px;
		min-width: 120px;
		height: 40px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.btn-default {
		background-color: #143E59;
		color: #fff;
		border: 1px solid #143E59;
		box-shadow: none;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		will-change: transform, box-shadow;
	}

	.btn-default:hover {
		background-color: #0c2638;
		border-color: #0c2638;
		box-shadow: 0 6px 16px rgba(20, 62, 89, 0.4);
		transform: translateY(-2px);
	}

	.article-card:hover .btn-default {
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.25);
		transform: translateY(-1px);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * EMPTY STATE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.empty-state {
		text-align: center;
		padding: 3rem 2rem;
		color: #666;
		background: #f9f9f9;
		border-radius: 5px;
	}

	.empty-state p {
		font-size: 1rem;
		margin: 0;
	}

	.empty-state.error {
		background: #fff3f3;
		border: 1px solid #ffcdd2;
		color: #c62828;
	}
</style>

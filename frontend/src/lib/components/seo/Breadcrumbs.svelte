<script lang="ts">
	/**
	 * Visible Accessible Breadcrumbs (`SD-03`, `SD-05`)
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Renders a visible `<nav aria-label="Breadcrumb"><ol>` matching the
	 * `BreadcrumbList` JSON-LD emitted by `resolveSEO` for all non-home
	 * indexable marketing routes.
	 */
	import { buildBreadcrumbsForPath } from '#lib/seo/config.js';
	import { PRODUCTION_ORIGIN } from '#lib/seo/config.js';

	interface Props {
		pathname: string;
		leafTitle?: string;
	}

	let props: Props = $props();

	const crumbs = $derived(buildBreadcrumbsForPath(props.pathname, props.leafTitle));

	function toRelativeHref(url: string): string {
		if (url === `${PRODUCTION_ORIGIN}/` || url === PRODUCTION_ORIGIN) {
			return '/';
		}
		if (url.startsWith(PRODUCTION_ORIGIN)) {
			return url.slice(PRODUCTION_ORIGIN.length) || '/';
		}
		return url;
	}
</script>

{#if crumbs.length >= 2}
	<nav aria-label="Breadcrumb" class="rtp-breadcrumbs-bar">
		<ol class="rtp-breadcrumbs-list">
			{#each crumbs as crumb, idx (crumb.url)}
				<li class="rtp-breadcrumbs-item">
					{#if idx < crumbs.length - 1}
						<a href={toRelativeHref(crumb.url)} class="rtp-breadcrumbs-link">
							{crumb.name}
						</a>
						<span class="rtp-breadcrumbs-sep" aria-hidden="true">/</span>
					{:else}
						<span aria-current="page" class="rtp-breadcrumbs-current">
							{crumb.name}
						</span>
					{/if}
				</li>
			{/each}
		</ol>
	</nav>
{/if}

<style>
	.rtp-breadcrumbs-bar {
		width: 100%;
		max-width: 1280px;
		margin: 0 auto;
		padding: 0.625rem 1.5rem;
		font-size: 0.8125rem;
		line-height: 1.4;
		color: #cbd5e1;
	}

	.rtp-breadcrumbs-list {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.375rem;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.rtp-breadcrumbs-item {
		display: inline-flex;
		align-items: center;
		gap: 0.375rem;
	}

	.rtp-breadcrumbs-link {
		color: #94a3b8;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.rtp-breadcrumbs-link:hover,
	.rtp-breadcrumbs-link:focus-visible {
		color: #f8fafc;
		text-decoration: underline;
	}

	.rtp-breadcrumbs-sep {
		color: #64748b;
		user-select: none;
	}

	.rtp-breadcrumbs-current {
		color: #f1f5f9;
		font-weight: 500;
	}
</style>


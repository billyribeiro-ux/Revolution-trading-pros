<script lang="ts">
	/**
	 * DashboardBreadcrumbs - Simpler Trading Style
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Svelte 5 component for breadcrumb navigation:
	 * - Light gray background (#f5f5f5)
	 * - Blue hover on links (#0984ae)
	 * - Auto-generates from URL or accepts custom crumbs
	 *
	 * @version 1.0.0 - Svelte 5 with $props()
	 */
	import { page } from '$app/stores';

	interface Breadcrumb {
		label: string;
		href: string;
		isCurrent?: boolean;
	}

	interface Props {
		/** Custom breadcrumbs (if not provided, auto-generates from URL) */
		crumbs?: Breadcrumb[];
		/** Title mappings for slugs */
		titleMap?: Record<string, string>;
	}

	// Default title mappings
	const defaultTitleMap: Record<string, string> = {
		'dashboard': 'Member Dashboard',
		'day-trading-room': 'Day Trading Room',
		'swing-trading-room': 'Swing Trading Room',
		'small-account-mentorship': 'Small Account Mentorship',
		'options-day-trading': 'Options Day Trading',
		'simpler-showcase': 'Simpler Showcase',
		'spx-profit-pulse': 'SPX Profit Pulse',
		'explosive-swing': 'Explosive Swing',
		'ww': 'Weekly Watchlist',
		'weekly-watchlist': 'Weekly Watchlist',
		'courses': 'Courses',
		'classes': 'My Classes',
		'indicators': 'My Indicators',
		'alerts': 'Alerts',
		'account': 'Account',
		'learning-center': 'Learning Center',
		'start-here': 'Start Here',
		'resources': 'Resources',
		'daily-videos': 'Premium Daily Videos',
		'trading-room-archive': 'Trading Room Archives',
		'traders': 'Meet the Traders',
		'trader-store': 'Trader Store'
	};

	let { crumbs, titleMap = {} }: Props = $props();

	// Merge custom title map with defaults
	let mergedTitleMap = $derived({ ...defaultTitleMap, ...titleMap });

	// Auto-generate breadcrumbs from URL if not provided
	let breadcrumbs = $derived.by(() => {
		if (crumbs) return crumbs;

		const pathname = $page.url.pathname;
		const segments = pathname.split('/').filter(Boolean);

		const result: Breadcrumb[] = [
			{ label: 'Home', href: '/', isCurrent: false }
		];

		let currentPath = '';
		segments.forEach((segment, index) => {
			currentPath += `/${segment}`;
			const isLast = index === segments.length - 1;

			let label = mergedTitleMap[segment];
			if (!label) {
				// Convert slug to title case
				label = segment.split('-').map(word =>
					word.charAt(0).toUpperCase() + word.slice(1)
				).join(' ');
			}

			result.push({
				label,
				href: currentPath,
				isCurrent: isLast
			});
		});

		return result;
	});
</script>

<nav class="breadcrumbs" aria-label="Breadcrumb">
	<div class="breadcrumbs__container">
		<ul>
			{#each breadcrumbs as crumb, index (crumb.href)}
				{#if index > 0}
					<li class="breadcrumbs__separator" aria-hidden="true"> / </li>
				{/if}
				<li class="breadcrumbs__item">
					{#if crumb.isCurrent}
						<strong class="breadcrumbs__current" aria-current="page">{crumb.label}</strong>
					{:else}
						<a class="breadcrumbs__link" href={crumb.href}>{crumb.label}</a>
					{/if}
				</li>
			{/each}
		</ul>
	</div>
</nav>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   BREADCRUMBS - Exact Simpler Trading Match
	   ═══════════════════════════════════════════════════════════════════════════ */
	.breadcrumbs {
		width: 100%;
		background-color: #f5f5f5;
		border-bottom: 1px solid #e5e5e5;
	}

	.breadcrumbs__container {
		max-width: 100%;
		padding: 10px 15px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		font-size: 13px;
		line-height: 1.4;
		font-family: 'Open Sans', sans-serif;
	}

	.breadcrumbs__item {
		display: inline;
	}

	.breadcrumbs__link {
		color: #666;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
	}

	.breadcrumbs__link:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.breadcrumbs__separator {
		margin: 0 8px;
		color: #999;
	}

	.breadcrumbs__current {
		color: #333;
		font-weight: 600;
	}
</style>

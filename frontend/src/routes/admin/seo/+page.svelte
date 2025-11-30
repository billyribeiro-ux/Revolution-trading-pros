<script lang="ts">
	import {
		IconFileText,
		IconArrowForward,
		IconExternalLink,
		IconError404,
		IconCode,
		IconKey,
		IconChartBar,
		IconSettings,
		IconSitemap,
		IconBrandGoogle,
		IconNews,
		IconVideo,
		IconPhoto,
		IconMapPin,
		IconMail
	} from '@tabler/icons-svelte';

	const sections = [
		{
			title: 'SEO Meta',
			description: 'Manage meta tags, Open Graph, and Twitter Cards for all content',
			icon: IconFileText,
			href: '/admin/seo/meta',
			color: 'blue'
		},
		{
			title: 'Redirects',
			description: 'Create and manage 301/302 redirects with regex support',
			icon: IconArrowForward,
			href: '/admin/seo/redirects',
			color: 'green'
		},
		{
			title: '404 Monitor',
			description: 'Track 404 errors and convert them to redirects',
			icon: IconError404,
			href: '/admin/seo/404-monitor',
			color: 'red'
		},
		{
			title: 'Schema Markup',
			description: 'Generate JSON-LD structured data for better search visibility',
			icon: IconCode,
			href: '/admin/seo/schema',
			color: 'purple'
		},
		{
			title: 'Keywords',
			description: 'Track keyword rankings and monitor performance',
			icon: IconKey,
			href: '/admin/seo/keywords',
			color: 'orange'
		},
		{
			title: 'Analytics',
			description: 'View search performance metrics and trends',
			icon: IconChartBar,
			href: '/admin/seo/analytics',
			color: 'teal'
		},
		{
			title: 'Sitemap',
			description: 'Generate and submit XML sitemaps to search engines',
			icon: IconSitemap,
			href: '/admin/seo/sitemap',
			color: 'indigo'
		},
		{
			title: 'News Sitemap',
			description: 'Google News sitemap with 48-hour content window and stock tickers',
			icon: IconNews,
			href: '/admin/seo/news-sitemap',
			color: 'rose',
			isNew: true
		},
		{
			title: 'Video Sitemap',
			description: 'Video sitemap with YouTube, Vimeo, Dailymotion auto-detection',
			icon: IconVideo,
			href: '/admin/seo/video-sitemap',
			color: 'crimson',
			isNew: true
		},
		{
			title: 'Image SEO',
			description: 'Auto alt text, title generation, and case transformation',
			icon: IconPhoto,
			href: '/admin/seo/image-seo',
			color: 'emerald',
			isNew: true
		},
		{
			title: 'Store Locator',
			description: 'Multi-location SEO with LocalBusiness schema and KML export',
			icon: IconMapPin,
			href: '/admin/seo/store-locator',
			color: 'amber',
			isNew: true
		},
		{
			title: 'SEO Reports',
			description: 'Automated email reports with scheduling and white-label branding',
			icon: IconMail,
			href: '/admin/seo/reports',
			color: 'sky',
			isNew: true
		},
		{
			title: 'Search Console',
			description: 'Connect with Google Search Console for data integration',
			icon: IconBrandGoogle,
			href: '/admin/seo/search-console',
			color: 'pink'
		},
		{
			title: 'Settings',
			description: 'Configure SEO plugin settings and preferences',
			icon: IconSettings,
			href: '/admin/seo/settings',
			color: 'gray'
		}
	];
</script>

<svelte:head>
	<title>SEO Dashboard | Admin</title>
</svelte:head>

<div class="seo-dashboard">
	<header class="dashboard-header">
		<div>
			<h1>SEO Dashboard</h1>
			<p>Comprehensive SEO tools to optimize your site's search visibility</p>
		</div>
	</header>

	<div class="sections-grid">
		{#each sections as section}
			<a href={section.href} class="section-card {section.color}">
				{#if section.isNew}
					<span class="new-badge">NEW</span>
				{/if}
				<div class="card-icon">
					<svelte:component this={section.icon} size={32} />
				</div>
				<h3>{section.title}</h3>
				<p>{section.description}</p>
				<div class="card-arrow">
					<IconExternalLink size={20} />
				</div>
			</a>
		{/each}
	</div>
</div>

<style>
	.seo-dashboard {
		padding: 2rem;
		max-width: 1400px;
		margin: 0 auto;
	}

	.dashboard-header {
		margin-bottom: 3rem;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.dashboard-header h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.dashboard-header p {
		color: #666;
		font-size: 1.1rem;
	}

	.sections-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.5rem;
	}

	.section-card {
		position: relative;
		display: block;
		padding: 2rem;
		background: white;
		border-radius: 12px;
		border: 2px solid #e5e5e5;
		text-decoration: none;
		transition: all 0.3s ease;
		overflow: hidden;
	}

	.section-card::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 4px;
		background: currentColor;
		transform: scaleX(0);
		transform-origin: left;
		transition: transform 0.3s ease;
	}

	.section-card:hover::before {
		transform: scaleX(1);
	}

	.section-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 12px 24px rgba(0, 0, 0, 0.1);
		border-color: currentColor;
	}

	.card-icon {
		margin-bottom: 1rem;
		opacity: 0.8;
	}

	.section-card h3 {
		font-size: 1.25rem;
		font-weight: 600;
		color: #1a1a1a;
		margin-bottom: 0.5rem;
	}

	.section-card p {
		color: #666;
		font-size: 0.95rem;
		line-height: 1.6;
		margin-bottom: 0;
	}

	.card-arrow {
		position: absolute;
		top: 2rem;
		right: 2rem;
		opacity: 0;
		transform: translateX(-10px);
		transition: all 0.3s ease;
	}

	.section-card:hover .card-arrow {
		opacity: 0.6;
		transform: translateX(0);
	}

	/* NEW badge */
	.new-badge {
		position: absolute;
		top: 1rem;
		right: 1rem;
		background: linear-gradient(135deg, #f97316, #ea580c);
		color: white;
		font-size: 0.65rem;
		font-weight: 700;
		padding: 0.25rem 0.5rem;
		border-radius: 4px;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		animation: pulse-badge 2s ease-in-out infinite;
	}

	@keyframes pulse-badge {
		0%, 100% { transform: scale(1); }
		50% { transform: scale(1.05); }
	}

	/* Color variants */
	.section-card.blue {
		color: #3b82f6;
	}
	.section-card.green {
		color: #10b981;
	}
	.section-card.red {
		color: #ef4444;
	}
	.section-card.purple {
		color: #8b5cf6;
	}
	.section-card.orange {
		color: #f59e0b;
	}
	.section-card.teal {
		color: #14b8a6;
	}
	.section-card.indigo {
		color: #6366f1;
	}
	.section-card.pink {
		color: #ec4899;
	}
	.section-card.gray {
		color: #6b7280;
	}
	.section-card.rose {
		color: #f43f5e;
	}
	.section-card.crimson {
		color: #dc2626;
	}
	.section-card.emerald {
		color: #059669;
	}
	.section-card.amber {
		color: #d97706;
	}
	.section-card.sky {
		color: #0284c7;
	}

	@media (max-width: 768px) {
		.seo-dashboard {
			padding: 1rem;
		}

		.dashboard-header h1 {
			font-size: 1.5rem;
		}

		.sections-grid {
			grid-template-columns: 1fr;
			gap: 1rem;
		}

		.section-card {
			padding: 1.5rem;
		}
	}
</style>

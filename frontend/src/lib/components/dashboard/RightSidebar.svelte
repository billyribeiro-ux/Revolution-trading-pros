<!--
	RightSidebar Component - Reusable Dashboard Right Sidebar
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	Svelte 5 Best Practices - Latest Nov/Dec 2024 Syntax
	
	SSOT for dashboard right sidebars (right panel).
	Used across Day Trading Room, Weekly Watchlist, etc.
	
	@version 1.0.0 - January 2026 - Extracted from Day Trading Room
-->
<script lang="ts">
	interface SidebarSection {
		heading: string;
		subheading?: string;
		content: 'schedule' | 'links' | 'custom';
		links?: Array<{ text: string; href: string; external?: boolean }>;
		customContent?: any;
	}

	interface Props {
		/** Array of sidebar sections to display */
		sections: SidebarSection[];
		/** Optional CSS class for container */
		className?: string;
	}

	let props: Props = $props();

	// Derived props with defaults
	let sections = $derived(props.sections ?? []);
	let className = $derived(props.className ?? '');
</script>

<aside class="dashboard__content-sidebar {className}">
	{#each sections as section}
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				{#if section.heading.includes('Weekly Watchlist Schedule')}
					<div class="text-center">{section.heading}</div>
				{:else}
					{section.heading}
				{/if}
				{#if section.subheading}
					<p class="pssubject text-center">{section.subheading}</p>
				{/if}
			</h4>

			{#if section.content === 'schedule'}
				<div class="script-container">
					<div class="room-sched"></div>
				</div>
			{:else if section.content === 'links' && section.links}
				<ul class="link-list">
					{#each section.links as link}
						<li>
							<a href={link.href} target={link.external ? '_blank' : '_self'}>
								{link.text}
							</a>
						</li>
					{/each}
				</ul>
			{:else if section.content === 'custom' && section.customContent}
				{@render section.customContent()}
			{/if}
		</section>
	{/each}
</aside>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * SIDEBAR - WordPress Exact Match from Day Trading Room
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content-sidebar {
		display: block;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		background: #fff;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
		font-family: 'Open Sans', sans-serif;
		font-size: 14px;
		line-height: 1.6;
	}

	/* Mobile-first: sidebar hidden by default, shown on lg (1024px+) */
	.dashboard__content-sidebar {
		display: none;
	}

	@media (min-width: 1080px) {
		.dashboard__content-sidebar {
			display: block;
		}
	}

	.content-sidebar__section {
		padding: 20px 30px 20px 20px;
		border-bottom: 1px solid #dbdbdb;
	}

	.content-sidebar__heading {
		padding: 15px 20px;
		margin: -20px -30px 20px -20px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #333;
		background: #ededed;
		border-bottom: 1px solid #dbdbdb;
		line-height: 1.4;
	}

	.pssubject {
		font-size: 10px;
		margin-top: 15px;
		text-transform: initial;
	}

	.script-container {
		margin: 0;
	}

	.room-sched {
		margin: 0;
	}

	/* Text center for Weekly Watchlist heading */
	.text-center {
		text-align: center;
	}

	.link-list {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.link-list li {
		margin-bottom: 12px;
	}

	.link-list a {
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: color 0.2s;
	}

	.link-list a:hover {
		color: #076787;
		text-decoration: underline;
	}
</style>

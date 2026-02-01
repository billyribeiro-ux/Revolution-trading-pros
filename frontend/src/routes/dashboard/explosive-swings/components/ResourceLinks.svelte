<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ResourceLinks Component - Quick Resources Sidebar Section
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Quick access links for common resources
	 * @version 4.1.0 - Visual Polish Pass
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */

	interface ResourceLink {
		icon: string;
		label: string;
		href: string;
		external?: boolean;
		description?: string;
	}

	interface Props {
		variant?: 'default' | 'compact' | 'card';
	}

	const { variant = 'default' }: Props = $props();

	const resources: ResourceLink[] = [
		{
			icon: 'video',
			label: 'Video Library',
			href: '/dashboard/explosive-swings/video-library',
			description: 'All trade breakdowns'
		},
		{
			icon: 'chart',
			label: 'Trade Tracker',
			href: '/dashboard/explosive-swings/trades',
			description: 'Performance history'
		},
		{
			icon: 'star',
			label: 'My Favorites',
			href: '/dashboard/explosive-swings/favorites',
			description: 'Saved alerts & videos'
		},
		{
			icon: 'download',
			label: 'Export CSV',
			href: '/dashboard/explosive-swings/export',
			description: 'Download trade data'
		},
		{
			icon: 'settings',
			label: 'Alert Settings',
			href: '/dashboard/explosive-swings/settings',
			description: 'Notification preferences'
		}
	];
</script>

<div
	class="resource-links"
	class:compact={variant === 'compact'}
	class:card-style={variant === 'card'}
	role="navigation"
	aria-labelledby="resources-heading"
>
	<div class="section-header">
		<h3 id="resources-heading" class="section-title">
			<span class="title-icon" aria-hidden="true">
				<svg viewBox="0 0 20 20" fill="currentColor">
					<path
						d="M7 3.5A1.5 1.5 0 018.5 2h3.879a1.5 1.5 0 011.06.44l3.122 3.12A1.5 1.5 0 0117 6.622V12.5a1.5 1.5 0 01-1.5 1.5h-1v-3.379a3 3 0 00-.879-2.121L10.5 5.379A3 3 0 008.379 4.5H7v-1z"
					/>
					<path
						d="M4.5 6A1.5 1.5 0 003 7.5v9A1.5 1.5 0 004.5 18h7a1.5 1.5 0 001.5-1.5v-5.879a1.5 1.5 0 00-.44-1.06L9.44 6.439A1.5 1.5 0 008.378 6H4.5z"
					/>
				</svg>
			</span>
			Resources
		</h3>
	</div>

	<nav class="links-list">
		{#each resources as resource, index}
			<a
				href={resource.href}
				class="resource-link"
				target={resource.external ? '_blank' : undefined}
				rel={resource.external ? 'noopener noreferrer' : undefined}
				style="animation-delay: {index * 0.05}s"
			>
				<span class="link-icon-wrapper" aria-hidden="true">
					{#if resource.icon === 'video'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path
								d="M3.25 4A2.25 2.25 0 001 6.25v7.5A2.25 2.25 0 003.25 16h7.5A2.25 2.25 0 0013 13.75v-7.5A2.25 2.25 0 0010.75 4h-7.5zM19 4.75a.75.75 0 00-1.28-.53l-3 3a.75.75 0 00-.22.53v4.5c0 .199.079.39.22.53l3 3a.75.75 0 001.28-.53V4.75z"
							/>
						</svg>
					{:else if resource.icon === 'chart'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path
								fill-rule="evenodd"
								d="M4.25 2A2.25 2.25 0 002 4.25v11.5A2.25 2.25 0 004.25 18h11.5A2.25 2.25 0 0018 15.75V4.25A2.25 2.25 0 0015.75 2H4.25zM15 5.75a.75.75 0 00-1.5 0v8.5a.75.75 0 001.5 0v-8.5zm-8.5 6a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5zM8.584 9a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5a.75.75 0 01.75-.75zm3.58-1.25a.75.75 0 00-1.5 0v6.5a.75.75 0 001.5 0v-6.5z"
								clip-rule="evenodd"
							/>
						</svg>
					{:else if resource.icon === 'star'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path
								fill-rule="evenodd"
								d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401z"
								clip-rule="evenodd"
							/>
						</svg>
					{:else if resource.icon === 'download'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path
								d="M10.75 2.75a.75.75 0 00-1.5 0v8.614L6.295 8.235a.75.75 0 10-1.09 1.03l4.25 4.5a.75.75 0 001.09 0l4.25-4.5a.75.75 0 00-1.09-1.03l-2.955 3.129V2.75z"
							/>
							<path
								d="M3.5 12.75a.75.75 0 00-1.5 0v2.5A2.75 2.75 0 004.75 18h10.5A2.75 2.75 0 0018 15.25v-2.5a.75.75 0 00-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5z"
							/>
						</svg>
					{:else if resource.icon === 'settings'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path
								fill-rule="evenodd"
								d="M7.84 1.804A1 1 0 018.82 1h2.36a1 1 0 01.98.804l.331 1.652a6.993 6.993 0 011.929 1.115l1.598-.54a1 1 0 011.186.447l1.18 2.044a1 1 0 01-.205 1.251l-1.267 1.113a7.047 7.047 0 010 2.228l1.267 1.113a1 1 0 01.206 1.25l-1.18 2.045a1 1 0 01-1.187.447l-1.598-.54a6.993 6.993 0 01-1.929 1.115l-.33 1.652a1 1 0 01-.98.804H8.82a1 1 0 01-.98-.804l-.331-1.652a6.993 6.993 0 01-1.929-1.115l-1.598.54a1 1 0 01-1.186-.447l-1.18-2.044a1 1 0 01.205-1.251l1.267-1.114a7.05 7.05 0 010-2.227L1.821 7.773a1 1 0 01-.206-1.25l1.18-2.045a1 1 0 011.187-.447l1.598.54A6.993 6.993 0 017.51 3.456l.33-1.652zM10 13a3 3 0 100-6 3 3 0 000 6z"
								clip-rule="evenodd"
							/>
						</svg>
					{/if}
				</span>
				<div class="link-content">
					<span class="link-label">{resource.label}</span>
					{#if variant === 'card' && resource.description}
						<span class="link-description">{resource.description}</span>
					{/if}
				</div>
				<span class="link-arrow" aria-hidden="true">
					<svg viewBox="0 0 20 20" fill="currentColor">
						<path
							fill-rule="evenodd"
							d="M3 10a.75.75 0 01.75-.75h10.638L10.23 5.29a.75.75 0 111.04-1.08l5.5 5.25a.75.75 0 010 1.08l-5.5 5.25a.75.75 0 11-1.04-1.08l4.158-3.96H3.75A.75.75 0 013 10z"
							clip-rule="evenodd"
						/>
					</svg>
				</span>
			</a>
		{/each}
	</nav>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   CONTAINER
	   ═══════════════════════════════════════════════════════════════════════ */
	.resource-links {
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
		border-radius: 14px;
		padding: 20px;
		box-shadow:
			0 1px 3px rgba(0, 0, 0, 0.04),
			0 1px 2px rgba(0, 0, 0, 0.06);
	}

	.resource-links.compact {
		padding: 16px;
	}

	.resource-links.card-style {
		padding: 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════ */
	.section-header {
		margin-bottom: 16px;
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		font-weight: 700;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		letter-spacing: 0.06em;
		margin: 0;
	}

	.title-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		color: var(--color-text-muted);
	}

	.title-icon svg {
		width: 100%;
		height: 100%;
	}

	.compact .section-header {
		margin-bottom: 12px;
	}

	.compact .section-title {
		font-size: 11px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   LINKS LIST
	   ═══════════════════════════════════════════════════════════════════════ */
	.links-list {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.card-style .links-list {
		gap: 8px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESOURCE LINK
	   ═══════════════════════════════════════════════════════════════════════ */
	.resource-link {
		display: flex;
		align-items: center;
		gap: 14px;
		padding: 12px 14px;
		margin: 0 -14px;
		color: var(--color-text-secondary);
		text-decoration: none;
		border-radius: 10px;
		transition: all 0.2s ease-out;
		min-height: 48px;
		position: relative;
	}

	.compact .resource-link {
		padding: 10px 12px;
		margin: 0 -12px;
		gap: 12px;
		min-height: 40px;
	}

	.card-style .resource-link {
		margin: 0;
		padding: 14px 16px;
		background: var(--color-bg-subtle);
		border: 1px solid var(--color-border-default);
	}

	.resource-link:hover {
		background: var(--color-bg-subtle);
		color: var(--color-text-primary);
	}

	.card-style .resource-link:hover {
		background: var(--color-bg-subtle);
		border-color: var(--color-border-strong);
		transform: translateX(4px);
	}

	/* Icon Wrapper */
	.link-icon-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: linear-gradient(
			135deg,
			var(--color-bg-subtle) 0%,
			var(--color-border-default) 100%
		);
		border-radius: 10px;
		color: var(--color-text-muted);
		flex-shrink: 0;
		transition: all 0.2s ease-out;
	}

	.compact .link-icon-wrapper {
		width: 32px;
		height: 32px;
		border-radius: 8px;
	}

	.card-style .link-icon-wrapper {
		width: 40px;
		height: 40px;
		background: linear-gradient(
			135deg,
			var(--color-brand-primary) 0%,
			var(--color-brand-primary-light) 100%
		);
		color: var(--color-bg-card);
	}

	.link-icon-wrapper svg {
		width: 18px;
		height: 18px;
	}

	.compact .link-icon-wrapper svg {
		width: 16px;
		height: 16px;
	}

	.card-style .link-icon-wrapper svg {
		width: 20px;
		height: 20px;
	}

	.resource-link:hover .link-icon-wrapper {
		background: linear-gradient(
			135deg,
			var(--color-brand-primary) 0%,
			var(--color-brand-primary-light) 100%
		);
		color: var(--color-bg-card);
		transform: scale(1.05);
	}

	.card-style .resource-link:hover .link-icon-wrapper {
		transform: scale(1.08);
		box-shadow: 0 4px 12px rgba(20, 62, 89, 0.25);
	}

	/* Link Content */
	.link-content {
		display: flex;
		flex-direction: column;
		gap: 2px;
		flex: 1;
		min-width: 0;
	}

	.link-label {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text-secondary);
		line-height: 1.3;
	}

	.compact .link-label {
		font-size: 13px;
	}

	.resource-link:hover .link-label {
		color: var(--color-text-primary);
	}

	.link-description {
		font-size: 12px;
		color: var(--color-text-muted);
		line-height: 1.3;
	}

	.resource-link:hover .link-description {
		color: var(--color-text-muted);
	}

	/* Arrow */
	.link-arrow {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		color: var(--color-border-strong);
		flex-shrink: 0;
		opacity: 0;
		transform: translateX(-8px);
		transition: all 0.2s ease-out;
	}

	.link-arrow svg {
		width: 16px;
		height: 16px;
	}

	.resource-link:hover .link-arrow {
		opacity: 1;
		transform: translateX(0);
		color: var(--color-brand-primary);
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   HOVER EFFECTS - STAR ICON SPECIAL
	   ═══════════════════════════════════════════════════════════════════════ */
	.resource-link:nth-child(3):hover .link-icon-wrapper {
		background: linear-gradient(135deg, #fbbf24 0%, var(--color-watching) 100%);
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   2026 MOBILE-FIRST RESPONSIVE
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   ═══════════════════════════════════════════════════════════════════════ */

	/* Base Mobile Styles (xs: 360px) */
	.resource-links {
		padding: 1rem;
		padding-left: calc(1rem + env(safe-area-inset-left, 0px));
		padding-right: calc(1rem + env(safe-area-inset-right, 0px));
	}

	.section-header {
		margin-bottom: 0.875rem;
	}

	.section-title {
		font-size: 0.75rem;
	}

	.title-icon {
		width: 1rem;
		height: 1rem;
	}

	.links-list {
		gap: 0.125rem;
	}

	.resource-link {
		padding: 0.75rem;
		margin: 0 -0.75rem;
		gap: 0.75rem;
		/* 44px touch target */
		min-height: 44px;
	}

	.link-icon-wrapper {
		width: 2.5rem;
		height: 2.5rem;
		border-radius: 0.5rem;
	}

	.link-icon-wrapper svg {
		width: 1.125rem;
		height: 1.125rem;
	}

	.link-label {
		font-size: 0.8125rem;
	}

	.link-description {
		font-size: 0.6875rem;
	}

	.link-arrow {
		width: 1.125rem;
		height: 1.125rem;
	}

	.link-arrow svg {
		width: 0.875rem;
		height: 0.875rem;
	}

	/* Compact variant - Mobile */
	.compact .section-header {
		margin-bottom: 0.625rem;
	}

	.compact .section-title {
		font-size: 0.6875rem;
	}

	.compact .resource-link {
		padding: 0.625rem;
		margin: 0 -0.625rem;
		gap: 0.625rem;
		/* 44px touch target */
		min-height: 44px;
	}

	.compact .link-icon-wrapper {
		width: 2.25rem;
		height: 2.25rem;
	}

	.compact .link-icon-wrapper svg {
		width: 1rem;
		height: 1rem;
	}

	.compact .link-label {
		font-size: 0.75rem;
	}

	/* Card variant - Mobile */
	.card-style .resource-links {
		padding: 1.25rem;
	}

	.card-style .links-list {
		gap: 0.5rem;
	}

	.card-style .resource-link {
		margin: 0;
		padding: 0.875rem 1rem;
		/* 44px touch target */
		min-height: 44px;
	}

	.card-style .link-icon-wrapper {
		width: 2.75rem;
		height: 2.75rem;
	}

	.card-style .link-icon-wrapper svg {
		width: 1.25rem;
		height: 1.25rem;
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.resource-links {
			padding: 1.25rem;
		}

		.section-header {
			margin-bottom: 1rem;
		}

		.section-title {
			font-size: 0.8125rem;
		}

		.title-icon {
			width: 1.125rem;
			height: 1.125rem;
		}

		.resource-link {
			padding: 0.75rem 0.875rem;
			margin: 0 -0.875rem;
			gap: 0.875rem;
		}

		.link-icon-wrapper {
			width: 2.25rem;
			height: 2.25rem;
			border-radius: 0.625rem;
		}

		.link-icon-wrapper svg {
			width: 1.125rem;
			height: 1.125rem;
		}

		.link-label {
			font-size: 0.875rem;
		}

		.link-description {
			font-size: 0.75rem;
		}

		.card-style .resource-links {
			padding: 1.5rem;
		}

		.card-style .resource-link {
			padding: 0.875rem 1rem;
		}

		.card-style .link-icon-wrapper {
			width: 2.5rem;
			height: 2.5rem;
		}

		.card-style .link-icon-wrapper svg {
			width: 1.25rem;
			height: 1.25rem;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.links-list {
			gap: 2px;
		}

		.resource-link {
			padding: 12px 14px;
			margin: 0 -14px;
			min-height: 48px;
		}

		.link-icon-wrapper {
			width: 36px;
			height: 36px;
		}

		.link-icon-wrapper svg {
			width: 18px;
			height: 18px;
		}

		.card-style .links-list {
			gap: 8px;
		}

		.card-style .resource-link {
			padding: 14px 16px;
		}

		.card-style .link-icon-wrapper {
			width: 40px;
			height: 40px;
		}

		.card-style .link-icon-wrapper svg {
			width: 20px;
			height: 20px;
		}
	}
</style>

<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * ResourceLinks Component - Quick Resources Sidebar Section
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Quick access links for common resources
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */

	interface ResourceLink {
		icon: string;
		label: string;
		href: string;
		external?: boolean;
	}

	const resources: ResourceLink[] = [
		{ icon: 'video', label: 'Video Library', href: '/dashboard/explosive-swings/video-library' },
		{ icon: 'chart', label: 'Trade Tracker', href: '/dashboard/explosive-swings/trades' },
		{ icon: 'star', label: 'My Favorites', href: '/dashboard/explosive-swings/favorites' },
		{ icon: 'download', label: 'Export CSV', href: '/dashboard/explosive-swings/export' },
		{ icon: 'settings', label: 'Alert Settings', href: '/dashboard/explosive-swings/settings' }
	];
</script>

<div class="resource-links" role="navigation" aria-labelledby="resources-heading">
	<h3 id="resources-heading" class="section-title">Resources</h3>

	<nav class="links-list">
		{#each resources as resource}
			<a
				href={resource.href}
				class="resource-link"
				target={resource.external ? '_blank' : undefined}
				rel={resource.external ? 'noopener noreferrer' : undefined}
			>
				<span class="link-icon" aria-hidden="true">
					{#if resource.icon === 'video'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path d="M3.25 4A2.25 2.25 0 001 6.25v7.5A2.25 2.25 0 003.25 16h7.5A2.25 2.25 0 0013 13.75v-7.5A2.25 2.25 0 0010.75 4h-7.5zM19 4.75a.75.75 0 00-1.28-.53l-3 3a.75.75 0 00-.22.53v4.5c0 .199.079.39.22.53l3 3a.75.75 0 001.28-.53V4.75z" />
						</svg>
					{:else if resource.icon === 'chart'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h16.5a.75.75 0 010 1.5H18v8.75A2.75 2.75 0 0115.25 15h-1.072l.798 3.06a.75.75 0 01-1.452.38L12.69 15H7.31l-.834 3.44a.75.75 0 01-1.452-.38L5.822 15H4.75A2.75 2.75 0 012 12.25V3.5h-.25A.75.75 0 011 2.75zM7.373 13.5l-.426 1.75 6.106-.002-.426-1.748H7.373zM15 3.5H4v8.75c0 .69.56 1.25 1.25 1.25h9.5c.69 0 1.25-.56 1.25-1.25V3.5z" clip-rule="evenodd" />
						</svg>
					{:else if resource.icon === 'star'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401z" clip-rule="evenodd" />
						</svg>
					{:else if resource.icon === 'download'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path d="M10.75 2.75a.75.75 0 00-1.5 0v8.614L6.295 8.235a.75.75 0 10-1.09 1.03l4.25 4.5a.75.75 0 001.09 0l4.25-4.5a.75.75 0 00-1.09-1.03l-2.955 3.129V2.75z" />
							<path d="M3.5 12.75a.75.75 0 00-1.5 0v2.5A2.75 2.75 0 004.75 18h10.5A2.75 2.75 0 0018 15.25v-2.5a.75.75 0 00-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5z" />
						</svg>
					{:else if resource.icon === 'settings'}
						<svg viewBox="0 0 20 20" fill="currentColor">
							<path fill-rule="evenodd" d="M7.84 1.804A1 1 0 018.82 1h2.36a1 1 0 01.98.804l.331 1.652a6.993 6.993 0 011.929 1.115l1.598-.54a1 1 0 011.186.447l1.18 2.044a1 1 0 01-.205 1.251l-1.267 1.113a7.047 7.047 0 010 2.228l1.267 1.113a1 1 0 01.206 1.25l-1.18 2.045a1 1 0 01-1.187.447l-1.598-.54a6.993 6.993 0 01-1.929 1.115l-.33 1.652a1 1 0 01-.98.804H8.82a1 1 0 01-.98-.804l-.331-1.652a6.993 6.993 0 01-1.929-1.115l-1.598.54a1 1 0 01-1.186-.447l-1.18-2.044a1 1 0 01.205-1.251l1.267-1.114a7.05 7.05 0 010-2.227L1.821 7.773a1 1 0 01-.206-1.25l1.18-2.045a1 1 0 011.187-.447l1.598.54A6.993 6.993 0 017.51 3.456l.33-1.652zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
						</svg>
					{/if}
				</span>
				<span class="link-label">{resource.label}</span>
			</a>
		{/each}
	</nav>
</div>

<style>
	.resource-links {
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		padding: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
	}

	.section-title {
		font-size: 12px;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 12px 0;
	}

	.links-list {
		display: flex;
		flex-direction: column;
	}

	.resource-link {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px;
		margin: 0 -12px;
		color: #334155;
		text-decoration: none;
		border-radius: 8px;
		transition: all 0.2s;
		min-height: 44px;
	}

	.resource-link:hover {
		background: #f1f5f9;
		color: #0f172a;
	}

	.link-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		color: #64748b;
		flex-shrink: 0;
	}

	.link-icon svg {
		width: 100%;
		height: 100%;
	}

	.resource-link:hover .link-icon {
		color: #143e59;
	}

	.link-label {
		font-size: 14px;
		font-weight: 500;
	}
</style>

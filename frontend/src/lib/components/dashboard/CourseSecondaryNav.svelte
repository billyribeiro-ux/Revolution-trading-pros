<!--
	Course Secondary Navigation - Appears when sidebar is collapsed
	═══════════════════════════════════════════════════════════════════════════
	
	This component renders a 280px blue navigation panel that appears when:
	- User is on a course-specific dashboard page
	- Main sidebar is collapsed (80px)
	
	Based on MasterDash reference implementation from Simpler Trading.
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	interface NavItem {
		href: string;
		icon: string;
		text: string;
		submenu?: NavItem[];
	}

	interface Props {
		courseName: string;
		menuItems: NavItem[];
	}

	let { courseName, menuItems }: Props = $props();

	// Submenu state
	let openSubmenu = $state<string | null>(null);

	function toggleSubmenu(text: string) {
		openSubmenu = openSubmenu === text ? null : text;
	}
</script>

<nav class="dashboard__nav-secondary" aria-label="{courseName} Navigation">
	<ul class="secondary-nav-list">
		{#each menuItems as item}
			<li class:has-submenu={item.submenu}>
				{#if item.submenu}
					<!-- Item with submenu -->
					<button
						type="button"
						class="secondary-nav-link"
						onclick={() => toggleSubmenu(item.text)}
						aria-expanded={openSubmenu === item.text}
					>
						<span class="dashboard__nav-item-icon">
							<RtpIcon name={item.icon} size={20} />
						</span>
						<span class="dashboard__nav-item-text">{item.text}</span>
						<span class="submenu-arrow" class:is-open={openSubmenu === item.text}>▼</span>
					</button>

					{#if openSubmenu === item.text}
						<ul class="dashboard__nav-submenu">
							{#each item.submenu as subItem}
								<li>
									<a href={subItem.href} class="submenu-link">
										{subItem.text}
									</a>
								</li>
							{/each}
						</ul>
					{/if}
				{:else}
					<!-- Regular link -->
					<a href={item.href} class="secondary-nav-link">
						<span class="dashboard__nav-item-icon">
							<RtpIcon name={item.icon} size={20} />
						</span>
						<span class="dashboard__nav-item-text">{item.text}</span>
					</a>
				{/if}
			</li>
		{/each}
	</ul>
</nav>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * COURSE SECONDARY NAVIGATION - Pixel Perfect Match to MasterDash
	 * Background: #0984ae (lighter blue than main sidebar)
	 * Width: 280px
	 * Position: Fixed, left: 80px (after collapsed sidebar)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__nav-secondary {
		position: fixed;
		top: 0;
		left: 80px;
		width: 280px;
		height: 100vh;
		background-color: #0984ae;
		overflow-y: auto;
		overflow-x: hidden;
		z-index: 98;
		box-shadow: 2px 0 10px rgba(0, 0, 0, 0.2);
	}

	/* Scrollbar Styling */
	.dashboard__nav-secondary::-webkit-scrollbar {
		width: 6px;
	}

	.dashboard__nav-secondary::-webkit-scrollbar-track {
		background: rgba(255, 255, 255, 0.05);
	}

	.dashboard__nav-secondary::-webkit-scrollbar-thumb {
		background: rgba(255, 255, 255, 0.2);
		border-radius: 3px;
	}

	.dashboard__nav-secondary::-webkit-scrollbar-thumb:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Navigation List */
	.secondary-nav-list {
		list-style: none;
		margin: 0;
		padding: 20px 0;
	}

	.secondary-nav-list > li {
		list-style: none;
		margin: 0;
	}

	/* Navigation Links */
	.secondary-nav-link {
		display: flex;
		align-items: center;
		width: 100%;
		padding: 15px 20px;
		text-decoration: none;
		background: none;
		border: none;
		color: rgba(255, 255, 255, 0.9);
		font-size: 14px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		transition: all 0.2s ease;
		cursor: pointer;
		text-align: left;
		position: relative;
	}

	.secondary-nav-link:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #ffffff;
	}

	/* Icons */
	.dashboard__nav-item-icon {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		margin-right: 12px;
		flex-shrink: 0;
		color: rgba(255, 255, 255, 0.9);
	}

	.secondary-nav-link:hover .dashboard__nav-item-icon {
		color: #ffffff;
	}

	/* Text */
	.dashboard__nav-item-text {
		flex: 1;
		font-size: 14px;
		line-height: 1.4;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	/* Submenu Arrow */
	.submenu-arrow {
		font-size: 10px;
		margin-left: 8px;
		transition: transform 0.2s ease;
		color: rgba(255, 255, 255, 0.7);
	}

	.submenu-arrow.is-open {
		transform: rotate(180deg);
	}

	/* Submenu */
	.dashboard__nav-submenu {
		list-style: none;
		margin: 0;
		padding: 0;
		background-color: rgba(0, 0, 0, 0.1);
		animation: slideDown 0.2s ease;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
			max-height: 0;
		}
		to {
			opacity: 1;
			max-height: 500px;
		}
	}

	.dashboard__nav-submenu li {
		list-style: none;
		margin: 0;
	}

	.submenu-link {
		display: block;
		padding: 12px 20px 12px 52px;
		text-decoration: none;
		color: rgba(255, 255, 255, 0.8);
		font-size: 13px;
		font-weight: 300;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		transition: all 0.2s ease;
	}

	.submenu-link:hover {
		background-color: rgba(255, 255, 255, 0.1);
		color: #ffffff;
		padding-left: 56px;
	}

	/* Has Submenu Indicator */
	.has-submenu > .secondary-nav-link {
		cursor: pointer;
	}

	/* Responsive - Hide on mobile */
	@media (max-width: 1280px) {
		.dashboard__nav-secondary {
			display: none;
		}
	}
</style>

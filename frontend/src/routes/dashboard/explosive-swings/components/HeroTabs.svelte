<script lang="ts">
	/**
	 * HeroTabs Component - Apple Principal Engineer ICT Level 7
	 * Extracted from WeeklyHero.svelte for better maintainability
	 * 
	 * @description Tab navigation for video breakdown and trade plan
	 * @version 1.0.0 - ICT 7 compliance
	 */

	interface Props {
		activeTab: 'video' | 'entries';
		onTabChange: (tab: 'video' | 'entries') => void;
	}

	const { activeTab, onTabChange }: Props = $props();
</script>

<nav class="hero-tabs-bar" aria-label="Hero content tabs">
	<button
		class="hero-tab"
		class:active={activeTab === 'video'}
		onclick={() => onTabChange('video')}
		type="button"
		aria-pressed={activeTab === 'video'}
	>
		<svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18" aria-hidden="true">
			<path d="M8 5v14l11-7z" />
		</svg>
		Video Breakdown
	</button>
	<button
		class="hero-tab"
		class:active={activeTab === 'entries'}
		onclick={() => onTabChange('entries')}
		type="button"
		aria-pressed={activeTab === 'entries'}
	>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			width="18"
			height="18"
			aria-hidden="true"
		>
			<path
				d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
			/>
		</svg>
		Trade Plan & Entries
	</button>
</nav>

<style>
	.hero-tabs-bar {
		display: flex;
		gap: 12px;
		justify-content: center;
		padding: 0 24px 24px;
	}

	.hero-tab {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 12px 24px;
		background: rgba(255, 255, 255, 0.15);
		border: 2px solid rgba(255, 255, 255, 0.25);
		border-radius: 10px;
		color: var(--color-bg-card);
		font-size: 14px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.hero-tab:hover {
		background: rgba(255, 255, 255, 0.25);
		border-color: rgba(255, 255, 255, 0.35);
	}

	.hero-tab.active {
		background: var(--color-bg-card);
		color: var(--color-brand-secondary);
		border-color: var(--color-bg-card);
	}

	.hero-tab:focus-visible {
		outline: 3px solid var(--color-bg-card);
		outline-offset: 2px;
	}

	/* Responsive */
	@media (min-width: 1920px) {
		.hero-tab {
			padding: 14px 28px;
			font-size: 15px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.hero-tab {
			transition: none;
		}
	}
</style>


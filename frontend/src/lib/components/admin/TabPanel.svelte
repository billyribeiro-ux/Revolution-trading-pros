<!--
	TabPanel.svelte - Layout-Shift-Free Tab Panel Component
	═══════════════════════════════════════════════════════════════════════════════
	
	ICT 7 Principal Engineer Grade - Svelte 5 Best Practices
	
	This component eliminates Content Layout Shift (CLS) by:
	1. Keeping all tab content in DOM (no unmount/remount)
	2. Using CSS visibility/opacity for instant switching
	3. CSS containment for paint isolation
	4. Transform-based animations (GPU accelerated)
	
	Usage:
	<TabPanel tabs={tabs} bind:activeTab={activeTab}>
		{#snippet panel(tabId)}
			{#if tabId === 'overview'}
				<OverviewContent />
			{:else if tabId === 'settings'}
				<SettingsContent />
			{/if}
		{/snippet}
	</TabPanel>
	
	@version 1.0.0 - January 2026
-->
<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Tab {
		id: string;
		label: string;
		icon?: string;
		badge?: string | number;
		disabled?: boolean;
	}

	interface Props {
		tabs: Tab[];
		activeTab: string;
		variant?: 'default' | 'pills' | 'underline' | 'switcher';
		panel: Snippet<[string]>;
		class?: string;
	}

	let props: Props = $props();

	// Bindable state for two-way binding
	let activeTab = $state(props.activeTab);

	// Destructure with defaults for internal use
	const tabs = $derived(props.tabs);
	const variant = $derived(props.variant ?? 'default');
	const panel = $derived(props.panel);
	const className = $derived(props.class ?? '');

	// Sync activeTab back when props change
	$effect(() => {
		if (props.activeTab !== undefined && props.activeTab !== activeTab) {
			activeTab = props.activeTab;
		}
	});

	function handleTabClick(tabId: string, disabled?: boolean) {
		if (disabled) return;
		activeTab = tabId;
	}

	function handleKeyDown(event: KeyboardEvent, tabId: string, index: number) {
		if (event.key === 'ArrowRight') {
			event.preventDefault();
			const nextIndex = (index + 1) % tabs.length;
			const nextTab = tabs[nextIndex];
			if (!nextTab.disabled) {
				activeTab = nextTab.id;
			}
		} else if (event.key === 'ArrowLeft') {
			event.preventDefault();
			const prevIndex = (index - 1 + tabs.length) % tabs.length;
			const prevTab = tabs[prevIndex];
			if (!prevTab.disabled) {
				activeTab = prevTab.id;
			}
		} else if (event.key === 'Home') {
			event.preventDefault();
			const firstEnabled = tabs.find((t) => !t.disabled);
			if (firstEnabled) activeTab = firstEnabled.id;
		} else if (event.key === 'End') {
			event.preventDefault();
			const lastEnabled = [...tabs].reverse().find((t) => !t.disabled);
			if (lastEnabled) activeTab = lastEnabled.id;
		}
	}
</script>

<div class="tab-panel-container {variant} {className}">
	<!-- Tab Navigation -->
	<div class="tab-nav" role="tablist" aria-label="Tab navigation">
		{#each tabs as tab, index (tab.id)}
			<button
				type="button"
				role="tab"
				id="tab-{tab.id}"
				aria-selected={activeTab === tab.id}
				aria-controls="panel-{tab.id}"
				tabindex={activeTab === tab.id ? 0 : -1}
				class="tab-btn"
				class:active={activeTab === tab.id}
				class:disabled={tab.disabled}
				disabled={tab.disabled}
				onclick={() => handleTabClick(tab.id, tab.disabled)}
				onkeydown={(e) => handleKeyDown(e, tab.id, index)}
			>
				{#if tab.icon}
					<span class="tab-icon">{tab.icon}</span>
				{/if}
				<span class="tab-label">{tab.label}</span>
				{#if tab.badge}
					<span class="tab-badge">{tab.badge}</span>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Tab Panels - ALL panels stay in DOM, visibility controlled via CSS -->
	<div class="tab-panels">
		{#each tabs as tab (tab.id)}
			<div
				role="tabpanel"
				id="panel-{tab.id}"
				aria-labelledby="tab-{tab.id}"
				class="tab-panel"
				class:active={activeTab === tab.id}
				class:inactive={activeTab !== tab.id}
				inert={activeTab !== tab.id ? true : undefined}
			>
				{@render panel(tab.id)}
			</div>
		{/each}
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   TAB PANEL CONTAINER - Layout Shift Prevention
	   ═══════════════════════════════════════════════════════════════════════════ */

	.tab-panel-container {
		display: flex;
		flex-direction: column;
		width: 100%;
		contain: layout style;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TAB NAVIGATION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.tab-nav {
		display: flex;
		gap: 0.25rem;
		flex-wrap: wrap;
		margin-bottom: 1.5rem;
	}

	.tab-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 1.25rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--admin-text-muted, #94a3b8);
		background: transparent;
		border: none;
		border-radius: 0.75rem;
		cursor: pointer;
		transition:
			color 0.15s ease,
			background-color 0.15s ease;
		white-space: nowrap;
	}

	.tab-btn:hover:not(.disabled) {
		color: var(--admin-text-primary, #f1f5f9);
		background: rgba(255, 255, 255, 0.05);
	}

	.tab-btn:focus-visible {
		outline: 2px solid var(--primary-500, #e6b800);
		outline-offset: 2px;
	}

	.tab-btn.active {
		color: var(--bg-base, #0f172a);
		background: linear-gradient(135deg, var(--primary-500, #e6b800), var(--primary-600, #b38f00));
		font-weight: 600;
		box-shadow: 0 4px 12px rgba(230, 184, 0, 0.25);
	}

	.tab-btn.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.tab-icon {
		font-size: 1.125rem;
		line-height: 1;
	}

	.tab-badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 1.25rem;
		height: 1.25rem;
		padding: 0 0.375rem;
		font-size: 0.75rem;
		font-weight: 600;
		background: rgba(255, 255, 255, 0.15);
		border-radius: 9999px;
	}

	.tab-btn.active .tab-badge {
		background: rgba(0, 0, 0, 0.2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TAB PANELS - Critical Layout Shift Prevention
	   ═══════════════════════════════════════════════════════════════════════════ */

	.tab-panels {
		position: relative;
		min-height: 400px;
		contain: layout style paint;
		isolation: isolate;
	}

	.tab-panel {
		/* CSS Grid stacking - all panels occupy same space */
		position: absolute;
		inset: 0;
		width: 100%;

		/* Layout containment */
		contain: content;

		/* GPU-accelerated properties only */
		opacity: 0;
		visibility: hidden;
		transform: translateY(8px);

		/* Smooth transitions using transform/opacity (no layout shift) */
		transition:
			opacity 0.2s ease,
			visibility 0.2s ease,
			transform 0.2s ease;

		/* Ensure proper stacking */
		z-index: 0;
	}

	.tab-panel.active {
		position: relative;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		z-index: 1;
	}

	.tab-panel.inactive {
		pointer-events: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VARIANT: Pills
	   ═══════════════════════════════════════════════════════════════════════════ */

	.pills .tab-nav {
		background: rgba(255, 255, 255, 0.05);
		padding: 0.25rem;
		border-radius: 1rem;
		border: 1px solid rgba(255, 255, 255, 0.1);
		gap: 0.25rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VARIANT: Underline
	   ═══════════════════════════════════════════════════════════════════════════ */

	.underline .tab-nav {
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		gap: 0;
		margin-bottom: 1.5rem;
	}

	.underline .tab-btn {
		border-radius: 0;
		padding: 0.75rem 1.25rem;
		margin-bottom: -1px;
		border-bottom: 2px solid transparent;
	}

	.underline .tab-btn.active {
		background: transparent;
		color: var(--primary-500, #e6b800);
		border-bottom-color: var(--primary-500, #e6b800);
		box-shadow: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   VARIANT: Switcher (Settings style)
	   ═══════════════════════════════════════════════════════════════════════════ */

	.switcher .tab-nav {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border-radius: 1rem;
		padding: 0.25rem;
		border: 1px solid rgba(255, 255, 255, 0.1);
		gap: 0.25rem;
		width: fit-content;
		margin: 0 auto 1.5rem;
	}

	.switcher .tab-btn {
		border-radius: 0.75rem;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.tab-nav {
			overflow-x: auto;
			flex-wrap: nowrap;
			-webkit-overflow-scrolling: touch;
			scrollbar-width: none;
		}

		.tab-nav::-webkit-scrollbar {
			display: none;
		}

		.tab-btn {
			flex-shrink: 0;
			padding: 0.5rem 1rem;
		}

		.tab-panels {
			min-height: 300px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.tab-panel {
			transition: none;
			transform: none;
		}

		.tab-panel.inactive {
			display: none;
		}
	}
</style>

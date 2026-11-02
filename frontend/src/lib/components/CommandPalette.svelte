<script lang="ts">
	/**
	 * Command Palette - Apple ICT9+ Design
	 *
	 * Global search and navigation with:
	 * - Cmd/Ctrl+K activation
	 * - Fuzzy search across pages, actions, and data
	 * - Keyboard navigation
	 * - Recent searches
	 * - Action shortcuts
	 *
	 */

	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { fade, scale, fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { browser } from '$app/env';
	import { toastStore } from '#lib/stores/toast.svelte.js';
	import {
		IconSearch,
		IconCode,
		IconArrowUp,
		IconArrowDown,
		IconHome,
		IconChartBar,
		IconUsers,
		IconMail,
		IconSettings,
		IconShoppingCart,
		IconFileText,
		IconPhoto,
		IconPlugConnected,
		IconActivity,
		IconBrandGoogle,
		IconForms,
		IconVideo,
		IconCreditCard,
		IconDatabase,
		IconRefresh,
		IconDownload,
		IconUpload,
		IconLogout
	} from '#lib/icons/index.js';

	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = $bindable(false), onclose }: Props = $props();

	let searchQuery = $state('');
	let selectedIndex = $state(0);

	// Navigation items
	const navigationItems = [
		{
			id: 'dashboard',
			label: 'Dashboard',
			path: '/admin',
			icon: IconHome,
			category: 'Navigation'
		},

		{
			id: 'analytics',
			label: 'Analytics',
			path: '/admin/analytics',
			icon: IconChartBar,
			category: 'Navigation'
		},
		{
			id: 'seo',
			label: 'SEO Settings',
			path: '/admin/seo',
			icon: IconBrandGoogle,
			category: 'Navigation'
		},
		{
			id: 'behavior',
			label: 'Behavior Tracking',
			path: '/admin/behavior',
			icon: IconActivity,
			category: 'Navigation'
		},
		{
			id: 'blog',
			label: 'Blog Posts',
			path: '/admin/blog',
			icon: IconFileText,
			category: 'Content'
		},
		{
			id: 'media',
			label: 'Media Library',
			path: '/admin/media',
			icon: IconPhoto,
			category: 'Content'
		},

		{
			id: 'videos',
			label: 'Videos',
			path: '/admin/videos',
			icon: IconVideo,
			category: 'Content'
		},

		{
			id: 'forms',
			label: 'Forms',
			path: '/admin/forms',
			icon: IconForms,
			category: 'Content'
		},

		{
			id: 'members',
			label: 'Members',
			path: '/admin/members',
			icon: IconUsers,
			category: 'Users'
		},

		{
			id: 'crm',
			label: 'CRM',
			path: '/admin/crm',
			icon: IconUsers,
			category: 'Users'
		},

		{
			id: 'campaigns',
			label: 'Email Campaigns',
			path: '/admin/email/campaigns',
			icon: IconMail,
			category: 'Marketing'
		},
		{
			id: 'templates',
			label: 'Email Templates',
			path: '/admin/email/templates',
			icon: IconMail,
			category: 'Marketing'
		},
		{
			id: 'products',
			label: 'Products',
			path: '/admin/products',
			icon: IconShoppingCart,
			category: 'Commerce'
		},
		{
			id: 'subscriptions',
			label: 'Subscriptions',
			path: '/admin/subscriptions',
			icon: IconCreditCard,
			category: 'Commerce'
		},
		{
			id: 'connections',
			label: 'API Connections',
			path: '/admin/connections',
			icon: IconPlugConnected,
			category: 'Settings'
		},
		{
			id: 'site-health',
			label: 'Site Health',
			path: '/admin/site-health',
			icon: IconActivity,
			category: 'Settings'
		},
		{
			id: 'settings',
			label: 'Settings',
			path: '/admin/settings',
			icon: IconSettings,
			category: 'Settings'
		}
	];

	// Quick actions
	const quickActions = [
		{
			id: 'new-post',
			label: 'Create New Post',
			action: () => goto('/admin/blog/create'),
			icon: IconFileText,
			category: 'Quick Actions'
		},
		{
			id: 'new-campaign',
			label: 'Create Email Campaign',
			action: () => goto('/admin/email/campaigns?new=true'),
			icon: IconMail,
			category: 'Quick Actions'
		},
		{
			id: 'upload-media',
			label: 'Upload Media',
			action: () => goto('/admin/media?upload=true'),
			icon: IconUpload,
			category: 'Quick Actions'
		},
		{
			id: 'export-data',
			label: 'Export Data',
			action: () => goto('/admin/settings?export=true'),
			icon: IconDownload,
			category: 'Quick Actions'
		},
		{
			id: 'refresh-all',
			label: 'Refresh All Data',
			action: () => window.location.reload(),
			icon: IconRefresh,
			category: 'Quick Actions'
		},
		{
			id: 'clear-cache',
			label: 'Clear Cache',
			// FIX-2026-04-26: was `localStorage.clear()` + `window.location.reload()` which wiped
			// auth tokens (rtp_access_token, rtp_refresh_token, auth_user) hydrated by the auth
			// store, instantly logging the operator out. Switched to Option A from the fix plan:
			// scope the clear to non-auth cache keys only (preserve rtp_*, authStore*, auth_*),
			// also clear sessionStorage (no auth state lives there), and toast on success
			// instead of a full page reload so the operator stays in-app and signed in.
			//   action: () => {
			//   	localStorage.clear();
			//   	window.location.reload();
			//   },
			action: () => {
				if (!browser) return;
				const RESERVED_PREFIXES = ['rtp_', 'authStore', 'auth_'];
				for (const key of Object.keys(localStorage)) {
					if (!RESERVED_PREFIXES.some((p) => key.startsWith(p))) {
						localStorage.removeItem(key);
					}
				}
				// sessionStorage does not hold auth state; safe to clear wholesale.
				sessionStorage.clear();
				toastStore.success('Cache cleared');
			},
			icon: IconDatabase,
			category: 'Quick Actions'
		},
		{
			id: 'logout',
			label: 'Logout',
			action: () => goto('/logout'),
			icon: IconLogout,
			category: 'Quick Actions'
		}
	];

	// Fuzzy search function
	function fuzzyMatch(str: string, pattern: string): boolean {
		const lowerStr = str.toLowerCase();
		const lowerPattern = pattern.toLowerCase();

		let patternIdx = 0;
		for (let i = 0; i < lowerStr.length && patternIdx < lowerPattern.length; i++) {
			if (lowerStr[i] === lowerPattern[patternIdx]) {
				patternIdx++;
			}
		}
		return patternIdx === lowerPattern.length;
	}

	// Filter and combine results
	let filteredResults = $derived(
		searchQuery.trim() === ''
			? [...navigationItems.slice(0, 5), ...quickActions.slice(0, 3)]
			: [
					...navigationItems.filter(
						(item) => fuzzyMatch(item.label, searchQuery) || fuzzyMatch(item.category, searchQuery)
					),
					...quickActions.filter(
						(item) => fuzzyMatch(item.label, searchQuery) || fuzzyMatch(item.category, searchQuery)
					)
				]
	);

	// Group results by category
	let groupedResults = $derived(
		filteredResults.reduce(
			(acc, item) => {
				if (!acc[item.category]) {
					acc[item.category] = [];
				}
				acc[item.category]?.push(item);
				return acc;
			},
			{} as Record<string, typeof filteredResults>
		)
	);

	// Flat list for keyboard navigation
	let flatResults = $derived(Object.values(groupedResults).flat());

	function handleKeydown(e: KeyboardEvent) {
		if (!isOpen) return;

		switch (e.key) {
			case 'ArrowDown':
				e.preventDefault();
				selectedIndex = Math.min(selectedIndex + 1, flatResults.length - 1);
				break;
			case 'ArrowUp':
				e.preventDefault();
				selectedIndex = Math.max(selectedIndex - 1, 0);
				break;
			case 'Enter':
				e.preventDefault();
				const selected = flatResults[selectedIndex];
				if (selected) selectItem(selected);
				break;
			case 'Escape':
				e.preventDefault();
				close();
				break;
		}
	}

	function selectItem(item: (typeof navigationItems)[0] | (typeof quickActions)[0]) {
		if (searchQuery.trim()) {
			saveRecentSearch(searchQuery);
		}

		if ('path' in item) {
			goto(item.path);
		} else if ('action' in item) {
			item.action();
		}
		close();
	}

	function saveRecentSearch(query: string) {
		if (!browser) return;
		const recent = JSON.parse(localStorage.getItem('recentSearches') || '[]') as string[];
		const updated = [query, ...recent.filter((s) => s !== query)].slice(0, 5);
		localStorage.setItem('recentSearches', JSON.stringify(updated));
		// TODO: When UI displays recent searches, add state tracking here
	}

	function close() {
		isOpen = false;
		searchQuery = '';
		selectedIndex = 0;
		onclose?.();
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			close();
		}
	}

	// Global keyboard shortcut
	function handleGlobalKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			isOpen = !isOpen;
		}
	}

	function focusSearchInput(element: HTMLInputElement): void {
		element.focus();
	}

	// Initialize global keyboard shortcut
	// FIX-2026-04-26: comment-out, verify, delete in follow-up.
	// The old $effect below re-fires on every reactive run and adds a NEW keydown listener
	// each time without first removing the previous one — causing listener accumulation.
	// The separate onDestroy only cleaned up on component destroy, not between re-runs.
	// Replaced with onMount + cleanup return so the listener is registered once and torn
	// down correctly when the component is destroyed.

	onMount(() => {
		// onMount only runs in the browser; no `browser` guard needed.
		window.addEventListener('keydown', handleGlobalKeydown);
		return () => {
			window.removeEventListener('keydown', handleGlobalKeydown);
		};
	});
</script>

{#if isOpen}
	<div
		class="command-palette-overlay"
		transition:fade={{ duration: 150 }}
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		role="dialog"
		aria-modal="true"
		aria-label="Command Palette"
		tabindex="-1"
	>
		<div
			class="command-palette"
			transition:scale={{ duration: 200, start: 0.95, easing: quintOut }}
		>
			<!-- Search Input -->
			<div class="search-header">
				<div class="search-icon">
					<IconSearch size={20} />
				</div>
				<label for="command-palette-search" class="sr-only">Search commands</label>
				<input
					{@attach focusSearchInput}
					type="text"
					id="command-palette-search"
					name="command-palette-search"
					bind:value={searchQuery}
					placeholder="Search pages, actions, or type a command..."
					class="search-input"
					autocomplete="off"
					spellcheck="false"
				/>
				<div class="keyboard-hint">
					<kbd>esc</kbd>
				</div>
			</div>

			<!-- Results -->
			<div class="results-container">
				{#if flatResults.length === 0}
					<div class="no-results" in:fade={{ duration: 150 }}>
						<IconSearch size={32} />
						<p>No results found for "{searchQuery}"</p>
					</div>
				{:else}
					{#each Object.entries(groupedResults) as [category, items], categoryIndex (category)}
						<div class="results-group" in:fly={{ y: 10, duration: 200, delay: categoryIndex * 50 }}>
							<div class="group-label">{category}</div>
							{#each items as item (item.id)}
								{@const globalIndex = flatResults.indexOf(item)}
								{@const Icon = item.icon}
								<button
									class={{ 'result-item': true, selected: selectedIndex === globalIndex }}
									onclick={() => selectItem(item)}
									onmouseenter={() => (selectedIndex = globalIndex)}
								>
									<div class="item-icon">
										<Icon size={18} />
									</div>
									<span class="item-label">{item.label}</span>
									{#if selectedIndex === globalIndex}
										<div class="item-action">
											<IconArrowDown size={14} />
										</div>
									{/if}
								</button>
							{/each}
						</div>
					{/each}
				{/if}
			</div>

			<!-- Footer -->
			<div class="palette-footer">
				<div class="footer-hints">
					<span class="hint">
						<kbd><IconArrowUp size={12} /></kbd>
						<kbd><IconArrowDown size={12} /></kbd>
						Navigate
					</span>
					<span class="hint">
						<kbd><IconArrowDown size={12} /></kbd>
						Select
					</span>
					<span class="hint">
						<kbd>esc</kbd>
						Close
					</span>
				</div>
				<div class="footer-shortcut">
					<kbd><IconCode size={12} /></kbd>
					<kbd>K</kbd>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.command-palette-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding-top: 15vh;
		z-index: var(--z-popover, 500);
	}

	.command-palette {
		width: 100%;
		max-width: 640px;
		background: var(--bg-elevated, #161b22);
		border: 1px solid var(--border-default, #30363d);
		border-radius: var(--radius-2xl, 12px);
		box-shadow: var(--shadow-2xl, 0 24px 48px rgba(0, 0, 0, 0.45));
		overflow: hidden;
	}

	.search-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1.25rem;
		border-bottom: 1px solid var(--border-muted, #21262d);
	}

	/* The header row owns the focus ring so it matches the toolbar's search trigger. */
	.search-header:focus-within {
		border-bottom-color: var(--primary-500, #e6b800);
	}

	.search-icon {
		color: var(--text-tertiary, #6e7681);
	}

	.search-input {
		flex: 1;
		background: transparent;
		border: none;
		color: var(--text-primary, #f0f6fc);
		font-size: 1rem;
		font-weight: 400;
		outline: none;
		caret-color: var(--primary-500, #e6b800);
	}

	.search-input::placeholder {
		color: var(--text-tertiary, #6e7681);
	}

	.keyboard-hint kbd {
		padding: 0.25rem 0.5rem;
		background: var(--bg-base, #0d1117);
		border: 1px solid var(--border-default, #30363d);
		border-radius: var(--radius-sm, 4px);
		color: var(--text-secondary, #8b949e);
		font-size: 0.75rem;
		font-family: var(--font-mono, monospace);
		text-transform: uppercase;
	}

	.results-container {
		max-height: 400px;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.results-group {
		margin-bottom: 0.5rem;
	}

	.group-label {
		padding: 0.5rem 0.75rem;
		font-size: 0.75rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: var(--text-tertiary, #6e7681);
	}

	.result-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		min-height: 44px;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-radius: var(--radius-lg, 8px);
		color: var(--text-secondary, #8b949e);
		font-size: 0.875rem;
		text-align: left;
		cursor: pointer;
		transition: background-color 0.15s ease, color 0.15s ease;
	}

	.result-item:hover,
	.result-item.selected {
		background: var(--bg-hover, #252b33);
		color: var(--text-primary, #f0f6fc);
	}

	.result-item.selected {
		box-shadow: inset 2px 0 0 var(--primary-500, #e6b800);
	}

	.item-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--bg-surface, #1c2128);
		border-radius: var(--radius-md, 6px);
		color: var(--primary-500, #e6b800);
	}

	.result-item.selected .item-icon {
		background: var(--sidebar-active, rgba(230, 184, 0, 0.15));
	}

	.item-label {
		flex: 1;
	}

	.item-action {
		padding: 0.25rem 0.5rem;
		background: var(--sidebar-active, rgba(230, 184, 0, 0.15));
		border-radius: var(--radius-sm, 4px);
		color: var(--primary-400, #ffd11a);
		font-size: 0.75rem;
	}

	.no-results {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		color: var(--text-tertiary, #6e7681);
		text-align: center;
	}

	.no-results p {
		margin-top: 1rem;
	}

	.palette-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1.25rem;
		border-top: 1px solid var(--border-muted, #21262d);
		background: var(--bg-base, #0d1117);
	}

	.footer-hints {
		display: flex;
		gap: 1.5rem;
	}

	.hint {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.75rem;
		color: var(--text-tertiary, #6e7681);
	}

	.hint kbd,
	.footer-shortcut kbd {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 20px;
		padding: 0.125rem 0.375rem;
		background: var(--bg-surface, #1c2128);
		border: 1px solid var(--border-default, #30363d);
		border-radius: var(--radius-sm, 4px);
		color: var(--text-secondary, #8b949e);
		font-size: 0.75rem;
		font-family: var(--font-mono, monospace);
	}

	.footer-shortcut {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	@media (max-width: 639.98px) {
		.command-palette-overlay {
			padding: 1rem;
			padding-top: 5vh;
		}

		.command-palette {
			max-height: 80vh;
		}

		.footer-hints {
			display: none;
		}
	}
</style>

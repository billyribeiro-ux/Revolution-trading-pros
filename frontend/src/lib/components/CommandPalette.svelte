<script lang="ts">
	/**
	 * Command Palette - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Global search and navigation with:
	 * - Cmd/Ctrl+K activation
	 * - Fuzzy search across pages, actions, and data
	 * - Keyboard navigation
	 * - Recent searches
	 * - Action shortcuts
	 *
	 * @version 1.0.0
	 */

	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import { fade, scale, fly } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { browser } from '$app/environment';
	// Direct path imports for Svelte 5 compatibility
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconCommand from '@tabler/icons-svelte/icons/command';
	import IconArrowRight from '@tabler/icons-svelte/icons/arrow-right';
	import IconCornerDownLeft from '@tabler/icons-svelte/icons/corner-down-left';
	import IconArrowUp from '@tabler/icons-svelte/icons/arrow-up';
	import IconArrowDown from '@tabler/icons-svelte/icons/arrow-down';
	import IconX from '@tabler/icons-svelte/icons/x';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconHome from '@tabler/icons-svelte/icons/home';
	import IconChartBar from '@tabler/icons-svelte/icons/chart-bar';
	import IconUsers from '@tabler/icons-svelte/icons/users';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconSettings from '@tabler/icons-svelte/icons/settings';
	import IconShoppingCart from '@tabler/icons-svelte/icons/shopping-cart';
	import IconFileText from '@tabler/icons-svelte/icons/file-text';
	import IconPhoto from '@tabler/icons-svelte/icons/photo';
	import IconPlugConnected from '@tabler/icons-svelte/icons/plug-connected';
	import IconActivity from '@tabler/icons-svelte/icons/activity';
	import IconBrandGoogle from '@tabler/icons-svelte/icons/brand-google';
	import IconTrendingUp from '@tabler/icons-svelte/icons/trending-up';
	import IconForms from '@tabler/icons-svelte/icons/forms';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconCreditCard from '@tabler/icons-svelte/icons/credit-card';
	import IconDatabase from '@tabler/icons-svelte/icons/database';
	import IconRefresh from '@tabler/icons-svelte/icons/refresh';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import IconUpload from '@tabler/icons-svelte/icons/upload';
	import IconLogout from '@tabler/icons-svelte/icons/logout';

	interface Props {
		isOpen?: boolean;
		onclose?: () => void;
	}

	let { isOpen = $bindable(false), onclose }: Props = $props();

	let searchQuery = $state('');
	let selectedIndex = $state(0);
	let inputRef = $state<HTMLInputElement | null>(null);
	let recentSearches = $state<string[]>([]);

	// Navigation items
	const navigationItems = [
		{ id: 'dashboard', label: 'Dashboard', path: '/admin', icon: IconHome, category: 'Navigation' },
		{ id: 'analytics', label: 'Analytics', path: '/admin/analytics', icon: IconChartBar, category: 'Navigation' },
		{ id: 'seo', label: 'SEO Settings', path: '/admin/seo', icon: IconBrandGoogle, category: 'Navigation' },
		{ id: 'behavior', label: 'Behavior Tracking', path: '/admin/behavior', icon: IconActivity, category: 'Navigation' },
		{ id: 'blog', label: 'Blog Posts', path: '/admin/blog', icon: IconFileText, category: 'Content' },
		{ id: 'media', label: 'Media Library', path: '/admin/media', icon: IconPhoto, category: 'Content' },
		{ id: 'videos', label: 'Videos', path: '/admin/videos', icon: IconVideo, category: 'Content' },
		{ id: 'forms', label: 'Forms', path: '/admin/forms', icon: IconForms, category: 'Content' },
		{ id: 'members', label: 'Members', path: '/admin/members', icon: IconUsers, category: 'Users' },
		{ id: 'crm', label: 'CRM', path: '/admin/crm', icon: IconUsers, category: 'Users' },
		{ id: 'campaigns', label: 'Email Campaigns', path: '/admin/email/campaigns', icon: IconMail, category: 'Marketing' },
		{ id: 'templates', label: 'Email Templates', path: '/admin/email/templates', icon: IconMail, category: 'Marketing' },
		{ id: 'products', label: 'Products', path: '/admin/products', icon: IconShoppingCart, category: 'Commerce' },
		{ id: 'subscriptions', label: 'Subscriptions', path: '/admin/subscriptions', icon: IconCreditCard, category: 'Commerce' },
		{ id: 'connections', label: 'API Connections', path: '/admin/connections', icon: IconPlugConnected, category: 'Settings' },
		{ id: 'site-health', label: 'Site Health', path: '/admin/site-health', icon: IconActivity, category: 'Settings' },
		{ id: 'settings', label: 'Settings', path: '/admin/settings', icon: IconSettings, category: 'Settings' }
	];

	// Quick actions
	const quickActions = [
		{ id: 'new-post', label: 'Create New Post', action: () => goto('/admin/blog/new'), icon: IconFileText, category: 'Quick Actions' },
		{ id: 'new-campaign', label: 'Create Email Campaign', action: () => goto('/admin/email/campaigns?new=true'), icon: IconMail, category: 'Quick Actions' },
		{ id: 'upload-media', label: 'Upload Media', action: () => goto('/admin/media?upload=true'), icon: IconUpload, category: 'Quick Actions' },
		{ id: 'export-data', label: 'Export Data', action: () => goto('/admin/settings?export=true'), icon: IconDownload, category: 'Quick Actions' },
		{ id: 'refresh-all', label: 'Refresh All Data', action: () => window.location.reload(), icon: IconRefresh, category: 'Quick Actions' },
		{ id: 'clear-cache', label: 'Clear Cache', action: () => { localStorage.clear(); window.location.reload(); }, icon: IconDatabase, category: 'Quick Actions' },
		{ id: 'logout', label: 'Logout', action: () => goto('/logout'), icon: IconLogout, category: 'Quick Actions' }
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
	let filteredResults = $derived(searchQuery.trim() === ''
		? [...navigationItems.slice(0, 5), ...quickActions.slice(0, 3)]
		: [
			...navigationItems.filter(item =>
				fuzzyMatch(item.label, searchQuery) ||
				fuzzyMatch(item.category, searchQuery)
			),
			...quickActions.filter(item =>
				fuzzyMatch(item.label, searchQuery) ||
				fuzzyMatch(item.category, searchQuery)
			)
		]);

	// Group results by category
	let groupedResults = $derived(filteredResults.reduce((acc, item) => {
		if (!acc[item.category]) {
			acc[item.category] = [];
		}
		acc[item.category].push(item);
		return acc;
	}, {} as Record<string, typeof filteredResults>));

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
				selectItem(flatResults[selectedIndex]);
				break;
			case 'Escape':
				e.preventDefault();
				close();
				break;
		}
	}

	function selectItem(item: typeof navigationItems[0] | typeof quickActions[0]) {
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
		const updated = [query, ...recent.filter(s => s !== query)].slice(0, 5);
		localStorage.setItem('recentSearches', JSON.stringify(updated));
		recentSearches = updated;
	}

	function loadRecentSearches() {
		if (!browser) return;
		recentSearches = JSON.parse(localStorage.getItem('recentSearches') || '[]');
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

	$effect(() => {
		if (isOpen && inputRef) {
			setTimeout(() => inputRef?.focus(), 50);
		}
	});

	onMount(() => {
		loadRecentSearches();
		if (browser) {
			window.addEventListener('keydown', handleGlobalKeydown);
		}
	});

	onDestroy(() => {
		if (browser) {
			window.removeEventListener('keydown', handleGlobalKeydown);
		}
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
				<input
					bind:this={inputRef}
					type="text"
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
					{#each Object.entries(groupedResults) as [category, items], categoryIndex}
						<div class="results-group" in:fly={{ y: 10, duration: 200, delay: categoryIndex * 50 }}>
							<div class="group-label">{category}</div>
							{#each items as item, itemIndex}
								{@const globalIndex = flatResults.indexOf(item)}
								{@const Icon = item.icon}
								<button
									class="result-item"
									class:selected={selectedIndex === globalIndex}
									onclick={() => selectItem(item)}
									onmouseenter={() => selectedIndex = globalIndex}
								>
									<div class="item-icon">
										<Icon size={18} />
									</div>
									<span class="item-label">{item.label}</span>
									{#if selectedIndex === globalIndex}
										<div class="item-action">
											<IconCornerDownLeft size={14} />
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
						<kbd><IconCornerDownLeft size={12} /></kbd>
						Select
					</span>
					<span class="hint">
						<kbd>esc</kbd>
						Close
					</span>
				</div>
				<div class="footer-shortcut">
					<kbd><IconCommand size={12} /></kbd>
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
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding-top: 15vh;
		z-index: 9999;
	}

	.command-palette {
		width: 100%;
		max-width: 640px;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.98), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 16px;
		box-shadow:
			0 25px 50px -12px rgba(0, 0, 0, 0.5),
			0 0 0 1px rgba(255, 255, 255, 0.05) inset;
		overflow: hidden;
	}

	.search-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem;
		border-bottom: 1px solid rgba(99, 102, 241, 0.1);
	}

	.search-icon {
		color: #64748b;
	}

	.search-input {
		flex: 1;
		background: transparent;
		border: none;
		color: #f1f5f9;
		font-size: 1.125rem;
		font-weight: 400;
		outline: none;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.keyboard-hint kbd {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #94a3b8;
		font-size: 0.6875rem;
		font-family: inherit;
		text-transform: uppercase;
	}

	.results-container {
		max-height: 400px;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.results-container::-webkit-scrollbar {
		width: 6px;
	}

	.results-container::-webkit-scrollbar-track {
		background: transparent;
	}

	.results-container::-webkit-scrollbar-thumb {
		background: rgba(99, 102, 241, 0.3);
		border-radius: 3px;
	}

	.results-group {
		margin-bottom: 0.5rem;
	}

	.group-label {
		padding: 0.5rem 0.75rem;
		font-size: 0.6875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #64748b;
	}

	.result-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.75rem;
		background: transparent;
		border: none;
		border-radius: 10px;
		color: #cbd5e1;
		font-size: 0.9375rem;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.result-item:hover,
	.result-item.selected {
		background: rgba(99, 102, 241, 0.15);
		color: #f1f5f9;
	}

	.item-icon {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border-radius: 8px;
		color: #818cf8;
	}

	.result-item.selected .item-icon {
		background: rgba(99, 102, 241, 0.2);
	}

	.item-label {
		flex: 1;
	}

	.item-action {
		padding: 0.25rem 0.5rem;
		background: rgba(99, 102, 241, 0.2);
		border-radius: 6px;
		color: #a5b4fc;
	}

	.no-results {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		color: #64748b;
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
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		background: rgba(15, 23, 42, 0.5);
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
		color: #64748b;
	}

	.hint kbd,
	.footer-shortcut kbd {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 20px;
		padding: 0.125rem 0.375rem;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 4px;
		color: #94a3b8;
		font-size: 0.625rem;
		font-family: inherit;
	}

	.footer-shortcut {
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	@media (max-width: 640px) {
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

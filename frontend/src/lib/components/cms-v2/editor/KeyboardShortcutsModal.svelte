<script lang="ts">
	/**
	 * Keyboard Shortcuts Help Modal - CMS v2 Editor
	 * ==============================================
	 * Comprehensive keyboard shortcuts reference modal with
	 * platform-aware display, search, and accessibility support.
	 *
	 * @version 1.0.0
	 * @author Revolution Trading Pros
	 */

	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// Types
	// ═══════════════════════════════════════════════════════════════════════════

	interface Shortcut {
		keys: string[];
		description: string;
	}

	interface ShortcutCategory {
		name: string;
		icon: string;
		shortcuts: Shortcut[];
	}

	interface Props {
		isOpen: boolean;
		onClose: () => void;
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Props & State
	// ═══════════════════════════════════════════════════════════════════════════

	let { isOpen, onClose }: Props = $props();

	let searchQuery = $state('');
	let searchInputRef = $state<HTMLInputElement | null>(null);
	let modalRef = $state<HTMLDivElement | null>(null);
	let previouslyFocusedElement = $state<HTMLElement | null>(null);

	// ═══════════════════════════════════════════════════════════════════════════
	// Platform Detection
	// ═══════════════════════════════════════════════════════════════════════════

	const isMac = browser && /Mac|iPod|iPhone|iPad/.test(navigator.platform);
	const modKey = isMac ? '\u2318' : 'Ctrl';
	const shiftKey = isMac ? '\u21E7' : 'Shift';
	const altKey = isMac ? '\u2325' : 'Alt';

	// ═══════════════════════════════════════════════════════════════════════════
	// Shortcut Categories Data
	// ═══════════════════════════════════════════════════════════════════════════

	const shortcutCategories: ShortcutCategory[] = [
		{
			name: 'System',
			icon: '\u2699',
			shortcuts: [
				{ keys: [modKey, 'S'], description: 'Save' },
				{ keys: [modKey, shiftKey, 'S'], description: 'Save and Publish' },
				{ keys: [modKey, 'Z'], description: 'Undo' },
				{ keys: [modKey, shiftKey, 'Z'], description: 'Redo' },
				{ keys: [modKey, 'P'], description: 'Preview' },
				{ keys: [modKey, 'Enter'], description: 'Publish' },
				{ keys: [modKey, '/'], description: 'Open slash commands' },
				{ keys: [modKey, 'K'], description: 'Quick actions' },
				{ keys: ['Escape'], description: 'Deselect/Close' },
				{ keys: [modKey, '?'], description: 'Show shortcuts' }
			]
		},
		{
			name: 'Formatting',
			icon: '\u270E',
			shortcuts: [
				{ keys: [modKey, 'B'], description: 'Bold' },
				{ keys: [modKey, 'I'], description: 'Italic' },
				{ keys: [modKey, 'U'], description: 'Underline' },
				{ keys: [modKey, 'E'], description: 'Inline code' },
				{ keys: [modKey, shiftKey, '1'], description: 'Heading 1' },
				{ keys: [modKey, shiftKey, '2'], description: 'Heading 2' },
				{ keys: [modKey, shiftKey, '3'], description: 'Heading 3' },
				{ keys: [modKey, shiftKey, '7'], description: 'Bullet list' },
				{ keys: [modKey, shiftKey, '8'], description: 'Numbered list' },
				{ keys: [modKey, shiftKey, '9'], description: 'Blockquote' }
			]
		},
		{
			name: 'Blocks',
			icon: '\u25A1',
			shortcuts: [
				{ keys: [modKey, 'D'], description: 'Duplicate block' },
				{ keys: [modKey, 'Backspace'], description: 'Delete block' },
				{ keys: [modKey, shiftKey, '\u2191'], description: 'Move block up' },
				{ keys: [modKey, shiftKey, '\u2193'], description: 'Move block down' },
				{ keys: [modKey, 'G'], description: 'Group blocks' },
				{ keys: [modKey, shiftKey, 'G'], description: 'Ungroup' }
			]
		},
		{
			name: 'Navigation',
			icon: '\u2194',
			shortcuts: [
				{ keys: [altKey, '\u2191'], description: 'Select previous block' },
				{ keys: [altKey, '\u2193'], description: 'Select next block' },
				{ keys: [modKey, 'Home'], description: 'First block' },
				{ keys: [modKey, 'End'], description: 'Last block' },
				{ keys: [modKey, 'F'], description: 'Find' },
				{ keys: [modKey, shiftKey, 'F'], description: 'Find and replace' }
			]
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// Filtered Results
	// ═══════════════════════════════════════════════════════════════════════════

	let filteredCategories = $derived(() => {
		if (!searchQuery.trim()) return shortcutCategories;

		const query = searchQuery.toLowerCase().trim();

		return shortcutCategories
			.map((category) => ({
				...category,
				shortcuts: category.shortcuts.filter(
					(shortcut) =>
						shortcut.description.toLowerCase().includes(query) ||
						shortcut.keys.join(' ').toLowerCase().includes(query)
				)
			}))
			.filter((category) => category.shortcuts.length > 0);
	});

	let totalShortcutsCount = $derived(() => {
		return filteredCategories().reduce((acc, cat) => acc + cat.shortcuts.length, 0);
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Event Handlers
	// ═══════════════════════════════════════════════════════════════════════════

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			closeModal();
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			e.preventDefault();
			e.stopPropagation();
			closeModal();
		}
	}

	function closeModal() {
		searchQuery = '';
		onClose();
		// Restore focus to previously focused element
		if (previouslyFocusedElement) {
			previouslyFocusedElement.focus();
		}
	}

	function clearSearch() {
		searchQuery = '';
		searchInputRef?.focus();
	}

	// Global keyboard shortcut handler
	function handleGlobalKeydown(e: KeyboardEvent) {
		// Open modal with Cmd+? or Ctrl+?
		if ((e.metaKey || e.ctrlKey) && e.shiftKey && e.key === '/') {
			e.preventDefault();
			if (!isOpen) {
				previouslyFocusedElement = document.activeElement as HTMLElement;
			}
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle & Effects
	// ═══════════════════════════════════════════════════════════════════════════

	$effect(() => {
		if (isOpen && searchInputRef) {
			// Save currently focused element before opening
			if (!previouslyFocusedElement) {
				previouslyFocusedElement = document.activeElement as HTMLElement;
			}
			// Focus search input when modal opens
			setTimeout(() => searchInputRef?.focus(), 50);
		}
	});

	// Trap focus within modal
	$effect(() => {
		if (!isOpen || !modalRef) return;

		const focusableElements = modalRef.querySelectorAll<HTMLElement>(
			'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
		);
		const firstFocusable = focusableElements[0];
		const lastFocusable = focusableElements[focusableElements.length - 1];

		function trapFocus(e: KeyboardEvent) {
			if (e.key !== 'Tab') return;

			if (e.shiftKey) {
				if (document.activeElement === firstFocusable) {
					e.preventDefault();
					lastFocusable?.focus();
				}
			} else {
				if (document.activeElement === lastFocusable) {
					e.preventDefault();
					firstFocusable?.focus();
				}
			}
		}

		modalRef.addEventListener('keydown', trapFocus);
		return () => modalRef?.removeEventListener('keydown', trapFocus);
	});

	// Lock body scroll when modal is open
	$effect(() => {
		if (!browser) return;

		if (isOpen) {
			document.body.style.overflow = 'hidden';
		} else {
			document.body.style.overflow = '';
		}

		return () => {
			document.body.style.overflow = '';
		};
	});

	onMount(() => {
		if (browser) {
			window.addEventListener('keydown', handleGlobalKeydown);
		}
	});

	onDestroy(() => {
		if (browser) {
			window.removeEventListener('keydown', handleGlobalKeydown);
			document.body.style.overflow = '';
		}
	});
</script>

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm
		       transition-opacity duration-200 ease-out"
		onclick={handleBackdropClick}
		onkeydown={handleKeydown}
		role="presentation"
		style="animation: fadeIn 0.2s ease-out"
	>
		<!-- Modal -->
		<div
			bind:this={modalRef}
			class="relative mx-4 flex max-h-[85vh] w-full max-w-2xl flex-col overflow-hidden
			       rounded-2xl bg-white shadow-2xl ring-1 ring-black/5
			       dark:bg-slate-900 dark:ring-white/10"
			role="dialog"
			aria-modal="true"
			aria-labelledby="shortcuts-modal-title"
			aria-describedby="shortcuts-modal-description"
			tabindex="-1"
			style="animation: slideUp 0.25s ease-out"
		>
			<!-- Header -->
			<div class="flex items-center justify-between border-b border-slate-200 px-6 py-4
			            dark:border-slate-700">
				<div>
					<h2
						id="shortcuts-modal-title"
						class="text-lg font-semibold text-slate-900 dark:text-white"
					>
						Keyboard Shortcuts
					</h2>
					<p
						id="shortcuts-modal-description"
						class="mt-0.5 text-sm text-slate-500 dark:text-slate-400"
					>
						{#if isMac}
							Showing macOS shortcuts
						{:else}
							Showing Windows/Linux shortcuts
						{/if}
					</p>
				</div>
				<button
					type="button"
					onclick={closeModal}
					class="flex h-9 w-9 items-center justify-center rounded-lg text-slate-400
					       transition-colors hover:bg-slate-100 hover:text-slate-600
					       focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2
					       dark:hover:bg-slate-800 dark:hover:text-slate-300"
					aria-label="Close modal"
				>
					<svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Search Bar -->
			<div class="border-b border-slate-200 px-6 py-3 dark:border-slate-700">
				<div class="relative">
					<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<svg class="h-5 w-5 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
							      d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
						</svg>
					</div>
					<input
						bind:this={searchInputRef}
						type="text"
						bind:value={searchQuery}
						placeholder="Search shortcuts..."
						class="block w-full rounded-lg border border-slate-300 bg-slate-50 py-2.5 pl-10 pr-10
						       text-sm text-slate-900 placeholder-slate-400
						       transition-colors focus:border-indigo-500 focus:bg-white focus:outline-none
						       focus:ring-2 focus:ring-indigo-500/20
						       dark:border-slate-600 dark:bg-slate-800 dark:text-white
						       dark:placeholder-slate-500 dark:focus:border-indigo-400"
						aria-label="Search keyboard shortcuts"
					/>
					{#if searchQuery}
						<button
							type="button"
							onclick={clearSearch}
							class="absolute inset-y-0 right-0 flex items-center pr-3 text-slate-400
							       transition-colors hover:text-slate-600 dark:hover:text-slate-300"
							aria-label="Clear search"
						>
							<svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
							</svg>
						</button>
					{/if}
				</div>
			</div>

			<!-- Shortcuts List -->
			<div class="flex-1 overflow-y-auto px-6 py-4" role="list">
				{#each filteredCategories() as category, categoryIndex (category.name)}
					<div
						class="mb-6 last:mb-0"
						role="listitem"
						style="animation: fadeSlideIn 0.2s ease-out {categoryIndex * 0.05}s both"
					>
						<!-- Category Header -->
						<div class="mb-3 flex items-center gap-2">
							<span class="text-base" aria-hidden="true">{category.icon}</span>
							<h3 class="text-xs font-semibold uppercase tracking-wider text-slate-500
							           dark:text-slate-400">
								{category.name}
							</h3>
							<span class="ml-auto rounded-full bg-slate-100 px-2 py-0.5 text-xs font-medium
							             text-slate-600 dark:bg-slate-800 dark:text-slate-400">
								{category.shortcuts.length}
							</span>
						</div>

						<!-- Shortcuts Grid -->
						<div class="grid gap-2">
							{#each category.shortcuts as shortcut (shortcut.description)}
								<div
									class="group flex items-center justify-between rounded-lg bg-slate-50 px-4 py-3
									       transition-colors hover:bg-slate-100
									       dark:bg-slate-800/50 dark:hover:bg-slate-800"
								>
									<!-- Key Combination -->
									<div class="flex items-center gap-1.5">
										{#each shortcut.keys as key, keyIndex}
											<kbd
												class="inline-flex min-w-[28px] items-center justify-center rounded-md
												       border border-slate-300 bg-white px-2 py-1.5
												       text-xs font-medium text-slate-700 shadow-sm
												       dark:border-slate-600 dark:bg-slate-700 dark:text-slate-200"
											>
												{key}
											</kbd>
											{#if keyIndex < shortcut.keys.length - 1}
												<span class="text-xs text-slate-400">+</span>
											{/if}
										{/each}
									</div>

									<!-- Description -->
									<span class="text-sm text-slate-600 dark:text-slate-300">
										{shortcut.description}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{:else}
					<!-- No Results -->
					<div class="flex flex-col items-center justify-center py-12 text-center">
						<div class="mb-4 rounded-full bg-slate-100 p-4 dark:bg-slate-800">
							<svg class="h-8 w-8 text-slate-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5"
								      d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
							</svg>
						</div>
						<p class="text-sm font-medium text-slate-900 dark:text-white">
							No shortcuts found
						</p>
						<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
							No results for "{searchQuery}"
						</p>
						<button
							type="button"
							onclick={clearSearch}
							class="mt-4 text-sm font-medium text-indigo-600 hover:text-indigo-500
							       dark:text-indigo-400 dark:hover:text-indigo-300"
						>
							Clear search
						</button>
					</div>
				{/each}
			</div>

			<!-- Footer -->
			<div class="flex items-center justify-between border-t border-slate-200 bg-slate-50 px-6 py-3
			            dark:border-slate-700 dark:bg-slate-800/50">
				<div class="flex items-center gap-4 text-xs text-slate-500 dark:text-slate-400">
					<span class="flex items-center gap-1.5">
						<kbd class="inline-flex items-center justify-center rounded border border-slate-300
						           bg-white px-1.5 py-0.5 text-[10px] font-medium text-slate-600
						           dark:border-slate-600 dark:bg-slate-700 dark:text-slate-300">
							Esc
						</kbd>
						<span>Close</span>
					</span>
					<span class="flex items-center gap-1.5">
						<kbd class="inline-flex items-center justify-center rounded border border-slate-300
						           bg-white px-1.5 py-0.5 text-[10px] font-medium text-slate-600
						           dark:border-slate-600 dark:bg-slate-700 dark:text-slate-300">
							{modKey}
						</kbd>
						<kbd class="inline-flex items-center justify-center rounded border border-slate-300
						           bg-white px-1.5 py-0.5 text-[10px] font-medium text-slate-600
						           dark:border-slate-600 dark:bg-slate-700 dark:text-slate-300">
							?
						</kbd>
						<span>Toggle</span>
					</span>
				</div>
				<div class="text-xs text-slate-500 dark:text-slate-400">
					{totalShortcutsCount()} shortcuts
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(16px) scale(0.98);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	@keyframes fadeSlideIn {
		from {
			opacity: 0;
			transform: translateY(8px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Custom scrollbar for the shortcuts list */
	:global(.dark) div[role="list"]::-webkit-scrollbar {
		width: 8px;
	}

	:global(.dark) div[role="list"]::-webkit-scrollbar-track {
		background: transparent;
	}

	:global(.dark) div[role="list"]::-webkit-scrollbar-thumb {
		background: rgba(100, 116, 139, 0.5);
		border-radius: 4px;
	}

	:global(.dark) div[role="list"]::-webkit-scrollbar-thumb:hover {
		background: rgba(100, 116, 139, 0.7);
	}

	div[role="list"]::-webkit-scrollbar {
		width: 8px;
	}

	div[role="list"]::-webkit-scrollbar-track {
		background: transparent;
	}

	div[role="list"]::-webkit-scrollbar-thumb {
		background: rgba(203, 213, 225, 0.8);
		border-radius: 4px;
	}

	div[role="list"]::-webkit-scrollbar-thumb:hover {
		background: rgba(148, 163, 184, 0.8);
	}
</style>

<!--
/**
 * Conflict Resolution Modal - CMS Editor Enhancement
 * ═══════════════════════════════════════════════════════════════════════════
 * Modal for resolving sync conflicts when someone else edited content while
 * the user was offline.
 *
 * FEATURES:
 * - Clear explanation of the conflict
 * - Display of who made the conflicting edit and when
 * - Three resolution options: Keep mine, Keep theirs, View diff
 * - Side-by-side preview of both versions
 * - Warning about data loss
 * - Confirmation step before applying resolution
 * - Loading state during resolution
 * - Error handling with retry option
 * - Full accessibility with focus trap and ARIA attributes
 *
 * @version 1.0.0
 * @author Revolution Trading Pros
 */
-->

<script module lang="ts">
	// Re-export types from the centralized types file
	export type { ContentData, ResolutionType } from './types';
</script>

<script lang="ts">
	import type { ContentData, ResolutionType } from './types';
	import { onMount, tick } from 'svelte';
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		IconAlertTriangle,
		IconCheck,
		IconClock,
		IconFileDiff,
		IconLoader2,
		IconRefresh,
		IconUser,
		IconUserCircle,
		IconX
	} from '$lib/icons';

	// ==========================================================================
	// Types (Internal)
	// ==========================================================================

	interface Props {
		/** Whether the modal is open */
		isOpen: boolean;
		/** The user's local version of the content */
		localData: ContentData;
		/** The server's version of the content */
		serverData: ContentData;
		/** Information about who made the server edit */
		serverEditedBy: { name: string; avatar?: string };
		/** When the server edit was made */
		serverEditedAt: string;
		/** Callback when user resolves the conflict */
		onResolve: (resolution: ResolutionType) => Promise<void>;
		/** Callback to close the modal */
		onClose: () => void;
	}

	let { isOpen, localData, serverData, serverEditedBy, serverEditedAt, onResolve, onClose }: Props =
		$props();

	// ==========================================================================
	// State
	// ==========================================================================

	let isResolving = $state(false);
	let selectedResolution = $state<ResolutionType | null>(null);
	let showConfirmation = $state(false);
	let error = $state<string | null>(null);
	let modalRef = $state<HTMLDivElement | null>(null);
	let previousFocusElement = $state<HTMLElement | null>(null);

	// ==========================================================================
	// Derived Values
	// ==========================================================================

	/** Check which fields differ between local and server versions */
	let differences = $derived.by(() => {
		const diffs: { field: string; label: string; localValue: string; serverValue: string }[] = [];

		// Title
		if (localData.title !== serverData.title) {
			diffs.push({
				field: 'title',
				label: 'Title',
				localValue: localData.title || '(empty)',
				serverValue: serverData.title || '(empty)'
			});
		}

		// Slug
		if (localData.slug !== serverData.slug) {
			diffs.push({
				field: 'slug',
				label: 'URL Slug',
				localValue: localData.slug || '(empty)',
				serverValue: serverData.slug || '(empty)'
			});
		}

		// Excerpt
		if (localData.excerpt !== serverData.excerpt) {
			diffs.push({
				field: 'excerpt',
				label: 'Excerpt',
				localValue: truncateText(localData.excerpt || '', 100),
				serverValue: truncateText(serverData.excerpt || '', 100)
			});
		}

		// Content (preview)
		if (localData.content !== serverData.content) {
			diffs.push({
				field: 'content',
				label: 'Content',
				localValue: truncateText(stripHtml(localData.content || ''), 150),
				serverValue: truncateText(stripHtml(serverData.content || ''), 150)
			});
		}

		// Status
		if (localData.status !== serverData.status) {
			diffs.push({
				field: 'status',
				label: 'Status',
				localValue: formatStatus(localData.status),
				serverValue: formatStatus(serverData.status)
			});
		}

		return diffs;
	});

	/** Format the server edit timestamp */
	let formattedServerTime = $derived.by(() => {
		try {
			const date = new Date(serverEditedAt);
			const now = new Date();
			const diffMs = now.getTime() - date.getTime();
			const diffMins = Math.floor(diffMs / 60000);
			const diffHours = Math.floor(diffMs / 3600000);
			const diffDays = Math.floor(diffMs / 86400000);

			if (diffMins < 1) return 'just now';
			if (diffMins < 60) return `${diffMins} minute${diffMins === 1 ? '' : 's'} ago`;
			if (diffHours < 24) return `${diffHours} hour${diffHours === 1 ? '' : 's'} ago`;
			if (diffDays < 7) return `${diffDays} day${diffDays === 1 ? '' : 's'} ago`;

			return date.toLocaleDateString('en-US', {
				month: 'short',
				day: 'numeric',
				year: date.getFullYear() !== now.getFullYear() ? 'numeric' : undefined,
				hour: 'numeric',
				minute: '2-digit'
			});
		} catch {
			return serverEditedAt;
		}
	});

	/** Get resolution button styles based on selection state */
	let resolutionButtonClasses = $derived.by(() => ({
		mine:
			selectedResolution === 'mine'
				? 'border-emerald-500 bg-emerald-500/10 ring-2 ring-emerald-500/30'
				: 'border-slate-700 bg-slate-800/50 hover:border-emerald-500/50 hover:bg-emerald-500/5',
		theirs:
			selectedResolution === 'theirs'
				? 'border-blue-500 bg-blue-500/10 ring-2 ring-blue-500/30'
				: 'border-slate-700 bg-slate-800/50 hover:border-blue-500/50 hover:bg-blue-500/5',
		merge:
			selectedResolution === 'merge'
				? 'border-amber-500 bg-amber-500/10 ring-2 ring-amber-500/30'
				: 'border-slate-700 bg-slate-800/50 hover:border-amber-500/50 hover:bg-amber-500/5'
	}));

	// ==========================================================================
	// Helper Functions
	// ==========================================================================

	function truncateText(text: string, maxLength: number): string {
		if (!text || text.length <= maxLength) return text || '(empty)';
		return text.slice(0, maxLength).trim() + '...';
	}

	function stripHtml(html: string): string {
		return html.replace(/<[^>]*>/g, '').trim();
	}

	function formatStatus(status?: string): string {
		if (!status) return '(unknown)';
		return status.replace(/_/g, ' ').replace(/\b\w/g, (l) => l.toUpperCase());
	}

	// ==========================================================================
	// Event Handlers
	// ==========================================================================

	function handleSelectResolution(resolution: ResolutionType): void {
		if (isResolving) return;
		selectedResolution = resolution;
		error = null;
	}

	function handleConfirm(): void {
		if (!selectedResolution || isResolving) return;
		showConfirmation = true;
	}

	async function handleApplyResolution(): Promise<void> {
		if (!selectedResolution || isResolving) return;

		isResolving = true;
		error = null;

		try {
			await onResolve(selectedResolution);
			// Success - modal will be closed by parent
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to resolve conflict. Please try again.';
			showConfirmation = false;
		} finally {
			isResolving = false;
		}
	}

	function handleCancel(): void {
		if (isResolving) return;
		showConfirmation = false;
		selectedResolution = null;
		error = null;
	}

	function handleClose(): void {
		if (isResolving) return;
		onClose();
	}

	function handleBackdropClick(event: MouseEvent): void {
		if (event.target === event.currentTarget && !isResolving) {
			handleClose();
		}
	}

	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape' && !isResolving && !showConfirmation) {
			handleClose();
		}
	}

	// ==========================================================================
	// Focus Management
	// ==========================================================================

	function trapFocus(event: KeyboardEvent): void {
		if (event.key !== 'Tab' || !modalRef) return;

		const focusableElements = modalRef.querySelectorAll<HTMLElement>(
			'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
		);

		const firstElement = focusableElements[0];
		const lastElement = focusableElements[focusableElements.length - 1];

		if (event.shiftKey && document.activeElement === firstElement) {
			event.preventDefault();
			lastElement?.focus();
		} else if (!event.shiftKey && document.activeElement === lastElement) {
			event.preventDefault();
			firstElement?.focus();
		}
	}

	// ==========================================================================
	// Effects
	// ==========================================================================

	$effect(() => {
		if (isOpen && typeof document !== 'undefined') {
			// Store previous focus
			previousFocusElement = document.activeElement as HTMLElement;

			// Prevent body scroll
			document.body.style.overflow = 'hidden';

			// Focus the modal after it renders
			tick().then(() => {
				modalRef?.focus();
			});
		}

		return () => {
			if (typeof document !== 'undefined') {
				document.body.style.overflow = '';
				// Restore focus
				previousFocusElement?.focus();
			}
		};
	});

	// Reset state when modal opens
	$effect(() => {
		if (isOpen) {
			selectedResolution = null;
			showConfirmation = false;
			error = null;
			isResolving = false;
		}
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
	<!-- Backdrop -->
	<div
		class="fixed inset-0 z-[1000] flex items-center justify-center p-4 bg-black/80 backdrop-blur-sm"
		onclick={handleBackdropClick}
		onkeydown={trapFocus}
		role="presentation"
		transition:fade={{ duration: 200 }}
	>
		<!-- Modal Container -->
		<div
			bind:this={modalRef}
			class="relative w-full max-w-4xl max-h-[90vh] bg-gradient-to-br from-slate-900 via-slate-900 to-slate-800 border border-slate-700/50 rounded-2xl shadow-2xl overflow-hidden flex flex-col"
			role="dialog"
			aria-modal="true"
			aria-labelledby="conflict-modal-title"
			aria-describedby="conflict-modal-description"
			tabindex="-1"
			transition:fly={{ y: 20, duration: 300, easing: quintOut }}
		>
			<!-- Header -->
			<header
				class="flex items-center justify-between px-6 py-4 border-b border-slate-700/50 bg-slate-800/30"
			>
				<div class="flex items-center gap-3">
					<div
						class="flex items-center justify-center w-10 h-10 rounded-full bg-amber-500/10 text-amber-500"
					>
						<IconAlertTriangle size={22} />
					</div>
					<div>
						<h2 id="conflict-modal-title" class="text-lg font-semibold text-white">
							Sync Conflict Detected
						</h2>
						<p id="conflict-modal-description" class="text-sm text-slate-400">
							This content has been modified by someone else
						</p>
					</div>
				</div>
				<button
					type="button"
					class="flex items-center justify-center w-9 h-9 rounded-lg text-slate-400 hover:text-white hover:bg-slate-700/50 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
					onclick={handleClose}
					disabled={isResolving}
					aria-label="Close modal"
				>
					<IconX size={20} />
				</button>
			</header>

			<!-- Content -->
			<div class="flex-1 overflow-y-auto p-6 space-y-6">
				<!-- Conflict Info Card -->
				<div class="p-4 rounded-xl bg-slate-800/50 border border-slate-700/50">
					<div class="flex items-start gap-4">
						<!-- Avatar -->
						<div class="flex-shrink-0">
							{#if serverEditedBy.avatar}
								<img
									src={serverEditedBy.avatar}
									alt={serverEditedBy.name}
									class="w-12 h-12 rounded-full object-cover border-2 border-slate-600"
								/>
							{:else}
								<div
									class="w-12 h-12 rounded-full bg-slate-700 flex items-center justify-center text-slate-400"
								>
									<IconUserCircle size={28} />
								</div>
							{/if}
						</div>

						<!-- Info -->
						<div class="flex-1 min-w-0">
							<p class="text-slate-300">
								<span class="font-medium text-white">{serverEditedBy.name}</span>
								{' '}made changes to this content
							</p>
							<div class="flex items-center gap-2 mt-1 text-sm text-slate-400">
								<IconClock size={14} />
								<span>{formattedServerTime}</span>
								{#if serverData.version}
									<span class="text-slate-500">|</span>
									<span>Version {serverData.version}</span>
								{/if}
							</div>
						</div>
					</div>
				</div>

				<!-- Warning Banner -->
				<div class="p-4 rounded-xl bg-amber-500/10 border border-amber-500/30">
					<div class="flex items-start gap-3">
						<IconAlertTriangle size={20} class="flex-shrink-0 text-amber-500 mt-0.5" />
						<div class="text-sm text-amber-200/90">
							<p class="font-medium text-amber-100">Warning: Potential Data Loss</p>
							<p class="mt-1">
								Choosing "Keep mine" or "Keep theirs" will overwrite one version permanently. If you
								need to combine changes from both versions, select "View diff" to merge manually.
							</p>
						</div>
					</div>
				</div>

				<!-- Differences Preview -->
				{#if differences.length > 0}
					<div class="space-y-4">
						<h3 class="text-sm font-medium text-slate-300 uppercase tracking-wide">
							Detected Differences ({differences.length})
						</h3>

						<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
							<!-- Your Version -->
							<div class="p-4 rounded-xl bg-emerald-500/5 border border-emerald-500/30">
								<div class="flex items-center gap-2 mb-3">
									<div
										class="flex items-center justify-center w-6 h-6 rounded-full bg-emerald-500/20 text-emerald-400"
									>
										<IconUser size={14} />
									</div>
									<h4 class="text-sm font-medium text-emerald-400">Your Version</h4>
									<span class="text-xs text-emerald-400/60 ml-auto">Local</span>
								</div>
								<div class="space-y-3">
									{#each differences as diff}
										<div class="text-sm">
											<span class="text-slate-500 text-xs uppercase tracking-wide"
												>{diff.label}</span
											>
											<p class="text-slate-300 mt-0.5 break-words">{diff.localValue}</p>
										</div>
									{/each}
								</div>
							</div>

							<!-- Their Version -->
							<div class="p-4 rounded-xl bg-blue-500/5 border border-blue-500/30">
								<div class="flex items-center gap-2 mb-3">
									{#if serverEditedBy.avatar}
										<img
											src={serverEditedBy.avatar}
											alt=""
											class="w-6 h-6 rounded-full object-cover"
										/>
									{:else}
										<div
											class="flex items-center justify-center w-6 h-6 rounded-full bg-blue-500/20 text-blue-400"
										>
											<IconUser size={14} />
										</div>
									{/if}
									<h4 class="text-sm font-medium text-blue-400">{serverEditedBy.name}'s Version</h4>
									<span class="text-xs text-blue-400/60 ml-auto">Server</span>
								</div>
								<div class="space-y-3">
									{#each differences as diff}
										<div class="text-sm">
											<span class="text-slate-500 text-xs uppercase tracking-wide"
												>{diff.label}</span
											>
											<p class="text-slate-300 mt-0.5 break-words">{diff.serverValue}</p>
										</div>
									{/each}
								</div>
							</div>
						</div>
					</div>
				{:else}
					<div class="p-4 rounded-xl bg-slate-800/50 border border-slate-700/50 text-center">
						<p class="text-slate-400">
							No visible differences in key fields, but the content may have other changes.
						</p>
					</div>
				{/if}

				<!-- Resolution Options -->
				{#if !showConfirmation}
					<div class="space-y-4">
						<h3 class="text-sm font-medium text-slate-300 uppercase tracking-wide">
							Choose Resolution
						</h3>

						<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
							<!-- Keep Mine -->
							<button
								type="button"
								class="p-4 rounded-xl border-2 transition-all duration-200 text-left {resolutionButtonClasses.mine}"
								onclick={() => handleSelectResolution('mine')}
								disabled={isResolving}
								aria-pressed={selectedResolution === 'mine'}
							>
								<div class="flex items-center gap-2 mb-2">
									<div
										class="w-8 h-8 rounded-lg bg-emerald-500/20 flex items-center justify-center text-emerald-400"
									>
										<IconUser size={18} />
									</div>
									<span class="font-medium text-white">Keep Mine</span>
								</div>
								<p class="text-sm text-slate-400">
									Use your local version and discard server changes
								</p>
								{#if selectedResolution === 'mine'}
									<div class="mt-3 flex items-center gap-1.5 text-xs text-emerald-400">
										<IconCheck size={14} />
										<span>Selected</span>
									</div>
								{/if}
							</button>

							<!-- Keep Theirs -->
							<button
								type="button"
								class="p-4 rounded-xl border-2 transition-all duration-200 text-left {resolutionButtonClasses.theirs}"
								onclick={() => handleSelectResolution('theirs')}
								disabled={isResolving}
								aria-pressed={selectedResolution === 'theirs'}
							>
								<div class="flex items-center gap-2 mb-2">
									<div
										class="w-8 h-8 rounded-lg bg-blue-500/20 flex items-center justify-center text-blue-400"
									>
										<IconUserCircle size={18} />
									</div>
									<span class="font-medium text-white">Keep Theirs</span>
								</div>
								<p class="text-sm text-slate-400">Accept server version and discard your changes</p>
								{#if selectedResolution === 'theirs'}
									<div class="mt-3 flex items-center gap-1.5 text-xs text-blue-400">
										<IconCheck size={14} />
										<span>Selected</span>
									</div>
								{/if}
							</button>

							<!-- View Diff / Merge -->
							<button
								type="button"
								class="p-4 rounded-xl border-2 transition-all duration-200 text-left {resolutionButtonClasses.merge}"
								onclick={() => handleSelectResolution('merge')}
								disabled={isResolving}
								aria-pressed={selectedResolution === 'merge'}
							>
								<div class="flex items-center gap-2 mb-2">
									<div
										class="w-8 h-8 rounded-lg bg-amber-500/20 flex items-center justify-center text-amber-400"
									>
										<IconFileDiff size={18} />
									</div>
									<span class="font-medium text-white">View Diff</span>
								</div>
								<p class="text-sm text-slate-400">Open diff view to manually merge both versions</p>
								{#if selectedResolution === 'merge'}
									<div class="mt-3 flex items-center gap-1.5 text-xs text-amber-400">
										<IconCheck size={14} />
										<span>Selected</span>
									</div>
								{/if}
							</button>
						</div>
					</div>
				{:else}
					<!-- Confirmation Step -->
					<div
						class="p-6 rounded-xl bg-red-500/10 border border-red-500/30"
						transition:scale={{ duration: 200, easing: quintOut }}
					>
						<div class="flex items-start gap-4">
							<div
								class="flex-shrink-0 w-12 h-12 rounded-full bg-red-500/20 flex items-center justify-center text-red-400"
							>
								<IconAlertTriangle size={24} />
							</div>
							<div class="flex-1">
								<h3 class="text-lg font-semibold text-red-400">Confirm Resolution</h3>
								<p class="mt-2 text-slate-300">
									{#if selectedResolution === 'mine'}
										You are about to <strong class="text-red-300">permanently discard</strong> all
										changes made by {serverEditedBy.name}. This action cannot be undone.
									{:else if selectedResolution === 'theirs'}
										You are about to <strong class="text-red-300">permanently discard</strong> all your
										local changes. This action cannot be undone.
									{:else}
										You will be taken to a diff view where you can manually merge both versions.
									{/if}
								</p>
							</div>
						</div>
					</div>
				{/if}

				<!-- Error Message -->
				{#if error}
					<div
						class="p-4 rounded-xl bg-red-500/10 border border-red-500/30"
						transition:fly={{ y: -10, duration: 200 }}
						role="alert"
					>
						<div class="flex items-start gap-3">
							<IconAlertTriangle size={20} class="flex-shrink-0 text-red-400 mt-0.5" />
							<div class="flex-1">
								<p class="text-red-300">{error}</p>
								<button
									type="button"
									class="mt-2 inline-flex items-center gap-1.5 text-sm text-red-400 hover:text-red-300 transition-colors"
									onclick={() => handleApplyResolution()}
									disabled={isResolving}
								>
									<IconRefresh size={14} />
									<span>Retry</span>
								</button>
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<footer
				class="flex items-center justify-between gap-4 px-6 py-4 border-t border-slate-700/50 bg-slate-800/30"
			>
				<p class="text-xs text-slate-500">
					Content ID: {localData.id.slice(0, 8)}...
				</p>

				<div class="flex items-center gap-3">
					{#if showConfirmation}
						<button
							type="button"
							class="px-4 py-2.5 rounded-lg text-sm font-medium text-slate-300 hover:text-white bg-slate-700/50 hover:bg-slate-700 border border-slate-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							onclick={handleCancel}
							disabled={isResolving}
						>
							Go Back
						</button>
						<button
							type="button"
							class="px-5 py-2.5 rounded-lg text-sm font-medium text-white bg-red-600 hover:bg-red-500 transition-colors disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
							onclick={handleApplyResolution}
							disabled={isResolving}
						>
							{#if isResolving}
								<IconLoader2 size={16} class="animate-spin" />
								<span>Applying...</span>
							{:else}
								<IconCheck size={16} />
								<span>Confirm & Apply</span>
							{/if}
						</button>
					{:else}
						<button
							type="button"
							class="px-4 py-2.5 rounded-lg text-sm font-medium text-slate-300 hover:text-white bg-slate-700/50 hover:bg-slate-700 border border-slate-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
							onclick={handleClose}
							disabled={isResolving}
						>
							Cancel
						</button>
						<button
							type="button"
							class="px-5 py-2.5 rounded-lg text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-500 transition-colors disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
							onclick={handleConfirm}
							disabled={!selectedResolution || isResolving}
						>
							{#if isResolving}
								<IconLoader2 size={16} class="animate-spin" />
								<span>Processing...</span>
							{:else}
								<span>Continue</span>
							{/if}
						</button>
					{/if}
				</div>
			</footer>
		</div>
	</div>
{/if}

<style>
	/* Custom scrollbar for modal content */
	:global(.overflow-y-auto::-webkit-scrollbar) {
		width: 8px;
	}

	:global(.overflow-y-auto::-webkit-scrollbar-track) {
		background: rgba(255, 255, 255, 0.05);
		border-radius: 4px;
	}

	:global(.overflow-y-auto::-webkit-scrollbar-thumb) {
		background: rgba(255, 255, 255, 0.15);
		border-radius: 4px;
	}

	:global(.overflow-y-auto::-webkit-scrollbar-thumb:hover) {
		background: rgba(255, 255, 255, 0.25);
	}
</style>

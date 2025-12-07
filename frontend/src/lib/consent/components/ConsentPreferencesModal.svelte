<script lang="ts">
	/**
	 * Consent Preferences Modal Component
	 *
	 * A detailed modal for granular consent category management.
	 * Shows all available categories with toggles and vendor information.
	 *
	 * Features:
	 * - Individual category toggles
	 * - Vendor information display
	 * - Save/Cancel actions
	 * - Keyboard accessible (Escape to close)
	 * - Focus trap when open
	 *
	 * @component
	 */

	import { fade, scale } from 'svelte/transition';
	import { browser } from '$app/environment';
	import { onMount, onDestroy } from 'svelte';
	import type { ConsentCategory } from '../types';
	import {
		consentStore,
		showPreferencesModal,
		closePreferencesModal,
	} from '../store';
	import { getVendorsByCategory } from '../vendors';

	/**
	 * Custom class for styling.
	 */
	let { class: className = '' } = $props();

	// Local state for pending changes (not saved until user clicks Save)
	let pendingConsent = $state({
		analytics: false,
		marketing: false,
		preferences: false,
	});

	// Sync with store when modal opens
	$effect(() => {
		if ($showPreferencesModal) {
			const current = $consentStore;
			pendingConsent = {
				analytics: current.analytics,
				marketing: current.marketing,
				preferences: current.preferences,
			};
		}
	});

	// Category definitions for the UI
	const categories: Array<{
		id: ConsentCategory;
		name: string;
		description: string;
		required: boolean;
	}> = [
		{
			id: 'necessary',
			name: 'Strictly Necessary',
			description:
				'These cookies are essential for the website to function properly. They enable core features like security, network management, and accessibility. You cannot disable these.',
			required: true,
		},
		{
			id: 'analytics',
			name: 'Analytics & Performance',
			description:
				'These cookies help us understand how visitors interact with our website by collecting and reporting information anonymously. This helps us improve our services.',
			required: false,
		},
		{
			id: 'marketing',
			name: 'Marketing & Advertising',
			description:
				'These cookies are used to track visitors across websites. The intention is to display ads that are relevant and engaging for the individual user.',
			required: false,
		},
		{
			id: 'preferences',
			name: 'Preferences & Personalization',
			description:
				'These cookies enable the website to remember choices you make (such as your language or region) and provide enhanced, more personal features.',
			required: false,
		},
	];

	// Get vendors for each category
	function getVendorsForCategory(categoryId: ConsentCategory): string[] {
		return getVendorsByCategory(categoryId).map((v) => v.name);
	}

	// Toggle a category
	function toggleCategory(categoryId: ConsentCategory): void {
		if (categoryId === 'necessary') return; // Can't toggle necessary

		pendingConsent = {
			...pendingConsent,
			[categoryId]: !pendingConsent[categoryId as keyof typeof pendingConsent],
		};
	}

	// Save preferences
	function handleSave(): void {
		consentStore.updateCategories({
			analytics: pendingConsent.analytics,
			marketing: pendingConsent.marketing,
			preferences: pendingConsent.preferences,
		});
		closePreferencesModal();
	}

	// Cancel and close
	function handleCancel(): void {
		closePreferencesModal();
	}

	// Accept all from modal
	function handleAcceptAll(): void {
		consentStore.acceptAll();
		closePreferencesModal();
	}

	// Reject all from modal
	function handleRejectAll(): void {
		consentStore.rejectAll();
		closePreferencesModal();
	}

	// Handle escape key
	function handleKeydown(event: KeyboardEvent): void {
		if (event.key === 'Escape' && $showPreferencesModal) {
			handleCancel();
		}
	}

	// Handle backdrop click
	function handleBackdropClick(event: MouseEvent): void {
		if (event.target === event.currentTarget) {
			handleCancel();
		}
	}

	// Focus trap
	let modalElement = $state<HTMLElement>();
	let previouslyFocused: HTMLElement | null = null;

	// Focus management when modal opens
	$effect(() => {
		if (browser && $showPreferencesModal) {
			previouslyFocused = document.activeElement as HTMLElement;
			// Focus the modal after it renders
			setTimeout(() => {
				modalElement?.focus();
			}, 0);
		}
	});

	// Restore focus on close
	$effect(() => {
		if (browser && !$showPreferencesModal && previouslyFocused) {
			previouslyFocused.focus();
			previouslyFocused = null;
		}
	});

	// Prevent body scroll when modal is open
	$effect(() => {
		if (browser) {
			if ($showPreferencesModal) {
				document.body.style.overflow = 'hidden';
			} else {
				document.body.style.overflow = '';
			}
		}
	});

	onDestroy(() => {
		if (browser) {
			document.body.style.overflow = '';
		}
	});

	// Check for reduced motion
	const prefersReducedMotion = $derived(
		browser
			? window.matchMedia('(prefers-reduced-motion: reduce)').matches
			: false
	);
</script>

<svelte:window onkeydown={handleKeydown} />

{#if $showPreferencesModal}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="modal-backdrop {className}"
		onclick={handleBackdropClick}
		transition:fade={{ duration: prefersReducedMotion ? 0 : 200 }}
	>
		<div
			class="modal-container"
			role="dialog"
			aria-modal="true"
			aria-labelledby="preferences-modal-title"
			tabindex="-1"
			bind:this={modalElement}
			transition:scale={{ start: 0.95, duration: prefersReducedMotion ? 0 : 200 }}
		>
			<div class="modal-header">
				<h2 id="preferences-modal-title" class="modal-title">
					Cookie Preferences
				</h2>
				<button
					type="button"
					class="modal-close"
					onclick={handleCancel}
					aria-label="Close preferences"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>

			<div class="modal-body">
				<p class="modal-intro">
					We use cookies and similar technologies to enhance your browsing
					experience. Choose which categories of cookies you want to allow.
					Your choices will be saved and you can change them at any time.
				</p>

				<div class="categories-list">
					{#each categories as category}
						{@const vendors = getVendorsForCategory(category.id)}
						{@const isEnabled =
							category.id === 'necessary'
								? true
								: pendingConsent[category.id as keyof typeof pendingConsent]}
						<div class="category-item">
							<div class="category-header">
								<div class="category-info">
									<h3 class="category-name">{category.name}</h3>
									{#if category.required}
										<span class="category-badge required">Always Active</span>
									{:else}
										<span
											class="category-badge"
											class:enabled={isEnabled}
											class:disabled={!isEnabled}
										>
											{isEnabled ? 'Enabled' : 'Disabled'}
										</span>
									{/if}
								</div>
								{#if !category.required}
									<button
										type="button"
										class="toggle-switch"
										class:active={isEnabled}
										role="switch"
										aria-checked={isEnabled}
										aria-label="Toggle {category.name}"
										onclick={() => toggleCategory(category.id)}
									>
										<span class="toggle-slider"></span>
									</button>
								{/if}
							</div>
							<p class="category-description">{category.description}</p>
							{#if vendors.length > 0}
								<div class="category-vendors">
									<span class="vendors-label">Vendors:</span>
									<span class="vendors-list">{vendors.join(', ')}</span>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			</div>

			<div class="modal-footer">
				<div class="footer-actions-left">
					<button
						type="button"
						class="consent-btn consent-btn-outline"
						onclick={handleRejectAll}
					>
						Reject All
					</button>
					<button
						type="button"
						class="consent-btn consent-btn-outline"
						onclick={handleAcceptAll}
					>
						Accept All
					</button>
				</div>
				<div class="footer-actions-right">
					<button
						type="button"
						class="consent-btn consent-btn-secondary"
						onclick={handleCancel}
					>
						Cancel
					</button>
					<button
						type="button"
						class="consent-btn consent-btn-primary"
						onclick={handleSave}
					>
						Save Preferences
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		z-index: 10000;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1rem;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
	}

	.modal-container {
		width: 100%;
		max-width: 640px;
		max-height: calc(100vh - 2rem);
		display: flex;
		flex-direction: column;
		background: linear-gradient(135deg, #1a1f2e 0%, #0d1117 100%);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 1rem;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		overflow: hidden;
	}

	.modal-container:focus {
		outline: none;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.modal-title {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 600;
		color: #ffffff;
	}

	.modal-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 2rem;
		height: 2rem;
		padding: 0;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		color: rgba(255, 255, 255, 0.6);
		cursor: pointer;
		transition: all 0.2s;
	}

	.modal-close:hover {
		background: rgba(255, 255, 255, 0.1);
		color: #ffffff;
	}

	.modal-close:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.modal-body {
		flex: 1;
		overflow-y: auto;
		padding: 1.5rem;
	}

	.modal-intro {
		margin: 0 0 1.5rem;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.8);
		line-height: 1.6;
	}

	.categories-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.category-item {
		padding: 1rem;
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 0.75rem;
	}

	.category-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.75rem;
	}

	.category-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
	}

	.category-name {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: #ffffff;
	}

	.category-badge {
		padding: 0.25rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
		border-radius: 0.25rem;
	}

	.category-badge.required {
		background: rgba(34, 197, 94, 0.2);
		color: #22c55e;
	}

	.category-badge.enabled {
		background: rgba(59, 130, 246, 0.2);
		color: #3b82f6;
	}

	.category-badge.disabled {
		background: rgba(107, 114, 128, 0.2);
		color: #9ca3af;
	}

	.category-description {
		margin: 0;
		font-size: 0.875rem;
		color: rgba(255, 255, 255, 0.7);
		line-height: 1.5;
	}

	.category-vendors {
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid rgba(255, 255, 255, 0.08);
		font-size: 0.8125rem;
	}

	.vendors-label {
		color: rgba(255, 255, 255, 0.5);
	}

	.vendors-list {
		color: rgba(255, 255, 255, 0.7);
	}

	/* Toggle Switch */
	.toggle-switch {
		position: relative;
		width: 44px;
		height: 24px;
		flex-shrink: 0;
		padding: 0;
		background: rgba(107, 114, 128, 0.5);
		border: none;
		border-radius: 12px;
		cursor: pointer;
		transition: background 0.2s;
	}

	.toggle-switch.active {
		background: #3b82f6;
	}

	.toggle-switch:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.toggle-slider {
		position: absolute;
		top: 2px;
		left: 2px;
		width: 20px;
		height: 20px;
		background: #ffffff;
		border-radius: 50%;
		transition: transform 0.2s;
	}

	.toggle-switch.active .toggle-slider {
		transform: translateX(20px);
	}

	/* Footer */
	.modal-footer {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
	}

	.footer-actions-left,
	.footer-actions-right {
		display: flex;
		gap: 0.75rem;
	}

	/* Buttons - matching banner styles */
	.consent-btn {
		padding: 0.625rem 1.25rem;
		font-size: 0.875rem;
		font-weight: 500;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
		border: none;
	}

	.consent-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.consent-btn-primary {
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		color: #ffffff;
	}

	.consent-btn-primary:hover {
		background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
		transform: translateY(-1px);
	}

	.consent-btn-secondary {
		background: rgba(255, 255, 255, 0.1);
		color: rgba(255, 255, 255, 0.9);
	}

	.consent-btn-secondary:hover {
		background: rgba(255, 255, 255, 0.15);
	}

	.consent-btn-outline {
		background: transparent;
		color: rgba(255, 255, 255, 0.9);
		border: 1px solid rgba(255, 255, 255, 0.3);
	}

	.consent-btn-outline:hover {
		background: rgba(255, 255, 255, 0.1);
		border-color: rgba(255, 255, 255, 0.5);
	}

	/* Responsive */
	@media (max-width: 640px) {
		.modal-container {
			max-height: 100vh;
			border-radius: 0;
		}

		.modal-footer {
			flex-direction: column;
		}

		.footer-actions-left,
		.footer-actions-right {
			width: 100%;
			justify-content: center;
		}

		.consent-btn {
			flex: 1;
		}
	}

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.toggle-slider {
			transition: none;
		}

		.consent-btn {
			transition: none;
		}

		.consent-btn-primary:hover {
			transform: none;
		}
	}
</style>

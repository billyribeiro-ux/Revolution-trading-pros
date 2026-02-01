<script lang="ts">
	/**
	 * Content Placeholder Component
	 *
	 * Shows a placeholder for blocked content (YouTube, Google Maps, etc.)
	 * until the user grants the required consent.
	 *
	 * Updated to Svelte 5 syntax (November 2025)
	 *
	 * @component
	 */
	import { consentStore, openPreferencesModal } from '../store.svelte';
	import { t } from '../i18n';
	import type { ConsentCategory } from '../types';
	import type { Snippet } from 'svelte';

	// Svelte 5: Props with $props()
	let {
		type = 'custom',
		requiredCategory = 'marketing' as ConsentCategory,
		width = '100%',
		height = '315px',
		aspectRatio = '16/9',
		thumbnailUrl = '',
		title = '',
		customIcon = '',
		children
	}: {
		type?: 'youtube' | 'vimeo' | 'google-maps' | 'twitter' | 'facebook' | 'instagram' | 'custom';
		requiredCategory?: ConsentCategory;
		width?: string;
		height?: string;
		aspectRatio?: string;
		thumbnailUrl?: string;
		title?: string;
		customIcon?: string;
		children?: Snippet;
	} = $props();

	// Type configurations
	const typeConfigs: Record<string, { icon: string; name: string; category: ConsentCategory }> = {
		youtube: { icon: '▶️', name: 'YouTube Video', category: 'marketing' },
		vimeo: { icon: '▶️', name: 'Vimeo Video', category: 'marketing' },
		'google-maps': { icon: '📍', name: 'Google Maps', category: 'analytics' },
		twitter: { icon: '🐦', name: 'Twitter/X Post', category: 'marketing' },
		facebook: { icon: '📘', name: 'Facebook Post', category: 'marketing' },
		instagram: { icon: '📷', name: 'Instagram Post', category: 'marketing' },
		custom: { icon: '🔒', name: 'External Content', category: 'marketing' }
	};

	// Svelte 5: Derived state
	let config = $derived(typeConfigs[type] || typeConfigs['custom']!);
	let effectiveCategory = $derived(requiredCategory || config?.category || 'marketing');
	let hasConsent = $derived($consentStore[effectiveCategory]);
	let displayIcon = $derived(customIcon || config?.icon || '🔒');
	let displayTitle = $derived(title || config?.name || 'Content');

	function handleEnableCookies() {
		openPreferencesModal();
	}
</script>

{#if hasConsent}
	<!-- Content is allowed, render the children -->
	{#if children}
		{@render children()}
	{/if}
{:else}
	<!-- Show placeholder -->
	<div
		class="content-placeholder"
		style:width
		style:height
		style:aspect-ratio={aspectRatio}
		role="region"
		aria-label={$t.contentBlocked}
	>
		{#if thumbnailUrl}
			<div class="thumbnail" style:background-image="url({thumbnailUrl})"></div>
		{/if}

		<div class="overlay">
			<div class="icon">{displayIcon}</div>
			<h3 class="title">{$t.contentBlocked}</h3>
			<p class="description">
				{displayTitle}
				{$t.contentBlockedDescription}
			</p>
			<button class="enable-btn" onclick={handleEnableCookies}>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path d="M12 2a10 10 0 1 0 10 10 4 4 0 0 1-5-5 4 4 0 0 1-5-5" />
					<path d="M8.5 8.5v.01" />
					<path d="M16 15.5v.01" />
					<path d="M12 12v.01" />
					<path d="M11 17v.01" />
					<path d="M7 14v.01" />
				</svg>
				{$t.enableCookies}
			</button>
			<p class="category-info">
				Requires: <strong style="text-transform: capitalize;">{effectiveCategory}</strong> cookies
			</p>
		</div>
	</div>
{/if}

<style>
	.content-placeholder {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
		border-radius: 12px;
		overflow: hidden;
		min-height: 200px;
	}

	.thumbnail {
		position: absolute;
		inset: 0;
		background-size: cover;
		background-position: center;
		filter: blur(8px) brightness(0.3);
	}

	.overlay {
		position: relative;
		z-index: 1;
		text-align: center;
		padding: 1.5rem;
		max-width: 400px;
	}

	.icon {
		font-size: 2.5rem;
		margin-bottom: 1rem;
		filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
	}

	.title {
		font-size: 1.1rem;
		font-weight: 600;
		color: white;
		margin: 0 0 0.5rem 0;
	}

	.description {
		font-size: 0.8rem;
		color: rgba(255, 255, 255, 0.8);
		margin: 0 0 1.5rem 0;
		line-height: 1.5;
	}

	.enable-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		background: linear-gradient(135deg, #0ea5e9 0%, #06b6d4 100%);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		box-shadow: 0 4px 12px rgba(14, 165, 233, 0.3);
	}

	.enable-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 6px 16px rgba(14, 165, 233, 0.4);
	}

	.enable-btn:active {
		transform: translateY(0);
	}

	.category-info {
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.5);
		margin-top: 1rem;
	}

	/* Responsive - Mobile First */
	@media (min-width: 640px) {
		.overlay {
			padding: 2rem;
		}

		.icon {
			font-size: 3rem;
		}

		.title {
			font-size: 1.25rem;
		}

		.description {
			font-size: 0.875rem;
		}
	}
</style>

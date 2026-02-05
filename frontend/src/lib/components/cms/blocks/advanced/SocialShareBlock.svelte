<!--
/**
 * Social Share Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Social media share buttons with platform-specific styling
 * Features: Twitter/X, Facebook, LinkedIn, Email, Copy Link
 *
 * @version 2.0.0
 * @author Revolution Trading Pros
 */
-->

<script lang="ts">
	import { IconLink, IconCheck } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import { browser } from '$app/environment';
	import type { Block, BlockContent } from '../types';

	// =========================================================================
	// Types
	// =========================================================================

	interface Platform {
		id: string;
		name: string;
		icon: string;
		color: string;
		hoverColor: string;
		getShareUrl: () => string;
	}

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	// =========================================================================
	// Props & State
	// =========================================================================

	let props: Props = $props();
	const stateManager = getBlockStateManager();

	// =========================================================================
	// Derived Values
	// =========================================================================

	let linkCopied = $derived(stateManager.getLinkCopied(props.blockId));
	let enabledPlatforms = $derived(
		props.block.content.sharePlatforms || ['twitter', 'facebook', 'linkedin', 'email']
	);

	// =========================================================================
	// Helper Functions
	// =========================================================================

	function getShareUrl(): string {
		return browser ? window.location.href : '';
	}

	function getPageTitle(): string {
		return browser ? document.title : '';
	}

	// =========================================================================
	// Platform Configuration
	// =========================================================================

	const platforms: Platform[] = [
		{
			id: 'twitter',
			name: 'Twitter (X)',
			icon: '𝕏',
			color: '#000000',
			hoverColor: '#1a1a1a',
			getShareUrl: () =>
				`https://twitter.com/intent/tweet?url=${encodeURIComponent(getShareUrl())}&text=${encodeURIComponent(getPageTitle())}`
		},
		{
			id: 'facebook',
			name: 'Facebook',
			icon: 'f',
			color: '#1877f2',
			hoverColor: '#0c5dc7',
			getShareUrl: () =>
				`https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(getShareUrl())}`
		},
		{
			id: 'linkedin',
			name: 'LinkedIn',
			icon: 'in',
			color: '#0a66c2',
			hoverColor: '#084d94',
			getShareUrl: () =>
				`https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(getShareUrl())}`
		},
		{
			id: 'email',
			name: 'Email',
			icon: '@',
			color: '#6b7280',
			hoverColor: '#4b5563',
			getShareUrl: () =>
				`mailto:?subject=${encodeURIComponent(getPageTitle())}&body=${encodeURIComponent(getShareUrl())}`
		}
	];

	// =========================================================================
	// Handlers
	// =========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function openSharePopup(platform: Platform): void {
		if (props.isEditing) return;

		try {
			const url = platform.getShareUrl();
			if (platform.id === 'email') {
				window.location.href = url;
			} else {
				window.open(url, '_blank', 'width=550,height=420,menubar=no,toolbar=no');
			}
		} catch (err) {
			props.onError?.(err instanceof Error ? err : new Error('Failed to share'));
		}
	}

	async function copyLink(): Promise<void> {
		if (props.isEditing) return;

		try {
			await navigator.clipboard.writeText(getShareUrl());
			stateManager.setLinkCopied(props.blockId, true);

			setTimeout(() => {
				stateManager.setLinkCopied(props.blockId, false);
			}, 2000);
		} catch (err) {
			props.onError?.(err instanceof Error ? err : new Error('Failed to copy link'));
		}
	}

	function togglePlatform(platformId: string): void {
		const current = [...enabledPlatforms];
		const index = current.indexOf(platformId);
		if (index > -1) {
			current.splice(index, 1);
		} else {
			current.push(platformId);
		}
		updateContent({ sharePlatforms: current });
	}
</script>

<div
	class="social-share-block"
	class:disabled={props.isEditing}
	role="region"
	aria-label="Share this content"
>
	<span class="share-label">Share this:</span>

	<div class="share-buttons">
		{#each platforms as platform (platform.id)}
			{#if enabledPlatforms.includes(platform.id)}
				<button
					type="button"
					class="share-btn"
					style="--btn-color: {platform.color}; --btn-hover-color: {platform.hoverColor};"
					onclick={() => openSharePopup(platform)}
					disabled={props.isEditing}
					aria-label="Share on {platform.name}"
				>
					<span class="platform-icon">{platform.icon}</span>
				</button>
			{/if}
		{/each}

		<button
			type="button"
			class="share-btn copy-btn"
			class:copied={linkCopied}
			onclick={copyLink}
			disabled={props.isEditing}
			aria-label={linkCopied ? 'Copied!' : 'Copy link'}
		>
			{#if linkCopied}
				<IconCheck size={20} />
			{:else}
				<IconLink size={20} />
			{/if}
		</button>
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="share-settings">
			<span class="settings-label">Show platforms:</span>
			<div class="platform-toggles">
				{#each platforms as platform (platform.id)}
					<label class="toggle-label">
						<input
							type="checkbox"
							checked={enabledPlatforms.includes(platform.id)}
							onchange={() => togglePlatform(platform.id)}
						/>
						<span>{platform.name}</span>
					</label>
				{/each}
				<label class="toggle-label">
					<input type="checkbox" checked={true} disabled />
					<span>Copy Link</span>
				</label>
			</div>
		</div>
	{/if}
</div>

<style>
	.social-share-block {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: #f8fafc;
		border-radius: 12px;
		flex-wrap: wrap;
	}

	.social-share-block.disabled {
		opacity: 0.7;
	}

	.share-label {
		font-size: 0.9375rem;
		font-weight: 500;
		color: #64748b;
	}

	.share-buttons {
		display: flex;
		gap: 0.5rem;
	}

	.share-btn {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition: all 0.15s ease;
		color: white;
		background: var(--btn-color);
		font-weight: 700;
	}

	.share-btn:hover:not(:disabled) {
		background: var(--btn-hover-color);
		transform: translateY(-2px);
	}

	.share-btn:active:not(:disabled) {
		transform: translateY(0);
	}

	.share-btn:disabled {
		cursor: not-allowed;
		opacity: 0.6;
	}

	.platform-icon {
		font-size: 1rem;
		line-height: 1;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	.copy-btn {
		--btn-color: #6b7280;
		--btn-hover-color: #4b5563;
	}

	.copy-btn.copied {
		--btn-color: #22c55e;
		--btn-hover-color: #16a34a;
	}

	.share-settings {
		width: 100%;
		margin-top: 0.75rem;
		padding-top: 0.75rem;
		border-top: 1px solid #e2e8f0;
	}

	.settings-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		margin-bottom: 0.5rem;
	}

	.platform-toggles {
		display: flex;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.toggle-label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		color: #475569;
		cursor: pointer;
	}

	.toggle-label input {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	.toggle-label input:disabled {
		cursor: not-allowed;
	}

	/* Dark Mode Support */
	:global(.dark) .social-share-block {
		background: #1e293b;
	}

	:global(.dark) .share-label {
		color: #94a3b8;
	}

	:global(.dark) .share-settings {
		border-color: #334155;
	}

	:global(.dark) .settings-label {
		color: #94a3b8;
	}

	:global(.dark) .toggle-label {
		color: #94a3b8;
	}

	:global(.dark) .share-btn {
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
	}

	:global(.dark) .share-btn:hover:not(:disabled) {
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.4);
	}
</style>

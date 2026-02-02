<!--
/**
 * Social Share Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Social media share buttons
 */
-->

<script lang="ts">
	import { IconBrandTwitter, IconBrandFacebook, IconBrandLinkedin, IconLink, IconCheck } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import { browser } from '$app/environment';
	import type { Block, BlockContent } from '../types';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();
	const stateManager = getBlockStateManager();

	let title = $derived(props.block.content.shareTitle || 'Share this article');
	let platforms = $derived(props.block.content.sharePlatforms || ['twitter', 'facebook', 'linkedin', 'copy']);

	let linkCopied = $state(false);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function getShareUrl(): string {
		return browser ? window.location.href : '';
	}

	function getPageTitle(): string {
		return browser ? document.title : '';
	}

	function shareTwitter(): void {
		const url = `https://twitter.com/intent/tweet?url=${encodeURIComponent(getShareUrl())}&text=${encodeURIComponent(getPageTitle())}`;
		window.open(url, '_blank', 'width=550,height=420');
	}

	function shareFacebook(): void {
		const url = `https://www.facebook.com/sharer/sharer.php?u=${encodeURIComponent(getShareUrl())}`;
		window.open(url, '_blank', 'width=550,height=420');
	}

	function shareLinkedin(): void {
		const url = `https://www.linkedin.com/sharing/share-offsite/?url=${encodeURIComponent(getShareUrl())}`;
		window.open(url, '_blank', 'width=550,height=420');
	}

	async function copyLink(): Promise<void> {
		await navigator.clipboard.writeText(getShareUrl());
		linkCopied = true;
		setTimeout(() => { linkCopied = false; }, 2000);
	}

	function togglePlatform(platform: string): void {
		const current = [...platforms];
		const index = current.indexOf(platform);
		if (index > -1) {
			current.splice(index, 1);
		} else {
			current.push(platform);
		}
		updateContent({ sharePlatforms: current });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}
</script>

<div class="social-share-block" role="region" aria-label="Share this content">
	{#if props.isEditing}
		<span
			contenteditable="true"
			class="share-title"
			oninput={(e) => updateContent({ shareTitle: (e.target as HTMLElement).textContent || '' })}
			onpaste={handlePaste}
		>{title}</span>
	{:else}
		<span class="share-title">{title}</span>
	{/if}

	<div class="share-buttons">
		{#if platforms.includes('twitter')}
			<button type="button" class="share-btn twitter" onclick={shareTwitter} aria-label="Share on Twitter">
				<IconBrandTwitter size={20} />
			</button>
		{/if}
		{#if platforms.includes('facebook')}
			<button type="button" class="share-btn facebook" onclick={shareFacebook} aria-label="Share on Facebook">
				<IconBrandFacebook size={20} />
			</button>
		{/if}
		{#if platforms.includes('linkedin')}
			<button type="button" class="share-btn linkedin" onclick={shareLinkedin} aria-label="Share on LinkedIn">
				<IconBrandLinkedin size={20} />
			</button>
		{/if}
		{#if platforms.includes('copy')}
			<button type="button" class="share-btn copy" class:copied={linkCopied} onclick={copyLink} aria-label={linkCopied ? 'Copied!' : 'Copy link'}>
				{#if linkCopied}
					<IconCheck size={20} />
				{:else}
					<IconLink size={20} />
				{/if}
			</button>
		{/if}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="share-settings">
			<span class="settings-label">Show platforms:</span>
			<div class="platform-toggles">
				<label><input type="checkbox" checked={platforms.includes('twitter')} onchange={() => togglePlatform('twitter')} /> Twitter</label>
				<label><input type="checkbox" checked={platforms.includes('facebook')} onchange={() => togglePlatform('facebook')} /> Facebook</label>
				<label><input type="checkbox" checked={platforms.includes('linkedin')} onchange={() => togglePlatform('linkedin')} /> LinkedIn</label>
				<label><input type="checkbox" checked={platforms.includes('copy')} onchange={() => togglePlatform('copy')} /> Copy Link</label>
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

	.share-title {
		font-size: 0.9375rem;
		font-weight: 500;
		color: #64748b;
		outline: none;
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
		transition: all 0.15s;
		color: white;
	}

	.share-btn:hover { transform: translateY(-2px); }
	.share-btn:active { transform: translateY(0); }

	.share-btn.twitter { background: #1da1f2; }
	.share-btn.twitter:hover { background: #0c8de4; }

	.share-btn.facebook { background: #1877f2; }
	.share-btn.facebook:hover { background: #0c5dc7; }

	.share-btn.linkedin { background: #0a66c2; }
	.share-btn.linkedin:hover { background: #084d94; }

	.share-btn.copy { background: #64748b; }
	.share-btn.copy:hover { background: #475569; }
	.share-btn.copy.copied { background: #22c55e; }

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

	.platform-toggles label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		font-size: 0.875rem;
		color: #475569;
		cursor: pointer;
	}

	:global(.dark) .social-share-block { background: #1e293b; }
	:global(.dark) .share-title { color: #94a3b8; }
	:global(.dark) .share-settings { border-color: #334155; }
	:global(.dark) .platform-toggles label { color: #94a3b8; }
</style>

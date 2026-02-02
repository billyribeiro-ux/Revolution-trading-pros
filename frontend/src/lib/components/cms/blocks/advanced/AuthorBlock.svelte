<!--
/**
 * Author Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Author bio box with photo and social links
 */
-->

<script lang="ts">
	import { IconUser, IconBrandTwitter, IconBrandLinkedin, IconWorld } from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	const props: Props = $props();

	let name = $derived(props.block.content.authorName || 'Author Name');
	let bio = $derived(props.block.content.authorBio || 'Author bio goes here. Share a brief description about the author.');
	let photo = $derived(props.block.content.authorPhoto || '');
	let socials = $derived(props.block.content.authorSocials || []);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		document.execCommand('insertText', false, e.clipboardData?.getData('text/plain') || '');
	}

	function getSocialIcon(platform: string) {
		switch (platform.toLowerCase()) {
			case 'twitter': return IconBrandTwitter;
			case 'linkedin': return IconBrandLinkedin;
			default: return IconWorld;
		}
	}

	function addSocial(): void {
		updateContent({ authorSocials: [...socials, { platform: 'twitter', url: '' }] });
	}

	function updateSocial(index: number, field: 'platform' | 'url', value: string): void {
		const newSocials = socials.map((s, i) => i === index ? { ...s, [field]: value } : s);
		updateContent({ authorSocials: newSocials });
	}

	function removeSocial(index: number): void {
		updateContent({ authorSocials: socials.filter((_, i) => i !== index) });
	}
</script>

<div class="author-block" role="article" aria-label="About the author">
	<div class="author-photo">
		{#if photo}
			<img src={sanitizeURL(photo)} alt={name} />
		{:else}
			<div class="photo-placeholder"><IconUser size={32} /></div>
		{/if}
	</div>

	<div class="author-content">
		<div class="author-header">
			<span class="author-label">Written by</span>
			{#if props.isEditing}
				<h3
					contenteditable="true"
					class="author-name"
					oninput={(e) => updateContent({ authorName: (e.target as HTMLElement).textContent || '' })}
					onpaste={handlePaste}
				>{name}</h3>
			{:else}
				<h3 class="author-name">{name}</h3>
			{/if}
		</div>

		{#if props.isEditing}
			<p
				contenteditable="true"
				class="author-bio"
				oninput={(e) => updateContent({ authorBio: (e.target as HTMLElement).textContent || '' })}
				onpaste={handlePaste}
			>{bio}</p>
		{:else}
			<p class="author-bio">{bio}</p>
		{/if}

		{#if socials.length > 0 || props.isEditing}
			<div class="author-socials">
				{#each socials as social, index (index)}
					{#if props.isEditing}
						<div class="social-edit">
							<select value={social.platform} onchange={(e) => updateSocial(index, 'platform', (e.target as HTMLSelectElement).value)}>
								<option value="twitter">Twitter</option>
								<option value="linkedin">LinkedIn</option>
								<option value="website">Website</option>
							</select>
							<input type="url" placeholder="URL" value={social.url} oninput={(e) => updateSocial(index, 'url', (e.target as HTMLInputElement).value)} />
							<button type="button" class="remove-social" onclick={() => removeSocial(index)}>×</button>
						</div>
					{:else if social.url}
						{@const Icon = getSocialIcon(social.platform)}
						<a href={sanitizeURL(social.url)} class="social-link" target="_blank" rel="noopener noreferrer" aria-label={social.platform}>
							<Icon size={18} />
						</a>
					{/if}
				{/each}
				{#if props.isEditing}
					<button type="button" class="add-social" onclick={addSocial}>+ Add Social</button>
				{/if}
			</div>
		{/if}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="author-settings">
			<label>
				<span>Photo URL:</span>
				<input type="url" value={photo} oninput={(e) => updateContent({ authorPhoto: (e.target as HTMLInputElement).value })} placeholder="https://..." />
			</label>
		</div>
	{/if}
</div>

<style>
	.author-block {
		display: flex;
		gap: 1.5rem;
		padding: 1.5rem;
		background: #f8fafc;
		border-radius: 16px;
		flex-wrap: wrap;
	}

	.author-photo {
		width: 80px;
		height: 80px;
		border-radius: 50%;
		overflow: hidden;
		flex-shrink: 0;
		background: #e2e8f0;
	}

	.author-photo img { width: 100%; height: 100%; object-fit: cover; }
	.photo-placeholder {
		width: 100%; height: 100%;
		display: flex; align-items: center; justify-content: center;
		color: #94a3b8;
	}

	.author-content { flex: 1; min-width: 200px; }

	.author-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.25rem;
	}

	.author-name {
		margin: 0 0 0.75rem;
		font-size: 1.25rem;
		font-weight: 700;
		color: #0f172a;
		outline: none;
	}

	.author-bio {
		margin: 0 0 1rem;
		font-size: 0.9375rem;
		line-height: 1.6;
		color: #475569;
		outline: none;
	}

	.author-socials {
		display: flex;
		gap: 0.75rem;
		flex-wrap: wrap;
		align-items: center;
	}

	.social-link {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e2e8f0;
		border-radius: 8px;
		color: #475569;
		transition: all 0.15s;
	}

	.social-link:hover { background: #3b82f6; color: white; }

	.social-edit {
		display: flex;
		gap: 0.5rem;
		align-items: center;
	}

	.social-edit select,
	.social-edit input {
		padding: 0.375rem 0.5rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.8125rem;
	}

	.social-edit input { flex: 1; min-width: 120px; }

	.remove-social {
		width: 24px;
		height: 24px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #fee2e2;
		border: none;
		border-radius: 4px;
		color: #dc2626;
		cursor: pointer;
		font-size: 1rem;
	}

	.add-social {
		padding: 0.375rem 0.75rem;
		background: transparent;
		border: 1px dashed #d1d5db;
		border-radius: 6px;
		color: #64748b;
		font-size: 0.8125rem;
		cursor: pointer;
	}

	.add-social:hover { border-color: #3b82f6; color: #3b82f6; }

	.author-settings {
		width: 100%;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e2e8f0;
	}

	.author-settings label {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.author-settings span {
		font-size: 0.75rem;
		font-weight: 600;
		color: #64748b;
	}

	.author-settings input {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		font-size: 0.875rem;
	}

	:global(.dark) .author-block { background: #1e293b; }
	:global(.dark) .author-photo { background: #334155; }
	:global(.dark) .photo-placeholder { color: #64748b; }
	:global(.dark) .author-name { color: #f8fafc; }
	:global(.dark) .author-bio { color: #94a3b8; }
	:global(.dark) .social-link { background: #334155; color: #94a3b8; }
	:global(.dark) .author-settings { border-color: #334155; }
	:global(.dark) .social-edit select, :global(.dark) .social-edit input, :global(.dark) .author-settings input {
		background: #0f172a; border-color: #475569; color: #e2e8f0;
	}
</style>

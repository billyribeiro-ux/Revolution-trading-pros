<!--
/**
 * Table of Contents Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Auto-generates TOC from document headings with smooth scroll
 */
-->

<script lang="ts">
	import { IconList, IconChevronDown } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import { onMount } from 'svelte';
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

	interface TocItem {
		id: string;
		text: string;
		level: number;
	}

	let tocItems = $state<TocItem[]>([]);
	let tocState = $derived(stateManager.getTocState(props.blockId));
	let isCollapsed = $derived(!tocState.open);
	let activeHeading = $derived(tocState.activeHeading);

	let title = $derived(props.block.content.tocTitle || 'Table of Contents');
	let maxLevel = $derived(props.block.settings.tocMaxLevel || 3);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function toggleCollapse(): void {
		stateManager.setTocState(props.blockId, { open: !tocState.open });
	}

	function scrollToHeading(id: string): void {
		if (!browser) return;

		const element = document.getElementById(id);
		if (element) {
			element.scrollIntoView({ behavior: 'smooth', block: 'start' });
			stateManager.setTocState(props.blockId, { activeHeading: id });
		}
	}

	function generateToc(): void {
		if (!browser) return;

		const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
		const items: TocItem[] = [];

		headings.forEach((heading, index) => {
			const level = parseInt(heading.tagName[1]);
			if (level <= maxLevel) {
				let id = heading.id;
				if (!id) {
					id = `heading-${index}`;
					heading.id = id;
				}

				items.push({
					id,
					text: heading.textContent || '',
					level
				});
			}
		});

		tocItems = items;
	}

	function handleScroll(): void {
		if (!browser) return;

		const headings = tocItems.map((item) => document.getElementById(item.id)).filter(Boolean);
		const scrollPos = window.scrollY + 100;

		let current = '';
		for (const heading of headings) {
			if (heading && heading.offsetTop <= scrollPos) {
				current = heading.id;
			}
		}

		if (current && current !== activeHeading) {
			stateManager.setTocState(props.blockId, { activeHeading: current });
		}
	}

	function handleTitleInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ tocTitle: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	onMount(() => {
		generateToc();
		window.addEventListener('scroll', handleScroll, { passive: true });

		const observer = new MutationObserver(() => {
			generateToc();
		});

		observer.observe(document.body, {
			childList: true,
			subtree: true
		});

		return () => {
			window.removeEventListener('scroll', handleScroll);
			observer.disconnect();
		};
	});
</script>

<nav class="toc-block" aria-label="Table of contents">
	<button
		type="button"
		class="toc-header"
		aria-expanded={!isCollapsed}
		aria-controls="toc-list-{props.blockId}"
		onclick={toggleCollapse}
	>
		<IconList size={20} aria-hidden="true" />

		{#if props.isEditing}
			<span
				contenteditable="true"
				class="toc-title editable-content"
				role="textbox"
				aria-label="TOC title"
				onclick={(e) => e.stopPropagation()}
				oninput={handleTitleInput}
				onpaste={handlePaste}
			>
				{title}
			</span>
		{:else}
			<span class="toc-title">{title}</span>
		{/if}

		<span class="toc-toggle" class:rotated={!isCollapsed} aria-hidden="true">
			<IconChevronDown size={18} />
		</span>
	</button>

	<div
		id="toc-list-{props.blockId}"
		class="toc-content"
		hidden={isCollapsed}
	>
		{#if tocItems.length > 0}
			<ul class="toc-list" role="list">
				{#each tocItems as item (item.id)}
					<li
						class="toc-item toc-level-{item.level}"
						class:active={activeHeading === item.id}
					>
						<button
							type="button"
							class="toc-link"
							onclick={() => scrollToHeading(item.id)}
						>
							{item.text}
						</button>
					</li>
				{/each}
			</ul>
		{:else}
			<p class="toc-empty">No headings found in the document.</p>
		{/if}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="toc-settings">
			<label class="setting-field">
				<span>Max heading level:</span>
				<select
					value={maxLevel}
					onchange={(e) =>
						props.onUpdate({
							settings: {
								...props.block.settings,
								tocMaxLevel: parseInt((e.target as HTMLSelectElement).value)
							}
						})}
					aria-label="Maximum heading level"
				>
					<option value="2">H2</option>
					<option value="3">H3</option>
					<option value="4">H4</option>
					<option value="5">H5</option>
					<option value="6">H6</option>
				</select>
			</label>
		</div>
	{/if}
</nav>

<style>
	.toc-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
		background: #f9fafb;
	}

	.toc-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 1rem 1.25rem;
		background: transparent;
		border: none;
		cursor: pointer;
		font-size: 1rem;
		font-weight: 600;
		color: #1f2937;
		text-align: left;
		transition: background 0.15s;
	}

	.toc-header:hover {
		background: #f3f4f6;
	}

	.toc-header:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
	}

	.toc-title {
		flex: 1;
		outline: none;
	}

	.toc-toggle {
		display: flex;
		color: #6b7280;
		transition: transform 0.2s;
	}

	.toc-toggle.rotated {
		transform: rotate(180deg);
	}

	.toc-content {
		animation: slideDown 0.2s ease-out;
	}

	.toc-content[hidden] {
		display: none;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	.toc-list {
		margin: 0;
		padding: 0 1.25rem 1rem;
		list-style: none;
	}

	.toc-item {
		margin: 0;
		padding: 0;
	}

	.toc-item.toc-level-1 {
		padding-left: 0;
	}

	.toc-item.toc-level-2 {
		padding-left: 0;
	}

	.toc-item.toc-level-3 {
		padding-left: 1rem;
	}

	.toc-item.toc-level-4 {
		padding-left: 2rem;
	}

	.toc-item.toc-level-5 {
		padding-left: 3rem;
	}

	.toc-item.toc-level-6 {
		padding-left: 4rem;
	}

	.toc-link {
		display: block;
		width: 100%;
		padding: 0.5rem 0.75rem;
		background: transparent;
		border: none;
		border-left: 2px solid transparent;
		font-size: 0.875rem;
		color: #6b7280;
		text-align: left;
		cursor: pointer;
		transition: all 0.15s;
		border-radius: 0 6px 6px 0;
	}

	.toc-link:hover {
		color: #3b82f6;
		background: rgba(59, 130, 246, 0.1);
	}

	.toc-link:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	.toc-item.active .toc-link {
		color: #3b82f6;
		border-left-color: #3b82f6;
		background: rgba(59, 130, 246, 0.1);
		font-weight: 500;
	}

	.toc-empty {
		padding: 1rem 1.25rem;
		margin: 0;
		color: #9ca3af;
		font-size: 0.875rem;
		font-style: italic;
	}

	.toc-settings {
		padding: 1rem 1.25rem;
		background: white;
		border-top: 1px solid #e5e7eb;
	}

	.setting-field {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.875rem;
		color: #374151;
	}

	.setting-field span {
		font-weight: 500;
	}

	.setting-field select {
		padding: 0.5rem 0.75rem;
		border: 1px solid #d1d5db;
		border-radius: 6px;
		background: white;
		font-size: 0.875rem;
		cursor: pointer;
	}

	.setting-field select:focus {
		outline: none;
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	/* Dark Mode */
	:global(.dark) .toc-block {
		background: #1e293b;
		border-color: #374151;
	}

	:global(.dark) .toc-header {
		color: #f9fafb;
	}

	:global(.dark) .toc-header:hover {
		background: #334155;
	}

	:global(.dark) .toc-link {
		color: #94a3b8;
	}

	:global(.dark) .toc-link:hover {
		color: #60a5fa;
		background: rgba(96, 165, 250, 0.1);
	}

	:global(.dark) .toc-item.active .toc-link {
		color: #60a5fa;
		border-left-color: #60a5fa;
		background: rgba(96, 165, 250, 0.1);
	}

	:global(.dark) .toc-empty {
		color: #64748b;
	}

	:global(.dark) .toc-settings {
		background: #0f172a;
		border-color: #374151;
	}

	:global(.dark) .setting-field {
		color: #e5e7eb;
	}

	:global(.dark) .setting-field select {
		background: #1e293b;
		border-color: #475569;
		color: #e2e8f0;
	}
</style>

<!--
/**
 * Tabs Block Component
 * ═══════════════════════════════════════════════════════════════════════════
 * Horizontal/vertical tabs with keyboard navigation and fade animations
 */
-->

<script lang="ts">
	import { IconPlus, IconX } from '$lib/icons';
	import { getBlockStateManager, type BlockId } from '$lib/stores/blockState.svelte';
	import type { Block, BlockContent } from '../types';

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();
	const stateManager = getBlockStateManager();

	let tabs = $derived(
		props.block.content.tabs || [
			{ id: 'tab_1', label: 'Tab 1', content: 'Content for tab 1' },
			{ id: 'tab_2', label: 'Tab 2', content: 'Content for tab 2' }
		]
	);

	let defaultTabId = $derived(tabs[0]?.id || '');
	let activeTab = $derived(stateManager.getActiveTab(props.blockId, defaultTabId));
	let orientation = $derived((props.block.settings.tabOrientation as 'horizontal' | 'vertical') || 'horizontal');

	// Track previous active tab for animation
	let previousActiveTab = $state<string | null>(null);
	let isTransitioning = $state(false);

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({ content: { ...props.block.content, ...updates } });
	}

	function setActiveTab(tabId: string): void {
		if (tabId !== activeTab) {
			previousActiveTab = activeTab;
			isTransitioning = true;
			stateManager.setActiveTab(props.blockId, tabId);
			// Reset transition state after animation completes
			setTimeout(() => {
				isTransitioning = false;
				previousActiveTab = null;
			}, 200);
		}
	}

	function updateTab(index: number, field: 'label' | 'content', value: string): void {
		const newTabs = tabs.map((tab, i) => (i === index ? { ...tab, [field]: value } : tab));
		updateContent({ tabs: newTabs });
	}

	function addTab(): void {
		const newTabs = [
			...tabs,
			{
				id: `tab_${Date.now()}`,
				label: `Tab ${tabs.length + 1}`,
				content: ''
			}
		];
		updateContent({ tabs: newTabs });
	}

	function removeTab(index: number): void {
		if (tabs.length > 1) {
			const removedTab = tabs[index];
			const newTabs = tabs.filter((_, i) => i !== index);
			updateContent({ tabs: newTabs });

			if (activeTab === removedTab.id) {
				setActiveTab(newTabs[0].id);
			}
		}
	}

	function handleKeyDown(e: KeyboardEvent, index: number): void {
		const isHorizontal = orientation === 'horizontal';
		const nextKey = isHorizontal ? 'ArrowRight' : 'ArrowDown';
		const prevKey = isHorizontal ? 'ArrowLeft' : 'ArrowUp';

		if (e.key === nextKey) {
			e.preventDefault();
			const nextIndex = (index + 1) % tabs.length;
			const nextTab = tabs[nextIndex];
			setActiveTab(nextTab.id);
			document.getElementById(`tab-btn-${props.blockId}-${nextTab.id}`)?.focus();
		} else if (e.key === prevKey) {
			e.preventDefault();
			const prevIndex = (index - 1 + tabs.length) % tabs.length;
			const prevTab = tabs[prevIndex];
			setActiveTab(prevTab.id);
			document.getElementById(`tab-btn-${props.blockId}-${prevTab.id}`)?.focus();
		} else if (e.key === 'Home') {
			e.preventDefault();
			setActiveTab(tabs[0].id);
			document.getElementById(`tab-btn-${props.blockId}-${tabs[0].id}`)?.focus();
		} else if (e.key === 'End') {
			e.preventDefault();
			const lastTab = tabs[tabs.length - 1];
			setActiveTab(lastTab.id);
			document.getElementById(`tab-btn-${props.blockId}-${lastTab.id}`)?.focus();
		}
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}
</script>

<div class="tabs-block" class:vertical={orientation === 'vertical'} role="region" aria-label="Tabbed content">
	<div class="tabs-list" role="tablist" aria-orientation={orientation}>
		{#each tabs as tab, index (tab.id)}
			<button
				type="button"
				id="tab-btn-{props.blockId}-{tab.id}"
				class="tab-button"
				class:active={activeTab === tab.id}
				role="tab"
				aria-selected={activeTab === tab.id}
				aria-controls="tab-panel-{props.blockId}-{tab.id}"
				tabindex={activeTab === tab.id ? 0 : -1}
				onclick={() => setActiveTab(tab.id)}
				onkeydown={(e) => handleKeyDown(e, index)}
			>
				{#if props.isEditing}
					<span
						contenteditable="true"
						class="tab-label editable-content"
						role="textbox"
						tabindex="0"
						aria-label="Tab label"
						onclick={(e) => e.stopPropagation()}
						onkeydown={(e) => e.stopPropagation()}
						oninput={(e) => updateTab(index, 'label', (e.target as HTMLElement).textContent || '')}
						onpaste={handlePaste}
					>
						{tab.label}
					</span>
				{:else}
					<span class="tab-label">{tab.label}</span>
				{/if}
			</button>
			{#if props.isEditing && tabs.length > 1}
				<button
					type="button"
					class="remove-tab-btn"
					onclick={(e) => {
						e.stopPropagation();
						removeTab(index);
					}}
					aria-label="Remove tab"
				>
					<IconX size={12} />
				</button>
			{/if}
		{/each}

		{#if props.isEditing}
			<button type="button" class="add-tab-btn" onclick={addTab} aria-label="Add tab">
				<IconPlus size={14} />
			</button>
		{/if}
	</div>

	<div class="tabs-panels">
		{#each tabs as tab, index (tab.id)}
			<div
				id="tab-panel-{props.blockId}-{tab.id}"
				class="tab-panel"
				class:active={activeTab === tab.id}
				class:fade-in={activeTab === tab.id && isTransitioning}
				class:fade-out={previousActiveTab === tab.id && isTransitioning}
				role="tabpanel"
				aria-labelledby="tab-btn-{props.blockId}-{tab.id}"
				hidden={activeTab !== tab.id && previousActiveTab !== tab.id}
				tabindex={activeTab === tab.id ? 0 : -1}
			>
				{#if props.isEditing}
					<div
						contenteditable="true"
						class="tab-content editable-content"
						class:placeholder={!tab.content}
						role="textbox"
						aria-label="Tab content"
						data-placeholder="Add content for this tab..."
						oninput={(e) => updateTab(index, 'content', (e.target as HTMLElement).textContent || '')}
						onpaste={handlePaste}
					>
						{tab.content}
					</div>
				{:else}
					<div class="tab-content">{tab.content}</div>
				{/if}
			</div>
		{/each}
	</div>

	{#if props.isEditing && props.isSelected}
		<div class="tabs-settings">
			<div class="setting-radio">
				<span class="setting-label">Orientation:</span>
				<label>
					<input
						type="radio"
						name="orientation-{props.blockId}"
						value="horizontal"
						checked={orientation === 'horizontal'}
						onchange={() =>
							props.onUpdate({
								settings: { ...props.block.settings, tabOrientation: 'horizontal' }
							})}
					/>
					<span>Horizontal</span>
				</label>
				<label>
					<input
						type="radio"
						name="orientation-{props.blockId}"
						value="vertical"
						checked={orientation === 'vertical'}
						onchange={() =>
							props.onUpdate({
								settings: { ...props.block.settings, tabOrientation: 'vertical' }
							})}
					/>
					<span>Vertical</span>
				</label>
			</div>
		</div>
	{/if}
</div>

<style>
	.tabs-block {
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		overflow: hidden;
	}

	.tabs-block.vertical {
		display: flex;
	}

	.tabs-list {
		display: flex;
		background: #f9fafb;
		border-bottom: 1px solid #e5e7eb;
		overflow-x: auto;
	}

	.tabs-block.vertical .tabs-list {
		flex-direction: column;
		border-bottom: none;
		border-right: 1px solid #e5e7eb;
		min-width: 160px;
	}

	.tab-button {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.25rem;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: #6b7280;
		font-size: 0.875rem;
		font-weight: 500;
		cursor: pointer;
		white-space: nowrap;
		transition: all 0.15s;
		position: relative;
	}

	.tabs-block.vertical .tab-button {
		border-bottom: none;
		border-left: 2px solid transparent;
		justify-content: flex-start;
	}

	.tab-button:hover {
		color: #374151;
		background: #f3f4f6;
	}

	.tab-button.active {
		color: #3b82f6;
		border-bottom-color: #3b82f6;
		background: white;
	}

	.tabs-block.vertical .tab-button.active {
		border-bottom-color: transparent;
		border-left-color: #3b82f6;
	}

	.tab-button:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
		z-index: 1;
	}

	.tab-label {
		outline: none;
	}

	.remove-tab-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.25rem;
		background: transparent;
		border: none;
		border-radius: 4px;
		color: #9ca3af;
		cursor: pointer;
		opacity: 0;
		transition: all 0.15s;
	}

	.tab-button:hover .remove-tab-btn {
		opacity: 1;
	}

	.remove-tab-btn:hover {
		background: #fee2e2;
		color: #dc2626;
	}

	.add-tab-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0.875rem 1rem;
		background: transparent;
		border: none;
		color: #9ca3af;
		cursor: pointer;
		transition: all 0.15s;
	}

	.add-tab-btn:hover {
		color: #3b82f6;
		background: #f3f4f6;
	}

	.add-tab-btn:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
	}

	.tabs-panels {
		flex: 1;
		position: relative;
		overflow: hidden;
	}

	.tab-panel {
		padding: 1.5rem;
		background: white;
		position: relative;
		opacity: 1;
		transition: opacity 0.2s ease-in-out;
	}

	.tab-panel[hidden] {
		display: none;
	}

	.tab-panel.fade-in {
		animation: fadeIn 0.2s ease-in-out forwards;
	}

	.tab-panel.fade-out {
		animation: fadeOut 0.2s ease-in-out forwards;
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(4px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@keyframes fadeOut {
		from {
			opacity: 1;
			transform: translateY(0);
		}
		to {
			opacity: 0;
			transform: translateY(-4px);
		}
	}

	.tab-panel:focus-visible {
		outline: 2px solid #3b82f6;
		outline-offset: -2px;
	}

	.tab-content {
		line-height: 1.7;
		color: #374151;
		outline: none;
		min-height: 100px;
	}

	.editable-content.placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
		pointer-events: none;
	}

	.tabs-settings {
		padding: 1rem 1.5rem;
		background: #f9fafb;
		border-top: 1px solid #e5e7eb;
	}

	.setting-radio {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-wrap: wrap;
		font-size: 0.875rem;
		color: #374151;
	}

	.setting-label {
		font-weight: 500;
	}

	.setting-radio label {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		cursor: pointer;
	}

	.setting-radio input {
		cursor: pointer;
	}

	/* Dark Mode */
	:global(.dark) .tabs-block {
		border-color: #374151;
	}

	:global(.dark) .tabs-list {
		background: #1e293b;
		border-color: #374151;
	}

	:global(.dark) .tab-button {
		color: #94a3b8;
	}

	:global(.dark) .tab-button:hover {
		color: #e2e8f0;
		background: #334155;
	}

	:global(.dark) .tab-button.active {
		color: #60a5fa;
		background: #111827;
	}

	:global(.dark) .tab-panel {
		background: #111827;
	}

	:global(.dark) .tab-content {
		color: #e5e7eb;
	}

	:global(.dark) .tabs-settings {
		background: #1e293b;
		border-color: #374151;
	}

	:global(.dark) .setting-radio {
		color: #e5e7eb;
	}

	:global(.dark) .add-tab-btn:hover {
		color: #60a5fa;
		background: #334155;
	}

	:global(.dark) .remove-tab-btn:hover {
		background: #450a0a;
		color: #fca5a5;
	}

	/* Mobile */
	@media (max-width: 640px) {
		.tabs-block.vertical {
			flex-direction: column;
		}

		.tabs-block.vertical .tabs-list {
			flex-direction: row;
			border-right: none;
			border-bottom: 1px solid #e5e7eb;
			min-width: 0;
		}

		.tabs-block.vertical .tab-button {
			border-left: none;
			border-bottom: 2px solid transparent;
		}

		.tabs-block.vertical .tab-button.active {
			border-left-color: transparent;
			border-bottom-color: #3b82f6;
		}
	}
</style>

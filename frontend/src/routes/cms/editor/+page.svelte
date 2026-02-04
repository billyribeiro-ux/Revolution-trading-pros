<!--
/**
 * CMS Block Editor - Example Page
 * Demonstrates the CMS block system with BlockStateManager integration
 */
-->

<script lang="ts">
    import { browser } from '$app/environment';
    import type { Block, BlockType } from '$lib/components/cms/blocks/types';
    import BlockLoader from '$lib/components/cms/blocks/BlockLoader.svelte';

    // State
    let blocks = $state<Block[]>([]);
    let selectedBlockId = $state<string | null>(null);
    let isEditing = $state(false);

    // Derived
    const selectedBlock = $derived(blocks.find(b => b.id === selectedBlockId));
    const blockCount = $derived(blocks.length);

    // Generate unique ID
    function generateId(): string {
        return `block-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    }

    // Create a new block
    function createBlock(type: BlockType): Block {
        return {
            id: generateId(),
            type,
            content: {},
            settings: {},
            metadata: {
                createdAt: Date.now(),
                updatedAt: Date.now(),
                version: 1
            }
        };
    }

    // Add block
    function addBlock(type: BlockType): void {
        const block = createBlock(type);
        blocks = [...blocks, block];
        selectedBlockId = block.id;
    }

    // Update block
    function updateBlock(blockId: string, updates: Partial<Block>): void {
        blocks = blocks.map(block =>
            block.id === blockId
                ? { ...block, ...updates, metadata: { ...block.metadata, updatedAt: Date.now() } }
                : block
        );
    }

    // Delete block
    function deleteBlock(blockId: string): void {
        blocks = blocks.filter(b => b.id !== blockId);
        if (selectedBlockId === blockId) {
            selectedBlockId = null;
        }
    }

    // Select block
    function selectBlock(blockId: string): void {
        selectedBlockId = blockId;
    }

    // Handle delete with stop propagation
    function handleDelete(event: MouseEvent, blockId: string): void {
        event.stopPropagation();
        deleteBlock(blockId);
    }

    // Available block types for quick add
    const blockTypes: { type: BlockType; label: string; icon: string }[] = [
        { type: 'paragraph', label: 'Paragraph', icon: 'P' },
        { type: 'heading', label: 'Heading', icon: 'H' },
        { type: 'image', label: 'Image', icon: 'IMG' },
        { type: 'video', label: 'Video', icon: 'VID' },
        { type: 'code', label: 'Code', icon: '</>' },
        { type: 'quote', label: 'Quote', icon: '"' },
        { type: 'list', label: 'List', icon: 'UL' },
        { type: 'accordion', label: 'Accordion', icon: 'ACC' },
        { type: 'callout', label: 'Callout', icon: '!' },
        { type: 'cta', label: 'CTA', icon: 'CTA' }
    ];

    // Initialize with a default paragraph
    $effect(() => {
        if (browser && blocks.length === 0) {
            addBlock('paragraph');
        }
    });
</script>

<svelte:head>
    <title>CMS Block Editor | Revolution Trading Pros</title>
</svelte:head>

<div class="editor">
    <header class="editor__header">
        <h1 class="editor__title">Block Editor</h1>
        <div class="editor__meta">
            {blockCount} block{blockCount !== 1 ? 's' : ''}
        </div>
    </header>

    <div class="editor__toolbar">
        {#each blockTypes as { type, label, icon }}
            <button
                class="editor__add-btn"
                onclick={() => addBlock(type)}
                title={`Add ${label}`}
            >
                <span class="editor__add-icon">{icon}</span>
                <span class="editor__add-label">{label}</span>
            </button>
        {/each}
    </div>

    <main class="editor__content">
        {#each blocks as block (block.id)}
            <div
                class="editor__block"
                class:editor__block--selected={block.id === selectedBlockId}
                onclick={() => selectBlock(block.id)}
                onkeydown={(e) => e.key === 'Enter' && selectBlock(block.id)}
                role="button"
                tabindex="0"
            >
                <div class="editor__block-actions">
                    <button
                        class="editor__block-delete"
                        onclick={(e) => handleDelete(e, block.id)}
                        title="Delete block"
                    >
                        x
                    </button>
                </div>

                <BlockLoader
                    {block}
                    blockId={block.id}
                    isSelected={block.id === selectedBlockId}
                    isEditing={isEditing && block.id === selectedBlockId}
                    onUpdate={(updates) => updateBlock(block.id, updates)}
                />
            </div>
        {/each}

        {#if blocks.length === 0}
            <div class="editor__empty">
                <p>No blocks yet. Add one to get started!</p>
            </div>
        {/if}
    </main>
</div>

<style>
    .editor {
        max-width: 900px;
        margin: 0 auto;
        padding: 2rem;
    }

    .editor__header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
    }

    .editor__title {
        font-size: 1.5rem;
        font-weight: 600;
        color: #1f2937;
    }

    .editor__meta {
        font-size: 0.875rem;
        color: #6b7280;
    }

    .editor__toolbar {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem;
        padding: 1rem;
        background: #f9fafb;
        border-radius: 0.5rem;
        margin-bottom: 1.5rem;
    }

    .editor__add-btn {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        padding: 0.5rem 0.75rem;
        background: white;
        border: 1px solid #e5e7eb;
        border-radius: 0.375rem;
        font-size: 0.875rem;
        cursor: pointer;
        transition: all 0.15s;
    }

    .editor__add-btn:hover {
        background: #f3f4f6;
        border-color: #d1d5db;
    }

    .editor__add-icon {
        font-size: 0.75rem;
        font-weight: 600;
        color: #6b7280;
    }

    .editor__content {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .editor__block {
        position: relative;
        padding: 1rem;
        border: 2px solid transparent;
        border-radius: 0.5rem;
        transition: border-color 0.15s;
    }

    .editor__block:hover {
        border-color: #e5e7eb;
    }

    .editor__block--selected {
        border-color: #3b82f6;
        background: rgba(59, 130, 246, 0.02);
    }

    .editor__block-actions {
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
        opacity: 0;
        transition: opacity 0.15s;
    }

    .editor__block:hover .editor__block-actions {
        opacity: 1;
    }

    .editor__block-delete {
        width: 1.5rem;
        height: 1.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #fee2e2;
        border: none;
        border-radius: 0.25rem;
        color: #dc2626;
        font-size: 1rem;
        cursor: pointer;
    }

    .editor__block-delete:hover {
        background: #fecaca;
    }

    .editor__empty {
        text-align: center;
        padding: 3rem;
        color: #6b7280;
    }

    /* Dark mode */
    :global(.dark) .editor__title {
        color: #f3f4f6;
    }

    :global(.dark) .editor__toolbar {
        background: #1f2937;
    }

    :global(.dark) .editor__add-btn {
        background: #374151;
        border-color: #4b5563;
        color: #e5e7eb;
    }

    :global(.dark) .editor__add-btn:hover {
        background: #4b5563;
    }

    :global(.dark) .editor__block:hover {
        border-color: #4b5563;
    }

    :global(.dark) .editor__block--selected {
        border-color: #3b82f6;
        background: rgba(59, 130, 246, 0.1);
    }
</style>

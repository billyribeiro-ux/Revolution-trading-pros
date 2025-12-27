<script lang="ts">
  /**
   * MediaGrid Component
   *
   * Displays media items in a responsive grid with selection,
   * preview, and bulk actions support.
   */
  import type { MediaItem } from '$lib/api/media';

  interface Props {
    items?: MediaItem[];
    selectedIds?: string[];
    selectable?: boolean;
    showStatus?: boolean;
    columns?: number;
    loading?: boolean;
    onselect?: (item: MediaItem) => void;
    onpreview?: (item: MediaItem) => void;
    ondelete?: (item: MediaItem) => void;
    onoptimize?: (item: MediaItem) => void;
    onselectionchange?: (ids: string[]) => void;
  }

  let { items = [], selectedIds = $bindable([]), selectable = true, showStatus = true, columns = 4, loading = false, onselect, onpreview, ondelete, onoptimize, onselectionchange }: Props = $props();

  // State
  let shiftKeyHeld = $state(false);
  let lastSelectedId: string | null = $state(null);

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Shift') shiftKeyHeld = true;
  }

  function handleKeyUp(e: KeyboardEvent) {
    if (e.key === 'Shift') shiftKeyHeld = false;
  }

  function toggleSelection(item: MediaItem) {
    if (!selectable) return;

    const idx = selectedIds.indexOf(item.id);

    if (shiftKeyHeld && lastSelectedId !== null) {
      // Range selection
      const lastIdx = items.findIndex((i) => i.id === lastSelectedId);
      const currentIdx = items.findIndex((i) => i.id === item.id);
      const sorted = [lastIdx, currentIdx].sort((a, b) => a - b);
      const start = sorted[0] ?? 0;
      const end = sorted[1] ?? 0;
      const rangeIds = items.slice(start, end + 1).map((i) => i.id);

      if (idx === -1) {
        // Add range
        selectedIds = [...new Set([...selectedIds, ...rangeIds])];
      } else {
        // Remove range
        selectedIds = selectedIds.filter((id) => !rangeIds.includes(id));
      }
    } else {
      // Single selection
      if (idx === -1) {
        selectedIds = [...selectedIds, item.id];
      } else {
        selectedIds = selectedIds.filter((id) => id !== item.id);
      }
    }

    lastSelectedId = item.id;
    onselectionchange?.(selectedIds);
  }

  function handleItemClick(item: MediaItem, e: MouseEvent) {
    if ((e.target as HTMLElement).closest('.item-actions')) {
      return; // Don't select when clicking actions
    }
    onselect?.(item);
  }

  function handlePreview(item: MediaItem, e: MouseEvent) {
    e.stopPropagation();
    onpreview?.(item);
  }

  function handleDelete(item: MediaItem, e: MouseEvent) {
    e.stopPropagation();
    ondelete?.(item);
  }

  function handleOptimize(item: MediaItem, e: MouseEvent) {
    e.stopPropagation();
    onoptimize?.(item);
  }

  function getStatusColor(status: string): string {
    switch (status) {
      case 'completed':
        return 'var(--success-color, #10b981)';
      case 'processing':
        return 'var(--warning-color, #f59e0b)';
      case 'failed':
        return 'var(--error-color, #ef4444)';
      default:
        return 'var(--text-muted, #9ca3af)';
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / 1024 / 1024).toFixed(2) + ' MB';
  }
</script>

<svelte:window onkeydown={handleKeyDown} onkeyup={handleKeyUp} />

<div class="media-grid" style="--columns: {columns}">
  {#if loading}
    <!-- Loading skeleton -->
    {#each Array(8) as _}
      <div class="grid-item skeleton">
        <div class="skeleton-image"></div>
        <div class="skeleton-text"></div>
      </div>
    {/each}
  {:else if items.length === 0}
    <!-- Empty state -->
    <div class="empty-state">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
        <circle cx="8.5" cy="8.5" r="1.5" />
        <path d="M21 15l-5-5L5 21" />
      </svg>
      <span>No media found</span>
    </div>
  {:else}
    {#each items as item (item.id)}
      {@const isSelected = selectedIds.includes(item.id)}
      <div
        class="grid-item"
        class:selected={isSelected}
        role="button"
        tabindex="0"
        onclick={(e: MouseEvent) => handleItemClick(item, e)}
        onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && onselect?.(item)}
      >
        <!-- Selection checkbox -->
        {#if selectable}
          <div class="item-checkbox">
            <input
              type="checkbox"
              checked={isSelected}
              onclick={(e: MouseEvent) => { e.stopPropagation(); toggleSelection(item); }}
              onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
            />
          </div>
        {/if}

        <!-- Thumbnail -->
        <div class="item-thumbnail">
          {#if item.file_type === 'image'}
            <img
              src={item.thumbnail_url || item.url}
              alt={item.alt_text || item.filename}
              loading="lazy"
            />
          {:else if item.file_type === 'video'}
            <div class="type-icon video">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
                <path d="M8 5v14l11-7z" />
              </svg>
            </div>
          {:else if item.file_type === 'document'}
            <div class="type-icon document">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z" />
                <path d="M14 2v6h6M16 13H8M16 17H8M10 9H8" />
              </svg>
            </div>
          {:else}
            <div class="type-icon other">
              <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M13 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V9z" />
                <path d="M13 2v7h7" />
              </svg>
            </div>
          {/if}

          <!-- Optimization status badge -->
          {#if showStatus && item.file_type === 'image'}
            <div class="status-badge" style="background: {getStatusColor(item.processing_status || 'pending')}">
              {#if item.is_optimized}
                <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
                </svg>
              {:else if item.processing_status === 'processing'}
                <svg class="animate-spin" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                  <circle cx="12" cy="12" r="10" stroke-dasharray="32" stroke-dashoffset="32" />
                </svg>
              {:else if item.processing_status === 'failed'}
                <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" />
                </svg>
              {:else}
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                  <circle cx="12" cy="12" r="10" />
                </svg>
              {/if}
            </div>
          {/if}

          <!-- Actions overlay -->
          <div class="item-actions">
            <button type="button" class="action-btn" title="Preview" onclick={(e: MouseEvent) => handlePreview(item, e)}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" />
                <circle cx="12" cy="12" r="3" />
              </svg>
            </button>
            {#if item.file_type === 'image' && !item.is_optimized}
              <button type="button" class="action-btn optimize" title="Optimize" onclick={(e: MouseEvent) => handleOptimize(item, e)}>
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
                </svg>
              </button>
            {/if}
            <button type="button" class="action-btn delete" title="Delete" onclick={(e: MouseEvent) => handleDelete(item, e)}>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Item info -->
        <div class="item-info">
          <span class="item-name" title={item.filename}>{item.filename}</span>
          <div class="item-meta">
            {#if item.width && item.height}
              <span>{item.width}x{item.height}</span>
            {/if}
            <span>{formatBytes(item.file_size)}</span>
          </div>
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .media-grid {
    display: grid;
    grid-template-columns: repeat(var(--columns), 1fr);
    gap: 1rem;
    padding: 1rem 0;
  }

  @media (max-width: 1024px) {
    .media-grid { --columns: 3 !important; }
  }

  @media (max-width: 768px) {
    .media-grid { --columns: 2 !important; }
  }

  @media (max-width: 480px) {
    .media-grid { --columns: 1 !important; }
  }

  .grid-item {
    position: relative;
    background: var(--bg-primary, white);
    border: 2px solid transparent;
    border-radius: 8px;
    overflow: hidden;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .grid-item:hover {
    border-color: var(--primary-color, #3b82f6);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .grid-item.selected {
    border-color: var(--primary-color, #3b82f6);
    background: var(--bg-selected, #eff6ff);
  }

  .item-checkbox {
    position: absolute;
    top: 8px;
    left: 8px;
    z-index: 10;
  }

  .item-checkbox input {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .item-thumbnail {
    position: relative;
    aspect-ratio: 1;
    background: var(--bg-secondary, #f3f4f6);
    overflow: hidden;
  }

  .item-thumbnail img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .type-icon {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted, #9ca3af);
  }

  .status-badge {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
  }

  .item-actions {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: linear-gradient(transparent, rgba(0, 0, 0, 0.7));
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .grid-item:hover .item-actions {
    opacity: 1;
  }

  .action-btn {
    width: 32px;
    height: 32px;
    border: none;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.9);
    color: var(--text-primary, #111827);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .action-btn:hover {
    background: white;
    transform: scale(1.1);
  }

  .action-btn.optimize:hover {
    background: var(--primary-color, #3b82f6);
    color: white;
  }

  .action-btn.delete:hover {
    background: var(--error-color, #ef4444);
    color: white;
  }

  .item-info {
    padding: 0.75rem;
  }

  .item-name {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary, #111827);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .item-meta {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.25rem;
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
  }

  /* Skeleton loading */
  .skeleton {
    pointer-events: none;
  }

  .skeleton-image {
    aspect-ratio: 1;
    background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
  }

  .skeleton-text {
    height: 40px;
    margin: 0.75rem;
    background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: 4px;
  }

  @keyframes shimmer {
    0% { background-position: -200% 0; }
    100% { background-position: 200% 0; }
  }

  /* Empty state */
  .empty-state {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 4rem 2rem;
    color: var(--text-muted, #9ca3af);
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>

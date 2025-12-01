<script lang="ts">
  /**
   * Media Library Admin Page
   *
   * Enterprise-grade media management with image optimization,
   * bulk operations, and analytics dashboard.
   */
  import { onMount } from 'svelte';
  import MediaUpload from '$lib/components/media/MediaUpload.svelte';
  import MediaGrid from '$lib/components/media/MediaGrid.svelte';
  import MediaPreview from '$lib/components/media/MediaPreview.svelte';
  import { mediaApi, type MediaItem, type OptimizationStatistics, type OptimizationPreset } from '$lib/api/media';

  // State
  let items: MediaItem[] = [];
  let selectedIds: string[] = [];
  let previewItem: MediaItem | null = null;
  let isPreviewOpen = false;
  let statistics: OptimizationStatistics | null = null;
  let presets: OptimizationPreset[] = [];
  let collections: string[] = [];

  // Pagination
  let currentPage = 1;
  let totalPages = 1;
  let totalItems = 0;
  let perPage = 24;

  // Filters
  let searchQuery = '';
  let filterType = '';
  let filterCollection = '';
  let filterOptimized = '';
  let sortBy = 'created_at';
  let sortDir: 'asc' | 'desc' = 'desc';

  // UI State
  let isLoading = true;
  let isOptimizing = false;
  let showUpload = false;
  let showStats = true;
  let selectedPreset = '';
  let error: string | null = null;
  let successMessage: string | null = null;

  // Load data on mount
  onMount(async () => {
    await Promise.all([loadMedia(), loadStatistics(), loadPresets(), loadCollections()]);
  });

  async function loadMedia() {
    isLoading = true;
    error = null;

    try {
      const response = await mediaApi.list({
        page: currentPage,
        per_page: perPage,
        search: searchQuery || undefined,
        type: filterType || undefined,
        collection: filterCollection || undefined,
        optimized: filterOptimized === 'optimized' ? true : filterOptimized === 'pending' ? false : undefined,
        needs_optimization: filterOptimized === 'pending' ? true : undefined,
        sort_by: sortBy,
        sort_dir: sortDir,
      });

      items = response.data;
      currentPage = response.meta.current_page;
      totalPages = response.meta.last_page;
      totalItems = response.meta.total;
    } catch (e: any) {
      error = e?.message || 'Failed to load media';
    } finally {
      isLoading = false;
    }
  }

  async function loadStatistics() {
    try {
      const response = await mediaApi.getStatistics();
      statistics = response.data;
    } catch (e) {
      console.error('Failed to load statistics', e);
    }
  }

  async function loadPresets() {
    try {
      const response = await mediaApi.getPresets();
      presets = response.data;
      const defaultPreset = presets.find((p) => p.is_default);
      if (defaultPreset) selectedPreset = defaultPreset.slug;
    } catch (e) {
      console.error('Failed to load presets', e);
    }
  }

  async function loadCollections() {
    try {
      // Collections endpoint not yet implemented, use empty array
      collections = [];
    } catch (e) {
      console.error('Failed to load collections', e);
    }
  }

  // Event handlers
  function handleUpload(item: MediaItem) {
    items = [item, ...items];
    totalItems++;
    loadStatistics();
    showSuccess('File uploaded successfully');
  }

  function handleUploadComplete(data: { uploaded: MediaItem[]; failed: { file: File; error: string }[] }) {
    if (data.failed.length > 0) {
      error = `${data.failed.length} file(s) failed to upload`;
    }
    if (data.uploaded.length > 0) {
      showSuccess(`${data.uploaded.length} file(s) uploaded successfully`);
    }
  }

  function handleSelect(item: MediaItem) {
    previewItem = item;
    isPreviewOpen = true;
  }

  function handlePreview(item: MediaItem) {
    previewItem = item;
    isPreviewOpen = true;
  }

  async function handleDelete(item: MediaItem) {
    if (!confirm(`Delete "${item.filename}"?`)) return;

    try {
      await mediaApi.delete(item.id);
      items = items.filter((i) => i.id !== item.id);
      selectedIds = selectedIds.filter((id) => id !== item.id);
      totalItems--;
      loadStatistics();
      showSuccess('File deleted');
      if (previewItem?.id === item.id) {
        isPreviewOpen = false;
        previewItem = null;
      }
    } catch (e: any) {
      error = e?.message || 'Failed to delete file';
    }
  }

  async function handleOptimize(item: MediaItem) {
    try {
      const response = await mediaApi.optimize(item.id, { preset: selectedPreset || undefined });
      if ('job_id' in response.data) {
        showSuccess('Optimization queued');
        // Update item status
        const idx = items.findIndex((i) => i.id === item.id);
        if (idx !== -1) {
          items[idx] = { ...items[idx], processing_status: 'processing' };
          items = items;
        }
      } else {
        items = items.map((i) => (i.id === item.id ? response.data as MediaItem : i));
        showSuccess('Image optimized successfully');
      }
      loadStatistics();
    } catch (err: any) {
      error = err?.message || 'Failed to optimize image';
    }
  }

  async function handleUpdate(data: { id: string; data: Partial<MediaItem> }) {
    try {
      const response = await mediaApi.update(data.id, data.data);
      items = items.map((i) => (i.id === data.id ? response.data : i));
      if (previewItem?.id === data.id) {
        previewItem = response.data;
      }
      showSuccess('Updated successfully');
    } catch (err: any) {
      error = err?.message || 'Failed to update';
    }
  }

  // Bulk operations
  async function handleBulkDelete() {
    if (selectedIds.length === 0) return;
    if (!confirm(`Delete ${selectedIds.length} selected items?`)) return;

    try {
      await mediaApi.bulkDelete(selectedIds);
      items = items.filter((i) => !selectedIds.includes(i.id));
      totalItems -= selectedIds.length;
      selectedIds = [];
      loadStatistics();
      showSuccess('Items deleted');
    } catch (e: any) {
      error = e?.message || 'Failed to delete items';
    }
  }

  async function handleBulkOptimize() {
    if (selectedIds.length === 0) return;
    isOptimizing = true;

    try {
      const response = await mediaApi.bulkOptimize(selectedIds, {
        preset: selectedPreset || undefined,
      });
      showSuccess(`${response.success} images queued for optimization`);
      // Update items status
      items = items.map((i) =>
        selectedIds.includes(i.id) && i.file_type === 'image' && !i.is_optimized
          ? { ...i, processing_status: 'processing' as const }
          : i
      );
      selectedIds = [];
      loadStatistics();
    } catch (e: any) {
      error = e?.message || 'Failed to queue optimizations';
    } finally {
      isOptimizing = false;
    }
  }

  async function handleOptimizeAll() {
    if (!confirm('Optimize all unoptimized images? This may take a while.')) return;
    isOptimizing = true;

    try {
      const response = await mediaApi.optimizeAll({
        preset: selectedPreset || undefined,
        limit: 100,
      });
      showSuccess(`${response.jobs_queued} optimization jobs queued (${response.total_pending} total pending)`);
      loadMedia();
      loadStatistics();
    } catch (e: any) {
      error = e?.message || 'Failed to queue optimizations';
    } finally {
      isOptimizing = false;
    }
  }

  function selectAll() {
    if (selectedIds.length === items.length) {
      selectedIds = [];
    } else {
      selectedIds = items.map((i) => i.id);
    }
  }

  // Pagination
  function goToPage(page: number) {
    if (page < 1 || page > totalPages) return;
    currentPage = page;
    loadMedia();
  }

  // Filters
  function applyFilters() {
    currentPage = 1;
    loadMedia();
  }

  function clearFilters() {
    searchQuery = '';
    filterType = '';
    filterCollection = '';
    filterOptimized = '';
    currentPage = 1;
    loadMedia();
  }

  // Helpers
  function showSuccess(message: string) {
    successMessage = message;
    setTimeout(() => (successMessage = null), 3000);
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(2) + ' MB';
    return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
  }
</script>

<svelte:head>
  <title>Media Library | Admin</title>
</svelte:head>

<div class="media-page">
  <!-- Header -->
  <div class="page-header">
    <div class="header-main">
      <h1>Media Library</h1>
      <p class="header-subtitle">Manage and optimize your images and files</p>
    </div>
    <div class="header-actions">
      <button type="button" class="btn btn-secondary" onclick={() => (showStats = !showStats)}>
        {showStats ? 'Hide Stats' : 'Show Stats'}
      </button>
      <button type="button" class="btn btn-primary" onclick={() => (showUpload = !showUpload)}>
        {showUpload ? 'Hide Upload' : 'Upload Files'}
      </button>
    </div>
  </div>

  <!-- Messages -->
  {#if error}
    <div class="alert alert-error">
      <span>{error}</span>
      <button type="button" onclick={() => (error = null)}>&times;</button>
    </div>
  {/if}
  {#if successMessage}
    <div class="alert alert-success">
      <span>{successMessage}</span>
    </div>
  {/if}

  <!-- Statistics -->
  {#if showStats && statistics}
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-icon images">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
            <circle cx="8.5" cy="8.5" r="1.5" />
            <path d="M21 15l-5-5L5 21" />
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-value">{statistics.total_images}</span>
          <span class="stat-label">Total Images</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon optimized">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-value">{statistics.optimized_images}</span>
          <span class="stat-label">Optimized</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon pending">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10" />
            <path d="M12 6v6l4 2" />
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-value">{statistics.pending_optimization}</span>
          <span class="stat-label">Pending</span>
        </div>
      </div>

      <div class="stat-card">
        <div class="stat-icon storage">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 7v10c0 2 1 3 3 3h10c2 0 3-1 3-3V7c0-2-1-3-3-3H7c-2 0-3 1-3 3z" />
            <path d="M8 7v10M12 7v10M16 7v10" />
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-value">{formatBytes(statistics.total_storage)}</span>
          <span class="stat-label">Total Storage</span>
        </div>
      </div>

      <div class="stat-card highlight">
        <div class="stat-icon savings">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 2v20M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6" />
          </svg>
        </div>
        <div class="stat-content">
          <span class="stat-value">{formatBytes(statistics.total_savings_bytes)}</span>
          <span class="stat-label">Total Savings</span>
        </div>
      </div>
    </div>
  {/if}

  <!-- Upload area -->
  {#if showUpload}
    <div class="upload-section">
      <MediaUpload
        collection={filterCollection || undefined}
        preset={selectedPreset || undefined}
        onupload={handleUpload}
        oncomplete={handleUploadComplete}
      />
    </div>
  {/if}

  <!-- Toolbar -->
  <div class="toolbar">
    <div class="toolbar-left">
      <!-- Search -->
      <div class="search-box">
        <input
          type="text"
          placeholder="Search files..."
          bind:value={searchQuery}
          onkeydown={(e) => e.key === 'Enter' && applyFilters()}
        />
        <button type="button" onclick={applyFilters} aria-label="Search">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="11" cy="11" r="8" />
            <path d="M21 21l-4.35-4.35" />
          </svg>
        </button>
      </div>

      <!-- Filters -->
      <select bind:value={filterType} onchange={applyFilters}>
        <option value="">All Types</option>
        <option value="image">Images</option>
        <option value="video">Videos</option>
        <option value="document">Documents</option>
      </select>

      <select bind:value={filterCollection} onchange={applyFilters}>
        <option value="">All Collections</option>
        {#each collections as collection}
          <option value={collection}>{collection}</option>
        {/each}
      </select>

      <select bind:value={filterOptimized} onchange={applyFilters}>
        <option value="">All Status</option>
        <option value="optimized">Optimized</option>
        <option value="pending">Needs Optimization</option>
      </select>

      {#if searchQuery || filterType || filterCollection || filterOptimized}
        <button type="button" class="btn btn-text" onclick={clearFilters}>
          Clear Filters
        </button>
      {/if}
    </div>

    <div class="toolbar-right">
      <!-- Preset selector -->
      <select bind:value={selectedPreset} class="preset-select">
        <option value="">Default Preset</option>
        {#each presets as preset}
          <option value={preset.slug}>{preset.name}</option>
        {/each}
      </select>

      <!-- Sort -->
      <select bind:value={sortBy} onchange={loadMedia}>
        <option value="created_at">Upload Date</option>
        <option value="filename">Name</option>
        <option value="size">Size</option>
      </select>

      <button
        type="button"
        class="btn btn-icon"
        onclick={() => {
          sortDir = sortDir === 'asc' ? 'desc' : 'asc';
          loadMedia();
        }}
        title={sortDir === 'asc' ? 'Ascending' : 'Descending'}
      >
        {#if sortDir === 'asc'}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 19V5M5 12l7-7 7 7" />
          </svg>
        {:else}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M12 5v14M19 12l-7 7-7-7" />
          </svg>
        {/if}
      </button>
    </div>
  </div>

  <!-- Bulk actions -->
  {#if selectedIds.length > 0}
    <div class="bulk-actions">
      <span class="selection-count">{selectedIds.length} selected</span>
      <button type="button" class="btn btn-secondary btn-sm" onclick={selectAll}>
        {selectedIds.length === items.length ? 'Deselect All' : 'Select All'}
      </button>
      <button
        type="button"
        class="btn btn-primary btn-sm"
        onclick={handleBulkOptimize}
        disabled={isOptimizing}
      >
        {isOptimizing ? 'Optimizing...' : 'Optimize Selected'}
      </button>
      <button type="button" class="btn btn-danger btn-sm" onclick={handleBulkDelete}>
        Delete Selected
      </button>
    </div>
  {/if}

  <!-- Optimize all button -->
  {#if statistics && statistics.pending_optimization > 0}
    <div class="optimize-all-banner">
      <span>
        {statistics.pending_optimization} image{statistics.pending_optimization !== 1 ? 's' : ''} need
        optimization
      </span>
      <button
        type="button"
        class="btn btn-primary btn-sm"
        onclick={handleOptimizeAll}
        disabled={isOptimizing}
      >
        {isOptimizing ? 'Processing...' : 'Optimize All'}
      </button>
    </div>
  {/if}

  <!-- Media grid -->
  <MediaGrid
    {items}
    {selectedIds}
    loading={isLoading}
    onselect={handleSelect}
    onpreview={handlePreview}
    ondelete={handleDelete}
    onoptimize={handleOptimize}
    onselectionchange={(ids) => (selectedIds = ids)}
  />

  <!-- Pagination -->
  {#if totalPages > 1}
    <div class="pagination">
      <button
        type="button"
        class="btn btn-icon"
        disabled={currentPage === 1}
        onclick={() => goToPage(currentPage - 1)}
        aria-label="Previous page"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M15 18l-6-6 6-6" />
        </svg>
      </button>

      <span class="page-info">
        Page {currentPage} of {totalPages} ({totalItems} items)
      </span>

      <button
        type="button"
        class="btn btn-icon"
        disabled={currentPage === totalPages}
        onclick={() => goToPage(currentPage + 1)}
        aria-label="Next page"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M9 18l6-6-6-6" />
        </svg>
      </button>
    </div>
  {/if}
</div>

<!-- Preview modal -->
<MediaPreview
  item={previewItem}
  isOpen={isPreviewOpen}
  onclose={() => {
    isPreviewOpen = false;
    previewItem = null;
  }}
  onoptimize={handleOptimize}
  ondelete={handleDelete}
  onupdate={handleUpdate}
/>

<style>
  .media-page {
    padding: 1.5rem;
    max-width: 1600px;
    margin: 0 auto;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1.5rem;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .page-header h1 {
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--text-primary, #111827);
    margin: 0;
  }

  .header-subtitle {
    font-size: 0.875rem;
    color: var(--text-muted, #6b7280);
    margin: 0.25rem 0 0 0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
  }

  /* Alerts */
  .alert {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-radius: 8px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .alert-error {
    background: var(--error-bg, #fef2f2);
    color: var(--error-color, #dc2626);
  }

  .alert-success {
    background: var(--success-bg, #d1fae5);
    color: var(--success-color, #059669);
  }

  .alert button {
    background: none;
    border: none;
    font-size: 1.25rem;
    cursor: pointer;
    opacity: 0.7;
  }

  /* Stats */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .stat-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: var(--bg-primary, white);
    border: 1px solid var(--border-color, #e5e7eb);
    border-radius: 8px;
  }

  .stat-card.highlight {
    background: linear-gradient(135deg, var(--success-bg, #d1fae5), var(--bg-primary, white));
    border-color: var(--success-color, #10b981);
  }

  .stat-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stat-icon.images {
    background: var(--primary-bg, #dbeafe);
    color: var(--primary-color, #3b82f6);
  }

  .stat-icon.optimized {
    background: var(--success-bg, #d1fae5);
    color: var(--success-color, #10b981);
  }

  .stat-icon.pending {
    background: var(--warning-bg, #fef3c7);
    color: var(--warning-color, #d97706);
  }

  .stat-icon.storage {
    background: var(--purple-bg, #ede9fe);
    color: var(--purple-color, #7c3aed);
  }

  .stat-icon.savings {
    background: var(--success-bg, #d1fae5);
    color: var(--success-color, #10b981);
  }

  .stat-content {
    display: flex;
    flex-direction: column;
  }

  .stat-value {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-primary, #111827);
  }

  .stat-label {
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
  }

  /* Upload section */
  .upload-section {
    margin-bottom: 1.5rem;
  }

  /* Toolbar */
  .toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1rem;
    flex-wrap: wrap;
  }

  .toolbar-left,
  .toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .search-box {
    display: flex;
    align-items: center;
    background: var(--bg-primary, white);
    border: 1px solid var(--border-color, #d1d5db);
    border-radius: 8px;
    overflow: hidden;
  }

  .search-box input {
    padding: 0.5rem 0.75rem;
    border: none;
    outline: none;
    font-size: 0.875rem;
    width: 200px;
  }

  .search-box button {
    padding: 0.5rem 0.75rem;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted, #6b7280);
  }

  .toolbar select {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border-color, #d1d5db);
    border-radius: 8px;
    font-size: 0.875rem;
    background: var(--bg-primary, white);
    cursor: pointer;
  }

  .preset-select {
    min-width: 150px;
  }

  /* Bulk actions */
  .bulk-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: var(--bg-secondary, #f3f4f6);
    border-radius: 8px;
    margin-bottom: 1rem;
  }

  .selection-count {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary, #111827);
  }

  /* Optimize all banner */
  .optimize-all-banner {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    background: var(--warning-bg, #fef3c7);
    border-radius: 8px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
    color: var(--warning-dark, #92400e);
  }

  /* Pagination */
  .pagination {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 1.5rem 0;
  }

  .page-info {
    font-size: 0.875rem;
    color: var(--text-muted, #6b7280);
  }

  /* Buttons */
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    border: none;
  }

  .btn-sm {
    padding: 0.375rem 0.75rem;
    font-size: 0.75rem;
  }

  .btn-icon {
    padding: 0.5rem;
  }

  .btn-primary {
    background: var(--primary-color, #3b82f6);
    color: white;
  }

  .btn-primary:hover {
    background: var(--primary-dark, #2563eb);
  }

  .btn-primary:disabled {
    background: var(--primary-light, #93c5fd);
    cursor: not-allowed;
  }

  .btn-secondary {
    background: var(--bg-primary, white);
    color: var(--text-primary, #374151);
    border: 1px solid var(--border-color, #d1d5db);
  }

  .btn-secondary:hover {
    background: var(--bg-hover, #f9fafb);
  }

  .btn-danger {
    background: var(--error-color, #ef4444);
    color: white;
  }

  .btn-danger:hover {
    background: var(--error-dark, #dc2626);
  }

  .btn-text {
    background: none;
    color: var(--primary-color, #3b82f6);
    padding: 0.25rem 0.5rem;
  }

  .btn-text:hover {
    background: var(--bg-hover, #f3f4f6);
  }

  @media (max-width: 768px) {
    .page-header {
      flex-direction: column;
      align-items: stretch;
    }

    .header-actions {
      justify-content: flex-end;
    }

    .toolbar {
      flex-direction: column;
      align-items: stretch;
    }

    .toolbar-left,
    .toolbar-right {
      width: 100%;
    }

    .search-box {
      flex: 1;
    }

    .search-box input {
      width: 100%;
    }
  }
</style>

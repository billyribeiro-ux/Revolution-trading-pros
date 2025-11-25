<script lang="ts">
  /**
   * MediaAnalytics Component
   *
   * Displays optimization statistics, savings charts,
   * and storage usage breakdowns.
   */
  import { onMount } from 'svelte';
  import { mediaApi, type MediaStatistics } from '$lib/api/media';

  // Props
  export let compact: boolean = false;

  // State
  let statistics: MediaStatistics | null = null;
  let loading = true;
  let error: string | null = null;

  onMount(async () => {
    await loadStatistics();
  });

  async function loadStatistics() {
    loading = true;
    error = null;

    try {
      const response = await mediaApi.getStatistics();
      statistics = response.data;
    } catch (e: any) {
      error = e?.message || 'Failed to load statistics';
    } finally {
      loading = false;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(2) + ' MB';
    return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
  }

  function formatNumber(num: number): string {
    if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
    if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
    return num.toString();
  }

  function getOptimizationRate(): number {
    if (!statistics || statistics.total_images === 0) return 0;
    return Math.round((statistics.optimized_images / statistics.total_images) * 100);
  }

  function getSavingsRate(): number {
    if (!statistics || statistics.total_storage === 0) return 0;
    return Math.round((statistics.total_savings_bytes / statistics.total_storage) * 100);
  }

  function getStorageTypes(): Array<{ type: string; count: number; size: number; color: string }> {
    if (!statistics?.storage_by_type) return [];

    const colors: Record<string, string> = {
      image: '#3b82f6',
      video: '#ef4444',
      document: '#10b981',
      audio: '#f59e0b',
      archive: '#8b5cf6',
      other: '#6b7280',
    };

    return Object.entries(statistics.storage_by_type).map(([type, data]) => ({
      type,
      count: data.count,
      size: data.size,
      color: colors[type] || colors.other,
    }));
  }
</script>

<div class="media-analytics" class:compact>
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <span>Loading analytics...</span>
    </div>
  {:else if error}
    <div class="error">
      <span>{error}</span>
      <button type="button" on:click={loadStatistics}>Retry</button>
    </div>
  {:else if statistics}
    <!-- Summary Cards -->
    <div class="summary-grid">
      <div class="summary-card primary">
        <div class="card-icon">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z" />
          </svg>
        </div>
        <div class="card-content">
          <span class="card-value">{formatBytes(statistics.total_savings_bytes)}</span>
          <span class="card-label">Total Savings</span>
        </div>
        <div class="card-trend positive">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 15l-6-6-6 6" />
          </svg>
          <span>{getSavingsRate()}%</span>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon storage">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M4 7v10c0 2 1 3 3 3h10c2 0 3-1 3-3V7c0-2-1-3-3-3H7c-2 0-3 1-3 3z" />
          </svg>
        </div>
        <div class="card-content">
          <span class="card-value">{formatBytes(statistics.total_storage)}</span>
          <span class="card-label">Total Storage</span>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon images">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
            <circle cx="8.5" cy="8.5" r="1.5" />
            <path d="M21 15l-5-5L5 21" />
          </svg>
        </div>
        <div class="card-content">
          <span class="card-value">{formatNumber(statistics.total_images)}</span>
          <span class="card-label">Total Images</span>
        </div>
      </div>

      <div class="summary-card">
        <div class="card-icon variants">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="2" y="2" width="8" height="8" rx="1" />
            <rect x="14" y="2" width="8" height="8" rx="1" />
            <rect x="2" y="14" width="8" height="8" rx="1" />
            <rect x="14" y="14" width="8" height="8" rx="1" />
          </svg>
        </div>
        <div class="card-content">
          <span class="card-value">{formatNumber(statistics.total_variants)}</span>
          <span class="card-label">Variants</span>
        </div>
      </div>
    </div>

    {#if !compact}
      <!-- Optimization Progress -->
      <div class="section">
        <h3 class="section-title">Optimization Progress</h3>
        <div class="progress-container">
          <div class="progress-bar">
            <div
              class="progress-fill"
              style="width: {getOptimizationRate()}%"
            ></div>
          </div>
          <div class="progress-stats">
            <span class="progress-label">
              <span class="optimized">{statistics.optimized_images}</span> of {statistics.total_images} images optimized
            </span>
            <span class="progress-percent">{getOptimizationRate()}%</span>
          </div>
        </div>

        {#if statistics.pending_optimization > 0}
          <div class="pending-alert">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10" />
              <path d="M12 6v6l4 2" />
            </svg>
            <span>{statistics.pending_optimization} images pending optimization</span>
          </div>
        {/if}
      </div>

      <!-- Storage by Type -->
      <div class="section">
        <h3 class="section-title">Storage by Type</h3>
        <div class="storage-breakdown">
          {#each getStorageTypes() as item}
            <div class="storage-item">
              <div class="storage-bar" style="--color: {item.color}">
                <div
                  class="storage-fill"
                  style="width: {(item.size / statistics.total_storage) * 100}%"
                ></div>
              </div>
              <div class="storage-info">
                <span class="storage-type">{item.type}</span>
                <span class="storage-count">{item.count} files</span>
                <span class="storage-size">{formatBytes(item.size)}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <!-- Queue Status -->
      {#if statistics.job_stats}
        <div class="section">
          <h3 class="section-title">Optimization Queue</h3>
          <div class="queue-stats">
            <div class="queue-item">
              <span class="queue-count pending">{statistics.job_stats.pending}</span>
              <span class="queue-label">Pending</span>
            </div>
            <div class="queue-item">
              <span class="queue-count processing">{statistics.job_stats.processing}</span>
              <span class="queue-label">Processing</span>
            </div>
            <div class="queue-item">
              <span class="queue-count completed">{statistics.job_stats.completed}</span>
              <span class="queue-label">Completed</span>
            </div>
            <div class="queue-item">
              <span class="queue-count failed">{statistics.job_stats.failed}</span>
              <span class="queue-label">Failed</span>
            </div>
          </div>

          {#if statistics.job_stats.avg_processing_time_ms}
            <div class="queue-meta">
              <span>Avg. processing time: {(statistics.job_stats.avg_processing_time_ms / 1000).toFixed(2)}s</span>
            </div>
          {/if}
        </div>
      {/if}

      <!-- Additional Stats -->
      <div class="section">
        <h3 class="section-title">Usage Statistics</h3>
        <div class="usage-grid">
          <div class="usage-item">
            <span class="usage-value">{formatNumber(statistics.total_downloads)}</span>
            <span class="usage-label">Downloads</span>
          </div>
          <div class="usage-item">
            <span class="usage-value">{formatNumber(statistics.total_views)}</span>
            <span class="usage-label">Views</span>
          </div>
          <div class="usage-item">
            <span class="usage-value">{statistics.unused_files}</span>
            <span class="usage-label">Unused Files</span>
          </div>
          <div class="usage-item">
            <span class="usage-value">{statistics.needs_optimization}</span>
            <span class="usage-label">Needs Optimization</span>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  .media-analytics {
    padding: 1.5rem;
    background: var(--bg-primary, white);
    border-radius: 12px;
    border: 1px solid var(--border-color, #e5e7eb);
  }

  .media-analytics.compact {
    padding: 1rem;
  }

  .loading,
  .error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 2rem;
    color: var(--text-muted, #6b7280);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border-color, #e5e7eb);
    border-top-color: var(--primary-color, #3b82f6);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error button {
    padding: 0.5rem 1rem;
    background: var(--primary-color, #3b82f6);
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
  }

  /* Summary Cards */
  .summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .compact .summary-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .summary-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    background: var(--bg-secondary, #f9fafb);
    border-radius: 8px;
    position: relative;
  }

  .summary-card.primary {
    background: linear-gradient(135deg, var(--success-bg, #d1fae5), var(--bg-secondary, #f9fafb));
    border: 1px solid var(--success-color, #10b981);
  }

  .card-icon {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--primary-bg, #dbeafe);
    color: var(--primary-color, #3b82f6);
  }

  .card-icon.storage {
    background: var(--purple-bg, #ede9fe);
    color: var(--purple-color, #7c3aed);
  }

  .card-icon.images {
    background: var(--primary-bg, #dbeafe);
    color: var(--primary-color, #3b82f6);
  }

  .card-icon.variants {
    background: var(--orange-bg, #ffedd5);
    color: var(--orange-color, #ea580c);
  }

  .summary-card.primary .card-icon {
    background: var(--success-color, #10b981);
    color: white;
  }

  .card-content {
    flex: 1;
  }

  .card-value {
    display: block;
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-primary, #111827);
  }

  .card-label {
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
  }

  .card-trend {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .card-trend.positive {
    color: var(--success-color, #10b981);
  }

  /* Sections */
  .section {
    margin-top: 1.5rem;
    padding-top: 1.5rem;
    border-top: 1px solid var(--border-color, #e5e7eb);
  }

  .section-title {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary, #111827);
    margin: 0 0 1rem 0;
  }

  /* Progress */
  .progress-container {
    background: var(--bg-secondary, #f9fafb);
    border-radius: 8px;
    padding: 1rem;
  }

  .progress-bar {
    height: 12px;
    background: var(--bg-tertiary, #e5e7eb);
    border-radius: 6px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--primary-color, #3b82f6), var(--success-color, #10b981));
    border-radius: 6px;
    transition: width 0.5s ease;
  }

  .progress-stats {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 0.5rem;
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
  }

  .progress-label .optimized {
    color: var(--success-color, #10b981);
    font-weight: 600;
  }

  .progress-percent {
    font-weight: 700;
    color: var(--primary-color, #3b82f6);
  }

  .pending-alert {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.75rem;
    padding: 0.5rem 0.75rem;
    background: var(--warning-bg, #fef3c7);
    color: var(--warning-dark, #92400e);
    border-radius: 6px;
    font-size: 0.75rem;
  }

  /* Storage Breakdown */
  .storage-breakdown {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .storage-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .storage-bar {
    height: 8px;
    background: var(--bg-tertiary, #e5e7eb);
    border-radius: 4px;
    overflow: hidden;
  }

  .storage-fill {
    height: 100%;
    background: var(--color);
    border-radius: 4px;
    transition: width 0.5s ease;
  }

  .storage-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.75rem;
  }

  .storage-type {
    font-weight: 500;
    color: var(--text-primary, #111827);
    text-transform: capitalize;
    min-width: 80px;
  }

  .storage-count {
    color: var(--text-muted, #9ca3af);
  }

  .storage-size {
    margin-left: auto;
    font-weight: 500;
    color: var(--text-secondary, #374151);
  }

  /* Queue Stats */
  .queue-stats {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.75rem;
  }

  .queue-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0.75rem;
    background: var(--bg-secondary, #f9fafb);
    border-radius: 8px;
  }

  .queue-count {
    font-size: 1.25rem;
    font-weight: 700;
  }

  .queue-count.pending {
    color: var(--warning-color, #f59e0b);
  }

  .queue-count.processing {
    color: var(--primary-color, #3b82f6);
  }

  .queue-count.completed {
    color: var(--success-color, #10b981);
  }

  .queue-count.failed {
    color: var(--error-color, #ef4444);
  }

  .queue-label {
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
  }

  .queue-meta {
    margin-top: 0.75rem;
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
    text-align: center;
  }

  /* Usage Grid */
  .usage-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.75rem;
  }

  .usage-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0.75rem;
    background: var(--bg-secondary, #f9fafb);
    border-radius: 8px;
  }

  .usage-value {
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--text-primary, #111827);
  }

  .usage-label {
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
  }

  @media (max-width: 640px) {
    .queue-stats,
    .usage-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>

<script lang="ts">
  /**
   * Invalidated Trades Archive
   * @version 1.0.0
   * @standards Apple ICT 7+ | Svelte 5 January 2026 | WCAG 2.1 AA
   */
  import { onMount } from 'svelte';
  import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
  import { ROOM_SLUG } from '../constants';

  interface InvalidatedTrade {
    id: number;
    ticker: string;
    entryDate: string;
    entryPrice: number;
    setup: string;
    invalidationReason: string;
    invalidatedAt: string;
  }

  let trades = $state<InvalidatedTrade[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  async function fetchInvalidatedTrades() {
    isLoading = true;
    error = null;

    try {
      const response = await fetch(`/api/room-content/rooms/${ROOM_SLUG}/trades?status=invalidated`, {
        credentials: 'include'
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const result = await response.json();
      
      // API returns { data: [...], meta: {...} }
      const data = result.data || [];

      trades = data.map((t: any) => ({
        id: t.id,
        ticker: t.ticker,
        entryDate: t.entry_date,
        entryPrice: t.entry_price,
        setup: t.setup || 'N/A',
        invalidationReason: t.invalidation_reason || 'No reason provided',
        invalidatedAt: t.updated_at
      }));
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load';
      console.error('Failed to fetch invalidated trades:', err);
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    fetchInvalidatedTrades();
  });

  function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      year: 'numeric'
    });
  }

  function formatPrice(price: number): string {
    return `$${price.toFixed(2)}`;
  }
</script>

<svelte:head>
  <title>Invalidated Trades | Explosive Swings</title>
</svelte:head>

<TradingRoomHeader
  roomName="Explosive Swings"
  startHereUrl="/dashboard/explosive-swings/start-here"
/>

<main class="invalidated-page">
  <header class="page-header">
    <div class="header-content">
      <a href="/dashboard/explosive-swings/trades" class="back-link">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="20" height="20">
          <path d="M19 12H5M12 19l-7-7 7-7"/>
        </svg>
        Back to Trades
      </a>
      <h1>Invalidated Trades</h1>
      <p>Setups that didn't trigger or were cancelled before entry</p>
    </div>
  </header>

  <section class="content-section">
    {#if isLoading}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading invalidated trades...</p>
      </div>
    {:else if error}
      <div class="error-state" role="alert">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48">
          <circle cx="12" cy="12" r="10"/>
          <path d="M12 8v4m0 4h.01"/>
        </svg>
        <h3>Unable to load trades</h3>
        <p>{error}</p>
        <button type="button" class="retry-btn" onclick={fetchInvalidatedTrades}>
          Try Again
        </button>
      </div>
    {:else if trades.length === 0}
      <div class="empty-state">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48">
          <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
        <h3>No invalidated trades</h3>
        <p>All setups have been executed successfully.</p>
        <a href="/dashboard/explosive-swings/trades" class="back-btn">
          View All Trades
        </a>
      </div>
    {:else}
      <div class="trades-grid">
        {#each trades as trade (trade.id)}
          <article class="trade-card">
            <header class="card-header">
              <span class="ticker">{trade.ticker}</span>
              <span class="badge invalidated">INVALIDATED</span>
            </header>

            <div class="card-body">
              <div class="detail-row">
                <span class="label">Planned Entry</span>
                <span class="value">{formatPrice(trade.entryPrice)}</span>
              </div>
              <div class="detail-row">
                <span class="label">Setup</span>
                <span class="value">{trade.setup}</span>
              </div>
              <div class="detail-row">
                <span class="label">Date</span>
                <span class="value">{formatDate(trade.entryDate)}</span>
              </div>
            </div>

            <footer class="card-footer">
              <div class="reason">
                <span class="reason-label">Reason:</span>
                <p class="reason-text">{trade.invalidationReason}</p>
              </div>
            </footer>
          </article>
        {/each}
      </div>

      <p class="summary-text">
        Showing {trades.length} invalidated trade{trades.length !== 1 ? 's' : ''}
      </p>
    {/if}
  </section>
</main>

<style>
  /* Mobile-first base styles */
  .invalidated-page {
    background: var(--color-bg-page);
    min-height: 100vh;
    padding: var(--space-4);
  }

  .page-header {
    max-width: 1200px;
    margin: 0 auto var(--space-6);
  }

  .header-content {
    text-align: center;
  }

  .back-link {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--color-text-tertiary);
    text-decoration: none;
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    margin-bottom: var(--space-4);
    transition: color 0.15s;
  }

  .back-link:hover {
    color: var(--color-brand-primary);
  }

  .page-header h1 {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
    margin: 0 0 var(--space-2);
    font-family: var(--font-display);
  }

  .page-header p {
    font-size: var(--text-base);
    color: var(--color-text-tertiary);
    margin: 0;
  }

  .content-section {
    max-width: 1200px;
    margin: 0 auto;
  }

  /* Loading State */
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-12);
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--color-border-default);
    border-top-color: var(--color-brand-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    margin-bottom: var(--space-4);
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .loading-state p {
    color: var(--color-text-tertiary);
    font-size: var(--text-sm);
  }

  /* Error State */
  .error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-12);
    text-align: center;
    background: var(--color-bg-card);
    border-radius: var(--radius-xl);
    border: 1px solid var(--color-border-default);
  }

  .error-state svg {
    color: var(--color-loss);
    margin-bottom: var(--space-4);
  }

  .error-state h3 {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--space-2);
  }

  .error-state p {
    color: var(--color-text-tertiary);
    margin: 0 0 var(--space-4);
  }

  .retry-btn {
    background: var(--color-loss);
    color: white;
    border: none;
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    cursor: pointer;
    transition: background 0.15s;
  }

  .retry-btn:hover {
    background: var(--color-loss-hover, #dc2626);
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-12);
    text-align: center;
    background: var(--color-bg-card);
    border-radius: var(--radius-xl);
    border: 1px solid var(--color-border-default);
  }

  .empty-state svg {
    color: var(--color-profit);
    margin-bottom: var(--space-4);
  }

  .empty-state h3 {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--space-2);
  }

  .empty-state p {
    color: var(--color-text-tertiary);
    margin: 0 0 var(--space-4);
  }

  .back-btn {
    background: var(--color-brand-primary);
    color: white;
    text-decoration: none;
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    transition: background 0.15s;
  }

  .back-btn:hover {
    background: var(--color-brand-primary-hover);
  }

  /* Trades Grid - Mobile first (1 column) */
  .trades-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: var(--space-4);
  }

  /* Trade Card */
  .trade-card {
    background: var(--color-bg-card);
    border: 1px solid var(--color-border-default);
    border-radius: var(--radius-lg);
    overflow: hidden;
    transition: box-shadow 0.15s;
  }

  .trade-card:hover {
    box-shadow: var(--shadow-md);
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-4);
    background: var(--color-bg-subtle);
    border-bottom: 1px solid var(--color-border-default);
  }

  .ticker {
    font-size: var(--text-lg);
    font-weight: var(--font-bold);
    color: var(--color-brand-primary);
    font-family: var(--font-display);
  }

  .badge {
    font-size: var(--text-xs);
    font-weight: var(--font-bold);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .badge.invalidated {
    background: var(--color-bg-muted);
    color: var(--color-text-muted);
    text-decoration: line-through;
  }

  .card-body {
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .label {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
  }

  .value {
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    font-variant-numeric: tabular-nums;
  }

  .card-footer {
    padding: var(--space-4);
    background: var(--color-bg-subtle);
    border-top: 1px solid var(--color-border-default);
  }

  .reason {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .reason-label {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-tertiary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .reason-text {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    margin: 0;
    line-height: 1.5;
  }

  .summary-text {
    text-align: center;
    color: var(--color-text-tertiary);
    font-size: var(--text-sm);
    margin-top: var(--space-6);
  }

  /* Tablet: 2 columns */
  @media (min-width: 640px) {
    .invalidated-page {
      padding: var(--space-6);
    }

    .page-header h1 {
      font-size: var(--text-3xl);
    }

    .trades-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  /* Desktop: 3 columns */
  @media (min-width: 1024px) {
    .invalidated-page {
      padding: var(--space-8);
    }

    .trades-grid {
      grid-template-columns: repeat(3, 1fr);
    }
  }
</style>

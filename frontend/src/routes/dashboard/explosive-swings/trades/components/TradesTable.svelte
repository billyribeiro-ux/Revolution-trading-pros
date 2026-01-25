<script lang="ts">
  /**
   * ═══════════════════════════════════════════════════════════════════════════════
   * TradesTable Component - Semantic HTML Table with Accessibility
   * ═══════════════════════════════════════════════════════════════════════════════
   * 
   * Uses native <table> elements for proper screen reader navigation.
   * Supports expandable notes rows and admin actions.
   * 
   * @version 1.0.0
   * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
   */
  import type { Trade } from '../types';

  interface Props {
    trades: Trade[];
    isAdmin?: boolean;
    onCloseTrade?: (trade: Trade) => void;
  }

  const { trades, isAdmin = false, onCloseTrade }: Props = $props();

  // Track expanded notes
  let expandedNotes = $state<Set<number>>(new Set());

  function toggleNotes(tradeId: number) {
    const newSet = new Set(expandedNotes);
    if (newSet.has(tradeId)) {
      newSet.delete(tradeId);
    } else {
      newSet.add(tradeId);
    }
    expandedNotes = newSet;
  }

  function formatPrice(price: number | null): string {
    if (price === null) return '—';
    return `$${price.toFixed(2)}`;
  }

  function formatProfit(profit: number): string {
    if (profit === 0) return '—';
    return `$${profit.toLocaleString()}`;
  }

  function formatPercent(percent: number): string {
    if (percent === 0) return '—';
    const sign = percent > 0 ? '+' : '';
    return `${sign}${percent.toFixed(1)}%`;
  }
</script>

<div class="table-container">
  <table class="trades-table" aria-label="Trade history">
    <thead>
      <tr>
        <th scope="col">Ticker</th>
        <th scope="col">Entry</th>
        <th scope="col">Exit</th>
        <th scope="col" class="numeric">Entry $</th>
        <th scope="col" class="numeric">Exit $</th>
        <th scope="col" class="numeric">Profit</th>
        <th scope="col" class="numeric">%</th>
        <th scope="col" class="numeric">Days</th>
        <th scope="col">Setup</th>
        <th scope="col">Result</th>
        {#if isAdmin}
          <th scope="col">Actions</th>
        {/if}
      </tr>
    </thead>
    <tbody>
      {#each trades as trade (trade.id)}
        {@const hasNotes = trade.notes && trade.notes.trim().length > 0}
        {@const isExpanded = expandedNotes.has(trade.id)}
        
        <tr 
          class:active={trade.result === 'ACTIVE'}
          class:expandable={hasNotes}
        >
          <td class="ticker-cell">
            {#if hasNotes}
              <button 
                type="button"
                class="ticker-btn"
                onclick={() => toggleNotes(trade.id)}
                aria-expanded={isExpanded}
                aria-controls="notes-{trade.id}"
              >
                <span class="expand-icon" class:expanded={isExpanded}>
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="14" height="14">
                    <path d="M9 18l6-6-6-6" />
                  </svg>
                </span>
                {trade.ticker}
              </button>
            {:else}
              {trade.ticker}
            {/if}
          </td>
          <td>{trade.entryDate}</td>
          <td>{trade.exitDate ?? 'Active'}</td>
          <td class="numeric">{formatPrice(trade.entryPrice)}</td>
          <td class="numeric">{formatPrice(trade.exitPrice)}</td>
          <td class="numeric" class:profit={trade.profit > 0} class:loss={trade.profit < 0}>
            {formatProfit(trade.profit)}
          </td>
          <td class="numeric" class:profit={trade.profitPercent > 0} class:loss={trade.profitPercent < 0}>
            {formatPercent(trade.profitPercent)}
          </td>
          <td class="numeric">{trade.duration || '—'}</td>
          <td><span class="setup-badge">{trade.setup}</span></td>
          <td>
            <span class="result-badge result--{trade.result.toLowerCase()}">
              {trade.result}
            </span>
          </td>
          {#if isAdmin}
            <td class="actions-cell">
              {#if trade.result === 'ACTIVE'}
                <button 
                  type="button"
                  class="close-trade-btn" 
                  onclick={() => onCloseTrade?.(trade)}
                  aria-label="Close {trade.ticker} trade"
                >
                  Close
                </button>
              {:else}
                <span class="closed-indicator">Closed</span>
              {/if}
            </td>
          {/if}
        </tr>
        
        <!-- Expandable Notes Row -->
        {#if hasNotes}
          <tr 
            id="notes-{trade.id}"
            class="notes-row"
            class:expanded={isExpanded}
            aria-hidden={!isExpanded}
          >
            <td colspan={isAdmin ? 11 : 10}>
              <div class="notes-content">
                <strong>Notes:</strong> {trade.notes}
              </div>
            </td>
          </tr>
        {/if}
      {/each}
    </tbody>
  </table>
</div>

<style>
  .table-container {
    max-width: 1400px;
    margin: 0 auto;
    background: var(--color-bg-card);
    border-radius: var(--radius-xl);
    padding: var(--space-6);
    box-shadow: var(--shadow-md);
    overflow-x: auto;
    
    /* Scroll shadow affordance */
    background: 
      linear-gradient(to right, var(--color-bg-card) 30%, transparent),
      linear-gradient(to right, transparent, var(--color-bg-card) 70%) 100% 0,
      radial-gradient(farthest-side at 0 50%, rgba(0,0,0,0.12), transparent),
      radial-gradient(farthest-side at 100% 50%, rgba(0,0,0,0.12), transparent) 100% 0;
    background-repeat: no-repeat;
    background-size: 40px 100%, 40px 100%, 14px 100%, 14px 100%;
    background-attachment: local, local, scroll, scroll;
  }

  .trades-table {
    width: 100%;
    min-width: 950px;
    border-collapse: collapse;
    table-layout: auto;
  }

  /* Header */
  thead {
    background: var(--color-bg-subtle);
  }

  th {
    padding: var(--space-4) var(--space-3);
    font-size: var(--text-xs);
    font-weight: var(--font-bold);
    text-transform: uppercase;
    letter-spacing: var(--tracking-wider);
    color: var(--color-text-tertiary);
    text-align: left;
    white-space: nowrap;
    position: sticky;
    top: 0;
    background: var(--color-bg-subtle);
  }

  th:first-child {
    border-radius: var(--radius-md) 0 0 var(--radius-md);
  }

  th:last-child {
    border-radius: 0 var(--radius-md) var(--radius-md) 0;
  }

  th.numeric {
    text-align: right;
  }

  /* Body */
  td {
    padding: var(--space-4) var(--space-3);
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    border-bottom: 1px solid var(--color-border-subtle);
    vertical-align: middle;
  }

  td.numeric {
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  tbody tr:hover {
    background: var(--color-bg-card-hover);
  }

  tbody tr.active {
    background: var(--color-active-bg);
  }

  tbody tr.active:hover {
    background: #fef0c7;
  }

  /* Ticker Cell */
  .ticker-cell {
    font-weight: var(--font-bold);
    color: var(--color-brand-primary);
    font-size: var(--text-base);
    font-family: var(--font-display);
  }

  .ticker-btn {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    color: inherit;
    cursor: pointer;
    font-weight: var(--font-bold);
  }

  .ticker-btn:hover {
    color: var(--color-brand-primary-hover);
  }

  .ticker-btn:focus-visible {
    outline: 2px solid var(--color-brand-primary);
    outline-offset: 2px;
    border-radius: var(--radius-sm);
  }

  .expand-icon {
    display: inline-flex;
    transition: transform var(--duration-fast) var(--ease-out);
  }

  .expand-icon.expanded {
    transform: rotate(90deg);
  }

  /* Profit/Loss */
  .profit {
    color: var(--color-profit);
    font-weight: var(--font-semibold);
  }

  .loss {
    color: var(--color-loss);
    font-weight: var(--font-semibold);
  }

  /* Badges */
  .setup-badge {
    display: inline-block;
    background: var(--color-bg-muted);
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-secondary);
  }

  .result-badge {
    display: inline-block;
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    font-weight: var(--font-bold);
    text-transform: uppercase;
    letter-spacing: var(--tracking-wide);
  }

  .result--win {
    background: var(--color-profit-bg);
    color: var(--color-profit);
    border: 1px solid var(--color-profit-border);
  }

  .result--loss {
    background: var(--color-loss-bg);
    color: var(--color-loss);
    border: 1px solid var(--color-loss-border);
  }

  .result--active {
    background: var(--color-active-bg);
    color: var(--color-active-text);
    border: 1px solid var(--color-active-border);
  }

  /* Actions */
  .actions-cell {
    white-space: nowrap;
  }

  .close-trade-btn {
    background: var(--color-brand-secondary);
    color: white;
    border: none;
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-sm);
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    cursor: pointer;
    transition: var(--transition-colors);
  }

  .close-trade-btn:hover {
    background: var(--color-brand-secondary-hover);
  }

  .close-trade-btn:focus-visible {
    outline: 2px solid var(--color-brand-secondary);
    outline-offset: 2px;
  }

  .closed-indicator {
    color: var(--color-text-muted);
    font-size: var(--text-xs);
  }

  /* Notes Row */
  .notes-row {
    display: none;
  }

  .notes-row.expanded {
    display: table-row;
  }

  .notes-row td {
    padding: 0 var(--space-3) var(--space-4);
    border-bottom: 2px solid var(--color-border-default);
  }

  .notes-content {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    line-height: 1.6;
    padding: var(--space-3) var(--space-4);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
    margin-left: var(--space-6);
  }

  .notes-content strong {
    color: var(--color-text-secondary);
  }
</style>

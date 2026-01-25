<script lang="ts">
  /**
   * ═══════════════════════════════════════════════════════════════════════════════
   * TableSkeleton Component - Loading State
   * ═══════════════════════════════════════════════════════════════════════════════
   * 
   * @version 1.0.0
   * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
   */
  interface Props {
    rows?: number;
    hasActions?: boolean;
  }

  const { rows = 5, hasActions = false }: Props = $props();
</script>

<div class="table-container skeleton-container">
  <table class="trades-table" aria-hidden="true">
    <thead>
      <tr>
        <th>Ticker</th>
        <th>Entry</th>
        <th>Exit</th>
        <th>Entry $</th>
        <th>Exit $</th>
        <th>Profit</th>
        <th>%</th>
        <th>Days</th>
        <th>Setup</th>
        <th>Result</th>
        {#if hasActions}
          <th>Actions</th>
        {/if}
      </tr>
    </thead>
    <tbody>
      {#each Array(rows) as _, i}
        <tr style="animation-delay: {i * 0.05}s">
          <td><div class="skeleton-cell wide"></div></td>
          <td><div class="skeleton-cell"></div></td>
          <td><div class="skeleton-cell"></div></td>
          <td><div class="skeleton-cell narrow"></div></td>
          <td><div class="skeleton-cell narrow"></div></td>
          <td><div class="skeleton-cell narrow"></div></td>
          <td><div class="skeleton-cell tiny"></div></td>
          <td><div class="skeleton-cell tiny"></div></td>
          <td><div class="skeleton-cell badge"></div></td>
          <td><div class="skeleton-cell badge"></div></td>
          {#if hasActions}
            <td><div class="skeleton-cell button"></div></td>
          {/if}
        </tr>
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
  }

  .trades-table {
    width: 100%;
    min-width: 950px;
    border-collapse: collapse;
  }

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
  }

  th:first-child {
    border-radius: var(--radius-md) 0 0 var(--radius-md);
  }

  th:last-child {
    border-radius: 0 var(--radius-md) var(--radius-md) 0;
  }

  td {
    padding: var(--space-4) var(--space-3);
    border-bottom: 1px solid var(--color-border-subtle);
  }

  tbody tr {
    animation: fadeIn 0.3s ease-out forwards;
    opacity: 0;
  }

  @keyframes fadeIn {
    to { opacity: 1; }
  }

  .skeleton-cell {
    height: 16px;
    background: linear-gradient(
      90deg,
      var(--color-bg-subtle) 25%,
      var(--color-bg-muted) 50%,
      var(--color-bg-subtle) 75%
    );
    background-size: 200% 100%;
    animation: shimmer 1.5s infinite;
    border-radius: var(--radius-sm);
  }

  .skeleton-cell.wide {
    width: 60px;
  }

  .skeleton-cell.narrow {
    width: 50px;
  }

  .skeleton-cell.tiny {
    width: 30px;
  }

  .skeleton-cell.badge {
    width: 70px;
    height: 24px;
    border-radius: var(--radius-sm);
  }

  .skeleton-cell.button {
    width: 60px;
    height: 28px;
    border-radius: var(--radius-sm);
  }

  @keyframes shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
</style>

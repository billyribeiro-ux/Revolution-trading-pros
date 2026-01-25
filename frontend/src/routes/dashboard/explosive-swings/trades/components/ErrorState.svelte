<script lang="ts">
  /**
   * ═══════════════════════════════════════════════════════════════════════════════
   * ErrorState Component - API Error Display
   * ═══════════════════════════════════════════════════════════════════════════════
   * 
   * @version 1.0.0
   * @standards Apple Principal Engineer ICT 7+ | WCAG 2.1 AA
   */
  interface Props {
    message: string;
    onRetry?: () => void;
  }

  const { message, onRetry }: Props = $props();
</script>

<div class="error-state" role="alert">
  <div class="error-icon">
    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" width="48" height="48">
      <circle cx="12" cy="12" r="10" />
      <path d="M12 8v4m0 4h.01" />
    </svg>
  </div>
  <h3 class="error-title">Unable to load trades</h3>
  <p class="error-message">{message}</p>
  {#if onRetry}
    <button type="button" class="retry-btn" onclick={onRetry}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
        <path d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
      </svg>
      Try Again
    </button>
  {/if}
</div>

<style>
  .error-state {
    text-align: center;
    padding: var(--space-12) var(--space-6);
  }

  .error-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 80px;
    height: 80px;
    background: var(--color-loss-bg);
    border-radius: var(--radius-xl);
    margin-bottom: var(--space-5);
    color: var(--color-loss);
  }

  .error-title {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin: 0 0 var(--space-2) 0;
  }

  .error-message {
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
    margin: 0 0 var(--space-5) 0;
  }

  .retry-btn {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    background: var(--color-loss);
    color: white;
    border: none;
    padding: var(--space-2) var(--space-5);
    border-radius: var(--radius-md);
    font-size: var(--text-sm);
    font-weight: var(--font-semibold);
    cursor: pointer;
    transition: var(--transition-colors);
  }

  .retry-btn:hover {
    background: var(--color-loss-light);
  }

  .retry-btn:focus-visible {
    outline: 2px solid var(--color-loss);
    outline-offset: 2px;
  }
</style>

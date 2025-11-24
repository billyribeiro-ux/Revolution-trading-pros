<script lang="ts">
  import { themeStore } from '$lib/stores/theme';

  function toggleTheme() {
    themeStore.toggle();
  }

  $: isDark = $themeStore === 'dark';
</script>

<button
  class="theme-toggle"
  on:click={toggleTheme}
  aria-label="Toggle theme"
  title={isDark ? 'Switch to light mode' : 'Switch to dark mode'}
>
  {#if isDark}
    <!-- Sun icon -->
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="12" cy="12" r="5"/>
      <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
    </svg>
  {:else}
    <!-- Moon icon -->
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
    </svg>
  {/if}
</button>

<style>
  .theme-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 8px;
    border: 1px solid #e5e7eb;
    background: white;
    color: #6b7280;
    cursor: pointer;
    transition: all 0.2s;
  }

  .theme-toggle:hover {
    background: #f9fafb;
    border-color: #d1d5db;
    color: #1f2937;
  }

  :global(.dark) .theme-toggle {
    background: #1f2937;
    border-color: #374151;
    color: #9ca3af;
  }

  :global(.dark) .theme-toggle:hover {
    background: #111827;
    border-color: #4b5563;
    color: #f3f4f6;
  }
</style>

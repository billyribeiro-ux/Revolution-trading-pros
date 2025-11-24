<script lang="ts">
  interface Props {
    size?: 'sm' | 'md' | 'lg';
    message?: string;
  }

  let { size = 'md', message }: Props = $props();

  let sizeClass = $derived({
    sm: 'h-6 w-6',
    md: 'h-10 w-10',
    lg: 'h-16 w-16'
  }[size]);
</script>

<div class="flex flex-col items-center justify-center gap-3 py-8">
  <div class="{sizeClass} animate-spin rounded-full border-4 border-slate-700 border-t-indigo-400"></div>
  {#if message}
    <p class="text-sm text-slate-400">{message}</p>
  {/if}
</div>

<style>
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>

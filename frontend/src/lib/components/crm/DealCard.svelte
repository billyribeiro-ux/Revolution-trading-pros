<script lang="ts">
  import type { Deal } from '$lib/crm/types';
  import { IconCurrencyDollar, IconClock, IconTrendingUp, IconAlertCircle } from '@tabler/icons-svelte';

  interface Props {
    deal: Deal;
    onclick?: () => void;
  }

  let { deal, onclick }: Props = $props();

  function formatCurrency(value: number): string {
    if (value >= 1_000_000) return `$${(value / 1_000_000).toFixed(1)}M`;
    if (value >= 1_000) return `$${(value / 1_000).toFixed(1)}k`;
    return `$${value.toFixed(0)}`;
  }

  function getProbabilityColor(prob: number): string {
    if (prob >= 75) return 'text-emerald-400';
    if (prob >= 50) return 'text-sky-400';
    if (prob >= 25) return 'text-amber-400';
    return 'text-slate-400';
  }

  function getPriorityColor(priority: string): string {
    switch (priority) {
      case 'urgent': return 'bg-rose-500/10 text-rose-300 border-rose-500/30';
      case 'high': return 'bg-amber-500/10 text-amber-300 border-amber-500/30';
      case 'normal': return 'bg-sky-500/10 text-sky-300 border-sky-500/30';
      default: return 'bg-slate-500/10 text-slate-300 border-slate-500/30';
    }
  }

  let isStale = $derived(deal.days_in_stage > 30);
</script>

<button
  type="button"
  class="group w-full rounded-xl border border-slate-800/80 bg-slate-900/90 p-3 text-left transition-all hover:border-sky-500/70 hover:bg-slate-900 hover:shadow-md focus:outline-none focus:ring-2 focus:ring-sky-500/60"
  onclick={onclick}
>
  <div class="mb-2 flex items-start justify-between gap-2">
    <h4 class="flex-1 text-[13px] font-medium text-slate-100 group-hover:text-sky-300 transition-colors">
      {deal.name}
    </h4>
    {#if deal.priority !== 'normal'}
      <span class="rounded-full border px-1.5 py-0.5 text-[9px] font-medium {getPriorityColor(deal.priority)}">
        {deal.priority}
      </span>
    {/if}
  </div>

  <p class="mb-2 text-[11px] text-slate-400 truncate">
    {deal.contact?.full_name ?? 'Unassigned contact'}
  </p>

  <div class="mb-2 flex items-center justify-between text-[11px]">
    <div class="flex items-center gap-1 text-slate-200">
      <IconCurrencyDollar size={14} />
      <span class="font-semibold">{formatCurrency(deal.amount)}</span>
    </div>
    <div class="flex items-center gap-1 {getProbabilityColor(deal.probability)}">
      <IconTrendingUp size={12} />
      <span class="font-medium">{deal.probability}%</span>
    </div>
  </div>

  <div class="flex items-center gap-2 text-[10px] text-slate-500">
    <div class="flex items-center gap-1">
      <IconClock size={11} />
      <span>{deal.days_in_stage}d in stage</span>
    </div>
    {#if isStale}
      <div class="flex items-center gap-1 text-amber-400">
        <IconAlertCircle size={11} />
        <span>Stale</span>
      </div>
    {/if}
  </div>
</button>

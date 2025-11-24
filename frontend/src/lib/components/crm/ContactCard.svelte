<script lang="ts">
	import type { Contact } from '$lib/crm/types';
	import {
		IconMail,
		IconPhone,
		IconTrendingUp,
		IconAlertTriangle,
		IconStar
	} from '@tabler/icons-svelte';

	interface Props {
		contact: Contact;
		onclick?: () => void;
	}

	let { contact, onclick }: Props = $props();

	function getScoreColor(score: number): string {
		if (score >= 75) return 'text-emerald-400';
		if (score >= 50) return 'text-sky-400';
		if (score >= 25) return 'text-amber-400';
		return 'text-rose-400';
	}

	function getHealthBadge(score: number): { color: string; label: string } {
		if (score >= 75)
			return {
				color: 'bg-emerald-500/10 text-emerald-300 border-emerald-500/30',
				label: 'Healthy'
			};
		if (score >= 50)
			return { color: 'bg-sky-500/10 text-sky-300 border-sky-500/30', label: 'Good' };
		if (score >= 25)
			return { color: 'bg-amber-500/10 text-amber-300 border-amber-500/30', label: 'At Risk' };
		return { color: 'bg-rose-500/10 text-rose-300 border-rose-500/30', label: 'Critical' };
	}

	let healthBadge = $derived(getHealthBadge(contact.health_score));
</script>

<button
	type="button"
	class="group w-full rounded-2xl border border-slate-800/80 bg-slate-900/70 p-4 text-left transition-all hover:border-indigo-500/50 hover:bg-slate-900/90 hover:shadow-lg hover:shadow-indigo-500/10 focus:outline-none focus:ring-2 focus:ring-indigo-500/60"
	{onclick}
>
	<div class="flex items-start justify-between gap-3">
		<div class="flex-1 min-w-0">
			<div class="flex items-center gap-2 mb-1">
				<h3
					class="text-base font-semibold text-slate-50 truncate group-hover:text-indigo-300 transition-colors"
				>
					{contact.full_name}
				</h3>
				{#if contact.is_vip}
					<IconStar size={14} class="text-amber-400 fill-amber-400 flex-shrink-0" />
				{/if}
			</div>

			<div class="flex items-center gap-2 text-xs text-slate-400 mb-2">
				<IconMail size={12} />
				<span class="truncate">{contact.email}</span>
			</div>

			{#if contact.phone}
				<div class="flex items-center gap-2 text-xs text-slate-400 mb-2">
					<IconPhone size={12} />
					<span>{contact.phone}</span>
				</div>
			{/if}

			{#if contact.job_title}
				<p class="text-xs text-slate-500 truncate">{contact.job_title}</p>
			{/if}
		</div>

		<div class="flex flex-col items-end gap-2 flex-shrink-0">
			<div class="flex items-center gap-1.5">
				<IconTrendingUp size={14} class={getScoreColor(contact.lead_score)} />
				<span class="text-sm font-semibold {getScoreColor(contact.lead_score)}"
					>{contact.lead_score}</span
				>
			</div>

			{#if contact.status === 'customer'}
				<span class="rounded-full border px-2 py-0.5 text-[10px] font-medium {healthBadge.color}">
					{healthBadge.label}
				</span>
			{/if}
		</div>
	</div>

	<div class="mt-3 flex flex-wrap items-center gap-2 text-[10px]">
		<span class="rounded-full bg-slate-800/80 px-2 py-1 text-slate-300">
			{contact.status}
		</span>
		<span class="rounded-full bg-slate-800/80 px-2 py-1 text-slate-300">
			{contact.lifecycle_stage}
		</span>
		{#if contact.subscription_status !== 'none'}
			<span class="rounded-full bg-emerald-500/10 px-2 py-1 text-emerald-300">
				{contact.subscription_status} · ${contact.subscription_mrr.toFixed(0)}/mo
			</span>
		{/if}
	</div>

	{#if contact.friction_events_count > 5}
		<div class="mt-2 flex items-center gap-1.5 text-[11px] text-rose-300">
			<IconAlertTriangle size={12} />
			<span>High friction detected</span>
		</div>
	{/if}
</button>

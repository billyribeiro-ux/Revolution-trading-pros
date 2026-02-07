<script lang="ts">
	import { Lightbulb, TrendingUp, TrendingDown, X } from '@lucide/svelte';
	import gsap from 'gsap';
	import type { EducationEntry } from '../../engine/types.js';

	interface Props {
		entry: EducationEntry;
		visible: boolean;
		onClose: () => void;
	}

	let { entry, visible, onClose }: Props = $props();

	let cardEl: HTMLDivElement | undefined = $state();
	let isExpanded = $state(false);

	$effect(() => {
		if (visible && cardEl) {
			isExpanded = false;
			gsap.fromTo(
				cardEl,
				{ scale: 0.85, opacity: 0, y: 8 },
				{ scale: 1, opacity: 1, y: 0, duration: 0.3, ease: 'back.out(2)' },
			);
		}
	});

	function handleClose() {
		if (!cardEl) {
			onClose();
			return;
		}
		gsap.to(cardEl, {
			scale: 0.85,
			opacity: 0,
			y: 8,
			duration: 0.15,
			ease: 'power2.in',
			onComplete: onClose,
		});
	}

	let truncatedExplanation = $derived(
		entry.fullExplanation.length > 200 && !isExpanded
			? entry.fullExplanation.slice(0, 200) + '...'
			: entry.fullExplanation,
	);
</script>

{#if visible}
	<div
		bind:this={cardEl}
		class="w-[360px] max-w-[90vw] rounded-2xl p-4 flex flex-col gap-3"
		style="
			background: var(--calc-surface);
			border: 1px solid var(--calc-border);
			box-shadow: 0 16px 48px rgba(0,0,0,0.4);
			backdrop-filter: blur(20px);
		"
		role="tooltip"
	>
		<!-- Header -->
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				{#if entry.symbol}
					<span
						class="text-xs font-bold px-2 py-0.5 rounded-md"
						style="
							background: var(--calc-accent-glow);
							color: var(--calc-accent);
							font-family: var(--calc-font-mono);
						"
					>{entry.symbol}</span>
				{/if}
				<h4
					class="text-sm font-semibold"
					style="color: var(--calc-text); font-family: var(--calc-font-display);"
				>
					{entry.term}
				</h4>
			</div>
			<button
				onclick={handleClose}
				class="cursor-pointer rounded-md p-0.5"
				style="color: var(--calc-text-muted);"
				aria-label="Close tooltip"
			>
				<X size={14} />
			</button>
		</div>

		<!-- Short description -->
		<p class="text-[11px] font-medium leading-relaxed" style="color: var(--calc-text-secondary);">
			{entry.shortDescription}
		</p>

		<!-- Full explanation (expandable) -->
		<div class="text-[10px] leading-relaxed" style="color: var(--calc-text-muted);">
			<p>{truncatedExplanation}</p>
			{#if entry.fullExplanation.length > 200 && !isExpanded}
				<button
					onclick={() => (isExpanded = true)}
					class="text-[10px] font-medium cursor-pointer mt-1"
					style="color: var(--calc-accent);"
				>
					Read more
				</button>
			{/if}
		</div>

		<!-- When increases / decreases -->
		{#if entry.whenIncreases || entry.whenDecreases}
			<div class="flex flex-col gap-1.5">
				{#if entry.whenIncreases}
					<div class="flex items-start gap-1.5">
						<TrendingUp size={11} style="color: var(--calc-call); flex-shrink: 0; margin-top: 1px;" />
						<span class="text-[10px] leading-relaxed" style="color: var(--calc-text-muted);">
							{entry.whenIncreases}
						</span>
					</div>
				{/if}
				{#if entry.whenDecreases}
					<div class="flex items-start gap-1.5">
						<TrendingDown size={11} style="color: var(--calc-put); flex-shrink: 0; margin-top: 1px;" />
						<span class="text-[10px] leading-relaxed" style="color: var(--calc-text-muted);">
							{entry.whenDecreases}
						</span>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Pro Tip -->
		<div
			class="flex items-start gap-2 rounded-lg px-3 py-2"
			style="background: rgba(245,158,11,0.06); border: 1px solid rgba(245,158,11,0.12);"
		>
			<Lightbulb size={12} style="color: #f59e0b; flex-shrink: 0; margin-top: 1px;" />
			<p class="text-[10px] leading-relaxed" style="color: var(--calc-text-secondary);">
				<span class="font-semibold" style="color: #f59e0b;">Pro Tip:</span>
				{entry.proTip}
			</p>
		</div>

		<!-- Related terms -->
		{#if entry.relatedTerms && entry.relatedTerms.length > 0}
			<div class="flex items-center gap-1.5 flex-wrap">
				<span class="text-[9px] uppercase tracking-wider font-medium" style="color: var(--calc-text-muted);">
					Related:
				</span>
				{#each entry.relatedTerms as termId (termId)}
					<span
						class="text-[9px] px-1.5 py-0.5 rounded"
						style="background: var(--calc-surface-hover); color: var(--calc-text-muted); border: 1px solid var(--calc-border);"
					>
						{termId}
					</span>
				{/each}
			</div>
		{/if}
	</div>
{/if}

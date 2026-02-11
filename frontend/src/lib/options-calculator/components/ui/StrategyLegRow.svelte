<script lang="ts">
	import { X, Plus, Minus } from '@lucide/svelte';
	import gsap from 'gsap';
	import { priceOption } from '../../engine/black-scholes.js';
	import { firstOrderGreeks } from '../../engine/greeks.js';
	import { formatCurrency } from '../../utils/formatters.js';
	import type { StrategyLeg, OptionType, BSInputs, FirstOrderGreeks } from '../../engine/types.js';

	interface Props {
		leg: StrategyLeg;
		spotPrice: number;
		volatility: number;
		riskFreeRate: number;
		dividendYield: number;
		color: string;
		onUpdate: (leg: StrategyLeg) => void;
		onRemove: (id: string) => void;
	}

	let {
		leg,
		spotPrice,
		volatility,
		riskFreeRate,
		dividendYield,
		color,
		onUpdate,
		onRemove
	}: Props = $props();

	let rowEl: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (rowEl) {
			gsap.fromTo(
				rowEl,
				{ x: 40, opacity: 0 },
				{ x: 0, opacity: 1, duration: 0.35, ease: 'power2.out' }
			);
		}
	});

	let legInputs = $derived<BSInputs>({
		spotPrice,
		strikePrice: leg.strike,
		volatility,
		timeToExpiry: leg.expiry,
		riskFreeRate,
		dividendYield
	});

	let computedPrice = $derived(priceOption(legInputs, leg.type));
	let computedGreeks = $derived<FirstOrderGreeks>(firstOrderGreeks(legInputs, leg.type));

	function updateType(type: OptionType) {
		onUpdate({ ...leg, type });
	}

	function updateStrike(delta: number) {
		onUpdate({ ...leg, strike: Math.max(0.01, leg.strike + delta) });
	}

	function updateStrikeValue(val: string) {
		const n = parseFloat(val);
		if (!isNaN(n) && n > 0) onUpdate({ ...leg, strike: n });
	}

	function updateExpiry(val: string) {
		const days = parseFloat(val);
		if (!isNaN(days) && days > 0) onUpdate({ ...leg, expiry: days / 365 });
	}

	function togglePosition() {
		onUpdate({ ...leg, position: leg.position === 1 ? -1 : 1 });
	}

	function updateQuantity(delta: number) {
		const newQty = Math.max(1, Math.min(100, leg.quantity + delta));
		onUpdate({
			...leg,
			quantity: newQty,
			premium: -computedPrice * newQty * (leg.position === 1 ? 1 : -1)
		});
	}

	function handleRemove() {
		if (rowEl) {
			gsap.to(rowEl, {
				x: 60,
				opacity: 0,
				height: 0,
				padding: 0,
				margin: 0,
				duration: 0.3,
				ease: 'power2.in',
				onComplete: () => onRemove(leg.id)
			});
		} else {
			onRemove(leg.id);
		}
	}
</script>

<div
	bind:this={rowEl}
	class="flex items-center gap-2 rounded-xl px-3 py-2.5 transition-all"
	style="background: var(--calc-surface); border: 1px solid var(--calc-border); border-left: 3px solid {color};"
>
	<!-- Call/Put Toggle -->
	<div class="flex rounded-lg overflow-hidden" style="border: 1px solid var(--calc-border);">
		<button
			onclick={() => updateType('call')}
			class="text-[10px] font-bold px-2 py-1 transition-colors cursor-pointer"
			style={leg.type === 'call'
				? 'background: var(--calc-call-bg); color: var(--calc-call);'
				: 'color: var(--calc-text-muted);'}>C</button
		>
		<button
			onclick={() => updateType('put')}
			class="text-[10px] font-bold px-2 py-1 transition-colors cursor-pointer"
			style={leg.type === 'put'
				? 'background: var(--calc-put-bg); color: var(--calc-put);'
				: 'color: var(--calc-text-muted);'}>P</button
		>
	</div>

	<!-- Strike -->
	<div class="flex items-center gap-0.5">
		<button
			onclick={() => updateStrike(-1)}
			class="w-5 h-5 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"><Minus size={10} /></button
		>
		<input
			type="number"
			value={leg.strike}
			oninput={(e: Event) => updateStrikeValue((e.currentTarget as HTMLInputElement).value)}
			class="w-14 text-center text-xs px-1 py-0.5 rounded outline-none"
			style="background: var(--calc-surface-hover); color: var(--calc-text); font-family: var(--calc-font-mono); border: 1px solid var(--calc-border);"
		/>
		<button
			onclick={() => updateStrike(1)}
			class="w-5 h-5 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"><Plus size={10} /></button
		>
	</div>

	<!-- DTE -->
	<input
		type="number"
		value={Math.round(leg.expiry * 365)}
		oninput={(e: Event) => updateExpiry((e.currentTarget as HTMLInputElement).value)}
		class="w-12 text-center text-xs px-1 py-0.5 rounded outline-none"
		style="background: var(--calc-surface-hover); color: var(--calc-text); font-family: var(--calc-font-mono); border: 1px solid var(--calc-border);"
	/>
	<span class="text-[9px]" style="color: var(--calc-text-muted);">DTE</span>

	<!-- Long/Short -->
	<button
		onclick={togglePosition}
		class="text-[10px] font-bold px-2 py-1 rounded-lg transition-colors cursor-pointer"
		style={leg.position === 1
			? 'background: rgba(0,212,170,0.15); color: #00d4aa; border: 1px solid rgba(0,212,170,0.3);'
			: 'background: rgba(255,68,119,0.15); color: #ff4477; border: 1px solid rgba(255,68,119,0.3);'}
	>
		{leg.position === 1 ? 'LONG' : 'SHORT'}
	</button>

	<!-- Quantity -->
	<div class="flex items-center gap-0.5">
		<button
			onclick={() => updateQuantity(-1)}
			class="w-4 h-4 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"><Minus size={8} /></button
		>
		<span
			class="text-xs w-5 text-center"
			style="color: var(--calc-text); font-family: var(--calc-font-mono);">{leg.quantity}</span
		>
		<button
			onclick={() => updateQuantity(1)}
			class="w-4 h-4 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"><Plus size={8} /></button
		>
	</div>

	<!-- Computed Price -->
	<span
		class="text-xs min-w-[3.5rem] text-right"
		style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
	>
		{formatCurrency(computedPrice)}
	</span>

	<!-- Delta -->
	<span
		class="text-[10px] min-w-[2.5rem] text-right"
		style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
	>
		Δ{computedGreeks.delta.toFixed(2)}
	</span>

	<!-- Remove -->
	<button
		onclick={handleRemove}
		class="w-5 h-5 flex items-center justify-center rounded-full transition-colors cursor-pointer ml-auto"
		style="color: var(--calc-text-muted);"
		aria-label="Remove leg"
	>
		<X size={12} />
	</button>
</div>

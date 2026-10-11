<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';
	import IconMinus from '@tabler/icons-svelte-runes/icons/minus';
	import { cubicIn, cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly, type TransitionConfig } from 'svelte/transition';
	import { priceOption } from '../../engine/black-scholes.js';
	import { firstOrderGreeks } from '../../engine/greeks.js';
	import { formatCurrency } from '../../utils/formatters.js';
	import type { StrategyLeg, OptionType, BSInputs } from '../../engine/types.js';

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

	const legInputs = $derived<BSInputs>({
		spotPrice,
		strikePrice: leg.strike,
		volatility,
		timeToExpiry: leg.expiry,
		riskFreeRate,
		dividendYield
	});

	const computedPrice = $derived(priceOption(legInputs, leg.type));
	const computedGreeks = $derived(firstOrderGreeks(legInputs, leg.type));
	const isLong = $derived(leg.position === 1);

	/** Only an explicit remove animates out; legs replaced wholesale (a preset) vanish at once. */
	let removing = false;

	function updateType(type: OptionType) {
		onUpdate({ ...leg, type });
	}

	function updateStrike(delta: number) {
		onUpdate({ ...leg, strike: Math.max(0.01, leg.strike + delta) });
	}

	function updateStrikeValue(strike: number) {
		if (Number.isFinite(strike) && strike > 0) onUpdate({ ...leg, strike });
	}

	function updateExpiry(days: number) {
		if (Number.isFinite(days) && days > 0) onUpdate({ ...leg, expiry: days / 365 });
	}

	function togglePosition() {
		onUpdate({ ...leg, position: isLong ? -1 : 1 });
	}

	function updateQuantity(delta: number) {
		const quantity = Math.max(1, Math.min(100, leg.quantity + delta));
		onUpdate({ ...leg, quantity });
	}

	function handleRemove() {
		removing = true;
		onRemove(leg.id);
	}

	/** Slides right and fades while the row's height and padding collapse to nothing. */
	function collapseOut(node: HTMLElement): TransitionConfig {
		if (!removing || prefersReducedMotion.current) return { duration: 0 };
		const style = getComputedStyle(node);
		const height = parseFloat(style.height);
		const top = parseFloat(style.paddingTop);
		const right = parseFloat(style.paddingRight);
		const bottom = parseFloat(style.paddingBottom);
		const left = parseFloat(style.paddingLeft);

		return {
			duration: 300,
			easing: cubicIn,
			css: (t, u) =>
				`overflow: hidden; transform: translateX(${u * 60}px); opacity: ${t}; height: ${t * height}px; padding: ${t * top}px ${t * right}px ${t * bottom}px ${t * left}px;`
		};
	}
</script>

<div
	in:fly|global={{ x: 40, duration: prefersReducedMotion.current ? 0 : 350, easing: cubicOut }}
	out:collapseOut
	class="flex items-center gap-2 rounded-xl px-3 py-2.5 transition-all"
	style:background="var(--calc-surface)"
	style:border="1px solid var(--calc-border)"
	style:border-left="3px solid {color}"
>
	<!-- Call/Put Toggle -->
	<div class="flex rounded-lg overflow-hidden" style="border: 1px solid var(--calc-border);">
		<button
			type="button"
			onclick={() => updateType('call')}
			class="text-[10px] font-bold px-2 py-1 transition-colors cursor-pointer"
			style:background={leg.type === 'call' ? 'var(--calc-call-bg)' : undefined}
			style:color={leg.type === 'call' ? 'var(--calc-call)' : 'var(--calc-text-muted)'}
			aria-label="Call"
			aria-pressed={leg.type === 'call'}>C</button
		>
		<button
			type="button"
			onclick={() => updateType('put')}
			class="text-[10px] font-bold px-2 py-1 transition-colors cursor-pointer"
			style:background={leg.type === 'put' ? 'var(--calc-put-bg)' : undefined}
			style:color={leg.type === 'put' ? 'var(--calc-put)' : 'var(--calc-text-muted)'}
			aria-label="Put"
			aria-pressed={leg.type === 'put'}>P</button
		>
	</div>

	<!-- Strike -->
	<div class="flex items-center gap-0.5">
		<button
			type="button"
			onclick={() => updateStrike(-1)}
			class="w-5 h-5 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"
			aria-label="Decrease strike"><IconMinus size={10} aria-hidden="true" /></button
		>
		<input
			type="number"
			value={leg.strike}
			oninput={(event) => updateStrikeValue(event.currentTarget.valueAsNumber)}
			aria-label="Strike"
			class="w-14 text-center text-xs px-1 py-0.5 rounded outline-none"
			style="background: var(--calc-surface-hover); color: var(--calc-text); font-family: var(--calc-font-mono); border: 1px solid var(--calc-border);"
		/>
		<button
			type="button"
			onclick={() => updateStrike(1)}
			class="w-5 h-5 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"
			aria-label="Increase strike"><IconPlus size={10} aria-hidden="true" /></button
		>
	</div>

	<!-- DTE -->
	<input
		type="number"
		value={Math.round(leg.expiry * 365)}
		oninput={(event) => updateExpiry(event.currentTarget.valueAsNumber)}
		aria-label="DTE (days to expiry)"
		class="w-12 text-center text-xs px-1 py-0.5 rounded outline-none"
		style="background: var(--calc-surface-hover); color: var(--calc-text); font-family: var(--calc-font-mono); border: 1px solid var(--calc-border);"
	/>
	<span class="text-[9px]" style="color: var(--calc-text-muted);" aria-hidden="true">DTE</span>

	<!-- Long/Short -->
	<button
		type="button"
		onclick={togglePosition}
		class="text-[10px] font-bold px-2 py-1 rounded-lg transition-colors cursor-pointer"
		style:background={isLong ? 'rgba(0,212,170,0.15)' : 'rgba(255,68,119,0.15)'}
		style:color={isLong ? '#00d4aa' : '#ff4477'}
		style:border="1px solid {isLong ? 'rgba(0,212,170,0.3)' : 'rgba(255,68,119,0.3)'}"
	>
		{isLong ? 'LONG' : 'SHORT'}
	</button>

	<!-- Quantity -->
	<div class="flex items-center gap-0.5">
		<button
			type="button"
			onclick={() => updateQuantity(-1)}
			class="w-4 h-4 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"
			aria-label="Decrease quantity"><IconMinus size={8} aria-hidden="true" /></button
		>
		<span
			class="text-xs w-5 text-center"
			style="color: var(--calc-text); font-family: var(--calc-font-mono);">{leg.quantity}</span
		>
		<button
			type="button"
			onclick={() => updateQuantity(1)}
			class="w-4 h-4 flex items-center justify-center rounded text-xs cursor-pointer"
			style="color: var(--calc-text-muted);"
			aria-label="Increase quantity"><IconPlus size={8} aria-hidden="true" /></button
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
		type="button"
		onclick={handleRemove}
		class="w-5 h-5 flex items-center justify-center rounded-full transition-colors cursor-pointer ml-auto"
		style="color: var(--calc-text-muted);"
		aria-label="Remove leg"
	>
		<IconX size={12} aria-hidden="true" />
	</button>
</div>

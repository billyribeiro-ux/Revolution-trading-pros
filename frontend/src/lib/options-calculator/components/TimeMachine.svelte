<script lang="ts">
	import IconPlayerPlay from '@tabler/icons-svelte-runes/icons/player-play';
	import IconPlayerPause from '@tabler/icons-svelte-runes/icons/player-pause';
	import { formatCurrency } from '../utils/formatters.js';
	import { getCalculator } from '../state/context.js';

	const calc = getCalculator();

	const SPEEDS = [1, 2, 5, 10];
	/** Days-to-expiry milestones marked on the timeline. */
	const KEY_DTES = [30, 14, 7, 1];
	/** Playback advances `speed` days every tick. */
	const TICK_MS = 50;

	let isPlaying = $state(false);
	let speed = $state(1);

	/** One playback step: advance by `speed` days, stopping at expiration. */
	function tick() {
		if (calc.timeMachineDay >= calc.timeMachineMaxDays) {
			isPlaying = false;
			return;
		}
		calc.timeMachineDay = Math.min(calc.timeMachineDay + speed, calc.timeMachineMaxDays);
	}

	// Playback timer: it exists only while playing, and pausing or unmounting runs
	// the cleanup. `tick` reads `speed` and the day when it fires (outside the
	// effect), so the effect depends on `isPlaying` alone.
	$effect(() => {
		if (!isPlaying) return;
		const timer = setInterval(tick, TICK_MS);
		return () => clearInterval(timer);
	});

	function togglePlay() {
		// Replay from the start once the timeline has reached expiration.
		if (!isPlaying && calc.timeMachineDay >= calc.timeMachineMaxDays) {
			calc.timeMachineDay = 0;
		}
		isPlaying = !isPlaying;
	}

	function cycleSpeed() {
		speed = SPEEDS[(SPEEDS.indexOf(speed) + 1) % SPEEDS.length];
	}

	/** Scrubbing the slider takes over from playback. */
	function scrubTo(day: number) {
		calc.timeMachineDay = day;
		isPlaying = false;
	}

	const remainingDTE = $derived(Math.max(0, calc.timeMachineMaxDays - calc.timeMachineDay));

	const snapshot = $derived(calc.currentTimeMachineSnapshot);

	const sliderPercent = $derived(
		calc.timeMachineMaxDays > 0 ? (calc.timeMachineDay / calc.timeMachineMaxDays) * 100 : 0
	);

	const markers = $derived.by(() => {
		const max = calc.timeMachineMaxDays;
		return KEY_DTES.filter((dte) => dte < max).map((dte) => ({
			dte,
			position: ((max - dte) / max) * 100
		}));
	});
</script>

<div class="flex flex-col gap-2">
	<!-- Header row -->
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-2">
			<button
				type="button"
				onclick={togglePlay}
				class="flex items-center justify-center w-7 h-7 rounded-lg transition-colors motion-reduce:transition-none cursor-pointer"
				style="background: var(--calc-accent); color: white;"
				aria-label={isPlaying ? 'Pause' : 'Play'}
			>
				{#if isPlaying}
					<IconPlayerPause size={14} aria-hidden="true" />
				{:else}
					<IconPlayerPlay size={14} aria-hidden="true" />
				{/if}
			</button>

			<button
				type="button"
				onclick={cycleSpeed}
				aria-label="Playback speed {speed}x"
				class="text-[10px] font-bold px-2 py-1 rounded-lg cursor-pointer"
				style="background: var(--calc-surface); color: var(--calc-accent); border: 1px solid var(--calc-border); font-family: var(--calc-font-mono);"
			>
				{speed}x
			</button>

			<span
				class="text-xs font-semibold"
				style="color: var(--calc-text); font-family: var(--calc-font-mono);"
			>
				Day {calc.timeMachineDay}
			</span>
			<span class="text-[10px]" style="color: var(--calc-text-muted);">
				({remainingDTE} DTE remaining)
			</span>
		</div>

		{#if snapshot}
			<div class="flex items-center gap-3">
				<div class="flex items-center gap-1">
					<span class="text-[10px]" style="color: var(--calc-text-muted);">Call:</span>
					<span
						class="text-xs font-semibold"
						style="color: var(--calc-call); font-family: var(--calc-font-mono);"
					>
						{formatCurrency(snapshot.callPrice)}
					</span>
				</div>
				<div class="flex items-center gap-1">
					<span class="text-[10px]" style="color: var(--calc-text-muted);">Put:</span>
					<span
						class="text-xs font-semibold"
						style="color: var(--calc-put); font-family: var(--calc-font-mono);"
					>
						{formatCurrency(snapshot.putPrice)}
					</span>
				</div>
				<div class="flex items-center gap-1">
					<span class="text-[10px]" style="color: var(--calc-text-muted);">Δ:</span>
					<span
						class="text-xs"
						style="color: var(--calc-text-secondary); font-family: var(--calc-font-mono);"
					>
						{snapshot.greeks.first.delta.toFixed(3)}
					</span>
				</div>
			</div>
		{/if}
	</div>

	<!-- Slider -->
	<div class="relative w-full h-8">
		<!-- Track background -->
		<div
			class="absolute top-3 left-0 right-0 h-2 rounded-full"
			style="background: var(--calc-surface-hover);"
			aria-hidden="true"
		>
			<!-- Filled track -->
			<div
				class="absolute top-0 left-0 h-full rounded-full transition-all motion-reduce:transition-none"
				style="background: linear-gradient(90deg, var(--calc-accent), var(--calc-warning));"
				style:width="{sliderPercent}%"
			></div>
		</div>

		<!-- Key DTE markers -->
		{#each markers as marker (marker.dte)}
			<div
				class="absolute top-0 flex flex-col items-center"
				style="transform: translateX(-50%);"
				style:left="{marker.position}%"
				aria-hidden="true"
			>
				<span
					class="text-[7px] font-medium"
					style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
				>
					{marker.dte}d
				</span>
				<div class="w-px h-2" style="background: var(--calc-border);"></div>
			</div>
		{/each}

		<!-- Native range input (invisible, on top for interaction) -->
		<input
			type="range"
			min="0"
			max={calc.timeMachineMaxDays}
			bind:value={() => calc.timeMachineDay, scrubTo}
			aria-label="Time machine day selector"
			aria-valuetext="Day {calc.timeMachineDay}, {remainingDTE} DTE remaining"
			class="time-slider absolute top-1.5 left-0 w-full h-5 opacity-0 cursor-pointer z-10"
		/>

		<!-- Custom thumb (shows the range input's keyboard focus) -->
		<div
			class="slider-thumb absolute top-1.5 w-4 h-4 rounded-full pointer-events-none"
			style="background: var(--calc-accent); box-shadow: 0 0 8px var(--calc-accent-glow); border: 2px solid var(--calc-bg);"
			style:left="calc({sliderPercent}% - 8px)"
			aria-hidden="true"
		></div>
	</div>

	<!-- Timeline labels -->
	<div class="flex items-center justify-between">
		<span
			class="text-[9px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);">Today</span
		>
		<span
			class="text-[9px]"
			style="color: var(--calc-text-muted); font-family: var(--calc-font-mono);"
			>Expiration ({calc.timeMachineMaxDays}d)</span
		>
	</div>
</div>

<style>
	.time-slider:focus-visible + .slider-thumb {
		outline: 2px solid var(--calc-accent);
		outline-offset: 2px;
	}
</style>

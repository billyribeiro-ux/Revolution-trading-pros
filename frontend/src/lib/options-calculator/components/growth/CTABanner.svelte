<script lang="ts">
	import IconX from '@tabler/icons-svelte-runes/icons/x';
	import IconArrowRight from '@tabler/icons-svelte-runes/icons/arrow-right';
	import { quadIn, quadOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly } from 'svelte/transition';

	const DISMISS_KEY = 'rtp:calc:cta-dismissed';
	const DISMISS_DAYS = 7;
	const DAY_MS = 24 * 60 * 60 * 1000;
	/** The banner waits this long before sliding in, so it never competes with first paint. */
	const REVEAL_DELAY_MS = 2000;

	/**
	 * Hidden until storage has been checked: the dismissal lives in localStorage, which the
	 * server cannot see, so the server renders no banner.
	 */
	let visible = $state(false);

	/** True when the visitor dismissed the banner within the last `DISMISS_DAYS` days. */
	function dismissedRecently(): boolean {
		try {
			const dismissedAt = Number(localStorage.getItem(DISMISS_KEY));
			return dismissedAt > 0 && Date.now() - dismissedAt < DISMISS_DAYS * DAY_MS;
		} catch {
			// Storage blocked (private mode, sandboxed embed): treat as never dismissed.
			return false;
		}
	}

	// Storage is I/O: read it once after mount, never during render.
	$effect(() => {
		visible = !dismissedRecently();
	});

	function handleDismiss(): void {
		visible = false;
		try {
			localStorage.setItem(DISMISS_KEY, Date.now().toString());
		} catch {
			// Not persisted; the banner simply returns on the next visit.
		}
	}
</script>

{#if visible}
	<div
		in:fly={{
			y: 20,
			delay: REVEAL_DELAY_MS,
			duration: prefersReducedMotion.current ? 0 : 400,
			easing: quadOut
		}}
		out:fly={{ y: 20, duration: prefersReducedMotion.current ? 0 : 200, easing: quadIn }}
		class="w-full rounded-xl px-4 py-3 flex items-center gap-3"
		style="
			background: linear-gradient(135deg, rgba(99,102,241,0.06), rgba(0,212,170,0.06));
			border: 1px solid rgba(99,102,241,0.15);
		"
		role="complementary"
		aria-label="Join Revolution Trading Pros"
	>
		<!-- Message -->
		<p class="flex-1 text-xs leading-relaxed" style="color: var(--calc-text-secondary);">
			Want live market data, saved strategies, and advanced analytics?
			<span class="font-semibold" style="color: var(--calc-text);">
				Join 18,000+ traders at Revolution Trading Pros.
			</span>
		</p>

		<!-- CTA Button -->
		<a
			href="https://revolutiontradingpros.com"
			target="_blank"
			rel="noopener"
			class="flex items-center gap-1 text-[10px] font-semibold px-3 py-1.5 rounded-lg shrink-0 transition-all duration-200"
			style="background: var(--calc-accent); color: white;"
		>
			Learn More
			<IconArrowRight size={10} aria-hidden="true" />
		</a>

		<!-- Dismiss -->
		<button
			type="button"
			onclick={handleDismiss}
			class="shrink-0 cursor-pointer p-1 rounded-md transition-colors"
			style="color: var(--calc-text-muted);"
			aria-label="Dismiss banner"
		>
			<IconX size={12} aria-hidden="true" />
		</button>
	</div>
{/if}

<script lang="ts">
	import { X, ArrowRight } from '@lucide/svelte';
	import gsap from 'gsap';

	let bannerEl: HTMLDivElement | undefined = $state();
	let dismissed = $state(false);

	const DISMISS_KEY = 'rtp:calc:cta-dismissed';
	const DISMISS_DAYS = 7;

	$effect(() => {
		if (typeof window === 'undefined') return;
		const raw = localStorage.getItem(DISMISS_KEY);
		if (raw) {
			const dismissedAt = parseInt(raw, 10);
			const daysSince = (Date.now() - dismissedAt) / (1000 * 60 * 60 * 24);
			if (daysSince < DISMISS_DAYS) {
				dismissed = true;
				return;
			}
		}
		// Animate in after a short delay
		if (bannerEl && !dismissed) {
			gsap.fromTo(
				bannerEl,
				{ y: 20, opacity: 0 },
				{ y: 0, opacity: 1, duration: 0.4, ease: 'power2.out', delay: 2 }
			);
		}
	});

	function handleDismiss(): void {
		if (bannerEl) {
			gsap.to(bannerEl, {
				y: 20,
				opacity: 0,
				duration: 0.2,
				ease: 'power2.in',
				onComplete: () => {
					dismissed = true;
					localStorage.setItem(DISMISS_KEY, Date.now().toString());
				}
			});
		} else {
			dismissed = true;
			localStorage.setItem(DISMISS_KEY, Date.now().toString());
		}
	}
</script>

{#if !dismissed}
	<div
		bind:this={bannerEl}
		class="w-full rounded-xl px-4 py-3 flex items-center gap-3 opacity-0"
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
			class="flex items-center gap-1 text-[10px] font-semibold px-3 py-1.5 rounded-lg flex-shrink-0 transition-all duration-200"
			style="background: var(--calc-accent); color: white;"
		>
			Learn More
			<ArrowRight size={10} />
		</a>

		<!-- Dismiss -->
		<button
			onclick={handleDismiss}
			class="flex-shrink-0 cursor-pointer p-1 rounded-md transition-colors"
			style="color: var(--calc-text-muted);"
			aria-label="Dismiss banner"
		>
			<X size={12} />
		</button>
	</div>
{/if}

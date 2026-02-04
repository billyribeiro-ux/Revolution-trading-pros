<script lang="ts">
	/**
	 * TestimonialCarousel - Trader Testimonials
	 * Apple Principal Engineer ICT 11 Grade
	 *
	 * Auto-rotating testimonial carousel using Embla
	 *
	 * ICT 11+ Patterns:
	 * - Embla carousel with autoplay plugin
	 * - Proper cleanup via onDestroy (prevents memory leaks)
	 * - Accessible navigation dots with ARIA labels
	 * - Touch-optimized for mobile devices
	 *
	 * @version 2.0.0 - ICT 11 Grade
	 */
	import emblaCarouselSvelte from 'embla-carousel-svelte';
	import Autoplay from 'embla-carousel-autoplay';
	import type { EmblaCarouselType } from 'embla-carousel';

	interface Testimonial {
		name: string;
		role: string;
		avatar: string;
		quote: string;
		profit?: string;
	}

	const testimonials: Testimonial[] = [
		{
			name: 'Michael Chen',
			role: 'Day Trader, 3 Years',
			avatar: 'MC',
			quote:
				'The live trading rooms completely transformed my approach. I went from losing consistently to profitable in 6 months.',
			profit: '+47% YTD'
		},
		{
			name: 'Sarah Williams',
			role: 'Options Trader',
			avatar: 'SW',
			quote:
				"The real-time alerts are incredible. I've caught moves I would have missed completely on my own.",
			profit: '+$32k this quarter'
		},
		{
			name: 'David Rodriguez',
			role: 'Swing Trader',
			avatar: 'DR',
			quote:
				'Finally, a community of serious traders. The education here is worth 10x what I paid.',
			profit: '8/10 winning trades'
		},
		{
			name: 'Jennifer Park',
			role: 'Part-time Trader',
			avatar: 'JP',
			quote:
				'As a working professional, the alerts let me trade around my schedule. Life-changing.',
			profit: '+$18k part-time'
		}
	];

	let emblaApi: EmblaCarouselType;
	let selectedIndex = $state(0);

	const plugins = [
		Autoplay({
			delay: 5000,
			stopOnInteraction: false,
			stopOnMouseEnter: true
		})
	];

	function onInit(event: CustomEvent<EmblaCarouselType>) {
		emblaApi = event.detail;
		emblaApi.on('select', () => {
			selectedIndex = emblaApi.selectedScrollSnap();
		});
	}

	function scrollTo(index: number) {
		emblaApi?.scrollTo(index);
	}

	$effect(() => {
		return () => {
			emblaApi?.destroy();
		};
	});
</script>

<div class="testimonial-section">
	<div
		class="carousel-viewport"
		use:emblaCarouselSvelte={{ plugins, options: { loop: true } }}
		onemblaInit={onInit}
	>
		<div class="carousel-container">
			{#each testimonials as testimonial}
				<div class="carousel-slide">
					<div class="testimonial-card">
						<!-- Quote Icon -->
						<div class="quote-icon" aria-hidden="true">
							<svg width="32" height="32" viewBox="0 0 24 24" fill="currentColor">
								<path d="M6 17h3l2-4V7H5v6h3zm8 0h3l2-4V7h-6v6h3z" />
							</svg>
						</div>

						<!-- Quote Text -->
						<blockquote class="testimonial-quote">
							"{testimonial.quote}"
						</blockquote>

						<!-- Author -->
						<div class="testimonial-author">
							<div class="author-avatar" aria-hidden="true">
								{testimonial.avatar}
							</div>
							<div class="author-info">
								<span class="author-name">{testimonial.name}</span>
								<span class="author-role">{testimonial.role}</span>
							</div>
							{#if testimonial.profit}
								<div class="author-profit">
									<span class="profit-value">{testimonial.profit}</span>
								</div>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	</div>

	<!-- Dots -->
	<div class="carousel-dots">
		{#each testimonials as _, i}
			<button
				type="button"
				class="carousel-dot"
				class:active={selectedIndex === i}
				onclick={() => scrollTo(i)}
				aria-label="Go to testimonial {i + 1}"
			></button>
		{/each}
	</div>
</div>

<style>
	.testimonial-section {
		width: 100%;
		max-width: 500px;
		margin: 0 auto;
	}

	/* Carousel Viewport */
	.carousel-viewport {
		overflow: hidden;
		border-radius: 16px;
	}

	.carousel-container {
		display: flex;
		touch-action: pan-y pinch-zoom;
	}

	.carousel-slide {
		flex: 0 0 100%;
		min-width: 0;
		padding: 0 0.5rem;
	}

	/* Testimonial Card */
	.testimonial-card {
		position: relative;
		padding: 1.5rem;
		background: var(--auth-card-bg, rgba(15, 23, 42, 0.6));
		backdrop-filter: blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-radius: 16px;
		border: 1px solid var(--auth-card-border, rgba(99, 102, 241, 0.15));
	}

	/* Quote Icon */
	.quote-icon {
		color: var(--auth-link, #818cf8);
		opacity: 0.5;
		margin-bottom: 0.75rem;
	}

	/* Quote Text */
	.testimonial-quote {
		font-family: var(--font-body);
		font-size: 1rem;
		font-style: italic;
		color: var(--auth-text, #e2e8f0);
		line-height: 1.6;
		margin: 0 0 1.25rem 0;
	}

	/* Author Section */
	.testimonial-author {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.author-avatar {
		width: 40px;
		height: 40px;
		border-radius: 50%;
		background: var(--auth-btn-primary-bg, linear-gradient(135deg, #6366f1, #8b5cf6));
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;
		font-family: var(--font-heading);
		font-size: 0.875rem;
		font-weight: 700;
		flex-shrink: 0;
	}

	.author-info {
		display: flex;
		flex-direction: column;
		flex: 1;
		min-width: 0;
	}

	.author-name {
		font-family: var(--font-heading);
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--auth-text, #e2e8f0);
	}

	.author-role {
		font-family: var(--font-body);
		font-size: 0.8125rem;
		color: var(--auth-muted, #64748b);
	}

	.author-profit {
		flex-shrink: 0;
	}

	.profit-value {
		font-family: var(--font-mono);
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--auth-bull, #22c55e);
		background: var(--auth-bull-soft, rgba(34, 197, 94, 0.15));
		padding: 0.25rem 0.625rem;
		border-radius: 6px;
	}

	/* Carousel Dots */
	.carousel-dots {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1rem;
	}

	.carousel-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--auth-card-border, rgba(99, 102, 241, 0.15));
		border: none;
		cursor: pointer;
		padding: 0;
		transition: all 0.3s ease;
	}

	.carousel-dot.active {
		width: 24px;
		border-radius: 4px;
		background: var(--auth-link, #818cf8);
	}

	.carousel-dot:hover:not(.active) {
		background: var(--auth-muted, #64748b);
	}

	/* Light Theme */
	:global(html.light) .testimonial-card,
	:global(body.light) .testimonial-card {
		background: rgba(255, 255, 255, 0.9);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
	}
</style>

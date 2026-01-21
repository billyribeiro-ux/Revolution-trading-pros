<script lang="ts">
	/**
	 * MarketingFooter - Apple Principal Engineer ICT Grade
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Svelte 5 / SvelteKit 2.x Best Practices (November/December 2025)
	 *
	 * PATTERNS IMPLEMENTED:
	 * ✅ $state() for reactive variables
	 * ✅ IntersectionObserver for viewport detection
	 * ✅ Conditional rendering with {#if isVisible}
	 * ✅ heavySlide transition matching MentorshipSection
	 * ✅ cubicOut easing for smooth animations
	 * ✅ Proper cleanup with observer.disconnect()
	 * ✅ SSR-safe browser check
	 *
	 * @version 2.0.0 - Svelte 5 Runes + MentorshipSection Pattern
	 * ═══════════════════════════════════════════════════════════════════════════
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { heavySlide, createVisibilityObserver } from '$lib/transitions';
	import {
		IconBrandTwitter,
		IconBrandInstagram,
		IconBrandYoutube,
		IconBrandFacebook
	} from '$lib/icons';

	// --- State (Svelte 5 Runes) ---
	let currentYear = $state(new Date().getFullYear());
	let containerRef = $state<HTMLElement | null>(null);
	// ICT11+ Fix: Start false, set true in onMount to trigger in: transitions
	let isVisible = $state(false);

	// Svelte 5 ICT7+ Pattern: Use centralized visibility observer
	onMount(() => {
		// Ensures correct year even if the page is prerendered at build time.
		currentYear = new Date().getFullYear();

		if (!browser) {
			isVisible = true;
			return;
		}
		return createVisibilityObserver(containerRef, (visible) => {
			isVisible = visible;
		});
	});

	const footerLinks = {
		products: [
			{ href: '/indicators', label: 'Indicators' },
			{ href: '/classes', label: 'Classes' },
			{ href: '/dashboard', label: 'Dashboard' },
			{ href: '/pricing', label: 'Pricing' }
		],
		company: [
			{ href: '/about', label: 'About Us' },
			{ href: '/blog', label: 'Blog' },
			{ href: '/contact', label: 'Contact' },
			{ href: '/careers', label: 'Careers' }
		],
		legal: [
			{ href: '/terms', label: 'Terms of Service' },
			{ href: '/privacy', label: 'Privacy Policy' },
			{ href: '/disclaimer', label: 'Risk Disclaimer' },
			{ href: '/refund', label: 'Refund Policy' }
		]
	};

	const socialLinks = [
		{
			href: 'https://facebook.com/revolutiontradingpros',
			icon: IconBrandFacebook,
			label: 'Facebook'
		},
		{
			href: 'https://twitter.com/revtradingpros',
			icon: IconBrandTwitter,
			label: 'X (Twitter)'
		},
		{
			href: 'https://instagram.com/revolutiontradingpros',
			icon: IconBrandInstagram,
			label: 'Instagram'
		},
		{
			href: 'https://youtube.com/@RevolutionTradingPros',
			icon: IconBrandYoutube,
			label: 'YouTube'
		}
	];
</script>

<!-- Invisible sentinel element for IntersectionObserver - always in DOM -->
<div bind:this={containerRef} class="footer-sentinel" aria-hidden="true"></div>

{#if isVisible}
	<footer in:heavySlide={{ delay: 0, duration: 800 }} class="marketing-footer">
		<div class="footer-container">
			<div class="footer-grid">
				<div class="footer-brand">
				<a href="/" class="footer-logo" aria-label="Revolution Trading Pros home">
					<img
						src="/revolution-trading-pros.png"
						alt="Revolution Trading Pros"
						width="160"
						height="40"
						loading="lazy"
						decoding="async"
					/>
				</a>

				<p class="footer-tagline">
					Professional trading education and tools for disciplined traders.
				</p>

				<div class="social-links" aria-label="Social links">
					{#each socialLinks as social}
						{@const IconComponent = social.icon}
						<a
							href={social.href}
							target="_blank"
							rel="noopener noreferrer"
							class="social-link"
							aria-label={social.label}
							title={social.label}
						>
							<IconComponent size={20} />
						</a>
					{/each}
				</div>
			</div>

			<nav class="footer-column" aria-labelledby="footer-products">
				<h2 class="footer-heading" id="footer-products">Products</h2>
				<ul class="footer-list">
					{#each footerLinks.products as link}
						<li><a href={link.href}>{link.label}</a></li>
					{/each}
				</ul>
			</nav>

			<nav class="footer-column" aria-labelledby="footer-company">
				<h2 class="footer-heading" id="footer-company">Company</h2>
				<ul class="footer-list">
					{#each footerLinks.company as link}
						<li><a href={link.href}>{link.label}</a></li>
					{/each}
				</ul>
			</nav>

			<nav class="footer-column" aria-labelledby="footer-legal">
				<h2 class="footer-heading" id="footer-legal">Legal</h2>
				<ul class="footer-list">
					{#each footerLinks.legal as link}
						<li><a href={link.href}>{link.label}</a></li>
					{/each}
				</ul>
			</nav>
			</div>

			<div class="risk-disclaimer">
				<p>
					<strong>Risk Disclaimer:</strong> Trading involves substantial risk of loss and is not suitable
					for all investors. Past performance is not indicative of future results. The content provided
					is for educational purposes only and should not be considered financial advice.
				</p>
			</div>

			<div class="footer-bottom">
				<p>&copy; {currentYear} Revolution Trading Pros. All rights reserved.</p>
			</div>
		</div>
	</footer>
{/if}

<style>
	/* Scoped box-sizing reset for footer */
	.marketing-footer,
	.marketing-footer *,
	.marketing-footer *::before,
	.marketing-footer *::after {
		box-sizing: border-box;
	}

	/* Invisible sentinel for IntersectionObserver - must be in DOM before conditional content */
	.footer-sentinel {
		position: absolute;
		width: 1px;
		height: 1px;
		pointer-events: none;
	}

	.marketing-footer {
		background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%);
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		padding: 4rem 0 2rem;
		width: 100%;
		max-width: 100%;
		min-width: 0;
		flex-shrink: 0;
		overflow-x: clip; /* Prevents any horizontal overflow from breaking layout */
		/* REMOVED: contain: paint - was causing rendering issues in flex contexts */
	}

	.footer-container {
		max-width: 1400px;
		width: 100%;
		margin: 0 auto;
		padding: 0 1.5rem;
		min-width: 0;
	}

	.footer-grid {
		display: grid;
		grid-template-columns: 2fr 1fr 1fr 1fr;
		gap: 3rem;
		margin-bottom: 3rem;
		min-width: 0;
	}

	.footer-brand {
		max-width: 320px;
		min-width: 0;
	}

	.footer-logo {
		display: inline-block;
		line-height: 0;
	}

	.footer-logo img {
		height: 40px;
		width: auto;
		max-width: 100%;
		margin-bottom: 1rem;
		display: block;
		object-fit: contain;
	}

	.footer-tagline {
		color: #64748b;
		font-size: 0.9375rem;
		line-height: 1.6;
		margin: 0 0 1.5rem 0;
		overflow-wrap: break-word;
		word-wrap: break-word;
	}

	.social-links {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: 0.75rem;
	}

	.social-link {
		width: 40px;
		height: 40px;
		min-width: 40px;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: 0.5rem;
		color: #94a3b8;
		transition: all 0.2s ease;
		text-decoration: none;
		flex-shrink: 0;
	}

	.social-link:hover {
		background: rgba(99, 102, 241, 0.2);
		color: #818cf8;
		transform: translateY(-2px);
	}

	.social-link:focus-visible {
		outline: 2px solid rgba(129, 140, 248, 0.9);
		outline-offset: 2px;
	}

	/* Ensure icons don't overflow */
	.social-link :global(svg) {
		width: 20px;
		height: 20px;
		flex-shrink: 0;
	}

	.footer-column {
		display: block;
		min-width: 0;
	}

	.footer-heading {
		color: #f1f5f9;
		font-size: 0.875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0 0 1.25rem 0;
	}

	.footer-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.footer-list li {
		margin-bottom: 0.75rem;
		display: block;
	}

	.footer-list li:last-child {
		margin-bottom: 0;
	}

	.footer-list a {
		color: #94a3b8;
		text-decoration: none;
		font-size: 0.9375rem;
		transition: color 0.2s ease;
		display: inline-block;
	}

	.footer-list a:hover {
		color: #f1f5f9;
	}

	.footer-list a:focus-visible {
		outline: 2px solid rgba(129, 140, 248, 0.9);
		outline-offset: 3px;
		border-radius: 0.25rem;
	}

	.risk-disclaimer {
		padding: 1.5rem;
		background: rgba(245, 158, 11, 0.05);
		border: 1px solid rgba(245, 158, 11, 0.2);
		border-radius: 0.75rem;
		margin-bottom: 2rem;
	}

	.risk-disclaimer p {
		color: #94a3b8;
		font-size: 0.8125rem;
		line-height: 1.6;
		margin: 0;
	}

	.risk-disclaimer strong {
		color: #fbbf24;
	}

	.footer-bottom {
		padding-top: 2rem;
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		text-align: center;
	}

	.footer-bottom p {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	@media (prefers-reduced-motion: reduce) {
		.social-link,
		.footer-list a {
			transition: none;
		}

		.social-link:hover {
			transform: none;
		}
	}

	/* Tablet breakpoint */
	@media (max-width: 1024px) {
		.marketing-footer {
			padding: 3rem 0 1.5rem;
		}

		.footer-grid {
			grid-template-columns: 1fr 1fr;
			gap: 2rem;
		}

		.footer-brand {
			grid-column: 1 / -1;
			max-width: 100%;
		}

		.social-links {
			justify-content: flex-start;
		}
	}

	/* Mobile breakpoint */
	@media (max-width: 640px) {
		.marketing-footer {
			padding: 2.5rem 0 1.5rem;
		}

		.footer-container {
			padding: 0 1rem;
		}

		.footer-grid {
			grid-template-columns: 1fr;
			gap: 2rem;
		}

		.footer-brand {
			grid-column: 1;
		}

		.risk-disclaimer {
			padding: 1rem;
		}

		.risk-disclaimer p {
			font-size: 0.75rem;
		}
	}
</style>

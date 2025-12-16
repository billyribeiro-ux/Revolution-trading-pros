<script lang="ts">
	/**
	 * MarketingFooter - Footer for marketing/public pages
	 * 
	 * @version 2.0.0
	 * @author Revolution Trading Pros
	 */
	import {
		IconBrandTwitter,
		IconBrandInstagram,
		IconBrandYoutube,
		IconBrandLinkedin,
		IconBrandDiscord
	} from '$lib/icons';

	const currentYear = new Date().getFullYear();

	const footerLinks = {
		products: [
			{ href: '/live-trading-rooms/day-trading', label: 'Day Trading Room' },
			{ href: '/live-trading-rooms/swing-trading', label: 'Swing Trading Room' },
			{ href: '/alerts', label: 'SPX Profit Pulse' },
			{ href: '/indicators', label: 'Trading Indicators' },
			{ href: '/courses', label: 'Trading Courses' }
		],
		company: [
			{ href: '/about', label: 'About Us' },
			{ href: '/our-mission', label: 'Our Mission' },
			{ href: '/mentorship', label: 'Mentorship' },
			{ href: '/blog', label: 'Blog' },
			{ href: '/resources', label: 'Resources' }
		],
		legal: [
			{ href: '/terms', label: 'Terms of Service' },
			{ href: '/privacy', label: 'Privacy Policy' },
			{ href: '/disclaimer', label: 'Risk Disclaimer' },
			{ href: '/refund', label: 'Refund Policy' }
		]
	};

	const socialLinks = [
		{ href: 'https://twitter.com/revtradingpros', icon: IconBrandTwitter, label: 'Twitter' },
		{ href: 'https://instagram.com/revolutiontradingpros', icon: IconBrandInstagram, label: 'Instagram' },
		{ href: 'https://youtube.com/@RevolutionTradingPros', icon: IconBrandYoutube, label: 'YouTube' },
		{ href: 'https://linkedin.com/company/revolution-trading-pros', icon: IconBrandLinkedin, label: 'LinkedIn' },
		{ href: 'https://discord.gg/rtp', icon: IconBrandDiscord, label: 'Discord' }
	];
</script>

<footer class="marketing-footer">
	<div class="footer-container">
		<!-- Main Footer Content -->
		<div class="footer-grid">
			<!-- Brand Column -->
			<div class="footer-brand">
				<a href="/" class="footer-logo">
					<img src="/revolution-trading-pros.png" alt="Revolution Trading Pros" />
				</a>
				<p class="footer-tagline">
					Professional trading education and tools for disciplined traders.
				</p>
				<div class="social-links">
					{#each socialLinks as social}
						{@const IconComponent = social.icon}
						<a 
							href={social.href} 
							target="_blank" 
							rel="noopener noreferrer"
							class="social-link"
							aria-label={social.label}
						>
							<IconComponent size={20} />
						</a>
					{/each}
				</div>
			</div>

			<!-- Products -->
			<div class="footer-column">
				<h4 class="footer-heading">Products</h4>
				<ul class="footer-list">
					{#each footerLinks.products as link}
						<li><a href={link.href}>{link.label}</a></li>
					{/each}
				</ul>
			</div>

			<!-- Company -->
			<div class="footer-column">
				<h4 class="footer-heading">Company</h4>
				<ul class="footer-list">
					{#each footerLinks.company as link}
						<li><a href={link.href}>{link.label}</a></li>
					{/each}
				</ul>
			</div>

			<!-- Legal -->
			<div class="footer-column">
				<h4 class="footer-heading">Legal</h4>
				<ul class="footer-list">
					{#each footerLinks.legal as link}
						<li><a href={link.href}>{link.label}</a></li>
					{/each}
				</ul>
			</div>
		</div>

		<!-- Risk Disclaimer -->
		<div class="risk-disclaimer">
			<p>
				<strong>Risk Disclaimer:</strong> Trading involves substantial risk of loss and is not suitable for all investors. 
				Past performance is not indicative of future results. The content provided is for educational purposes only 
				and should not be considered financial advice.
			</p>
		</div>

		<!-- Copyright -->
		<div class="footer-bottom">
			<p>&copy; {currentYear} Revolution Trading Pros. All rights reserved.</p>
		</div>
	</div>
</footer>

<style>
	.marketing-footer {
		background: linear-gradient(180deg, var(--color-rtp-surface, #1e293b) 0%, var(--color-rtp-bg, #0f172a) 100%);
		border-top: 1px solid rgba(99, 102, 241, 0.1);
		padding: 4rem 0 2rem;
	}

	.footer-container {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 1rem;
	}

	.footer-grid {
		display: grid;
		grid-template-columns: 2fr 1fr 1fr 1fr;
		gap: 3rem;
		margin-bottom: 3rem;
	}

	@media (max-width: 1024px) {
		.footer-grid {
			grid-template-columns: 1fr 1fr;
			gap: 2rem;
		}

		.footer-brand {
			grid-column: span 2;
		}
	}

	@media (max-width: 640px) {
		.footer-grid {
			grid-template-columns: 1fr;
		}

		.footer-brand {
			grid-column: span 1;
		}
	}

	.footer-brand {
		max-width: 320px;
	}

	.footer-logo img {
		height: 40px;
		width: auto;
		margin-bottom: 1rem;
	}

	.footer-tagline {
		color: var(--color-rtp-muted, #64748b);
		font-size: 0.9375rem;
		line-height: 1.6;
		margin-bottom: 1.5rem;
	}

	.social-links {
		display: flex;
		gap: 0.75rem;
	}

	.social-link {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(99, 102, 241, 0.1);
		border: 1px solid rgba(99, 102, 241, 0.2);
		border-radius: var(--radius-md, 0.5rem);
		color: var(--color-rtp-muted, #94a3b8);
		transition: all 0.2s;
	}

	.social-link:hover {
		background: rgba(99, 102, 241, 0.2);
		color: var(--color-rtp-primary, #818cf8);
		transform: translateY(-2px);
	}

	.footer-heading {
		color: var(--color-rtp-text, #f1f5f9);
		font-size: 0.875rem;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 1.25rem;
	}

	.footer-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.footer-list li {
		margin-bottom: 0.75rem;
	}

	.footer-list a {
		color: var(--color-rtp-muted, #94a3b8);
		text-decoration: none;
		font-size: 0.9375rem;
		transition: color 0.2s;
	}

	.footer-list a:hover {
		color: var(--color-rtp-text, #f1f5f9);
	}

	.risk-disclaimer {
		padding: 1.5rem;
		background: rgba(245, 158, 11, 0.05);
		border: 1px solid rgba(245, 158, 11, 0.2);
		border-radius: var(--radius-lg, 0.75rem);
		margin-bottom: 2rem;
	}

	.risk-disclaimer p {
		color: var(--color-rtp-muted, #94a3b8);
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
		color: var(--color-rtp-muted, #64748b);
		font-size: 0.875rem;
		margin: 0;
	}
</style>

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
		IconBrandDiscord,
		IconBrandFacebook
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
		{ href: 'https://facebook.com/revolutiontradingpros', icon: IconBrandFacebook, label: 'Facebook' },
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
	/* ICT 7 Apple Principal Engineer: Maximum specificity to override Tailwind preflight */
	/* The * { margin: 0; padding: 0; } reset in app.css requires !important everywhere */
	
	:global(footer.marketing-footer) {
		background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%) !important;
		border-top: 1px solid rgba(99, 102, 241, 0.1) !important;
		padding: 4rem 0 2rem !important;
		width: 100% !important;
		box-sizing: border-box !important;
	}

	:global(footer.marketing-footer .footer-container) {
		max-width: 1400px !important;
		margin: 0 auto !important;
		padding: 0 1.5rem !important;
		box-sizing: border-box !important;
	}

	:global(footer.marketing-footer .footer-grid) {
		display: grid !important;
		grid-template-columns: 2fr 1fr 1fr 1fr !important;
		gap: 3rem !important;
		margin-bottom: 3rem !important;
	}

	@media (max-width: 1024px) {
		:global(footer.marketing-footer .footer-grid) {
			grid-template-columns: 1fr 1fr !important;
			gap: 2rem !important;
		}

		:global(footer.marketing-footer .footer-brand) {
			grid-column: span 2 !important;
		}
	}

	@media (max-width: 640px) {
		:global(footer.marketing-footer .footer-grid) {
			grid-template-columns: 1fr !important;
		}

		:global(footer.marketing-footer .footer-brand) {
			grid-column: span 1 !important;
		}
	}

	:global(footer.marketing-footer .footer-brand) {
		max-width: 320px !important;
	}

	:global(footer.marketing-footer .footer-logo img) {
		height: 40px !important;
		width: auto !important;
		margin-bottom: 1rem !important;
	}

	:global(footer.marketing-footer .footer-tagline) {
		color: #64748b !important;
		font-size: 0.9375rem !important;
		line-height: 1.6 !important;
		margin-bottom: 1.5rem !important;
	}

	:global(footer.marketing-footer .social-links) {
		display: flex !important;
		flex-direction: row !important;
		gap: 0.75rem !important;
	}

	:global(footer.marketing-footer .social-link) {
		width: 40px !important;
		height: 40px !important;
		display: flex !important;
		align-items: center !important;
		justify-content: center !important;
		background: rgba(99, 102, 241, 0.1) !important;
		border: 1px solid rgba(99, 102, 241, 0.2) !important;
		border-radius: 0.5rem !important;
		color: #94a3b8 !important;
		transition: all 0.2s !important;
	}

	:global(footer.marketing-footer .social-link:hover) {
		background: rgba(99, 102, 241, 0.2) !important;
		color: #818cf8 !important;
		transform: translateY(-2px) !important;
	}

	:global(footer.marketing-footer .footer-column) {
		display: block !important;
	}

	:global(footer.marketing-footer .footer-heading) {
		color: #f1f5f9 !important;
		font-size: 0.875rem !important;
		font-weight: 600 !important;
		text-transform: uppercase !important;
		letter-spacing: 0.05em !important;
		margin-bottom: 1.25rem !important;
	}

	:global(footer.marketing-footer .footer-list) {
		list-style: none !important;
		padding: 0 !important;
		margin: 0 !important;
	}

	:global(footer.marketing-footer .footer-list li) {
		margin-bottom: 0.75rem !important;
		display: block !important;
	}

	:global(footer.marketing-footer .footer-list a) {
		color: #94a3b8 !important;
		text-decoration: none !important;
		font-size: 0.9375rem !important;
		transition: color 0.2s !important;
	}

	:global(footer.marketing-footer .footer-list a:hover) {
		color: #f1f5f9 !important;
	}

	:global(footer.marketing-footer .risk-disclaimer) {
		padding: 1.5rem !important;
		background: rgba(245, 158, 11, 0.05) !important;
		border: 1px solid rgba(245, 158, 11, 0.2) !important;
		border-radius: 0.75rem !important;
		margin-bottom: 2rem !important;
	}

	:global(footer.marketing-footer .risk-disclaimer p) {
		color: #94a3b8 !important;
		font-size: 0.8125rem !important;
		line-height: 1.6 !important;
		margin: 0 !important;
	}

	:global(footer.marketing-footer .risk-disclaimer strong) {
		color: #fbbf24 !important;
	}

	:global(footer.marketing-footer .footer-bottom) {
		padding-top: 2rem !important;
		border-top: 1px solid rgba(99, 102, 241, 0.1) !important;
		text-align: center !important;
	}

	:global(footer.marketing-footer .footer-bottom p) {
		color: #64748b !important;
		font-size: 0.875rem !important;
		margin: 0 !important;
	}
</style>

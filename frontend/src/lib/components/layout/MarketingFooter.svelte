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

<!-- ICT 7 Apple Principal Engineer: Inline styles for bulletproof rendering -->
<!-- Tailwind v4's @tailwindcss/vite plugin can interfere with Svelte's :global() CSS -->
<!-- Inline styles cannot be overridden by any build process or CSS layer -->
<footer 
	class="marketing-footer"
	style="background: linear-gradient(180deg, #1e293b 0%, #0f172a 100%); border-top: 1px solid rgba(99, 102, 241, 0.1); padding: 4rem 0 2rem; width: 100%; box-sizing: border-box;"
>
	<div 
		class="footer-container"
		style="max-width: 1400px; margin: 0 auto; padding: 0 1.5rem; box-sizing: border-box;"
	>
		<!-- Main Footer Content -->
		<div 
			class="footer-grid"
			style="display: grid; grid-template-columns: 2fr 1fr 1fr 1fr; gap: 3rem; margin-bottom: 3rem;"
		>
			<!-- Brand Column -->
			<div class="footer-brand" style="max-width: 320px;">
				<a href="/" class="footer-logo">
					<img 
						src="/revolution-trading-pros.png" 
						alt="Revolution Trading Pros" 
						style="height: 40px; width: auto; margin-bottom: 1rem;"
					/>
				</a>
				<p 
					class="footer-tagline"
					style="color: #64748b; font-size: 0.9375rem; line-height: 1.6; margin-bottom: 1.5rem;"
				>
					Professional trading education and tools for disciplined traders.
				</p>
				<div 
					class="social-links"
					style="display: flex; flex-direction: row; gap: 0.75rem;"
				>
					{#each socialLinks as social}
						{@const IconComponent = social.icon}
						<a 
							href={social.href} 
							target="_blank" 
							rel="noopener noreferrer"
							class="social-link"
							aria-label={social.label}
							style="width: 40px; height: 40px; display: flex; align-items: center; justify-content: center; background: rgba(99, 102, 241, 0.1); border: 1px solid rgba(99, 102, 241, 0.2); border-radius: 0.5rem; color: #94a3b8; transition: all 0.2s;"
						>
							<IconComponent size={20} />
						</a>
					{/each}
				</div>
			</div>

			<!-- Products -->
			<div class="footer-column">
				<h4 
					class="footer-heading"
					style="color: #f1f5f9; font-size: 0.875rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 1.25rem;"
				>Products</h4>
				<ul class="footer-list" style="list-style: none; padding: 0; margin: 0;">
					{#each footerLinks.products as link}
						<li style="margin-bottom: 0.75rem; display: block;">
							<a 
								href={link.href}
								style="color: #94a3b8; text-decoration: none; font-size: 0.9375rem; transition: color 0.2s;"
							>{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Company -->
			<div class="footer-column">
				<h4 
					class="footer-heading"
					style="color: #f1f5f9; font-size: 0.875rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 1.25rem;"
				>Company</h4>
				<ul class="footer-list" style="list-style: none; padding: 0; margin: 0;">
					{#each footerLinks.company as link}
						<li style="margin-bottom: 0.75rem; display: block;">
							<a 
								href={link.href}
								style="color: #94a3b8; text-decoration: none; font-size: 0.9375rem; transition: color 0.2s;"
							>{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Legal -->
			<div class="footer-column">
				<h4 
					class="footer-heading"
					style="color: #f1f5f9; font-size: 0.875rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 1.25rem;"
				>Legal</h4>
				<ul class="footer-list" style="list-style: none; padding: 0; margin: 0;">
					{#each footerLinks.legal as link}
						<li style="margin-bottom: 0.75rem; display: block;">
							<a 
								href={link.href}
								style="color: #94a3b8; text-decoration: none; font-size: 0.9375rem; transition: color 0.2s;"
							>{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>
		</div>

		<!-- Risk Disclaimer -->
		<div 
			class="risk-disclaimer"
			style="padding: 1.5rem; background: rgba(245, 158, 11, 0.05); border: 1px solid rgba(245, 158, 11, 0.2); border-radius: 0.75rem; margin-bottom: 2rem;"
		>
			<p style="color: #94a3b8; font-size: 0.8125rem; line-height: 1.6; margin: 0;">
				<strong style="color: #fbbf24;">Risk Disclaimer:</strong> Trading involves substantial risk of loss and is not suitable for all investors. 
				Past performance is not indicative of future results. The content provided is for educational purposes only 
				and should not be considered financial advice.
			</p>
		</div>

		<!-- Copyright -->
		<div 
			class="footer-bottom"
			style="padding-top: 2rem; border-top: 1px solid rgba(99, 102, 241, 0.1); text-align: center;"
		>
			<p style="color: #64748b; font-size: 0.875rem; margin: 0;">&copy; {currentYear} Revolution Trading Pros. All rights reserved.</p>
		</div>
	</div>
</footer>

<style>
	/* ICT 7 Apple Principal Engineer: Responsive breakpoints and hover states */
	/* Inline styles handle base layout; CSS handles media queries (can't be inline) */
	
	/* Responsive: Tablet */
	@media (max-width: 1024px) {
		:global(footer.marketing-footer .footer-grid) {
			grid-template-columns: 1fr 1fr !important;
			gap: 2rem !important;
		}

		:global(footer.marketing-footer .footer-brand) {
			grid-column: span 2 !important;
			max-width: 100% !important;
		}
	}

	/* Responsive: Mobile */
	@media (max-width: 640px) {
		:global(footer.marketing-footer .footer-grid) {
			grid-template-columns: 1fr !important;
		}

		:global(footer.marketing-footer .footer-brand) {
			grid-column: span 1 !important;
		}
	}

	/* Hover states (can't be inline) */
	:global(footer.marketing-footer .social-link:hover) {
		background: rgba(99, 102, 241, 0.2) !important;
		color: #818cf8 !important;
		transform: translateY(-2px) !important;
	}

	:global(footer.marketing-footer .footer-list a:hover) {
		color: #f1f5f9 !important;
	}
</style>

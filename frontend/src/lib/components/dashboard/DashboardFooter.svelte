<script lang="ts">
	/**
	 * Dashboard Footer Component - Simpler Trading EXACT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Full footer for dashboard pages with:
	 * - Logo and contact info
	 * - LEARN MORE, ABOUT, LEGAL columns
	 * - App store badges
	 * - Social media links
	 * - Disclaimer text
	 * - Copyright
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import {
		IconBrandFacebook,
		IconBrandTwitter,
		IconBrandYoutube,
		IconBrandInstagram
	} from '$lib/icons';

	const currentYear = new Date().getFullYear();

	// Footer configuration (editable)
	const config = {
		logo: '/revolution-trading-pros.png',
		email: 'support@revolutiontradingpros.com',
		address: {
			line1: '123 Trading Street',
			line2: 'Suite 100',
			city: 'Austin, TX 78759'
		},
		phone: '512-266-8659'
	};

	const learnMoreLinks = [
		{ href: '/memberships', label: 'Memberships' },
		{ href: '/daily-videos', label: 'Daily Videos' },
		{ href: '/support', label: 'Support' },
		{ href: '/mailing-list', label: 'Rejoin Mailing List' },
		{ href: '/partners', label: 'Partners' }
	];

	const aboutLinks = [
		{ href: '/about', label: 'About Revolution' },
		{ href: '/traders', label: 'Our Traders' },
		{ href: '/store', label: 'Revolution Store' },
		{ href: '/blog', label: 'Blog' }
	];

	const legalLinks = [
		{ href: '/privacy-policy', label: 'Privacy Policy' },
		{ href: '/terms-and-conditions', label: 'Terms & Conditions' },
		{ href: '/consent-preferences', label: 'My Consent Preferences' }
	];

	const socialLinks = [
		{ href: 'https://facebook.com', icon: IconBrandFacebook, label: 'Facebook' },
		{ href: 'https://twitter.com', icon: IconBrandTwitter, label: 'Twitter' },
		{ href: 'https://youtube.com', icon: IconBrandYoutube, label: 'YouTube' },
		{ href: 'https://instagram.com', icon: IconBrandInstagram, label: 'Instagram' }
	];
</script>

<footer class="dashboard-footer">
	<div class="dashboard-footer__container">
		<!-- Top Section: Logo + Links -->
		<div class="dashboard-footer__top">
			<!-- Logo Column -->
			<div class="dashboard-footer__brand">
				<img
					src={config.logo}
					alt="Revolution Trading Pros"
					class="dashboard-footer__logo"
				/>
				<div class="dashboard-footer__contact">
					<a href="mailto:{config.email}" class="dashboard-footer__email">{config.email}</a>
					<p class="dashboard-footer__address">
						{config.address.line1}<br />
						{config.address.line2}<br />
						{config.address.city}
					</p>
					<p class="dashboard-footer__phone">{config.phone}</p>
				</div>
			</div>

			<!-- Learn More Column -->
			<div class="dashboard-footer__column">
				<h4 class="dashboard-footer__heading">LEARN MORE</h4>
				<ul class="dashboard-footer__links">
					{#each learnMoreLinks as link (link.href)}
						<li>
							<a href={link.href}>{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- About Column -->
			<div class="dashboard-footer__column">
				<h4 class="dashboard-footer__heading">ABOUT</h4>
				<ul class="dashboard-footer__links">
					{#each aboutLinks as link (link.href)}
						<li>
							<a href={link.href}>{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Legal Column -->
			<div class="dashboard-footer__column">
				<h4 class="dashboard-footer__heading">LEGAL</h4>
				<ul class="dashboard-footer__links">
					{#each legalLinks as link (link.href)}
						<li>
							<a href={link.href}>{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- App Store & Social -->
			<div class="dashboard-footer__column dashboard-footer__column--apps">
				<div class="dashboard-footer__app-badges">
					<a href="https://apps.apple.com" target="_blank" rel="noopener noreferrer" class="app-badge">
						<img src="https://simpler-cdn.s3.amazonaws.com/images/apple-app-store-badge.svg" alt="Download on App Store" />
					</a>
					<a href="https://play.google.com" target="_blank" rel="noopener noreferrer" class="app-badge">
						<img src="https://simpler-cdn.s3.amazonaws.com/images/google-play-badge.svg" alt="Get it on Google Play" />
					</a>
				</div>
				<div class="dashboard-footer__social">
					{#each socialLinks as social (social.href)}
						{@const Icon = social.icon}
						<a
							href={social.href}
							target="_blank"
							rel="noopener noreferrer"
							class="dashboard-footer__social-link"
							aria-label={social.label}
						>
							<Icon size={20} />
						</a>
					{/each}
				</div>
			</div>
		</div>

		<!-- Disclaimer Section -->
		<div class="dashboard-footer__disclaimer">
			<p>
				The information contained on this website is solely for educational purposes, and does not constitute investment advice. The risk of trading in securities markets can be substantial. You must review and agree to our Disclaimers and Terms and Conditions before using this site.
			</p>
			<p>
				U.S. Government Required Disclaimer - Commodity Futures Trading Commission. Futures and Options trading has large potential rewards, but also large potential risk. You must be aware of the risks and be willing to accept them in order to invest in the futures and options markets. Don't trade with money you can't afford to lose. This website is neither a solicitation nor an offer to Buy/Sell futures or options. No representation is being made that any account will or is likely to achieve profits or losses similar to those discussed on this website. The past performance of any trading system or methodology is not necessarily indicative of future results.
			</p>
			<p>
				Individual results may vary, and testimonials are not claimed to represent typical results. All testimonials are by real people, and may not reflect the typical purchaser's experience, and are not intended to represent or guarantee that anyone will achieve the same or similar results.
			</p>
			<p>
				Revolution Trading Pros' Traders and employees will NEVER manage or offer to manage a customer or individual's options, stocks, currencies, futures, or any financial markets or securities account. If someone claiming to represent or be associated with Revolution Trading Pros solicits you for money or offers to manage your trading account, do not provide any personal information and contact us immediately.
			</p>
			<p>
				CFTC RULE 4.41 - HYPOTHETICAL OR SIMULATED PERFORMANCE RESULTS HAVE CERTAIN LIMITATIONS. UNLIKE AN ACTUAL PERFORMANCE RECORD, SIMULATED RESULTS DO NOT REPRESENT ACTUAL TRADING. ALSO, SINCE THE TRADES HAVE NOT BEEN EXECUTED, THE RESULTS MAY HAVE UNDER-OR-OVER COMPENSATED FOR THE IMPACT, IF ANY, OF CERTAIN MARKET FACTORS, SUCH AS LACK OF LIQUIDITY. SIMULATED TRADING PROGRAMS IN GENERAL ARE ALSO SUBJECT TO THE FACT THAT THEY ARE DESIGNED WITH THE BENEFIT OF HINDSIGHT. NO REPRESENTATION IS BEING MADE THAT ANY ACCOUNT WILL OR IS LIKELY TO ACHIEVE PROFIT OR LOSSES SIMILAR TO THOSE SHOWN.
			</p>
		</div>

		<!-- Copyright Section -->
		<div class="dashboard-footer__copyright">
			<p>&copy; {currentYear} <strong>Revolution Trading Pros</strong> ALL RIGHTS RESERVED. Reproduction without Permission Prohibited</p>
		</div>
	</div>
</footer>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   DASHBOARD FOOTER - Simpler Trading EXACT
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-footer {
		background: #0c2434;
		color: #fff;
		padding: 40px 0 0;
		font-family: 'Open Sans', system-ui, sans-serif;
	}

	.dashboard-footer__container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 30px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   TOP SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-footer__top {
		display: grid;
		grid-template-columns: 1.5fr 1fr 1fr 1fr 1.2fr;
		gap: 30px;
		padding-bottom: 30px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	/* Brand Column */
	.dashboard-footer__brand {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.dashboard-footer__logo {
		width: 180px;
		height: auto;
	}

	.dashboard-footer__contact {
		font-size: 13px;
		line-height: 1.6;
	}

	.dashboard-footer__email {
		color: #fff;
		text-decoration: none;
		display: block;
		margin-bottom: 8px;
	}

	.dashboard-footer__email:hover {
		text-decoration: underline;
	}

	.dashboard-footer__address {
		color: rgba(255, 255, 255, 0.7);
		margin: 0 0 8px 0;
	}

	.dashboard-footer__phone {
		color: rgba(255, 255, 255, 0.7);
		margin: 0;
	}

	/* Link Columns */
	.dashboard-footer__column {
		display: flex;
		flex-direction: column;
	}

	.dashboard-footer__heading {
		color: #fff;
		font-size: 13px;
		font-weight: 700;
		letter-spacing: 0.5px;
		margin: 0 0 16px 0;
	}

	.dashboard-footer__links {
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.dashboard-footer__links li {
		margin-bottom: 8px;
	}

	.dashboard-footer__links a {
		color: rgba(255, 255, 255, 0.7);
		font-size: 13px;
		text-decoration: none;
		transition: color 0.15s ease;
	}

	.dashboard-footer__links a:hover {
		color: #fff;
	}

	/* App Store & Social Column */
	.dashboard-footer__column--apps {
		align-items: flex-start;
	}

	.dashboard-footer__app-badges {
		display: flex;
		gap: 10px;
		margin-bottom: 16px;
	}

	.app-badge img {
		height: 36px;
		width: auto;
	}

	.dashboard-footer__social {
		display: flex;
		gap: 8px;
	}

	.dashboard-footer__social-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 4px;
		color: #fff;
		transition: background 0.15s ease;
	}

	.dashboard-footer__social-link:hover {
		background: rgba(255, 255, 255, 0.2);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   DISCLAIMER SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-footer__disclaimer {
		padding: 24px 0;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.dashboard-footer__disclaimer p {
		color: rgba(255, 255, 255, 0.5);
		font-size: 11px;
		line-height: 1.6;
		margin: 0 0 12px 0;
		text-align: center;
	}

	.dashboard-footer__disclaimer p:last-child {
		margin-bottom: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   COPYRIGHT SECTION
	   ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard-footer__copyright {
		padding: 20px 0;
		text-align: center;
	}

	.dashboard-footer__copyright p {
		color: rgba(255, 255, 255, 0.5);
		font-size: 12px;
		margin: 0;
	}

	.dashboard-footer__copyright strong {
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media screen and (max-width: 1024px) {
		.dashboard-footer__top {
			grid-template-columns: 1fr 1fr 1fr;
		}

		.dashboard-footer__brand {
			grid-column: 1 / -1;
			flex-direction: row;
			align-items: center;
			gap: 30px;
		}

		.dashboard-footer__column--apps {
			grid-column: 1 / -1;
			flex-direction: row;
			align-items: center;
			gap: 20px;
		}

		.dashboard-footer__app-badges {
			margin-bottom: 0;
		}
	}

	@media screen and (max-width: 768px) {
		.dashboard-footer__top {
			grid-template-columns: 1fr 1fr;
		}

		.dashboard-footer__brand {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard-footer__column--apps {
			flex-direction: column;
			align-items: flex-start;
		}

		.dashboard-footer__app-badges {
			margin-bottom: 16px;
		}
	}

	@media screen and (max-width: 576px) {
		.dashboard-footer {
			padding: 30px 0 0;
		}

		.dashboard-footer__container {
			padding: 0 16px;
		}

		.dashboard-footer__top {
			grid-template-columns: 1fr;
			gap: 24px;
		}

		.dashboard-footer__disclaimer p {
			text-align: left;
		}
	}
</style>

<script lang="ts">
	/**
	 * Footer Component - Revolution Trading Pros
	 * Matches brand design with logo, links, contact info, and legal text
	 */
	import {
		IconBrandFacebook,
		IconBrandInstagram,
		IconBrandTwitter,
		IconBrandYoutube,
		IconPhone,
		IconMail
	} from '@tabler/icons-svelte';

	const currentYear = new Date().getFullYear();

	const learnMoreLinks = [
		{ href: '/courses', label: 'Courses' },
		{ href: '/alerts', label: 'Our Atlases' },
		{ href: '/mentorship', label: 'Mentorship' },
		{ href: '/indicators', label: 'Indicators' }
	];

	const legalLinks = [
		{ href: '/privacy-policy', label: 'Privacy Policy' },
		{ href: '/terms-and-conditions', label: 'Terms and Conditions' },
		{ href: '/disclaimer', label: 'Disclaimer' },
		{ href: '/refund-policy', label: 'Refund Policy' }
	];

	const socialLinks = [
		{ href: 'https://facebook.com', icon: IconBrandFacebook, label: 'Facebook' },
		{ href: 'https://instagram.com', icon: IconBrandInstagram, label: 'Instagram' },
		{ href: 'https://twitter.com', icon: IconBrandTwitter, label: 'Twitter' },
		{ href: 'https://youtube.com', icon: IconBrandYoutube, label: 'YouTube' }
	];
</script>

<footer class="footer">
	<div class="footer__container">
		<!-- Top Section: Logo + Links + Contact -->
		<div class="footer__top">
			<!-- Logo Column -->
			<div class="footer__brand">
				<img
					src="/revolution-trading-pros.png"
					alt="Revolution Trading Pros"
					width="200"
					height="68"
					class="footer__logo"
				/>
				<p class="footer__tagline">EMPOWERING FINANCIAL FREEDOM!</p>
			</div>

			<!-- Learn More Column -->
			<div class="footer__column">
				<h3 class="footer__heading">LEARN MORE</h3>
				<ul class="footer__links">
					{#each learnMoreLinks as link (link.href)}
						<li>
							<a href={link.href} class="footer__link">{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Legal Pages Column -->
			<div class="footer__column">
				<h3 class="footer__heading">LEGAL PAGES</h3>
				<ul class="footer__links">
					{#each legalLinks as link (link.href)}
						<li>
							<a href={link.href} class="footer__link">{link.label}</a>
						</li>
					{/each}
				</ul>
			</div>

			<!-- Contact Column -->
			<div class="footer__column">
				<h3 class="footer__heading">Dedicated to Your Success</h3>
				<p class="footer__subtitle">Reach Out Anytime for Support and Guidance</p>

				<div class="footer__contact">
					<a href="tel:+12037210940" class="footer__contact-item">
						<IconPhone size={18} />
						<span>+1-203-721-0940</span>
					</a>
					<a href="mailto:support@revolutiontradingpros.com" class="footer__contact-item">
						<IconMail size={18} />
						<span>support@revolutiontradingpros.com</span>
					</a>
				</div>

				<!-- Social Icons -->
				<div class="footer__social">
					{#each socialLinks as social (social.href)}
						{@const Icon = social.icon}
						<a
							href={social.href}
							target="_blank"
							rel="noopener noreferrer"
							class="footer__social-link"
							aria-label={social.label}
						>
							<Icon size={20} />
						</a>
					{/each}
				</div>
			</div>
		</div>

		<!-- Disclaimer Section -->
		<div class="footer__disclaimer">
			<p>
				<strong>Disclaimer:</strong> The information contained on this website is solely for educational purposes,
				and does not constitute investment advice. The risk of trading in securities markets can be substantial.
				You must review and agree to our Disclaimers and Terms and Conditions before using this site.
			</p>
			<p>
				U.S. Government Required Disclaimer - Commodity Futures Trading Commission. Futures and Options trading has
				large potential rewards, but also large potential risk. You must be aware of the risks and be willing to
				accept them in order to invest in the futures and options markets. Don't trade with money you can't afford
				to lose. This website is neither a solicitation nor an offer to Buy/Sell futures or options. No
				representation is being made that any account will or is likely to achieve profits or losses similar to
				those discussed on this website. The past performance of any trading system or methodology is not
				necessarily indicative of future results.
			</p>
			<p>
				Hypothetical results have many inherent limitations, some of which are described below. No representation
				is being made that any account will or is likely to achieve profits or losses similar to those shown; in
				fact, there are frequently sharp differences between hypothetical performance results and the actual
				results subsequently achieved by any particular trading program. One of the limitations of hypothetical
				performance results is that they are generally prepared with the benefit of hindsight. In addition,
				hypothetical trading does not involve financial risk, and no hypothetical trading record can completely
				account for the impact of financial risk of actual trading. For example, the ability to withstand losses
				or to adhere to a particular trading program in spite of trading losses are material points which can also
				adversely affect actual trading results. There are numerous other factors related to the markets in general
				or to the implementation of any specific trading program which cannot be fully accounted for in the
				preparation of hypothetical performance results and all which can adversely affect trading results.
			</p>
			<p>
				CFTC RULE 4.41 - HYPOTHETICAL OR SIMULATED PERFORMANCE RESULTS HAVE CERTAIN LIMITATIONS. UNLIKE AN ACTUAL
				PERFORMANCE RECORD, SIMULATED RESULTS DO NOT REPRESENT ACTUAL TRADING. ALSO, SINCE THE TRADES HAVE NOT BEEN
				EXECUTED, THE RESULTS MAY HAVE UNDER-OR-OVER COMPENSATED FOR THE IMPACT, IF ANY, OF CERTAIN MARKET FACTORS,
				SUCH AS LACK OF LIQUIDITY. SIMULATED TRADING PROGRAMS IN GENERAL ARE ALSO SUBJECT TO THE FACT THAT THEY ARE
				DESIGNED WITH THE BENEFIT OF HINDSIGHT. NO REPRESENTATION IS BEING MADE THAT ANY ACCOUNT WILL OR IS LIKELY
				TO ACHIEVE PROFIT OR LOSSES SIMILAR TO THOSE SHOWN.
			</p>
		</div>

		<!-- Bottom Section: Copyright -->
		<div class="footer__bottom">
			<p>&copy; {currentYear} Revolution Trading Pros. All Rights Reserved. Reproduction without Permission Prohibited.</p>
		</div>
	</div>
</footer>

<style>
	.footer {
		background: #1e293b;
		color: #e5e7eb;
		padding: 60px 0 0;
		font-family: 'Montserrat', system-ui, sans-serif;
	}

	.footer__container {
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 24px;
	}

	/* Top Section */
	.footer__top {
		display: grid;
		grid-template-columns: 1.5fr 1fr 1fr 1.5fr;
		gap: 48px;
		padding-bottom: 48px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	@media (max-width: 1024px) {
		.footer__top {
			grid-template-columns: 1fr 1fr;
			gap: 32px;
		}
	}

	@media (max-width: 640px) {
		.footer__top {
			grid-template-columns: 1fr;
			gap: 32px;
		}
	}

	/* Brand Column */
	.footer__brand {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.footer__logo {
		width: 200px;
		height: 68px;
		object-fit: contain;
		object-position: left center;
	}

	.footer__tagline {
		color: #94a3b8;
		font-size: 0.85rem;
		font-weight: 600;
		letter-spacing: 0.05em;
		margin: 0;
	}

	/* Column Styles */
	.footer__column {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.footer__heading {
		color: #facc15;
		font-size: 0.9rem;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin: 0;
	}

	.footer__subtitle {
		color: #cbd5e1;
		font-size: 0.9rem;
		font-weight: 500;
		margin: 0;
		line-height: 1.4;
	}

	.footer__links {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.footer__link {
		color: #94a3b8;
		font-size: 0.9rem;
		text-decoration: none;
		transition: color 0.2s;
	}

	.footer__link:hover {
		color: #facc15;
	}

	/* Contact Section */
	.footer__contact {
		display: flex;
		flex-direction: column;
		gap: 12px;
		margin-top: 8px;
	}

	.footer__contact-item {
		display: flex;
		align-items: center;
		gap: 10px;
		color: #e5e7eb;
		font-size: 0.9rem;
		text-decoration: none;
		transition: color 0.2s;
	}

	.footer__contact-item:hover {
		color: #facc15;
	}

	/* Social Icons */
	.footer__social {
		display: flex;
		gap: 12px;
		margin-top: 16px;
	}

	.footer__social-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: rgba(255, 255, 255, 0.1);
		border-radius: 8px;
		color: #e5e7eb;
		transition: background-color 0.2s, color 0.2s;
	}

	.footer__social-link:hover {
		background: #facc15;
		color: #05142b;
	}

	/* Disclaimer Section */
	.footer__disclaimer {
		padding: 32px 0;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}

	.footer__disclaimer p {
		color: #94a3b8;
		font-size: 0.75rem;
		line-height: 1.6;
		margin: 0 0 12px 0;
	}

	.footer__disclaimer p:last-child {
		margin-bottom: 0;
	}

	.footer__disclaimer strong {
		color: #cbd5e1;
		font-weight: 700;
	}

	/* Bottom Section */
	.footer__bottom {
		padding: 24px 0;
		text-align: center;
	}

	.footer__bottom p {
		color: #64748b;
		font-size: 0.85rem;
		margin: 0;
	}

	@media (max-width: 640px) {
		.footer {
			padding: 40px 0 0;
		}

		.footer__container {
			padding: 0 16px;
		}

		.footer__logo {
			width: 160px;
			height: 54px;
		}

		.footer__bottom p {
			font-size: 0.75rem;
		}
	}
</style>

<!--
	URL: /privacy-policy

	Privacy Policy — Revolution Trading Pros
	═══════════════════════════════════════════════════════════════════════════
	Required by Google Consent Mode v2, GDPR Article 13, CCPA, and Lighthouse
	Best Practices. The actual legal content needs to be reviewed by counsel
	before launch — this is a structurally complete stub with the standard
	sections every commercial web service must disclose.

	@version 1.0.0 - April 2026 stub
-->
<script lang="ts">
	import type { PageProps } from './$types';

	let { data: _data }: PageProps = $props();

	const lastUpdated = '2026-04-11';
</script>

<main class="legal-page" id="main-content">
	<div
		class="mx-auto w-full max-w-3xl 3xl:max-w-[1100px] 4xl:max-w-[1400px] 5xl:max-w-[1700px] 6xl:max-w-[2000px] px-4 sm:px-6 lg:px-8 py-12 sm:py-16 lg:py-24"
	>
		<header class="mb-10">
			<h1 class="text-3xl sm:text-4xl lg:text-5xl font-bold tracking-tight">Privacy Policy</h1>
			<p class="mt-3 text-sm text-gray-600">
				Last updated: <time datetime={lastUpdated}>{lastUpdated}</time>
			</p>
		</header>

		<div class="prose prose-lg max-w-none">
			<p>
				This Privacy Policy explains how Revolution Trading Pros (&ldquo;we,&rdquo;
				&ldquo;us,&rdquo; or &ldquo;our&rdquo;) collects, uses, discloses, and protects information
				about you when you visit our website, create an account, or use any of our trading education
				services.
			</p>

			<h2>1. Information We Collect</h2>
			<p>We collect the following categories of personal information:</p>
			<ul>
				<li>
					<strong>Account information:</strong> name, email address, password hash, and any profile details
					you provide.
				</li>
				<li>
					<strong>Billing information:</strong> processed by our payment provider (Stripe). We never receive
					or store full card numbers.
				</li>
				<li>
					<strong>Usage data:</strong> pages visited, features used, device, browser, IP address, and
					approximate location, collected only with your consent.
				</li>
				<li>
					<strong>Cookies and similar technologies:</strong> see our
					<a href="/cookie-policy">Cookie Policy</a> for full details.
				</li>
			</ul>

			<h2>2. How We Use Your Information</h2>
			<p>We use personal information to:</p>
			<ul>
				<li>Provide, maintain, and improve our educational services.</li>
				<li>Process payments and manage subscriptions.</li>
				<li>Send service-related notifications and (with your consent) marketing emails.</li>
				<li>Detect and prevent fraud, abuse, and security incidents.</li>
				<li>Comply with legal obligations.</li>
			</ul>

			<h2>3. Legal Bases (GDPR)</h2>
			<p>
				If you are in the European Economic Area or the United Kingdom, our legal bases for
				processing are: contract performance (Article 6(1)(b)), legitimate interest (6(1)(f)), legal
				obligation (6(1)(c)), and consent (6(1)(a)) for analytics and marketing.
			</p>

			<h2>4. Sharing Your Information</h2>
			<p>
				We do not sell personal information. We share it only with: payment processors (Stripe),
				email/notification providers, hosting and analytics providers under data processing
				agreements, and with law enforcement when legally compelled.
			</p>

			<h2>5. Your Rights</h2>
			<p>
				Depending on your jurisdiction, you may have the right to access, rectify, erase, restrict,
				port, or object to processing of your personal data, and to withdraw consent at any time. To
				exercise any of these rights, contact
				<a href="mailto:privacy@revolution-trading-pros.com">privacy@revolution-trading-pros.com</a
				>.
			</p>

			<h2>6. International Data Transfers</h2>
			<p>
				Your information may be processed in countries other than your own, including the United
				States. We rely on Standard Contractual Clauses where required.
			</p>

			<h2>7. Data Retention</h2>
			<p>
				We retain personal information for as long as your account is active and for a reasonable
				period thereafter to comply with legal obligations and resolve disputes.
			</p>

			<h2>8. Security</h2>
			<p>
				We use industry-standard safeguards including TLS encryption in transit, encryption at rest,
				hashed passwords, and least-privilege access controls.
			</p>

			<h2>9. Children</h2>
			<p>
				Our services are not directed at children under 16. We do not knowingly collect personal
				information from children.
			</p>

			<h2>10. Changes to This Policy</h2>
			<p>
				We may update this Privacy Policy from time to time. Material changes will be announced on
				this page with a new &ldquo;Last updated&rdquo; date.
			</p>

			<h2>11. Contact Us</h2>
			<p>
				For privacy questions or to exercise your rights, email
				<a href="mailto:privacy@revolution-trading-pros.com">privacy@revolution-trading-pros.com</a
				>.
			</p>
		</div>
	</div>
</main>

<style>
	.legal-page :global(h2) {
		margin-top: 2.5rem;
		margin-bottom: 1rem;
		font-size: 1.5rem;
		font-weight: 700;
	}
	.legal-page :global(p),
	.legal-page :global(li) {
		line-height: 1.7;
	}
	.legal-page :global(a) {
		color: #2563eb;
		text-decoration: underline;
	}
	.legal-page :global(a:hover) {
		color: #1e40af;
	}
</style>

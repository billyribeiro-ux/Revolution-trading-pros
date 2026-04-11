<!--
	URL: /terms-of-service

	Terms of Service — Revolution Trading Pros
	═══════════════════════════════════════════════════════════════════════════
	Required by Google Consent Mode v2, GDPR, the FTC, and Lighthouse Best
	Practices. The actual legal content needs to be reviewed by counsel before
	launch — this is a structurally complete stub with the standard sections
	every commercial trading-education service must disclose, including the
	mandatory financial-risk disclaimer.

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
			<h1 class="text-3xl sm:text-4xl lg:text-5xl font-bold tracking-tight">Terms of Service</h1>
			<p class="mt-3 text-sm text-gray-600">
				Last updated: <time datetime={lastUpdated}>{lastUpdated}</time>
			</p>
		</header>

		<div class="prose prose-lg max-w-none">
			<p>
				These Terms of Service (&ldquo;Terms&rdquo;) govern your use of the Revolution Trading Pros
				website, trading education products, live trading rooms, software indicators, and any other
				services we offer (collectively, the &ldquo;Services&rdquo;). By accessing the Services you
				agree to be bound by these Terms.
			</p>

			<h2 id="risk-disclaimer" class="risk-heading">Risk Disclaimer</h2>
			<div class="risk-box" role="note" aria-label="Financial risk disclaimer">
				<p>
					<strong>Trading and investing carry a high level of risk.</strong> The information, education,
					signals, alerts, and tools provided by Revolution Trading Pros are for educational and informational
					purposes only and do not constitute financial, investment, legal, or tax advice. Past performance
					is not indicative of future results. You may lose some or all of your investment. You are solely
					responsible for your trading decisions and should consult a qualified financial advisor before
					acting on any information you receive from us.
				</p>
			</div>

			<h2>1. Eligibility</h2>
			<p>
				You must be at least 18 years old and legally able to enter into contracts to use the
				Services. By creating an account you represent that you meet these requirements.
			</p>

			<h2>2. Account &amp; Security</h2>
			<p>
				You are responsible for maintaining the confidentiality of your account credentials and for
				all activity that occurs under your account. Notify us immediately at
				<a href="mailto:security@revolution-trading-pros.com"
					>security@revolution-trading-pros.com</a
				> if you suspect unauthorized access.
			</p>

			<h2>3. Subscriptions, Billing, and Refunds</h2>
			<p>
				Subscriptions renew automatically at the end of each billing period unless you cancel
				beforehand. All fees are non-refundable except where required by law or explicitly offered
				in writing. You may cancel at any time from your account dashboard.
			</p>

			<h2>4. Acceptable Use</h2>
			<p>You agree not to:</p>
			<ul>
				<li>Share account credentials or content with non-subscribers.</li>
				<li>Reverse-engineer, decompile, or scrape the Services.</li>
				<li>Use the Services for any unlawful, harmful, or fraudulent purpose.</li>
				<li>Interfere with or disrupt the Services or other users&rsquo; access.</li>
				<li>Upload malicious code, viruses, or content that infringes others&rsquo; rights.</li>
			</ul>

			<h2>5. Intellectual Property</h2>
			<p>
				All content provided through the Services &mdash; including videos, indicators, course
				materials, software, and written content &mdash; is the property of Revolution Trading Pros
				or its licensors and is protected by copyright, trademark, and other intellectual property
				laws. We grant you a limited, non-exclusive, non-transferable license to access the content
				for personal, non-commercial use.
			</p>

			<h2>6. User Content</h2>
			<p>
				You retain ownership of any content you submit to the Services (e.g., chat messages, forum
				posts) but grant us a worldwide, royalty-free license to use, host, display, and adapt it as
				necessary to operate the Services.
			</p>

			<h2>7. No Investment Advice</h2>
			<p>
				The Services do not provide personalized investment advice. We are not a registered
				investment advisor, broker-dealer, or financial planner. Nothing we publish should be
				construed as a recommendation to buy or sell any security.
			</p>

			<h2>8. Disclaimer of Warranties</h2>
			<p>
				THE SERVICES ARE PROVIDED &ldquo;AS IS&rdquo; AND &ldquo;AS AVAILABLE&rdquo; WITHOUT
				WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING WITHOUT LIMITATION THE IMPLIED
				WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, OR NON-INFRINGEMENT.
			</p>

			<h2>9. Limitation of Liability</h2>
			<p>
				TO THE MAXIMUM EXTENT PERMITTED BY LAW, REVOLUTION TRADING PROS AND ITS OFFICERS, DIRECTORS,
				EMPLOYEES, AND AGENTS WILL NOT BE LIABLE FOR ANY INDIRECT, INCIDENTAL, CONSEQUENTIAL,
				SPECIAL, OR PUNITIVE DAMAGES, INCLUDING LOSS OF PROFITS, ARISING FROM YOUR USE OF THE
				SERVICES.
			</p>

			<h2>10. Indemnification</h2>
			<p>
				You agree to indemnify and hold Revolution Trading Pros harmless from any claims, losses, or
				expenses (including reasonable attorneys&rsquo; fees) arising out of your breach of these
				Terms or your use of the Services.
			</p>

			<h2>11. Termination</h2>
			<p>
				We may suspend or terminate your access to the Services at any time, with or without notice,
				for any reason, including violation of these Terms.
			</p>

			<h2>12. Governing Law</h2>
			<p>
				These Terms are governed by the laws of the United States and the state in which Revolution
				Trading Pros is incorporated, without regard to conflict of laws principles. Any disputes
				will be resolved exclusively in the state or federal courts located in that jurisdiction.
			</p>

			<h2>13. Changes to These Terms</h2>
			<p>
				We may modify these Terms at any time. Material changes will be announced on this page with
				a new &ldquo;Last updated&rdquo; date. Your continued use of the Services after such changes
				constitutes acceptance.
			</p>

			<h2>14. Contact</h2>
			<p>
				For questions about these Terms, email
				<a href="mailto:legal@revolution-trading-pros.com">legal@revolution-trading-pros.com</a>.
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
	.legal-page :global(.risk-heading) {
		color: #b91c1c;
	}
	.legal-page :global(.risk-box) {
		margin: 1.5rem 0;
		padding: 1.25rem 1.5rem;
		background: #fef2f2;
		border-left: 4px solid #b91c1c;
		border-radius: 0.5rem;
	}
	.legal-page :global(.risk-box p) {
		margin: 0;
		color: #7f1d1d;
		line-height: 1.7;
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

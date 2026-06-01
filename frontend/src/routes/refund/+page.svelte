<!--
	URL: /refund
-->
<script lang="ts">
	const lastUpdated = 'June 1, 2026';
</script>

<svelte:head>
	<title>Refund Policy | Revolution Trading Pros</title>
	<meta name="description" content="Revolution Trading Pros refund and cancellation policy. Understand your rights and our commitment to your satisfaction." />
</svelte:head>

<div class="legal-page">
	<div class="legal-inner">
		<div class="legal-header">
			<p class="legal-header__label">Legal</p>
			<h1 class="legal-header__title">Refund Policy</h1>
			<p class="legal-header__meta">Last updated: {lastUpdated}</p>
		</div>

		<div class="legal-body">
			<h2>7-Day Money-Back Guarantee</h2>
			<p>We stand behind the quality of our platform. If you are not satisfied with your subscription for any reason, you may request a full refund within <strong>7 days</strong> of your initial purchase. No questions asked.</p>
			<p>To request a refund within the 7-day window, contact our support team at <a href="mailto:support@revolutiontradingpros.com">support@revolutiontradingpros.com</a> with your account email and the reason for your request (optional). Refunds are processed within 5–10 business days back to your original payment method.</p>

			<h2>After the 7-Day Window</h2>
			<p>After the initial 7-day period, subscription charges are <strong>non-refundable</strong>. You will retain access to your current subscription tier until the end of your billing period.</p>
			<p>We do not offer prorated refunds for unused portions of a subscription period.</p>

			<h2>Cancellations</h2>
			<p>You may cancel your subscription at any time from your <a href="/dashboard/account/billing">billing settings</a> in your dashboard. Cancellation takes effect at the end of your current billing cycle — you will not be charged again, and you will retain access until the cycle ends.</p>

			<h2>Exceptions</h2>
			<p>Refund requests may be declined in cases of:</p>
			<ul>
				<li>Abuse of the refund policy (e.g., repeated purchases and refund requests)</li>
				<li>Violation of our <a href="/terms">Terms of Service</a></li>
				<li>Requests made after the 7-day window</li>
			</ul>

			<h2>Chargebacks</h2>
			<p>If you initiate a chargeback with your bank or credit card provider without first contacting us, your account will be suspended pending investigation. We encourage you to reach out to us directly — we will always work to resolve any issues fairly and promptly.</p>

			<h2>One-Time Purchases</h2>
			<p>Courses, workshops, and other one-time purchases are refundable within 7 days of purchase, provided less than 20% of the course content has been accessed.</p>

			<h2>Contact Us</h2>
			<p>For any refund-related questions, please contact our billing team at <a href="mailto:billing@revolutiontradingpros.com">billing@revolutiontradingpros.com</a> or visit our <a href="/contact">contact page</a>.</p>
		</div>
	</div>
</div>

<style>
	.legal-page {
		background: var(--rtp-bg);
		color: var(--rtp-text);
		min-height: 100vh;
		padding: 5rem 1.5rem;
	}
	.legal-inner {
		max-width: 780px;
		margin: 0 auto;
	}
	.legal-header {
		margin-bottom: 3rem;
		padding-bottom: 2rem;
		border-bottom: 1px solid var(--rtp-border-soft);
	}
	.legal-header__label {
		font-size: 0.8125rem;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--rtp-primary);
		margin: 0 0 0.75rem;
	}
	.legal-header__title {
		font-size: clamp(1.75rem, 4vw, 2.5rem);
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 0.75rem;
		line-height: 1.2;
	}
	.legal-header__meta {
		font-size: 0.875rem;
		color: var(--rtp-text-subtle);
		margin: 0;
	}

	h2 {
		font-size: 1.125rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 2.5rem 0 0.75rem;
	}
	p {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		line-height: 1.8;
		margin: 0 0 1rem;
	}
	ul {
		padding-left: 1.5rem;
		margin: 0 0 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}
	li {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		line-height: 1.7;
	}
	strong { color: var(--rtp-text); }
	a {
		color: var(--rtp-indigo);
		text-decoration: underline;
		text-underline-offset: 3px;
	}
	a:hover { color: #818cf8; }
</style>

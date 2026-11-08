<script lang="ts">
	import InformationPage from '#lib/components/marketing/InformationPage.svelte';
	const lastUpdated = 'June 1, 2026';
</script>

<InformationPage title="Refund Policy" metadata={`Last updated: ${lastUpdated}`}>
	<h2>7-Day Money-Back Guarantee</h2>
	<p>
		We stand behind the quality of our platform. If you are not satisfied with your subscription for
		any reason, you may request a full refund within <strong>7 days</strong> of your initial purchase.
		No questions asked.
	</p>
	<p>
		To request a refund within the 7-day window, contact our support team at <a
			href="mailto:support@revolutiontradingpros.com">support@revolutiontradingpros.com</a
		> with your account email and the reason for your request (optional). Refunds are processed within
		5–10 business days back to your original payment method.
	</p>

	<h2>After the 7-Day Window</h2>
	<p>
		After the initial 7-day period, subscription charges are <strong>non-refundable</strong>. You
		will retain access to your current subscription tier until the end of your billing period.
	</p>
	<p>We do not offer prorated refunds for unused portions of a subscription period.</p>

	<h2>Cancellations</h2>
	<p>
		You may cancel your subscription at any time from your <a href="/dashboard/account/billing"
			>billing settings</a
		> in your dashboard. Cancellation takes effect at the end of your current billing cycle — you will
		not be charged again, and you will retain access until the cycle ends.
	</p>

	<h2>Exceptions</h2>
	<p>Refund requests may be declined in cases of:</p>
	<ul>
		<li>Abuse of the refund policy (e.g., repeated purchases and refund requests)</li>
		<li>Violation of our <a href="/terms">Terms of Service</a></li>
		<li>Requests made after the 7-day window</li>
	</ul>

	<h2>Chargebacks</h2>
	<p>
		If you initiate a chargeback with your bank or credit card provider without first contacting us,
		your account will be suspended pending investigation. We encourage you to reach out to us
		directly — we will always work to resolve any issues fairly and promptly.
	</p>

	<h2>One-Time Purchases</h2>
	<p>
		Courses, workshops, and other one-time purchases are refundable within 7 days of purchase,
		provided less than 20% of the course content has been accessed.
	</p>

	<h2>Contact Us</h2>
	<p>
		For any refund-related questions, please contact our billing team at <a
			href="mailto:billing@revolutiontradingpros.com">billing@revolutiontradingpros.com</a
		>
		or visit our <a href="/contact">contact page</a>.
	</p>
</InformationPage>

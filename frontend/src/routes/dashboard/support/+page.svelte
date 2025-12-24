<script lang="ts">
	/**
	 * Support Center Page - Member Dashboard
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Help center and support resources
	 *
	 * @version 1.0.0
	 */
	import IconHelpCircle from '@tabler/icons-svelte/icons/help-circle';
	import IconMail from '@tabler/icons-svelte/icons/mail';
	import IconPhone from '@tabler/icons-svelte/icons/phone';
	import IconMessageCircle from '@tabler/icons-svelte/icons/message-circle';
	import IconBook from '@tabler/icons-svelte/icons/book';
	import IconVideo from '@tabler/icons-svelte/icons/video';
</script>

<svelte:head>
	<title>Support Center | Revolution Trading Pros</title>
</svelte:head>

<div class="support-page">
	<!-- Page Header -->
	<header class="page-header">
		<h1 class="page-title">Support Center</h1>
		<p class="page-subtitle">We're here to help you succeed</p>
	</header>

	<!-- Contact Methods -->
	<section class="contact-section">
		<h2 class="section-title">Get in Touch</h2>
		<div class="contact-grid">
			<a href="mailto:support@revolutiontradingpros.com" class="contact-card">
				<div class="contact-icon email">
					<IconMail size={28} />
				</div>
				<h3>Email Support</h3>
				<p>support@revolutiontradingpros.com</p>
				<span class="contact-meta">Response within 24 hours</span>
			</a>

			<a href="tel:+15555551234" class="contact-card">
				<div class="contact-icon phone">
					<IconPhone size={28} />
				</div>
				<h3>Phone Support</h3>
				<p>(555) 555-1234</p>
				<span class="contact-meta">Mon-Fri, 9AM-5PM EST</span>
			</a>

			<button class="contact-card" onclick={() => alert('Live chat coming soon!')}>
				<div class="contact-icon chat">
					<IconMessageCircle size={28} />
				</div>
				<h3>Live Chat</h3>
				<p>Chat with our team</p>
				<span class="contact-meta">Available during business hours</span>
			</button>
		</div>
	</section>

	<!-- Resources -->
	<section class="resources-section">
		<h2 class="section-title">Help Resources</h2>
		<div class="resources-grid">
			<article class="resource-card">
				<div class="resource-icon">
					<IconBook size={24} />
				</div>
				<div class="resource-content">
					<h3>Knowledge Base</h3>
					<p>Browse articles and guides to get the most out of your membership</p>
					<button class="resource-link" onclick={() => alert('Knowledge Base coming soon!')}>Browse Articles →</button>
				</div>
			</article>

			<article class="resource-card">
				<div class="resource-icon">
					<IconVideo size={24} />
				</div>
				<div class="resource-content">
					<h3>Video Tutorials</h3>
					<p>Watch step-by-step tutorials on platform features and trading strategies</p>
					<button class="resource-link" onclick={() => alert('Video Tutorials coming soon!')}>Watch Videos →</button>
				</div>
			</article>

			<article class="resource-card">
				<div class="resource-icon">
					<IconHelpCircle size={24} />
				</div>
				<div class="resource-content">
					<h3>FAQs</h3>
					<p>Find quick answers to commonly asked questions</p>
					<button class="resource-link" onclick={() => alert('FAQs coming soon!')}>View FAQs →</button>
				</div>
			</article>
		</div>
	</section>

	<!-- Common Topics -->
	<section class="topics-section">
		<h2 class="section-title">Common Topics</h2>
		<div class="topics-list">
			<button class="topic-item">
				<span class="topic-title">Getting Started</span>
				<span class="topic-arrow">→</span>
			</button>
			<button class="topic-item">
				<span class="topic-title">Account & Billing</span>
				<span class="topic-arrow">→</span>
			</button>
			<button class="topic-item">
				<span class="topic-title">Trading Room Access</span>
				<span class="topic-arrow">→</span>
			</button>
			<button class="topic-item">
				<span class="topic-title">Technical Issues</span>
				<span class="topic-arrow">→</span>
			</button>
			<button class="topic-item">
				<span class="topic-title">Indicator Installation</span>
				<span class="topic-arrow">→</span>
			</button>
			<button class="topic-item">
				<span class="topic-title">Course Access</span>
				<span class="topic-arrow">→</span>
			</button>
		</div>
	</section>
</div>

<style>
	.support-page {
		padding: 30px;
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		text-align: center;
		margin-bottom: 48px;
	}

	.page-title {
		font-size: 32px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.page-subtitle {
		font-size: 16px;
		color: #666;
		margin: 0;
	}

	.section-title {
		font-size: 20px;
		font-weight: 600;
		color: #333;
		margin: 0 0 24px;
	}

	/* Contact Section */
	.contact-section {
		margin-bottom: 48px;
	}

	.contact-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 24px;
	}

	.contact-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		padding: 32px 24px;
		background: #fff;
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		text-decoration: none;
		color: inherit;
		cursor: pointer;
		transition: all 0.15s;
		border: 2px solid transparent;
	}

	.contact-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
		border-color: #0984ae;
	}

	.contact-icon {
		width: 64px;
		height: 64px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 16px;
		color: #fff;
	}

	.contact-icon.email {
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
	}

	.contact-icon.phone {
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
	}

	.contact-icon.chat {
		background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
	}

	.contact-card h3 {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.contact-card p {
		font-size: 14px;
		color: #0984ae;
		font-weight: 500;
		margin: 0 0 8px;
	}

	.contact-meta {
		font-size: 12px;
		color: #6b7280;
	}

	/* Resources Section */
	.resources-section {
		margin-bottom: 48px;
	}

	.resources-grid {
		display: grid;
		gap: 20px;
	}

	.resource-card {
		display: flex;
		gap: 20px;
		padding: 24px;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		transition: all 0.15s;
	}

	.resource-card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
	}

	.resource-icon {
		width: 48px;
		height: 48px;
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		flex-shrink: 0;
	}

	.resource-content {
		flex: 1;
	}

	.resource-content h3 {
		font-size: 18px;
		font-weight: 600;
		color: #333;
		margin: 0 0 8px;
	}

	.resource-content p {
		font-size: 14px;
		color: #666;
		margin: 0 0 12px;
		line-height: 1.5;
	}

	.resource-link {
		font-size: 14px;
		color: #0984ae;
		font-weight: 600;
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		transition: color 0.15s;
	}

	.resource-link:hover {
		color: #076787;
	}

	/* Topics Section */
	.topics-section {
		margin-bottom: 48px;
	}

	.topics-list {
		display: grid;
		gap: 12px;
	}

	.topic-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 16px 20px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.topic-item:hover {
		background: #f9fafb;
		border-color: #0984ae;
	}

	.topic-title {
		font-size: 14px;
		font-weight: 500;
		color: #333;
	}

	.topic-arrow {
		color: #0984ae;
		font-weight: 600;
	}

	@media (max-width: 768px) {
		.support-page {
			padding: 20px;
		}

		.contact-grid {
			grid-template-columns: 1fr;
		}
	}
</style>

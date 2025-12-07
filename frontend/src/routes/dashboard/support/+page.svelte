<script lang="ts">
	/**
	 * Dashboard - Support Page
	 * Follows WordPress Simpler Trading patterns exactly
	 */
	import SEOHead from '$lib/components/SEOHead.svelte';

	// Support categories
	const categories = [
		{
			id: 'getting-started',
			icon: 'st-icon-learning-center',
			title: 'Getting Started',
			description: 'New member guides and orientation',
			articles: [
				{ title: 'How to access the trading room', url: '#' },
				{ title: 'Setting up your platform', url: '#' },
				{ title: 'Understanding your membership', url: '#' },
				{ title: 'First trade walkthrough', url: '#' }
			]
		},
		{
			id: 'account-billing',
			icon: 'st-icon-settings',
			title: 'Account & Billing',
			description: 'Manage your account and payments',
			articles: [
				{ title: 'Update payment method', url: '#' },
				{ title: 'Change subscription plan', url: '#' },
				{ title: 'View billing history', url: '#' },
				{ title: 'Cancel or pause membership', url: '#' }
			]
		},
		{
			id: 'indicators',
			icon: 'st-icon-indicators',
			title: 'Indicators & Tools',
			description: 'Installation and usage guides',
			articles: [
				{ title: 'Installing ThinkOrSwim indicators', url: '#' },
				{ title: 'TradingView setup guide', url: '#' },
				{ title: 'Indicator troubleshooting', url: '#' },
				{ title: 'Platform compatibility', url: '#' }
			]
		},
		{
			id: 'trading-room',
			icon: 'st-icon-dashboard',
			title: 'Trading Room',
			description: 'Live room questions and issues',
			articles: [
				{ title: 'Trading room schedule', url: '#' },
				{ title: 'Audio/video troubleshooting', url: '#' },
				{ title: 'Chat features guide', url: '#' },
				{ title: 'Recording access', url: '#' }
			]
		}
	];

	// FAQ items
	const faqs = [
		{
			question: 'What are the trading room hours?',
			answer: 'Our live trading room is open Monday through Friday from 9:00 AM to 4:00 PM Eastern Time. Pre-market analysis begins at 8:30 AM ET.'
		},
		{
			question: 'How do I install the indicators on my platform?',
			answer: 'Each indicator comes with detailed installation instructions. Navigate to Dashboard > My Indicators, click on the indicator, and follow the step-by-step guide for your specific platform (ThinkOrSwim, TradingView, etc.).'
		},
		{
			question: 'Can I access past trading room sessions?',
			answer: 'Yes! All trading room sessions are recorded and available in the Archive section of your membership dashboard within 2 hours of the session ending.'
		},
		{
			question: 'How do I cancel or pause my membership?',
			answer: 'You can manage your subscription from Dashboard > Account > Billing. Click "Manage Subscription" to pause, cancel, or modify your plan.'
		},
		{
			question: 'What devices can I use to access the trading room?',
			answer: 'Our trading room is accessible on desktop, laptop, tablet, and mobile devices. We recommend using Chrome or Firefox for the best experience.'
		}
	];

	let expandedFaq = $state<number | null>(null);
	let searchQuery = $state('');
	let contactForm = $state({
		name: '',
		email: '',
		subject: '',
		message: ''
	});
	let isSubmitting = $state(false);
	let submitSuccess = $state(false);

	function toggleFaq(index: number) {
		expandedFaq = expandedFaq === index ? null : index;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		isSubmitting = true;
		// Simulate API call
		await new Promise(resolve => setTimeout(resolve, 1500));
		isSubmitting = false;
		submitSuccess = true;
		contactForm = { name: '', email: '', subject: '', message: '' };
		setTimeout(() => submitSuccess = false, 5000);
	}
</script>

<SEOHead
	title="Support - Dashboard"
	description="Get help with your Revolution Trading Pros membership. Browse FAQs, guides, and contact our support team."
	noindex
/>

<div class="dashboard__page">
	<!-- Page Header -->
	<header class="dashboard__header">
		<div class="dashboard__header-left">
			<nav class="dashboard__breadcrumb">
				<a href="/dashboard">Dashboard</a>
				<span class="separator">/</span>
				<span class="current">Support</span>
			</nav>
			<h1 class="dashboard__page-title">
				<span class="dashboard__nav-item-icon st-icon-support"></span>
				Support Center
			</h1>
		</div>
	</header>

	<!-- Search Section -->
	<section class="support__search">
		<div class="support__search-container">
			<h2 class="support__search-title">How can we help you?</h2>
			<div class="support__search-input-wrapper">
				<span class="support__search-icon st-icon-search"></span>
				<input
					type="text"
					class="support__search-input"
					placeholder="Search for help articles..."
					bind:value={searchQuery}
				/>
			</div>
		</div>
	</section>

	<!-- Categories Section -->
	<section class="dashboard__content-section">
		<h2 class="dashboard__section-title">Browse by Topic</h2>
		<div class="row">
			{#each categories as category}
				<div class="col-sm-6 col-xl-3">
					<div class="support__category-card">
						<span class="support__category-icon {category.icon}"></span>
						<h3 class="support__category-title">{category.title}</h3>
						<p class="support__category-description">{category.description}</p>
						<ul class="support__category-articles">
							{#each category.articles as article}
								<li>
									<a href={article.url}>{article.title}</a>
								</li>
							{/each}
						</ul>
						<a href="/dashboard/support/{category.id}" class="support__category-link">
							View all articles
							<span class="arrow">→</span>
						</a>
					</div>
				</div>
			{/each}
		</div>
	</section>

	<!-- FAQ Section -->
	<section class="dashboard__content-section">
		<h2 class="dashboard__section-title">Frequently Asked Questions</h2>
		<div class="support__faq-list">
			{#each faqs as faq, index}
				<div class="support__faq-item" class:expanded={expandedFaq === index}>
					<button class="support__faq-question" onclick={() => toggleFaq(index)}>
						<span>{faq.question}</span>
						<span class="support__faq-toggle">{expandedFaq === index ? '−' : '+'}</span>
					</button>
					{#if expandedFaq === index}
						<div class="support__faq-answer">
							<p>{faq.answer}</p>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</section>

	<!-- Contact Section -->
	<section class="dashboard__content-section">
		<div class="support__contact">
			<div class="support__contact-info">
				<h2 class="support__contact-title">Still Need Help?</h2>
				<p class="support__contact-description">
					Our support team is available Monday through Friday, 9 AM - 5 PM ET.
					We typically respond within 24 hours.
				</p>
				<div class="support__contact-methods">
					<div class="support__contact-method">
						<span class="support__contact-method-icon st-icon-mail"></span>
						<div>
							<h4>Email Support</h4>
							<p>support@revolutiontradingpros.com</p>
						</div>
					</div>
					<div class="support__contact-method">
						<span class="support__contact-method-icon st-icon-chat"></span>
						<div>
							<h4>Live Chat</h4>
							<p>Available during market hours</p>
						</div>
					</div>
				</div>
			</div>
			<div class="support__contact-form-wrapper">
				<h3 class="support__form-title">Send us a Message</h3>
				{#if submitSuccess}
					<div class="support__success-message">
						<span class="success-icon">✓</span>
						<p>Thank you! We've received your message and will respond within 24 hours.</p>
					</div>
				{:else}
					<form class="support__contact-form" onsubmit={handleSubmit}>
						<div class="form-group">
							<label for="name">Name</label>
							<input type="text" id="name" bind:value={contactForm.name} required />
						</div>
						<div class="form-group">
							<label for="email">Email</label>
							<input type="email" id="email" bind:value={contactForm.email} required />
						</div>
						<div class="form-group">
							<label for="subject">Subject</label>
							<select id="subject" bind:value={contactForm.subject} required>
								<option value="">Select a topic...</option>
								<option value="account">Account & Billing</option>
								<option value="indicators">Indicators</option>
								<option value="trading-room">Trading Room</option>
								<option value="courses">Courses</option>
								<option value="other">Other</option>
							</select>
						</div>
						<div class="form-group">
							<label for="message">Message</label>
							<textarea id="message" rows="4" bind:value={contactForm.message} required></textarea>
						</div>
						<button type="submit" class="btn btn-orange btn-block" disabled={isSubmitting}>
							{isSubmitting ? 'Sending...' : 'Send Message'}
						</button>
					</form>
				{/if}
			</div>
		</div>
	</section>
</div>

<style>
	.dashboard__page {
		padding: 0;
	}

	/* Header */
	.dashboard__header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		margin-bottom: 2rem;
		flex-wrap: wrap;
		gap: 1rem;
	}

	.dashboard__breadcrumb {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin-bottom: 0.5rem;
	}

	.dashboard__breadcrumb a {
		color: var(--st-color-gray-400, #9ca3af);
		text-decoration: none;
	}

	.dashboard__breadcrumb a:hover {
		color: var(--st-color-orange, #f97316);
	}

	.dashboard__breadcrumb .separator {
		margin: 0 0.5rem;
	}

	.dashboard__breadcrumb .current {
		color: var(--st-color-white, #fff);
	}

	.dashboard__page-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.dashboard__page-title .st-icon-support::before {
		content: "?";
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 1.5rem;
		height: 1.5rem;
		background: var(--st-color-orange, #f97316);
		border-radius: 50%;
		font-size: 1rem;
		font-weight: 700;
	}

	/* Search Section */
	.support__search {
		background: linear-gradient(135deg, var(--st-color-orange, #f97316) 0%, #ea580c 100%);
		border-radius: 1rem;
		padding: 3rem 2rem;
		margin-bottom: 2rem;
		text-align: center;
	}

	.support__search-title {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--st-color-white, #fff);
		margin-bottom: 1.5rem;
	}

	.support__search-input-wrapper {
		position: relative;
		max-width: 600px;
		margin: 0 auto;
	}

	.support__search-icon {
		position: absolute;
		left: 1.25rem;
		top: 50%;
		transform: translateY(-50%);
		color: var(--st-color-gray-400, #9ca3af);
	}

	.support__search-icon::before {
		content: "🔍";
	}

	.support__search-input {
		width: 100%;
		padding: 1rem 1rem 1rem 3rem;
		font-size: 1rem;
		border: none;
		border-radius: 8px;
		background: var(--st-color-white, #fff);
		color: var(--st-color-gray-900, #111827);
	}

	.support__search-input::placeholder {
		color: var(--st-color-gray-400, #9ca3af);
	}

	/* Content Sections */
	.dashboard__content-section {
		margin-bottom: 2.5rem;
	}

	.dashboard__section-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 1.5rem;
	}

	/* Grid */
	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -0.75rem;
	}

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 0.75rem;
		margin-bottom: 1.5rem;
	}

	.col-xl-3 {
		flex: 0 0 25%;
		max-width: 25%;
	}

	/* Category Cards */
	.support__category-card {
		background: var(--st-color-gray-800, #1f2937);
		border: 1px solid var(--st-color-gray-700, #374151);
		border-radius: 1rem;
		padding: 1.5rem;
		height: 100%;
		transition: all 0.2s ease;
	}

	.support__category-card:hover {
		border-color: var(--st-color-orange, #f97316);
		transform: translateY(-2px);
	}

	.support__category-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 3rem;
		height: 3rem;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 0.75rem;
		color: var(--st-color-orange, #f97316);
		font-size: 1.5rem;
		margin-bottom: 1rem;
	}

	.support__category-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.5rem;
	}

	.support__category-description {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin-bottom: 1rem;
	}

	.support__category-articles {
		list-style: none;
		padding: 0;
		margin: 0 0 1rem;
	}

	.support__category-articles li {
		padding: 0.5rem 0;
		border-bottom: 1px solid var(--st-color-gray-700, #374151);
	}

	.support__category-articles li:last-child {
		border-bottom: none;
	}

	.support__category-articles a {
		font-size: 0.875rem;
		color: var(--st-color-gray-300, #d1d5db);
		text-decoration: none;
	}

	.support__category-articles a:hover {
		color: var(--st-color-orange, #f97316);
	}

	.support__category-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--st-color-orange, #f97316);
		text-decoration: none;
	}

	.support__category-link:hover {
		text-decoration: underline;
	}

	/* FAQ Section */
	.support__faq-list {
		max-width: 800px;
	}

	.support__faq-item {
		background: var(--st-color-gray-800, #1f2937);
		border: 1px solid var(--st-color-gray-700, #374151);
		border-radius: 0.75rem;
		margin-bottom: 0.75rem;
		overflow: hidden;
	}

	.support__faq-item.expanded {
		border-color: var(--st-color-orange, #f97316);
	}

	.support__faq-question {
		width: 100%;
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1.25rem;
		background: transparent;
		border: none;
		color: var(--st-color-white, #fff);
		font-size: 1rem;
		font-weight: 500;
		text-align: left;
		cursor: pointer;
	}

	.support__faq-toggle {
		font-size: 1.5rem;
		color: var(--st-color-orange, #f97316);
	}

	.support__faq-answer {
		padding: 0 1.25rem 1.25rem;
	}

	.support__faq-answer p {
		font-size: 0.9375rem;
		color: var(--st-color-gray-300, #d1d5db);
		line-height: 1.6;
		margin: 0;
	}

	/* Contact Section */
	.support__contact {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 3rem;
		background: var(--st-color-gray-800, #1f2937);
		border-radius: 1rem;
		padding: 2rem;
	}

	.support__contact-title {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 1rem;
	}

	.support__contact-description {
		font-size: 1rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin-bottom: 2rem;
		line-height: 1.6;
	}

	.support__contact-methods {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.support__contact-method {
		display: flex;
		gap: 1rem;
		align-items: flex-start;
	}

	.support__contact-method-icon {
		flex-shrink: 0;
		width: 2.5rem;
		height: 2.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(249, 115, 22, 0.1);
		border-radius: 0.5rem;
		color: var(--st-color-orange, #f97316);
		font-size: 1.25rem;
	}

	.support__contact-method h4 {
		font-size: 1rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 0.25rem;
	}

	.support__contact-method p {
		font-size: 0.875rem;
		color: var(--st-color-gray-400, #9ca3af);
		margin: 0;
	}

	/* Contact Form */
	.support__form-title {
		font-size: 1.25rem;
		font-weight: 600;
		color: var(--st-color-white, #fff);
		margin-bottom: 1.5rem;
	}

	.support__contact-form {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--st-color-gray-300, #d1d5db);
	}

	.form-group input,
	.form-group select,
	.form-group textarea {
		padding: 0.875rem 1rem;
		background: var(--st-color-gray-700, #374151);
		border: 1px solid var(--st-color-gray-600, #4b5563);
		border-radius: 8px;
		color: var(--st-color-white, #fff);
		font-size: 0.9375rem;
	}

	.form-group input:focus,
	.form-group select:focus,
	.form-group textarea:focus {
		outline: none;
		border-color: var(--st-color-orange, #f97316);
	}

	.form-group textarea {
		resize: vertical;
		min-height: 100px;
	}

	/* Success Message */
	.support__success-message {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		padding: 2rem;
		background: rgba(34, 197, 94, 0.1);
		border: 1px solid rgba(34, 197, 94, 0.3);
		border-radius: 1rem;
	}

	.support__success-message .success-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 3rem;
		height: 3rem;
		background: var(--st-color-green, #22c55e);
		border-radius: 50%;
		color: var(--st-color-white, #fff);
		font-size: 1.5rem;
		margin-bottom: 1rem;
	}

	.support__success-message p {
		color: var(--st-color-green, #22c55e);
		margin: 0;
	}

	/* Buttons */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 0.875rem 1.75rem;
		font-size: 1rem;
		font-weight: 600;
		text-decoration: none;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-orange {
		background: linear-gradient(135deg, var(--st-color-orange, #f97316), #ea580c);
		color: var(--st-color-white, #fff);
	}

	.btn-orange:hover:not(:disabled) {
		transform: translateY(-2px);
		box-shadow: 0 10px 20px rgba(249, 115, 22, 0.3);
	}

	.btn-orange:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.btn-block {
		width: 100%;
	}

	/* Responsive */
	@media (max-width: 1199px) {
		.col-xl-3 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (max-width: 767px) {
		.col-sm-6, .col-xl-3 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.support__contact {
			grid-template-columns: 1fr;
		}

		.support__search {
			padding: 2rem 1rem;
		}
	}
</style>

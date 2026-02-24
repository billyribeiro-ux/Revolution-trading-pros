<!--
	URL: /test-backend
-->

<script lang="ts">
import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import { submitForm } from '$lib/api/forms';
	import { popupsApi, recordPopupImpression } from '$lib/api/popups';
	import { IconMail, IconUser, IconCheck } from '$lib/icons';

	// Form state
	let formData = $state({
		name: '',
		email: '',
		message: ''
	});
	let formStatus: 'idle' | 'submitting' | 'success' | 'error' = $state('idle');
	let formMessage = $state('');
	let formErrors: Record<string, string[]> = $state({});

	// Popup state
	let showPopup = $state(false);
	let popupEmail = $state('');
	let popupStatus: 'idle' | 'submitting' | 'success' | 'error' = $state('idle');
	let popupMessage = $state('');

	// Newsletter form
	let newsletterEmail = $state('');
	let newsletterStatus: 'idle' | 'submitting' | 'success' | 'error' = $state('idle');
	let newsletterMessage = $state('');

	// Load active popups on mount
	onMount(async () => {
		try {
			const popups = await popupsApi.getActive('/test-backend');
			if (popups.length > 0) {
				// Show popup after 3 seconds
				setTimeout(() => {
					showPopup = true;
					recordPopupImpression(popups[0].id);
				}, 3000);
			}
		} catch (error) {
			logger.error('Error loading popups:', error);
		}
	});

	async function handleContactSubmit(e: Event) {
		e.preventDefault();
		formStatus = 'submitting';
		formErrors = {};
		formMessage = '';

		try {
			// Note: You'll need to create a contact form in your backend first
			// For now, this will test the API connection
			const result = await submitForm('contact', formData);

			if (result.success) {
				formStatus = 'success';
				formMessage = result.message || 'Thank you! Your message has been sent successfully.';
				formData = { name: '', email: '', message: '' };
			} else {
				formStatus = 'error';
				formErrors = result.errors || {};
				formMessage = 'Please fix the errors below.';
			}
		} catch (error) {
			formStatus = 'error';
			formMessage =
				error instanceof Error ? error.message : 'Failed to submit form. Please try again.';
		}
	}

	async function handleNewsletterSubmit(e: Event) {
		e.preventDefault();
		newsletterStatus = 'submitting';
		newsletterMessage = '';

		try {
			const response = await fetch(
				'https://revolution-trading-pros-api.fly.dev/api/newsletter/subscribe',
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Accept: 'application/json'
					},
					body: JSON.stringify({ email: newsletterEmail })
				}
			);

			const data = await response.json();

			if (response.ok) {
				newsletterStatus = 'success';
				newsletterMessage = data.message || 'Successfully subscribed to newsletter!';
				newsletterEmail = '';
			} else {
				newsletterStatus = 'error';
				newsletterMessage = data.message || 'Failed to subscribe. Please try again.';
			}
		} catch (_error) {
			newsletterStatus = 'error';
			newsletterMessage = 'Network error. Please try again.';
		}
	}

	async function handlePopupSubmit(e: Event) {
		e.preventDefault();
		popupStatus = 'submitting';
		popupMessage = '';

		try {
			const response = await fetch(
				'https://revolution-trading-pros-api.fly.dev/api/newsletter/subscribe',
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Accept: 'application/json'
					},
					body: JSON.stringify({ email: popupEmail })
				}
			);

			const data = await response.json();

			if (response.ok) {
				popupStatus = 'success';
				popupMessage = data.message || 'Successfully subscribed!';
				popupEmail = '';
				// Record conversion
				// await recordPopupConversion('popup-id', { action: 'newsletter_signup' });
				setTimeout(() => {
					showPopup = false;
				}, 2000);
			} else {
				popupStatus = 'error';
				popupMessage = data.message || 'Failed to subscribe.';
			}
		} catch (_error) {
			popupStatus = 'error';
			popupMessage = 'Network error. Please try again.';
		}
	}

	function closePopup() {
		showPopup = false;
	}

	// SEO Schema
	const pageSchema = {
		'@context': 'https://schema.org',
		'@type': 'WebPage',
		name: 'Backend Test Page',
		description: 'Testing forms, popups, and SEO integration with Laravel backend',
		url: 'https://revolution-trading-pros.pages.dev/test-backend'
	};
</script>

<div class="test-page">
	<div class="container">
		<!-- Header -->
		<header class="page-header">
			<h1>Backend Integration Test</h1>
			<p class="subtitle">Testing Forms, Popups, and SEO Features</p>
		</header>

		<!-- Status Cards -->
		<div class="status-grid">
			<div class="status-card">
				<div class="status-icon success">
					<IconCheck size={24} />
				</div>
				<h3>Backend Connected</h3>
				<p>API running on revolution-trading-pros-api.fly.dev</p>
			</div>

			<div class="status-card">
				<div class="status-icon success">
					<IconCheck size={24} />
				</div>
				<h3>SEO Configured</h3>
				<p>Meta tags and structured data active</p>
			</div>

			<div class="status-card">
				<div class="status-icon success">
					<IconCheck size={24} />
				</div>
				<h3>Forms Ready</h3>
				<p>Contact and newsletter forms active</p>
			</div>
		</div>

		<!-- Newsletter Section -->
		<section class="section newsletter-section">
			<div class="section-content">
				<h2>Newsletter Signup Test</h2>
				<p>Test the newsletter subscription endpoint</p>

				<form class="newsletter-form" onsubmit={handleNewsletterSubmit}>
					<div class="form-group">
						<input
							id="page-newsletteremail"
							autocomplete="email"
							name="page-newsletteremail"
							type="email"
							bind:value={newsletterEmail}
							placeholder="Enter your email"
							required
							disabled={newsletterStatus === 'submitting'}
							class="input-field"
						/>
						<button type="submit" disabled={newsletterStatus === 'submitting'} class="btn-primary">
							{#if newsletterStatus === 'submitting'}
								Subscribing...
							{:else}
								Subscribe
							{/if}
						</button>
					</div>

					{#if newsletterMessage}
						<div
							class="message"
							class:success={newsletterStatus === 'success'}
							class:error={newsletterStatus === 'error'}
						>
							{newsletterMessage}
						</div>
					{/if}
				</form>
			</div>
		</section>

		<!-- Contact Form Section -->
		<section class="section contact-section">
			<div class="section-content">
				<h2>Contact Form Test</h2>
				<p>Test form submission and validation</p>

				<form class="contact-form" onsubmit={handleContactSubmit}>
					<div class="form-row">
						<div class="form-group">
							<label for="name">
								<IconUser size={18} />
								Name
							</label>
							<input
								id="name"
								name="name"
								type="text"
								bind:value={formData.name}
								placeholder="Your name"
								required
								disabled={formStatus === 'submitting'}
								class="input-field"
								class:error={formErrors.name}
							/>
							{#if formErrors.name}
								<span class="error-text">{formErrors.name[0]}</span>
							{/if}
						</div>

						<div class="form-group">
							<label for="email">
								<IconMail size={18} />
								Email
							</label>
							<input
								id="email"
								name="email"
								autocomplete="email"
								type="email"
								bind:value={formData.email}
								placeholder="your@email.com"
								required
								disabled={formStatus === 'submitting'}
								class="input-field"
								class:error={formErrors.email}
							/>
							{#if formErrors.email}
								<span class="error-text">{formErrors.email[0]}</span>
							{/if}
						</div>
					</div>

					<div class="form-group">
						<label for="message"> Message </label>
						<textarea
							id="message"
							bind:value={formData.message}
							placeholder="Your message..."
							required
							disabled={formStatus === 'submitting'}
							class="input-field"
							class:error={formErrors.message}
							rows="5"
						></textarea>
						{#if formErrors.message}
							<span class="error-text">{formErrors.message[0]}</span>
						{/if}
					</div>

					{#if formMessage}
						<div
							class="message"
							class:success={formStatus === 'success'}
							class:error={formStatus === 'error'}
						>
							{formMessage}
						</div>
					{/if}

					<button type="submit" disabled={formStatus === 'submitting'} class="btn-submit">
						{#if formStatus === 'submitting'}
							Sending...
						{:else}
							Send Message
						{/if}
					</button>
				</form>
			</div>
		</section>

		<!-- API Endpoints Info -->
		<section class="section api-section">
			<div class="section-content">
				<h2>Available API Endpoints</h2>
				<div class="endpoints-grid">
					<div class="endpoint-card">
						<span class="method post">POST</span>
						<code>/api/newsletter/subscribe</code>
						<p>Subscribe to newsletter</p>
					</div>
					<div class="endpoint-card">
						<span class="method get">GET</span>
						<code>/api/posts</code>
						<p>Get blog posts</p>
					</div>
					<div class="endpoint-card">
						<span class="method post">POST</span>
						<code>/api/forms/:slug/submit</code>
						<p>Submit a form</p>
					</div>
					<div class="endpoint-card">
						<span class="method post">POST</span>
						<code>/api/register</code>
						<p>User registration</p>
					</div>
					<div class="endpoint-card">
						<span class="method post">POST</span>
						<code>/api/login</code>
						<p>User login</p>
					</div>
					<div class="endpoint-card">
						<span class="method get">GET</span>
						<code>/api/indicators</code>
						<p>Get indicators</p>
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<!-- Newsletter Popup -->
{#if showPopup}
	<div
		class="popup-overlay"
		onclick={closePopup}
		onkeydown={(e: KeyboardEvent) => e.key === 'Escape' && closePopup()}
		role="button"
		tabindex="0"
	>
		<div
			class="popup-content"
			onclick={(e: MouseEvent) => e.stopPropagation()}
			onkeydown={(e: KeyboardEvent) => e.stopPropagation()}
			role="dialog"
			tabindex="-1"
		>
			<button class="popup-close" onclick={closePopup} aria-label="Close">×</button>

			<div class="popup-header">
				<h3>🎉 Special Offer!</h3>
				<p>Subscribe to our newsletter and get exclusive trading insights</p>
			</div>

			<form class="popup-form" onsubmit={handlePopupSubmit}>
				<input
					id="page-popupemail"
					autocomplete="email"
					name="page-popupemail"
					type="email"
					bind:value={popupEmail}
					placeholder="Enter your email"
					required
					disabled={popupStatus === 'submitting'}
					class="popup-input"
				/>

				{#if popupMessage}
					<div
						class="popup-message"
						class:success={popupStatus === 'success'}
						class:error={popupStatus === 'error'}
					>
						{popupMessage}
					</div>
				{/if}

				<button type="submit" disabled={popupStatus === 'submitting'} class="popup-button">
					{#if popupStatus === 'submitting'}
						Subscribing...
					{:else}
						Subscribe Now
					{/if}
				</button>
			</form>
		</div>
	</div>
{/if}

<style>
	.test-page {
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
		padding: 4rem 2rem;
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.page-header {
		text-align: center;
		margin-bottom: 4rem;
	}

	.page-header h1 {
		font-size: clamp(2.5rem, 5vw, 4rem);
		font-weight: 800;
		background: linear-gradient(135deg, #60a5fa, #a78bfa, #ec4899);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		margin-bottom: 1rem;
	}

	.subtitle {
		font-size: 1.25rem;
		color: #94a3b8;
	}

	.status-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 2rem;
		margin-bottom: 4rem;
	}

	.status-card {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(96, 165, 250, 0.2);
		border-radius: 16px;
		padding: 2rem;
		text-align: center;
		backdrop-filter: blur(10px);
	}

	.status-icon {
		width: 60px;
		height: 60px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0 auto 1rem;
	}

	.status-icon.success {
		background: linear-gradient(135deg, #10b981, #059669);
		color: white;
	}

	.status-card h3 {
		font-size: 1.25rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.status-card p {
		color: #94a3b8;
		font-size: 0.95rem;
	}

	.section {
		margin-bottom: 3rem;
	}

	.section-content {
		background: rgba(30, 41, 59, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 16px;
		padding: 3rem;
		backdrop-filter: blur(10px);
	}

	.section-content h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #f1f5f9;
		margin-bottom: 0.5rem;
	}

	.section-content > p {
		color: #94a3b8;
		margin-bottom: 2rem;
	}

	/* Newsletter Form */
	.newsletter-form {
		max-width: 600px;
	}

	.newsletter-form .form-group {
		display: flex;
		gap: 1rem;
	}

	/* Contact Form */
	.contact-form {
		max-width: 800px;
	}

	.form-row {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.form-group {
		margin-bottom: 1.5rem;
	}

	.form-group label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-weight: 600;
		color: #cbd5e1;
		margin-bottom: 0.5rem;
		font-size: 0.95rem;
	}

	.input-field {
		width: 100%;
		padding: 0.875rem 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 1rem;
		transition: all 0.3s;
	}

	.input-field:focus {
		outline: none;
		border-color: #60a5fa;
		box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.1);
	}

	.input-field.error {
		border-color: #ef4444;
	}

	.input-field:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	textarea.input-field {
		resize: vertical;
		min-height: 120px;
	}

	.error-text {
		display: block;
		color: #f87171;
		font-size: 0.875rem;
		margin-top: 0.5rem;
	}

	.message {
		padding: 1rem;
		border-radius: 8px;
		margin-top: 1rem;
		font-weight: 500;
	}

	.message.success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.message.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.btn-primary,
	.btn-submit {
		padding: 0.875rem 2rem;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
		white-space: nowrap;
	}

	.btn-primary:hover,
	.btn-submit:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.btn-primary:disabled,
	.btn-submit:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.btn-submit {
		width: 100%;
		max-width: 300px;
	}

	/* API Endpoints */
	.endpoints-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 1rem;
	}

	.endpoint-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(148, 163, 184, 0.2);
		border-radius: 12px;
		padding: 1.5rem;
	}

	.method {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 6px;
		font-size: 0.75rem;
		font-weight: 700;
		text-transform: uppercase;
		margin-bottom: 0.75rem;
	}

	.method.get {
		background: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.method.post {
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.endpoint-card code {
		display: block;
		color: #a78bfa;
		font-size: 0.875rem;
		margin-bottom: 0.5rem;
		word-break: break-all;
	}

	.endpoint-card p {
		color: #94a3b8;
		font-size: 0.875rem;
		margin: 0;
	}

	/* Popup */
	.popup-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		animation: fadeIn 0.3s ease;
	}

	.popup-content {
		background: linear-gradient(135deg, #1e293b, #0f172a);
		border: 1px solid rgba(96, 165, 250, 0.3);
		border-radius: 20px;
		padding: 3rem;
		max-width: 500px;
		width: 90%;
		position: relative;
		animation: slideUp 0.3s ease;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
	}

	.popup-close {
		position: absolute;
		top: 1rem;
		right: 1rem;
		background: none;
		border: none;
		color: #94a3b8;
		font-size: 2rem;
		cursor: pointer;
		transition: color 0.3s;
		line-height: 1;
		padding: 0;
		width: 32px;
		height: 32px;
	}

	.popup-close:hover {
		color: #f1f5f9;
	}

	.popup-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.popup-header h3 {
		font-size: 2rem;
		font-weight: 800;
		color: #f1f5f9;
		margin-bottom: 0.75rem;
	}

	.popup-header p {
		color: #94a3b8;
		font-size: 1rem;
	}

	.popup-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.popup-input {
		padding: 1rem;
		background: rgba(15, 23, 42, 0.8);
		border: 1px solid rgba(148, 163, 184, 0.3);
		border-radius: 8px;
		color: #f1f5f9;
		font-size: 1rem;
	}

	.popup-input:focus {
		outline: none;
		border-color: #60a5fa;
	}

	.popup-message {
		padding: 0.75rem;
		border-radius: 8px;
		font-size: 0.875rem;
		text-align: center;
	}

	.popup-message.success {
		background: rgba(16, 185, 129, 0.1);
		border: 1px solid rgba(16, 185, 129, 0.3);
		color: #34d399;
	}

	.popup-message.error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #f87171;
	}

	.popup-button {
		padding: 1rem 2rem;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
		color: white;
		border: none;
		border-radius: 8px;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s;
	}

	.popup-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.popup-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes slideUp {
		from {
			transform: translateY(30px);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}

	@media (max-width: 768px) {
		.test-page {
			padding: 2rem 1rem;
		}

		.section-content {
			padding: 2rem 1.5rem;
		}

		.form-row {
			grid-template-columns: 1fr;
		}

		.newsletter-form .form-group {
			flex-direction: column;
		}

		.popup-content {
			padding: 2rem 1.5rem;
		}
	}
</style>

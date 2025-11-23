<script lang="ts">
	import MentorshipSection from '$lib/components/sections/MentorshipSection.svelte';
	import SEOHead from '$lib/components/SEOHead.svelte';
	import {
		IconSchool,
		IconSparkles,
		IconCheck,
		IconTrophy,
		IconChartLine,
		IconUsers
	} from '@tabler/icons-svelte';

	const mentorshipSchema = {
		'@context': 'https://schema.org',
		'@type': 'Service',
		name: '1-on-1 Trading Mentorship',
		description:
			'Private trading mentorship with experienced traders. Custom curriculum, personalized feedback, and accountability.',
		provider: {
			'@type': 'Organization',
			name: 'Revolution Trading Pros'
		},
		areaServed: 'Worldwide',
		availableChannel: {
			'@type': 'ServiceChannel',
			serviceType: 'Private 1-on-1 Trading Mentorship'
		}
	};
</script>

<SEOHead
	title="1-on-1 Trading Mentorship | Elite Private Coaching"
	description="Work directly with experienced traders in private mentorship sessions. Custom curriculum, personalized feedback, and accountability to accelerate your trading development."
	canonical="/mentorship"
	ogType="website"
	schema={mentorshipSchema}
/>

<div class="mentorship-page">
	<!-- Hero Section with Mentorship Component -->
	<MentorshipSection />

	<!-- Detailed Benefits Section -->
	<section class="benefits-section">
		<div class="container">
			<h2 class="section-title">What You Get</h2>

			<div class="benefits-grid">
				<div class="benefit-card">
					<div class="benefit-icon">
						<IconUsers size={32} />
					</div>
					<h3>Private 1-on-1 Sessions</h3>
					<p>
						Direct access to experienced traders for personalized coaching sessions tailored to your
						skill level and goals.
					</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconChartLine size={32} />
					</div>
					<h3>Custom Curriculum</h3>
					<p>
						Personalized learning path designed specifically for you, covering strategies, risk
						management, and psychology.
					</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconTrophy size={32} />
					</div>
					<h3>Performance Reviews</h3>
					<p>
						Regular review of your trades, strategies, and progress with constructive feedback and
						actionable insights.
					</p>
				</div>

				<div class="benefit-card">
					<div class="benefit-icon">
						<IconSchool size={32} />
					</div>
					<h3>Lifetime Support</h3>
					<p>
						Ongoing support even after your mentorship period ends. We're invested in your long-term
						success.
					</p>
				</div>
			</div>
		</div>
	</section>

	<!-- What's Included -->
	<section class="included-section">
		<div class="container">
			<h2 class="section-title">Program Includes</h2>

			<div class="included-list">
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Weekly 1-on-1 video sessions (60-90 minutes)</span>
				</div>
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Daily communication via private Discord channel</span>
				</div>
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Personalized trading plan development</span>
				</div>
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Trade journal reviews and performance analysis</span>
				</div>
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Screen sharing and live trade observation</span>
				</div>
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Access to all courses and trading rooms</span>
				</div>
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Custom indicators and tools</span>
				</div>
				<div class="included-item">
					<IconCheck size={24} class="check-icon" />
					<span>Priority support and resources</span>
				</div>
			</div>
		</div>
	</section>

	<!-- Application CTA -->
	<section class="cta-section">
		<div class="container">
			<div class="cta-card">
				<IconSparkles size={48} class="sparkle-icon" />
				<h2>Ready to Transform Your Trading?</h2>
				<p>Limited spots available. Application required.</p>
				<a href="mailto:mentorship@revolutiontradingpros.com" class="apply-button"> Apply Now </a>
				<p class="pricing-note">Starting at $2,500/month • 3-month minimum commitment</p>
			</div>
		</div>
	</section>
</div>

<style>
	.mentorship-page {
		min-height: 100vh;
		background: linear-gradient(135deg, #0a1628 0%, #050e1f 100%);
	}

	.benefits-section,
	.included-section {
		padding: 6rem 2rem;
		background: linear-gradient(135deg, #0f172a 0%, #1e293b 50%, #0f172a 100%);
	}

	.cta-section {
		padding: 6rem 2rem;
		background: linear-gradient(135deg, #064e3b 0%, #0f766e 50%, #064e3b 100%);
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
	}

	.section-title {
		font-size: clamp(2.5rem, 5vw, 3.5rem);
		font-weight: 800;
		text-align: center;
		margin-bottom: 4rem;
		background: linear-gradient(135deg, #10b981, #14b8a6, #06b6d4);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.benefits-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: 2rem;
	}

	.benefit-card {
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 16px;
		padding: 2rem;
		transition: all 0.3s ease;
		backdrop-filter: blur(10px);
	}

	.benefit-card:hover {
		border-color: rgba(16, 185, 129, 0.4);
		transform: translateY(-4px);
		box-shadow: 0 10px 40px rgba(16, 185, 129, 0.2);
	}

	.benefit-icon {
		width: 64px;
		height: 64px;
		background: linear-gradient(135deg, #10b981, #14b8a6);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		margin-bottom: 1.5rem;
	}

	.benefit-card h3 {
		font-size: 1.5rem;
		font-weight: 700;
		color: #ffffff;
		margin-bottom: 1rem;
	}

	.benefit-card p {
		font-size: 1rem;
		color: #cbd5e1;
		line-height: 1.6;
	}

	.included-list {
		max-width: 800px;
		margin: 0 auto;
		display: grid;
		gap: 1.5rem;
	}

	.included-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.5rem;
		background: rgba(15, 23, 42, 0.6);
		border: 1px solid rgba(16, 185, 129, 0.2);
		border-radius: 12px;
		color: #e2e8f0;
		font-size: 1.1rem;
		transition: all 0.3s ease;
		backdrop-filter: blur(10px);
	}

	.included-item:hover {
		border-color: rgba(16, 185, 129, 0.4);
		transform: translateX(8px);
	}

	.check-icon {
		color: #10b981;
		flex-shrink: 0;
	}

	.cta-card {
		text-align: center;
		padding: 4rem 2rem;
		background: rgba(15, 23, 42, 0.8);
		border: 2px solid rgba(16, 185, 129, 0.3);
		border-radius: 24px;
		backdrop-filter: blur(10px);
	}

	.sparkle-icon {
		color: #10b981;
		margin: 0 auto 1.5rem;
		animation: sparkle 2s ease-in-out infinite;
	}

	@keyframes sparkle {
		0%,
		100% {
			opacity: 1;
			transform: scale(1) rotate(0deg);
		}
		50% {
			opacity: 0.5;
			transform: scale(1.2) rotate(180deg);
		}
	}

	.cta-card h2 {
		font-size: clamp(2rem, 4vw, 3rem);
		font-weight: 800;
		color: #ffffff;
		margin-bottom: 1rem;
	}

	.cta-card p {
		font-size: 1.25rem;
		color: #cbd5e1;
		margin-bottom: 2rem;
	}

	.apply-button {
		display: inline-block;
		padding: 1.25rem 3rem;
		background: linear-gradient(135deg, #10b981, #14b8a6);
		color: white;
		font-size: 1.25rem;
		font-weight: 700;
		border-radius: 12px;
		text-decoration: none;
		transition: all 0.3s ease;
		box-shadow: 0 10px 30px rgba(16, 185, 129, 0.3);
	}

	.apply-button:hover {
		transform: translateY(-2px);
		box-shadow: 0 15px 40px rgba(16, 185, 129, 0.4);
	}

	.pricing-note {
		margin-top: 1.5rem;
		font-size: 0.95rem;
		color: #94a3b8;
		font-style: italic;
	}

	@media (max-width: 768px) {
		.benefits-section,
		.included-section,
		.cta-section {
			padding: 4rem 1rem;
		}

		.section-title {
			margin-bottom: 2rem;
		}

		.benefits-grid {
			grid-template-columns: 1fr;
		}

		.included-item {
			font-size: 1rem;
			padding: 1rem;
		}

		.cta-card {
			padding: 3rem 1.5rem;
		}
	}
</style>

<!--
	URL: /careers
-->
<script lang="ts">
	import { IconArrowRight, IconBriefcase, IconMapPin, IconClock } from '$lib/icons';

	const openings = [
		{
			title: 'Senior Trading Educator',
			dept: 'Education',
			location: 'Remote (US)',
			type: 'Full-time',
			href: 'mailto:careers@revolutiontradingpros.com?subject=Senior Trading Educator'
		},
		{
			title: 'Community Manager',
			dept: 'Growth',
			location: 'Remote',
			type: 'Full-time',
			href: 'mailto:careers@revolutiontradingpros.com?subject=Community Manager'
		},
		{
			title: 'Full-Stack Developer',
			dept: 'Engineering',
			location: 'Remote',
			type: 'Full-time',
			href: 'mailto:careers@revolutiontradingpros.com?subject=Full-Stack Developer'
		},
		{
			title: 'Video Producer',
			dept: 'Content',
			location: 'Remote',
			type: 'Contract',
			href: 'mailto:careers@revolutiontradingpros.com?subject=Video Producer'
		}
	];

	const values = [
		{
			title: 'Mission-Driven',
			desc: 'We exist to make professional trading education accessible to everyone, not just the privileged few.'
		},
		{
			title: 'Remote-First',
			desc: "Work from anywhere. We're async-friendly and results-focused, not hours-focused."
		},
		{
			title: 'Continuous Growth',
			desc: 'Every team member gets a learning budget and direct access to our full trading education platform.'
		},
		{
			title: 'Transparency',
			desc: 'Open salaries, open roadmap, and open communication. No politics, no surprises.'
		}
	];
</script>

<svelte:head>
	<title>Careers | Revolution Trading Pros</title>
	<meta
		name="description"
		content="Join the Revolution Trading Pros team. We're building the best trading education platform in the world — come help us do it."
	/>
</svelte:head>

<div class="careers-page">
	<!-- Hero -->
	<section class="careers-hero">
		<div class="careers-hero__inner">
			<p class="careers-hero__eyebrow">We're Hiring</p>
			<h1 class="careers-hero__title">Build the Future of Trading Education</h1>
			<p class="careers-hero__subtitle">
				Join a small, passionate team helping thousands of traders develop real, lasting edge in the
				markets.
			</p>
		</div>
	</section>

	<!-- Values -->
	<section class="values-section">
		<div class="values-inner">
			<h2 class="section-title">Why Work With Us</h2>
			<div class="values-grid">
				{#each values as v (v.title)}
					<div class="value-card">
						<h3 class="value-card__title">{v.title}</h3>
						<p class="value-card__desc">{v.desc}</p>
					</div>
				{/each}
			</div>
		</div>
	</section>

	<!-- Open Roles -->
	<section class="openings-section">
		<div class="openings-inner">
			<h2 class="section-title">Open Positions</h2>
			<div class="openings-list">
				{#each openings as role (role.title)}
					<a href={role.href} class="opening-card">
						<div class="opening-card__left">
							<div class="opening-card__icon"><IconBriefcase size={18} /></div>
							<div>
								<p class="opening-card__title">{role.title}</p>
								<p class="opening-card__dept">{role.dept}</p>
							</div>
						</div>
						<div class="opening-card__meta">
							<span class="opening-card__tag"><IconMapPin size={13} /> {role.location}</span>
							<span class="opening-card__tag"><IconClock size={13} /> {role.type}</span>
							<span class="opening-card__arrow"><IconArrowRight size={16} /></span>
						</div>
					</a>
				{/each}
			</div>
		</div>
	</section>

	<!-- General Application -->
	<section class="general-cta">
		<div class="general-cta__inner">
			<h2 class="general-cta__title">Don't see your role?</h2>
			<p class="general-cta__sub">
				We always want to hear from exceptional people. Send us your story.
			</p>
			<a href="mailto:careers@revolutiontradingpros.com" class="general-cta__btn">
				Send a General Application <IconArrowRight size={16} />
			</a>
		</div>
	</section>
</div>

<style>
	.careers-page {
		background: var(--rtp-bg);
		color: var(--rtp-text);
		min-height: 100vh;
	}

	/* Hero */
	.careers-hero {
		padding: 6rem 1.5rem 4rem;
		text-align: center;
		background: radial-gradient(
			ellipse 80% 50% at 50% 0%,
			rgba(16, 185, 129, 0.1) 0%,
			transparent 70%
		);
		border-bottom: 1px solid var(--rtp-border-soft);
	}
	.careers-hero__inner {
		max-width: 640px;
		margin: 0 auto;
	}
	.careers-hero__eyebrow {
		font-size: 0.8125rem;
		font-weight: 600;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--rtp-emerald);
		margin: 0 0 1rem;
	}
	.careers-hero__title {
		font-size: clamp(2rem, 5vw, 3rem);
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 1rem;
		line-height: 1.15;
	}
	.careers-hero__subtitle {
		font-size: 1.125rem;
		color: var(--rtp-text-muted);
		margin: 0;
		line-height: 1.6;
	}

	/* Values */
	.values-section {
		padding: 5rem 1.5rem;
		max-width: 1100px;
		margin: 0 auto;
	}
	.section-title {
		font-size: 2rem;
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 2.5rem;
	}
	.values-grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1.25rem;
	}
	.value-card {
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border-soft);
		border-radius: 0.75rem;
		padding: 1.75rem;
	}
	.value-card__title {
		font-size: 1rem;
		font-weight: 700;
		color: var(--rtp-text);
		margin: 0 0 0.5rem;
	}
	.value-card__desc {
		font-size: 0.9375rem;
		color: var(--rtp-text-muted);
		margin: 0;
		line-height: 1.6;
	}

	/* Openings */
	.openings-section {
		padding: 5rem 1.5rem;
		border-top: 1px solid var(--rtp-border-soft);
		background: var(--rtp-bg-deeper);
	}
	.openings-inner {
		max-width: 800px;
		margin: 0 auto;
	}
	.openings-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}
	.opening-card {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		background: var(--rtp-surface);
		border: 1px solid var(--rtp-border-soft);
		border-radius: 0.75rem;
		text-decoration: none;
		transition:
			border-color 0.2s ease,
			background 0.2s ease;
	}
	.opening-card:hover {
		border-color: var(--rtp-emerald);
		background: var(--rtp-surface-elev);
	}
	.opening-card__left {
		display: flex;
		align-items: center;
		gap: 1rem;
	}
	.opening-card__icon {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(16, 185, 129, 0.1);
		border-radius: 0.5rem;
		color: var(--rtp-emerald);
		flex-shrink: 0;
	}
	.opening-card__title {
		font-size: 1rem;
		font-weight: 600;
		color: var(--rtp-text);
		margin: 0 0 0.25rem;
	}
	.opening-card__dept {
		font-size: 0.8125rem;
		color: var(--rtp-text-muted);
		margin: 0;
	}
	.opening-card__meta {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		flex-shrink: 0;
	}
	.opening-card__tag {
		display: inline-flex;
		align-items: center;
		gap: 0.3rem;
		font-size: 0.8125rem;
		color: var(--rtp-text-muted);
		background: rgba(255, 255, 255, 0.04);
		padding: 0.25rem 0.625rem;
		border-radius: 9999px;
		border: 1px solid var(--rtp-border-soft);
	}
	.opening-card__arrow {
		color: var(--rtp-text-muted);
	}

	/* General CTA */
	.general-cta {
		padding: 5rem 1.5rem;
		border-top: 1px solid var(--rtp-border-soft);
		text-align: center;
	}
	.general-cta__inner {
		max-width: 560px;
		margin: 0 auto;
	}
	.general-cta__title {
		font-size: 1.75rem;
		font-weight: 800;
		color: var(--rtp-text);
		margin: 0 0 0.75rem;
	}
	.general-cta__sub {
		font-size: 1rem;
		color: var(--rtp-text-muted);
		margin: 0 0 2rem;
		line-height: 1.6;
	}
	.general-cta__btn {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.875rem 1.75rem;
		background: var(--rtp-emerald);
		color: #fff;
		font-size: 0.9375rem;
		font-weight: 700;
		border-radius: 0.5rem;
		text-decoration: none;
		transition: background 0.2s ease;
	}
	.general-cta__btn:hover {
		background: #0da870;
	}

	@media (max-width: 640px) {
		.values-grid {
			grid-template-columns: 1fr;
		}
		.opening-card {
			flex-direction: column;
			align-items: flex-start;
		}
		.opening-card__meta {
			flex-wrap: wrap;
		}
	}
</style>

<!--
	HaveQuestionsSection - Reusable Support Contact Section
	═══════════════════════════════════════════════════════════════════════════
	WordPress Reference: volume-max-model (fl-builder-content-878010)
	
	Usage:
	<HaveQuestionsSection />
	<HaveQuestionsSection email="custom@email.com" phone="555-555-5555" />
-->
<script lang="ts">
	interface Props {
		email?: string;
		phone?: string;
		phoneDisplay?: string;
	}

	let { 
		email = 'support@simplertrading.com',
		phone = '5122668659',
		phoneDisplay = '(512) 266-8659'
	}: Props = $props();
</script>

<section class="have-questions">
	<div class="have-questions__container">
		<h2 class="have-questions__title">
			<strong>Have Questions?</strong>
		</h2>
		<p class="have-questions__text">
			Our support staff is the best by far! You can email 
			<a href="mailto:{email}">{email}</a> 
			or call us at 
			<a href="tel:{phone}">{phoneDisplay}</a>
		</p>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Have Questions Section - Mobile-First
	 * WordPress Reference: fl-node-5d56ce96c4c56
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.have-questions {
		width: 100%;
		background-color: #f7f7f7;
	}

	.have-questions__container {
		max-width: 800px;
		margin: 0 auto;
		padding: 30px 20px;
		text-align: center;
		font-family: 'Open Sans', sans-serif;
	}

	.have-questions__title {
		font-size: 24px;
		margin: 0 0 20px;
		color: #666666;
		font-weight: 400;
	}

	.have-questions__title strong {
		font-weight: 700;
	}

	.have-questions__text {
		font-size: 16px;
		margin: 0;
		color: #666666;
		line-height: 1.6;
	}

	.have-questions__text a {
		color: #0984ae;
		text-decoration: none;
	}

	.have-questions__text a:hover {
		color: #000000;
		text-decoration: underline;
	}

	/* Tablet (768px+) */
	@media (min-width: 768px) {
		.have-questions__container {
			padding: 40px 20px;
		}

		.have-questions__title {
			font-size: 42px;
		}

		.have-questions__text {
			font-size: 22px;
		}
	}

	/* Desktop (992px+) */
	@media (min-width: 992px) {
		.have-questions__container {
			padding: 60px 20px;
		}
	}
</style>

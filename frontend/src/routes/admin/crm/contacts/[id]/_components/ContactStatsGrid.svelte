<script lang="ts">
	import type { ContactRecord } from '#lib/crm/contact-types.js';
	let {
		contact,
		recalculating = false,
		onRecalculate
	}: { contact: ContactRecord; recalculating?: boolean; onRecalculate: () => void } = $props();
</script>

<section aria-label="Recorded contact measurements">
	<div class="stats">
		<div>
			<strong
				>{contact.lead_score === null
					? 'Not measured'
					: `${contact.lead_score} / ${contact.scoring.max_score}`}</strong
			><span>Configured lead score</span>
		</div>
		<div><strong>Not measured</strong><span>Health score</span></div>
		<div><strong>Not measured</strong><span>Engagement score</span></div>
		<div>
			<strong>{contact.facts?.email.open_signals ?? 'Unavailable'}</strong><span
				>CRM outbox open signals</span
			>
		</div>
		<div>
			<strong>{contact.facts?.email.click_signals ?? 'Unavailable'}</strong><span
				>CRM outbox click signals</span
			>
		</div>
		<div><strong>Not measured</strong><span>Lifetime value</span></div>
	</div>
	<p>
		Scoring: {contact.scoring.status.replaceAll('_', ' ')}. {contact.scoring.reason ??
			'Calculated from the configured contact rules.'}
	</p>
	<button onclick={onRecalculate} disabled={recalculating}
		>{recalculating ? 'Recalculating…' : 'Recalculate lead score'}</button
	>
	<details>
		<summary>Score evidence</summary>
		<p>
			Scope: configured contact rules. Assessed: {contact.scoring.evaluated_at ?? 'Not assessed'}.
			Policy: {contact.scoring.policy_hash ?? 'Unavailable'}.
		</p>
		<p>
			Recorded raw score: {contact.recorded_score ?? 'Unknown'}. Historical raw score: {contact.historical_score ??
				'Unknown'}. Raw scores are not current measurements.
		</p>
		{#each contact.scoring.contributions as contribution (contribution.rule_id)}<p>
				Rule {contribution.rule_id}: {contribution.matched ? 'matched' : 'not matched'}, {contribution.applied_points}
				applied points ({contribution.points} configured).
			</p>{/each}
	</details>
	<p>Open and click signals are recorded observations; human engagement has not been measured.</p>
</section>

<style>
	section {
		padding: 1rem;
		border: 1px solid #334155;
		border-radius: 1rem;
		margin-bottom: 1rem;
	}
	.stats {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(9rem, 1fr));
		gap: 1rem;
	}
	.stats div {
		display: grid;
		gap: 0.4rem;
	}
	span,
	p,
	details {
		color: #94a3b8;
		font-size: 0.85rem;
	}
	button {
		padding: 0.6rem;
		border-radius: 0.4rem;
		border: 1px solid #475569;
		background: #0f172a;
		color: #e2e8f0;
	}
	details {
		margin-top: 1rem;
	}
</style>

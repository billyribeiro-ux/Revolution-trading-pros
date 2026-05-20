<script lang="ts">
	// AI pricing-analysis result modal.
	// Behaviour preserved 1:1 from courses/create +page.svelte (lines 1629-1662).
	interface Competitor {
		name: string;
		price: number;
		hours: number;
	}

	interface Analysis {
		show: boolean;
		marketAverage: number;
		suggestedPrice: number;
		competitors: Competitor[];
	}

	interface Props {
		analysis: Analysis | null;
		durationHours: number;
		onClose: () => void;
	}

	let { analysis, durationHours, onClose }: Props = $props();
</script>

{#if analysis?.show}
	<div class="modal-overlay">
		<div class="modal-content pricing-modal">
			<h3>AI Pricing Analysis Complete</h3>
			<div class="pricing-details">
				<p><strong>Your Course:</strong> {durationHours} hours of content</p>
				<p><strong>Market Average:</strong> ${analysis.marketAverage}</p>

				<h4>Competitor Analysis</h4>
				<ul>
					{#each analysis.competitors as comp (comp.name)}
						<li>{comp.name}: ${comp.price} ({comp.hours}h)</li>
					{/each}
				</ul>

				<div class="suggested-price">
					<span>Recommended Price:</span>
					<strong>${analysis.suggestedPrice}</strong>
				</div>

				<h4>Pricing Strategies</h4>
				<ul>
					<li>One-time: ${analysis.suggestedPrice}</li>
					<li>Payment plan: 3 × ${Math.ceil((analysis.suggestedPrice / 3) * 1.1)}</li>
					<li>Subscription: ${Math.ceil(analysis.suggestedPrice / 6)}/month</li>
					<li>Early bird: ${Math.ceil(analysis.suggestedPrice * 0.7)} (30% off)</li>
				</ul>
			</div>
			<div class="modal-actions">
				<button class="btn-primary" onclick={onClose}>Got it</button>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Mirrors the original +page.svelte rules so the modal renders identically. */
	.modal-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.7);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal-content {
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		border: 1px solid rgba(230, 184, 0, 0.3);
		border-radius: 16px;
		padding: 2rem;
		max-width: 500px;
		width: 90%;
		box-shadow: 0 25px 50px rgba(0, 0, 0, 0.5);
	}

	.modal-content h3 {
		color: #f1f5f9;
		font-size: 1.25rem;
		margin-bottom: 1rem;
	}

	.modal-content p {
		color: #94a3b8;
		margin-bottom: 1.5rem;
		line-height: 1.6;
	}

	.modal-actions {
		display: flex;
		gap: 1rem;
		justify-content: flex-end;
	}

	.pricing-modal {
		max-width: 600px;
	}

	.pricing-details h4 {
		color: var(--primary-500);
		font-size: 1rem;
		margin: 1.5rem 0 0.75rem;
	}

	.pricing-details ul {
		list-style: none;
		color: #94a3b8;
	}

	.pricing-details li {
		padding: 0.5rem 0;
		border-bottom: 1px solid rgba(148, 163, 184, 0.1);
	}

	.suggested-price {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 8px;
		margin: 1rem 0;
	}

	.suggested-price span {
		color: #94a3b8;
	}

	.suggested-price strong {
		color: #86efac;
		font-size: 1.5rem;
	}

	.btn-primary {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border-radius: 6px;
		font-weight: 500;
		border: none;
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.875rem;
		background: linear-gradient(135deg, var(--primary-500), var(--primary-600));
		color: white;
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.9;
		transform: translateY(-1px);
	}

	.btn-primary:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}
</style>

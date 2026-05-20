<script lang="ts">
	import { IconChartBar } from '$lib/icons';

	interface TopService {
		id: number;
		name: string;
		type: string;
		members_count: number;
	}

	interface Props {
		services: TopService[];
		onSelectService: (id: number) => void;
	}

	let { services, onSelectService }: Props = $props();
</script>

<div class="top-services-section">
	<h3>Top Services by Members</h3>
	<div class="services-grid">
		{#each services as service (service.id)}
			<button class="service-card" onclick={() => onSelectService(service.id)}>
				<div class="service-icon">
					<IconChartBar size={20} />
				</div>
				<div class="service-info">
					<div class="service-name">{service.name}</div>
					<div class="service-type">{service.type}</div>
				</div>
				<div class="service-count">{service.members_count}</div>
			</button>
		{/each}
	</div>
</div>

<style>
	.top-services-section {
		margin-bottom: 2rem;
	}

	.top-services-section h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 1rem;
	}

	.services-grid {
		display: flex;
		gap: 1rem;
		overflow-x: auto;
		padding-bottom: 0.5rem;
	}

	.service-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.5rem;
		background: rgba(22, 27, 34, 0.8);
		border: 1px solid var(--border-muted);
		border-radius: 12px;
		cursor: pointer;
		transition: all 0.2s;
		min-width: 200px;
		backdrop-filter: blur(10px);
	}

	.service-card:hover {
		border-color: rgba(230, 184, 0, 0.3);
		transform: translateY(-2px);
	}

	.service-icon {
		width: 40px;
		height: 40px;
		background: rgba(230, 184, 0, 0.15);
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--primary-400);
	}

	.service-name {
		font-weight: 600;
		color: var(--text-primary);
	}

	.service-type {
		font-size: 0.75rem;
		color: var(--text-tertiary);
		text-transform: capitalize;
	}

	.service-count {
		margin-left: auto;
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--primary-400);
	}
</style>

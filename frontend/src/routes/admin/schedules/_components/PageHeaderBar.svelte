<!--
	PageHeaderBar — page title with calendar icon, conflict-count badge, Add button.
	Extracted from /admin/schedules in R21-C.
-->
<script lang="ts">
	import IconCalendar from '@tabler/icons-svelte-runes/icons/calendar';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';
	import IconPlus from '@tabler/icons-svelte-runes/icons/plus';

	interface Props {
		conflictCount: number;
		onaddSchedule: () => void;
	}

	let { conflictCount, onaddSchedule }: Props = $props();
</script>

<header class="page-header">
	<div class="header-content">
		<div class="header-title">
			<IconCalendar size={32} />
			<div>
				<h1>Trading Room Schedules</h1>
				<p class="subtitle">Manage weekly schedules for each trading room and service</p>
			</div>
		</div>
		<div class="header-actions">
			{#if conflictCount > 0}
				<div class="conflict-warning">
					<IconAlertCircle size={18} />
					<span>{conflictCount} time conflict{conflictCount !== 1 ? 's' : ''}</span>
				</div>
			{/if}
			<button class="btn btn-primary" onclick={onaddSchedule}>
				<IconPlus size={18} />
				Add Schedule
			</button>
		</div>
	</div>
</header>

<style>
	.page-header {
		margin-bottom: 24px;
		padding-bottom: 24px;
		border-bottom: 1px solid #e2e8f0;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: 24px;
	}

	.header-title {
		display: flex;
		align-items: flex-start;
		gap: 16px;
	}

	.header-title :global(svg) {
		color: #143e59;
		flex-shrink: 0;
		margin-top: 4px;
	}

	.header-title h1 {
		font-size: 28px;
		font-weight: 700;
		color: #1e293b;
		margin: 0 0 4px 0;
	}

	.subtitle {
		color: #64748b;
		font-size: 14px;
		margin: 0;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.conflict-warning {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 12px;
		background: #fef3c7;
		color: #b45309;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
	}

	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s ease;
		border: none;
	}

	.btn-primary {
		background: #143e59;
		color: #fff;
	}

	.btn-primary:hover {
		background: #0d2a3d;
	}

	@media (max-width: 767.98px) {
		.header-content {
			flex-direction: column;
		}
	}
</style>

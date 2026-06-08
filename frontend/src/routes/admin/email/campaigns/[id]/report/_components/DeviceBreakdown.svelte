<!--
	DeviceBreakdown — desktop/mobile/tablet bars + email-client list.
	Extracted from +page.svelte (R22-C) — read-only display, 0 binds.
-->
<script lang="ts">
	import { fly } from 'svelte/transition';
	import { IconDevices, IconDeviceMobile, IconBrowser } from '$lib/icons';

	interface DeviceCounts {
		desktop: number;
		mobile: number;
		tablet: number;
		unknown: number;
	}

	interface EmailClient {
		client: string;
		count: number;
		percentage: number;
	}

	interface Props {
		deviceBreakdown: DeviceCounts;
		totalDevices: number;
		emailClientBreakdown: EmailClient[];
	}

	const { deviceBreakdown, totalDevices, emailClientBreakdown }: Props = $props();
</script>

<section class="glass-panel device-panel" in:fly={{ x: -20, duration: 500, delay: 300 }}>
	<div class="panel-header">
		<div class="panel-title">
			<div class="panel-icon purple">
				<IconDevices size={24} />
			</div>
			<div>
				<h3>Device Breakdown</h3>
				<span class="panel-subtitle">How subscribers read your email</span>
			</div>
		</div>
	</div>

	<div class="device-chart">
		<div class="device-row">
			<div class="device-info">
				<IconDevices size={18} />
				<span>Desktop</span>
			</div>
			<div class="device-bar-wrap">
				<div
					class="device-bar desktop"
					style:width={`${(deviceBreakdown.desktop / totalDevices) * 100}%`}
				></div>
			</div>
			<span class="device-percent"
				>{((deviceBreakdown.desktop / totalDevices) * 100).toFixed(1)}%</span
			>
		</div>
		<div class="device-row">
			<div class="device-info">
				<IconDeviceMobile size={18} />
				<span>Mobile</span>
			</div>
			<div class="device-bar-wrap">
				<div
					class="device-bar mobile"
					style:width={`${(deviceBreakdown.mobile / totalDevices) * 100}%`}
				></div>
			</div>
			<span class="device-percent"
				>{((deviceBreakdown.mobile / totalDevices) * 100).toFixed(1)}%</span
			>
		</div>
		<div class="device-row">
			<div class="device-info">
				<IconBrowser size={18} />
				<span>Tablet</span>
			</div>
			<div class="device-bar-wrap">
				<div
					class="device-bar tablet"
					style:width={`${(deviceBreakdown.tablet / totalDevices) * 100}%`}
				></div>
			</div>
			<span class="device-percent"
				>{((deviceBreakdown.tablet / totalDevices) * 100).toFixed(1)}%</span
			>
		</div>
	</div>

	<div class="email-clients">
		<h4>Email Clients</h4>
		<div class="clients-list">
			{#each emailClientBreakdown as client (client.client)}
				<div class="client-row">
					<span class="client-name">{client.client}</span>
					<div class="client-bar-wrap">
						<div class="client-bar" style:width={`${client.percentage}%`}></div>
					</div>
					<span class="client-percent">{client.percentage.toFixed(1)}%</span>
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	.glass-panel {
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: 20px;
		padding: 1.75rem;
		backdrop-filter: blur(20px);
		box-shadow: var(--admin-card-shadow, 0 4px 20px rgba(0, 0, 0, 0.4));
		position: relative;
		overflow: hidden;
	}

	.glass-panel::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 1px;
		background: linear-gradient(
			90deg,
			transparent,
			var(--admin-accent-primary-soft, rgba(230, 184, 0, 0.15)),
			transparent
		);
	}

	.panel-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.panel-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.panel-title h3 {
		font-size: 1.125rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
	}

	.panel-subtitle {
		font-size: 0.8rem;
		color: var(--text-tertiary);
	}

	.panel-icon {
		width: 44px;
		height: 44px;
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.panel-icon.purple {
		background: var(--admin-widget-purple-bg, rgba(139, 92, 246, 0.15));
		color: var(--admin-widget-purple-icon, #a78bfa);
	}

	.device-chart {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.device-row {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.device-info {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		min-width: 100px;
		color: var(--text-secondary);
		font-size: 0.875rem;
	}

	.device-bar-wrap {
		flex: 1;
		height: 10px;
		background: var(--bg-hover);
		border-radius: 5px;
		overflow: hidden;
	}

	.device-bar {
		height: 100%;
		border-radius: 5px;
		transition: width 1s ease-out;
	}

	.device-bar.desktop {
		background: linear-gradient(
			90deg,
			var(--admin-accent-secondary, var(--secondary-500)),
			var(--admin-accent-secondary-hover, var(--secondary-300))
		);
	}

	.device-bar.mobile {
		background: linear-gradient(
			90deg,
			var(--primary-500),
			var(--admin-accent-primary-hover, var(--primary-400))
		);
	}

	.device-bar.tablet {
		background: linear-gradient(90deg, var(--admin-info, #388bfd), var(--admin-info-text, #58a6ff));
	}

	.device-percent {
		min-width: 50px;
		text-align: right;
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary);
	}

	.email-clients {
		margin-top: 2rem;
		padding-top: 1.5rem;
		border-top: 1px solid var(--border-muted);
	}

	.email-clients h4 {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 1rem;
	}

	.clients-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.client-row {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.client-name {
		min-width: 100px;
		font-size: 0.8125rem;
		color: var(--text-secondary);
	}

	.client-bar-wrap {
		flex: 1;
		height: 6px;
		background: var(--bg-hover);
		border-radius: 3px;
		overflow: hidden;
	}

	.client-bar {
		height: 100%;
		background: linear-gradient(
			90deg,
			var(--admin-widget-purple-icon, #a78bfa),
			var(--admin-info-text, #58a6ff)
		);
		border-radius: 3px;
		transition: width 1s ease-out;
	}

	.client-percent {
		min-width: 45px;
		text-align: right;
		font-size: 0.8125rem;
		font-weight: 600;
		color: var(--text-secondary);
	}

	@media (prefers-reduced-motion: reduce) {
		.device-bar,
		.client-bar {
			transition: none;
		}
	}
</style>

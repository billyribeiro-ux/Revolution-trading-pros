<script lang="ts">
	/**
	 * API Not Connected Component - Apple ICT9+ Design
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * Displays when an API integration is not connected. Provides:
	 * - Clear visual indication
	 * - One-click navigation to connections portal
	 * - Service-specific messaging
	 * - Apple-level polish and animations
	 */

	import { goto } from '$app/navigation';
	import { fade, scale } from 'svelte/transition';
	import { IconPlugConnected, IconArrowRight, IconLock } from '$lib/icons';

	interface Props {
		/** Name of the service that needs to be connected */
		serviceName: string;
		/** Description of what this service provides */
		description?: string;
		/** Service key for direct navigation */
		serviceKey?: string;
		/** Icon to display (component or emoji) */
		icon?: string;
		/** Primary color for theming */
		color?: string;
		/** List of features this connection enables */
		features?: string[];
		/** Whether to show in compact mode */
		compact?: boolean;
	}

	let {
		serviceName,
		description = '',
		serviceKey,
		icon = '🔌',
		color = '#3b82f6',
		features = [],
		compact = false
	}: Props = $props();

	let finalDescription = $derived(description || `Connect ${serviceName} to enable this feature`);

	function handleConnect() {
		if (serviceKey) {
			goto(`/admin/connections?connect=${serviceKey}`);
		} else {
			goto('/admin/connections');
		}
	}
</script>

{#if compact}
	<!-- Compact Inline Version -->
	<div class="api-not-connected-compact" in:fade={{ duration: 300 }} role="alert">
		<div class="compact-icon">
			<IconPlugConnected size={18} />
		</div>
		<span class="compact-message">{serviceName} not connected</span>
		<button class="compact-btn" onclick={handleConnect}>
			Connect
			<IconArrowRight size={14} />
		</button>
	</div>
{:else}
	<!-- Full Card Version -->
	<div class="api-not-connected" in:scale={{ duration: 400, start: 0.95 }} role="alert">
		<!-- Background Glow -->
		<div class="glow" style="--glow-color: {color}"></div>

		<!-- Content -->
		<div class="content">
			<!-- Icon -->
			<div class="icon-container" style="--icon-color: {color}">
				<span class="service-icon">{icon}</span>
				<div class="plug-indicator">
					<IconPlugConnected size={20} />
				</div>
			</div>

			<!-- Text -->
			<h3 class="title">Connect {serviceName}</h3>
			<p class="description">{finalDescription}</p>

			<!-- Features List -->
			{#if features.length > 0}
				<div class="features">
					<p class="features-title">This connection enables:</p>
					<ul class="features-list">
						{#each features as feature}
							<li>
								<span class="check">✓</span>
								{feature}
							</li>
						{/each}
					</ul>
				</div>
			{/if}

			<!-- Connect Button -->
			<button class="connect-btn" onclick={handleConnect} style="--btn-color: {color}">
				<IconPlugConnected size={20} />
				<span>Connect {serviceName}</span>
				<IconArrowRight size={18} />
			</button>

			<!-- Security Note -->
			<div class="security-note">
				<IconLock size={14} />
				<span>Your credentials are encrypted and stored securely</span>
			</div>
		</div>

		<!-- Decorative Elements -->
		<div class="decoration decoration-1"></div>
		<div class="decoration decoration-2"></div>
	</div>
{/if}

<style>
	/* Full Version Styles */
	.api-not-connected {
		position: relative;
		background: linear-gradient(135deg, rgba(30, 41, 59, 0.95), rgba(15, 23, 42, 0.98));
		border: 1px solid rgba(148, 163, 184, 0.1);
		border-radius: 24px;
		padding: 3rem;
		text-align: center;
		overflow: hidden;
		backdrop-filter: blur(20px);
	}

	.glow {
		position: absolute;
		top: -50%;
		left: 50%;
		transform: translateX(-50%);
		width: 300px;
		height: 300px;
		background: radial-gradient(circle, var(--glow-color, #3b82f6) 0%, transparent 70%);
		opacity: 0.1;
		pointer-events: none;
	}

	.content {
		position: relative;
		z-index: 1;
	}

	.icon-container {
		position: relative;
		width: 80px;
		height: 80px;
		margin: 0 auto 1.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.05), rgba(255, 255, 255, 0.02));
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 20px;
	}

	.service-icon {
		font-size: 2.5rem;
		filter: grayscale(50%);
		opacity: 0.7;
	}

	.plug-indicator {
		position: absolute;
		bottom: -8px;
		right: -8px;
		width: 32px;
		height: 32px;
		background: linear-gradient(135deg, #475569, #334155);
		border-radius: 10px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #94a3b8;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}

	.title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #f1f5f9;
		margin: 0 0 0.75rem 0;
	}

	.description {
		font-size: 1rem;
		color: #94a3b8;
		margin: 0 0 2rem 0;
		max-width: 400px;
		margin-left: auto;
		margin-right: auto;
		line-height: 1.6;
	}

	.features {
		background: rgba(0, 0, 0, 0.2);
		border-radius: 12px;
		padding: 1.25rem;
		margin-bottom: 2rem;
		text-align: left;
		max-width: 350px;
		margin-left: auto;
		margin-right: auto;
	}

	.features-title {
		font-size: 0.75rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		color: #64748b;
		margin: 0 0 0.75rem 0;
	}

	.features-list {
		list-style: none;
		padding: 0;
		margin: 0;
	}

	.features-list li {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: #cbd5e1;
		font-size: 0.875rem;
		padding: 0.375rem 0;
	}

	.check {
		color: #10b981;
		font-weight: 600;
	}

	.connect-btn {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 2rem;
		background: linear-gradient(
			135deg,
			var(--btn-color, #3b82f6),
			color-mix(in srgb, var(--btn-color, #3b82f6), #8b5cf6 50%)
		);
		color: white;
		border: none;
		border-radius: 14px;
		font-size: 1rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: 0 10px 30px rgba(59, 130, 246, 0.3);
	}

	.connect-btn:hover {
		transform: translateY(-3px);
		box-shadow: 0 15px 40px rgba(59, 130, 246, 0.4);
	}

	.connect-btn:active {
		transform: translateY(-1px);
	}

	.security-note {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		margin-top: 1.5rem;
		font-size: 0.75rem;
		color: #64748b;
	}

	.decoration {
		position: absolute;
		border-radius: 50%;
		opacity: 0.03;
		pointer-events: none;
	}

	.decoration-1 {
		width: 200px;
		height: 200px;
		bottom: -50px;
		right: -50px;
		background: linear-gradient(135deg, #3b82f6, #8b5cf6);
	}

	.decoration-2 {
		width: 150px;
		height: 150px;
		top: 20%;
		left: -30px;
		background: linear-gradient(135deg, #8b5cf6, #ec4899);
	}

	/* Compact Version Styles */
	.api-not-connected-compact {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 0.875rem 1rem;
		background: linear-gradient(135deg, rgba(245, 158, 11, 0.1), rgba(245, 158, 11, 0.05));
		border: 1px solid rgba(245, 158, 11, 0.2);
		border-radius: 10px;
	}

	.compact-icon {
		color: #f59e0b;
	}

	.compact-message {
		flex: 1;
		font-size: 0.875rem;
		color: #fbbf24;
		font-weight: 500;
	}

	.compact-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 1rem;
		background: rgba(245, 158, 11, 0.2);
		border: 1px solid rgba(245, 158, 11, 0.3);
		border-radius: 6px;
		color: #fbbf24;
		font-size: 0.8125rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s;
	}

	.compact-btn:hover {
		background: rgba(245, 158, 11, 0.3);
		border-color: rgba(245, 158, 11, 0.4);
	}
</style>

<!--
	DevicePreviewToggle — desktop/tablet/mobile preview switcher.
	Extracted from BlockEditor.svelte (R7-C).
-->
<script lang="ts">
	import { IconDeviceDesktop, IconDeviceTablet, IconDeviceMobile } from '#lib/icons/index.js';

	type Device = 'desktop' | 'tablet' | 'mobile';

	interface Props {
		device: Device;
		onChange: (device: Device) => void;
	}

	let { device, onChange }: Props = $props();
</script>

<div class="device-preview" role="group" aria-label="Device preview options">
	<button
		type="button"
		class={['device-btn', { active: device === 'desktop' }]}
		onclick={() => onChange('desktop')}
		title="Desktop Preview"
		aria-label="Preview as desktop"
		aria-pressed={device === 'desktop'}
	>
		<IconDeviceDesktop size={18} aria-hidden="true" />
	</button>
	<button
		type="button"
		class={['device-btn', { active: device === 'tablet' }]}
		onclick={() => onChange('tablet')}
		title="Tablet Preview"
		aria-label="Preview as tablet"
		aria-pressed={device === 'tablet'}
	>
		<IconDeviceTablet size={18} aria-hidden="true" />
	</button>
	<button
		type="button"
		class={['device-btn', { active: device === 'mobile' }]}
		onclick={() => onChange('mobile')}
		title="Mobile Preview"
		aria-label="Preview as mobile"
		aria-pressed={device === 'mobile'}
	>
		<IconDeviceMobile size={18} aria-hidden="true" />
	</button>
</div>

<style>
	.device-preview {
		display: flex;
		background: var(--bg-hover, #252b33);
		border-radius: 8px;
		padding: 0.25rem;
	}

	.device-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border: none;
		background: transparent;
		border-radius: 6px;
		cursor: pointer;
		color: var(--text-secondary, #8b949e);
		transition: all 0.15s;
	}

	.device-btn:hover {
		color: var(--text-primary, #f0f6fc);
	}

	.device-btn.active {
		background: var(--bg-elevated, #161b22);
		color: var(--primary-500, #e6b800);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.device-btn:focus-visible {
		outline: 2px solid var(--primary-500, #e6b800);
		outline-offset: 2px;
	}
</style>

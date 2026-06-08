<script lang="ts">
	interface Props {
		visible: boolean;
		ticker?: string;
		summaryText?: string;
		showLogo?: boolean;
		showInfoBar?: boolean;
		showFrame?: boolean;
		isDark?: boolean;
	}

	let {
		visible,
		ticker = '',
		summaryText = '',
		showLogo = true,
		showInfoBar = true,
		showFrame = true,
		isDark = true
	}: Props = $props();

	let dateStr = $derived(
		new Date().toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		})
	);

	let timeStr = $derived(
		new Date().toLocaleTimeString('en-US', {
			hour: 'numeric',
			minute: '2-digit',
			timeZoneName: 'short'
		})
	);
	let frameBorder = $derived(showFrame ? '2px solid transparent' : undefined);
	let frameBorderImage = $derived(
		showFrame ? 'linear-gradient(135deg, #6366f1, #00d4aa, #6366f1) 1' : undefined
	);
	let frameRadius = $derived(showFrame ? '12px' : undefined);
	let topBarBackground = $derived(isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.03)');
	let bottomBarBackground = $derived(isDark ? 'rgba(255,255,255,0.03)' : 'rgba(0,0,0,0.02)');
	let primaryTextColor = $derived(isDark ? '#e8e8f0' : '#111827');
	let mutedTextColor = $derived(isDark ? '#8888aa' : '#6b7280');
	let brandColor = $derived(isDark ? '#6366f1' : '#4f46e5');

	let logoFailed = $state(false);
</script>

{#if visible}
	<div
		class="absolute inset-0 pointer-events-none z-[9000] flex flex-col"
		style:border={frameBorder}
		style:border-image={frameBorderImage}
		style:border-radius={frameRadius}
	>
		<!-- Top Info Bar -->
		{#if showInfoBar}
			<div
				class="flex items-center justify-between px-4 py-2 rounded-t-lg"
				style:background={topBarBackground}
			>
				<div class="flex items-center gap-2">
					{#if ticker}
						<span
							class="text-sm font-bold"
							style:color={primaryTextColor}
							style:font-family="'Plus Jakarta Sans', 'Inter', system-ui, sans-serif">{ticker}</span
						>
					{/if}
					{#if summaryText}
						<span
							class="text-xs"
							style:color={mutedTextColor}
							style:font-family="'JetBrains Mono', 'Fira Code', monospace"
							>|&nbsp;&nbsp;{summaryText}</span
						>
					{/if}
				</div>
				<span
					class="text-[11px]"
					style:color={mutedTextColor}
					style:font-family="'Inter', system-ui, sans-serif">{dateStr} &bull; {timeStr}</span
				>
			</div>
		{/if}

		<!-- Spacer (content area — not rendered by this overlay) -->
		<div class="flex-1"></div>

		<!-- Bottom Footer Bar -->
		{#if showLogo}
			<div
				class="flex items-center justify-between px-4 py-2.5 rounded-b-lg"
				style:background={bottomBarBackground}
			>
				<div class="flex items-center gap-2">
					{#if !logoFailed}
						<img
							src="/images/rtp-logo.png"
							alt="Revolution Trading Pros"
							class="h-6"
							onerror={() => (logoFailed = true)}
						/>
					{:else}
						<span
							class="text-xs font-bold"
							style:color={primaryTextColor}
							style:font-family="'Plus Jakarta Sans', 'Inter', system-ui, sans-serif"
							>Revolution Trading Pros</span
						>
					{/if}
				</div>
				<span
					class="text-[11px]"
					style:color={brandColor}
					style:font-family="'Inter', system-ui, sans-serif">revolutiontradingpros.com</span
				>
			</div>
		{/if}
	</div>
{/if}

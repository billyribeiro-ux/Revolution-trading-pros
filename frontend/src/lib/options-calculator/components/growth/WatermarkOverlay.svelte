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

	const DISPLAY_FONT = "'Plus Jakarta Sans', 'Inter', system-ui, sans-serif";
	const BODY_FONT = "'Inter', system-ui, sans-serif";
	const MONO_FONT = "'JetBrains Mono', 'Fira Code', monospace";

	const DATE_FORMAT: Intl.DateTimeFormatOptions = {
		month: 'short',
		day: 'numeric',
		year: 'numeric'
	};
	const TIME_FORMAT: Intl.DateTimeFormatOptions = {
		hour: 'numeric',
		minute: '2-digit',
		timeZoneName: 'short'
	};

	const DARK_PALETTE = {
		topBar: 'rgba(255,255,255,0.04)',
		bottomBar: 'rgba(255,255,255,0.03)',
		text: '#e8e8f0',
		muted: '#8888aa',
		brand: '#6366f1'
	};
	const LIGHT_PALETTE: typeof DARK_PALETTE = {
		topBar: 'rgba(0,0,0,0.03)',
		bottomBar: 'rgba(0,0,0,0.02)',
		text: '#111827',
		muted: '#6b7280',
		brand: '#4f46e5'
	};

	const palette = $derived(isDark ? DARK_PALETTE : LIGHT_PALETTE);

	/** "Oct 10, 2026 • 3:45 PM EDT" */
	function formatStamp(date: Date): string {
		const day = date.toLocaleDateString('en-US', DATE_FORMAT);
		const time = date.toLocaleTimeString('en-US', TIME_FORMAT);
		return `${day} • ${time}`;
	}

	let logoFailed = $state(false);
</script>

{#if visible}
	<!-- Stamped when the overlay appears (each capture), not when the component mounted. -->
	{const stamp = formatStamp(new Date())}

	<div
		class="absolute inset-0 pointer-events-none z-[9000] flex flex-col"
		style:border={showFrame ? '2px solid transparent' : undefined}
		style:border-image={showFrame
			? 'linear-gradient(135deg, #6366f1, #00d4aa, #6366f1) 1'
			: undefined}
		style:border-radius={showFrame ? '12px' : undefined}
	>
		<!-- Top Info Bar -->
		{#if showInfoBar}
			<div
				class="flex items-center justify-between px-4 py-2 rounded-t-lg"
				style:background={palette.topBar}
			>
				<div class="flex items-center gap-2">
					{#if ticker}
						<span
							class="text-sm font-bold"
							style:color={palette.text}
							style:font-family={DISPLAY_FONT}>{ticker}</span
						>
					{/if}
					{#if summaryText}
						<span class="text-xs" style:color={palette.muted} style:font-family={MONO_FONT}
							>|&nbsp;&nbsp;{summaryText}</span
						>
					{/if}
				</div>
				<span class="text-[11px]" style:color={palette.muted} style:font-family={BODY_FONT}
					>{stamp}</span
				>
			</div>
		{/if}

		<!-- Spacer (content area — not rendered by this overlay) -->
		<div class="flex-1"></div>

		<!-- Bottom Footer Bar -->
		{#if showLogo}
			<div
				class="flex items-center justify-between px-4 py-2.5 rounded-b-lg"
				style:background={palette.bottomBar}
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
							style:color={palette.text}
							style:font-family={DISPLAY_FONT}>Revolution Trading Pros</span
						>
					{/if}
				</div>
				<span class="text-[11px]" style:color={palette.brand} style:font-family={BODY_FONT}
					>revolutiontradingpros.com</span
				>
			</div>
		{/if}
	</div>
{/if}

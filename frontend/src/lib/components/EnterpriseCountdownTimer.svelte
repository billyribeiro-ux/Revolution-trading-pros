<script lang="ts">
	import { onMount } from 'svelte';
	import CountdownTimer from './CountdownTimer.svelte';
	import type { TimerEventPayload, TimerEventType } from '$lib/api/timers';
	import { getServerTime } from '$lib/api/time';
	import { recordTimerEvent } from '$lib/api/timers';

	// Minimal wrapper that wires CountdownTimer to backend time + analytics

	interface Props {
		timerId: string;
		endDate: string | Date;
		startDate?: string | Date | null;
		timezone?: string | null;
		serverTime?: Date | null;
		format?: 'default' | 'digital' | 'circular' | 'flip' | 'minimal' | 'bar' | 'custom';
		showDays?: boolean;
		showHours?: boolean;
		showMinutes?: boolean;
		showSeconds?: boolean;
		showMilliseconds?: boolean;
		showLabels?: boolean;
		showSeparators?: boolean;
		compactMode?: boolean;
		leadingZeros?: boolean;
		timerColor?: string;
		warningColor?: string;
		dangerColor?: string;
		backgroundColor?: string;
		textColor?: string;
		labelColor?: string;
		size?: 'small' | 'medium' | 'large' | 'custom';
		customSize?: string | null;
		fontFamily?: string;
		borderRadius?: string;
		gap?: string;
		padding?: string;
		animated?: boolean;
		animationType?: 'slide' | 'fade' | 'flip' | 'zoom' | 'none';
		animationDuration?: number;
		pulseOnUpdate?: boolean;
		urgencyAnimation?: boolean;
		particleEffects?: boolean;
		warningThreshold?: number;
		dangerThreshold?: number;
		milestones?: number[];
		updateInterval?: number;
		pauseOnHover?: boolean;
		pauseOnBlur?: boolean;
		showProgressBar?: boolean;
		showPercentage?: boolean;
	}

	let {
		timerId,
		endDate,
		startDate = null,
		timezone = null,
		serverTime = null,
		format = 'default',
		showDays = true,
		showHours = true,
		showMinutes = true,
		showSeconds = true,
		showMilliseconds = false,
		showLabels = true,
		showSeparators = true,
		compactMode = false,
		leadingZeros = true,
		timerColor = '#6366f1',
		warningColor = '#f59e0b',
		dangerColor = '#ef4444',
		backgroundColor = 'rgba(99, 102, 241, 0.1)',
		textColor = 'currentColor',
		labelColor = '#94a3b8',
		size = 'medium',
		customSize = null,
		fontFamily = 'inherit',
		borderRadius = '12px',
		gap = '1rem',
		padding = '0.75rem 1rem',
		animated = true,
		animationType = 'slide',
		animationDuration = 300,
		pulseOnUpdate = false,
		urgencyAnimation = true,
		particleEffects = false,
		warningThreshold = 60000,
		dangerThreshold = 10000,
		milestones = [],
		updateInterval = 1000,
		pauseOnHover = false,
		pauseOnBlur = false,
		showProgressBar = false,
		showPercentage = false
	}: Props = $props();

	// Internal resolved server time from backend
	let resolvedServerTime: Date | null = $state(null);

	onMount(async () => {
		if (!serverTime) {
			try {
				const response = await getServerTime();
				resolvedServerTime = new Date(response.server_time);
			} catch (error) {
				// If backend time fails, fall back to client time (leave resolvedServerTime as null)
				console.error('[EnterpriseCountdownTimer] Failed to fetch server time', error);
			}
		} else {
			resolvedServerTime = serverTime;
		}
	});

	function emitTimerEvent(type: TimerEventType, detail: any) {
		const remainingMs = detail?.timeData?.totalMilliseconds ?? 0;

		const payload: TimerEventPayload = {
			timer_id: timerId,
			type,
			timestamp: new Date().toISOString(),
			remaining_ms: remainingMs,
			payload: detail ?? undefined
		};

		recordTimerEvent(payload).catch((error) => {
			console.error('[EnterpriseCountdownTimer] Failed to record timer event', type, error);
		});
	}

	function handleStart() {
		emitTimerEvent('start', {});
	}

	function handlePause() {
		emitTimerEvent('pause', {});
	}

	function handleResume() {
		emitTimerEvent('resume', {});
	}

	function handleWarning() {
		emitTimerEvent('warning', {});
	}

	function handleDanger() {
		emitTimerEvent('danger', {});
	}

	function handleMilestone(milestone: number) {
		emitTimerEvent('milestone', { milestone });
	}

	function handleExpire(data: any) {
		emitTimerEvent('expire', data);
	}

	function handleUpdate(timeData: any) {
		emitTimerEvent('update', { timeData });
	}
</script>

<CountdownTimer
	{endDate}
	{startDate}
	{timezone}
	serverTime={resolvedServerTime}
	{format}
	{showDays}
	{showHours}
	{showMinutes}
	{showSeconds}
	{showMilliseconds}
	{showLabels}
	{showSeparators}
	{compactMode}
	{leadingZeros}
	{timerColor}
	{warningColor}
	{dangerColor}
	{backgroundColor}
	{textColor}
	{labelColor}
	{size}
	{customSize}
	{fontFamily}
	{borderRadius}
	{gap}
	{padding}
	{animated}
	{animationType}
	{animationDuration}
	{pulseOnUpdate}
	{urgencyAnimation}
	{particleEffects}
	{warningThreshold}
	{dangerThreshold}
	{milestones}
	{updateInterval}
	{pauseOnHover}
	{pauseOnBlur}
	{showProgressBar}
	{showPercentage}
	onStart={handleStart}
	onPause={handlePause}
	onResume={handleResume}
	onUpdate={handleUpdate}
	onWarning={handleWarning}
	onDanger={handleDanger}
	onMilestone={handleMilestone}
	onExpire={handleExpire}
/>

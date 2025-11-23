<script lang="ts">
	import { onMount } from 'svelte';
	import CountdownTimer from './CountdownTimer.svelte';
	import type { TimerEventPayload, TimerEventType } from '$lib/api/timers';
	import { getServerTime } from '$lib/api/time';
	import { recordTimerEvent } from '$lib/api/timers';

	// Minimal wrapper that wires CountdownTimer to backend time + analytics

	export let timerId: string;
	export let endDate: string | Date;
	export let startDate: string | Date | null = null;
	export let timezone: string | null = null;
	// Allow parent to override serverTime; otherwise we fetch from backend
	export let serverTime: Date | null = null;

	// Pass-through props with same defaults as CountdownTimer
	export let format: 'default' | 'digital' | 'circular' | 'flip' | 'minimal' | 'bar' | 'custom' = 'default';
	export let showDays: boolean = true;
	export let showHours: boolean = true;
	export let showMinutes: boolean = true;
	export let showSeconds: boolean = true;
	export let showMilliseconds: boolean = false;
	export let showLabels: boolean = true;
	export let showSeparators: boolean = true;
	export let compactMode: boolean = false;
	export let leadingZeros: boolean = true;

	export let timerColor: string = '#6366f1';
	export let warningColor: string = '#f59e0b';
	export let dangerColor: string = '#ef4444';
	export let backgroundColor: string = 'rgba(99, 102, 241, 0.1)';
	export let textColor: string = 'currentColor';
	export let labelColor: string = '#94a3b8';
	export let size: 'small' | 'medium' | 'large' | 'custom' = 'medium';
	export let customSize: string | null = null;
	export let fontFamily: string = 'inherit';
	export let borderRadius: string = '12px';
	export let gap: string = '1rem';
	export let padding: string = '0.75rem 1rem';

	export let animated: boolean = true;
	export let animationType: 'slide' | 'fade' | 'flip' | 'zoom' | 'none' = 'slide';
	export let animationDuration: number = 300;
	export let pulseOnUpdate: boolean = false;
	export let urgencyAnimation: boolean = true;
	export let particleEffects: boolean = false;

	export let warningThreshold: number = 60000;
	export let dangerThreshold: number = 10000;
	export let milestones: number[] = [];
	export let updateInterval: number = 1000;
	export let pauseOnHover: boolean = false;
	export let pauseOnBlur: boolean = false;
	export let showProgressBar: boolean = false;
	export let showPercentage: boolean = false;

	// Internal resolved server time from backend
	let resolvedServerTime: Date | null = null;

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

	function handleStart(event: CustomEvent<any>) {
		emitTimerEvent('start', event.detail);
	}

	function handlePause(event: CustomEvent<any>) {
		emitTimerEvent('pause', event.detail);
	}

	function handleResume(event: CustomEvent<any>) {
		emitTimerEvent('resume', event.detail);
	}

	function handleStop(event: CustomEvent<any>) {
		emitTimerEvent('stop', event.detail);
	}

	function handleReset(event: CustomEvent<any>) {
		emitTimerEvent('reset', event.detail);
	}

	function handleUpdate(event: CustomEvent<any>) {
		emitTimerEvent('update', event.detail);
	}

	function handleWarning(event: CustomEvent<any>) {
		emitTimerEvent('warning', event.detail);
	}

	function handleDanger(event: CustomEvent<any>) {
		emitTimerEvent('danger', event.detail);
	}

	function handleMilestone(event: CustomEvent<any>) {
		emitTimerEvent('milestone', event.detail);
	}

	function handleExpire(event: CustomEvent<any>) {
		emitTimerEvent('expire', event.detail);
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
	on:start={handleStart}
	on:pause={handlePause}
	on:resume={handleResume}
	on:stop={handleStop}
	on:reset={handleReset}
	on:update={handleUpdate}
	on:warning={handleWarning}
	on:danger={handleDanger}
	on:milestone={handleMilestone}
	on:expire={handleExpire}
/>

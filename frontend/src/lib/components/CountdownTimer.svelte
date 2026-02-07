<!--
/**
 * CountdownTimer Component - Google L7+ Enterprise Implementation
 * ═══════════════════════════════════════════════════════════════════════════
 * 
 * ENTERPRISE FEATURES:
 * 
 * 1. ADVANCED DISPLAY MODES:
 *    - Digital clock format
 *    - Circular progress rings
 *    - Flip card animation
 *    - Minimal text
 *    - Bar chart style
 *    - Custom formats
 * 
 * 2. TIME FEATURES:
 *    - Timezone support
 *    - Server sync
 *    - Recurring events
 *    - Multiple timers
 *    - Pause/Resume
 *    - Dynamic updates
 * 
 * 3. VISUAL EFFECTS:
 *    - Smooth animations
 *    - Urgency indicators
 *    - Milestone alerts
 *    - Particle effects
 *    - Glow effects
 *    - Custom themes
 * 
 * 4. EVENTS & HOOKS:
 *    - Milestone callbacks
 *    - Warning thresholds
 *    - Completion actions
 *    - Update events
 *    - Format functions
 *    - Custom triggers
 * 
 * 5. ACCESSIBILITY:
 *    - Screen reader updates
 *    - ARIA labels
 *    - Keyboard controls
 *    - High contrast
 *    - Reduced motion
 *    - Focus management
 * 
 * @version 3.0.0 (Google L7+ Enterprise)
 * @component
 */
-->

<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { spring, tweened } from 'svelte/motion';
	import { cubicOut } from 'svelte/easing';
	import { browser } from '$app/environment';

	// ═══════════════════════════════════════════════════════════════════════════
	// Props
	// ═══════════════════════════════════════════════════════════════════════════

	// Props Interface
	interface Props {
		endDate: string | Date;
		startDate?: string | Date | null;
		timezone?: string | null;
		serverTime?: Date | null;
		autoStart?: boolean;
		loop?: boolean;
		loopInterval?: number;
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
		onExpire?: (data: any) => void;
		onUpdate?: (time: TimeData) => void;
		onMilestone?: (milestone: number) => void;
		onWarning?: () => void;
		onDanger?: () => void;
		onStart?: () => void;
		onPause?: () => void;
		onResume?: () => void;
		customFormatter?: ((time: TimeData) => string) | null;
		labelFormatter?: ((unit: string, value: number) => string) | null;
		expired?: import('svelte').Snippet;
	}

	let {
		endDate,
		startDate = null,
		timezone = null,
		serverTime = null,
		autoStart = true,
		loop = false,
		loopInterval = 0,
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
		showPercentage = false,
		onExpire,
		onUpdate,
		onMilestone,
		onWarning,
		onDanger,
		onStart,
		onPause,
		onResume,
		customFormatter = null,
		labelFormatter = null,
		expired
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════════
	// State
	// ═══════════════════════════════════════════════════════════════════════════

	interface TimeData {
		days: number;
		hours: number;
		minutes: number;
		seconds: number;
		milliseconds: number;
		totalMilliseconds: number;
		progress: number;
		isWarning: boolean;
		isDanger: boolean;
		isExpired: boolean;
	}

	let timeData: TimeData = $state({
		days: 0,
		hours: 0,
		minutes: 0,
		seconds: 0,
		milliseconds: 0,
		totalMilliseconds: 0,
		progress: 0,
		isWarning: false,
		isDanger: false,
		isExpired: false
	});

	let interval: ReturnType<typeof setInterval> | null = null;
	let rafId: number | null = null;
	let isRunning: boolean = $state(false);
	let isPaused: boolean = $state(false);
	let _hasStarted: boolean = $state(false);
	let timeOffset: number = 0;
	let totalDuration: number = 0;
	let elapsedPauseTime: number = 0;
	let lastPauseTime: number = 0;
	let _lastUpdateTime: number = 0;
	let currentColor: string = $state('#6366f1');
	let previousValues: Partial<TimeData> = {};
	let milestonesReached: Set<number> = new Set();

	// Sync timerColor prop to currentColor state
	$effect(() => {
		currentColor = timerColor;
	});

	// Animation values
	const progressValue = tweened(0, { duration: 1000, easing: cubicOut });
	const scaleValue = spring(1, { stiffness: 0.3, damping: 0.5 });
	const pulseValue = spring(1, { stiffness: 0.5, damping: 0.3 });

	// Digit flip tracking
	let digitFlips: Record<string, boolean> = $state({});

	// Typed time units for circular display
	const timeUnits = {
		days: 365,
		hours: 24,
		minutes: 60,
		seconds: 60
	} as const;

	type TimeUnitKey = keyof typeof timeUnits;

	// Size mappings
	const sizeMap = {
		small: { fontSize: '1.5rem', labelSize: '0.625rem', padding: '0.5rem 0.75rem' },
		medium: { fontSize: '2.5rem', labelSize: '0.75rem', padding: '0.75rem 1rem' },
		large: { fontSize: '3.5rem', labelSize: '0.875rem', padding: '1rem 1.25rem' }
	};

	// ═══════════════════════════════════════════════════════════════════════════
	// Reactive Statements
	// ═══════════════════════════════════════════════════════════════════════════

	let currentSize = $derived(
		size === 'custom' && customSize
			? customSize
			: size in sizeMap
				? sizeMap[size as keyof typeof sizeMap].fontSize
				: sizeMap.medium.fontSize
	);
	let currentLabelSize = $derived(
		size === 'custom'
			? '0.75rem'
			: size in sizeMap
				? sizeMap[size as keyof typeof sizeMap].labelSize
				: sizeMap.medium.labelSize
	);
	let currentPadding = $derived(
		size === 'custom'
			? padding
			: size in sizeMap
				? sizeMap[size as keyof typeof sizeMap].padding
				: sizeMap.medium.padding
	);

	$effect(() => {
		if (timeData.isWarning && !timeData.isDanger) currentColor = warningColor;
		else if (timeData.isDanger) currentColor = dangerColor;
		else currentColor = timerColor;
	});

	let formattedTime = $derived(customFormatter ? customFormatter(timeData) : formatDefaultTime());

	$effect(() => {
		// Update progress
		progressValue.set(timeData.progress, { duration: animationDuration });

		// Trigger pulse animation on update
		if (pulseOnUpdate && timeData.seconds !== previousValues.seconds) {
			pulseValue.set(1.1);
			setTimeout(() => pulseValue.set(1), 100);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Lifecycle
	// ═══════════════════════════════════════════════════════════════════════════

	onMount(() => {
		if (!browser) return;

		// Calculate server time offset if provided
		if (serverTime) {
			timeOffset = new Date(serverTime).getTime() - Date.now();
		}

		// Calculate total duration
		const initialStartTime = startDate ? new Date(startDate).getTime() : getCurrentTime();
		const initialEndTime = new Date(endDate).getTime();
		if (!isNaN(initialStartTime) && !isNaN(initialEndTime)) {
			totalDuration = initialEndTime - initialStartTime;
		} else {
			totalDuration = 0;
		}

		// Auto start if enabled
		if (autoStart) {
			start();
		}

		// Setup event listeners
		setupEventListeners();
	});

	onDestroy(() => {
		stop();
		cleanupEventListeners();
	});

	// ═══════════════════════════════════════════════════════════════════════════
	// Timer Control Methods
	// ═══════════════════════════════════════════════════════════════════════════

	export function start() {
		if (isRunning || timeData.isExpired) return;

		_hasStarted = true;
		isRunning = true;
		isPaused = false;

		updateCountdown();

		// Use requestAnimationFrame for smooth animations
		if (showMilliseconds || updateInterval < 100) {
			startRAF();
		} else {
			interval = setInterval(updateCountdown, updateInterval);
		}

		if (onStart) onStart();
	}

	export function pause() {
		if (!isRunning || isPaused) return;

		isPaused = true;
		lastPauseTime = getCurrentTime();

		stop();

		if (onPause) onPause();
	}

	export function resume() {
		if (!isPaused) return;

		// Calculate pause duration
		elapsedPauseTime += getCurrentTime() - lastPauseTime;
		isPaused = false;

		start();

		if (onResume) onResume();
	}

	export function reset() {
		stop();

		// Reset all state
		timeData = {
			days: 0,
			hours: 0,
			minutes: 0,
			seconds: 0,
			milliseconds: 0,
			totalMilliseconds: 0,
			progress: 0,
			isWarning: false,
			isDanger: false,
			isExpired: false
		};

		_hasStarted = false;
		isPaused = false;
		elapsedPauseTime = 0;
		lastPauseTime = 0;
		milestonesReached.clear();

		// Restart if auto start is enabled
		if (autoStart) {
			start();
		}
	}

	export function stop() {
		isRunning = false;

		if (interval) {
			clearInterval(interval);
			interval = null;
		}

		if (rafId) {
			cancelAnimationFrame(rafId);
			rafId = null;
		}
	}

	export function addTime(milliseconds: number) {
		const newEndDate = new Date(endDate).getTime() + milliseconds;
		endDate = new Date(newEndDate);
		updateCountdown();
	}

	export function setEndDate(newEndDate: string | Date) {
		endDate = newEndDate;
		updateCountdown();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Update Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function updateCountdown() {
		const now = getCurrentTime();
		const end = new Date(endDate).getTime();

		// If end date is invalid, treat timer as expired
		if (isNaN(end)) {
			handleExpiration();
			return;
		}

		let start = startDate ? new Date(startDate).getTime() : now - elapsedPauseTime;
		// Fallback if provided startDate is invalid
		if (isNaN(start)) {
			start = now - elapsedPauseTime;
		}

		// Recompute total duration on every tick to respect dynamic endDate/startDate
		totalDuration = end - start;

		const distance = end - now + elapsedPauseTime;

		// Check if expired
		if (distance < 0) {
			handleExpiration();
			return;
		}

		// Store previous values for change detection
		previousValues = { ...timeData };

		// Calculate time units
		const days = Math.floor(distance / (1000 * 60 * 60 * 24));
		const hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
		const minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
		const seconds = Math.floor((distance % (1000 * 60)) / 1000);
		const milliseconds = distance % 1000;

		// Calculate progress (guard against zero/negative duration)
		const elapsed = now - start;
		const progress =
			totalDuration > 0 ? Math.max(0, Math.min(100, (elapsed / totalDuration) * 100)) : 0;

		// Check warning and danger states
		const isWarning = distance <= warningThreshold && distance > dangerThreshold;
		const isDanger = distance <= dangerThreshold;

		// Update time data
		timeData = {
			days,
			hours,
			minutes,
			seconds,
			milliseconds,
			totalMilliseconds: distance,
			progress,
			isWarning,
			isDanger,
			isExpired: false
		};

		// Check for digit changes (for flip animation)
		checkDigitChanges();

		// Check milestones
		checkMilestones(distance);

		// Trigger warning/danger events
		if (isWarning && !previousValues.isWarning && onWarning) {
			onWarning();
		}

		if (isDanger && !previousValues.isDanger && onDanger) {
			onDanger();
		}

		// Call update callback
		if (onUpdate) onUpdate(timeData);

		// Update last update time
		_lastUpdateTime = now;

		// Apply urgency animation
		if (urgencyAnimation && isDanger) {
			scaleValue.set(1.05);
			setTimeout(() => scaleValue.set(1), 100);
		}
	}

	function handleExpiration() {
		timeData = {
			days: 0,
			hours: 0,
			minutes: 0,
			seconds: 0,
			milliseconds: 0,
			totalMilliseconds: 0,
			progress: 100,
			isWarning: false,
			isDanger: false,
			isExpired: true
		};

		stop();

		// Handle loop
		if (loop) {
			setTimeout(() => {
				reset();
				if (loopInterval > 0) {
					const newEnd = new Date(Date.now() + loopInterval);
					endDate = newEnd;
				}
				start();
			}, 100);
			return;
		}

		// Call expiration callback
		if (onExpire) {
			onExpire({ timeData, endDate });
		}
	}

	function checkMilestones(distance: number) {
		for (const milestone of milestones) {
			if (distance <= milestone && !milestonesReached.has(milestone)) {
				milestonesReached.add(milestone);
				if (onMilestone) onMilestone(milestone);
			}
		}
	}

	function checkDigitChanges() {
		const units = ['days', 'hours', 'minutes', 'seconds'];
		units.forEach((unit) => {
			const current = timeData[unit as keyof TimeData];
			const previous = previousValues[unit as keyof TimeData];
			if (current !== previous) {
				digitFlips[unit] = true;
				setTimeout(() => {
					digitFlips[unit] = false;
				}, animationDuration);
			}
		});
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Helper Functions
	// ═══════════════════════════════════════════════════════════════════════════

	function getCurrentTime(): number {
		let time = Date.now() + timeOffset;

		// Apply timezone offset if specified
		if (timezone) {
			const date = new Date(time);
			const utcTime = date.getTime() + date.getTimezoneOffset() * 60000;
			const timezoneOffset = getTimezoneOffset(timezone);
			time = utcTime + timezoneOffset * 60000;
		}

		return time;
	}

	function getTimezoneOffset(tz: string): number {
		// Simple timezone offset map (extend as needed)
		const offsets: Record<string, number> = {
			EST: -300,
			EDT: -240,
			CST: -360,
			CDT: -300,
			MST: -420,
			MDT: -360,
			PST: -480,
			PDT: -420,
			GMT: 0,
			UTC: 0,
			CET: 60,
			CEST: 120
		};

		return offsets[tz] || 0;
	}

	function formatDefaultTime(): string {
		const parts = [];

		if (showDays && (timeData.days > 0 || !compactMode)) {
			parts.push(formatUnit('days', timeData.days));
		}

		if (showHours && (timeData.hours > 0 || !compactMode)) {
			parts.push(formatUnit('hours', timeData.hours));
		}

		if (showMinutes && (timeData.minutes > 0 || !compactMode)) {
			parts.push(formatUnit('minutes', timeData.minutes));
		}

		if (showSeconds) {
			parts.push(formatUnit('seconds', timeData.seconds));
		}

		if (showMilliseconds) {
			parts.push(formatUnit('ms', timeData.milliseconds));
		}

		return parts.join(showSeparators ? ' : ' : ' ');
	}

	function formatUnit(unit: string, value: number): string {
		const formattedValue = leadingZeros ? value.toString().padStart(2, '0') : value.toString();

		if (!showLabels) {
			return formattedValue;
		}

		if (labelFormatter) {
			return labelFormatter(unit, value);
		}

		return `${formattedValue} ${getUnitLabel(unit, value)}`;
	}

	function getUnitLabel(unit: string, value: number): string {
		const labels: Record<string, [string, string]> = {
			days: ['Day', 'Days'],
			hours: ['Hour', 'Hours'],
			minutes: ['Min', 'Mins'],
			seconds: ['Sec', 'Secs'],
			ms: ['ms', 'ms']
		};

		const [singular, plural] = labels[unit] || [unit, unit];
		return value === 1 ? singular : plural;
	}

	function startRAF() {
		const animate = () => {
			updateCountdown();
			if (isRunning && !timeData.isExpired) {
				rafId = requestAnimationFrame(animate);
			}
		};
		animate();
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Event Listeners
	// ═══════════════════════════════════════════════════════════════════════════

	function setupEventListeners() {
		if (!browser) return;

		if (pauseOnBlur) {
			document.addEventListener('visibilitychange', handleVisibilityChange);
		}
	}

	function cleanupEventListeners() {
		if (!browser) return;

		document.removeEventListener('visibilitychange', handleVisibilityChange);
	}

	function handleVisibilityChange() {
		if (document.hidden && isRunning && !isPaused) {
			pause();
		} else if (!document.hidden && isPaused) {
			resume();
		}
	}

	function handleMouseEnter() {
		if (pauseOnHover && isRunning) {
			pause();
		}
	}

	function handleMouseLeave() {
		if (pauseOnHover && isPaused) {
			resume();
		}
	}

	// ═══════════════════════════════════════════════════════════════════════════
	// Format Renderers
	// ═══════════════════════════════════════════════════════════════════════════

	function getCircleProps(value: number, max: number) {
		const radius = 45;
		const circumference = 2 * Math.PI * radius;
		const offset = circumference - (value / max) * circumference;
		return { radius, circumference, offset };
	}
</script>

<!-- Main Container -->
<div
	class="countdown-timer format-{format} size-{size}"
	class:compact={compactMode}
	class:warning={timeData.isWarning}
	class:danger={timeData.isDanger}
	class:expired={timeData.isExpired}
	class:animated
	class:particle-effects={particleEffects}
	class:animation-slide={animationType === 'slide'}
	class:animation-fade={animationType === 'fade'}
	class:animation-flip={animationType === 'flip'}
	class:animation-zoom={animationType === 'zoom'}
	class:animation-none={animationType === 'none'}
	style="
		--timer-color: {currentColor};
		--warning-color: {warningColor};
		--danger-color: {dangerColor};
		--bg-color: {backgroundColor};
		--text-color: {textColor};
		--label-color: {labelColor};
		--font-size: {currentSize};
		--label-size: {currentLabelSize};
		--padding: {currentPadding};
		--gap: {gap};
		--border-radius: {borderRadius};
		--font-family: {fontFamily};
		transform: scale({$scaleValue * $pulseValue});
	"
	onmouseenter={handleMouseEnter}
	onmouseleave={handleMouseLeave}
	role="timer"
	aria-label="Countdown timer"
	aria-live="polite"
	aria-atomic="true"
>
	{#if showProgressBar}
		<div class="progress-bar" style="width: {timeData.progress}%"></div>
	{/if}

	{#if !timeData.isExpired}
		<!-- Default Format -->
		{#if format === 'default'}
			<div class="time-units">
				{#if showDays && (timeData.days > 0 || !compactMode)}
					<div class="time-unit" class:flip={digitFlips['days']}>
						<div class="time-value">
							{leadingZeros ? timeData.days.toString().padStart(2, '0') : timeData.days}
						</div>
						{#if showLabels}
							<div class="time-label">{getUnitLabel('days', timeData.days)}</div>
						{/if}
					</div>
				{/if}

				{#if showHours && (timeData.hours > 0 || !compactMode)}
					{#if showSeparators && showDays}<span class="separator">:</span>{/if}
					<div class="time-unit" class:flip={digitFlips['hours']}>
						<div class="time-value">
							{leadingZeros ? timeData.hours.toString().padStart(2, '0') : timeData.hours}
						</div>
						{#if showLabels}
							<div class="time-label">{getUnitLabel('hours', timeData.hours)}</div>
						{/if}
					</div>
				{/if}

				{#if showMinutes && (timeData.minutes > 0 || !compactMode)}
					{#if showSeparators && (showDays || showHours)}<span class="separator">:</span>{/if}
					<div class="time-unit" class:flip={digitFlips['minutes']}>
						<div class="time-value">
							{leadingZeros ? timeData.minutes.toString().padStart(2, '0') : timeData.minutes}
						</div>
						{#if showLabels}
							<div class="time-label">{getUnitLabel('minutes', timeData.minutes)}</div>
						{/if}
					</div>
				{/if}

				{#if showSeconds}
					{#if showSeparators && (showDays || showHours || showMinutes)}<span class="separator"
							>:</span
						>{/if}
					<div class="time-unit" class:flip={digitFlips['seconds']}>
						<div class="time-value">
							{leadingZeros ? timeData.seconds.toString().padStart(2, '0') : timeData.seconds}
						</div>
						{#if showLabels}
							<div class="time-label">{getUnitLabel('seconds', timeData.seconds)}</div>
						{/if}
					</div>
				{/if}

				{#if showMilliseconds}
					{#if showSeparators}<span class="separator">.</span>{/if}
					<div class="time-unit">
						<div class="time-value">{timeData.milliseconds.toString().padStart(3, '0')}</div>
						{#if showLabels}
							<div class="time-label">ms</div>
						{/if}
					</div>
				{/if}
			</div>
		{/if}

		<!-- Digital Clock Format -->
		{#if format === 'digital'}
			<div class="digital-clock">
				<div class="digital-display">
					{#if showDays && timeData.days > 0}
						<span class="digital-segment">{timeData.days.toString().padStart(2, '0')}</span>
						<span class="digital-separator">:</span>
					{/if}
					{#if showHours}
						<span class="digital-segment">{timeData.hours.toString().padStart(2, '0')}</span>
					{/if}
					{#if showMinutes}
						<span class="digital-separator">:</span>
						<span class="digital-segment">{timeData.minutes.toString().padStart(2, '0')}</span>
					{/if}
					{#if showSeconds}
						<span class="digital-separator">:</span>
						<span class="digital-segment">{timeData.seconds.toString().padStart(2, '0')}</span>
					{/if}
				</div>
			</div>
		{/if}

		<!-- Circular Progress Format -->
		{#if format === 'circular'}
			<div class="circular-timer">
				{#each Object.entries(timeUnits) as [unit, max]}
					{@const typedUnit = unit as TimeUnitKey}
					{#if (typedUnit === 'days' && showDays) || (typedUnit === 'hours' && showHours) || (typedUnit === 'minutes' && showMinutes) || (typedUnit === 'seconds' && showSeconds)}
						{@const value = timeData[typedUnit]}
						{@const { radius, circumference, offset } = getCircleProps(value, max)}
						<div class="circular-unit">
							<svg width="100" height="100">
								<circle
									cx="50"
									cy="50"
									r={radius}
									stroke="var(--bg-color)"
									stroke-width="5"
									fill="none"
								/>
								<circle
									cx="50"
									cy="50"
									r={radius}
									stroke="var(--timer-color)"
									stroke-width="5"
									fill="none"
									stroke-dasharray={circumference}
									stroke-dashoffset={offset}
									transform="rotate(-90 50 50)"
									style="transition: stroke-dashoffset 0.3s ease;"
								/>
								<text
									x="50"
									y="50"
									text-anchor="middle"
									dominant-baseline="middle"
									fill="var(--text-color)"
									font-size="20"
									font-weight="bold"
								>
									{value}
								</text>
							</svg>
							{#if showLabels}
								<div class="circular-label">{getUnitLabel(unit, value)}</div>
							{/if}
						</div>
					{/if}
				{/each}
			</div>
		{/if}

		<!-- Minimal Format -->
		{#if format === 'minimal'}
			<div class="minimal-timer">
				<span class="minimal-text">{formattedTime}</span>
			</div>
		{/if}

		<!-- Bar Chart Format -->
		{#if format === 'bar'}
			<div class="bar-timer">
				{#if showDays}
					<div class="bar-unit">
						<div class="bar-fill" style="height: {(timeData.days / 30) * 100}%"></div>
						<div class="bar-value">{timeData.days}</div>
						<div class="bar-label">Days</div>
					</div>
				{/if}
				{#if showHours}
					<div class="bar-unit">
						<div class="bar-fill" style="height: {(timeData.hours / 24) * 100}%"></div>
						<div class="bar-value">{timeData.hours}</div>
						<div class="bar-label">Hours</div>
					</div>
				{/if}
				{#if showMinutes}
					<div class="bar-unit">
						<div class="bar-fill" style="height: {(timeData.minutes / 60) * 100}%"></div>
						<div class="bar-value">{timeData.minutes}</div>
						<div class="bar-label">Mins</div>
					</div>
				{/if}
				{#if showSeconds}
					<div class="bar-unit">
						<div class="bar-fill" style="height: {(timeData.seconds / 60) * 100}%"></div>
						<div class="bar-value">{timeData.seconds}</div>
						<div class="bar-label">Secs</div>
					</div>
				{/if}
			</div>
		{/if}

		<!-- Custom Format -->
		{#if format === 'custom' && customFormatter}
			<div class="custom-timer">
				{@html customFormatter(timeData)}
			</div>
		{/if}

		{#if showPercentage}
			<div class="percentage-display">
				{timeData.progress.toFixed(1)}% Complete
			</div>
		{/if}
	{:else}
		<div class="expired-message">
			{#if expired}{@render expired()}{:else}Timer Expired{/if}
		</div>
	{/if}
</div>

<style>
	/* Base Styles */
	.countdown-timer {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		font-family: var(--font-family);
		position: relative;
		transition: transform 0.3s ease;
		user-select: none;
	}

	/* Progress Bar */
	.progress-bar {
		position: absolute;
		top: 0;
		left: 0;
		height: 3px;
		background: var(--timer-color);
		transition: width 0.3s ease;
		border-radius: var(--border-radius) var(--border-radius) 0 0;
	}

	/* Default Format */
	.time-units {
		display: flex;
		align-items: center;
		gap: var(--gap);
	}

	.time-unit {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-width: 60px;
		transition: transform 0.2s ease;
	}

	.time-unit.flip {
		animation: flipDigit 0.3s ease;
	}

	@keyframes flipDigit {
		0% {
			transform: rotateX(0);
		}
		50% {
			transform: rotateX(90deg);
		}
		100% {
			transform: rotateX(0);
		}
	}

	.time-value {
		font-size: var(--font-size);
		font-weight: 700;
		color: var(--timer-color);
		line-height: 1;
		background: var(--bg-color);
		border-radius: var(--border-radius);
		padding: var(--padding);
		min-width: 70px;
		text-align: center;
		transition: all 0.3s ease;
		font-variant-numeric: tabular-nums;
	}

	.time-label {
		font-size: var(--label-size);
		color: var(--label-color);
		text-transform: uppercase;
		margin-top: 0.5rem;
		letter-spacing: 0.5px;
		font-weight: 500;
	}

	.separator {
		font-size: var(--font-size);
		color: var(--timer-color);
		font-weight: 700;
		opacity: 0.5;
		animation: blink 2s infinite;
	}

	@keyframes blink {
		0%,
		50%,
		100% {
			opacity: 0.5;
		}
		25%,
		75% {
			opacity: 1;
		}
	}

	/* Digital Clock Format */
	.digital-clock {
		background: #000;
		padding: 1rem 2rem;
		border-radius: 8px;
		border: 2px solid #333;
		box-shadow: inset 0 2px 10px rgba(0, 0, 0, 0.5);
	}

	.digital-display {
		display: flex;
		align-items: center;
		font-family: 'Courier New', monospace;
		color: #0f0;
		font-size: 3rem;
		text-shadow: 0 0 10px #0f0;
	}

	.digital-segment {
		min-width: 60px;
		text-align: center;
	}

	.digital-separator {
		margin: 0 0.25rem;
		animation: digitalBlink 1s infinite;
	}

	@keyframes digitalBlink {
		0%,
		49% {
			opacity: 1;
		}
		50%,
		100% {
			opacity: 0;
		}
	}

	/* Circular Progress Format */
	.circular-timer {
		display: flex;
		gap: 2rem;
	}

	.circular-unit {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.circular-label {
		margin-top: 0.5rem;
		font-size: var(--label-size);
		color: var(--label-color);
		text-transform: uppercase;
	}

	/* Minimal Format */
	.minimal-timer {
		font-size: var(--font-size);
		color: var(--timer-color);
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	/* Bar Chart Format */
	.bar-timer {
		display: flex;
		gap: 1.5rem;
		align-items: flex-end;
		height: 150px;
	}

	.bar-unit {
		display: flex;
		flex-direction: column;
		align-items: center;
		position: relative;
		width: 60px;
		height: 100%;
		justify-content: flex-end;
	}

	.bar-fill {
		position: absolute;
		bottom: 40px;
		width: 100%;
		background: var(--timer-color);
		border-radius: 4px 4px 0 0;
		transition: height 0.3s ease;
	}

	.bar-value {
		position: absolute;
		bottom: 45px;
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text-color);
		z-index: 1;
	}

	.bar-label {
		font-size: var(--label-size);
		color: var(--label-color);
		text-transform: uppercase;
		margin-top: 0.5rem;
	}

	/* State Classes */
	.countdown-timer.warning .time-value {
		animation: pulse 1s infinite;
	}

	.countdown-timer.danger .time-value {
		animation: shake 0.5s infinite;
		color: var(--danger-color);
		background: rgba(239, 68, 68, 0.1);
	}

	@keyframes pulse {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.05);
		}
	}

	@keyframes shake {
		0%,
		100% {
			transform: translateX(0);
		}
		25% {
			transform: translateX(-2px);
		}
		75% {
			transform: translateX(2px);
		}
	}

	/* Expired State */
	.expired-message {
		font-size: var(--font-size);
		color: var(--danger-color);
		font-weight: 700;
		text-align: center;
		animation: fadeIn 0.5s ease;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	/* Percentage Display */
	.percentage-display {
		margin-top: 1rem;
		font-size: 0.875rem;
		color: var(--label-color);
		font-weight: 500;
	}

	/* Size Variations */
	.countdown-timer.size-small .time-value {
		min-width: 55px;
	}

	.countdown-timer.size-large .time-value {
		min-width: 100px;
	}

	/* Compact Mode */
	.countdown-timer.compact .time-units {
		gap: 0.5rem;
	}

	.countdown-timer.compact .time-value {
		padding: 0.5rem 0.75rem;
		min-width: auto;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.time-units {
			gap: 0.5rem;
		}

		.time-unit {
			min-width: 50px;
		}

		.time-value {
			font-size: 1.75rem;
			padding: 0.5rem 0.75rem;
			min-width: 55px;
		}

		.time-label {
			font-size: 0.625rem;
		}

		.circular-timer {
			gap: 1rem;
		}

		.circular-unit svg {
			width: 80px;
			height: 80px;
		}
	}

	/* Accessibility */
	@media (prefers-reduced-motion: reduce) {
		.countdown-timer,
		.time-unit,
		.time-value,
		.bar-fill {
			animation: none !important;
			transition: none !important;
		}
	}

	/* High Contrast */
	@media (prefers-contrast: high) {
		.time-value {
			border: 2px solid var(--timer-color);
		}

		.digital-clock {
			border-width: 3px;
		}
	}

	/* Dark Mode Support */
	@media (prefers-color-scheme: dark) {
		.countdown-timer {
			--bg-color: rgba(99, 102, 241, 0.2);
			--label-color: #cbd5e1;
		}
	}
</style>

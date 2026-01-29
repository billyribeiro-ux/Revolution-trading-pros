<!--
	DatePicker.svelte
	═══════════════════════════════════════════════════════════════════════════════
	Apple Principal Engineer ICT Level 7 - January 2026
	
	Custom in-app date picker with calendar dropdown.
	No native browser dialogs — everything resolves within the app.
	
	@version 1.0.0
-->
<script lang="ts">
	const DAYS_OF_WEEK = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'] as const;
	const MONTHS = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	] as const;

	interface Props {
		value: string;
		onchange: (date: string) => void;
		label?: string;
		required?: boolean;
		disabled?: boolean;
	}

	const { value, onchange, label, required = false, disabled = false }: Props = $props();

	// Local state
	let isOpen = $state(false);
	let viewDate = $state(new Date());
	let containerRef = $state<HTMLDivElement | null>(null);

	// Derived
	const days = $derived(getCalendarDays(viewDate));
	const monthYear = $derived(`${MONTHS[viewDate.getMonth()]} ${viewDate.getFullYear()}`);
	const formattedValue = $derived(formatDisplayDate(value));

	// Initialize view to selected date when opening
	$effect(() => {
		if (isOpen && value) {
			const selected = new Date(value);
			viewDate = new Date(selected.getFullYear(), selected.getMonth(), 1);
		}
	});

	// Click outside to close
	$effect(() => {
		if (!isOpen) return;

		const handler = (e: MouseEvent) => {
			if (containerRef && !containerRef.contains(e.target as Node)) {
				isOpen = false;
			}
		};

		// Delay to avoid immediate close from the toggle click
		const timer = setTimeout(() => {
			document.addEventListener('click', handler);
		}, 0);

		return () => {
			clearTimeout(timer);
			document.removeEventListener('click', handler);
		};
	});

	function getCalendarDays(date: Date): (number | null)[] {
		const year = date.getFullYear();
		const month = date.getMonth();
		const firstDay = new Date(year, month, 1).getDay();
		const daysInMonth = new Date(year, month + 1, 0).getDate();

		const result: (number | null)[] = [];
		for (let i = 0; i < firstDay; i++) result.push(null);
		for (let i = 1; i <= daysInMonth; i++) result.push(i);
		return result;
	}

	function formatDisplayDate(dateStr: string): string {
		if (!dateStr) return 'Select date';
		return new Date(dateStr).toLocaleDateString('en-US', {
			weekday: 'short',
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}

	function isSelected(day: number | null): boolean {
		if (!day || !value) return false;
		const selected = new Date(value);
		return (
			selected.getFullYear() === viewDate.getFullYear() &&
			selected.getMonth() === viewDate.getMonth() &&
			selected.getDate() === day
		);
	}

	function isToday(day: number | null): boolean {
		if (!day) return false;
		const today = new Date();
		return (
			today.getFullYear() === viewDate.getFullYear() &&
			today.getMonth() === viewDate.getMonth() &&
			today.getDate() === day
		);
	}

	function select(day: number | null) {
		if (!day) return;
		const newDate = new Date(viewDate.getFullYear(), viewDate.getMonth(), day);
		onchange(newDate.toISOString().split('T')[0]);
		isOpen = false;
	}

	function prevMonth() {
		viewDate = new Date(viewDate.getFullYear(), viewDate.getMonth() - 1, 1);
	}

	function nextMonth() {
		viewDate = new Date(viewDate.getFullYear(), viewDate.getMonth() + 1, 1);
	}

	function selectToday() {
		onchange(new Date().toISOString().split('T')[0]);
		isOpen = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') isOpen = false;
	}
</script>

<div class="date-picker" bind:this={containerRef}>
	{#if label}
		<span class="date-picker-label" id="date-picker-label">
			{label}{#if required}<span class="required">*</span>{/if}
		</span>
	{/if}

	<button
		type="button"
		class="date-picker-trigger"
		onclick={() => (isOpen = !isOpen)}
		{disabled}
		aria-expanded={isOpen}
		aria-haspopup="dialog"
		aria-labelledby={label ? 'date-picker-label' : undefined}
	>
		<span class="date-picker-value">{formattedValue}</span>
		<svg
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			width="18"
			height="18"
			aria-hidden="true"
		>
			<rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect>
			<line x1="16" y1="2" x2="16" y2="6"></line>
			<line x1="8" y1="2" x2="8" y2="6"></line>
			<line x1="3" y1="10" x2="21" y2="10"></line>
		</svg>
	</button>

	{#if isOpen}
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="calendar-dropdown"
			role="dialog"
			aria-label="Choose date"
			tabindex="-1"
			onkeydown={handleKeydown}
		>
			<div class="calendar-header">
				<button type="button" class="calendar-nav" onclick={prevMonth} aria-label="Previous month">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="18"
						height="18"
					>
						<path d="M15 18l-6-6 6-6" />
					</svg>
				</button>
				<span class="calendar-title">{monthYear}</span>
				<button type="button" class="calendar-nav" onclick={nextMonth} aria-label="Next month">
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="18"
						height="18"
					>
						<path d="M9 18l6-6-6-6" />
					</svg>
				</button>
			</div>

			<div class="calendar-weekdays">
				{#each DAYS_OF_WEEK as day}
					<span class="weekday">{day}</span>
				{/each}
			</div>

			<div class="calendar-days">
				{#each days as day}
					{#if day === null}
						<span class="day-empty"></span>
					{:else}
						<button
							type="button"
							class="day"
							class:selected={isSelected(day)}
							class:today={isToday(day)}
							onclick={() => select(day)}
						>
							{day}
						</button>
					{/if}
				{/each}
			</div>

			<div class="calendar-footer">
				<button type="button" class="today-btn" onclick={selectToday}>Today</button>
			</div>
		</div>
	{/if}
</div>

<style>
	.date-picker {
		position: relative;
	}

	.date-picker-label {
		display: block;
		font-size: 12px;
		font-weight: 600;
		color: #475569;
		margin-bottom: 6px;
	}

	.required {
		color: #ef4444;
		margin-left: 2px;
	}

	.date-picker-trigger {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 12px 14px;
		background: #f8fafc;
		border: 2px solid #e2e8f0;
		border-radius: 10px;
		font-size: 14px;
		font-weight: 500;
		color: #1e293b;
		cursor: pointer;
		transition: all 0.15s ease;
		text-align: left;
	}

	.date-picker-trigger:hover:not(:disabled) {
		border-color: #cbd5e1;
	}

	.date-picker-trigger:focus {
		outline: none;
		border-color: #143e59;
		background: #fff;
		box-shadow: 0 0 0 4px rgba(20, 62, 89, 0.1);
	}

	.date-picker-trigger:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.date-picker-value {
		flex: 1;
	}

	.date-picker-trigger svg {
		color: #64748b;
		flex-shrink: 0;
	}

	.calendar-dropdown {
		position: absolute;
		top: calc(100% + 8px);
		left: 0;
		z-index: 1000;
		width: 280px;
		background: #fff;
		border: 1px solid #e2e8f0;
		border-radius: 12px;
		box-shadow:
			0 10px 40px -8px rgba(0, 0, 0, 0.2),
			0 4px 12px rgba(0, 0, 0, 0.08);
		animation: dropdownIn 0.2s cubic-bezier(0.16, 1, 0.3, 1);
		overflow: hidden;
	}

	@keyframes dropdownIn {
		from {
			opacity: 0;
			transform: translateY(-8px) scale(0.96);
		}
		to {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	.calendar-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 14px 12px;
		background: #143e59;
		color: #fff;
	}

	.calendar-title {
		font-size: 15px;
		font-weight: 600;
	}

	.calendar-nav {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: rgba(255, 255, 255, 0.15);
		border: none;
		border-radius: 8px;
		color: #fff;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.calendar-nav:hover {
		background: rgba(255, 255, 255, 0.25);
	}

	.calendar-weekdays {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		gap: 4px;
		padding: 12px 12px 8px;
	}

	.weekday {
		text-align: center;
		font-size: 11px;
		font-weight: 600;
		color: #94a3b8;
		text-transform: uppercase;
	}

	.calendar-days {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		gap: 4px;
		padding: 0 12px 12px;
	}

	.day-empty {
		aspect-ratio: 1;
	}

	.day {
		aspect-ratio: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 500;
		color: #334155;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.day:hover {
		background: #f1f5f9;
		color: #0f172a;
	}

	.day.today {
		background: #e0f2fe;
		color: #0369a1;
		font-weight: 600;
	}

	.day.selected {
		background: #143e59;
		color: #fff;
		font-weight: 600;
	}

	.day.selected:hover {
		background: #0f2d42;
	}

	.calendar-footer {
		display: flex;
		justify-content: center;
		padding: 12px;
		border-top: 1px solid #f1f5f9;
	}

	.today-btn {
		padding: 8px 20px;
		background: #f1f5f9;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 600;
		color: #475569;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.today-btn:hover {
		background: #e2e8f0;
		color: #1e293b;
	}
</style>

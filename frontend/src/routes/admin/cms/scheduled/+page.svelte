<!--
	CMS Scheduler - Apple ICT 11+ Principal Engineer Grade
	10/10 Content Publishing Scheduler
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { fly, scale, fade } from 'svelte/transition';
	import { backOut } from 'svelte/easing';
	import {
		IconCalendarEvent,
		IconArrowLeft,
		IconClock,
		IconCalendar,
		IconFileText,
		IconTrash,
		IconEdit,
		IconCheck,
		IconX,
		IconPlayerPlay,
		IconPlayerPause,
		IconChevronLeft,
		IconChevronRight
	} from '$lib/icons';

	let mounted = false;
	let isLoading = true;
	let scheduledItems: any[] = [];
	let currentMonth = new Date();
	let selectedDate: Date | null = null;

	async function fetchScheduled() {
		isLoading = true;
		try {
			const response = await fetch('/api/admin/cms/scheduled', {
				credentials: 'include'
			});
			if (response.ok) {
				scheduledItems = await response.json();
			}
		} catch (e) {
			console.error('Failed to fetch scheduled items:', e);
		} finally {
			isLoading = false;
		}
	}

	async function cancelScheduled(id: number) {
		if (!confirm('Are you sure you want to cancel this scheduled publication?')) return;
		try {
			const response = await fetch(`/api/admin/cms/scheduled/${id}`, {
				method: 'DELETE',
				credentials: 'include'
			});
			if (response.ok) {
				fetchScheduled();
			}
		} catch (e) {
			console.error('Failed to cancel scheduled item:', e);
		}
	}

	async function publishNow(id: number) {
		try {
			const response = await fetch(`/api/admin/cms/scheduled/${id}/publish-now`, {
				method: 'POST',
				credentials: 'include'
			});
			if (response.ok) {
				fetchScheduled();
			}
		} catch (e) {
			console.error('Failed to publish:', e);
		}
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			weekday: 'short',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatTime(dateStr: string): string {
		return new Date(dateStr).toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getCalendarDays() {
		const year = currentMonth.getFullYear();
		const month = currentMonth.getMonth();
		const firstDay = new Date(year, month, 1);
		const lastDay = new Date(year, month + 1, 0);
		const startPad = firstDay.getDay();
		const days = [];

		// Previous month padding
		for (let i = startPad - 1; i >= 0; i--) {
			const d = new Date(year, month, -i);
			days.push({ date: d, isCurrentMonth: false });
		}

		// Current month
		for (let i = 1; i <= lastDay.getDate(); i++) {
			const d = new Date(year, month, i);
			days.push({ date: d, isCurrentMonth: true });
		}

		// Next month padding
		const remaining = 42 - days.length;
		for (let i = 1; i <= remaining; i++) {
			const d = new Date(year, month + 1, i);
			days.push({ date: d, isCurrentMonth: false });
		}

		return days;
	}

	function getScheduledForDate(date: Date) {
		return scheduledItems.filter(item => {
			const itemDate = new Date(item.scheduled_for);
			return itemDate.toDateString() === date.toDateString();
		});
	}

	function prevMonth() {
		currentMonth = new Date(currentMonth.getFullYear(), currentMonth.getMonth() - 1, 1);
	}

	function nextMonth() {
		currentMonth = new Date(currentMonth.getFullYear(), currentMonth.getMonth() + 1, 1);
	}

	function isToday(date: Date): boolean {
		return date.toDateString() === new Date().toDateString();
	}

	function isPast(date: Date): boolean {
		const today = new Date();
		today.setHours(0, 0, 0, 0);
		return date < today;
	}

	onMount(() => {
		mounted = true;
		fetchScheduled();
	});
</script>

<div class="scheduler-page" class:mounted>
	<!-- Header -->
	<header class="page-header" in:fly={{ y: -20, duration: 500 }}>
		<div class="header-left">
			<a href="/admin/cms" class="back-link">
				<IconArrowLeft size={18} />
				<span>Back to CMS</span>
			</a>
			<div class="header-title">
				<div class="header-icon">
					<IconCalendarEvent size={24} />
				</div>
				<div>
					<h1>Content Scheduler</h1>
					<p>Schedule and manage content publication times</p>
				</div>
			</div>
		</div>

		<div class="header-stats">
			<div class="stat-card">
				<span class="stat-value">{scheduledItems.length}</span>
				<span class="stat-label">Scheduled</span>
			</div>
			<div class="stat-card">
				<span class="stat-value">{scheduledItems.filter(i => new Date(i.scheduled_for) <= new Date(Date.now() + 86400000)).length}</span>
				<span class="stat-label">Next 24h</span>
			</div>
		</div>
	</header>

	<div class="scheduler-layout">
		<!-- Calendar -->
		<section class="calendar-section" in:fly={{ x: -20, duration: 500, delay: 100 }}>
			<div class="calendar-header">
				<button class="nav-btn" onclick={prevMonth}>
					<IconChevronLeft size={18} />
				</button>
				<h2 class="calendar-title">
					{currentMonth.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
				</h2>
				<button class="nav-btn" onclick={nextMonth}>
					<IconChevronRight size={18} />
				</button>
			</div>

			<div class="calendar-grid">
				<div class="weekdays">
					{#each ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'] as day}
						<span class="weekday">{day}</span>
					{/each}
				</div>

				<div class="days-grid">
					{#each getCalendarDays() as { date, isCurrentMonth }}
						{@const scheduled = getScheduledForDate(date)}
						<button
							class="day-cell"
							class:other-month={!isCurrentMonth}
							class:today={isToday(date)}
							class:past={isPast(date)}
							class:has-items={scheduled.length > 0}
							class:selected={selectedDate?.toDateString() === date.toDateString()}
							onclick={() => selectedDate = date}
						>
							<span class="day-number">{date.getDate()}</span>
							{#if scheduled.length > 0}
								<div class="day-indicators">
									{#each scheduled.slice(0, 3) as _}
										<span class="indicator"></span>
									{/each}
									{#if scheduled.length > 3}
										<span class="indicator-more">+{scheduled.length - 3}</span>
									{/if}
								</div>
							{/if}
						</button>
					{/each}
				</div>
			</div>
		</section>

		<!-- Scheduled Items -->
		<section class="items-section" in:fly={{ x: 20, duration: 500, delay: 100 }}>
			<div class="items-header">
				<h2>
					{#if selectedDate}
						{selectedDate.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' })}
					{:else}
						All Scheduled Items
					{/if}
				</h2>
				{#if selectedDate}
					<button class="clear-btn" onclick={() => selectedDate = null}>
						Clear Selection
					</button>
				{/if}
			</div>

			{#if isLoading}
				<div class="loading-state">
					<div class="spinner"></div>
					<p>Loading scheduled items...</p>
				</div>
			{:else}
				{@const displayItems = selectedDate ? getScheduledForDate(selectedDate) : scheduledItems}
				{#if displayItems.length === 0}
					<div class="empty-state" in:scale={{ duration: 400 }}>
						<div class="empty-icon">
							<IconCalendarEvent size={40} />
						</div>
						<h3>No scheduled items</h3>
						<p>
							{selectedDate ? 'Nothing scheduled for this date' : 'Schedule content from the content editor'}
						</p>
					</div>
				{:else}
					<div class="items-list">
						{#each displayItems as item, i}
							<div
								class="scheduled-item"
								class:past={new Date(item.scheduled_for) < new Date()}
								in:fly={{ y: 20, duration: 300, delay: i * 50 }}
							>
								<div class="item-time">
									<IconClock size={16} />
									<span>{formatTime(item.scheduled_for)}</span>
								</div>

								<div class="item-content">
									<div class="item-type">
										<IconFileText size={14} />
										<span>{item.content_type}</span>
									</div>
									<span class="item-id">#{item.content_id}</span>
								</div>

								<div class="item-date">
									<IconCalendar size={14} />
									<span>{formatDate(item.scheduled_for)}</span>
								</div>

								<div class="item-actions">
									<button
										class="action-btn publish"
										title="Publish Now"
										onclick={() => publishNow(item.id)}
									>
										<IconPlayerPlay size={14} />
									</button>
									<button
										class="action-btn cancel"
										title="Cancel"
										onclick={() => cancelScheduled(item.id)}
									>
										<IconTrash size={14} />
									</button>
								</div>
							</div>
						{/each}
					</div>
				{/if}
			{/if}
		</section>
	</div>
</div>

<style>
	.scheduler-page {
		max-width: 1400px;
		margin: 0 auto;
		padding: 0 1.5rem 3rem;
		opacity: 0;
		transform: translateY(10px);
		transition: all 0.5s ease;
	}

	.scheduler-page.mounted {
		opacity: 1;
		transform: translateY(0);
	}

	/* Header */
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		padding: 1.5rem 0 2rem;
		flex-wrap: wrap;
		gap: 1.5rem;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.85rem;
		font-weight: 500;
		color: #6366f1;
		text-decoration: none;
		margin-bottom: 1rem;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.header-icon {
		width: 56px;
		height: 56px;
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(6, 182, 212, 0.12) 0%, rgba(6, 182, 212, 0.06) 100%);
		color: #0891b2;
	}

	.header-title h1 {
		font-size: 1.75rem;
		font-weight: 800;
		color: #1e293b;
		margin: 0;
	}

	.header-title p {
		font-size: 0.9rem;
		color: #64748b;
		margin: 0.25rem 0 0 0;
	}

	.header-stats {
		display: flex;
		gap: 1rem;
	}

	.stat-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 1rem 1.5rem;
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 14px;
	}

	.stat-value {
		font-size: 1.5rem;
		font-weight: 800;
		color: #0891b2;
	}

	.stat-label {
		font-size: 0.75rem;
		font-weight: 500;
		color: #64748b;
	}

	/* Layout */
	.scheduler-layout {
		display: grid;
		grid-template-columns: 400px 1fr;
		gap: 1.5rem;
	}

	@media (max-width: 1024px) {
		.scheduler-layout {
			grid-template-columns: 1fr;
		}
	}

	/* Calendar */
	.calendar-section {
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 20px;
		padding: 1.5rem;
	}

	.calendar-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.calendar-title {
		font-size: 1.1rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.nav-btn {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #f8fafc;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 10px;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.nav-btn:hover {
		background: #f1f5f9;
		color: #6366f1;
	}

	.weekdays {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		gap: 0.25rem;
		margin-bottom: 0.5rem;
	}

	.weekday {
		text-align: center;
		font-size: 0.75rem;
		font-weight: 600;
		color: #94a3b8;
		padding: 0.5rem 0;
	}

	.days-grid {
		display: grid;
		grid-template-columns: repeat(7, 1fr);
		gap: 0.25rem;
	}

	.day-cell {
		aspect-ratio: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 10px;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
	}

	.day-cell:hover {
		background: #f8fafc;
	}

	.day-cell.other-month {
		opacity: 0.3;
	}

	.day-cell.today {
		background: rgba(6, 182, 212, 0.1);
		border-color: rgba(6, 182, 212, 0.3);
	}

	.day-cell.today .day-number {
		color: #0891b2;
		font-weight: 700;
	}

	.day-cell.past:not(.today) {
		opacity: 0.5;
	}

	.day-cell.selected {
		background: #6366f1;
		border-color: #6366f1;
	}

	.day-cell.selected .day-number {
		color: #ffffff;
	}

	.day-number {
		font-size: 0.85rem;
		font-weight: 500;
		color: #334155;
	}

	.day-indicators {
		display: flex;
		gap: 2px;
		margin-top: 0.25rem;
	}

	.indicator {
		width: 5px;
		height: 5px;
		border-radius: 50%;
		background: #0891b2;
	}

	.day-cell.selected .indicator {
		background: rgba(255, 255, 255, 0.8);
	}

	.indicator-more {
		font-size: 0.6rem;
		color: #0891b2;
	}

	/* Items Section */
	.items-section {
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.06);
		border-radius: 20px;
		padding: 1.5rem;
		min-height: 500px;
	}

	.items-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
	}

	.items-header h2 {
		font-size: 1.1rem;
		font-weight: 700;
		color: #1e293b;
		margin: 0;
	}

	.clear-btn {
		padding: 0.5rem 1rem;
		background: #f1f5f9;
		border: none;
		border-radius: 8px;
		font-size: 0.8rem;
		font-weight: 500;
		color: #64748b;
		cursor: pointer;
		transition: all 0.2s;
	}

	.clear-btn:hover {
		background: #e2e8f0;
	}

	.items-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.scheduled-item {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1rem 1.25rem;
		background: linear-gradient(135deg, rgba(248, 250, 252, 0.9) 0%, rgba(255, 255, 255, 0.7) 100%);
		border: 1px solid rgba(0, 0, 0, 0.05);
		border-radius: 14px;
		transition: all 0.2s;
	}

	.scheduled-item:hover {
		border-color: rgba(6, 182, 212, 0.25);
		box-shadow: 0 4px 15px rgba(0, 0, 0, 0.04);
	}

	.scheduled-item.past {
		opacity: 0.6;
	}

	.item-time {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		padding: 0.4rem 0.75rem;
		background: rgba(6, 182, 212, 0.1);
		border-radius: 8px;
		font-size: 0.85rem;
		font-weight: 600;
		color: #0891b2;
		min-width: 90px;
	}

	.item-content {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.item-type {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.9rem;
		font-weight: 600;
		color: #1e293b;
		text-transform: capitalize;
	}

	.item-id {
		font-size: 0.8rem;
		color: #94a3b8;
	}

	.item-date {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		font-size: 0.8rem;
		color: #64748b;
	}

	.item-actions {
		display: flex;
		gap: 0.4rem;
	}

	.action-btn {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #ffffff;
		border: 1px solid rgba(0, 0, 0, 0.08);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.action-btn.publish {
		color: #059669;
	}

	.action-btn.publish:hover {
		background: #059669;
		border-color: #059669;
		color: #ffffff;
	}

	.action-btn.cancel {
		color: #dc2626;
	}

	.action-btn.cancel:hover {
		background: #dc2626;
		border-color: #dc2626;
		color: #ffffff;
	}

	/* Loading & Empty */
	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem;
		text-align: center;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 3px solid #f1f5f9;
		border-top-color: #0891b2;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.empty-icon {
		width: 72px;
		height: 72px;
		border-radius: 18px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, rgba(6, 182, 212, 0.1) 0%, rgba(6, 182, 212, 0.05) 100%);
		color: #22d3ee;
		margin-bottom: 1rem;
	}

	.empty-state h3 {
		font-size: 1rem;
		font-weight: 700;
		color: #475569;
		margin: 0 0 0.25rem 0;
	}

	.empty-state p {
		font-size: 0.85rem;
		color: #94a3b8;
		margin: 0;
	}
</style>

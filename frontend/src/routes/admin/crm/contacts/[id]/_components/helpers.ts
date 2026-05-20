/**
 * Pure helpers shared across Contact Detail panels.
 *
 * These were inlined in +page.svelte before R15-C; lifted to a module
 * so every extracted component can reuse them without prop drilling.
 */

export function formatDate(dateString: string | undefined): string {
	if (!dateString) return 'N/A';
	return new Date(dateString).toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	});
}

export function formatDateTime(dateString: string | undefined): string {
	if (!dateString) return 'N/A';
	return new Date(dateString).toLocaleString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	});
}

export function getStatusColor(status: string): string {
	const colors: Record<string, string> = {
		subscribed: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
		pending: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
		unsubscribed: 'bg-slate-500/15 text-slate-400 border-slate-500/30',
		bounced: 'bg-red-500/15 text-red-400 border-red-500/30',
		complained: 'bg-red-500/15 text-red-400 border-red-500/30'
	};
	return colors[status] || colors.subscribed;
}

export function getLifecycleColor(stage: string): string {
	const colors: Record<string, string> = {
		subscriber: 'bg-blue-500/15 text-blue-400 border-blue-500/30',
		lead: 'bg-cyan-500/15 text-cyan-400 border-cyan-500/30',
		mql: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
		sql: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
		opportunity: 'bg-orange-500/15 text-orange-400 border-orange-500/30',
		customer: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
		evangelist: 'bg-pink-500/15 text-pink-400 border-pink-500/30'
	};
	return colors[stage] || colors.subscriber;
}

export function getEmailStatusColor(status: string): string {
	const colors: Record<string, string> = {
		sent: 'bg-blue-500/15 text-blue-400',
		opened: 'bg-emerald-500/15 text-emerald-400',
		clicked: 'bg-amber-500/15 text-amber-400',
		bounced: 'bg-red-500/15 text-red-400',
		failed: 'bg-red-500/15 text-red-400'
	};
	return colors[status] || colors.sent;
}

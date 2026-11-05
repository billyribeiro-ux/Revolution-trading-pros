/**
 * Pure helpers shared across Contact Detail panels.
 *
 * These were inlined in +page.svelte before R15-C; lifted to a module
 * so every extracted component can reuse them without prop drilling.
 */

export function formatDate(dateString: string | null | undefined): string {
	if (!dateString || !Number.isFinite(Date.parse(dateString))) return 'Unknown';
	return new Date(dateString).toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	});
}

export function formatDateTime(dateString: string | null | undefined): string {
	if (!dateString || !Number.isFinite(Date.parse(dateString))) return 'Unknown';
	return new Date(dateString).toLocaleString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	});
}

export function getStatusColor(status: string | null): string {
	const colors: Record<string, string> = {
		customer: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
		lead: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
		unqualified: 'bg-slate-500/15 text-slate-400 border-slate-500/30',
		churned: 'bg-red-500/15 text-red-400 border-red-500/30',
		lost: 'bg-red-500/15 text-red-400 border-red-500/30'
	};
	return colors[status ?? ''] || 'text-slate-400';
}

export function getLifecycleColor(stage: string | null): string {
	const colors: Record<string, string> = {
		subscriber: 'bg-blue-500/15 text-blue-400 border-blue-500/30',
		lead: 'bg-cyan-500/15 text-cyan-400 border-cyan-500/30',
		mql: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
		sql: 'bg-amber-500/15 text-amber-400 border-amber-500/30',
		opportunity: 'bg-orange-500/15 text-orange-400 border-orange-500/30',
		customer: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30',
		evangelist: 'bg-pink-500/15 text-pink-400 border-pink-500/30'
	};
	return colors[stage ?? ''] || 'text-slate-400';
}

export function getEmailStatusColor(status: string): string {
	const colors: Record<string, string> = {
		processing: 'bg-blue-500/15 text-blue-400',
		accepted: 'bg-emerald-500/15 text-emerald-400',
		unknown: 'bg-amber-500/15 text-amber-400',
		unavailable: 'bg-red-500/15 text-red-400',
		rejected: 'bg-red-500/15 text-red-400'
	};
	return colors[status] || 'text-slate-400';
}

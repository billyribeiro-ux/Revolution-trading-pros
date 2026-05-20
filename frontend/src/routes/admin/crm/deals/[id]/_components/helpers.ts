// Extracted from +page.svelte during R17-C maintenance pass.
// Pure helpers used by the deal-detail page and its child components.
//
// Per CLAUDE.md "Re-read your own diff before commit": these helpers had no
// reactivity / no closed-over state in the original — only inputs in, value
// out — so a plain `.ts` module is safe (no $state survives the move).

import type { Stage } from '$lib/crm/types';

export function formatCurrency(amount: number | undefined): string {
	if (!amount) return '$0';
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency: 'USD',
		minimumFractionDigits: 0,
		maximumFractionDigits: 0
	}).format(amount);
}

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

export function getStageColor(stage: Stage | null | undefined): string {
	if (!stage) return '#6366f1';
	if (stage.is_closed_won) return '#22c55e';
	if (stage.is_closed_lost) return '#ef4444';
	return stage.color || '#6366f1';
}

export function getPriorityColor(priority: string | undefined): string {
	const colors: Record<string, string> = {
		low: '#64748b',
		normal: '#3b82f6',
		high: '#f59e0b',
		urgent: '#ef4444'
	};
	return colors[priority || 'normal'] || colors.normal;
}

export function getStatusBadge(
	status: string | undefined
): { bg: string; color: string; text: string } {
	const badges: Record<string, { bg: string; color: string; text: string }> = {
		open: { bg: 'rgba(59, 130, 246, 0.15)', color: '#60a5fa', text: 'Open' },
		won: { bg: 'rgba(34, 197, 94, 0.15)', color: '#4ade80', text: 'Won' },
		lost: { bg: 'rgba(239, 68, 68, 0.15)', color: '#f87171', text: 'Lost' },
		abandoned: { bg: 'rgba(100, 116, 139, 0.15)', color: '#94a3b8', text: 'Abandoned' }
	};
	return badges[status || 'open'] || badges.open;
}

export interface DealNote {
	id: string;
	content: string;
	created_at: string;
	created_by?: { name: string };
}

export type ToastMessage = { type: 'success' | 'error'; text: string };

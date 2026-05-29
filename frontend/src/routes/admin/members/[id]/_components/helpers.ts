// Extracted from +page.svelte during R16-C maintenance pass.
// Pure helpers used by the member-detail page and its child components.
//
// Per CLAUDE.md "Re-read your own diff before commit": these helpers had no
// reactivity / no closed-over state in the original — only inputs in, value out
// — so a plain `.ts` module is safe (no $state survives the move).

import { IconCreditCard, IconReceipt, IconMail, IconUser, IconActivity } from '$lib/icons';
import type { Member } from '$lib/api/members';

export function getStatusColor(status: string): string {
	switch (status) {
		case 'active':
			return 'bg-emerald-500/20 text-emerald-400 border-emerald-500/30';
		case 'trial':
			return 'bg-blue-500/20 text-blue-400 border-blue-500/30';
		case 'churned':
			return 'bg-red-500/20 text-red-400 border-red-500/30';
		default:
			return 'bg-slate-500/20 text-slate-400 border-slate-500/30';
	}
}

export function getEmailStatusColor(status: string): string {
	switch (status) {
		case 'opened':
			return 'text-emerald-400';
		case 'clicked':
			return 'text-blue-400';
		case 'bounced':
			return 'text-red-400';
		default:
			return 'text-slate-400';
	}
}

/**
 * FIX-2026-04-26 (audit 02 §P3-4): null/undefined/NaN guard. The
 * `Member.total_spent` API field is nullable for never-paid users — the
 * previous version rendered "$NaN" via `Intl.NumberFormat.format(null)`.
 */
export function formatCurrency(amount: number | null | undefined): string {
	if (amount === null || amount === undefined || Number.isNaN(amount)) return '$0';
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency: 'USD',
		minimumFractionDigits: 0,
		maximumFractionDigits: 0
	}).format(amount);
}

export function formatDate(dateString: string): string {
	return new Date(dateString).toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	});
}

export function formatDateTime(dateString: string): string {
	return new Date(dateString).toLocaleString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit'
	});
}

export function getTimelineIcon(type: string) {
	switch (type) {
		case 'subscription':
			return IconCreditCard;
		case 'payment':
			return IconReceipt;
		case 'email':
			return IconMail;
		case 'login':
			return IconUser;
		case 'support':
			return IconMail;
		default:
			return IconActivity;
	}
}

export function getMemberInitials(member: Member | null): string {
	if (member?.first_name && member?.last_name) {
		return `${member.first_name[0]}${member.last_name[0]}`.toUpperCase();
	}
	return member?.name?.slice(0, 2).toUpperCase() || 'U';
}

export function getEngagementLabel(score: number): { label: string; color: string } {
	if (score >= 80) return { label: 'Highly Engaged', color: 'text-emerald-400' };
	if (score >= 60) return { label: 'Engaged', color: 'text-blue-400' };
	if (score >= 40) return { label: 'Moderate', color: 'text-yellow-400' };
	return { label: 'Low Engagement', color: 'text-red-400' };
}

export type TimelineEvent = {
	type: string;
	title: string;
	date: string;
	icon: string;
	meta?: Record<string, unknown>;
};

export type EmailHistoryItem = {
	id: number;
	subject: string;
	sent_at: string;
	status: 'sent' | 'opened' | 'clicked' | 'bounced';
	campaign_type: string;
};

export type NoteItem = {
	id: number;
	content: string;
	created_at: string;
	author: string;
};

export type MembershipPlan = {
	id: number;
	name: string;
	slug: string;
};

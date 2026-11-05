import type { MemberProfileRecord } from '#lib/api/member-profile.js';
/** Historical source values do not identify an instant. Display the recorded wall clock unchanged. */
export function recordedWallClock(value: string | null | undefined): string {
	return value === null || value === undefined
		? 'Not recorded'
		: `${value} (timezone not recorded)`;
}
export function recordedInstant(value: string | null | undefined): string {
	if (value === null || value === undefined) return 'Not recorded';
	if (!/(?:Z|[+-]\d{2}:\d{2})$/.test(value)) return `${value} (recorded source value)`;
	const milliseconds = Date.parse(value);
	if (!Number.isFinite(milliseconds)) return `${value} (recorded instant)`;
	return (
		new Intl.DateTimeFormat('en-US', {
			dateStyle: 'medium',
			timeStyle: 'short',
			timeZone: 'UTC'
		}).format(new Date(milliseconds)) + ' UTC'
	);
}
/** Preserve source precision and unknown currency; do not pass recorded NUMERIC strings through Number. */
export function recordedUnits(value: string | null | undefined, currency: string | null): string {
	if (value === null || value === undefined) return 'Not recorded';
	const match = /^(-?)(\d+)(?:\.(\d+))?$/.exec(value);
	if (!match) return `${currency || 'Currency not recorded'} ${value}`;
	const whole = (match[2] ?? '').replace(/\B(?=(\d{3})+(?!\d))/g, ',');
	return `${currency || 'Currency not recorded'} ${match[1]}${whole}.${(match[3] ?? '').padEnd(2, '0')}`;
}
export function recordedUsdCents(cents: number): string {
	if (!Number.isSafeInteger(cents) || cents < 0) throw new Error('Invalid recorded USD cents');
	return recordedUnits(`${Math.floor(cents / 100)}.${String(cents % 100).padStart(2, '0')}`, 'USD');
}
export function profileName(member: Pick<MemberProfileRecord, 'name' | 'email'>): string {
	return member.name?.trim() || member.email;
}
export function profileInitials(
	member: Pick<MemberProfileRecord, 'first_name' | 'last_name' | 'name' | 'email'>
): string {
	if (member.first_name && member.last_name)
		return `${Array.from(member.first_name)[0]}${Array.from(member.last_name)[0]}`.toUpperCase();
	return Array.from(profileName(member)).slice(0, 2).join('').toUpperCase();
}
export function cohortLabel(status: MemberProfileRecord['status']): string {
	return {
		active: 'Active',
		trial: 'Trial',
		churned: 'Churned',
		never_subscribed: 'Never subscribed'
	}[status];
}
export function accountActivityLabel(active: boolean | null): string {
	return active === null
		? 'Account activity not recorded'
		: active
			? 'Active account'
			: 'Inactive account';
}

/** Browser-only invalidation signal, containing no grants, identity or bearer data. */
export function notifyCrmDenial(status: number): void {
	if ([401, 403].includes(status) && typeof window !== 'undefined')
		window.dispatchEvent(new Event('crm-access-denied'));
}

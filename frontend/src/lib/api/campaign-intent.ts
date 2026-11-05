/** Validate reference identities before opening a draft from another admin module. */
export function campaignReferences(query: URLSearchParams) {
	function reference(key: string): number | null {
		const values = query.getAll(key);
		if (!values.length) return null;
		if (
			values.length !== 1 ||
			!/^[1-9]\d*$/.test(values[0] ?? '') ||
			!Number.isSafeInteger(Number(values[0]))
		)
			throw new Error('Choose a valid campaign template or member segment reference');
		return Number(values[0]);
	}
	return { template: reference('template'), segment: reference('segment') };
}

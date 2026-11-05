/** Confirm a downloadable export before the UI creates a file or shows success. */
export async function downloadMemberExport(
	url: string,
	format: 'csv' | 'xlsx' | 'pdf',
	token: string | null
): Promise<Blob> {
	const response = await fetch(url, {
		headers: token ? { Authorization: `Bearer ${token}` } : {},
		credentials: 'same-origin',
		signal: AbortSignal.timeout(60_000)
	});
	if (!response.ok) {
		let message = `Member export failed (${response.status})`;
		try {
			const body: unknown = await response.json();
			if (body && typeof body === 'object' && !Array.isArray(body)) {
				const record = body as Record<string, unknown>;
				const detail = typeof record.message === 'string' ? record.message : record.error;
				if (typeof detail === 'string' && detail.trim()) message = detail;
			}
		} catch {
			// The HTTP status still identifies a failed request without a JSON body.
		}
		throw new Error(message);
	}
	const expected = {
		csv: 'text/csv',
		xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
		pdf: 'application/pdf'
	}[format];
	if (response.headers.get('content-type')?.split(';')[0]?.trim() !== expected)
		throw new Error('The server did not return the requested member export format');
	const blob = await response.blob();
	if (!blob.size || blob.size > 32 * 1024 * 1024)
		throw new Error('The member export is empty or exceeds the download limit');
	const signature = new Uint8Array(await blob.slice(0, 5).arrayBuffer());
	if (
		(format === 'pdf' && new TextDecoder().decode(signature) !== '%PDF-') ||
		(format === 'xlsx' &&
			![0x50, 0x4b, 0x03, 0x04].every((byte, index) => signature[index] === byte))
	)
		throw new Error('The member export file is invalid; no download was created');
	return blob;
}

import { memberIdentity } from './member-profile.js';

function recordedRows(text: string): number {
	let quoted = false,
		closed = false,
		start = true,
		columns = 1,
		records = 0,
		unfinished = false;
	for (let index = 0; index < text.length; index++) {
		const character = text[index];
		if (quoted) {
			if (character === '"') {
				if (text[index + 1] === '"') index++;
				else {
					quoted = false;
					closed = true;
				}
			}
			continue;
		}
		if (closed && character !== ',' && character !== '\r')
			throw new Error('Invalid order CSV quoting');
		if (character === '"') {
			if (!start) throw new Error('Invalid order CSV quoting');
			quoted = true;
			start = false;
			unfinished = true;
		} else if (character === ',') {
			columns++;
			start = true;
			closed = false;
			unfinished = true;
		} else if (character === '\r') {
			if (text[index + 1] !== '\n' || columns !== 12) throw new Error('Invalid order CSV record');
			index++;
			records++;
			columns = 1;
			start = true;
			closed = false;
			unfinished = false;
		} else {
			if (character === '\n') throw new Error('Invalid order CSV record');
			start = false;
			unfinished = true;
		}
	}
	if (quoted || unfinished || records < 1) throw new Error('The order CSV is incomplete');
	return records - 1;
}

/** All recorded orders, confirmed against the actual response's scoped receipt. */
export async function downloadMemberOrders(memberId: string | number) {
	const id = memberIdentity(memberId);
	const response = await fetch(`/api/admin/member-management/${id}/orders/export?format=csv`, {
		credentials: 'same-origin',
		cache: 'no-store',
		redirect: 'error',
		signal: AbortSignal.timeout(60_000)
	});
	if (!response.ok) {
		let message = `Member order export failed (${response.status})`;
		try {
			const body: unknown = await response.json();
			if (body && typeof body === 'object' && !Array.isArray(body)) {
				const record = body as Record<string, unknown>;
				const detail = typeof record.message === 'string' ? record.message : record.error;
				if (typeof detail === 'string' && detail.trim()) message = detail;
			}
		} catch {
			/* Preserve the actual failed status when no JSON is available. */
		}
		throw new Error(message);
	}
	const count = response.headers.get('x-export-row-count'),
		sha256 = response.headers.get('x-export-sha256'),
		filename = `member-${id}-orders.csv`;
	if (
		response.headers.get('content-type')?.split(';', 1)[0].trim() !== 'text/csv' ||
		response.headers.get('content-disposition') !== `attachment; filename="${filename}"` ||
		response.headers.get('x-export-member-id') !== id ||
		response.headers.get('x-export-scope') !== 'all_recorded_member_orders' ||
		!count ||
		!/^(?:0|[1-9]\d*)$/.test(count) ||
		Number(count) > 10000 ||
		!sha256 ||
		!/^[a-f0-9]{64}$/.test(sha256)
	)
		throw new Error('The server did not confirm the requested all-order CSV export');
	const blob = await response.blob();
	if (!blob.size || blob.size > 16 * 1024 * 1024)
		throw new Error('The order export is empty or exceeds its supported size');
	const bytes = await blob.arrayBuffer(),
		actual = Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256', bytes)), (byte) =>
			byte.toString(16).padStart(2, '0')
		).join('');
	if (actual !== sha256)
		throw new Error('Order export integrity could not be confirmed; no file was created');
	const heading =
		'id,member_id,order_number,recorded_status,currency,total_units,refund_units,net_recorded_units,created_at_recorded,completed_at_recorded,refunded_at_recorded,timestamp_basis\r\n';
	const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes);
	if (!text.startsWith(heading))
		throw new Error('The order CSV does not contain the recorded-order columns');
	if (recordedRows(text) !== Number(count))
		throw new Error('Order CSV row count does not match its receipt');
	return { blob, filename, rows: Number(count), sha256 };
}

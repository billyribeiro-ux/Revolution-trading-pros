/** Read-only XLSX previews. Run from a temporary directory with bundled artifact-tool deps. */
import fs from 'node:fs/promises';
import path from 'node:path';
import { FileBlob, SpreadsheetFile } from '@oai/artifact-tool';

const directory = path.resolve(process.argv[2] ?? '');
const output = path.resolve(process.argv[3] ?? `${directory}-qa`);
if (directory === output) throw new Error('Use a separate directory for QA previews');
const expected = JSON.parse(await fs.readFile(path.join(directory, 'expected.json'), 'utf8'));
const workbook = await SpreadsheetFile.importXlsx(
	await FileBlob.load(path.join(directory, 'members.xlsx'))
);
const overview = await workbook.inspect({
	kind: 'workbook,sheet,table',
	maxChars: 2500,
	tableMaxRows: 2,
	tableMaxCols: 5
});
const windows = new Map([
	['first', [1, 9]],
	['middle', [Math.max(2, Math.floor(expected.length / 2)), Math.floor(expected.length / 2) + 7]],
	['last', [Math.max(2, expected.length - 5), expected.length + 1]]
]);
for (const [index, row] of expected.entries()) {
	if (
		Array.from(row.name ?? '').some((character) => (character.codePointAt(0) ?? 0) > 127) ||
		(row.name ?? '').length >= 200
	)
		windows.set(`special-member-${row.id}`, [
			Math.max(1, index + 1),
			Math.min(expected.length + 1, index + 3)
		]);
}
await fs.mkdir(output, { recursive: true });
await fs.writeFile(path.join(output, 'xlsx-overview.ndjson'), overview.ndjson);
const images = [];
for (const [name, [start, end]] of windows) {
	const image = await workbook.render({
		sheetName: 'Members',
		range: `A${start}:E${end}`,
		scale: 1.5,
		format: 'png'
	});
	const filename = path.join(output, `xlsx-${name}.png`);
	await fs.writeFile(filename, new Uint8Array(await image.arrayBuffer()));
	images.push({ range: `Members!A${start}:E${end}`, image: filename });
}
console.log(JSON.stringify({ source_unchanged: true, images }, null, 2));

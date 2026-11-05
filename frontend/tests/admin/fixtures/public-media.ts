import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
/** The checked-in MP4 is a locally generated2-second color clip; no external media or credentials. */
export async function publicMediaFixture(port = 19030) {
	const video = await readFile(new URL('../../fixtures/seo-public-video.mp4', import.meta.url));
	const thumbnail = await readFile(new URL('../../fixtures/test-image.png', import.meta.url));
	const server = createServer((request, response) => {
		const value =
			request.url === '/actual.mp4' ? video : request.url === '/actual.png' ? thumbnail : null;
		if (!value) {
			response.writeHead(404);
			response.end();
			return;
		}
		response.writeHead(200, {
			'Content-Type': request.url === '/actual.mp4' ? 'video/mp4' : 'image/png',
			'Content-Length': value.byteLength,
			'Cache-Control': 'no-store'
		});
		response.end(request.method === 'HEAD' ? undefined : value);
	});
	await new Promise<void>((resolve, reject) => {
		server.once('error', reject);
		server.listen(port, '127.0.0.1', resolve);
	});
	return {
		origin: `http://127.0.0.1:${port}`,
		close: async () => {
			server.closeAllConnections();
			await new Promise<void>((resolve, reject) =>
				server.close((error) => (error ? reject(error) : resolve()))
			);
		}
	};
}

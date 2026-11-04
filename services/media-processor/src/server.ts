import { createProcessor } from './app.ts';

const token = process.env.MEDIA_PROCESSOR_TOKEN ?? '';
const host = process.env.MEDIA_PROCESSOR_HOST ?? '127.0.0.1';
const port = Number(process.env.MEDIA_PROCESSOR_PORT ?? '19010');
if (!Number.isSafeInteger(port) || port < 1 || port > 65535)
	throw new Error('Invalid MEDIA_PROCESSOR_PORT');
const server = createProcessor({ token });
server.listen(port, host, () => {
	console.info(`Sharp media processor listening on ${host}:${port}`);
});
const stop = () => {
	server.close();
	server.closeIdleConnections();
};
process.once('SIGINT', stop);
process.once('SIGTERM', stop);

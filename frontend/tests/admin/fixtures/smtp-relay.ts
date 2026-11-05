import { createServer, type Socket } from 'node:net';
export async function fixtureRelay(port: number, allowed: Set<string>) {
	let connections = 0;
	const messages: Array<{ raw: string; receivedAt: number }> = [],
		sockets = new Set<Socket>();
	const relay = createServer((socket) => {
		connections += 1;
		sockets.add(socket);
		socket.on('close', () => sockets.delete(socket));
		socket.on('error', () => socket.destroy());
		let buffer = '',
			content = '',
			authenticated = false,
			data = false,
			recipient = false;
		socket.write('220 fixture.example.test ESMTP\r\n');
		socket.on('data', (chunk) => {
			buffer += chunk.toString();
			if (buffer.length + content.length > 8_000_000) return socket.destroy();
			while (buffer.includes('\r\n')) {
				const position = buffer.indexOf('\r\n'),
					line = buffer.slice(0, position);
				buffer = buffer.slice(position + 2);
				if (data) {
					if (line === '.') {
						messages.push({ raw: content, receivedAt: Date.now() });
						data = false;
						socket.end('250 2.0.0 fixture accepted\r\n');
					} else content += `${line}\r\n`;
				} else if (line.startsWith('EHLO '))
					socket.write('250-fixture.example.test\r\n250-AUTH PLAIN\r\n250 SMTPUTF8\r\n');
				else if (line.startsWith('AUTH PLAIN ')) {
					authenticated =
						Buffer.from(line.slice(11), 'base64').toString() === '\0fixture-user\0fixture-password';
					socket.write(authenticated ? '235 2.7.0 Accepted\r\n' : '535 5.7.8 Rejected\r\n');
				} else if (!authenticated && line !== 'QUIT') socket.end('530 Authentication required\r\n');
				else if (line === 'NOOP' || line === 'RSET' || line.startsWith('MAIL FROM:'))
					socket.write('250 OK\r\n');
				else if (line.startsWith('RCPT TO:')) {
					const email = line.match(/<([^>]+)>/)?.[1]?.toLowerCase();
					recipient = !!email && email.endsWith('@example.test') && allowed.has(email);
					socket.write(recipient ? '250 OK\r\n' : '550 Fixture recipient only\r\n');
				} else if (line === 'DATA') {
					if (!recipient) socket.end('550 Fixture recipient required\r\n');
					else {
						data = true;
						socket.write('354 Continue\r\n');
					}
				} else if (line === 'QUIT') socket.end('221 Bye\r\n');
				else socket.end('500 Unsupported fixture command\r\n');
			}
		});
	});
	await new Promise<void>((resolve, reject) => {
		relay.once('error', reject);
		relay.listen(port, '127.0.0.1', resolve);
	});
	return {
		messages,
		get connections() {
			return connections;
		},
		close: async () => {
			for (const socket of sockets) socket.destroy();
			await new Promise<void>((resolve, reject) =>
				relay.close((error) => (error ? reject(error) : resolve()))
			);
		}
	};
}
export function mimeText(raw: string): string {
	const decoded = raw
		.split('Content-Transfer-Encoding: base64\r\n\r\n')
		.slice(1)
		.map((part) =>
			Buffer.from((part.split('\r\n--')[0] ?? '').replace(/\r\n/g, ''), 'base64').toString()
		)
		.join('\n');
	return (
		decoded ||
		raw
			.replace(/=\r\n/g, '')
			.replace(/=([0-9A-F]{2})/gi, (_, hex: string) => String.fromCharCode(parseInt(hex, 16)))
	);
}

/** Compare decoded RFC 2047 headers, including the folding used by lettre. */
export function mimeSubject(raw: string): string {
	const headers = (raw.split('\r\n\r\n')[0] ?? '').replace(/\r\n[ \t]+/g, ' ');
	const value = headers.match(/^Subject:[ \t]*(.*)$/im)?.[1] ?? '';
	return value
		.replace(/(\?=)[ \t]+(=\?)/g, '$1$2')
		.replace(/=\?utf-8\?([bq])\?([^?]*)\?=/gi, (_, encoding: string, encoded: string) => {
			if (encoding.toLowerCase() === 'b') return Buffer.from(encoded, 'base64').toString('utf8');
			const bytes: number[] = [];
			const text = encoded.replace(/_/g, ' ');
			for (let i = 0; i < text.length; i++) {
				if (text[i] === '=' && /^[0-9a-f]{2}$/i.test(text.slice(i + 1, i + 3))) {
					bytes.push(parseInt(text.slice(i + 1, i + 3), 16));
					i += 2;
				} else bytes.push(text.charCodeAt(i));
			}
			return Buffer.from(bytes).toString('utf8');
		});
}

/** Development-only Svelte source inspector for Vite. */

import type { Plugin } from 'vite';
import { randomBytes } from 'node:crypto';
import { spawn } from 'node:child_process';
import { existsSync, realpathSync, statSync } from 'node:fs';
import { isAbsolute, relative, resolve, sep } from 'node:path';
import { RUNTIME_SCRIPT } from './runtime';

export interface ClickToSourceOptions {
	hotkey?: 'alt' | 'ctrl' | 'meta' | 'shift';
	editor?: 'vscode' | 'cursor' | 'windsurf' | 'webstorm' | 'vim' | 'neovim';
	highlight?: boolean;
	highlightColor?: string;
	enabled?: boolean;
	/** Permit requests from non-loopback clients. Keep false unless the dev server is trusted. */
	allowRemote?: boolean;
	/** Optional trusted executable path or command name for a GUI editor. */
	editorCommand?: string;
}

const VIRTUAL_MODULE_ID = 'virtual:click-to-source';
const RESOLVED_VIRTUAL_MODULE_ID = '\0' + VIRTUAL_MODULE_ID;
const OPEN_ENDPOINT = '/__click-to-source-open';
const MAX_POSITION = 1_000_000;

function detectEditor(): NonNullable<ClickToSourceOptions['editor']> {
	const values = [process.env.TERM_PROGRAM, process.env.EDITOR, process.env.VISUAL]
		.filter((value): value is string => Boolean(value))
		.map((value) => value.toLowerCase());
	for (const name of ['cursor', 'windsurf', 'nvim', 'vim', 'webstorm'] as const) {
		if (values.some((value) => value.includes(name))) {
			return name === 'nvim' ? 'neovim' : name;
		}
	}
	return 'vscode';
}

function getEditorCommand(editor: NonNullable<ClickToSourceOptions['editor']>): string {
	const commands: Record<NonNullable<ClickToSourceOptions['editor']>, string[]> = {
		vscode: [
			'code',
			'/usr/local/bin/code',
			'/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code'
		],
		cursor: [
			'cursor',
			'/usr/local/bin/cursor',
			'/Applications/Cursor.app/Contents/Resources/app/bin/cursor'
		],
		windsurf: [
			'windsurf',
			'/usr/local/bin/windsurf',
			'/Applications/Windsurf.app/Contents/Resources/app/bin/windsurf'
		],
		webstorm: ['webstorm', '/usr/local/bin/webstorm'],
		vim: ['vim'],
		neovim: ['nvim']
	};
	const candidates = commands[editor];
	return (
		candidates.find((candidate) => candidate.startsWith('/') && existsSync(candidate)) ??
		candidates[0]
	);
}

export function resolveSourceFile(root: string, file: string): string | null {
	if (!file || file.includes('\0') || isAbsolute(file) || !file.endsWith('.svelte')) return null;
	try {
		const rootPath = realpathSync(root);
		const filePath = realpathSync(resolve(rootPath, file));
		const withinRoot = relative(rootPath, filePath);
		if (
			!withinRoot ||
			withinRoot === '..' ||
			withinRoot.startsWith(`..${sep}`) ||
			isAbsolute(withinRoot)
		) {
			return null;
		}
		return statSync(filePath).isFile() ? filePath : null;
	} catch {
		return null;
	}
}

function validPosition(value: string | null): value is string {
	if (!value || !/^[1-9]\d{0,6}$/.test(value)) return false;
	return Number(value) <= MAX_POSITION;
}

function isLoopback(address: string | undefined): boolean {
	return (
		address === '::1' ||
		address?.startsWith('127.') === true ||
		address?.startsWith('::ffff:127.') === true
	);
}

export function clickToSource(options: ClickToSourceOptions = {}): Plugin {
	const {
		hotkey = 'alt',
		editor = detectEditor(),
		highlight = true,
		highlightColor = '#3b82f6',
		enabled = true,
		allowRemote = false,
		editorCommand
	} = options;
	const requestToken = randomBytes(32).toString('base64url');
	let isDev = false;
	let projectRoot = '';
	const editorCmd = editorCommand ?? getEditorCommand(editor);

	return {
		name: 'vite-plugin-click-to-source',
		enforce: 'pre',
		configResolved(config) {
			isDev = config.command === 'serve';
			projectRoot = config.root;
			if (isDev && enabled) {
				console.log(
					`[click-to-source] Active. Editor: ${editor}. Hotkey: ${hotkey.toUpperCase()}+Click`
				);
			}
		},
		configureServer(server) {
			if (!enabled || !isDev) return;
			server.middlewares.use((req, res, next) => {
				let url: URL;
				try {
					url = new URL(req.url ?? '/', 'http://localhost');
				} catch {
					return next();
				}
				if (url.pathname !== OPEN_ENDPOINT) return next();
				res.setHeader('Cache-Control', 'no-store');
				res.setHeader('Content-Type', 'application/json');
				if (!allowRemote && !isLoopback(req.socket.remoteAddress)) {
					res.statusCode = 403;
					res.end(JSON.stringify({ error: 'Remote requests are disabled' }));
					return;
				}
				if (req.method !== 'POST') {
					res.statusCode = 405;
					res.setHeader('Allow', 'POST');
					res.end(JSON.stringify({ error: 'Method not allowed' }));
					return;
				}
				if (req.headers['x-click-to-source-token'] !== requestToken) {
					res.statusCode = 403;
					res.end(JSON.stringify({ error: 'Forbidden' }));
					return;
				}
				if (req.headers.origin) {
					try {
						if (new URL(req.headers.origin).host !== req.headers.host) {
							res.statusCode = 403;
							res.end(JSON.stringify({ error: 'Forbidden' }));
							return;
						}
					} catch {
						res.statusCode = 403;
						res.end(JSON.stringify({ error: 'Forbidden' }));
						return;
					}
				}
				const file = url.searchParams.get('file');
				const line = url.searchParams.get('line');
				const col = url.searchParams.get('col');
				const sourcePath = file ? resolveSourceFile(projectRoot, file) : null;
				if (!sourcePath || !validPosition(line) || !validPosition(col)) {
					res.statusCode = 400;
					res.end(JSON.stringify({ error: 'Invalid source location' }));
					return;
				}
				const args =
					editor === 'vim' || editor === 'neovim'
						? [`+${line}`, sourcePath]
						: ['--goto', `${sourcePath}:${line}:${col}`];
				const child = spawn(editorCmd, args, { detached: true, stdio: 'ignore', shell: false });
				child.once('error', () => {
					if (res.writableEnded) return;
					res.statusCode = 500;
					res.end(JSON.stringify({ error: 'Editor could not be started' }));
				});
				child.once('spawn', () => {
					child.unref();
					if (res.writableEnded) return;
					res.statusCode = 200;
					res.end(JSON.stringify({ ok: true }));
				});
			});
		},
		resolveId(id) {
			return id === VIRTUAL_MODULE_ID ? RESOLVED_VIRTUAL_MODULE_ID : undefined;
		},
		load(id) {
			if (id !== RESOLVED_VIRTUAL_MODULE_ID) return undefined;
			if (!isDev || !enabled) return 'export default {}';
			return RUNTIME_SCRIPT.replace('__HOTKEY__', JSON.stringify(hotkey))
				.replace('__HIGHLIGHT_ENABLED__', JSON.stringify(highlight))
				.replace('__HIGHLIGHT_COLOR__', JSON.stringify(highlightColor))
				.replace('__OPEN_ENDPOINT__', JSON.stringify(OPEN_ENDPOINT))
				.replace('__REQUEST_TOKEN__', JSON.stringify(requestToken));
		},
		transformIndexHtml() {
			if (!isDev || !enabled) return [];
			return [
				{
					tag: 'script',
					attrs: { type: 'module' },
					children: `import '${VIRTUAL_MODULE_ID}'`,
					injectTo: 'body'
				}
			];
		}
	};
}

export { createPreprocessor } from './preprocessor';
export default clickToSource;

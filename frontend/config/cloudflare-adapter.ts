import adapter from '@sveltejs/adapter-cloudflare';
import type { Adapter } from '@sveltejs/kit';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { planCloudflareRoutes, type PagesRoutePlan } from '../src/lib/config/cloudflare-routes.js';
import { appendStaticNoindexHeaders } from '../src/lib/config/cloudflare-headers.js';

function staticRedirects(assets: string): string[] {
	const source = ['_redirects', join(assets, '_redirects')].find((file) => existsSync(file));
	if (!source) return [];
	return readFileSync(source, 'utf8')
		.split('\n')
		.map((line) => line.trim())
		.filter((line) => line && !line.startsWith('#'))
		.map((line) => {
			const path = line.split(/\s+/)[0];
			if (!path || !path.startsWith('/') || path.includes('/:'))
				throw new Error(`Unsupported Cloudflare redirect source: ${path}`);
			return path;
		});
}
/** Use the adapter's actual written asset list, retaining its other hooks/defaults. */
export function createCloudflareAdapter(): Adapter {
	const options: { routes: PagesRoutePlan } = { routes: { include: ['/*'], exclude: [] } };
	const base = adapter(options);
	return {
		...base,
		async adapt(builder) {
			const redirects = [
				...staticRedirects(builder.config.files.assets),
				...builder.prerendered.redirects.keys()
			];
			let outputRoot: string | undefined;
			await base.adapt({
				...builder,
				writeClient(destination) {
					outputRoot = builder.config.paths.base
						.split('/')
						.filter(Boolean)
						.reduce((root) => dirname(root), destination);
					const assets = builder.writeClient(destination);
					options.routes = planCloudflareRoutes({
						appPath: builder.getAppPath(),
						appDir: builder.config.appDir,
						base: builder.config.paths.base,
						clientAssets: assets,
						prerenderedPaths: builder.prerendered.paths,
						redirectPaths: redirects
					});
					builder.log.info(
						`Cloudflare routes: ${1 + options.routes.exclude.length}/100 rules; exact public HTML/assets, runtime private/data paths`
					);
					return assets;
				}
			});
			if (!outputRoot) throw new Error('Cloudflare adapter did not write client assets');
			const headersFile = join(outputRoot, '_headers');
			writeFileSync(
				headersFile,
				appendStaticNoindexHeaders(
					readFileSync(headersFile, 'utf8'),
					builder.prerendered.paths,
					builder.config.paths.base
				)
			);
		}
	};
}

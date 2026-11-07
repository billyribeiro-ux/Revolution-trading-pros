# Imagify

Portable, byte-only image optimization for Node.js 20.9+. The package is independent of the Revolution API and can be installed from its built tarball in another Node project. It uses Sharp 0.35.5 and never fetches a caller URL. `optimizeWebp()` compares a full-size WebP with the input and returns the smaller file. `encodeWebp()` always returns WebP when that format is required.

## Use in another project

Build a distributable tarball, then install it in another project:

```sh
pnpm --filter @revolution/imagify pack --pack-destination /tmp
pnpm add /tmp/revolution-imagify-1.0.0.tgz
```

For an existing pnpm workspace, add `imagify` to `pnpm-workspace.yaml` and declare `"@revolution/imagify": "workspace:*"` in the consumer package. Run `pnpm --filter @revolution/imagify build` before starting a workspace consumer. Packing builds JavaScript and declarations automatically. The tarball bundles code and types; the consuming project installs Sharp as the package dependency.

```ts
import { readFile } from 'node:fs/promises';
import { optimizeWebp, encodeWebp } from '@revolution/imagify';

const source = await readFile('photo.png');
const best = await optimizeWebp(source); // source bytes if WebP is larger
const forced = await encodeWebp(source, { quality: 80, effort: 4 });
```

Run once on a file or directory:

```sh
pnpm exec imagify ./downloads ./optimized
```

Watch a downloads directory and convert new or changed JPEG, PNG, GIF, WebP, or AVIF files automatically:

```sh
pnpm exec imagify ./downloads ./optimized --watch
```

The output directory must be separate from the input. Outputs use stable source-derived suffixes so similarly named files cannot overwrite each other. A JSON manifest records the source, chosen output, dimensions, byte sizes, and savings. Symbolic links are ignored. The watcher polls every two seconds, converts only new or changed images, and removes outputs for deleted inputs. The CLI's default is quality 80 and WebP effort 4; `--quality=...` and `--effort=...` tune the trade-off. The source directory should be treated as trusted local input: the package validates image bytes, pixels, frame count, and output size, but it is not a network downloader or a malware scanner.

The package handles up to 50 MiB per image, 16 million decoded pixels across at most 256 frames, and WebP encoding with a 25 second timeout. It never enlarges an image when resizing. Sharp strips metadata by default, which removes EXIF information. Animated inputs remain animated in WebP. No API can safely promise that a given format is always the smallest at equal visual quality.

## Repository integration

`services/media-processor` now uses `encodeWebp()` from this package for its WebP HTTP endpoint. The Rust media API queues a default optimization job automatically, in the same database transaction, for supported raster images created through direct upload, presigned confirmation, and replacement. The job scans the stored original with ClamAV before decoding, stores WebP variants separately, and preserves the original for recovery. If `MEDIA_PROCESSOR_URL` is unset, media uploads continue but automatic optimization is disabled. Configure the processor, token, object storage, ClamAV, and the Rust API as described in `services/media-processor/README.md` before relying on automatic jobs.

The website has a public `/api/media/image/{token}` selector. New blog featured images uploaded through the editor use that URL, and blog images request responsive widths through `srcset`. The selector redirects to a smaller, completed WebP or AVIF variant only when the browser accepts it; otherwise it uses the scanned original. It does not rewrite URLs already saved in posts, arbitrary browser downloads, or third-party URLs. Those require explicit integration or a content migration in each project. The admin library can also retrieve and preview stored variants.

The original remains the master for replacement history, recovery, and future encoders. The worker skips a derived image when it would use at least as many bytes as the source. Automatically deleting originals would make re-encoding and restoration impossible and is not the default storage policy.

## Evidence and tuning

Google's [WebP FAQ](https://developers.google.com/speed/webp/faq) says WebP can be larger than a small palette PNG or an already compressed JPEG. Sharp's [encoder documentation](https://sharp.pixelplumbing.com/api-output/#webp) defines `effort` as a speed and compression trade-off. Sharp also documents [AVIF output](https://sharp.pixelplumbing.com/api-output/#avif), which can beat WebP on some images. Its [constructor limits](https://sharp.pixelplumbing.com/api-constructor/) motivate the pixel and frame caps here.

Run a repeatable comparison with your own corpus:

```sh
node imagify/src/benchmark.ts path/to/image.png path/to/another.jpg
```

On this workspace's 159,939-byte, 3163×835 PNG, one local run produced 46,434-byte WebP at quality 80/effort 4 in 108 ms. Effort 6 produced 45,646 bytes in 1,649 ms; effort 0 produced 129,782 bytes in 54 ms. AVIF quality 60/effort 4 produced 36,901 bytes in 170 ms. These are single-run measurements on one machine, not a fair visual-quality comparison or a universal speed ranking. They show why “always WebP,” “smallest ever,” and “fastest” cannot all be guaranteed at once. The default favors useful compression with bounded CPU work; benchmark your actual image mix before changing it.

## Validation

```sh
pnpm --filter @revolution/imagify check
pnpm --filter @revolution/imagify test
pnpm --filter @revolution/imagify pack --pack-destination /tmp
pnpm --filter @revolution/media-processor check
pnpm --filter @revolution/media-processor test
```

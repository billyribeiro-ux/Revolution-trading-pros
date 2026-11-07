# Media processor

This workspace runs the Sharp version pinned by the root lockfile. The Rust API
sends stored image bytes to it; the service never downloads caller URLs or opens
caller paths. Originals stay in storage and completed jobs publish derived files.

From the repository root, install with `pnpm install --frozen-lockfile`. Use Node
26.13 or later. Generate a token with `openssl rand -hex 32`, then supply it through
your environment to both processes (do not commit it):

```sh
export MEDIA_PROCESSOR_TOKEN='<generated token>'
pnpm --filter @revolution/media-processor start
```

For the Rust API, also set `MEDIA_PROCESSOR_URL=http://127.0.0.1:19010`. Restart
the API after configuration changes. Migrations create the balanced preset, job
queue and variant records; the API starts the recurring worker when the processor
URL is configured. Missing configuration leaves unrelated API modules available.
Enabled configuration requires a valid HTTP(S) base URL and a 32–256 character
visible ASCII token. An unreachable enabled processor reports an unavailable
health check; processing attempts retain errors and retry with backoff.

`CLAMD_ADDRESS` must point to an actual ClamAV daemon for processing to succeed.
Every attempt retrieves and scans the stored original before decoding. Unavailable
scanners cannot produce optimized or clean results. Original and derived object
deletion uses the configured S3-compatible storage service.

The service binds loopback by default. For separate containers, set
`MEDIA_PROCESSOR_HOST=0.0.0.0` inside a private network and use its internal service
URL in the API. Keep the token private; use HTTPS when traffic crosses a trusted
local/container boundary. No browser calls the processor directly.

Limits are 50 MiB per image, 16 million decoded pixels across all frames, 256
frames, one concurrent request, and a 25 second codec timeout. Larger images fail
with a useful error. WebP/GIF preserve animations; animated AVIF/JPEG/PNG conversion
is rejected to avoid silently losing frames. Resize never enlarges an original.
The balanced preset generates full WebP and widths 320, 768 and 1280 where smaller
than the source dimensions. A derivative is stored only when its bytes are smaller
than the original. The original remains available for recovery, revisions and
future encoders. The website's public selector requires a valid `R2_PUBLIC_URL`;
new blog featured images use it automatically once the API and processor are
deployed with the required storage and scanner configuration.

Verify with `pnpm --filter @revolution/media-processor check` and
`pnpm --filter @revolution/media-processor test`. HTTP tests run the real codecs,
including actual dimensions, alpha, animation, authentication and resource limits.

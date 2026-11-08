# Media processor

This workspace runs the Sharp version pinned by the root lockfile. The Rust API
sends stored image bytes to it; the service never downloads caller URLs or opens
caller paths. Originals stay in storage and completed jobs publish derived files.

## Docker Compose

Use the optional overlay to build and start the processor on the private Compose
network. Set one shared token in your shell or an untracked `.env` file; the
overlay refuses to start without it.

```sh
export MEDIA_PROCESSOR_TOKEN="$(openssl rand -hex 32)"
docker compose -f docker-compose.yml -f docker-compose.media.yml up -d --build db redis meili media-processor api
```

The overlay gives the API the processor's internal URL and token. The processor
runs as a nonroot user, has a read-only filesystem, drops Linux capabilities,
and exposes no host port. Its health check authenticates to `/health`. Memory
and CPU defaults are 1 GiB and two CPUs; set `MEDIA_PROCESSOR_MEMORY_LIMIT` and
`MEDIA_PROCESSOR_CPUS` to tune them for a measured workload. A valid object
store and `CLAMD_ADDRESS` are still required for upload-to-variant processing:
the worker fails closed when it cannot scan the stored original. The base
Compose stack does not supply either service, so starting this overlay alone
proves the private processor service, not a complete media upload pipeline.

On 2026-11-08, the Docker image built on Apple silicon and a running container
returned HTTP 401 without a token, HTTP 200 for authenticated health, and a
decodable 512×512 WebP from a generated PNG. That sample was 377,747 bytes as
PNG and 5,834 bytes as WebP. The production-dependency-only runtime image was
400,841,624 bytes, versus 707,806,780 bytes when development dependencies
were copied into it. These numbers describe one platform and one synthetic
image, not a representative compression or speed benchmark.

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

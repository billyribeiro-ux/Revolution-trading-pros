# `!important` is BANNED — zero exceptions

**Effective:** 2026-04-25 · **Status:** absolute prohibition. The previous
"94 legitimate" exemption registry is REVOKED.

The repo contains **zero** `!important` declarations as of this date. Verify
with:

```bash
grep -rn ': .*!important;' frontend/src --include='*.css' --include='*.svelte' | wc -l
# expected: 0
```

Any PR that re-introduces `!important` fails review on principle. There are
no legitimate categories. Every previously "legitimate" use case has a
better solution:

| Old "legitimate" use                                                              | Fix without `!important`                                                                                                                                                                                                                              |
| --------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `@media print { * { background: white !important } }`                             | Bump selector to `html *` (specificity 0,0,2) and rely on cascade order — print stylesheets load last.                                                                                                                                                |
| `@media (prefers-reduced-motion) { * { animation-duration: 0.01ms !important } }` | Same: bump to `html *` or scope to a wrapper class (`.dashboard *`, `.navbar *`, etc.) — specificity wins, no `!important` needed.                                                                                                                    |
| Fullscreen overlay reset (`.fullscreen { position: fixed !important }`)           | Use a compound selector (`.video-embed-container.fullscreen`, specificity 0,2,0) that beats the base class rule (0,1,0) — no `!important`.                                                                                                            |
| Fight inline `style="..."` set by JS or `style:prop=`                             | Convert the inline binding to an inline CSS variable on a _parent_ element (`style:--foo={bar}`), have the child read `var(--foo)`. The media-query rule then sets `--foo` directly on the child, which wins over the inherited inline value cleanly. |
| Three.js / canvas libraries that write to `canvas.style.width/height`             | Don't fight it. The library is doing the right thing (ResizeObserver-driven `setSize()`). Trust the library; remove the override entirely.                                                                                                            |

## Stylelint enforcement

Recommended next step — turn this into a hard CI check:

```js
// stylelint.config.js
{
  "rules": {
    "declaration-no-important": [true, { "severity": "error" }]
  }
}
```

There are no per-file exceptions. There is no escape hatch.

## History

- **2026-04-25** — initial registry (321 declarations, 71% reduction → 94 "legitimate" remained).
- **2026-04-25 (revoked same day)** — exemption registry abolished. All
  remaining 94 declarations eliminated. Repo state: **zero `!important`**.
  See commit referenced in this file's git log.

# Rendering backends & software fallback

`mc-scan` builds iced with **both** the `wgpu` (GPU) and `tiny-skia` (CPU)
renderers enabled (see `Cargo.toml`). iced composes them into a fallback
compositor: at startup it tries the GPU backend first and, if that fails to
initialize, automatically falls back to the software backend. No code change is
needed to get the fallback — enabling both features is what turns it on.

## Choosing a backend at runtime

iced reads the `ICED_BACKEND` environment variable (comma-separated list of
candidates, tried in order). For each candidate it attempts wgpu, then tiny-skia.

| Goal | Command |
| --- | --- |
| Default (auto: wgpu → tiny-skia) | *(unset)* |
| Force the software renderer | `ICED_BACKEND=tiny-skia ./mc-scan` |
| Pin a specific wgpu backend | `ICED_BACKEND=vulkan ./mc-scan` (also `gl`, `dx12`, `metal`) |

`WGPU_BACKEND` is also honored by wgpu itself for finer control, and
`WGPU_POWER_PREF=low` can pick an integrated GPU.

Forcing `tiny-skia` works because wgpu rejects the unknown backend name and the
compositor drops to the software renderer — the same path taken automatically
when GPU init genuinely fails.

## Verification checklist (CROSS-12)

The fallback is initialization-dependent, so it must be smoke-tested on the
configurations where wgpu is most likely to be unavailable:

- [x] **Software renderer forced** — `ICED_BACKEND=tiny-skia`: window opens and
      renders, no panic or black window. *(verified on Linux)*
- [ ] Headless/VM with no GPU (e.g. `LIBGL_ALWAYS_SOFTWARE=1` / llvmpipe).
- [ ] Windows over RDP (no hardware acceleration).
- [ ] Old/blocklisted GPUs and macOS under screen sharing.

On each, confirm the app starts, the UI is drawn, and scrolling/scanning work.
If wgpu hangs rather than failing cleanly on some target, force `tiny-skia` there
via `ICED_BACKEND`.

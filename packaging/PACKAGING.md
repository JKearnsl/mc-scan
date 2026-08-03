# Packaging

The repo is a Cargo workspace: the scanning core is the `scanner` library
(`crates/scanner`) and the GUI front-end is the `gui` crate (`crates/gui`), which
builds the `mc-scan` binary and owns `build.rs`, `assets/` and the
`[package.metadata.bundle]` metadata. Build outputs land in the shared workspace
`target/`.

Build the GUI with `cargo build --release --bin mc-scan` (→ `target/release/mc-scan`,
or `mc-scan.exe` on Windows). This directory holds the platform packaging bits.

## Assets

Icons live in the GUI crate, at `crates/gui/assets/`.

- `icon.png` — 256×256 source icon (embedded in the window at runtime).
- `icon.ico` — multi-size Windows icon, embedded into the `.exe` by `build.rs`
  (via `winresource`). Regenerate from the PNG with ImageMagick:

  ```sh
  cd crates/gui
  magick assets/icon.png -define icon:auto-resize=256,128,64,48,32,16 assets/icon.ico
  ```

- `packaging/mc-scan.desktop` — Linux desktop entry. Its `StartupWMClass` matches
  the window `application_id` (`mc-scan`) so the compositor links the window to
  the icon.

## Linux

Install the binary, icon and desktop entry (from the repo root):

```sh
install -Dm755 target/release/mc-scan              ~/.local/bin/mc-scan
install -Dm644 crates/gui/assets/icon.png      ~/.local/share/icons/hicolor/256x256/apps/mc-scan.png
install -Dm644 packaging/mc-scan.desktop           ~/.local/share/applications/mc-scan.desktop
```

For distributable artifacts (`.deb`, `AppImage`) use
[`cargo bundle`](https://github.com/burtonageo/cargo-bundle) or
[`cargo dist`](https://github.com/axodotdev/cargo-dist). `cargo bundle` reads the
current package's `[package.metadata.bundle]`, so run it from the GUI crate:

```sh
cargo install cargo-bundle
cd crates/gui
cargo bundle --release --format deb        # or: appimage
```

## macOS

`cargo bundle` produces an `.app` (and `.dmg`) from the same metadata:

```sh
cd crates/gui
cargo bundle --release                     # → target/release/bundle/osx/mc-scan.app
```

The `identifier` (`com.jkearnsl.mc-scan`) and `icon` come from
`[package.metadata.bundle]`. For a proper `.icns` icon, convert the PNG on a Mac
(`iconutil`/`sips`) or via ImageMagick (`magick assets/icon.png assets/icon.icns`)
and add it to the `icon = [...]` list.

## Windows

`build.rs` embeds `assets/icon.ico` (and a default manifest) into the `.exe` at
build time, so a plain `cargo build --release --bin mc-scan` already yields an icon'd
binary. For an installer, feed `target/release/mc-scan.exe` to WiX or NSIS, or use
`cargo bundle --release` / `cargo dist`.

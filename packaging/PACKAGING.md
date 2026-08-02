# Packaging

`mc-scan` is a single GUI binary (`cargo build --release` → `target/release/mc-scan`,
or `mc-scan.exe` on Windows). This directory holds the platform packaging bits.

## Assets

- `assets/icon.png` — 256×256 source icon (embedded in the window at runtime).
- `assets/icon.ico` — multi-size Windows icon, embedded into the `.exe` by
  `build.rs` (via `winresource`). Regenerate from the PNG with ImageMagick:

  ```sh
  magick assets/icon.png -define icon:auto-resize=256,128,64,48,32,16 assets/icon.ico
  ```

- `packaging/mc-scan.desktop` — Linux desktop entry. Its `StartupWMClass` matches
  the window `application_id` (`mc-scan`) so the compositor links the window to
  the icon.

## Linux

Install the binary, icon and desktop entry:

```sh
install -Dm755 target/release/mc-scan            ~/.local/bin/mc-scan
install -Dm644 assets/icon.png                   ~/.local/share/icons/hicolor/256x256/apps/mc-scan.png
install -Dm644 packaging/mc-scan.desktop         ~/.local/share/applications/mc-scan.desktop
```

For distributable artifacts (`.deb`, `AppImage`) use
[`cargo bundle`](https://github.com/burtonageo/cargo-bundle) or
[`cargo dist`](https://github.com/axodotdev/cargo-dist); the
`[package.metadata.bundle]` section in `Cargo.toml` drives the former:

```sh
cargo install cargo-bundle
cargo bundle --release --format deb        # or: appimage
```

## macOS

`cargo bundle` produces an `.app` (and `.dmg`) from the same metadata:

```sh
cargo bundle --release                     # → target/release/bundle/osx/mc-scan.app
```

The `identifier` (`com.jkearnsl.mc-scan`) and `icon` come from
`[package.metadata.bundle]`. For a proper `.icns` icon, convert the PNG on a Mac
(`iconutil`/`sips`) or via ImageMagick (`magick assets/icon.png assets/icon.icns`)
and add it to the `icon = [...]` list.

## Windows

`build.rs` embeds `assets/icon.ico` (and a default manifest) into the `.exe` at
build time, so a plain `cargo build --release` already yields an icon'd binary.
For an installer, feed `target/release/mc-scan.exe` to WiX or NSIS, or use
`cargo bundle --release` / `cargo dist`.

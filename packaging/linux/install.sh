#!/usr/bin/env bash
# Install the mc-scan binary, .desktop entry and hicolor icons.
#
# Usage:
#   ./install.sh            # per-user install into ~/.local (no root)
#   sudo ./install.sh --system   # system-wide install into /usr
#
# Run `cargo build --release` first so target/release/mc-scan exists.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$here/../.." && pwd)"

mode="user"
if [[ "${1:-}" == "--system" ]]; then
    mode="system"
fi

if [[ "$mode" == "system" ]]; then
    prefix="/usr"
    bindir="$prefix/bin"
    datadir="$prefix/share"
else
    prefix="${XDG_DATA_HOME:-$HOME/.local/share}"
    bindir="$HOME/.local/bin"
    datadir="$prefix"
fi

apps_dir="$datadir/applications"
icons_dir="$datadir/icons/hicolor"

echo "Installing mc-scan ($mode) into:"
echo "  binary:   $bindir"
echo "  desktop:  $apps_dir"
echo "  icons:    $icons_dir"

# Binary
bin_src="$repo_root/target/release/mc-scan"
if [[ ! -x "$bin_src" ]]; then
    echo "error: $bin_src not found. Run 'cargo build --release' first." >&2
    exit 1
fi
install -Dm755 "$bin_src" "$bindir/mc-scan"

# Desktop entry — write an absolute Exec so the launcher does not depend on PATH.
install -Dm644 "$here/mc-scan.desktop" "$apps_dir/mc-scan.desktop"
sed -i "s|^Exec=.*|Exec=$bindir/mc-scan|" "$apps_dir/mc-scan.desktop"

# Icons (all hicolor sizes + scalable svg)
while IFS= read -r -d '' icon; do
    rel="${icon#"$here/icons/hicolor/"}"
    install -Dm644 "$icon" "$icons_dir/$rel"
done < <(find "$here/icons/hicolor" -type f -print0)

# Refresh caches (best-effort)
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$apps_dir" || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "$icons_dir" || true
fi

echo "Done."

# For a per-user install, make sure $bindir is reachable from the terminal too.
# The .desktop launcher already uses an absolute path, so the menu works
# regardless; this only affects running `mc-scan` from a shell.
#
# We inspect the rc file of the user's login shell rather than the live $PATH:
# a login shell (or this script) may inherit ~/.local/bin from ~/.profile while
# an interactive zsh — which does not read ~/.profile — still lacks it.
if [[ "$mode" == "user" ]]; then
    # Pick the rc file of the user's login shell.
    case "$(basename "${SHELL:-}")" in
        zsh)  rc="$HOME/.zshrc" ;;
        bash) rc="$HOME/.bashrc" ;;
        *)    rc="$HOME/.profile" ;;
    esac

    marker="# added by mc-scan install.sh"
    if [[ -f "$rc" ]] && grep -qF "$marker" "$rc"; then
        echo "PATH entry already present in $rc (open a new terminal to pick it up)."
    elif [[ -f "$rc" ]] && grep -qE '\.local/bin' "$rc"; then
        # The rc already puts ~/.local/bin on PATH by its own means.
        echo "$rc already adds ~/.local/bin to PATH; nothing to do."
    else
        {
            echo ""
            echo "$marker"
            echo 'export PATH="$HOME/.local/bin:$PATH"'
        } >> "$rc"
        echo "Added $bindir to PATH in $rc."
        echo "Run 'source $rc' or open a new terminal to use 'mc-scan'."
    fi
fi

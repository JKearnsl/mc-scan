#!/usr/bin/env bash
# Remove the mc-scan binary, .desktop entry and hicolor icons.
#
# Usage:
#   ./uninstall.sh            # remove per-user install from ~/.local
#   sudo ./uninstall.sh --system   # remove system-wide install from /usr
#
# Must match the paths used by install.sh.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

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

echo "Removing mc-scan ($mode) from:"
echo "  binary:   $bindir"
echo "  desktop:  $apps_dir"
echo "  icons:    $icons_dir"

rm -f "$bindir/mc-scan"
rm -f "$apps_dir/mc-scan.desktop"

# Remove only the icons we installed (mirror the packaged layout).
while IFS= read -r -d '' icon; do
    rel="${icon#"$here/icons/hicolor/"}"
    rm -f "$icons_dir/$rel"
done < <(find "$here/icons/hicolor" -type f -print0)

# Remove the PATH line install.sh may have added (user mode only).
if [[ "$mode" == "user" ]]; then
    marker="# added by mc-scan install.sh"
    for rc in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.profile"; do
        if [[ -f "$rc" ]] && grep -qF "$marker" "$rc"; then
            # Delete the marker line and the export line that follows it.
            sed -i "/$marker/,+1d" "$rc"
            echo "Removed PATH entry from $rc."
        fi
    done
fi

# Refresh caches (best-effort)
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$apps_dir" || true
fi
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "$icons_dir" || true
fi

echo "Done."

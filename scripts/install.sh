#!/usr/bin/env bash
# /************************************************
# * File: install.sh
# * Author: Michal Švrček
# *
# * DevDock Linux installer
# *
# * ver. 0.4.0
# *************************************************/

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

SOURCE="${1:-$PROJECT_ROOT/target/release/dock}"
INSTALL_DIR="${HOME}/.local/bin"
DESTINATION="${INSTALL_DIR}/dock"

echo "DevDock Linux Installer"

if [[ ! -f "$SOURCE" ]]; then
    echo "Error: dock binary not found: $SOURCE" >&2
    echo "Run cargo build --release first." >&2
    exit 1
fi

mkdir -p "$INSTALL_DIR"

SOURCE_ABS="$(realpath "$SOURCE")"

if [[ "$SOURCE_ABS" != "$DESTINATION" ]]; then
    install -m 755 "$SOURCE_ABS" "$DESTINATION"
fi

# Add ~/.local/bin to PATH when necessary.
PROFILE="${HOME}/.profile"
PATH_LINE='export PATH="$HOME/.local/bin:$PATH"'

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    if ! grep -Fqx "$PATH_LINE" "$PROFILE" 2>/dev/null; then
        printf '\n# DevDock user binaries\n%s\n' "$PATH_LINE" >> "$PROFILE"
    fi

    export PATH="$INSTALL_DIR:$PATH"
fi

echo
echo "DevDock installed successfully!"
echo "Location: $DESTINATION"
"$DESTINATION" --version

echo
echo "Open a new login shell or load ~/.profile:"
echo "  source ~/.profile"
echo "Then run:"
echo "  dock --version"
echo "  dock list"

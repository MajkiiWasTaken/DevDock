#!/usr/bin/env bash
# /************************************************
# * File: uninstall.sh
# * Author: Michal Švrček
# *
# * DevDock Linux uninstaller
# *
# * ver. 0.4.0
# *************************************************/

set -euo pipefail

INSTALL_DIR="${HOME}/.local/bin"
DESTINATION="${INSTALL_DIR}/dock"
PROFILE="${HOME}/.profile"
PATH_LINE='export PATH="$HOME/.local/bin:$PATH"'

echo "DevDock Linux Uninstaller"

if [[ -f "$DESTINATION" ]]; then
    rm -- "$DESTINATION"
fi

# Do not remove ~/.local/bin from PATH:
# other applications may depend on this directory.

echo
echo "DevDock uninstalled successfully."
echo "Your sessions were preserved in:"
echo "  ${XDG_CONFIG_HOME:-$HOME/.config}/devdock/sessions"
echo
echo "The PATH configuration was preserved for other applications."

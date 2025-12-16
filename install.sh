#!/usr/bin/env sh
set -e

REPO="cyhndaugust/fekit-cli"
BIN="fekit"

OS=$(uname | tr '[:upper:]' '[:lower:]')

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

curl -fsSL \
  "https://github.com/$REPO/releases/latest/download/$BIN" \
  -o "$INSTALL_DIR/$BIN"

chmod +x "$INSTALL_DIR/$BIN"

echo "Installed to $INSTALL_DIR/$BIN"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo "⚠️ 请将 $INSTALL_DIR 加入 PATH"
fi
#!/usr/bin/env sh
set -eu

REPO="cyhndaugust/fekit-cli"
BIN="fekit"

command -v curl >/dev/null 2>&1 || {
  echo "curl 未安装，请先安装 curl 再重试。" >&2
  exit 1
}

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux) OS="linux" ;;
  darwin) OS="macos" ;;
  msys*|mingw*|cygwin*) OS="windows" ;;
  *) echo "暂不支持的操作系统: $OS" >&2; exit 1 ;;
esac

case "$ARCH" in
  x86_64|amd64|AMD64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="arm64" ;;
  *) echo "暂不支持的架构: $ARCH" >&2; exit 1 ;;
esac

EXT=""
[ "$OS" = "windows" ] && EXT=".exe"

ASSET="$BIN-$OS-$ARCH$EXT"
URL="https://github.com/$REPO/releases/latest/download/$ASSET"

INSTALL_DIR="${FEKIT_INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$INSTALL_DIR"
DEST="$INSTALL_DIR/$BIN$EXT"
ALIAS="$INSTALL_DIR/fk$EXT"

echo "Downloading $ASSET ..."
curl -fsSL "$URL" -o "$DEST"
chmod +x "$DEST"

cp "$DEST" "$ALIAS"
chmod +x "$ALIAS"

echo "已安装到 $DEST"
echo "已创建别名 $ALIAS"

if ! printf '%s' "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
  PROFILE=""
  for file in "$HOME/.zshrc" "$HOME/.bashrc" "$HOME/.profile"; do
    if [ -f "$file" ]; then
      PROFILE="$file"
      break
    fi
  done

  if [ -n "$PROFILE" ] && ! grep -Fq "$INSTALL_DIR" "$PROFILE"; then
    {
      echo ""
      echo "# Added by fekit installer"
      echo "export PATH=\"$INSTALL_DIR:\$PATH\""
    } >> "$PROFILE"
    echo "已将 $INSTALL_DIR 添加到 PATH，并写入 $PROFILE。重新打开终端或运行: source $PROFILE"
  else
    echo "请手动将 $INSTALL_DIR 添加到 PATH"
  fi
else
  echo "PATH 已包含 $INSTALL_DIR"
fi

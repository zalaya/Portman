#!/bin/sh
set -eu

REPO="zalaya/Portman"
BIN_NAME="portman"
INSTALL_DIR="${PORTMAN_INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${PORTMAN_VERSION:-latest}"

fail() {
    echo "error: $1" >&2
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || fail "'$1' is required but not installed"
}

need curl
need tar
need uname

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
    Darwin) platform="apple-darwin" ;;
    Linux) platform="unknown-linux-gnu" ;;
    *) fail "unsupported OS: $os (Windows: download the .zip from the Releases page instead)" ;;
esac

case "$arch" in
    arm64 | aarch64) target="aarch64-$platform" ;;
    x86_64 | amd64) target="x86_64-$platform" ;;
    *) fail "unsupported architecture: $arch" ;;
esac

if [ "$VERSION" = "latest" ]; then
    url="https://github.com/$REPO/releases/latest/download/$BIN_NAME-$target.tar.gz"
else
    url="https://github.com/$REPO/releases/download/$VERSION/$BIN_NAME-$target.tar.gz"
fi

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

echo "Downloading $BIN_NAME ($target, $VERSION)..."
curl -fsSL "$url" -o "$tmp_dir/$BIN_NAME.tar.gz" || fail "download failed — is there a release for $target yet?"

tar -xzf "$tmp_dir/$BIN_NAME.tar.gz" -C "$tmp_dir"

mkdir -p "$INSTALL_DIR"
mv "$tmp_dir/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

echo "Installed to $INSTALL_DIR/$BIN_NAME"

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *) echo "Add it to your PATH: export PATH=\"$INSTALL_DIR:\$PATH\"" ;;
esac

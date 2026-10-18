#!/bin/sh
set -eu

: "${HOME:?HOME is required}"

WORKSPACE_VOLUME_PATH="${WORKSPACE_VOLUME_PATH:-/workspace}"
WORKSPACE_LINK_NAME="${WORKSPACE_LINK_NAME:-workspace}"
WORKSPACE_HOME_PATH="${WORKSPACE_HOME_PATH:-$HOME/$WORKSPACE_LINK_NAME}"

if [ -e "$WORKSPACE_HOME_PATH" ] && [ ! -L "$WORKSPACE_HOME_PATH" ]; then
  echo "$WORKSPACE_HOME_PATH already exists and is not a symbolic link" >&2
  exit 1
fi

mkdir -p "$HOME"
ln -sfn "$WORKSPACE_VOLUME_PATH" "$WORKSPACE_HOME_PATH"

ssh-keygen -A
exec "$@"

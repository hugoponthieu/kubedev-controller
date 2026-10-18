#!/bin/sh
set -eu

: "${REPOSITORY_URL:?REPOSITORY_URL is required}"

REVISION="${REVISION:-main}"
TARGET_DIR="${TARGET_DIR:-/workspace/repository}"
WORKSPACE_UID="${WORKSPACE_UID:-1000}"
WORKSPACE_GID="${WORKSPACE_GID:-1000}"

mkdir -p "$(dirname "$TARGET_DIR")"

if [ -d "$TARGET_DIR/.git" ]; then
  echo "Updating existing repository"
  git -C "$TARGET_DIR" remote set-url origin "$REPOSITORY_URL"
else
  echo "Initializing repository"
  rm -rf "$TARGET_DIR"
  mkdir -p "$TARGET_DIR"
  git -C "$TARGET_DIR" init
  git -C "$TARGET_DIR" remote add origin "$REPOSITORY_URL"
fi

git -C "$TARGET_DIR" fetch --depth=1 -- origin "$REVISION"
git -C "$TARGET_DIR" checkout --detach --force FETCH_HEAD
git -C "$TARGET_DIR" clean -ffd

COMMIT_SHA="$(git -C "$TARGET_DIR" rev-parse HEAD)"

# The workspace image runs as the remote user (UID/GID 1000 by default).
chown -R "$WORKSPACE_UID:$WORKSPACE_GID" "$TARGET_DIR"

# This marker is written last so consumers only start after a complete clone.
printf '%s\n' "$COMMIT_SHA" > "$TARGET_DIR/.clone-complete.tmp"
chown "$WORKSPACE_UID:$WORKSPACE_GID" "$TARGET_DIR/.clone-complete.tmp"
mv "$TARGET_DIR/.clone-complete.tmp" "$TARGET_DIR/.clone-complete"

echo "Repository ready at commit $COMMIT_SHA"

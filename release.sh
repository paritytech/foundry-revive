#!/bin/bash

set -e  # exit on error.

### Check parameters for the new tag.

if [ -z "$1" ]; then
  echo "Error: No new TAG provided."
  echo "Example: $0 v1.1.0"
  exit 1
fi

TAG="$1"

### Verify and prepare.

git checkout master
git pull origin master

# cargo build --release

### Update CHANGELOG.md.

LATEST_TAG=$(git tag --list 'v*' | sort -V | tail -n 1)                             # get the latest tag.
LATEST_RELEASE_NOTES_FILE="/tmp/release_notes@${LATEST_TAG}.txt"
git log --oneline --no-merges "$LATEST_TAG"..HEAD > "$LATEST_RELEASE_NOTES_FILE"    # get the latest changes and save to a file.
# TODO (@filip-parity): Update CHANGELOG.md with the latest changes.

### Update stable tag.

git push origin :refs/tags/stable           # delete the remote stable tag.
git tag -fa stable -m "Update stable tag"   # create or move the local stable tag (force, annotated).
git push origin --tags                      # push the new stable tag.

### Create and push version tag.

git tag -d "$TAG" || true
git tag -a "$TAG" -m "Created release tag $TAG" # create an annotated version tag.
git push origin "$TAG"                          # push the version tag.

### Create a release on GitHub.

gh repo set-default paritytech/foundry-polkadot
gh release create "$TAG" --title "$TAG" --notes-file "$LATEST_RELEASE_NOTES_FILE"

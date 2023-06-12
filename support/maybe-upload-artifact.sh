#!/usr/bin/env bash
# Originally from https://cirrus-ci.org/examples/#release-assets

set -eo pipefail

if [[ "$CIRRUS_RELEASE" == "" ]]; then
  echo "Not a release. No need to deploy!"
  exit 0
fi

if [[ "$GITHUB_TOKEN" == "" ]]; then
  echo "Please provide GitHub access token via GITHUB_TOKEN environment variable!"
  exit 1
fi

if [[ "$1" == "" ]]; then
  echo "Please provide a destination filename (shown on the Github Releases page) as \$1."
  exit 1
fi

file_content_type="application/octet-stream"
fpath="./thcon"
name=$1
url_to_upload="https://uploads.github.com/repos/$CIRRUS_REPO_FULL_NAME/releases/$CIRRUS_RELEASE/assets?name=$name"

echo "Uploading $fpath..."
curl -X POST \
  --data-binary @$fpath \
  --header "Authorization: token $GITHUB_TOKEN" \
  --header "Content-Type: application/octet-stream" \
  $url_to_upload



#!/bin/bash

# exit if any command fails
set -e          #exit if any command fails
set -E          #exit if any command fails
set -T          # DEBUG trap will work in functions
set -o nounset  #no unset variables
set -o pipefail # get the correct error in a pipe i.e. : cat f | grep Mike
# FIX: Remove before check-in!!!
#set -v

if ! source /home/mike/scripts/utils.sh; then
   echo utils.sh not found
   exit 1
fi

sourceFile .env

GH_USER=mikewallacedev
APP_TAG=penguin-site
APP_VERSION=latest

banner "Deploying $MW2_APP_NAME to GHCR"

if [ -z "$GHCR_TOKEN" ]; then
   printError "'GHCR_TOKEN' is not set."
   printError "Exiting...."
   exit 1
fi

printMessage "Logging in to Github Container Registry with user : $GH_USER"
echo "$GHCR_TOKEN" | docker login ghcr.io -u "$GH_USER" --password-stdin

printMessage "Building $MW2_APP_NAME"
docker build -t "$APP_TAG" .

printMessage "Tagging $MW2_APP_NAME:$APP_VERSION"
docker tag "$APP_TAG":"$APP_VERSION" ghcr.io/"$GH_USER"/"$APP_TAG":"$APP_VERSION"

printMessage "Pushing $APP_TAG" to GHCR
docker push ghcr.io/"$GH_USER"/"$APP_TAG":"$APP_VERSION"

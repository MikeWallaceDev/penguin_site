#!/bin/zsh

source ~/scripts/utils.sh

printMessage "Building..."

if [ -f ".env" ]; then
   printMessage "Sourcing .env"
   source .env
fi

cargo --verbose leptos build

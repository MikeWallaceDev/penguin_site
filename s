#!/bin/zsh

source ~/scripts/utils.sh

printMessage "Serving..."

# cd ~/wh_code/resumes/alloresume/

if [ -f ".env" ]; then
   printMessage "Sourcing .env"
   source .env
fi

cargo leptos serve

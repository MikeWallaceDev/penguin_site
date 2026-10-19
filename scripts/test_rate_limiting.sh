#!/bin/bash

source /home/mike/scripts/utils.sh

# Source the .env file to set environment variables
sourceFile .env

DELAY=5
FINAL_DELAY=75
HOST=127.0.0.1
PORT=3000
HEADER="Content-Type: application/json"
DATA='{"name":"John Doe", "email":"john@example.com", "message":"Hello from curl! Hello from curl! Hello from curl! Hello from curl!"}'

# Function to send requests
send_request() {

   code=$(
      curl -s -o /tmp/opencode/resp_"$i".txt -w "%{http_code}" \
         -X POST http://"$HOST":"$PORT"/api/SaveContact \
         -H "$HEADER" \
         -d "$DATA"
   )

   printMessage "request $i -> HTTP $code"

   printMessage "Waiting for $DELAY seconds..."
   sleep "$DELAY"
}

main() {

   # Send 7 requests within a 15-minute window
   for i in {1..7}; do
      send_request
      echo
   done

   printDebug "===== HEADERS of last response =====\n"

   curl -s -i \
      -X POST http://"$HOST":"$PORT"/api/SaveContact \
      -H "$HEADER" \
      -d "$DATA" | head -30

   echo
   printDebug "===== End of HEADERS =====\n"

   # Wait for 15 minutes
   printMessage "Waiting for $FINAL_DELAY seconds..."
   sleep "$FINAL_DELAY"

   # Send an additional request to observe rate limiting
   send_request
}

main

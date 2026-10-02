source ~/scripts/utils.sh

PORT_NUMBER=3000
APP_NAME="techno_penguin"

printMessage "Searching for '$APP_NAME' on port : $PORT_NUMBER "

if ss -lptn "sport = :$PORT_NUMBER" | grep "$APP_NAME"; then
   app_pid=$(fuser 3000/tcp)
   printMessage "Found app PID : $app_pid"

   printMessage "Killing!!!"
   kill "$app_pid"
   # sudo kill "$app_pid"
else
   printWarning "Not found..."
fi

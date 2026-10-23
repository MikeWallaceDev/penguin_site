source "$HOME/scripts/utils.sh"

PORT_NUMBER=3000
APP_NAME="techno_penguin"

DO_KILL=false
FOUND_APP_PID=-1

proces_args() {
   while [[ $# -gt 0 ]]; do # Loop through all arguments
      case "$1" in
      -n | --name)
         APP_NAME="$2"
         shift 2 # Past argument name and value
         ;;
      -p | --port)
         PORT_NUMBER="$2"
         shift 2 # Past argument name and value
         ;;
      -k | --kill)
         DO_KILL="true"
         shift 1 # Past argument name (flag only, no value)
         ;;
      *)
         echo "Usage: $0 [options]"
         echo "  -n, --name   Name of the app to find"
         echo "  -p, --port   Set the port (default: 3000)"
         echo "  -k, --kill   Kill app if found"
         exit 0
         ;;
      esac
   done
}

# Takes in an app name and returns its process id if running AND
# it's using the specified port number
#
# Sets FOUND_APP_PID to process ID if the item is found
# Sets FOUND_APP_PID to -1 if not
#
# usage :
#
# find_app_id app_name port_number
find_app_id() {
   local app_name=$1
   local port_number=$2

   if ss -lptn "sport = :$port_number" | grep "$app_name"; then
      FOUND_APP_PID=$(fuser "$port_number"/tcp)
   else
      FOUND_APP_PID=-1
   fi
}

find_port_for_app() {
   local app_name=$1
   sudo ss -tulpn | grep "$app_name"
}

kill_app_by_id() {
   printMessage "Killing!!!"
   kill "$FOUND_APP_PID"
   # sudo kill "$app_pid"
}

main() {
   proces_args "$@"

   printMessage "Searching for '$APP_NAME' on port : $PORT_NUMBER "

   find_app_id "$APP_NAME" "$PORT_NUMBER"

   if [ "$FOUND_APP_PID" -ne -1 ]; then
      printMessage "Found app PID : $FOUND_APP_PID"

      if [ "$DO_KILL" = true ]; then
         kill_app_by_id
      fi
      exit # processing finisned
   fi

   printWarning "'$APP_NAME' was NOT found on port : $PORT_NUMBER\n"

   prompt=$(printMessage "Do you wish to see if it's running on a different port? ")
   read -r -n 1 -p "$prompt" yn
   yn=${yn:-n} # Default to no
   echo

   if [[ "$yn" =~ [Yy] ]]; then
      find_port_for_app "$APP_NAME"
      if [ "$FOUND_APP_PID" -ne -1 ]; then
         printMessage "Found PID $FOUND_APP_PID\n"

         prompt=$(printMessage "Do you wish to kill it? ")
         read -r -n 1 -p "$prompt" yn
         yn=${yn:-n} # Default to no
         echo

         if [[ "$yn" =~ [Yy] ]]; then
            kill_app_by_id
         fi
      else
         printWarning "'$APP_NAME' is NOT using any port"
      fi
   fi
}

main "$@"

#!/bin/bash

# Created by Typefox under MIT license in unknown year
# Refactored by Jacob Hrbek <kreyren@rixotstudio.cz> under the same license in 2020

: "
  This is a script used to start a VNC session

  We are allowing following window managers to be used as $WINDOW_MANAGER value of variable:
  - openbox

	See https://en.wikipedia.org/wiki/Xvfb#Remote_control_over_SSH for reference
"

die() { printf 'FATAL: %s\n' "$2" ; exit "$1";}

DISP="${DISPLAY:1}"

Xvfb -screen "$DISP" 1920x1080x16 -ac -pn -noreset &

# Process WINDOW_MANAGER
if ! apt list --installed "$WINDOW_MANAGER" | grep -oP "^$WINDOW_MANAGER\\s{1}-.*"; then
	apt install -y "$WINDOW_MANAGER" || die 1 "Unable to install $WINDOW_MANAGER on the system"
elif apt list --installed "$WINDOW_MANAGER" | grep -oP "^$WINDOW_MANAGER\\s{1}-.*"; then
	true
else
	exit 255
fi

# Execute WINDOW_MANAGER
"$WINDOW_MANAGER" &

# FIXME-QA: Why?
VNC_PORT="$(( 5900 + "$DISP" ))"
NOVNC_PORT="$(( 6080 + "$DISP" ))"

# FIXME-QA: QA!
x11vnc -localhost -shared -display ":$DISP" -forever -rfbport "$VNC_PORT" -bg -o "/tmp/x11vnc-$DISP.log"
cd /opt/novnc/utils && /launch.sh --vnc "localhost:$VNC_PORT" --listen "$NOVNC_PORT" &

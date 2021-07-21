#!/bin/bash

# Start the ap0 service (access point) if wlan0 is active but not connected

# returns "active" or "inactive"
wlan_active=$(/usr/bin/systemctl is-active wpa_supplicant@wlan0.service)

# returns "up" or "down"
wlan_state=$(cat /sys/class/net/wlan0/operstate)

if [ $wlan_active = "active" ] && [ $wlan_state = "down" ]; then
    echo "Starting ap0 service"
    /usr/bin/systemctl start wpa_supplicant@ap0.service
fi

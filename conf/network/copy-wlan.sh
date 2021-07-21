#!/bin/bash

FILE=/boot/firmware/wpa_supplicant.conf
if test -f "$FILE"; then
    cp $FILE /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
    chown root:netdev /etc/wpa_supplicant/wpa_supplicant-wlan0.conf
    rm $FILE
fi